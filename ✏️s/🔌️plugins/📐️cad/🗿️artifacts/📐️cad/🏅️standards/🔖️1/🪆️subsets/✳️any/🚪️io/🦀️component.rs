//! 🚪️ IO s.cad (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"] }
pub fn cad_to_wire(from: &crate::artifacts::cad::CadSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(from)
}
pub fn cad_from_wire(bytes: &[u8]) -> Result<crate::artifacts::cad::CadSnapshot, store::PackError> {
    <crate::artifacts::cad::CadSnapshot as store::ArtifactPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
