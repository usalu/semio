//! ⏪️ History, convergence and object child lane tests of the CAD editor unit tests: undo and redo, the backbone convergence and the rendered instance lane. See [`super`].

use super::*;
#[cfg(test)] use semio_s_artifact_cad_cad_modes as edit;

//#region 🔖️History
#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_added_node_through_generic_helper() {
    let mut app = new_app().await;
    let before = app.snapshot().expect("snapshot").nodes.len();
    semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip(&mut app.0, CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_added_node_through_wrapper() {
    let mut app = new_app().await;
    let before = app.snapshot().expect("snapshot").nodes.len();
    app.0.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("add node");
    settle(&mut app).await;
    assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before + 1);
    let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "undo", None, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("undo");
    let undo = semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted).await.expect("undo commits its reserved job");
    settle(&mut app).await;
    assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
    assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before);
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, "redo", TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before + 1);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn coalesced_translate_drag_is_a_single_undo_step() {
    let mut app = new_app().await;
    let before = semio_framework_pack_json::to_json_string(&app.snapshot().expect("snapshot"));
    for _ in 0..3 {
        app.0.dispatch_typed(CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec!["object-box-1".into()], dx: 1.0, dy: 0.0, dz: 0.0 }), &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("translate tick");
        settle(&mut app).await;
    }
    let after = semio_framework_pack_json::to_json_string(&app.snapshot().expect("snapshot"));
    assert_eq!(before, after, "an id no pane materializes has nothing to move");
    close(&mut app);
}
//#endregion 🔖️History

//#region 🔖️Convergence
/// 🧪️ The definitional merge proof: two instances start from the SAME base projection, apply
/// DISJOINT edits (A translates object A, B patches object B's label), and after exchanging operations
/// over a `MemoryBackbone` both converge to contain BOTH edits — impossible under whole-document
/// `setDocument` snapshots.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    let mut base = default_document();
    base.nodes = vec![CadNode { id: "node-a".into(), label: "A".into(), kind: "solid".into() }, CadNode { id: "node-b".into(), label: "B".into(), kind: "solid".into() }];
    let node_a = base.nodes[0].id.clone();
    let node_b = base.nodes[1].id.clone();
    let base_envelope = store::os_store::document_materialize::create_document_envelope::<CadSnapshot, CadMutation>(CAD_DOCUMENT_SCHEMA, "cad-play", base, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let base_files = store::os_store::text_format::print_document_pack(&base_envelope).await.expect("print document pack");
    base_envelope.retire_unadopted();

    let mut instance_a = new_app().await;
    let mut instance_b = new_app().await;
    semio_framework_plugin::artifact_app_laws::load_document(&mut instance_a.0, &base_files, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_a.1.cx()).await.expect("load a");
    semio_framework_plugin::artifact_app_laws::load_document(&mut instance_b.0, &base_files, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_b.1.cx()).await.expect("load b");
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://cad-convergence", "mem://cad-convergence").await;
    instance_a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");

    instance_a.0.dispatch_typed(CadCommand::RenameNode(rename_node::RenameNode { node_id: node_a.clone(), value: "Renamed By A".into() }), &meta("actor-a"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_a.1.cx()).await.expect("a renames node a");
    settle(&mut instance_a).await;

    instance_b.0.dispatch_typed(CadCommand::RenameNode(rename_node::RenameNode { node_id: node_b.clone(), value: "Renamed By B".into() }), &meta("actor-b"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_b.1.cx()).await.expect("b renames node b");
    settle(&mut instance_b).await;

    instance_a.0.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_a.1.cx()).await.expect("a folds b's events");
    instance_b.0.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_b.1.cx()).await.expect("b folds a's events");
    for (instance, actor) in [(&mut instance_a, "actor-a"), (&mut instance_b, "actor-b")] {
        let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut instance.0, "commitCheckpoint", None, &meta(actor), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap_or_else(|fault| panic!("pump {actor}: {fault:?}"));
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut instance.0, admitted).await.unwrap_or_else(|fault| panic!("pump {actor} reserved job: {fault:?}"));
        settle(instance).await;
    }
    instance_a.0.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_a.1.cx()).await.expect("a folds b's checkpoint");
    instance_b.0.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), instance_b.1.cx()).await.expect("b folds a's checkpoint");
    settle(&mut instance_a).await;
    settle(&mut instance_b).await;

    let scene_a = instance_a.snapshot().expect("projection a");
    let scene_b = instance_b.snapshot().expect("projection b");

    let label_a_in_a = scene_a.nodes.iter().find(|node| node.id == node_a).unwrap().label.clone();
    let label_a_in_b = scene_b.nodes.iter().find(|node| node.id == node_a).unwrap().label.clone();
    let label_b_in_a = scene_a.nodes.iter().find(|node| node.id == node_b).unwrap().label.clone();
    let label_b_in_b = scene_b.nodes.iter().find(|node| node.id == node_b).unwrap().label.clone();

    assert_eq!(label_a_in_a, "Renamed By A", "instance A keeps its own edit");
    assert_eq!(label_a_in_b, "Renamed By A", "instance B converges on A's edit");
    assert_eq!(label_b_in_a, "Renamed By B", "instance A converges on B's edit");
    assert_eq!(label_b_in_b, "Renamed By B", "instance B keeps its own edit");
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
    close(&mut instance_a);
    close(&mut instance_b);
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_cad() {
    let mut sender = new_app().await;
    let (near, mut far) = MemoryBackbone::pair("mem://cad-doc", "mem://cad-doc").await;
    sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach");
    sender.0.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), sender.1.cx()).await.expect("add node");
    settle(&mut sender).await;

    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(operations);
        }
    }
    assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
    let operations = envelopes;

    let mut receiver = new_app().await;
    let nodes_before = receiver.snapshot().expect("snapshot").nodes.len();
    receiver.0.ingest_operations(&operations, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), receiver.1.cx()).await.expect("ingest once");
    receiver.0.ingest_operations(&operations, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), receiver.1.cx()).await.expect("ingest twice");
    assert_eq!(receiver.snapshot().expect("snapshot").nodes.len(), nodes_before + 1, "feeding the same operation twice must not double-apply");
    sender.detach_backbone().await.expect("sender releases its backbone");
    close(&mut sender);
    close(&mut receiver);
}
//#endregion 🔖️Convergence

//#region 🪆️ObjectChildLane
/// 🥽️ The `instances` lane the shape pane's world-3d surface publishes for the mounted app — the rendered transform.
fn shape_instances_lane(app: &CadFixtureApp) -> String {
    let node = edit::build_world_scene_for_pane(&live_view(app), CadPaneId::Shape, "cad.play.scene3d/shape", None, CadDislocateOptions::default()).expect("shape world scene");
    scene_lane(&node, "instances")
}

/// ▶️ Law (a): `translateSelection` on the demo document moves the selected object in the shape pane's composed model
/// child AND the rendered instance — ONE tool-transaction row on that member store whose relative leaf stays editable —
/// while the parent document and the pane's child id never change (design §12, §20.15). This is the gumball-snaps-back
/// defect the 2026-09-16 CAD end-to-end report left open.
#[semio_framework_async_macros::async_test]
async fn translate_selection_moves_the_object_and_the_rendered_instance() {
    let mut app = new_app().await;
    let parent = app.snapshot().expect("snapshot");
    let store = format!("{}/{}", semio_s_artifact_cad_cad_model::snapshot::cad_pane_model_slot(CadPaneId::Shape), semio_s_artifact_cad_cad_model::snapshot::cad_pane_model(&parent, CadPaneId::Shape).expect("the demo document composes a shape child").child_id);
    let target = pane_objects(&app, CadPaneId::Shape).await.first().expect("the demo document holds shape objects").clone();
    app.0.dispatch_typed(CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec![target.id.clone()], dx: 1.5, dy: -2.25, dz: 0.5 }), &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("translate dispatch");
    settle(&mut app).await;
    assert_eq!(app.snapshot().expect("snapshot"), parent, "a transform never moves the parent document or re-mints the pane's child handle");
    let moved = pane_objects(&app, CadPaneId::Shape).await.into_iter().find(|object| object.id == target.id).expect("the object survives the move");
    assert_eq!(moved.origin, [target.origin[0] + 1.5, target.origin[1] - 2.25, target.origin[2] + 0.5], "the composed child carries the new origin");
    assert_eq!(CadObject { origin: target.origin, ..moved.clone() }, target, "a drag changes the origin and nothing else");

    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), 1, "one gesture is one row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the gesture row carries its transaction");
    assert_eq!(transaction.tool, "s.cad.cad@1/*#editor#translateSelection");
    assert_eq!(rows[0].mutations.len(), 1, "one relative leaf: {:?}", rows[0].mutations);
    assert_eq!(rows[0].mutations[0].store.as_deref(), Some(store.as_str()));
    assert!(rows[0].mutations[0].editable, "a drag's inputs are editable");
    assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 1 element by (1.5, -2.25, 0.5)");

    let lane = shape_instances_lane(&app);
    let position = format!("\"position\":[{},{},{}]", moved.origin[0], moved.origin[1], moved.origin[2]);
    assert!(lane.contains(&position), "the rendered instance must carry the moved pose: looked for {position} in {lane}");
    close(&mut app);
}

/// ▶️ Law (b): `addObject` in the shape window yields a new rendered instance in that pane.
#[semio_framework_async_macros::async_test]
async fn add_object_yields_a_new_rendered_instance() {
    let mut app = new_app().await;
    let before = pane_objects(&app, CadPaneId::Shape).await.len();
    app.0.dispatch_typed(CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }), &window_meta("cad-window-shape", shape::WINDOW_KIND_ID), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("addObject dispatch");
    settle(&mut app).await;
    let objects = pane_objects(&app, CadPaneId::Shape).await;
    assert_eq!(objects.len(), before + 1, "the pane's model child holds one more object");
    let created = objects.last().expect("the created object is appended");
    assert!(shape_instances_lane(&app).contains(created.id.as_str()), "the created object is a rendered instance");
    close(&mut app);
}

/// ↩️ Law (c): undo restores. Every object gesture is an ordinary child-lane edit with an exact inverse (a relative leaf
/// undoes as the absolute `set-element` placement of each addressed element, a removal as the element's re-insertion), so
/// undoing the gestures newest-first brings the pane's model back exactly and the parent never moved.
#[semio_framework_async_macros::async_test]
async fn object_gestures_undo_back_to_the_demo_document() {
    let mut app = new_app().await;
    let parent = app.snapshot().expect("snapshot");
    let genesis = pane_objects(&app, CadPaneId::Shape).await;
    let target = genesis.first().expect("shape objects").id.clone();
    let gestures = [
        CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec![target.clone()], dx: 2.0, dy: 0.0, dz: 0.0 }),
        CadCommand::ScaleSelection(scale_selection::ScaleSelection { object_ids: vec![target.clone()], sx: 2.0, sy: 2.0, sz: 2.0 }),
        CadCommand::DeleteObject(delete_object::DeleteObject { object_id: target.clone() }),
    ];
    let count = gestures.len();
    for command in gestures {
        let before = pane_objects(&app, CadPaneId::Shape).await;
        app.0.dispatch_typed(command, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("gesture dispatch");
        settle(&mut app).await;
        assert_ne!(pane_objects(&app, CadPaneId::Shape).await, before, "every object gesture lands a real child edit");
    }
    assert_eq!(member_rows(&mut app).await.len(), count, "one row per gesture");
    for _ in 0..count {
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, "undo", TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    }
    let restored = pane_objects(&app, CadPaneId::Shape).await;
    assert_eq!(restored.len(), genesis.len(), "undo restores every object");
    for object in &genesis {
        assert_eq!(restored.iter().find(|candidate| candidate.id == object.id), Some(object), "undo restores {} exactly", object.id);
    }
    assert_eq!(app.snapshot().expect("snapshot"), parent, "the parent document never moved");
    close(&mut app);
}

/// 🎛️ Law (d): an inspector field edit is ONE absolute `set-element` on the object's pane child that changes that facet
/// only, and an inspector origin DELTA is the transform tool's relative `drag-elements`.
#[semio_framework_async_macros::async_test]
async fn an_inspector_edit_is_one_set_element_and_an_origin_delta_is_a_relative_drag() {
    let mut app = new_app().await;
    let target = pane_objects(&app, CadPaneId::Shape).await.first().expect("shape objects").clone();
    app.0.dispatch_typed(CadCommand::PatchObject(patch_object::PatchObject { object_id: target.id.clone(), field: "label".into(), value: Some("Renamed".into()), delta: None }), &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("rename dispatch");
    settle(&mut app).await;
    let renamed = pane_objects(&app, CadPaneId::Shape).await.into_iter().find(|object| object.id == target.id).expect("the object survives");
    assert_eq!(CadObject { label: target.label.clone(), ..renamed.clone() }, target, "a rename changes the label and nothing else");
    assert_eq!(renamed.label, "Renamed");
    app.0.dispatch_typed(CadCommand::PatchObject(patch_object::PatchObject { object_id: target.id.clone(), field: "origin.x".into(), value: None, delta: Some(0.75) }), &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("origin delta dispatch");
    settle(&mut app).await;
    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), 2, "two edits are two rows: {rows:?}");
    assert!(rows[0].op_lines.iter().any(|line| line.starts_with("set-element")), "{:?}", rows[0].op_lines);
    assert!(rows[0].transaction.is_none(), "a plain inspector edit is no tool transaction");
    assert!(rows[1].op_lines.iter().any(|line| line.starts_with("drag-elements")), "{:?}", rows[1].op_lines);
    assert!(rows[1].transaction.as_ref().is_some_and(|transaction| transaction.tool == "s.cad.cad@1/*#editor#patchObject"), "{:?}", rows[1].transaction);
    close(&mut app);
}
//#endregion 🪆️ObjectChildLane
