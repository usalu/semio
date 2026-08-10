//! present IO stdio matrix
pub fn register() {
    crate::artifacts::present::io::import::deserializers::artifacts::json::register();
    crate::artifacts::present::io::import::deserializers::artifacts::md::register();
    crate::artifacts::present::io::import::deserializers::artifacts::pdf::register();
    crate::artifacts::present::io::import::deserializers::artifacts::png::register();
    crate::artifacts::present::io::import::deserializers::artifacts::pptx::register();
    crate::artifacts::present::io::import::deserializers::artifacts::svg::register();
    crate::artifacts::present::io::export::serializers::artifacts::json::register();
    crate::artifacts::present::io::export::serializers::artifacts::md::register();
    crate::artifacts::present::io::export::serializers::artifacts::pdf::register();
    crate::artifacts::present::io::export::serializers::artifacts::png::register();
    crate::artifacts::present::io::export::serializers::artifacts::pptx::register();
    crate::artifacts::present::io::export::serializers::artifacts::svg::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg"]
}
