//! 🔌️ Presentation IO and frame helpers: the presentation media IO surface, the frame import grid columns, the next frame tile id and crop and the frame media name. See [`super`].

use super::*;

//#region 🔖️Io
/// 🔌️ Relocated verbatim from the former artifact-tree `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors `create_animate_presentation_app`'s `.artifact_kind(...)` literal (schema/
/// media type copied verbatim) plus the extra `frames:in` input port (Wave-2 port recipe).
pub fn presentation_io() -> AppIo {
    AppIo {
        artifact_schema: PRESENTATION_DOCUMENT_SCHEMA.into(),
        artifact_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Presentation, form: semio_framework_plugin::MediaForm::Deck },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "frames:in".into(),
            label: "Frames".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
            kind_id: Some("2d.image".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: PRESENTATION_DOCUMENT_SCHEMA.into(), name: "Animate Presentation Deck".into(), dimension: "2d".into(), component_kind: "panel".into() },
    }
}

/// 🎞️ `frames:in` placement (Wave-2 port recipe) — `PresentationSnapshot` models one shared background
/// `source` image with named crop-`tiles` over it; there is no per-tile independent raster payload in
/// this schema, so an incoming `2d.image` frame becomes a new tile positioned in a deterministic
/// contact-sheet grid (4 columns) rather than replacing `source` — exactly the surface `seedGrid`/
/// `addTile` (see the app's `🎮️commands/🀄️add-tile`/`🎮️commands/🌱️seed-grid`) already let a user crop/arrange
/// candidate frames on. Pure: both functions depend only on the current tile COUNT, so repeated imports
/// land in distinct, stable cells without needing a live host/counter.
const FRAME_IMPORT_GRID_COLUMNS: usize = 4;

pub fn next_frame_tile_id(existing_tile_count: usize) -> String {
    format!("frame-{}", existing_tile_count + 1)
}

pub fn next_frame_tile_crop(existing_tile_count: usize) -> crate::FigureTileFrame {
    let cell = 1.0 / FRAME_IMPORT_GRID_COLUMNS as f64;
    let column = existing_tile_count % FRAME_IMPORT_GRID_COLUMNS;
    let row = existing_tile_count / FRAME_IMPORT_GRID_COLUMNS;
    crate::standards::v1::subsets::any::schema::tile_grid::clamp_tile_crop(&crate::FigureTileFrame { x: column as f64 * cell, y: (row as f64 * cell).min(1.0 - cell), width: cell, height: cell })
}
//#endregion 🔖️Io

//#region 🔖️Helpers
/// 🎞️ `frames:in` display name (Wave-2 port recipe) — a `Structured` payload's `"name"`/`"src"` field
/// (falling back to a generic label), a `Binary` payload's leading blob-hash characters.
pub(super) fn frame_media_name(port: &str, media: &Media) -> Result<String, MediaError> {
    match &media.payload {
        MediaPayload::Structured { json, .. } => {
            let value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            Ok(value.get("name").and_then(|v| v.as_str()).or_else(|| value.get("src").and_then(|v| v.as_str())).map_or_else(|| "Imported frame".into(), str::to_string))
        }
        MediaPayload::Binary { blob_hash, .. } => Ok(format!("frame-{}", &blob_hash[..blob_hash.len().min(8)])),
        MediaPayload::Intrinsic { value, .. } => Ok(value.get("name").and_then(semio_framework_value::DslValue::as_str).or_else(|| value.get("src").and_then(semio_framework_value::DslValue::as_str)).map_or_else(|| "Imported frame".into(), str::to_string)),
    }
}
//#endregion 🔖️Helpers
