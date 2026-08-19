//! 🧩️ Process wood machine catalog extension — contributes wood-shop machines to `process3d-play`.

use semio_framework_plugin::ExtensionBundle;
use semio_s_plugin_process::artifacts::process3d::{Capability, CapabilityParameter, CapabilityRule, MachineCatalog, MeasureRecipe, StockQuantity, WorkshopMachine};

//#region 🔖️Catalog
pub struct WoodCatalog;

async fn parameter(id: &str, label: &str, value: f64) -> CapabilityParameter {
    CapabilityParameter { id: id.into(), label: label.into(), value }
}

async fn max_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Max { quantity, parameter: parameter.into(), margin }
}

async fn min_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Min { quantity, parameter: parameter.into(), margin }
}

impl MachineCatalog for WoodCatalog {
    async fn catalog_id(&self) -> &'static str {
        "wood"
    }

    async fn label(&self) -> &'static str {
        "Wood"
    }

    async fn icon_id(&self) -> &'static str {
        "beam"
    }

    async fn machines(&self) -> Vec<WorkshopMachine> {
        vec![
            WorkshopMachine {
                id: "circularSaw".into(),
                label: "Circular Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "crosscut".into(),
                    label: "Crosscut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.184), parameter("kerf", "Kerf", 0.002), parameter("maxCutDepth", "Max Cut Depth", 0.065)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "tableSaw".into(),
                label: "Table Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "rip".into(),
                    label: "Rip".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.315), parameter("kerf", "Kerf", 0.0032), parameter("maxCutDepth", "Max Cut Depth", 0.102), parameter("fenceWidth", "Fence Width", 0.8)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0), max_rule(StockQuantity::Width, "fenceWidth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "bandSaw".into(),
                label: "Band Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "curveCut".into(),
                    label: "Curve Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "bladeLength".into(), depth: "maxCutHeight".into() },
                    parameters: vec![parameter("kerf", "Kerf", 0.0015), parameter("bladeLength", "Blade Length", 0.5), parameter("maxCutHeight", "Max Cut Height", 0.30), parameter("throatDepth", "Throat Depth", 0.44)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutHeight", 0.0), max_rule(StockQuantity::Width, "throatDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "chainSaw".into(),
                label: "Chain Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "roughCut".into(),
                    label: "Rough Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "barLength".into(), depth: "barLength".into() },
                    parameters: vec![parameter("kerf", "Kerf", 0.008), parameter("barLength", "Bar Length", 0.45), parameter("minStockDimension", "Min Stock Dimension", 0.05)],
                    rules: vec![max_rule(StockQuantity::Height, "barLength", 0.0), min_rule(StockQuantity::MinDimension, "minStockDimension", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "drillPress".into(),
                label: "Drill Press".into(),
                icon_id: "circle-dot".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "bore".into(),
                    label: "Bore".into(),
                    icon_id: "circle-dot".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "strokeDepth".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.005), parameter("strokeDepth", "Stroke Depth", 0.10), parameter("throatDepth", "Throat Depth", 0.16)],
                    rules: vec![max_rule(StockQuantity::Height, "strokeDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "cncRouter".into(),
                label: "CNC Router".into(),
                icon_id: "cpu".into(),
                catalog_id: None,
                capabilities: vec![
                    Capability {
                        id: "pocket".into(),
                        label: "Pocket".into(),
                        icon_id: "cpu".into(),
                        recipe: MeasureRecipe::PocketCut { diameter: "endmillDiameter".into(), depth: "pocketDepth".into() },
                        parameters: vec![parameter("endmillDiameter", "Endmill Diameter", 0.012), parameter("pocketDepth", "Pocket Depth", 0.04), parameter("bedWidth", "Bed Width", 1.25), parameter("bedDepth", "Bed Depth", 2.5)],
                        rules: vec![max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                    },
                    Capability {
                        id: "bore".into(),
                        label: "Bore".into(),
                        icon_id: "cpu".into(),
                        recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "boreDepth".into() },
                        parameters: vec![parameter("bitRadius", "Bit Radius", 0.006), parameter("boreDepth", "Bore Depth", 0.04), parameter("bedWidth", "Bed Width", 1.25), parameter("bedDepth", "Bed Depth", 2.5)],
                        rules: vec![max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                    },
                ],
            },
            WorkshopMachine {
                id: "dowelJig".into(),
                label: "Doweling Jig".into(),
                icon_id: "plus".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "dowel".into(),
                    label: "Dowel".into(),
                    icon_id: "plus".into(),
                    recipe: MeasureRecipe::CylinderAttach { radius: "dowelRadius".into(), length: "dowelLength".into() },
                    parameters: vec![parameter("dowelRadius", "Dowel Radius", 0.004), parameter("dowelLength", "Dowel Length", 0.04), parameter("minStockThickness", "Min Stock Thickness", 0.018)],
                    rules: vec![min_rule(StockQuantity::Height, "minStockThickness", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "screwGun".into(),
                label: "Screw Gun".into(),
                icon_id: "wrench".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "screw".into(),
                    label: "Screw".into(),
                    icon_id: "wrench".into(),
                    recipe: MeasureRecipe::CylinderAttach { radius: "screwRadius".into(), length: "screwLength".into() },
                    parameters: vec![parameter("screwRadius", "Screw Radius", 0.0025), parameter("screwLength", "Screw Length", 0.05)],
                    rules: vec![min_rule(StockQuantity::Height, "screwLength", 0.0)],
                }],
            },
        ]
    }
}

pub async fn catalog() -> Box<dyn MachineCatalog> {
    Box::new(WoodCatalog)
}
//#endregion 🔖️Catalog

//#region 🔖️Bundle
const EXTENSION_ID: &str = "process-extension-wood";
const HOST_APP_ID: &str = "process3d-play";

async fn bundle() -> ExtensionBundle {
    let catalog = WoodCatalog;
    ExtensionBundle::new(EXTENSION_ID, "Process Wood Machines", "0.1.0")
        .extends("process")
        .mode(semio_framework_plugin::ExecutionMode::Declarative)
        .contributes_topic(
            "process.machines",
            serde_json::json!({
                "appId": HOST_APP_ID,
                "moduleId": catalog.catalog_id(),
                "label": catalog.label(),
                "iconId": catalog.icon_id(),
                "machinesJson": serde_json::to_string(&catalog.machines()).unwrap_or_default(),
            }),
        )
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    #[test]
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

    #[test]
    async fn machines_round_trip_json() {
        let machines = WoodCatalog.machines();
        let json = serde_json::to_string(&machines).expect("serialize");
        let parsed: Vec<WorkshopMachine> = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, machines);
    }

    #[test]
    async fn catalog_has_wood_identity() {
        let catalog = WoodCatalog;
        assert_eq!(catalog.catalog_id(), "wood");
        assert_eq!(catalog.label(), "Wood");
    }

    #[test]
    async fn bundle_contributes_wood_machines_for_process3d_play() {
        let manifest = bundle().manifest;
        assert_eq!(manifest.extension_id, "process-extension-wood");
        assert_eq!(manifest.extends, "process");
        assert_eq!(manifest.topic_contributions.len(), 1);
        let topic_contribution = &manifest.topic_contributions[0];
        assert_eq!(topic_contribution.topic, "process.machines");
        let payload = topic_contribution.payload.as_object().expect("object payload");
        assert_eq!(payload["appId"], "process3d-play");
        assert_eq!(payload["moduleId"], "wood");
        assert_eq!(payload["label"], "Wood");
        assert!(serde_json::from_str::<Vec<WorkshopMachine>>(payload["machinesJson"].as_str().expect("string")).is_ok());
    }
}
//#endregion 🧪️Tests
