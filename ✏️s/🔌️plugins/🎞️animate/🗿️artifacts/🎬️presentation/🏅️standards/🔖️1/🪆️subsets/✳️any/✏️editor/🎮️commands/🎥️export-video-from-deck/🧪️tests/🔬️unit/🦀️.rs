use super::*;
use crate::editor::animate::commands::seed_grid::SeedGrid;
use crate::editor::animate::unit_tests::context::{dispatch, dispatch_refusal, presentation_app_with_registry};
use crate::editor::animate::PresentationCommand;
use semio_framework_plugin::kernel::{VideoRenderOp, VideoRenderRun};

/// ⚖️ LAW: the UI route runs the export (it is `Migrated`, never refused as `interactive-job.not-ui-safe`) and answers
/// exactly one `VideoRenderExport` the host admits: the deck's own picture as the program's one image, an overview (the
/// whole picture with every tile outlined) for 3 s, then one 2 s slide per tile showing that tile's crop — one scene per
/// slide, well inside the single-effect byte ceiling — and no document mutation.
#[semio_framework_async_macros::async_test]
async fn export_video_from_deck_emits_the_decks_video_program() {
    let mut app = presentation_app_with_registry().await;
    dispatch(&mut app, PresentationCommand::SeedGrid(SeedGrid { rows: 2, columns: 2 })).await;
    let result = dispatch(&mut app, PresentationCommand::ExportVideoFromDeck(ExportVideoFromDeck { scene_json: String::new() })).await;
    assert!(result.mutations.is_empty(), "a video export is a host effect, not a document operation");
    match result.requested_effects.as_slice() {
        [Effect::VideoRenderExport { filename, program }] => {
            assert_eq!(filename, "habitat-67.mp4", "named after the deck's picture");
            assert_eq!(program.validate(), Ok(()));
            assert_eq!((program.width, program.height, program.fps), (1280, 720, 15));
            assert_eq!(program.images.iter().map(|image| image.url.as_str()).collect::<Vec<_>>(), vec![crate::DEMO_FIGURE_SRC]);
            assert_eq!(
                program.timeline,
                vec![VideoRenderRun { scene: 0, frames: 45 }, VideoRenderRun { scene: 1, frames: 30 }, VideoRenderRun { scene: 2, frames: 30 }, VideoRenderRun { scene: 3, frames: 30 }, VideoRenderRun { scene: 4, frames: 30 }]
            );
            assert_eq!(program.duration_milliseconds(), 11_000);
            assert!(
                matches!(program.scenes[0].ops.as_slice(), [VideoRenderOp::Image { crop: [0.0, 0.0, 1.0, 1.0], .. }, VideoRenderOp::Stroke { .. }, VideoRenderOp::Stroke { .. }, VideoRenderOp::Stroke { .. }, VideoRenderOp::Stroke { .. }]),
                "the overview is the whole picture with four tile outlines"
            );
            let crops: Vec<[f64; 4]> = program.scenes[1..]
                .iter()
                .map(|scene| match scene.ops.as_slice() {
                    [VideoRenderOp::Image { crop, .. }] => *crop,
                    other => panic!("a tile slide is one image op, got {other:?}"),
                })
                .collect();
            let area: f64 = crops.iter().map(|crop| crop[2] * crop[3]).sum();
            assert!((area - 1.0).abs() < 1e-9, "a 2x2 grid over the whole frame covers the picture exactly once: {crops:?}");
            assert!(video_program_packed_bytes(program) <= ANIMATE_VIDEO_PROGRAM_MAXIMUM_BYTES);
        }
        other => panic!("expected exactly one video render export, got {other:?}"),
    }
}

/// ⚖️ LAW: a stated scene that references no scene is refused with a named fault — never an empty video, never a
/// silent default; neither is a malformed one.
#[semio_framework_async_macros::async_test]
async fn a_stated_scene_without_scene_hashes_is_refused_by_name() {
    let mut app = presentation_app_with_registry().await;
    let refused = dispatch_refusal(&mut app, PresentationCommand::ExportVideoFromDeck(ExportVideoFromDeck { scene_json: r#"{"schema":"animate.presentation.scene","title":"Empty","sections":[]}"#.into() })).await;
    assert!(format!("{refused:?}").contains("animate.video.export.no-scenes"), "{refused:?}");
    let malformed = dispatch_refusal(&mut app, PresentationCommand::ExportVideoFromDeck(ExportVideoFromDeck { scene_json: "{".into() })).await;
    assert!(format!("{malformed:?}").contains("animate.video.export.scene-json"), "{malformed:?}");
}

/// ⚖️ LAW: a deck whose source is a PDF page (or empty) is refused by name — a host canvas cannot draw it.
#[test]
fn a_deck_without_a_picture_is_refused_by_name() {
    let mut source = crate::default_figure_tile_source();
    source.kind = "pdf".into();
    let deck = crate::presentation_snapshot_with_tiles(&source, &[]);
    assert_eq!(crate::editor::animate::engine::video_render_program_from_deck(&deck).map(|_| ()), Err(crate::editor::animate::engine::PresentationVideoExportError::SourceKind { kind: "pdf".into() }));
}

/// ⚖️ LAW: a deck with no tiles still exports its overview, and the file is named after the picture or the title.
#[test]
fn a_tileless_deck_is_its_overview_and_files_are_named_by_slug() {
    let deck = crate::presentation_snapshot_with_tiles(&crate::default_figure_tile_source(), &[]);
    let program = crate::editor::animate::engine::video_render_program_from_deck(&deck).expect("admitted");
    assert_eq!((program.scenes.len(), program.frame_count()), (1, 45));
    assert_eq!(crate::editor::animate::engine::video_filename_for_deck(&deck), "habitat-67.mp4");
    assert_eq!(video_filename_for_title("Hero Shot #2"), "hero-shot-2.mp4");
    assert_eq!(video_filename_for_title("  "), "animate-presentation.mp4");
}
