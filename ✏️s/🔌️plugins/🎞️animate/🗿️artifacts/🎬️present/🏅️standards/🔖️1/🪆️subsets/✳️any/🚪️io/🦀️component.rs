//! 🚪️ IO s.present (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg", "stdio.txt"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg", "stdio.txt"] }
