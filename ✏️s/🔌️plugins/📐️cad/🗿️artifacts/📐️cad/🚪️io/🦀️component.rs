//! CAD IO facet — stdio deserializers/serializers.

//#region Register
pub fn register() {
    crate::artifacts::cad::io::import::deserializers::artifacts::dwg::register();
    crate::artifacts::cad::io::export::serializers::artifacts::dwg::register();
    crate::artifacts::cad::io::import::deserializers::artifacts::glb::register();
    crate::artifacts::cad::io::export::serializers::artifacts::glb::register();
    crate::artifacts::cad::io::import::deserializers::artifacts::gltf::register();
    crate::artifacts::cad::io::export::serializers::artifacts::gltf::register();
    crate::artifacts::cad::io::import::deserializers::artifacts::ifc::register();
    crate::artifacts::cad::io::export::serializers::artifacts::ifc::register();
    crate::artifacts::cad::io::import::deserializers::artifacts::json::register();
    crate::artifacts::cad::io::export::serializers::artifacts::json::register();
    crate::artifacts::cad::io::import::deserializers::artifacts::obj::register();
    crate::artifacts::cad::io::export::serializers::artifacts::obj::register();
    crate::artifacts::cad::io::import::deserializers::artifacts::png::register();
    crate::artifacts::cad::io::export::serializers::artifacts::png::register();
    crate::artifacts::cad::io::import::deserializers::artifacts::step::register();
    crate::artifacts::cad::io::export::serializers::artifacts::step::register();
    crate::artifacts::cad::io::import::deserializers::artifacts::stl::register();
    crate::artifacts::cad::io::export::serializers::artifacts::stl::register();
}
//#endregion Register

//#region Wire
/// Pack a cad snapshot to wire bytes.
pub fn cad_to_wire(from: &crate::artifacts::cad::CadSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(from)
}

/// Unpack a cad snapshot from wire bytes.
pub fn cad_from_wire(bytes: &[u8]) -> Result<crate::artifacts::cad::CadSnapshot, store::PackError> {
    <crate::artifacts::cad::CadSnapshot as store::DocumentPack>::decode_pack(bytes)
}

pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
//#endregion Wire
