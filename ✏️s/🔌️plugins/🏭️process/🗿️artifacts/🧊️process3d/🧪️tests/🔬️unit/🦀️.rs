use super::*;

/// ⏩️ Insertion advances the viewer on the resulting timeline, preserving the unlimited cursor.
#[test]
fn inserted_step_cursor_matches_neutral_json_oracle() {
    use crate::editor::process3d::{commands::step::insert_step_emit, config::{Process3dConfig, Process3dConfigMutation}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cursorInsertions"].as_array().unwrap() {
        let count = case["stepCount"].as_u64().unwrap() as usize;
        let cursor = case["cursor"].as_u64().map(|value| value as usize);
        let next = case["next"].as_u64().map(|value| value as usize);
        let step = |id: String| ProcessStep { id: id.clone(), label: id, enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.1, depth: 0.2, pose: Pose::default() } };
        let mut snapshot = empty_process3d_snapshot();
        snapshot.step_payloads = (0..count).map(|index| step(index.to_string())).collect();
        let config = Process3dConfig { resolved_up_to: cursor, ..Process3dConfig::default() };
        let emit = insert_step_emit(&snapshot, &config, step("inserted".into()));
        assert!(matches!(&emit.artifact_mutations[..], [Process3dMutation::CreateStep(payload)] if payload.index == case["index"].as_u64().unwrap() as usize));
        let expected = if next == cursor { Vec::new() } else { vec![Process3dConfigMutation::SetCursor { value: next }] };
        assert_eq!(emit.config_mutations, expected);
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&emit.config_mutations)).unwrap();
        let oracle = if next == cursor { serde_json::json!([]) } else { serde_json::json!([{ "SetCursor": { "value": case["next"] } }]) };
        assert_eq!(actual, oracle);
        eprintln!("[DEBUG] Process cursor insertion count={count} cursor={cursor:?} next={next:?}");
    }
}

/// 🧬️ Literal variant identities agree across lazy borrowed metadata, owned metadata, and neutral cases.
#[test]
fn tagged_field_borrowed_schema_matches_neutral_cases() {
    use semio_framework_dsl_record::{BorrowedDslField, BorrowedShape, DslField, Shape};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("neutral fixture");
    let first_party = semio_framework_pack_json::parse(include_str!("🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("first-party fixture");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_string(&first_party)).unwrap(), fixture);
    for (key, borrowed, owned) in [
        ("measureRecipe", <MeasureRecipe as BorrowedDslField>::SHAPE, <MeasureRecipe as DslField>::shape()),
        ("workingSolid", <WorkingSolid as BorrowedDslField>::SHAPE, <WorkingSolid as DslField>::shape()),
        ("processMeasure", <ProcessMeasure as BorrowedDslField>::SHAPE, <ProcessMeasure as DslField>::shape()),
    ] {
        let BorrowedShape::Statements(borrowed) = borrowed else { panic!("tagged borrowed field") };
        let Shape::Statements(owned) = owned else { panic!("tagged owned field") };
        let borrowed_names: Vec<_> = borrowed.iter().map(|(name, _)| *name).collect();
        let owned_names: Vec<_> = owned.iter().map(|(name, _)| name.as_str()).collect();
        let expected: Vec<_> = fixture["variants"][key].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
        assert_eq!(borrowed_names, expected);
        assert_eq!(owned_names, expected);
        eprintln!("[DEBUG] Process3D borrowed schema {key}: {} literal variants", expected.len());
    }
}

/// 📦️ The current binary mutation frame agrees with independently encoded LEB128 UTF-8 lengths.
#[test]
fn delete_step_binary_matches_neutral_leb128_oracle() {
    use crate::standards::v1::subsets::any::{io::binary::mutations::{decode_op, encode_op}, schema::mutations::delete_step::DeleteStep};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["deleteSteps"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let mutation = Process3dMutation::DeleteStep(DeleteStep { id: id.into() });
        let actual = encode_op(&mutation).expect("current binary mutation");
        let mut oracle = vec![2, 1];
        leb128::write::unsigned(&mut oracle, id.len() as u64).unwrap();
        oracle.extend_from_slice(id.as_bytes());
        assert_eq!(actual, oracle);
        assert_eq!(actual.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), case["hex"].as_str().unwrap());
        assert_eq!(decode_op(&actual).expect("current decode"), mutation);
        let mut extra = actual.clone();
        extra.push(0);
        assert!(decode_op(&extra).is_err());
        eprintln!("[DEBUG] Process3D binary delete-step {} UTF-8 bytes: {} frame bytes", id.len(), actual.len());
    }
}

/// 🔤️ The legacy enum-typed `export_formats`/`import_formats` are retired in favor of the
/// string-id `export_stdio_kinds`/`import_stdio_kinds` peers below — both stay empty.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_declares_the_expected_media_surface() {
    let kind = artifact_kind();
    assert_eq!(kind.id, "3d.process");
    assert_eq!(kind.schema, PROCESS_3D_SCHEMA);
    assert!(kind.export_formats.is_empty());
    assert!(kind.import_formats.is_empty());
    assert_eq!(kind.export_stdio_kinds, kind.import_stdio_kinds);
    assert_eq!(kind.export_stdio_kinds, vec!["stdio.json".to_string()], "the machined solid is a child artifact the kernel host resolves; no serializer can write it");
}

//#region 🔖️WorkshopTests
fn sample_capability() -> Capability {
    Capability {
        id: "crosscut".into(),
        label: "Crosscut".into(),
        icon_id: "scissors".into(),
        recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
        parameters: vec![CapabilityParameter { id: "bladeDiameter".into(), label: "Blade Diameter".into(), value: 0.184 }, CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.002 }],
        rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "bladeDiameter".into(), margin: 0.0 }, CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "bladeDiameter".into(), margin: 0.05 }],
    }
}

fn sample_workshop() -> Workshop {
    Workshop { machines: vec![WorkshopMachine { id: "circularSaw".into(), label: "Circular Saw".into(), icon_id: "scissors".into(), catalog_id: Some("wood".into()), capabilities: vec![sample_capability()] }] }
}

/// 📜️ The document's deepest new nesting (workshop → machines → capabilities → parameters/rules,
/// 3 `Vec` levels deep) must round-trip through the DSL text codec — the riskiest new grammar surface.
#[semio_framework_async_macros::async_test]
async fn workshop_dsl_round_trips_through_document() {
    let snapshot = Process3dSnapshot { workshop: sample_workshop(), ..empty_process3d_snapshot() };
    store::os_store::test_support::assert_dsl_round_trip(&snapshot);
}

/// 🔤️ `ToValue`/`FromValue` (`semio_framework_value_derive`) over `semio_framework_os_kernel::
/// json::{to_json_string, from_json_str}` — the `serde_json::to_string`/`from_str` replacement
/// every `process-extension-*` catalog crate's `machinesJson` payload routes through instead
/// (ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).
#[semio_framework_async_macros::async_test]
async fn workshop_machines_round_trip_through_the_first_party_json_bridge() {
    let machines = sample_workshop().machines;
    let text = semio_framework_pack_json::to_json_string(&machines);
    let parsed: Vec<WorkshopMachine> = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("decode");
    assert_eq!(parsed, machines);
}

#[semio_framework_async_macros::async_test]
async fn default_workshop_has_the_three_generic_machines() {
    let workshop = Workshop::default();
    let ids: Vec<&str> = workshop.machines.iter().map(|machine| machine.id.as_str()).collect();
    assert_eq!(ids, ["saw", "drill", "attacher"]);
    assert!(workshop.machines.iter().all(|machine| machine.catalog_id.is_none()));
}

#[semio_framework_async_macros::async_test]
async fn workshop_machine_patch_apply_only_changes_the_named_fields() {
    let mut machine = WorkshopMachine { id: "circularSaw".into(), label: "Circular Saw".into(), icon_id: "scissors".into(), catalog_id: Some("wood".into()), capabilities: vec![sample_capability()] };
    let original = machine.clone();
    let patch = WorkshopMachinePatch { label: Some("Big Saw".into()), icon_id: None, capabilities: None };
    machine.apply_patch(&patch);
    assert_eq!(machine.label, "Big Saw");
    assert_eq!(machine.capabilities, original.capabilities);
    assert_eq!(machine.icon_id, original.icon_id);
}
//#endregion 🔖️WorkshopTests

//#region 🔖️WorkingSceneTests
/// ⚖️ Real round-trip law: every field a `WorkingSolid` box/cylinder/sphere carries survives a
/// full `ProcessWorkingScene` → `Process3dSnapshot` → back-through-the-brep-content path
/// (content-level, not handle-level — resolving `working_solid_from_brep_snapshot` is the
/// documented gap, so this asserts on the minted `SemioBrepSnapshot` content directly).
#[semio_framework_async_macros::async_test]
async fn box_working_solid_mints_a_real_six_face_one_solid_brep() {
    let content = brep_snapshot_for_working_solid(&WorkingSolid::Box { width: 2.0, depth: 3.0, height: 4.0 });
    assert_eq!(content.vertices.len(), 8);
    assert_eq!(content.edges.len(), 12);
    assert_eq!(content.faces.len(), 6);
    assert_eq!(content.solids.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn cylinder_working_solid_mints_a_real_three_face_brep() {
    let content = brep_snapshot_for_working_solid(&WorkingSolid::Cylinder { radius: 1.0, height: 2.0 });
    assert_eq!(content.faces.len(), 3);
    assert_eq!(content.solids.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn sphere_working_solid_mints_a_real_one_face_untrimmed_brep() {
    let content = brep_snapshot_for_working_solid(&WorkingSolid::Sphere { radius: 1.5 });
    assert_eq!(content.faces.len(), 1);
    assert!(content.loops[0].edges.is_empty(), "sphere's single face loop must be untrimmed");
}

#[semio_framework_async_macros::async_test]
async fn brep_snapshot_for_working_solid_round_trips_pack_and_dsl() {
    for solid in [WorkingSolid::Box { width: 1.0, depth: 2.0, height: 3.0 }, WorkingSolid::Cylinder { radius: 1.0, height: 2.0 }, WorkingSolid::Sphere { radius: 1.0 }] {
        let content = brep_snapshot_for_working_solid(&solid);
        let packed = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&content);
        assert_eq!(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), content);
    }
}

fn sample_step(id: &str, measure: ProcessMeasure) -> ProcessStep {
    ProcessStep { id: id.into(), label: format!("Step {id}"), enabled: true, origin: Some(StepOrigin { machine_id: "saw".into(), capability_id: "cut".into() }), measure }
}

/// ⚖️ Flow round-trip law: `enabled`/`origin`/pose/measure-scalar fields survive
/// `ProcessStep → FlowNode → ProcessStep` exactly (the `WorkingSolid` tool/component is the
/// documented gap, asserted separately below).
#[semio_framework_async_macros::async_test]
async fn process_step_flow_round_trips_scalar_fields() {
    let step = sample_step("s1", ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.5 } });
    let node = flow_node_from_process_step(&step, 0, None);
    let back = process_step_from_flow_node(&node);
    assert_eq!(back.id, step.id);
    assert_eq!(back.label, step.label);
    assert_eq!(back.enabled, step.enabled);
    assert_eq!(back.origin, step.origin);
    assert_eq!(back.measure, step.measure);
}

#[semio_framework_async_macros::async_test]
async fn flow_snapshot_for_steps_is_a_real_linear_chain() {
    let steps = vec![sample_step("a", ProcessMeasure::Drill { radius: 0.1, depth: 0.1, pose: Pose::default() }), sample_step("b", ProcessMeasure::Drill { radius: 0.1, depth: 0.1, pose: Pose::default() })];
    let flow = flow_snapshot_for_steps(&steps, &Default::default());
    assert_eq!(flow.nodes.len(), 2);
    assert_eq!(flow.edges.len(), 1);
    assert_eq!(flow.edges[0].from.node, "a");
    assert_eq!(flow.edges[0].to.node, "b");
    let recovered = process_steps_from_flow_snapshot(&flow);
    assert_eq!(recovered.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), vec!["a".to_string(), "b".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn empty_process3d_snapshot_mints_real_stock_and_steps_children() {
    let snapshot = empty_process3d_snapshot();
    assert!(!snapshot.stock_solid.child_id.is_empty());
    assert!(!snapshot.steps.child_id.is_empty());
    assert!(snapshot.tool_solids.is_empty());
}
//#endregion 🔖️WorkingSceneTests
