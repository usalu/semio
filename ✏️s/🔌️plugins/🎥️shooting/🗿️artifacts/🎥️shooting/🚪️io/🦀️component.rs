//! shooting IO stdio matrix
pub fn register() {
    crate::artifacts::shooting::io::import::deserializers::artifacts::bmp::register();
    crate::artifacts::shooting::io::import::deserializers::artifacts::dwg::register();
    crate::artifacts::shooting::io::import::deserializers::artifacts::gif::register();
    crate::artifacts::shooting::io::import::deserializers::artifacts::jpg::register();
    crate::artifacts::shooting::io::import::deserializers::artifacts::json::register();
    crate::artifacts::shooting::io::import::deserializers::artifacts::pdf::register();
    crate::artifacts::shooting::io::import::deserializers::artifacts::png::register();
    crate::artifacts::shooting::io::import::deserializers::artifacts::svg::register();
    crate::artifacts::shooting::io::import::deserializers::artifacts::tiff::register();
    crate::artifacts::shooting::io::export::serializers::artifacts::bmp::register();
    crate::artifacts::shooting::io::export::serializers::artifacts::dwg::register();
    crate::artifacts::shooting::io::export::serializers::artifacts::gif::register();
    crate::artifacts::shooting::io::export::serializers::artifacts::jpg::register();
    crate::artifacts::shooting::io::export::serializers::artifacts::json::register();
    crate::artifacts::shooting::io::export::serializers::artifacts::pdf::register();
    crate::artifacts::shooting::io::export::serializers::artifacts::png::register();
    crate::artifacts::shooting::io::export::serializers::artifacts::svg::register();
    crate::artifacts::shooting::io::export::serializers::artifacts::tiff::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.bmp", "stdio.dwg", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.tiff"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.bmp", "stdio.dwg", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.tiff"]
}
