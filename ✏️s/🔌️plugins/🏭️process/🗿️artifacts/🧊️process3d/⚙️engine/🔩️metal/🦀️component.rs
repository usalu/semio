//! 🔩️ Process3d engine — built-in metal-shop machine catalog (saws, grinding, drilling, plasma, welding).
//!
//! Folded in from the old, standalone `semio-s-plugin-process-metal` crate — see `🦀️catalog_wood.rs`'s
//! doc comment for why these built-in catalogs are engine topic files rather than satellite crates in
//! this merged layout.

use crate::artifacts::process3d::{Capability, CapabilityParameter, CapabilityRule, MachineCatalog, MeasureRecipe, StockQuantity, WorkshopMachine};

//#region 🔖️Catalog
pub struct MetalCatalog;

fn parameter(id: &str, label: &str, value: f64) -> CapabilityParameter {
    CapabilityParameter { id: id.into(), label: label.into(), value }
}

fn max_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Max { quantity, parameter: parameter.into(), margin }
}

fn min_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Min { quantity, parameter: parameter.into(), margin }
}

impl MachineCatalog for MetalCatalog {
    fn catalog_id(&self) -> &'static str {
        "metal"
    }

    fn label(&self) -> &'static str {
        "Metal"
    }

    fn icon_id(&self) -> &'static str {
        "wrench"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        vec![
            WorkshopMachine {
                id: "chopSaw".into(),
                label: "Chop Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "chop".into(),
                    label: "Chop".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.355), parameter("kerf", "Kerf", 0.003), parameter("maxCutDepth", "Max Cut Depth", 0.12), parameter("maxStockWidth", "Max Stock Width", 0.23)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0), max_rule(StockQuantity::Width, "maxStockWidth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "angleGrinder".into(),
                label: "Angle Grinder".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "grindCut".into(),
                    label: "Grind Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "discDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("discDiameter", "Disc Diameter", 0.125), parameter("kerf", "Kerf", 0.0025), parameter("maxCutDepth", "Max Cut Depth", 0.038)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "pillarDrill".into(),
                label: "Pillar Drill".into(),
                icon_id: "circle-dot".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "bore".into(),
                    label: "Bore".into(),
                    icon_id: "circle-dot".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "strokeDepth".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.008), parameter("strokeDepth", "Stroke Depth", 0.08)],
                    rules: vec![max_rule(StockQuantity::Height, "strokeDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "magDrill".into(),
                label: "Mag Drill".into(),
                icon_id: "magnet".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "annularBore".into(),
                    label: "Annular Bore".into(),
                    icon_id: "magnet".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "cutterLength".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.017), parameter("cutterLength", "Cutter Length", 0.05), parameter("minPlateThickness", "Min Plate Thickness", 0.006)],
                    rules: vec![max_rule(StockQuantity::Height, "cutterLength", 0.0), min_rule(StockQuantity::Height, "minPlateThickness", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "plasmaCutter".into(),
                label: "Plasma Cutter".into(),
                icon_id: "sparkles".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "plasmaCut".into(),
                    label: "Plasma Cut".into(),
                    icon_id: "sparkles".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "cutLength".into(), depth: "maxCutThickness".into() },
                    parameters: vec![parameter("kerf", "Kerf", 0.0015), parameter("cutLength", "Cut Length", 1.5), parameter("maxCutThickness", "Max Cut Thickness", 0.02)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutThickness", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "migWelder".into(),
                label: "MIG Welder".into(),
                icon_id: "combine".into(),
                catalog_id: None,
                capabilities: vec![
                    Capability {
                        id: "weldBead".into(),
                        label: "Weld Bead".into(),
                        icon_id: "combine".into(),
                        recipe: MeasureRecipe::CylinderAttach { radius: "beadRadius".into(), length: "beadLength".into() },
                        parameters: vec![parameter("beadRadius", "Bead Radius", 0.004), parameter("beadLength", "Bead Length", 0.05), parameter("minThickness", "Min Thickness", 0.0008)],
                        rules: vec![min_rule(StockQuantity::Height, "minThickness", 0.0)],
                    },
                    Capability {
                        id: "weldPlate".into(),
                        label: "Weld Plate".into(),
                        icon_id: "combine".into(),
                        recipe: MeasureRecipe::BoxAttach { width: "plateWidth".into(), depth: "plateDepth".into(), height: "plateThickness".into() },
                        parameters: vec![parameter("plateWidth", "Plate Width", 0.1), parameter("plateDepth", "Plate Depth", 0.1), parameter("plateThickness", "Plate Thickness", 0.008), parameter("minThickness", "Min Thickness", 0.0008)],
                        rules: vec![min_rule(StockQuantity::Height, "minThickness", 0.0)],
                    },
                ],
            },
            WorkshopMachine {
                id: "studWelder".into(),
                label: "Stud Welder".into(),
                icon_id: "plug".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "stud".into(),
                    label: "Stud".into(),
                    icon_id: "plug".into(),
                    recipe: MeasureRecipe::CylinderAttach { radius: "studRadius".into(), length: "studLength".into() },
                    parameters: vec![parameter("studRadius", "Stud Radius", 0.005), parameter("studLength", "Stud Length", 0.025), parameter("minThickness", "Min Thickness", 0.002)],
                    rules: vec![min_rule(StockQuantity::Height, "minThickness", 0.0)],
                }],
            },
        ]
    }
}

pub fn catalog() -> Box<dyn MachineCatalog> {
    Box::new(MetalCatalog)
}
//#endregion 🔖️Catalog

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_machine_and_capability_id_is_unique() {
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

    #[test]
    fn every_recipe_and_rule_parameter_resolves() {
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

    #[test]
    fn machines_round_trip_json() {
        let machines = MetalCatalog.machines();
        let json = serde_json::to_string(&machines).expect("serialize");
        let parsed: Vec<WorkshopMachine> = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, machines);
    }

    #[test]
    fn catalog_has_metal_identity() {
        let catalog = MetalCatalog;
        assert_eq!(catalog.catalog_id(), "metal");
        assert_eq!(catalog.label(), "Metal");
    }
}
//#endregion 🧪️Tests
