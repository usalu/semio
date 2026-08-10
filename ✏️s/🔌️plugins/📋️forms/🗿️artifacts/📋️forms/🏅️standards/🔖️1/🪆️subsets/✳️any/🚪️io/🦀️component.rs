//! 🚪️ IO s.forms (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"] }
pub fn forms_to_wire(from: &crate::artifacts::forms::FormsSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(from)
}
pub fn forms_from_wire(bytes: &[u8]) -> Result<crate::artifacts::forms::FormsSnapshot, store::PackError> {
    <crate::artifacts::forms::FormsSnapshot as store::DocumentPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
