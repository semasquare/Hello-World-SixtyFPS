
use crate::typeloader::{VirtualFile, VirtualDirectory};
pub fn widget_library() -> &'static [(&'static str, &'static VirtualDirectory<'static>)] {
    &[
("native", &[&VirtualFile {path: r#"sixtyfps_widgets.60"# , contents: include_str!(r#"/home/fabian/.cargo/registry/src/github.com-1ecc6299db9ec823/sixtyfps-compilerlib-0.1.0/widgets/native/sixtyfps_widgets.60"#)}]),
("ugly", &[&VirtualFile {path: r#"sixtyfps_widgets.60"# , contents: include_str!(r#"/home/fabian/.cargo/registry/src/github.com-1ecc6299db9ec823/sixtyfps-compilerlib-0.1.0/widgets/ugly/sixtyfps_widgets.60"#)}]),
]
}
