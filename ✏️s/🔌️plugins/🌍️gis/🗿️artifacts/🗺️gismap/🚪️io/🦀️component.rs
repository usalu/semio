//! gismap IO stdio matrix
pub fn register() {
    crate::artifacts::gismap::io::import::deserializers::artifacts::dwg::register();
    crate::artifacts::gismap::io::import::deserializers::artifacts::dxf::register();
    crate::artifacts::gismap::io::import::deserializers::artifacts::json::register();
    crate::artifacts::gismap::io::import::deserializers::artifacts::pdf::register();
    crate::artifacts::gismap::io::import::deserializers::artifacts::png::register();
    crate::artifacts::gismap::io::import::deserializers::artifacts::svg::register();
    crate::artifacts::gismap::io::export::serializers::artifacts::dwg::register();
    crate::artifacts::gismap::io::export::serializers::artifacts::dxf::register();
    crate::artifacts::gismap::io::export::serializers::artifacts::json::register();
    crate::artifacts::gismap::io::export::serializers::artifacts::pdf::register();
    crate::artifacts::gismap::io::export::serializers::artifacts::png::register();
    crate::artifacts::gismap::io::export::serializers::artifacts::svg::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"]
}
