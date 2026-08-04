//! 📜️ Process 3d app — textual document grammar surface + laws (constitutional: dsl).

use process_3d::Process3dDocument;

/// 🗄️ The timber-beam-joinery example fixture, handcrafted in `process_3d`'s DSL (`store::DocumentDsl`).
pub const PROCESS_3D_TIMBER_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/⚡️implementations/🦀️rust/📚️examples/🏭️timber-beam-joinery.process3d");

/// 🗄️ The drilled-plate example fixture, handcrafted in `process_3d`'s DSL (`store::DocumentDsl`).
pub const PROCESS_3D_PLATE_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/⚡️implementations/🦀️rust/📚️examples/🏭️drilled-plate.process3d");

/// 📖️ Parses `.process3d` DSL text into a `Process3dDocument`.
pub fn parse_dsl(text: &str) -> Result<Process3dDocument, store::TextError> {
    <Process3dDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Process3dDocument` back to `.process3d` DSL text.
pub fn print_dsl(document: &Process3dDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use process_3d::{empty_process3d_projection, Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessMeasure, ProcessStep, SolidSpec, StepOrigin, Stock, StockQuantity, Workshop, WorkshopMachine};

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
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
            measure: ProcessMeasure::Attach { component: SolidSpec::Sphere { radius: 0.05 }, pose: Pose { position: [0.1, -0.2, 0.3], axis: [0.0, 1.0, 0.0], angle: 1.2 } },
        }
    }

    fn imported_mesh_stock() -> Stock {
        Stock { id: "stock".into(), label: "Imported GLB".into(), solid: SolidSpec::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }, pose: Pose::default() }
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

    /// 📜️ A document exercising every `SolidSpec`/`ProcessMeasure` shape, both `origin` states, and a
    /// non-default workshop machine (3-deep nesting), so the DSL round trip covers the full grammar.
    fn sample_document() -> Process3dDocument {
        Process3dDocument {
            workshop: Workshop { machines: vec![circular_saw_machine()] },
            stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose { position: [0.0, 0.0, 0.12], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            steps: vec![cut_step("cut-1"), drill_step("drill-1"), attach_step("attach-1")],
            resolved_up_to: Some(2),
        }
    }

    #[test]
    fn process3d_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&sample_document());
        store::test_support::assert_dsl_round_trip(&empty_process3d_projection());
    }

    #[test]
    fn process3d_dsl_round_trips_imported_solid_shapes() {
        let mut document = sample_document();
        document.stock = imported_mesh_stock();
        document.steps.push(ProcessStep {
            id: "imported-tool".into(),
            label: "Imported Cut".into(),
            enabled: true,
            origin: None,
            measure: ProcessMeasure::Cut { tool: SolidSpec::ImportedSolid { solid_handle: "solid-7".into() }, pose: Pose::default() },
        });
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn process3d_dsl_round_trips_with_no_resolved_cursor() {
        let mut document = sample_document();
        document.resolved_up_to = None;
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn timber_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(PROCESS_3D_TIMBER_EXAMPLE_TEXT).expect("parse timber example");
        assert_eq!(document.steps.len(), 4);
        assert!(document.resolved_up_to.is_none());
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn drilled_plate_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(PROCESS_3D_PLATE_EXAMPLE_TEXT).expect("parse drilled plate example");
        assert_eq!(document.steps.len(), 3);
        assert_eq!(document.resolved_up_to, Some(2));
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
