//! layout IO stdio matrix
pub fn register() {
    crate::artifacts::layout::io::import::deserializers::artifacts::dwg::register();
    crate::artifacts::layout::io::export::serializers::artifacts::dwg::register();
    crate::artifacts::layout::io::import::deserializers::artifacts::dxf::register();
    crate::artifacts::layout::io::export::serializers::artifacts::dxf::register();
    crate::artifacts::layout::io::import::deserializers::artifacts::json::register();
    crate::artifacts::layout::io::export::serializers::artifacts::json::register();
    crate::artifacts::layout::io::import::deserializers::artifacts::pdf::register();
    crate::artifacts::layout::io::export::serializers::artifacts::pdf::register();
    crate::artifacts::layout::io::import::deserializers::artifacts::png::register();
    crate::artifacts::layout::io::export::serializers::artifacts::png::register();
    crate::artifacts::layout::io::import::deserializers::artifacts::svg::register();
    crate::artifacts::layout::io::export::serializers::artifacts::svg::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"] }

pub fn layout_to_wire(from: &crate::artifacts::layout::LayoutSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(from)
}
pub fn layout_from_wire(bytes: &[u8]) -> Result<crate::artifacts::layout::LayoutSnapshot, store::PackError> {
    <crate::artifacts::layout::LayoutSnapshot as store::DocumentPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
