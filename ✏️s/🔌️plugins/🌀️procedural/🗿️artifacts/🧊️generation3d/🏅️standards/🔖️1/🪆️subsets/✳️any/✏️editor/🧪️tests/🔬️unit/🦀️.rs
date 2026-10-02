use super::*;
use crate::app_fixture::{self as context, app_with_registry};
use semio_framework_plugin::{EditorApp, PluginApp};

const GENERATION3D_MEASURED_CONTRIBUTIONS_PACK_CHARS: usize = 272_089;

#[test]
fn tessellate_transfer_unit_fits_the_declared_response_wire_bound() {
    let maximum = semio_framework_os_flow::mesh::tessellate_envelope_maximum_bytes();
    assert!(maximum <= GENERATION3D_FLOW_EVAL_RAW_BYTES, "one tessellate step envelope is at most {maximum} bytes but the declared wire bound is {GENERATION3D_FLOW_EVAL_RAW_BYTES}");
    assert!(maximum > GENERATION3D_RETAINED_RAW_BYTES, "a transfer unit that still fits the gesture quota needs no route of its own");
    assert_eq!(generation3d_flow_eval_contract().max_raw_wire_bytes, GENERATION3D_FLOW_EVAL_RAW_BYTES, "the registered contract and the factory-side wire cap are one bound");
    assert_eq!(generation3d_bounded_contract().max_raw_wire_bytes, GENERATION3D_RETAINED_RAW_BYTES, "the gesture quota stays exactly where it was");
}

#[test]
fn command_ids_are_unique_and_cover_every_row() {
    let _serial = crate::test_serial::lock();
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(
        ids.len(),
        GENERATION3D_RETAINED_TOOL_IDS.len() + GENERATION3D_FLOW_EVAL_TOOL_IDS.len() + GENERATION3D_CONTRIBUTIONS_TOOL_IDS.len() + GENERATION3D_DOCUMENT_IO_TOOL_IDS.len(),
        "every Generation3dCommand row must be covered by every_command()"
    );
}

#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let _serial = crate::test_serial::lock();
    assert_eq!(GENERATION3D_RETAINED_TOOL_IDS.len(), 33);
    assert_eq!(GENERATION3D_FLOW_EVAL_TOOL_IDS.len(), 5);
    assert_eq!(GENERATION3D_CONTRIBUTIONS_TOOL_IDS.len(), 1);
    assert_eq!(GENERATION3D_DOCUMENT_IO_TOOL_IDS.len(), 3);
    assert_eq!(<Generation3dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 42, "all four factories' proofs, aggregated");
    assert_eq!(Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 33);
    assert_eq!(Generation3dFlowEvalJobFactory::PUBLICATION_CONTRACTS.len(), 5);
    assert_eq!(Generation3dContributionsJobFactory::PUBLICATION_CONTRACTS.len(), 1);
    assert_eq!(Generation3dDocumentIoJobFactory::PUBLICATION_CONTRACTS.len(), 3);
    assert_eq!(generation3d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation3d_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert_eq!(generation3d_flow_eval_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation3d_flow_eval_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert!(GENERATION3D_RETAINED_TOOL_IDS.iter().all(|tool_id| Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
    assert!(GENERATION3D_FLOW_EVAL_TOOL_IDS.iter().all(|tool_id| Generation3dFlowEvalJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
    assert!(GENERATION3D_DOCUMENT_IO_TOOL_IDS.iter().all(|tool_id| Generation3dDocumentIoJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
    let mut sorted_ids = GENERATION3D_RETAINED_TOOL_IDS.to_vec();
    sorted_ids.extend_from_slice(GENERATION3D_FLOW_EVAL_TOOL_IDS);
    sorted_ids.extend_from_slice(GENERATION3D_CONTRIBUTIONS_TOOL_IDS);
    sorted_ids.extend_from_slice(GENERATION3D_DOCUMENT_IO_TOOL_IDS);
    let declared = sorted_ids.len();
    sorted_ids.sort_unstable();
    sorted_ids.dedup();
    assert_eq!(sorted_ids.len(), declared, "a tool id may be owned by exactly one factory");
    for command in every_command() {
        assert!(
            sorted_ids.contains(&command.command_id()),
            "command {} is owned by none of Generation3dBoundedCommandJobFactory, Generation3dFlowEvalJobFactory, Generation3dContributionsJobFactory or Generation3dDocumentIoJobFactory",
            command.command_id()
        );
    }
}

#[test]
fn contributions_route_declares_a_reachable_wire_ceiling() {
    let _serial = crate::test_serial::lock();
    assert_eq!(GENERATION3D_CONTRIBUTIONS_RAW_BYTES, semio_framework::kernel::COMMAND_MAXIMUM_BYTES, "the contributions route is bound by what the paged command ingress can assemble, not by the JSON entry point's body cap");
    assert!(
        GENERATION3D_CONTRIBUTIONS_RAW_BYTES > semio_framework::PUBLIC_INVOCATION_BODY_BYTES,
        "a pack-encoded push does not pass through the JSON body cap, and declaring that cap here refused a 273 136-byte contributions command the transport had already delivered"
    );
    let pack: String = std::iter::repeat_n('x', GENERATION3D_MEASURED_CONTRIBUTIONS_PACK_CHARS).collect();
    let wire = protocol::json::to_json_string(&("setContributions", Some(dsl::DslValue::object([
        ("json".to_string(), dsl::DslValue::String(pack)),
        ("page".to_string(), dsl::DslValue::uint(0)),
        ("pageCount".to_string(), dsl::DslValue::uint(1)),
    ]))));
    assert!(wire.len() <= GENERATION3D_CONTRIBUTIONS_RAW_BYTES, "a scoped pack encodes to {} bytes but the contract declares {GENERATION3D_CONTRIBUTIONS_RAW_BYTES}", wire.len());
    assert!(GENERATION3D_CONTRIBUTIONS_RAW_BYTES > GENERATION3D_RETAINED_RAW_BYTES, "the contributions route exists precisely because the gesture quota cannot carry it");
    assert_eq!(generation3d_contributions_contract().max_raw_wire_bytes, GENERATION3D_CONTRIBUTIONS_RAW_BYTES);
}


/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<Generation3dCommand> {
    vec![
        Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-mushroom-column".into() }),
        Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "extrude".into() }),
        Generation3dCommand::AddWidget(add_widget::AddWidget { neuron_kind: None, format: None, action: None, kind: "inputSlider".into(), x: Some(10.0), y: None }),
        Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }),
        Generation3dCommand::Reorganize(reorganize::Reorganize {}),
        Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0, phase: None, reason: None, window_id: None }),
        Generation3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5, phase: None, reason: None, window_id: None }),
        Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0, phase: None, reason: None, window_id: None }),
        Generation3dCommand::AddGeneration(add_generation::AddGeneration {}),
        Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "generation-1".into() }),
        Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "generation-1".into(), name: "Renamed".into() }),
        Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("generation-1".into()), question_id: "q1".into(), value: dsl::DslValue::float(5.0) }),
        Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 3.0 } }),
        Generation3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "coarse".into() }),
        Generation3dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wireframe".into() }),
        Generation3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
        Generation3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
        Generation3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
        Generation3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
        Generation3dCommand::SetCamera(set_camera::SetCamera { camera: crate::editor::generation3d::config::Generation3dPreviewCamera::default() }),
        Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: "generation-1".into() }),
        Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() }),
        Generation3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash: 7, output_json: "{}".into(), extension_id: String::new(), ok: true, fault_code: String::new(), fault_message: String::new() }),
        Generation3dCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash: 9, output_json: "{}".into() }),
        Generation3dCommand::FlowEvalRelease(flow_eval_release::FlowEvalRelease { window_id: "w1".into(), window_kind_id: "procedural-preview".into() }),
        Generation3dCommand::FlowTessellateCancelResolve(flow_tessellate_cancel_resolve::FlowTessellateCancelResolve { window_id: "w1".into(), window_kind_id: "procedural-preview".into(), output_json: "{\"ok\":true,\"retired\":1}".into(), ok: true }),
        Generation3dCommand::SetContributions(set_contributions::SetContributions { json: "[]".into(), page: 0, page_count: 1 }),
        Generation3dCommand::ImportDocumentRequest(import_document_request::ImportDocumentRequest {}),
        Generation3dCommand::ImportDocument(import_document::ImportDocument { name: "cube.stl".into(), payload: "data:model/stl;base64,aGVsbG8=".into() }),
        Generation3dCommand::ExportDocument(export_document::ExportDocument { format: "stl".into() }),
        Generation3dCommand::CycleShowMode(cycle_show_mode::CycleShowMode {}),
        Generation3dCommand::CycleLodMode(cycle_lod_mode::CycleLodMode {}),
        Generation3dCommand::SelectNextNode(select_next_node::SelectNextNode {}),
        Generation3dCommand::SelectPreviousNode(select_previous_node::SelectPreviousNode {}),
        Generation3dCommand::SelectUpstreamNode(select_upstream_node::SelectUpstreamNode {}),
        Generation3dCommand::SelectDownstreamNode(select_downstream_node::SelectDownstreamNode {}),
        Generation3dCommand::ActivateSelection(activate_selection::ActivateSelection {}),
        Generation3dCommand::EditMeshSelection(edit_mesh_selection::EditMeshSelection { cuts: 1, operation: "extrude".into(), amount: 0.1, dx: 0.0, dy: 0.0, dz: 0.0 }),
        Generation3dCommand::KnifeMeshSelection(knife_mesh_selection::KnifeMeshSelection { start: [0.0, -1.0, 0.0], end: [0.0, 1.0, 0.0] }),
        Generation3dCommand::SetWidgetInput(set_widget_input::SetWidgetInput { widget_id: "shape".into(), channel: "width".into(), value: "2".into(), component: None }),
    ]
}

#[test]
fn document_io_route_declares_a_reachable_wire_ceiling() {
    use crate::editor::generation3d::commands::import_document::GENERATION3D_IMPORT_TOTAL_BYTES;
    let _serial = crate::test_serial::lock();
    assert_eq!(GENERATION3D_IMPORT_TOTAL_BYTES, GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES, "the import budget is one Artifact-lane edit, never a literal");
    assert_eq!(GENERATION3D_DOCUMENT_IO_RAW_BYTES, semio_framework::PUBLIC_INVOCATION_BODY_BYTES);
    assert_eq!(generation3d_document_io_contract().max_raw_wire_bytes, GENERATION3D_DOCUMENT_IO_RAW_BYTES, "the registered contract and the factory-side cap are one bound");
    assert!(GENERATION3D_IMPORT_TOTAL_BYTES < GENERATION3D_DOCUMENT_IO_RAW_BYTES, "one whole import plus its envelope has to fit the route's body");
    assert!(GENERATION3D_IMPORT_TOTAL_BYTES <= semio_framework::kernel::IMPORT_STAGING_MAXIMUM_BYTES, "the framework staging reassembles every import this route admits");
    assert_eq!(generation3d_document_io_contract().shape, semio_framework::ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation3d_document_io_contract().cancellation, semio_framework::ToolCancellationPolicy::PerOperation, "an import run is cancellable per operation");
}

#[test]
fn no_pointer_down_route_survives_the_framework_owned_selection_domain() {
    let definition = create_generation3d_app();
    let json = serde_json::to_string(&definition).expect("app definition json");
    assert!(!json.contains("PointerDown"), "pointer-down routes are framework-owned now");
    assert!(GENERATION3D_RETAINED_TOOL_IDS.iter().chain(GENERATION3D_FLOW_EVAL_TOOL_IDS).all(|id| !id.ends_with("PointerDown")), "a retained tool id outlived its action");
    assert!(definition.interactions.iter().any(|interaction| interaction.id == "graph"), "the graph domain is what replaced them");
}

#[test]
fn mesh_component_action_is_scoped_and_publishes_history_and_selection() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let _serial = crate::test_serial::lock();
    let definition = create_generation3d_app();
    for kind in &definition.window_kinds {
        assert_eq!(kind.actions.iter().any(|action| action.id == "editMeshSelection"), kind.id == edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
        if let Some(action) = kind.actions.iter().find(|action| action.id == "editMeshSelection") {
            let cuts = action.args.iter().find(|arg| arg.id == "cuts").unwrap();
            assert!(matches!(cuts.schema, semio_framework::ArgSchema::Number { min: Some(1.0), max: Some(256.0), step: Some(1.0), integer: true, .. }));
        }
    }
    let geometry = definition.interactions.iter().find(|domain| domain.id == selection::DOMAIN).unwrap();
    assert_eq!(geometry.hierarchy, HierarchyProvider::Flat);
    assert_eq!(geometry.granularities.iter().map(|level| level.id.as_str()).collect::<Vec<_>>(), vec!["object", "vertex", "edge", "face"]);
    let publication = Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == "editMeshSelection").unwrap();
    assert_eq!(publication.lanes, &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction]);
}

fn production_initial_snapshot(label: &str) -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.host_snapshot.schema = label.into();
    for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
        snapshot.host_snapshot.widgets.push(semio_framework_artifact_flow_flow::Widget::InputNote { id: id.into(), text: text.into() });
    }
    snapshot.host_snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
    snapshot.host_snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
    snapshot.host_snapshot.layout.insert("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 });
    snapshot.host_snapshot.layout.insert("clear-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 });
    for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
        snapshot.generation.cold_builder_mut().unwrap().generations.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: id.into(), name: name.into(), values: Default::default() });
    }
    snapshot.generation.cold_builder_mut().unwrap().selected_generation_id = Some("rename-generation".into());
    snapshot
}

fn production_mutations() -> Vec<Generation3dMutation> {
    use crate::standards::v1::subsets::any::schema::mutations::*;
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("production".into()))),
        ),
    );
    vec![
        Generation3dMutation::CreateWidget(create_widget::CreateWidget {
            index: 0,
            widget: semio_framework_artifact_flow_flow::Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true },
        }),
        Generation3dMutation::UpdateWidget(update_widget::UpdateWidget {
            widget: semio_framework_artifact_flow_flow::Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() },
        }),
        Generation3dMutation::DeleteWidget(delete_widget::DeleteWidget { id: "delete-target".into() }),
        Generation3dMutation::ConnectSynapse(connect_synapse::ConnectSynapse {
            index: 0,
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() },
        }),
        Generation3dMutation::UpdateSynapse(update_synapse::UpdateSynapse {
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() },
        }),
        Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: "disconnect-synapse".into() }),
        Generation3dMutation::MoveWidget(move_widget::MoveWidget { id: "move-target".into(), layout: semio_framework_artifact_flow_flow::WidgetLayout { x: 31.0, y: -17.0 } }),
        Generation3dMutation::DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition { id: "clear-target".into() }),
        Generation3dMutation::UpdateCamera(update_camera::UpdateCamera { camera: semio_framework_artifact_flow_flow::CameraJson { x: 9.0, y: 8.0, zoom: 1.75 } }),
        Generation3dMutation::ChangeSchema(change_schema::ChangeSchema { new_schema: "flow.host_snapshot.production-retained".into() }),
        Generation3dMutation::CreateGeneration(create_generation::CreateGeneration { generation: semio_framework_artifact_playbook_playbook::FormGeneration { id: "created-generation".into(), name: "Created".into(), values: Default::default() } }),
        Generation3dMutation::DeleteGeneration(delete_generation::DeleteGeneration { id: "delete-generation".into() }),
        Generation3dMutation::RenameGeneration(rename_generation::RenameGeneration { id: "rename-generation".into(), new_name: "After Rename".into() }),
        Generation3dMutation::ChangeGenerationValue(change_generation_value::ChangeGenerationValue {
            id: "change-generation".into(),
            question_id: "deep-answer".into(),
            new_value: serde_json::json!({"object": {"array": [1.0, false, "retained"]}}).into(),
        }),
    ]
}

fn production_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::new();
    value.try_reserve_exact(bytes.len() * 2).expect("P3 production hex preflight");
    for byte in bytes {
        value.push(char::from(DIGITS[usize::from(byte >> 4)]));
        value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    value
}

fn production_semantic_digest(snapshot: &Generation3dSnapshot) -> [u8; 32] {
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation3d.production-law.semantic");
    digest.observe(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(snapshot));
    digest.finish()
}

fn production_envelope_wire(label: &str) -> (Vec<u8>, Generation3dSnapshot, [u8; 32]) {
    let snapshot = production_initial_snapshot(label);
    let mutations = production_mutations();
    assert_eq!(mutations.len(), 14, "production ingress carries every P3 mutation variant including delete-widget-position");
    let mut mutation_hex = Vec::new();
    mutation_hex.try_reserve_exact(mutations.len()).expect("P3 production mutation owner preflight");
    for mutation in &mutations {
        mutation_hex.push(production_hex(&crate::standards::v1::subsets::any::schema::mutations::binary::encode_op(mutation).expect("P3 production mutation encoding")));
    }
    let mut expected = production_initial_snapshot(label);
    crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_apply_retained_mutations_for_test(&mut expected, &mutations);
    let expected_digest = production_semantic_digest(&expected);
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": GENERATION_3D_SCHEMA,
        "id": "generation3d-production-mounted-law",
        "vcs": {
            "initialSnapshot": production_hex(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode_mounted(&snapshot)),
            "edits": [{
                "id": "generation3d-production-all14-edit",
                "actor": "generation3d-production-law",
                "forwards": mutation_hex,
                "inverse": [],
                "sequenceNumber": 1,
                "startedAt": "1",
                "line": null
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .expect("schema-first P3 production fixture envelope");
    // 🧹️ A `create-widget`/`update-widget` row owns a whole `Widget`, and the seed projection owns
    // an `OrderedMap` layout root — both fail-close on a bare drop, so this fixture retires what it
    // authored instead of letting the scope drop it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    for mutation in mutations {
        mutation.retire_cold();
    }
    snapshot.retire_cold();
    (wire, expected, expected_digest)
}

/// 🔐️ Owns the publication lease `admit_production_envelope` took and releases it even when the law
/// panics before its explicit release. The lease table is a PROCESS-GLOBAL 4-slot
/// `FixedOperationRegistry` (`🧬️schema/🧬️mutations/💾️binary/🦀️.rs:211`), so one leaked slot turns every
/// later law in the same binary into `generation3d-publication.saturated` — an order-dependent red
/// that has nothing to do with what those laws assert
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
struct Generation3dProductionLease {
    handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle,
    released: bool,
}

impl Generation3dProductionLease {
    fn release(&mut self) -> bool {
        if self.released {
            return false;
        }
        self.released = true;
        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_release_publication_authority(self.handle.operation, self.handle.generation)
    }
}

impl Drop for Generation3dProductionLease {
    fn drop(&mut self) {
        self.release();
    }
}

fn admit_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>, wire: &[u8]) -> Generation3dProductionLease {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P3 production ingress credits");
    crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_admit_publication_authority(handle.operation, handle.generation, handle.generation.0, handle.generation.0, handle.generation.0, crate::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationCredits { maximum_items: 8_192, maximum_output_pages: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_CONTROL_CREDITS })
    .expect("P3 production publication authority");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded P3 production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("P3 production envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("P3 production envelope seal"));
    Generation3dProductionLease { handle, released: false }
}

/// 🚿️ Drives ONE production envelope load to its terminal poll.
///
/// 🔎️ It does NOT terminate today, and the turn budget is not why: measured at 300 000 turns with
/// `std::thread::yield_now`, and again with `advance_typed_operation_publication().await` plus a
/// cooperative yield per turn, `poll_artifact_envelope_decode` reads `Pending` on every single turn —
/// so the job exists (a missing one reads `Fault`) and stays in
/// `ActiveArtifactEnvelopeDecodeState::Active`, and no maintenance turn ever reports `Blocked`, so
/// nothing on the ladder names an authority it is waiting for. The stall is inside the decode's own
/// `WorkerJobSession`, not in this driver (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️remaining-suite-reds-2026-09-13.md` §3.3).
fn drive_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..300_000 {
        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0)
            .expect("P3 authority refresh immediately before production maintenance");
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one P3 production maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("P3 production load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("P3 production envelope load did not reach terminal, last decode poll {:?}", app.poll_artifact_envelope_decode(handle));
}

/// 🔐️ LAW: non-empty P3D3 canonical ingress reaches the real VCS maintenance replacement,
/// and accepted, stale, ABA, and displaced stores remain owned until explicit terminal ACK/close.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed() {
    // 🧹️ Registry-backed, never `VcsArtifactApp::new` — this app publishes
    // `bounded_first_step_tool_proofs!`, so a registryless instance faults at construction with
    // `interactive-job.catalog-authority` and its unwind aborts the binary.
    let _serial = crate::publication_authority::lock();
    let mut accepted = app_with_registry().await;
    let base_generation = accepted.artifact_generation_now();
    let (wire, expected, expected_digest) = production_envelope_wire("accepted-production-swap");
    let mut lease = admit_production_envelope(&mut accepted, &wire);
    let handle = lease.handle;
    assert_eq!(drive_production_envelope(&mut accepted, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let snapshot = context::snapshot(&accepted);
    assert_eq!(&snapshot, &expected, "real maintenance must publish all P3 snapshot and all-14 replay fields");
    assert_eq!(production_semantic_digest(&snapshot), expected_digest);
    assert!(snapshot.host_snapshot.layout.contains_key("move-target"));
    assert!(!snapshot.host_snapshot.layout.contains_key("clear-target"), "3D-only delete-widget-position must survive retained replay");
    assert!(accepted.acknowledge_artifact_store_replacement(handle).expect("accepted P3 terminal ACK"));
    assert!(lease.release());
    drop(snapshot);
    expected.retire_cold();

    use crate::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
    for (hostile, expected_code) in [
        (Missing, "generation3d-publication.authority-missing"),
        (WrongOperation, "generation3d-publication.wrong-operation"),
        (WrongGeneration, "generation3d-publication.wrong-generation"),
        (WrongBase, "generation3d-publication.wrong-base"),
        (WrongParent, "generation3d-publication.wrong-parent"),
    ] {
        let mut app = app_with_registry().await;
        let last_valid = context::snapshot(&app);
        let last_valid_digest = production_semantic_digest(&last_valid);
        let base_generation = app.artifact_generation_now();
        let (wire, candidate, _) = production_envelope_wire("rejected-production-candidate");
        candidate.retire_cold();
        let mut lease = admit_production_envelope(&mut app, &wire);
        let handle = lease.handle;
        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_arm_publication_hostile(handle.operation, hostile);
        assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_take_publication_hostile_observed(handle.operation), Some(expected_code));
        assert_eq!(app.artifact_generation_now(), base_generation);
        let retained = context::snapshot(&app);
        assert_eq!(production_semantic_digest(&retained), last_valid_digest);
        assert_eq!(retained, last_valid);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected P3 terminal ACK after candidate retirement"));
        assert!(lease.release());
        drop(retained);
        drop(last_valid);
    }
}
