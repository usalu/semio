//! 📜️ Process3d artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::process3d::Process3dSnapshot;

/// 🗄️ The timber-beam-joinery example fixture, handcrafted in this artifact's DSL (`store::ArtifactDsl`).
pub const PROCESS_3D_TIMBER_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 🗄️ The drilled-plate example fixture — regenerated (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
/// wave 4's fixture-regeneration technique: real `process_working_scene_to_snapshot` + `print_dsl()`
/// output, never hand-transcribed) for the hand-rolled hex/bracket text codec's current shape.
pub const PROCESS_3D_PLATE_EXAMPLE_TEXT: &str = "semio process.process3d.dsl v1
workshop=7b226d616368696e6573223a5b7b226964223a22736177222c226c6162656c223a2247656e6572696320536177222c2269636f6e4964223a2273636973736f7273222c226361706162696c6974696573223a5b7b226964223a22637574222c226c6162656c223a22437574222c2269636f6e4964223a2273636973736f7273222c22726563697065223a7b22726563697065223a22626c616465437574222c226b657266223a226b657266222c226c656e677468223a226c656e677468222c226465707468223a226465707468227d2c22706172616d6574657273223a5b7b226964223a226b657266222c226c6162656c223a224b657266222c2276616c7565223a302e30357d2c7b226964223a226c656e677468222c226c6162656c223a224c656e677468222c2276616c7565223a302e357d2c7b226964223a226465707468222c226c6162656c223a224465707468222c2276616c7565223a302e357d5d2c2272756c6573223a5b5d7d5d7d2c7b226964223a226472696c6c222c226c6162656c223a2247656e65726963204472696c6c222c2269636f6e4964223a22636972636c652d646f74222c226361706162696c6974696573223a5b7b226964223a226472696c6c222c226c6162656c223a224472696c6c222c2269636f6e4964223a22636972636c652d646f74222c22726563697065223a7b22726563697065223a22626f72654472696c6c222c22726164697573223a22726164697573222c226465707468223a226465707468227d2c22706172616d6574657273223a5b7b226964223a22726164697573222c226c6162656c223a22526164697573222c2276616c7565223a302e30357d2c7b226964223a226465707468222c226c6162656c223a224465707468222c2276616c7565223a302e337d5d2c2272756c6573223a5b5d7d5d7d2c7b226964223a226174746163686572222c226c6162656c223a2247656e65726963204174746163686572222c2269636f6e4964223a22706c7573222c226361706162696c6974696573223a5b7b226964223a22617474616368222c226c6162656c223a22417474616368222c2269636f6e4964223a22706c7573222c22726563697065223a7b22726563697065223a2263796c696e646572417474616368222c22726164697573223a22726164697573222c226c656e677468223a226c656e677468227d2c22706172616d6574657273223a5b7b226964223a22726164697573222c226c6162656c223a22526164697573222c2276616c7565223a302e30337d2c7b226964223a226c656e677468222c226c6162656c223a224c656e677468222c2276616c7565223a302e327d5d2c2272756c6573223a5b5d7d5d7d5d7d
stockId=706c617465
stockLabel=506c617465
stockPose=7b22706f736974696f6e223a5b302e302c302e302c302e305d2c2261786973223a5b302e302c302e302c312e305d2c22616e676c65223a302e307d
stockSolid=[73746f636b2d627265702d36396364393036383537363934646333,70726f636573732d73746f636b2d6272657021732e737464696f2e73656d696f4076312f62726570]
steps=[73746570732d666c6f772d65643466396264663562653539656139,70726f636573732d73746570732d666c6f7721732e737464696f2e73656d696f4076312f666c6f77]
toolSolids=[]
resolvedUpTo=32";

/// 📖️ Parses `.process3d` DSL text into a `Process3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Process3dSnapshot, store::TextError> {
    <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Process3dSnapshot` back to `.process3d` DSL text.
pub fn print_dsl(document: &Process3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    use crate::artifacts::process3d::{
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
        let mut machines = crate::artifacts::process3d::generic_machines();
        machines.extend(<crate::artifacts::process3d::schema::WoodCatalog as crate::artifacts::process3d::MachineCatalog>::machines(&crate::artifacts::process3d::schema::WoodCatalog));
        Workshop { machines }
    }

    /// 🪵️ A believable four-step joinery sequence on a real timber beam: crosscut to length, cut a
    /// lap joint, drill a dowel hole, then plug it — each against a machine/capability that actually
    /// exists in `timber_workshop()`.
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
                    measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.02, depth: 0.3, height: 0.4 }, pose: Pose { position: [2.7, 0.0, 0.15], ..Pose::default() } },
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
                    measure: ProcessMeasure::Drill { radius: 0.004, depth: 0.04, pose: Pose { position: [0.6, 0.05, beam_top_z - 0.08], ..Pose::default() } },
                },
                ProcessStep {
                    id: "dowel-attach".into(),
                    label: "Insert Dowel".into(),
                    enabled: true,
                    origin: Some(StepOrigin { machine_id: "dowelJig".into(), capability_id: "dowel".into() }),
                    measure: ProcessMeasure::Attach { component: WorkingSolid::Cylinder { radius: 0.004, height: 0.04 }, pose: Pose { position: [0.6, 0.05, beam_top_z - 0.08], ..Pose::default() } },
                },
            ],
        }
    }

    /// 🧱️ A four-hole bolt pattern drilled through a real plate, against the default generic
    /// workshop's `drill`/`drill` capability (the only capability the plate fixture's shipped
    /// `Workshop::default()` carries).
    fn drilled_plate_scene() -> ProcessWorkingScene {
        let z = 0.01;
        let hole = |id: &str, label: &str, x: f64, y: f64| ProcessStep {
            id: id.into(),
            label: label.into(),
            enabled: true,
            origin: Some(StepOrigin { machine_id: "drill".into(), capability_id: "drill".into() }),
            measure: ProcessMeasure::Drill { radius: 0.008, depth: 0.02, pose: Pose { position: [x, y, z], ..Pose::default() } },
        };
        ProcessWorkingScene {
            stock: Stock { id: "plate".into(), label: "Plate".into(), solid: WorkingSolid::Box { width: 1.2, depth: 0.8, height: 0.02 }, pose: Pose { position: [0.0, 0.0, z], ..Pose::default() } },
            steps: vec![
                hole("drill-1", "Drill Corner 1", 0.15, 0.15),
                hole("drill-2", "Drill Corner 2", 1.05, 0.15),
                hole("drill-3", "Drill Corner 3", 0.15, 0.65),
                hole("drill-4", "Drill Corner 4", 1.05, 0.65),
            ],
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
}
//#endregion 🧪️Tests
