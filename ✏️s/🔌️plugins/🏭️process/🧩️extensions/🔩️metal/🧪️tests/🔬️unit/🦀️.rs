
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
