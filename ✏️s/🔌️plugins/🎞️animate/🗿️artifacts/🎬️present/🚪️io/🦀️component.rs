//! 🚪️ 🎬️present IO facet — declared MediaFormat table + OS handler registration.

use semio_framework_plugin::{ArtifactIo, IoFormatSpec, MediaFormat};

//#region 🔖️Formats
pub fn format_specs() -> &'static [IoFormatSpec] {
    &[
        IoFormatSpec { format: MediaFormat::Json, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Md, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Pdf, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Png, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Pptx, import: true, export: true },
        IoFormatSpec { format: MediaFormat::Svg, import: true, export: true }
    ]
}
//#endregion 🔖️Formats

//#region 🔖️ArtifactIo
/// 🚪️ present artifact IO registration surface.
pub struct Io;

impl ArtifactIo for Io {
    fn formats() -> &'static [IoFormatSpec] { format_specs() }
    fn register() {
        super::json::export::register();
        super::json::import::register();
        super::md::export::register();
        super::md::import::register();
        super::pdf::export::register();
        super::pdf::import::register();
        super::png::export::register();
        super::png::import::register();
        super::pptx::export::register();
        super::pptx::import::register();
        super::svg::export::register();
        super::svg::import::register();
    }
}

pub fn register() {
    <Io as ArtifactIo>::register();
}
//#endregion 🔖️ArtifactIo
