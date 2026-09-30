use super::*;
use semio_framework_plugin::ArtifactViewer;

#[test]
fn view_tessellate_envelope_fits_the_declared_wire_bound() {
    let maximum = semio_framework_os_flow::mesh::tessellate_envelope_maximum_bytes();
    assert!(maximum <= GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES, "one tessellate step envelope is at most {maximum} bytes but the viewer's declared wire bound is {GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES}");
    assert!(maximum > GENERATION3D_VIEW_RAW_BYTES, "a transfer unit that still fits the gesture quota needs no route of its own");
    assert_eq!(generation3d_view_flow_eval_contract().max_raw_wire_bytes, GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES, "the registered contract and the factory-side wire cap are one bound");
}

#[test]
fn every_viewer_tool_id_is_declared_in_all_four_tables() {
    let commands: std::collections::BTreeSet<&str> = Generation3dViewCommand::TOOL_JOB_IDS.iter().copied().collect();
    let retained: std::collections::BTreeSet<&str> =
        GENERATION3D_VIEW_TOOL_IDS
            .iter()
            .chain(GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.iter())
            .chain(GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.iter())
            .chain(GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.iter())
            .chain(GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS.iter())
            .copied()
            .collect();
    let published: std::collections::BTreeSet<&str> = <Generation3dViewBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .chain(<Generation3dViewContributionsJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewExampleJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewFlowEvalJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewDocumentIoJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .map(|contract| contract.tool_id)
        .collect();
    let proved: std::collections::BTreeSet<String> = <Generation3dViewer as ArtifactViewer>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id().to_string()).collect();
    let proved: std::collections::BTreeSet<&str> = proved.iter().map(String::as_str).collect();
    assert_eq!(commands, retained, "command enum ids and retained tool ids must be a bijection");
    assert_eq!(commands, published, "every tool needs an exact publication contract");
    assert_eq!(commands, proved, "every tool needs an exact bounded first-step proof");
}

#[test]
fn no_viewer_tool_publishes_on_the_artifact_lane() {
    for contract in <Generation3dViewBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .chain(<Generation3dViewContributionsJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewExampleJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewFlowEvalJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        // 📤️ The io route is in this law deliberately: `exportDocument` is the one io direction a
        // viewer may own, and the reason it may is that it publishes on NO store lane at all.
        .chain(<Generation3dViewDocumentIoJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
    {
        assert!(!contract.lanes.is_empty(), "{} declares no publication lane", contract.tool_id);
        assert!(!contract.lanes.contains(&ArtifactToolPublicationLane::Artifact), "viewer tool {} must never publish on the artifact lane", contract.tool_id);
        assert!(!contract.lanes.contains(&ArtifactToolPublicationLane::Draft), "viewer tool {} must never publish on the draft lane", contract.tool_id);
    }
}

#[test]
fn every_declared_viewer_action_is_migrated() {
    let def = create_generation3d_viewer();
    let window = def.window_kinds.iter().find(|window| window.id == preview::WINDOW_KIND_ID).expect("the viewer declares its preview window kind");
    for tool_id in GENERATION3D_VIEW_TOOL_IDS.iter().chain(GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.iter()).chain(GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS.iter()) {
        let action = semio_framework::window_kind_actions(&def, window).into_iter().find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("tool {tool_id} has no declared action"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "action {tool_id} is not Migrated");
        assert_eq!(action.kind, ActionKind::View, "a viewer action must be a View action");
    }
}

#[test]
fn the_viewer_exports_the_example_it_is_showing_not_the_opened_document() {
    let opened = crate::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
    for example_id in ["hexagonal-mushroom-column", "rectangle-extrude-volume"] {
        let config = Generation3dViewConfig { active_example_id: Some(example_id.to_string()), ..Default::default() };
        let viewed = super::Generation3dViewedDocument::resolve(&opened, &config);
        let widgets = viewed.snapshot().host_snapshot.widgets.len();
        assert!(widgets > 0, "{example_id}: the viewed document must carry the example's widgets, not the opened document's none");
        viewed.retire();
    }
    let none = Generation3dViewConfig { active_example_id: None, ..Default::default() };
    let viewed = super::Generation3dViewedDocument::resolve(&opened, &none);
    assert_eq!(viewed.snapshot().host_snapshot.widgets.len(), opened.host_snapshot.widgets.len(), "with no example picked the viewer exports the opened document itself");
    viewed.retire();
    opened.retire_cold();
}
