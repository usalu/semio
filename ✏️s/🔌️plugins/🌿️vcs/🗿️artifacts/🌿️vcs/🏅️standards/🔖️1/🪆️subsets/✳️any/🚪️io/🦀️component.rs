//! 🚪️ IO s.vcs (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.txt", "stdio.xlsx", "stdio.zip"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.txt", "stdio.xlsx", "stdio.zip"] }
pub fn vcs_to_wire(from: &crate::artifacts::vcs::VcsSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(from)
}
pub fn vcs_from_wire(bytes: &[u8]) -> Result<crate::artifacts::vcs::VcsSnapshot, store::PackError> {
    <crate::artifacts::vcs::VcsSnapshot as store::ArtifactPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
