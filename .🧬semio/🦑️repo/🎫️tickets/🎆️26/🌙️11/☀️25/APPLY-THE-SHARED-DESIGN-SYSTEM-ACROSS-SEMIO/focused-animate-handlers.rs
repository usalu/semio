//! 🎬 Verify the production Animate editor handlers using the authority supplied at command admission.
use semio_s_artifact_animate_presentation_animate::{
    FigureTileDraft, PresentationSnapshot, default_presentation_snapshot,
    presentation_snapshot_with_tiles, presentation_working_scene,
    editor::animate::{
        commands::{add_tile, export_video_from_deck},
        config::PresentationConfig, support::PresentationDispatchCtx,
    },
    standards::v1::subsets::any::schema::mutations::PresentationMutation,
};
use semio_plugin::{AppOperationContext, ArtifactView, ConfigView, Effect, HistoryView};

fn assigned_tile(snapshot: &PresentationSnapshot, authoring_seed: &str, expected_index: usize) -> FigureTileDraft {
    let history = HistoryView::empty();
    let operation = AppOperationContext {
        app_instance_id: 1,
        parent_document_id: "presentation".into(),
        operation_id: 1,
        generation: 1,
        canonical_base_revision: [0; 32],
        retained: Default::default(),
        authoring_seed: authoring_seed.into(),
    };
    let document = ArtifactView::with_operation(snapshot, &history, operation);
    let config = ConfigView { snapshot: &PresentationConfig::default(), window: None };
    let mut context = PresentationDispatchCtx { selected_ids: Vec::new() };
    let payload = add_tile::AddTile { crop: None };
    let result = add_tile::handle(&payload, &document, &config, &mut context).expect("add tile");
    let [PresentationMutation::CreateTile(created)] = result.artifact_mutations.as_slice() else {
        panic!("expected one create tile mutation");
    };
    assert_eq!(created.index, expected_index);
    assert!(matches!(result.effects.as_slice(), [Effect::ReplayShellCommand { .. }]));
    created.tile.clone()
}

#[test]
fn production_handlers_create_replay_stable_ids_and_export_the_window() {
    let start = default_presentation_snapshot();
    let (source, _) = presentation_working_scene(&start);
    let first = assigned_tile(&start, "actor-clock:first", 0);
    assert_eq!(first.id, assigned_tile(&start, "actor-clock:first", 0).id);
    let first_snapshot = presentation_snapshot_with_tiles(&source, std::slice::from_ref(&first));
    let second = assigned_tile(&first_snapshot, "actor-clock:second", 1);
    assert_ne!(first.id, second.id);
    let deck = presentation_snapshot_with_tiles(&source, &[first, second]);
    let history = HistoryView::empty();
    let document = ArtifactView::new(&deck, &history);
    let config = ConfigView { snapshot: &PresentationConfig::default(), window: None };
    let mut context = PresentationDispatchCtx { selected_ids: Vec::new() };
    let payload = export_video_from_deck::ExportVideoFromDeck { scene_json: String::new() };
    let output = export_video_from_deck::handle(&payload, &document, &config, &mut context).expect("export video");
    assert!(output.artifact_mutations.is_empty());
    let [Effect::VideoRenderExport { program, .. }] = output.effects.as_slice() else {
        panic!("expected one video export");
    };
    assert_eq!(program.validate(), Ok(()));
    assert_eq!((program.width, program.height, program.fps, program.frame_count()), (1280, 720, 15, 105));
    assert_eq!(program.scenes.len(), 3);
    assert_eq!(program.background.map(|channel| (channel * 255.0).round() as u8), [6, 26, 31, 255]);
}
