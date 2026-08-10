//! vcs IO stdio matrix
pub fn register() {
    crate::artifacts::vcs::io::import::deserializers::artifacts::csv::register();
    crate::artifacts::vcs::io::export::serializers::artifacts::csv::register();
    crate::artifacts::vcs::io::import::deserializers::artifacts::json::register();
    crate::artifacts::vcs::io::export::serializers::artifacts::json::register();
    crate::artifacts::vcs::io::import::deserializers::artifacts::xlsx::register();
    crate::artifacts::vcs::io::export::serializers::artifacts::xlsx::register();
    crate::artifacts::vcs::io::import::deserializers::artifacts::zip::register();
    crate::artifacts::vcs::io::export::serializers::artifacts::zip::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"] }
pub fn vcs_to_wire(from: &crate::artifacts::vcs::VcsSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(from)
}
pub fn vcs_from_wire(bytes: &[u8]) -> Result<crate::artifacts::vcs::VcsSnapshot, store::PackError> {
    <crate::artifacts::vcs::VcsSnapshot as store::DocumentPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
