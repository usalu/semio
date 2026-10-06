//! 🎥️ Animate presentation app command — `export-video-from-deck`: turns the deck (an overview of its source picture, then
//! one slide per tile) — or the presentation scene `scene_json` states, played through the engine's own capture pipeline —
//! into a `VideoRenderProgram` and asks the host to render, encode and download it as an MP4 (`Effect::VideoRenderExport`,
//! the `media.video-render` capability this plugin requests). A guest has no canvas and no encoder; the host owns both and
//! runs the render as an event-sourced job with progress and cancellation.

use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::engine::{video_filename_for_deck, video_filename_for_title, video_render_program_from_deck, video_render_program_from_scene, PresentationScene, PresentationVideoExportError};
use crate::editor::animate::PresentationDispatchCtx;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::PresentationSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

/// 📏️ The largest program one export may hand the host in a single effect: every path point is 8 bytes, every paint
/// operation about 100 and every image URL its own length once packed, and the whole effect must stay under the guest's
/// contiguous-request ceiling (64 KiB).
pub const ANIMATE_VIDEO_PROGRAM_MAXIMUM_BYTES: usize = 49_152;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-video-from-deck")]
pub struct ExportVideoFromDeck {
    pub scene_json: String,
}

/// 📏️ What `program` costs packed, by the same per-point / per-op / per-URL measure as [`ANIMATE_VIDEO_PROGRAM_MAXIMUM_BYTES`].
pub fn video_program_packed_bytes(program: &semio_framework::kernel::VideoRenderProgram) -> usize {
    let points: usize = program.paths.iter().map(|path| path.points.len() * 8 + path.verbs.len() + 16).sum();
    let images: usize = program.images.iter().map(|image| image.url.len() + 16).sum();
    let ops: usize = program.scenes.iter().map(|scene| scene.ops.len() * 100 + 8).sum();
    points + images + ops + program.timeline.len() * 12 + 128
}

fn refused(error: PresentationVideoExportError) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(error.code()), error.to_string())
}

fn scene_json_fault(detail: String) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("animate.video.export.scene-json"), detail)
}

pub fn handle(payload: &ExportVideoFromDeck, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let stated = payload.scene_json.trim();
    let (filename, program) = if stated.is_empty() || stated == "null" {
        (video_filename_for_deck(doc.snapshot), video_render_program_from_deck(doc.snapshot).map_err(refused)?)
    } else {
        let value = semio_framework_pack_json::parse(stated, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| scene_json_fault(format!("export-video-from-deck scene is not JSON: {error}")))?;
        let scene: PresentationScene = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|error| scene_json_fault(format!("export-video-from-deck scene is not a presentation scene: {error}")))?;
        (video_filename_for_title(&scene.title), video_render_program_from_scene(&scene).map_err(refused)?)
    };
    let bytes = video_program_packed_bytes(&program);
    if bytes > ANIMATE_VIDEO_PROGRAM_MAXIMUM_BYTES {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("animate.video.export.program-too-large"), format!("video program of {bytes} bytes exceeds {ANIMATE_VIDEO_PROGRAM_MAXIMUM_BYTES}")));
    }
    Ok(Emit { effects: vec![Effect::VideoRenderExport { filename, program }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
