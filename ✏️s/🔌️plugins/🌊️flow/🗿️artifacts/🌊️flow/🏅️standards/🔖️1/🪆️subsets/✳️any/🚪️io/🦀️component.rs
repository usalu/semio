//! 🚪️ IO s.flow (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.md", "stdio.txt"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.md", "stdio.txt"] }
pub fn flow_to_wire(from: &crate::artifacts::flow::FlowSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(from)
}
pub fn flow_from_wire(bytes: &[u8]) -> Result<crate::artifacts::flow::FlowSnapshot, store::PackError> {
    <crate::artifacts::flow::FlowSnapshot as store::DocumentPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
