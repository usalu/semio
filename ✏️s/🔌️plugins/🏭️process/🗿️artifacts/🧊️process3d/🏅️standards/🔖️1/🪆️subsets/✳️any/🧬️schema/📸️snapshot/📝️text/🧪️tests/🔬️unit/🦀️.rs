use super::*;

use crate::{
    empty_process3d_snapshot, process_working_scene_to_snapshot, Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessMeasure, ProcessStep, ProcessWorkingScene, StepOrigin, Stock, StockQuantity, WorkingSolid, Workshop,
    WorkshopMachine,
};

fn cut_step(id: &str) -> ProcessStep {
    ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
}

fn drill_step(id: &str) -> ProcessStep {
    ProcessStep {
        id: id.into(),
        label: "Drill".into(),
        enabled: true,
        origin: Some(StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() }),
        measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() },
    }
}

fn attach_step(id: &str) -> ProcessStep {
    ProcessStep {
        id: id.into(),
        label: "Attach".into(),
        enabled: false,
        origin: None,
        measure: ProcessMeasure::Attach { component: WorkingSolid::Sphere { radius: 0.05 }, pose: Pose { position: [0.1, -0.2, 0.3], axis: [0.0, 1.0, 0.0], angle: 1.2 } },
    }
}

fn imported_mesh_stock() -> Stock {
    Stock { id: "stock".into(), label: "Imported GLB".into(), solid: WorkingSolid::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }, pose: Pose::default() }
}

fn circular_saw_machine() -> WorkshopMachine {
    WorkshopMachine {
        id: "circularSaw".into(),
        label: "Circular Saw".into(),
        icon_id: "scissors".into(),
        catalog_id: Some("wood".into()),
        capabilities: vec![Capability {
            id: "crosscut".into(),
            label: "Crosscut".into(),
            icon_id: "scissors".into(),
            recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
            parameters: vec![CapabilityParameter { id: "bladeDiameter".into(), label: "Blade Diameter".into(), value: 0.184 }, CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.002 }],
            rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "bladeDiameter".into(), margin: 0.0 }],
        }],
    }
}

/// 📜️ A document exercising every `WorkingSolid`/`ProcessMeasure` shape, both `origin` states,
/// and a non-default workshop machine (3-deep nesting), so the DSL round trip covers the full
/// grammar. Real composed children minted from a literal `ProcessWorkingScene`
/// (`process_working_scene_to_snapshot`), never a bare/hand-built handle.
fn sample_document() -> Process3dSnapshot {
    let scene = ProcessWorkingScene {
        stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: WorkingSolid::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose { position: [0.0, 0.0, 0.12], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
        steps: vec![cut_step("cut-1"), drill_step("drill-1"), attach_step("attach-1")],
    };
    process_working_scene_to_snapshot(&scene, Workshop { machines: vec![circular_saw_machine()] }, Some(2))
}

#[semio_framework_async_macros::async_test]
async fn process3d_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&sample_document());
    store::os_store::test_support::assert_dsl_round_trip(&empty_process3d_snapshot());
}

#[semio_framework_async_macros::async_test]
async fn process3d_dsl_round_trips_imported_solid_shapes() {
    let scene = ProcessWorkingScene {
        stock: imported_mesh_stock(),
        steps: vec![
            cut_step("cut-1"),
            drill_step("drill-1"),
            attach_step("attach-1"),
            ProcessStep { id: "imported-tool".into(), label: "Imported Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::ImportedSolid { solid_handle: "solid-7".into() }, pose: Pose::default() } },
        ],
    };
    let document = process_working_scene_to_snapshot(&scene, Workshop { machines: vec![circular_saw_machine()] }, Some(2));
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn process3d_dsl_round_trips_with_no_resolved_cursor() {
    let mut document = sample_document();
    document.resolved_up_to = None;
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn timber_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(PROCESS_3D_TIMBER_EXAMPLE_TEXT).expect("parse timber example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn drilled_plate_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(PROCESS_3D_PLATE_EXAMPLE_TEXT).expect("parse drilled plate example");
    assert_eq!(document.resolved_up_to, Some(2));
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

//#region 🔖️FixtureRegeneration
/// 🏭️ The exact workshop the shipped `timber-beam-joinery` fixture carries today (generic
/// catalog + `WoodCatalog`) — reused verbatim so regeneration never drifts the catalog content.
fn timber_workshop() -> Workshop {
    let mut machines = crate::generic_machines();
    machines.extend(<crate::schema::WoodCatalog as crate::MachineCatalog>::machines(&crate::schema::WoodCatalog));
    Workshop { machines }
}

/// 🪵️ A believable four-step joinery sequence on a real timber beam: crosscut to length, cut a
/// lap joint, drill a dowel hole, then plug it — each against a machine/capability that actually
/// exists in `timber_workshop()`. Every tool pose is inside the stock on purpose: `box_prim` is
/// ORIGIN-CENTRED (`🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs:415-421`), so this 3.0×0.2×0.3 beam
/// posed at `z = 0.15` occupies `x ∈ [-1.5, 1.5]`, `y ∈ [-0.1, 0.1]`, `z ∈ [0.0, 0.3]` — a tool
/// outside that box is a silent no-op and the step would leave the rendered mesh unchanged.
fn timber_beam_joinery_scene() -> ProcessWorkingScene {
    let beam_top_z = 0.30;
    ProcessWorkingScene {
        stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: WorkingSolid::Box { width: 3.0, depth: 0.2, height: 0.3 }, pose: Pose { position: [0.0, 0.0, 0.15], ..Pose::default() } },
        steps: vec![
            ProcessStep {
                id: "crosscut".into(),
                label: "Crosscut To Length".into(),
                enabled: true,
                origin: Some(StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() }),
                measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.02, depth: 0.3, height: 0.4 }, pose: Pose { position: [1.35, 0.0, 0.15], ..Pose::default() } },
            },
            ProcessStep {
                id: "lap-joint-cut".into(),
                label: "Cut Lap Joint".into(),
                enabled: true,
                origin: Some(StepOrigin { machine_id: "cncRouter".into(), capability_id: "pocket".into() }),
                measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.3, depth: 0.2, height: 0.08 }, pose: Pose { position: [0.6, 0.0, beam_top_z - 0.04], ..Pose::default() } },
            },
            ProcessStep {
                id: "dowel-drill".into(),
                label: "Drill Dowel Hole".into(),
                enabled: true,
                origin: Some(StepOrigin { machine_id: "drillPress".into(), capability_id: "bore".into() }),
                measure: ProcessMeasure::Drill { radius: 0.004, depth: 0.08, pose: Pose { position: [0.6, 0.05, beam_top_z - 0.10], ..Pose::default() } },
            },
            ProcessStep {
                id: "dowel-attach".into(),
                label: "Insert Dowel".into(),
                enabled: true,
                origin: Some(StepOrigin { machine_id: "dowelJig".into(), capability_id: "dowel".into() }),
                measure: ProcessMeasure::Attach { component: WorkingSolid::Cylinder { radius: 0.004, height: 0.12 }, pose: Pose { position: [0.6, 0.05, beam_top_z - 0.08], ..Pose::default() } },
            },
        ],
    }
}

/// 🧱️ A four-hole bolt pattern drilled clear through a real plate, against the default generic
/// workshop's `drill`/`drill` capability (the only capability the plate fixture's shipped
/// `Workshop::default()` carries). The plate is origin-centred too, so the pattern sits inside
/// `x ∈ [-0.6, 0.6]`, `y ∈ [-0.4, 0.4]`, and each bore is deeper than the 0.02 plate so it cuts
/// through instead of meeting its faces coincidentally.
fn drilled_plate_scene() -> ProcessWorkingScene {
    let z = 0.01;
    let hole = |id: &str, label: &str, x: f64, y: f64| ProcessStep {
        id: id.into(),
        label: label.into(),
        enabled: true,
        origin: Some(StepOrigin { machine_id: "drill".into(), capability_id: "drill".into() }),
        measure: ProcessMeasure::Drill { radius: 0.008, depth: 0.06, pose: Pose { position: [x, y, z], ..Pose::default() } },
    };
    ProcessWorkingScene {
        stock: Stock { id: "plate".into(), label: "Plate".into(), solid: WorkingSolid::Box { width: 1.2, depth: 0.8, height: 0.02 }, pose: Pose { position: [0.0, 0.0, z], ..Pose::default() } },
        steps: vec![hole("drill-1", "Drill Corner 1", -0.45, -0.28), hole("drill-2", "Drill Corner 2", 0.45, -0.28), hole("drill-3", "Drill Corner 3", -0.45, 0.28), hole("drill-4", "Drill Corner 4", 0.45, 0.28)],
    }
}

/// 🌉️ Regenerates both shipped example fixtures via the REAL `process_working_scene_to_snapshot`
/// + `print_dsl()` (never hand-transcribed hex), writing their text to the ticket's
/// `🗑️generated` folder for manual copy into the asset file / `PROCESS_3D_PLATE_EXAMPLE_TEXT`.
/// `#[ignore]`d: a one-shot authoring tool, not part of the regular test run.
#[semio_framework_async_macros::async_test]
#[ignore]
async fn regenerate_example_fixtures() {
    let timber = process_working_scene_to_snapshot(&timber_beam_joinery_scene(), timber_workshop(), None);
    let plate = process_working_scene_to_snapshot(&drilled_plate_scene(), Workshop::default(), Some(2));
    let out_dir = std::path::Path::new("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/PROCESS-END-TO-END/🗑️generated");
    std::fs::write(out_dir.join("timber.dsl.semio"), print_dsl(&timber)).expect("write timber fixture");
    std::fs::write(out_dir.join("plate.dsl.semio"), print_dsl(&plate)).expect("write plate fixture");
}
//#endregion 🔖️FixtureRegeneration
