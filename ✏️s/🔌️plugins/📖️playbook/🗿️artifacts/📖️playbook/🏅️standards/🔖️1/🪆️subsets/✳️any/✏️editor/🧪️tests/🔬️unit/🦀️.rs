
use super::*;
use crate::editor::playbook::testkit::{dispatch, playbook_app};
use crate::op::AddBlock;
use semio_framework_plugin::testkit;
use semio_framework_plugin::{MediaClass, MediaForm};

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 9, "every PlaybookCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// pre-migration `playbook_protocol::PlaybookCommand`'s own `#[dsl(key = ..)]` attribute) so the wire
/// format stays byte-identical across the migration; see TEMPLATE.md §5.1.
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected = match id {
            "setContributions" => "contributions".to_string(),
            _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
        };
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// 🌐️ The language-neutral job catalog has the same row order through an owned minimal
/// parser and the test-only third-party JSON oracle, then matches the schema-generated command enum.
#[semio_framework_async_macros::async_test]
async fn interactive_job_catalog_matches_owned_and_json_oracle_projections() {
    let source = include_str!("../../🧫️fixtures/🧫️interactive-jobs/🔣️.json");
    let owned = source.lines().filter_map(|line| line.trim().strip_prefix("{\"id\":\"").and_then(|tail| tail.split_once('"')).map(|(id, _)| id.to_string())).collect::<Vec<_>>();
    let oracle = serde_json::from_str::<serde_json::Value>(source)
        .expect("language-neutral interactive job fixture")
        .get("tools")
        .and_then(serde_json::Value::as_array)
        .expect("tool rows")
        .iter()
        .map(|row| row.get("id").and_then(serde_json::Value::as_str).expect("tool id").to_string())
        .collect::<Vec<_>>();
    let generated = every_command().into_iter().map(|command| command.command_id().to_string()).collect::<Vec<_>>();
    assert_eq!(owned, oracle);
    assert_eq!(owned, generated);
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<PlaybookCommand> {
    vec![
        PlaybookCommand::AddStep(add_step::AddStep {}),
        PlaybookCommand::RemoveStep(remove_step::RemoveStep { step_id: "s".into() }),
        PlaybookCommand::MoveStep(move_step::MoveStep { step_id: "s".into(), index: 2 }),
        PlaybookCommand::AddBlock(add_block::AddBlock { kind: "text".into(), step_id: None }),
        PlaybookCommand::RemoveBlock(remove_block::RemoveBlock { step_id: "s".into(), block_id: "b".into() }),
        PlaybookCommand::MoveBlock(move_block::MoveBlock { block_id: "b".into(), from_step_id: "s1".into(), to_step_id: "s2".into(), index: 0 }),
        PlaybookCommand::UpdatePlaybook(update_playbook::UpdatePlaybook { value: "Recipe".into() }),
        PlaybookCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_playbook_play_app()).expect("app definition json");
    assert!(json.contains(PLAYBOOK_PLAY_WINDOW_BUILDER), "window kind missing from the manifest: {json}");
    assert!(json.contains(builder::PLAYBOOK_PLAY_MODE_BUILDER), "mode missing from the manifest");
    assert!(json.contains("text.playbook"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn playbook_play_app_declares_builder_window_only() {
    let definition = create_playbook_play_app();
    assert_eq!(definition.window_kinds.len(), 1);
    assert_eq!(definition.window_kinds[0].id, PLAYBOOK_PLAY_WINDOW_BUILDER);
    assert_eq!(definition.window_kinds[0].body_key, PLAYBOOK_PLAY_BODY_BUILDER);
}
//#endregion 🔖️ManifestSanity

//#region 🔖️Interaction
#[semio_framework_async_macros::async_test]
async fn blocks_interaction_domain_is_declared_topology_pick_only_on_the_builder_window() {
    let definition = create_playbook_play_app();
    let domain = definition.interactions.iter().find(|interaction| interaction.id == PLAYBOOK_INTERACTION_BLOCKS).expect("blocks interaction domain declared");
    assert!(matches!(domain.hierarchy, HierarchyProvider::Topology));
    assert_eq!(domain.selection.methods, vec![SelectionMethod::Pick]);
    assert!(!domain.selection.transitive, "selecting a step must not implicitly select its blocks — no pre-migration code ever did that");
    let builder_window = definition.window_kinds.iter().find(|window| window.id == PLAYBOOK_PLAY_WINDOW_BUILDER).expect("builder window declared");
    assert!(builder_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == PLAYBOOK_INTERACTION_BLOCKS), "builder window must reference the blocks interaction domain");
}

/// 🌳️ `interaction_topology` walks every step and every one of its blocks into a `TopologyNode`, so
/// `validate_state` can prune a deleted step's OR block's id out of a stale selection.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_covers_every_step_and_block() {
    let mut app = playbook_app().await;
    dispatch(&mut app, PlaybookCommand::AddStep(add_step::AddStep {})).await;
    let step_id = app.snapshot().expect("projection").steps()[0].id.clone();
    dispatch(&mut app, PlaybookCommand::AddBlock(add_block::AddBlock { kind: "text".into(), step_id: Some(step_id.clone()) })).await;
    let spec = app.snapshot().expect("projection");
    let block_id = spec.steps().iter().find(|step| step.id == step_id).expect("step present").blocks[0].id.clone();
    let history = semio_framework_plugin::HistoryView::empty();
    let cfg = PlaybookConfig::default();
    let doc = ArtifactView::new(&spec, &history);
    let topology = PlaybookPlayApp::interaction_topology(&doc, &ConfigView { snapshot: &cfg });
    let blocks = topology.domains.get(PLAYBOOK_INTERACTION_BLOCKS).expect("blocks domain present in topology");
    assert!(blocks.ordered.iter().any(|node| node.id == step_id && node.granularity == PLAYBOOK_INTERACTION_GRANULARITY_STEP && node.parent.is_none()));
    assert!(blocks.ordered.iter().any(|node| node.id == block_id && node.granularity == PLAYBOOK_INTERACTION_GRANULARITY_BLOCK && node.parent.as_deref() == Some(step_id.as_str())));
}
//#endregion 🔖️Interaction

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = playbook_app().await;
    testkit::assert_undo_redo_round_trip(&mut app, PlaybookCommand::AddStep(add_step::AddStep {}), |app| app.snapshot().expect("materialize projection").steps().len(), 1, 2).await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::playbook::testkit::render;
    let mut app = playbook_app().await;
    assert!(render(&mut app, "playbook.play.nope").await.contains("Unknown body"));
}

/// 🧪️ The definitional proof: two independent instances start from the same document, apply
/// DISJOINT edits (A adds a step, B adds a block to the pre-existing step), and exchanging operations
/// over a backbone converges both sides onto the same projection — impossible under whole-document
/// `setDocument` snapshots, where one side's write would clobber the other's.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    testkit::assert_two_instances_converge::<EditorApp<PlaybookPlayApp>, (usize, usize)>(
        "mem://playbook-convergence",
        PlaybookCommand::AddStep(add_step::AddStep {}),
        PlaybookCommand::AddBlock(add_block::AddBlock { kind: "number".into(), step_id: None }),
        |app| {
            let projection = app.snapshot().expect("materialize projection");
            let steps = projection.steps();
            (steps.len(), steps[0].blocks.len())
        },
    )
    .await;
}
//#endregion 🔖️CrossCutting

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn playbook_io_declares_the_extra_chapters_in_port_and_its_own_kind() {
    let io = playbook_io();
    assert_eq!(io.artifact.id, "text.playbook");
    let ports = io.all_ports().await;
    let chapters_in = ports.iter().find(|port| port.id == "chapters:in").expect("chapters:in declared");
    assert_eq!(chapters_in.kind_id.as_deref(), Some("text.document"));
}

fn chapter_media(text: &str, title: &str) -> Media {
    let payload = PlaybookChapterPayload { id: "jack".into(), title: title.into(), text: text.into(), language_id: "jack".into() };
    Media { media_type: semio_framework_plugin::MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: protocol::json::to_json_string(&payload) } }
}

#[semio_framework_async_macros::async_test]
async fn import_media_creates_the_imported_step_and_a_note_block() {
    let spec = crate::empty_playbook_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_view = ArtifactView::new(&spec, &history);
    let media = chapter_media("MATCH (a) RETURN a", "Jack Query");
    let emit = PlaybookPlayApp::import_media("chapters:in", &media, &doc_view).expect("import chapters:in");
    assert_eq!(emit.artifact_mutations.len(), 2, "creates the imported step, then the note block");
    assert!(matches!(&emit.artifact_mutations[0], PlaybookMutation::AddStep(payload) if payload.step.id == PLAYBOOK_IMPORTED_STEP_ID));
    match &emit.artifact_mutations[1] {
        PlaybookMutation::AddBlock(AddBlock { step_id, block, .. }) => {
            assert_eq!(step_id, PLAYBOOK_IMPORTED_STEP_ID);
            assert_eq!(block.kind, "note");
            assert_eq!(block.label, "Jack Query");
            assert_eq!(block.text.as_deref(), Some("MATCH (a) RETURN a"));
        }
        other => panic!("expected AddBlock, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn import_media_reuses_the_imported_step_on_a_second_import() {
    let base = crate::empty_playbook_snapshot();
    let mut steps = base.steps();
    steps.push(PlaybookStep { id: PLAYBOOK_IMPORTED_STEP_ID.into(), title: "Imported".into(), description: None, blocks: Vec::new() });
    let spec = crate::playbook_snapshot_with_steps(&base.schema, &base.id, &base.version, base.title.clone(), steps);
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_view = ArtifactView::new(&spec, &history);
    let media = chapter_media("second chapter", "Second");
    let emit = PlaybookPlayApp::import_media("chapters:in", &media, &doc_view).expect("import chapters:in");
    assert_eq!(emit.artifact_mutations.len(), 1, "the imported step already exists, only the block is added");
    assert!(matches!(&emit.artifact_mutations[0], PlaybookMutation::AddBlock(payload) if payload.step_id == PLAYBOOK_IMPORTED_STEP_ID));
}

#[semio_framework_async_macros::async_test]
async fn import_media_rejects_unknown_ports_and_malformed_payloads() {
    let spec = crate::empty_playbook_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_view = ArtifactView::new(&spec, &history);
    assert!(matches!(PlaybookPlayApp::import_media("nonsense:in", &chapter_media("x", "y"), &doc_view), Err(MediaError::NotImplemented)));
    let bad_media = Media { media_type: semio_framework_plugin::MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: "not json".into() } };
    assert!(matches!(PlaybookPlayApp::import_media("chapters:in", &bad_media, &doc_view), Err(MediaError::Payload(..))));
}
//#endregion 🔖️PortTests
