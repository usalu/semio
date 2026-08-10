//! 🚪️ IO s.writer (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.docx", "stdio.json", "stdio.md", "stdio.pdf", "stdio.txt"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.docx", "stdio.json", "stdio.md", "stdio.pdf", "stdio.txt"] }
pub fn writer_to_wire(from: &crate::artifacts::writer::WriterSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(from)
}
pub fn writer_from_wire(bytes: &[u8]) -> Result<crate::artifacts::writer::WriterSnapshot, store::PackError> {
    <crate::artifacts::writer::WriterSnapshot as store::DocumentPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
