//! 🔬️ Mounted gbXML completion, cancellation and exact render revision laws.
use semio_framework::RetainedCloneGrant;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_plugin::{ArtifactView, EditorApp, InteractionState, NoConfig};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_s_artifact_bim_model_bim::editor::bim::{BimCommand, BimModelApp};
use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::{ExportModel, stem};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::empty_instance;

const CLOSE_STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 20,
    maximum_copy_bytes: 1 << 26,
    maximum_capacity_bytes: 1 << 26,
    maximum_release_bytes: 1 << 26,
    maximum_depth: 128,
};
const CLOSE_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_copy_bytes: 1 << 16, ..CLOSE_STOCK };

fn close_projection(owner: &semio_plugin::ArtifactInstanceOperationOwnerHandle, caller: &mut OriginalJobFixtureCaller) {
    for _ in 0..(1 << 18) {
        if owner.close_terminal_is_empty().expect("inference state") {
            return;
        }
        caller.retirement_turn(CLOSE_GRANT, |cx| Ok(owner.close_inference_step(cx, CLOSE_GRANT)))
            .expect("funded inference retirement").expect("the projection closes");
    }
    panic!("the inference owner did not retire, next demand {:?}", owner.inference_retirement_demands(CLOSE_GRANT.maximum_copy_bytes));
}

#[semio_framework_async_macros::async_test]
async fn mounted_gbxml_publishes_original_pages_for_the_exact_committed_render() {
    use semio_framework_job::RevisionId;
    use semio_plugin::__value::ValueRefusalKind;
    use semio_plugin::app::ArtifactOutputChunks;
    use semio_plugin::retained_command::ArtifactCommandSnapshotRead;
    use semio_plugin::{AppOperationContext, AppRenderOperationContext, ChildContentView, HistoryView, MountedOwnerPolicyV1};
    use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::GbxmlExportWork;
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_view_inference;
    use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🌿️gbxml/🖥️mounted/🔣️.json")).unwrap();
    let snapshot = decode_model_snapshot_json(include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🌡️energy-envelope/🏠️box/📸️snapshot/🔣️.json")).unwrap();
    let expected = include_bytes!("../../../../../🧪️tests/🧫️fixtures/🌿️gbxml/🏠️box/gbxml.xml");
    let history = HistoryView::empty();
    let command = BimCommand::ExportModel(ExportModel { format: fixture["arguments"]["format"].as_str().unwrap().into(), pressed: None });
    assert_eq!(command.command_id(), fixture["action"].as_str().unwrap());
    let output_limit = fixture["maximumOutputBytes"].as_u64().unwrap() as usize;
    assert_eq!(output_limit, ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES);
    let grant = RetainedCloneGrant { maximum_items: 1 << 16, maximum_copy_bytes: 4096, maximum_capacity_bytes: 1 << 17, maximum_release_bytes: 1 << 17, maximum_depth: 128 };
    let owner = empty_instance();
    let render = AppRenderOperationContext {
        app_instance_id: 1, base_revision: RevisionId(1), canonical_base_revision: [7; 32],
        generation: Generation(1), mounted_policy: MountedOwnerPolicyV1::INTERACTIVE_COMPONENT,
    };
    let doc = ArtifactView::with_render_context(&snapshot, &history, ChildContentView::EMPTY, render, "bim-test", None).await.with_committed_render_snapshot(true);
    with_view_inference(Some(&owner), &doc, |_| ());
    for (number, scenario) in fixture["scenarios"].as_array().unwrap().iter().enumerate() {
        let revision = [scenario["revisionByte"].as_u64().unwrap() as u8; 32];
        let cancel_after = scenario["cancelAfterWritingTurns"].as_u64();
        let operation = AppOperationContext {
            app_instance_id: 1, parent_document_id: "bim-model".into(), operation_id: number as u64 + 1, generation: 1,
            canonical_base_revision: revision, retained: grant, authoring_seed: String::new(),
        };
        let input = ArtifactCommandInputs::<EditorApp<BimModelApp>> {
            command: &command, snapshot: &snapshot, snapshot_owner: None,
            snapshot_read: ArtifactCommandSnapshotRead { original: None, operation: &operation, source: None },
            config: &NoConfig::default(), history: &history, interaction: &InteractionState::default(),
            hover: &Default::default(), context: None, operation: &operation,
        };
        let mut work = GbxmlExportWork::new(command.command_id(), owner.clone());
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(operation.operation_id), Generation(1), CLOSE_STOCK, || Some(1), 65_536, 1 << 25);
        let mut output = None;
        assert!(caller.turn(grant, |cx| ArtifactOutputChunks::original_birth_admit(output_limit, &mut output, cx)).unwrap());
        assert!(caller.turn(grant, |cx| work.receive_original_output_chunks(&mut output, cx)).unwrap());
        assert!(output.is_none());
        let mut writing = 0;
        let mut stopped = false;
        for _ in 0..100_000 {
            let result = caller.turn(grant, |cx| Ok(work.step(&input, cx))).expect("funded gbXML turn");
            match result {
                Ok(ArtifactCommandWorkStep::Progress { stage, .. }) if stage == "bim-export-encode" => {
                    writing += 1;
                    if cancel_after == Some(writing) { caller.cancel().cancel().await; }
                }
                Ok(ArtifactCommandWorkStep::Progress { .. } | ArtifactCommandWorkStep::Pending { .. }) => (),
                Ok(ArtifactCommandWorkStep::CompleteDownload { download, .. }) => {
                    assert_eq!(revision, [7; 32]);
                    assert_eq!(scenario["result"], "download");
                    assert!(writing > 1);
                    assert_eq!(download.filename, format!("{}{}", stem(&snapshot.project.name), fixture["requiredExtension"].as_str().unwrap()));
                    assert_eq!(download.mime_type, fixture["requiredMimeType"].as_str().unwrap());
                    assert!(download.encoding.is_none());
                    let mut bytes = Vec::new();
                    while let Some(page) = download.chunks.take_chunk().unwrap() {
                        assert!(page.len() as u64 <= fixture["outputPageBytes"].as_u64().unwrap());
                        bytes.extend(page);
                    }
                    assert_eq!(bytes.as_slice(), expected);
                    stopped = true;
                    break;
                }
                Err(error) => {
                    use semio_framework_job::OriginalTurnPrimary;
                    let OriginalTurnPrimary::Value(error) = error.primary else { panic!("gbXML refusal must preserve its value fault"); };
                    if let Some(cancel_after) = cancel_after {
                        assert_eq!(scenario["result"], "canceled");
                        assert_eq!(error.kind, ValueRefusalKind::Canceled);
                        assert_eq!(writing, cancel_after);
                    } else {
                        assert_eq!(scenario["result"], "unsupportedOwner");
                        assert_eq!(revision, [8; 32]);
                        assert_eq!(error.kind, ValueRefusalKind::UnsupportedOwner);
                        assert_eq!(writing, 0);
                    }
                    stopped = true;
                    break;
                }
                _ => panic!("mounted gbXML must return the original segmented download"),
            }
        }
        assert!(stopped);
        work.begin_close();
        for _ in 0..(1 << 18) {
            if work.terminal_is_empty() { break; }
            caller.retirement_turn(CLOSE_GRANT, |cx| work.close_step(cx)).expect("funded gbXML close");
        }
        assert!(work.terminal_is_empty(), "gbXML close stalled in {} with next copy demand {:?}", scenario["name"], work.next_close_copy_byte_demand());
        if revision == [8; 32] { close_projection(&owner, &mut caller); }
        caller.retire(CLOSE_STOCK);
    }
}
