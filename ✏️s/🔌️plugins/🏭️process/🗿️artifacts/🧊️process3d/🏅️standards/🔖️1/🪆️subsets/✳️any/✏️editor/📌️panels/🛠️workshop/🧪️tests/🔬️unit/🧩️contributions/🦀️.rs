//! 🧩️ Published Process3d extensions installed and exercised through the mounted workshop.

use crate::context::{self, published_a_document_mutation, settled_action};
use semio_framework::DslValue;
use semio_s_artifact_process_process3d_kit::{catalog_machine, installable_contributions, PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES, PROCESS_3D_PLAY_APP_ID};
use semio_s_artifact_process_process3d_model::snapshot::{Process3dSnapshot, ProcessMeasure, WorkingSolid, WorkshopMachine};
use semio_s_artifact_process_process3d_panels::workshop::PROCESS_3D_PLAY_BODY_WORKSHOP;

fn shipped_extension_inputs() -> (String, Vec<(&'static str, Vec<WorkshopMachine>)>) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|path| path.join("✏️s/🔌️plugins/🏭️process/🧩️extensions").is_dir()).expect("workspace root");
    let mut entries = Vec::new();
    let mut expected = Vec::new();
    for (folder, id) in [("🪵️wood", "wood"), ("🔩️metal", "metal"), ("🧱️concrete", "concrete"), ("🤖️robotic", "robotic")] {
        let path = root.join("✏️s/🔌️plugins/🏭️process/🧩️extensions").join(folder).join("🧩️component/🔣️.json");
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("extension descriptor")).expect("descriptor JSON");
        let manifest = &descriptor["manifest"];
        let topics = manifest["topicContributions"].as_array().expect("extension topics");
        assert_eq!(topics.len(), 1);
        assert_eq!(topics[0]["topic"], "process.machines");
        let payload = &topics[0]["payload"];
        assert_eq!(payload["appId"], PROCESS_3D_PLAY_APP_ID);
        assert_eq!(payload["moduleId"], id);
        let machines_json = payload["machinesJson"].as_str().expect("machines JSON");
        let oracle: serde_json::Value = serde_json::from_str(machines_json).expect("machine JSON oracle");
        let machines: Vec<WorkshopMachine> = semio_framework_pack_json::from_json_str(machines_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("machine payload");
        assert_eq!(oracle.as_array().expect("machine array").len(), machines.len());
        assert!(!machines.is_empty());
        expected.push((id, machines));
        entries.push(serde_json::json!({"pluginId": manifest["pluginId"], "topicContribution": topics[0]}));
    }
    (serde_json::to_string(&entries).expect("contribution pack"), expected)
}

/// 🧩️ The published extension descriptors deliver the same machines the editor installs.
#[test]
fn shipped_extension_descriptors_deliver_installable_machines() {
    let (pack, expected) = shipped_extension_inputs();
    let distilled = installable_contributions(&pack, PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES);
    assert_eq!(semio_framework::parse_contributions(&distilled).len(), 4);
    let mut app = context::unseeded_app_with_registry();
    let receipt = context::install_contributions(&mut app, pack);
    assert!(receipt.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Config));
    let stock_id = app.snapshot().expect("document").stock_id.clone();
    for (field, value) in [("width", 0.2), ("depth", 0.2), ("height", 0.05)] {
        let args = DslValue::object([("target".into(), DslValue::String(stock_id.clone())), ("field".into(), DslValue::String(field.into())), ("value".into(), DslValue::float(value))]);
        settled_action(&mut app, "patchInspector", Some(&args));
    }
    assert_eq!(app.snapshot().expect("document").stock_payload.solid, WorkingSolid::Box { width: 0.2, depth: 0.2, height: 0.05 });
    for (catalog_id, machines) in expected {
        let available = machines.iter().find(|machine| !machine.capabilities.is_empty()).expect("machine with a capability").clone();
        for mut machine in machines {
            let installed = catalog_machine(&distilled, catalog_id, &machine.id).expect("extension machine is installable");
            machine.catalog_id = Some(catalog_id.into());
            assert_eq!(installed, machine, "{catalog_id} payload differs from the installed machine");
        }
        if app.snapshot().expect("document").workshop.machines.iter().any(|existing| existing.id == available.id) {
            let args = DslValue::object([("id".into(), DslValue::String(available.id.clone()))]);
            assert!(published_a_document_mutation(&settled_action(&mut app, "removeWorkshopMachine", Some(&args)).1));
        }
        let args = DslValue::object([("catalogId".into(), DslValue::String(catalog_id.into())), ("machineId".into(), DslValue::String(available.id.clone()))]);
        assert!(published_a_document_mutation(&settled_action(&mut app, "addWorkshopMachine", Some(&args)).1));
        let mut expected_machine = available.clone();
        expected_machine.catalog_id = Some(catalog_id.into());
        assert_eq!(app.snapshot().expect("document").workshop.machines.iter().find(|machine| machine.id == available.id), Some(&expected_machine));
        let capability = &available.capabilities[0];
        let args = DslValue::object([("machineId".into(), DslValue::String(available.id.clone())), ("capabilityId".into(), DslValue::String(capability.id.clone()))]);
        if catalog_id == "wood" {
            let height = |value| DslValue::object([("target".into(), DslValue::String(stock_id.clone())), ("field".into(), DslValue::String("height".into())), ("value".into(), DslValue::float(value))]);
            assert!(published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&height(0.2))).1));
            let before = app.snapshot().expect("document").step_payloads.len();
            let meta = semio_plugin::artifact_app_laws::meta("local");
            let admission = ::semio_framework_async::poll::resolve_ready(semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "addStep", Some(args.clone()), &meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close));
            let outcome = match admission {
                Err(fault) => format!("{fault:?}"),
                Ok(admitted) => match ::semio_framework_async::poll::resolve_ready(semio_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted, app.1.cx())) {
                    Err(fault) => format!("{fault:?}"),
                    Ok(result) => {
                        let settled = ::semio_framework_async::poll::resolve_ready(semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()));
                        format!("{result:?} {:?}", settled.err())
                    }
                },
            };
            assert!(outcome.contains("process3d.capability.stock-limit"), "expected stock-limit refusal: {outcome}");
            assert_eq!(app.snapshot().expect("document").step_payloads.len(), before);
            assert!(published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&height(0.05))).1));
        }
        assert!(published_a_document_mutation(&settled_action(&mut app, "addStep", Some(&args)).1));
        let snapshot = app.snapshot().expect("document");
        let (index, step) = snapshot.step_payloads.iter().enumerate().find(|(_, step)| step.origin.as_ref().is_some_and(|origin| origin.machine_id == available.id && origin.capability_id == capability.id)).expect("catalog step");
        let mut expected = step.clone();
        let ProcessMeasure::Cut { tool, .. } = &mut expected.measure else { panic!("expected a catalog cut") };
        let (field, size) = match tool {
            WorkingSolid::Cylinder { height, .. } => ("height", height),
            WorkingSolid::Box { width, .. } => ("width", width),
            _ => panic!("expected an analytic catalog tool"),
        };
        *size *= 0.75;
        let edit = DslValue::object([("target".into(), DslValue::String(step.id.clone())), ("field".into(), DslValue::String(field.into())), ("value".into(), DslValue::float(*size))]);
        assert!(published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&edit)).1));
        assert_eq!(app.snapshot().expect("edited document").step_payloads[index], expected);
        assert!(!published_a_document_mutation(&settled_action(&mut app, "patchInspector", Some(&edit)).1));
    }
    let before = app.snapshot().expect("document before uninstall");
    context::install_contributions(&mut app, "[]".into());
    assert!(catalog_machine("[]", "wood", "circularSaw").is_none());
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_WORKSHOP);
    assert!(!rendered.contains("process3d-play-workshop.catalog.wood"), "the removed provider no longer appears in the mounted catalog");
    let after = app.snapshot().expect("document after uninstall");
    assert_eq!(after.workshop, before.workshop);
    assert_eq!(after.step_payloads, before.step_payloads);
    let args = DslValue::object([("id".into(), DslValue::String("circularSaw".into()))]);
    assert!(published_a_document_mutation(&settled_action(&mut app, "removeWorkshopMachine", Some(&args)).1));
    let args = DslValue::object([("catalogId".into(), DslValue::String("wood".into())), ("machineId".into(), DslValue::String("circularSaw".into()))]);
    assert!(!published_a_document_mutation(&settled_action(&mut app, "addWorkshopMachine", Some(&args)).1));
    assert!(!app.snapshot().expect("document").workshop.machines.iter().any(|machine| machine.id == "circularSaw"));
}

/// 🧮️ Capabilities from distinct published extensions compose one saved workpiece.
#[test]
fn shipped_extensions_replay_mixed_capabilities_on_one_workpiece() {
    use pack::snapshot::ArtifactPack;
    use semio_s_artifact_process_process3d_inferences::processed_volume;
    use semio_s_artifact_process_process3d_model::snapshot::process_working_scene_from_snapshot;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/mixed-capability-replay.json")).expect("mixed capability scenario");
    let (pack, _) = shipped_extension_inputs();
    let mut app = context::unseeded_app_with_registry();
    let receipt = context::install_contributions(&mut app, pack);
    assert!(receipt.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Config));

    let mutate = |app: &mut context::Process3dApp, action, args| {
        assert!(published_a_document_mutation(&settled_action(app, action, Some(&args)).1), "{action} did not publish");
    };
    let patch = |app: &mut context::Process3dApp, target: &str, field: &str, value| {
        mutate(app, "patchInspector", DslValue::object([
            ("target".into(), DslValue::String(target.into())),
            ("field".into(), DslValue::String(field.into())),
            ("value".into(), DslValue::float(value)),
        ]));
    };
    let initial = app.snapshot().expect("initial document");
    for step in &initial.step_payloads {
        mutate(&mut app, "removeStep", DslValue::object([("id".into(), DslValue::String(step.id.clone()))]));
    }
    let dimensions: [f64; 3] = serde_json::from_value(fixture["stockMetres"].clone()).expect("box dimensions");
    for (field, value) in ["width", "depth", "height"].into_iter().zip(dimensions) {
        patch(&mut app, &initial.stock_id, field, value);
    }
    patch(&mut app, &initial.stock_id, "posZ", 0.0);

    let steps = fixture["steps"].as_array().expect("extension steps");
    for (index, step) in steps.iter().enumerate() {
        let catalog = step["catalog"].as_str().expect("catalog id");
        let machine = step["machine"].as_str().expect("machine id");
        let capability = step["capability"].as_str().expect("capability id");
        if app.snapshot().expect("workshop").workshop.machines.iter().any(|existing| existing.id == machine) {
            mutate(&mut app, "removeWorkshopMachine", DslValue::object([("id".into(), DslValue::String(machine.into()))]));
        }
        mutate(&mut app, "addWorkshopMachine", DslValue::object([
            ("catalogId".into(), DslValue::String(catalog.into())),
            ("machineId".into(), DslValue::String(machine.into())),
        ]));
        mutate(&mut app, "addStep", DslValue::object([
            ("machineId".into(), DslValue::String(machine.into())),
            ("capabilityId".into(), DslValue::String(capability.into())),
        ]));
        let snapshot = app.snapshot().expect("steps after catalogue action");
        assert_eq!(snapshot.step_payloads.len(), index + 1);
        let id = &snapshot.step_payloads[index].id;
        let position: [f64; 3] = serde_json::from_value(step["positionMetres"].clone()).expect("step position");
        for (field, value) in ["posX", "posY", "posZ"].into_iter().zip(position).filter(|(_, value)| *value != 0.0) {
            patch(&mut app, id, field, value);
        }
    }

    let tolerance = fixture["volumeToleranceCubicMetres"].as_f64().expect("volume tolerance");
    let assert_volume = |scene, prefix, expected: &serde_json::Value| {
        let volume = processed_volume(scene, prefix).expect("workpiece replay");
        assert!((volume - expected.as_f64().expect("expected volume")).abs() < tolerance, "prefix {prefix:?} volume {volume}, expected {expected}");
    };
    let snapshot = app.snapshot().expect("saved mixed capability document");
    let reopened = Process3dSnapshot::decode_pack(&snapshot.encode_pack()).expect("reopened mixed capability document");
    let scene = process_working_scene_from_snapshot(&reopened);
    assert_eq!(scene.steps.len(), steps.len());
    for (step, expected) in scene.steps.iter().zip(steps) {
        let machine = reopened.workshop.machines.iter().find(|machine| Some(machine.id.as_str()) == expected["machine"].as_str()).expect("saved workshop machine");
        assert_eq!(machine.catalog_id.as_deref(), expected["catalog"].as_str());
        let origin = step.origin.as_ref().expect("saved machine and capability");
        assert_eq!(Some(origin.machine_id.as_str()), expected["machine"].as_str());
        assert_eq!(Some(origin.capability_id.as_str()), expected["capability"].as_str());
        assert_eq!(Some(step.measure.kind_slug()), expected["measure"].as_str());
        assert_eq!(step.measure.pose().position, serde_json::from_value::<[f64; 3]>(expected["positionMetres"].clone()).expect("expected position"));
    }
    for (prefix, expected) in fixture["prefixesCubicMetres"].as_array().expect("prefix volumes").iter().enumerate() {
        assert_volume(&scene, Some(prefix), expected);
    }

    let edit = &fixture["drillEdit"];
    let radius = edit["radiusMetres"].as_f64().expect("edited radius");
    patch(&mut app, &scene.steps[1].id, "radius", radius);
    context::install_contributions(&mut app, "[]".into());
    let edited = app.snapshot().expect("document after removing extension contributions");
    let restored = Process3dSnapshot::decode_pack(&edited.encode_pack()).expect("reopened edited document");
    let scene = process_working_scene_from_snapshot(&restored);
    assert!(matches!(scene.steps[1].measure, ProcessMeasure::Drill { radius: stored, .. } if stored == radius));
    assert_volume(&scene, None, &edit["volumeCubicMetres"]);
}
