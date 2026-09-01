//! 🧩️ Process robotic machine catalog extension — contributes robotic/CNC machines to `process3d-play`.

use semio_framework_plugin::ExtensionBundle;
use semio_s_plugin_process::artifacts::process3d::{Capability, CapabilityParameter, CapabilityRule, MachineCatalog, MeasureRecipe, StockQuantity, WorkshopMachine};

//#region 🔖️Catalog
pub struct RoboticCatalog;

fn parameter(id: &str, label: &str, value: f64) -> CapabilityParameter {
    CapabilityParameter { id: id.into(), label: label.into(), value }
}

fn max_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Max { quantity, parameter: parameter.into(), margin }
}

impl MachineCatalog for RoboticCatalog {
    fn catalog_id(&self) -> &str {
        "robotic"
    }

    fn label(&self) -> &str {
        "Robotic"
    }

    fn icon_id(&self) -> &str {
        "cpu"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        vec![
            WorkshopMachine {
                id: "sixAxisMill".into(),
                label: "6-Axis Robotic Mill".into(),
                icon_id: "cpu".into(),
                catalog_id: None,
                capabilities: vec![
                    Capability {
                        id: "mill".into(),
                        label: "Mill".into(),
                        icon_id: "cpu".into(),
                        recipe: MeasureRecipe::PocketCut { diameter: "endmillDiameter".into(), depth: "millDepth".into() },
                        parameters: vec![parameter("endmillDiameter", "Endmill Diameter", 0.02), parameter("millDepth", "Mill Depth", 0.1), parameter("reach", "Reach", 2.8)],
                        rules: vec![max_rule(StockQuantity::MaxDimension, "reach", 0.0)],
                    },
                    Capability {
                        id: "bore".into(),
                        label: "Bore".into(),
                        icon_id: "cpu".into(),
                        recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "boreDepth".into() },
                        parameters: vec![parameter("bitRadius", "Bit Radius", 0.01), parameter("boreDepth", "Bore Depth", 0.15), parameter("reach", "Reach", 2.8)],
                        rules: vec![max_rule(StockQuantity::MaxDimension, "reach", 0.0)],
                    },
                ],
            },
            WorkshopMachine {
                id: "gantryCnc".into(),
                label: "5-Axis Gantry CNC".into(),
                icon_id: "grid-3x3".into(),
                catalog_id: None,
                capabilities: vec![
                    Capability {
                        id: "mill".into(),
                        label: "Mill".into(),
                        icon_id: "grid-3x3".into(),
                        recipe: MeasureRecipe::PocketCut { diameter: "endmillDiameter".into(), depth: "millDepth".into() },
                        parameters: vec![parameter("endmillDiameter", "Endmill Diameter", 0.025), parameter("millDepth", "Mill Depth", 0.2), parameter("bedWidth", "Bed Width", 3.5), parameter("bedDepth", "Bed Depth", 12.0)],
                        rules: vec![max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                    },
                    Capability {
                        id: "saw".into(),
                        label: "Saw".into(),
                        icon_id: "grid-3x3".into(),
                        recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "sawDepth".into() },
                        parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.5), parameter("kerf", "Kerf", 0.005), parameter("sawDepth", "Saw Depth", 0.2), parameter("bedWidth", "Bed Width", 3.5), parameter("bedDepth", "Bed Depth", 12.0)],
                        rules: vec![max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                    },
                ],
            },
            WorkshopMachine {
                id: "waterjet".into(),
                label: "Waterjet".into(),
                icon_id: "pen-tool".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "jetCut".into(),
                    label: "Jet Cut".into(),
                    icon_id: "pen-tool".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "cutLength".into(), depth: "maxCutThickness".into() },
                    parameters: vec![
                        parameter("kerf", "Kerf", 0.001),
                        parameter("cutLength", "Cut Length", 1.5),
                        parameter("maxCutThickness", "Max Cut Thickness", 0.2),
                        parameter("bedWidth", "Bed Width", 3.0),
                        parameter("bedDepth", "Bed Depth", 1.5),
                    ],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutThickness", 0.0), max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "laserCutter".into(),
                label: "Laser Cutter".into(),
                icon_id: "scan-line".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "laserCut".into(),
                    label: "Laser Cut".into(),
                    icon_id: "scan-line".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "cutLength".into(), depth: "maxCutThickness".into() },
                    parameters: vec![
                        parameter("kerf", "Kerf", 0.0002),
                        parameter("cutLength", "Cut Length", 1.5),
                        parameter("maxCutThickness", "Max Cut Thickness", 0.025),
                        parameter("bedWidth", "Bed Width", 1.5),
                        parameter("bedDepth", "Bed Depth", 3.0),
                    ],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutThickness", 0.0), max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "assembler".into(),
                label: "Robotic Assembler".into(),
                icon_id: "component".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "place".into(),
                    label: "Place".into(),
                    icon_id: "component".into(),
                    recipe: MeasureRecipe::BoxAttach { width: "componentWidth".into(), depth: "componentDepth".into(), height: "componentHeight".into() },
                    parameters: vec![parameter("componentWidth", "Component Width", 0.1), parameter("componentDepth", "Component Depth", 0.1), parameter("componentHeight", "Component Height", 0.1), parameter("reach", "Reach", 2.8)],
                    rules: vec![max_rule(StockQuantity::MaxDimension, "reach", 0.0)],
                }],
            },
        ]
    }
}

pub fn catalog() -> RoboticCatalog {
    RoboticCatalog
}
//#endregion 🔖️Catalog

//#region 🔖️Bundle
const EXTENSION_ID: &str = "process-extension-robotic";
const HOST_APP_ID: &str = "process3d-play";

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request asking the SDK
// owner to revert those two (plus `TopicContribution::new`) to sync directly, matching the sibling
// reversion already applied to `ExtensionBundle::new`/`.extends`/`.depends_on` in that same impl block.
fn bundle() -> ExtensionBundle {
    let catalog = RoboticCatalog;
    let bundle = ExtensionBundle::new(EXTENSION_ID, "Process Robotic Machines", "0.1.0").extends("process");
    let bundle = semio_framework::io::resolve_ready(bundle.mode(semio_framework_plugin::ExecutionMode::Declarative));
    semio_framework::io::resolve_ready(bundle.contributes_topic(
        "process.machines",
        semio_framework_os_kernel::DslValue::object([
            ("appId".to_string(), semio_framework_os_kernel::DslValue::String(HOST_APP_ID.to_string())),
            ("moduleId".to_string(), semio_framework_os_kernel::DslValue::String(catalog.catalog_id().to_string())),
            ("label".to_string(), semio_framework_os_kernel::DslValue::String(catalog.label().to_string())),
            ("iconId".to_string(), semio_framework_os_kernel::DslValue::String(catalog.icon_id().to_string())),
            ("machinesJson".to_string(), semio_framework_os_kernel::DslValue::String(semio_framework_os_kernel::json::to_json_string(&catalog.machines()))),
        ]),
    ))
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
//#endregion 🧪️Tests
