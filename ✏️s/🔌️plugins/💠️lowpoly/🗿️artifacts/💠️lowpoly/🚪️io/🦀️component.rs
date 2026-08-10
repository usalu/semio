//! 🚪️ 💠️lowpoly IO facet — declared MediaFormat table + OS handler registration.

use semio_framework_plugin::{ArtifactIo, IoFormatSpec, MediaFormat};

//#region 🔖️Formats
pub fn format_specs() -> &'static [IoFormatSpec] {
    &[
        IoFormatSpec { format: MediaFormat::Dwg, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Glb, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Gltf, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Json, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Las, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Obj, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Ply, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Png, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Stl, import: true, export: true }
    ]
}
//#endregion 🔖️Formats

//#region 🔖️ArtifactIo
/// 🚪️ lowpoly artifact IO registration surface.
pub struct Io;

impl ArtifactIo for Io {
    fn formats() -> &'static [IoFormatSpec] { format_specs() }
    fn register() {
        super::dwg::export::register();
        super::dwg::import::register();
        super::glb::export::register();
        super::glb::import::register();
        super::gltf::export::register();
        super::gltf::import::register();
        super::json::export::register();
        super::json::import::register();
        super::las::export::register();
        super::las::import::register();
        super::obj::export::register();
        super::obj::import::register();
        super::ply::export::register();
        super::ply::import::register();
        super::png::export::register();
        super::png::import::register();
        super::stl::export::register();
        super::stl::import::register();
    }
}

pub fn register() {
    <Io as ArtifactIo>::register();
}
//#endregion 🔖️ArtifactIo
