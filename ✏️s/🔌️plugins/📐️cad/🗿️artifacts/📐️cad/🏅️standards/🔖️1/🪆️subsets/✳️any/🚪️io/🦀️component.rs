//! 🚪️ IO s.cad (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.glb", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.glb", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"] }
