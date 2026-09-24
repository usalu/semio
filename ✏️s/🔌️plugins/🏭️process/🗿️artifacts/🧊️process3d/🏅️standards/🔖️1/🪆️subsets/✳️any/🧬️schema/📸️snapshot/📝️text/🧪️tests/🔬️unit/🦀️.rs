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

/// 🧱️ The workshop the shipped `concrete-forest` example carries: the generic catalog plus every
/// `ConcreteCatalog` machine, so each of its seven steps names a capability that really exists.
fn concrete_workshop() -> Workshop {
    let mut machines = crate::generic_machines();
    machines.extend(<crate::schema::ConcreteCatalog as crate::MachineCatalog>::machines(&crate::schema::ConcreteCatalog));
    Workshop { machines }
}

/// 🌲️ Every concrete machine applied once to the real hexagonal-cut concrete forest piece
/// (`REFERENCE_SOLID_CONCRETE_FOREST_LEFT`, posed so its 10.8 m × 4.68 m footprint is centred on
/// the origin and it stands on `z = 0`): the wall saw scores the slab full-depth across its
/// width (`x = 5.4`, 0.32 m from the top — the beam underneath keeps both halves joined), the
/// diamond saw relieves that beam from below on the same line, the wire saw severs the right
/// column at `z = 1.2`, the core drill takes a 0.2 m core sample out of the slab, the rotary
/// hammer drills a Ø20 anchor hole, the anchor setter sets an M16 anchor into it (seated 10 mm
/// into the hole floor — a chemical anchor before grouting — and standing 30 mm proud), and the
/// surface grinder takes 5 mm off a pad-sized patch of the slab top. Every tool sits inside the
/// piece's own coordinates (slab `z ∈ [2.735, 3.0]`, beams `[2.285, 2.735]`, columns at
/// `(2.7, 2.338)`/`(8.1, 2.338)`), shifted by the stock pose into world space. Each geometry was
/// probed against the kernel first (ticket `26/09/16/PROCESS-CONCRETE-FOREST-EXAMPLE`): a through
/// core and a coincident-wall anchor still fail the boolean on this piece, which is why the core
/// is blind and the hole is 2 mm wider than the anchor.
fn concrete_forest_scene() -> ProcessWorkingScene {
    let offset = [-5.0, -2.0, 0.0];
    let at = |x: f64, y: f64, z: f64| Pose { position: [x + offset[0], y + offset[1], z + offset[2]], ..Pose::default() };
    let step = |id: &str, label: &str, machine_id: &str, capability_id: &str, measure: ProcessMeasure| ProcessStep { id: id.into(), label: label.into(), enabled: true, origin: Some(StepOrigin { machine_id: machine_id.into(), capability_id: capability_id.into() }), measure };
    ProcessWorkingScene {
        stock: Stock { id: crate::REFERENCE_SOLID_CONCRETE_FOREST_LEFT.into(), label: "Hexagonal Cut Concrete Forest Left".into(), solid: WorkingSolid::Reference { reference_id: crate::REFERENCE_SOLID_CONCRETE_FOREST_LEFT.into() }, pose: Pose { position: offset, ..Pose::default() } },
        steps: vec![
            step("wall-saw-cut", "Wall Saw Slab Cut", "wallSaw", "wallCut", ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.0045, depth: 6.0, height: 0.32 }, pose: at(5.4, 2.3385, 2.84) }),
            step("beam-relief-cut", "Diamond Saw Beam Relief", "diamondSaw", "crosscut", ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.004, depth: 0.5, height: 0.125 }, pose: at(5.4, 2.338, 2.3475) }),
            step("column-wire-cut", "Wire Saw Column Cut", "wireSaw", "wireCut", ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 0.011 }, pose: at(8.1, 2.338, 1.2) }),
            step("core-sample", "Core Sample", "coreDrill", "core", ProcessMeasure::Drill { radius: 0.051, depth: 0.4, pose: at(6.0, 3.6, 3.0) }),
            step("anchor-hole", "Anchor Hole", "rotaryHammer", "anchorHole", ProcessMeasure::Drill { radius: 0.010, depth: 0.16, pose: at(4.5, 1.2, 2.93) }),
            step("anchor-set", "Set Anchor", "anchorSetter", "anchor", ProcessMeasure::Attach { component: WorkingSolid::Cylinder { radius: 0.008, height: 0.19 }, pose: at(4.5, 1.2, 2.935) }),
            step("surface-grind", "Grind Surface Patch", "surfaceGrinder", "grind", ProcessMeasure::Cut { tool: WorkingSolid::Cylinder { radius: 0.125, height: 0.01 }, pose: at(3.5, 3.5, 3.0) }),
        ],
    }
}

/// 🌲️ The shipped concrete forest fixture is exactly `concrete_forest_scene()` on
/// `concrete_workshop()` — regenerate it (below) whenever either changes.
#[semio_framework_async_macros::async_test]
async fn concrete_forest_example_fixture_is_the_authored_scene() {
    let document = parse_dsl(PROCESS_3D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("parse concrete forest example");
    let authored = process_working_scene_to_snapshot(&concrete_forest_scene(), concrete_workshop(), None);
    assert_eq!(document, authored);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

/// 🧱️ Every machine of the concrete catalog is the origin of exactly one step, and every step's
/// origin resolves to a real capability of the example's own workshop.
#[semio_framework_async_macros::async_test]
async fn concrete_forest_example_applies_every_concrete_machine() {
    let document = parse_dsl(PROCESS_3D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("parse concrete forest example");
    let concrete: Vec<String> = <crate::schema::ConcreteCatalog as crate::MachineCatalog>::machines(&crate::schema::ConcreteCatalog).into_iter().map(|machine| machine.id).collect();
    let mut origins: Vec<String> = document.step_payloads.iter().filter_map(|step| step.origin.as_ref()).map(|origin| origin.machine_id.clone()).collect();
    origins.sort();
    let mut concrete_sorted = concrete.clone();
    concrete_sorted.sort();
    assert_eq!(origins, concrete_sorted, "exactly one step per concrete machine");
    for step in &document.step_payloads {
        let origin = step.origin.as_ref().expect("every step has an origin");
        let (_, capability) = crate::schema::inferences::find_capability(&document.workshop, &origin.machine_id, &origin.capability_id).expect("origin names a workshop capability");
        let expected = match capability.recipe.measure_kind() {
            crate::MeasureKind::Cut => "cut",
            crate::MeasureKind::Drill => "drill",
            crate::MeasureKind::Attach => "attach",
        };
        assert_eq!(step.measure.kind_slug(), expected, "step {} measure matches its capability's recipe kind", step.id);
    }
}

/// 🧊️ The kernel replays the whole concrete forest timeline on the real STEP piece: the stock
/// alone measures the piece's true volume (14.0998 m³, the GLB export's own signed volume), and
/// every cut step removes material while the anchor adds it — so no step is a silent no-op.
#[semio_framework_async_macros::async_test]
async fn concrete_forest_example_replays_every_step_on_the_kernel() {
    use crate::schema::inferences::{replay_process, ProcessKernelReplay};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::BrepKernel;
    let scene = crate::process_working_scene_from_snapshot(&parse_dsl(PROCESS_3D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("parse concrete forest example"));
    let mut session = ProcessKernelReplay::new();
    let stock = replay_process(&mut session, &scene, Some(0)).expect("stock replays");
    let stock_volume = session.kernel().volume(&stock).expect("stock volume");
    assert!((stock_volume - 14.0998).abs() < 1e-3, "stock volume {stock_volume}");
    let mut previous = stock_volume;
    for resolved in 1..=scene.steps.len() {
        let handle = replay_process(&mut session, &scene, Some(resolved)).unwrap_or_else(|| panic!("step {} replays", scene.steps[resolved - 1].id));
        let volume = session.kernel().volume(&handle).expect("step volume");
        match scene.steps[resolved - 1].measure {
            ProcessMeasure::Attach { .. } => assert!(volume > previous + 1e-9, "step {} adds material ({previous} → {volume})", scene.steps[resolved - 1].id),
            _ => assert!(volume < previous - 1e-9, "step {} removes material ({previous} → {volume})", scene.steps[resolved - 1].id),
        }
        previous = volume;
    }
}

/// 🌉️ Regenerates every shipped example fixture via the REAL `process_working_scene_to_snapshot`
/// + `print_dsl()`, writing their text to `$PROCESS3D_FIXTURE_OUT` for manual copy into the asset
/// files / `PROCESS_3D_PLATE_EXAMPLE_TEXT`.
/// `#[ignore]`d: a one-shot authoring tool, not part of the regular test run.
#[semio_framework_async_macros::async_test]
#[ignore]
async fn regenerate_example_fixtures() {
    let timber = process_working_scene_to_snapshot(&timber_beam_joinery_scene(), timber_workshop(), None);
    let plate = process_working_scene_to_snapshot(&drilled_plate_scene(), Workshop::default(), Some(2));
    let concrete_forest = process_working_scene_to_snapshot(&concrete_forest_scene(), concrete_workshop(), None);
    let out_dir = std::path::PathBuf::from(std::env::var("PROCESS3D_FIXTURE_OUT").expect("PROCESS3D_FIXTURE_OUT names the output folder"));
    std::fs::write(out_dir.join("timber.dsl.semio"), print_dsl(&timber)).expect("write timber fixture");
    std::fs::write(out_dir.join("plate.dsl.semio"), print_dsl(&plate)).expect("write plate fixture");
    std::fs::write(out_dir.join("concrete-forest.dsl.semio"), print_dsl(&concrete_forest)).expect("write concrete forest fixture");
}
//#endregion 🔖️FixtureRegeneration

