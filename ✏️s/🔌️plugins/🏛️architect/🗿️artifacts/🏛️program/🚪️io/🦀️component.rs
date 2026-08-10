//! program IO stdio matrix
pub fn register() {
    crate::artifacts::program::io::import::deserializers::artifacts::csv::register();
    crate::artifacts::program::io::import::deserializers::artifacts::json::register();
    crate::artifacts::program::io::import::deserializers::artifacts::xlsx::register();
    crate::artifacts::program::io::import::deserializers::artifacts::zip::register();
    crate::artifacts::program::io::export::serializers::artifacts::csv::register();
    crate::artifacts::program::io::export::serializers::artifacts::json::register();
    crate::artifacts::program::io::export::serializers::artifacts::xlsx::register();
    crate::artifacts::program::io::export::serializers::artifacts::zip::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"]
}
