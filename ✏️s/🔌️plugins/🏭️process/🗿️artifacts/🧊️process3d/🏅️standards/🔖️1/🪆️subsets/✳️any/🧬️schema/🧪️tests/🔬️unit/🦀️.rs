
use super::*;

//#region 🔖️ExampleFixtures
/// 🧭️ Every `step.origin` on a document must name a machine+capability that actually exists in
/// that document's own `workshop` — the mutations' only source of truth for legal origins.
fn assert_origins_resolve(document: &crate::Process3dSnapshot) {
    for step in &document.step_payloads {
        let origin = step.origin.as_ref().unwrap_or_else(|| panic!("step {:?} is missing its origin", step.id));
        let machine = document.workshop.machines.iter().find(|m| m.id == origin.machine_id).unwrap_or_else(|| panic!("step {:?} references unknown machine {:?}", step.id, origin.machine_id));
        assert!(machine.capabilities.iter().any(|c| c.id == origin.capability_id), "step {:?} references unknown capability {:?} on machine {:?}", step.id, origin.capability_id, machine.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn default_document_parses_timber_example() {
    let document = default_document();
    assert!(!document.steps.child_id.is_empty());
    assert!(document.resolved_up_to.is_none());

    let printed = document.print_dsl();
    let round_tripped = <crate::Process3dSnapshot as ArtifactDsl>::parse_dsl(&printed).expect("timber fixture round trip");
    assert_eq!(round_tripped, document, "timber fixture must round-trip through print_dsl/parse_dsl unchanged");

    match &document.stock_payload.solid {
        crate::WorkingSolid::Box { width, depth, height } => {
            assert!((*width - 3.0).abs() < 1e-9, "timber beam width should be 3.0m, got {width}");
            assert!((*depth - 0.2).abs() < 1e-9, "timber beam depth should be 0.2m, got {depth}");
            assert!((*height - 0.3).abs() < 1e-9, "timber beam height should be 0.3m, got {height}");
        }
        other => panic!("expected timber beam stock to be a non-degenerate Box, got {other:?}"),
    }

    let expected_ids = ["crosscut", "lap-joint-cut", "dowel-drill", "dowel-attach"];
    assert_eq!(document.step_payloads.len(), expected_ids.len(), "timber joinery timeline should have {} steps", expected_ids.len());
    for (step, expected_id) in document.step_payloads.iter().zip(expected_ids.iter()) {
        assert_eq!(&step.id, expected_id);
        assert!(step.enabled, "step {:?} should be enabled", step.id);
    }
    assert_origins_resolve(&document);
}

#[semio_framework_async_macros::async_test]
async fn plate_document_parses_and_opens_mid_timeline() {
    let document = plate_document();
    assert!(!document.steps.child_id.is_empty());
    assert_eq!(document.resolved_up_to, Some(2));

    let printed = document.print_dsl();
    let round_tripped = <crate::Process3dSnapshot as ArtifactDsl>::parse_dsl(&printed).expect("plate fixture round trip");
    assert_eq!(round_tripped, document, "plate fixture must round-trip through print_dsl/parse_dsl unchanged");

    match &document.stock_payload.solid {
        crate::WorkingSolid::Box { width, depth, height } => {
            assert!((*width - 1.2).abs() < 1e-9, "plate width should be 1.2m, got {width}");
            assert!((*depth - 0.8).abs() < 1e-9, "plate depth should be 0.8m, got {depth}");
            assert!((*height - 0.02).abs() < 1e-9, "plate height should be 0.02m, got {height}");
        }
        other => panic!("expected plate stock to be a non-degenerate Box, got {other:?}"),
    }

    assert_eq!(document.step_payloads.len(), 4, "drilled plate timeline should have 4 holes");
    for (index, step) in document.step_payloads.iter().enumerate() {
        assert_eq!(step.id, format!("drill-{}", index + 1));
        assert!(step.enabled, "step {:?} should be enabled", step.id);
        assert!(matches!(step.measure, crate::ProcessMeasure::Drill { .. }), "plate step {:?} should be a Drill measure", step.id);
    }
    assert_origins_resolve(&document);
}
//#endregion 🔖️ExampleFixtures

//#region 🔖️MetalCatalog
mod metal_catalog_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn every_machine_and_capability_id_is_unique() {
        let machines = MetalCatalog.machines();
        let mut machine_ids: Vec<&str> = machines.iter().map(|machine| machine.id.as_str()).collect();
        machine_ids.sort_unstable();
        machine_ids.dedup();
        assert_eq!(machine_ids.len(), machines.len(), "duplicate machine id in metal catalog");
        for machine in &machines {
            let mut capability_ids: Vec<&str> = machine.capabilities.iter().map(|capability| capability.id.as_str()).collect();
            capability_ids.sort_unstable();
            capability_ids.dedup();
            assert_eq!(capability_ids.len(), machine.capabilities.len(), "duplicate capability id on machine {}", machine.id);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn every_recipe_and_rule_parameter_resolves() {
        for machine in MetalCatalog.machines() {
            for capability in &machine.capabilities {
                let ids: Vec<&str> = capability.parameters.iter().map(|parameter| parameter.id.as_str()).collect();
                let recipe_params: Vec<&str> = match &capability.recipe {
                    MeasureRecipe::DiscCut { diameter, kerf } => vec![diameter.as_str(), kerf.as_str()],
                    MeasureRecipe::BladeCut { kerf, length, depth } => vec![kerf.as_str(), length.as_str(), depth.as_str()],
                    MeasureRecipe::PocketCut { diameter, depth } => vec![diameter.as_str(), depth.as_str()],
                    MeasureRecipe::BoreDrill { radius, depth } => vec![radius.as_str(), depth.as_str()],
                    MeasureRecipe::CylinderAttach { radius, length } => vec![radius.as_str(), length.as_str()],
                    MeasureRecipe::BoxAttach { width, depth, height } => vec![width.as_str(), depth.as_str(), height.as_str()],
                };
                for name in recipe_params {
                    assert!(ids.contains(&name), "{}.{}: recipe references unknown parameter '{name}'", machine.id, capability.id);
                }
                for rule in &capability.rules {
                    let name = match rule {
                        CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter.as_str(),
                    };
                    assert!(ids.contains(&name), "{}.{}: rule references unknown parameter '{name}'", machine.id, capability.id);
                }
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn machines_round_trip_json() {
        let machines = MetalCatalog.machines();
        let json = semio_framework_os_kernel::json::to_json_string(&machines);
        let parsed: Vec<WorkshopMachine> = semio_framework_os_kernel::json::from_json_str(&json).expect("deserialize");
        assert_eq!(parsed, machines);
    }

    #[semio_framework_async_macros::async_test]
    async fn catalog_has_metal_identity() {
        let catalog = MetalCatalog;
        assert_eq!(catalog.catalog_id(), "metal");
        assert_eq!(catalog.label(), "Metal");
    }
}
//#endregion 🔖️MetalCatalog

//#region 🔖️WoodCatalog
mod wood_catalog_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn every_machine_and_capability_id_is_unique() {
        let machines = WoodCatalog.machines();
        let mut machine_ids: Vec<&str> = machines.iter().map(|machine| machine.id.as_str()).collect();
        machine_ids.sort_unstable();
        machine_ids.dedup();
        assert_eq!(machine_ids.len(), machines.len(), "duplicate machine id in wood catalog");
        for machine in &machines {
            let mut capability_ids: Vec<&str> = machine.capabilities.iter().map(|capability| capability.id.as_str()).collect();
            capability_ids.sort_unstable();
            capability_ids.dedup();
            assert_eq!(capability_ids.len(), machine.capabilities.len(), "duplicate capability id on machine {}", machine.id);
        }
    }

    /// ✅️ Every recipe field and rule parameter must resolve within its own capability's parameters.
    #[semio_framework_async_macros::async_test]
    async fn every_recipe_and_rule_parameter_resolves() {
        for machine in WoodCatalog.machines() {
            for capability in &machine.capabilities {
                let ids: Vec<&str> = capability.parameters.iter().map(|parameter| parameter.id.as_str()).collect();
                let recipe_params: Vec<&str> = match &capability.recipe {
                    MeasureRecipe::DiscCut { diameter, kerf } => vec![diameter.as_str(), kerf.as_str()],
                    MeasureRecipe::BladeCut { kerf, length, depth } => vec![kerf.as_str(), length.as_str(), depth.as_str()],
                    MeasureRecipe::PocketCut { diameter, depth } => vec![diameter.as_str(), depth.as_str()],
                    MeasureRecipe::BoreDrill { radius, depth } => vec![radius.as_str(), depth.as_str()],
                    MeasureRecipe::CylinderAttach { radius, length } => vec![radius.as_str(), length.as_str()],
                    MeasureRecipe::BoxAttach { width, depth, height } => vec![width.as_str(), depth.as_str(), height.as_str()],
                };
                for name in recipe_params {
                    assert!(ids.contains(&name), "{}.{}: recipe references unknown parameter '{name}'", machine.id, capability.id);
                }
                for rule in &capability.rules {
                    let name = match rule {
                        CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter.as_str(),
                    };
                    assert!(ids.contains(&name), "{}.{}: rule references unknown parameter '{name}'", machine.id, capability.id);
                }
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn machines_round_trip_json() {
        let machines = WoodCatalog.machines();
        let json = semio_framework_os_kernel::json::to_json_string(&machines);
        let parsed: Vec<WorkshopMachine> = semio_framework_os_kernel::json::from_json_str(&json).expect("deserialize");
        assert_eq!(parsed, machines);
    }

    #[semio_framework_async_macros::async_test]
    async fn catalog_has_wood_identity() {
        let catalog = WoodCatalog;
        assert_eq!(catalog.catalog_id(), "wood");
        assert_eq!(catalog.label(), "Wood");
    }
}
//#endregion 🔖️WoodCatalog

//#region 🔖️RoboticCatalog
mod robotic_catalog_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn every_machine_and_capability_id_is_unique() {
        let machines = RoboticCatalog.machines();
        let mut machine_ids: Vec<&str> = machines.iter().map(|machine| machine.id.as_str()).collect();
        machine_ids.sort_unstable();
        machine_ids.dedup();
        assert_eq!(machine_ids.len(), machines.len(), "duplicate machine id in robotic catalog");
        for machine in &machines {
            let mut capability_ids: Vec<&str> = machine.capabilities.iter().map(|capability| capability.id.as_str()).collect();
            capability_ids.sort_unstable();
            capability_ids.dedup();
            assert_eq!(capability_ids.len(), machine.capabilities.len(), "duplicate capability id on machine {}", machine.id);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn every_recipe_and_rule_parameter_resolves() {
        for machine in RoboticCatalog.machines() {
            for capability in &machine.capabilities {
                let ids: Vec<&str> = capability.parameters.iter().map(|parameter| parameter.id.as_str()).collect();
                let recipe_params: Vec<&str> = match &capability.recipe {
                    MeasureRecipe::DiscCut { diameter, kerf } => vec![diameter.as_str(), kerf.as_str()],
                    MeasureRecipe::BladeCut { kerf, length, depth } => vec![kerf.as_str(), length.as_str(), depth.as_str()],
                    MeasureRecipe::PocketCut { diameter, depth } => vec![diameter.as_str(), depth.as_str()],
                    MeasureRecipe::BoreDrill { radius, depth } => vec![radius.as_str(), depth.as_str()],
                    MeasureRecipe::CylinderAttach { radius, length } => vec![radius.as_str(), length.as_str()],
                    MeasureRecipe::BoxAttach { width, depth, height } => vec![width.as_str(), depth.as_str(), height.as_str()],
                };
                for name in recipe_params {
                    assert!(ids.contains(&name), "{}.{}: recipe references unknown parameter '{name}'", machine.id, capability.id);
                }
                for rule in &capability.rules {
                    let name = match rule {
                        CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter.as_str(),
                    };
                    assert!(ids.contains(&name), "{}.{}: rule references unknown parameter '{name}'", machine.id, capability.id);
                }
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn machines_round_trip_json() {
        let machines = RoboticCatalog.machines();
        let json = semio_framework_os_kernel::json::to_json_string(&machines);
        let parsed: Vec<WorkshopMachine> = semio_framework_os_kernel::json::from_json_str(&json).expect("deserialize");
        assert_eq!(parsed, machines);
    }

    #[semio_framework_async_macros::async_test]
    async fn catalog_has_robotic_identity() {
        let catalog = RoboticCatalog;
        assert_eq!(catalog.catalog_id(), "robotic");
        assert_eq!(catalog.label(), "Robotic");
    }
}
//#endregion 🔖️RoboticCatalog

//#region 🔖️ConcreteCatalog
mod concrete_catalog_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn every_machine_and_capability_id_is_unique() {
        let machines = ConcreteCatalog.machines();
        let mut machine_ids: Vec<&str> = machines.iter().map(|machine| machine.id.as_str()).collect();
        machine_ids.sort_unstable();
        machine_ids.dedup();
        assert_eq!(machine_ids.len(), machines.len(), "duplicate machine id in concrete catalog");
        for machine in &machines {
            let mut capability_ids: Vec<&str> = machine.capabilities.iter().map(|capability| capability.id.as_str()).collect();
            capability_ids.sort_unstable();
            capability_ids.dedup();
            assert_eq!(capability_ids.len(), machine.capabilities.len(), "duplicate capability id on machine {}", machine.id);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn every_recipe_and_rule_parameter_resolves() {
        for machine in ConcreteCatalog.machines() {
            for capability in &machine.capabilities {
                let ids: Vec<&str> = capability.parameters.iter().map(|parameter| parameter.id.as_str()).collect();
                let recipe_params: Vec<&str> = match &capability.recipe {
                    MeasureRecipe::DiscCut { diameter, kerf } => vec![diameter.as_str(), kerf.as_str()],
                    MeasureRecipe::BladeCut { kerf, length, depth } => vec![kerf.as_str(), length.as_str(), depth.as_str()],
                    MeasureRecipe::PocketCut { diameter, depth } => vec![diameter.as_str(), depth.as_str()],
                    MeasureRecipe::BoreDrill { radius, depth } => vec![radius.as_str(), depth.as_str()],
                    MeasureRecipe::CylinderAttach { radius, length } => vec![radius.as_str(), length.as_str()],
                    MeasureRecipe::BoxAttach { width, depth, height } => vec![width.as_str(), depth.as_str(), height.as_str()],
                };
                for name in recipe_params {
                    assert!(ids.contains(&name), "{}.{}: recipe references unknown parameter '{name}'", machine.id, capability.id);
                }
                for rule in &capability.rules {
                    let name = match rule {
                        CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter.as_str(),
                    };
                    assert!(ids.contains(&name), "{}.{}: rule references unknown parameter '{name}'", machine.id, capability.id);
                }
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn machines_round_trip_json() {
        let machines = ConcreteCatalog.machines();
        let json = semio_framework_os_kernel::json::to_json_string(&machines);
        let parsed: Vec<WorkshopMachine> = semio_framework_os_kernel::json::from_json_str(&json).expect("deserialize");
        assert_eq!(parsed, machines);
    }

    #[semio_framework_async_macros::async_test]
    async fn catalog_has_concrete_identity() {
        let catalog = ConcreteCatalog;
        assert_eq!(catalog.catalog_id(), "concrete");
        assert_eq!(catalog.label(), "Concrete");
    }
}
//#endregion 🔖️ConcreteCatalog
//#region 🔖️DocumentHelpers
fn timeline_fixture(cursor: Option<usize>) -> crate::Process3dSnapshot {
    use crate::{Pose, ProcessMeasure, ProcessStep, ProcessWorkingScene, Stock, WorkingSolid, Workshop};
    let step = |id: &str| ProcessStep { id: id.into(), label: id.into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.01, depth: 0.05, pose: Pose::default() } };
    let scene = ProcessWorkingScene { stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 0.5, height: 0.25 }, pose: Pose::default() }, steps: vec![step("a"), step("b"), step("c"), step("d")] };
    crate::process_working_scene_to_snapshot(&scene, Workshop::default(), cursor)
}

fn new_step() -> ProcessStep {
    use crate::{Pose, ProcessMeasure, ProcessStep, WorkingSolid};
    ProcessStep { id: "e".into(), label: "e".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.01, depth: 0.2, height: 0.2 }, pose: Pose::default() } }
}

/// 🧭️ `resolved_up_to` is a COUNT of resolved steps (`replay_process` slices `step_payloads[..limit]`),
/// so a step added at cursor `c` must land AT index `c` and push the cursor to `c + 1` — that is the
/// only pairing under which the step the user just added is the one that becomes visible.
#[semio_framework_async_macros::async_test]
async fn inserting_a_step_at_the_cursor_makes_that_step_the_newly_resolved_one() {
    use crate::op::Process3dMutation;
    let fixture = timeline_fixture(Some(2));
    let operations = insert_step_mutations(&fixture, new_step());
    match &operations[0] {
        Process3dMutation::CreateStep(create) => assert_eq!(create.index, 2, "the new step must land at the cursor, not past it"),
        other => panic!("expected CreateStep, got {other:?}"),
    }
    match &operations[1] {
        Process3dMutation::ChangeCursor(cursor) => assert_eq!(cursor.new_resolved_up_to, Some(3), "the cursor must advance past the step just added"),
        other => panic!("expected ChangeCursor, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_step_with_no_cursor_appends_and_leaves_the_cursor_alone() {
    use crate::op::Process3dMutation;
    let fixture = timeline_fixture(None);
    let operations = insert_step_mutations(&fixture, new_step());
    assert_eq!(operations.len(), 1, "a fully-resolved document needs no cursor mutation");
    match &operations[0] {
        Process3dMutation::CreateStep(create) => assert_eq!(create.index, 4),
        other => panic!("expected CreateStep, got {other:?}"),
    }
}

/// 🧭️ Deleting the first UNRESOLVED step (index == cursor) leaves the resolved prefix untouched, so
/// the cursor must not move; only a deletion strictly inside the prefix pulls it back.
#[semio_framework_async_macros::async_test]
async fn removing_a_step_only_pulls_the_cursor_back_when_the_step_was_inside_the_resolved_prefix() {
    use crate::op::Process3dMutation;
    let fixture = timeline_fixture(Some(2));
    assert_eq!(remove_step_mutations(&fixture, "c").expect("step c exists").len(), 1, "deleting the first unresolved step must not move the cursor");
    let inside = remove_step_mutations(&fixture, "b").expect("step b exists");
    assert_eq!(inside.len(), 2);
    match &inside[1] {
        Process3dMutation::ChangeCursor(cursor) => assert_eq!(cursor.new_resolved_up_to, Some(1)),
        other => panic!("expected ChangeCursor, got {other:?}"),
    }
    assert!(remove_step_mutations(&fixture, "missing").is_none(), "an unknown id yields no operations at all");
}
//#endregion 🔖️DocumentHelpers
