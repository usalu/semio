//! 📜️ Process3d artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::process3d::Process3dSnapshot;

/// 🗄️ The timber-beam-joinery example fixture, handcrafted in this artifact's DSL (`store::DocumentDsl`).
pub const PROCESS_3D_TIMBER_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 🗄️ The drilled-plate example fixture, handcrafted inline (same DSL surface as the timber demo asset).
pub const PROCESS_3D_PLATE_EXAMPLE_TEXT: &str = r#"semio process.process3d.dsl v1
resolved-up-to=2
workshop {
  machines=[ id=saw label="Generic Saw" icon-id=scissors capabilities=[ id=cut label=Cut icon-id=scissors parameters=[ id=kerf label=Kerf value=0.05m id=length label=Length value=0.5m id=depth label=Depth value=0.5m ]
  rules {
  }
  blade-cut kerf=kerf length=length depth=depth ] id=drill label="Generic Drill" icon-id=circle-dot capabilities=[ id=drill label=Drill icon-id=circle-dot parameters=[ id=radius label=Radius value=0.05m id=depth label=Depth value=0.3m ]
  rules {
  }
  bore-drill radius=radius depth=depth ] id=attacher label="Generic Attacher" icon-id=plus capabilities=[ id=attach label=Attach icon-id=plus parameters=[ id=radius label=Radius value=0.03m id=length label=Length value=0.2m ]
  rules {
  }
  cylinder-attach radius=radius length=length ] ]
}
stock {
  id=plate label=Plate
  pose {
    position=@0,0,0 axis=^0,0,0 angle=0rad
  }
  box width=1m depth=1m height=0.05m
}
steps=[ id=d1 label=Drill enabled=true
origin {
  machine-id=circularSaw capability-id=crosscut
}
drill radius=0.02m depth=0.3m
pose {
  position=@0,0,0 axis=^0,0,0 angle=0rad
}
id=d2 label=Drill enabled=true
origin {
  machine-id=circularSaw capability-id=crosscut
}
drill radius=0.02m depth=0.3m
pose {
  position=@0,0,0 axis=^0,0,0 angle=0rad
}
id=d3 label=Drill enabled=true
origin {
  machine-id=circularSaw capability-id=crosscut
}
drill radius=0.02m depth=0.3m
pose {
  position=@0,0,0 axis=^0,0,0 angle=0rad
}
]"#;

/// 📖️ Parses `.process3d` DSL text into a `Process3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Process3dSnapshot, store::TextError> {
    <Process3dSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Process3dSnapshot` back to `.process3d` DSL text.
pub fn print_dsl(document: &Process3dSnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;






    use crate::artifacts::process3d::{empty_process3d_snapshot, Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessMeasure, ProcessStep, SolidSpec, StepOrigin, Stock, StockQuantity, Workshop, WorkshopMachine};

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
    fn sample_document() -> Process3dSnapshot {
        Process3dSnapshot {
            workshop: Workshop { machines: vec![circular_saw_machine()] },
            stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose { position: [0.0, 0.0, 0.12], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            steps: vec![cut_step("cut-1"), drill_step("drill-1"), attach_step("attach-1")],
            resolved_up_to: Some(2),
        }
    }

    #[test]
    fn process3d_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&sample_document());
        store::os_store::test_support::assert_dsl_round_trip(&empty_process3d_snapshot());
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
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn process3d_dsl_round_trips_with_no_resolved_cursor() {
        let mut document = sample_document();
        document.resolved_up_to = None;
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn timber_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(PROCESS_3D_TIMBER_EXAMPLE_TEXT).expect("parse timber example");
        assert_eq!(document.steps.len(), 4);
        assert!(document.resolved_up_to.is_none());
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn drilled_plate_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(PROCESS_3D_PLATE_EXAMPLE_TEXT).expect("parse drilled plate example");
        assert_eq!(document.steps.len(), 3);
        assert_eq!(document.resolved_up_to, Some(2));
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
