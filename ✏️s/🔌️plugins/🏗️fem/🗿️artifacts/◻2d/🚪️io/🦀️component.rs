//! fem2d IO stdio matrix
pub fn register() {
    crate::artifacts::fem2d::io::import::deserializers::artifacts::csv::register();
    crate::artifacts::fem2d::io::import::deserializers::artifacts::json::register();
    crate::artifacts::fem2d::io::import::deserializers::artifacts::md::register();
    crate::artifacts::fem2d::io::export::serializers::artifacts::csv::register();
    crate::artifacts::fem2d::io::export::serializers::artifacts::json::register();
    crate::artifacts::fem2d::io::export::serializers::artifacts::md::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.csv", "stdio.json", "stdio.md"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.csv", "stdio.json", "stdio.md"]
}
