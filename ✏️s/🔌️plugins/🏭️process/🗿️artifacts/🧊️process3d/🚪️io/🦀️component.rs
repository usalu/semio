//! process3d IO stdio matrix
pub fn register() {
    crate::artifacts::process3d::io::import::deserializers::artifacts::dwg::register();
    crate::artifacts::process3d::io::import::deserializers::artifacts::glb::register();
    crate::artifacts::process3d::io::import::deserializers::artifacts::gltf::register();
    crate::artifacts::process3d::io::import::deserializers::artifacts::ifc::register();
    crate::artifacts::process3d::io::import::deserializers::artifacts::json::register();
    crate::artifacts::process3d::io::import::deserializers::artifacts::obj::register();
    crate::artifacts::process3d::io::import::deserializers::artifacts::png::register();
    crate::artifacts::process3d::io::import::deserializers::artifacts::step::register();
    crate::artifacts::process3d::io::import::deserializers::artifacts::stl::register();
    crate::artifacts::process3d::io::export::serializers::artifacts::dwg::register();
    crate::artifacts::process3d::io::export::serializers::artifacts::glb::register();
    crate::artifacts::process3d::io::export::serializers::artifacts::gltf::register();
    crate::artifacts::process3d::io::export::serializers::artifacts::ifc::register();
    crate::artifacts::process3d::io::export::serializers::artifacts::json::register();
    crate::artifacts::process3d::io::export::serializers::artifacts::obj::register();
    crate::artifacts::process3d::io::export::serializers::artifacts::png::register();
    crate::artifacts::process3d::io::export::serializers::artifacts::step::register();
    crate::artifacts::process3d::io::export::serializers::artifacts::stl::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.glb", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.glb", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"]
}
