//! 🧩️ Process concrete machine catalog extension — contributes concrete-shop machines to `process3d-play`.

use semio_framework_plugin::ExtensionBundle;
use semio_s_plugin_process::artifacts::process3d::{Capability, CapabilityParameter, CapabilityRule, MachineCatalog, MeasureRecipe, StockQuantity, WorkshopMachine};

//#region 🔖️Catalog
pub struct ConcreteCatalog;

fn parameter(id: &str, label: &str, value: f64) -> CapabilityParameter {
    CapabilityParameter { id: id.into(), label: label.into(), value }
}

fn max_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Max { quantity, parameter: parameter.into(), margin }
}

fn min_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Min { quantity, parameter: parameter.into(), margin }
}

impl MachineCatalog for ConcreteCatalog {
    fn catalog_id(&self) -> &str {
        "concrete"
    }

    fn label(&self) -> &str {
        "Concrete"
    }

    fn icon_id(&self) -> &str {
        "slab"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        vec![
            WorkshopMachine {
                id: "diamondSaw".into(),
                label: "Diamond Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "crosscut".into(),
                    label: "Crosscut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.35), parameter("kerf", "Kerf", 0.004), parameter("maxCutDepth", "Max Cut Depth", 0.125)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "wallSaw".into(),
                label: "Wall Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "wallCut".into(),
                    label: "Wall Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.8), parameter("kerf", "Kerf", 0.0045), parameter("maxCutDepth", "Max Cut Depth", 0.32)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "wireSaw".into(),
                label: "Wire Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "wireCut".into(),
                    label: "Wire Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "wireSpan".into(), depth: "maxSection".into() },
                    parameters: vec![parameter("kerf", "Kerf", 0.011), parameter("wireSpan", "Wire Span", 3.0), parameter("maxSection", "Max Section", 2.5)],
                    rules: vec![max_rule(StockQuantity::MaxDimension, "maxSection", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "coreDrill".into(),
                label: "Core Drill".into(),
                icon_id: "circle-dot".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "core".into(),
                    label: "Core".into(),
                    icon_id: "circle-dot".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "coreLength".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.051), parameter("coreLength", "Core Length", 0.45)],
                    rules: vec![max_rule(StockQuantity::Height, "coreLength", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "rotaryHammer".into(),
                label: "Rotary Hammer".into(),
                icon_id: "hammer".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "anchorHole".into(),
                    label: "Anchor Hole".into(),
                    icon_id: "hammer".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "maxDrillDepth".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.006), parameter("maxDrillDepth", "Max Drill Depth", 0.16), parameter("minStockThickness", "Min Stock Thickness", 0.01)],
                    rules: vec![min_rule(StockQuantity::Height, "minStockThickness", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "anchorSetter".into(),
                label: "Anchor Setter".into(),
                icon_id: "plus".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "anchor".into(),
                    label: "Anchor".into(),
                    icon_id: "plus".into(),
                    recipe: MeasureRecipe::CylinderAttach { radius: "anchorRadius".into(), length: "anchorLength".into() },
                    parameters: vec![parameter("anchorRadius", "Anchor Radius", 0.008), parameter("anchorLength", "Anchor Length", 0.11), parameter("minEmbedment", "Min Embedment", 0.07)],
                    rules: vec![min_rule(StockQuantity::Height, "minEmbedment", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "surfaceGrinder".into(),
                label: "Surface Grinder".into(),
                icon_id: "layers".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "grind".into(),
                    label: "Grind".into(),
                    icon_id: "layers".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "padDiameter".into(), kerf: "grindDepth".into() },
                    parameters: vec![parameter("padDiameter", "Pad Diameter", 0.25), parameter("grindDepth", "Grind Depth", 0.005), parameter("minGrindDimension", "Min Grind Dimension", 0.02)],
                    rules: vec![min_rule(StockQuantity::MinDimension, "minGrindDimension", 0.0)],
                }],
            },
        ]
    }
}

pub fn catalog() -> ConcreteCatalog {
    ConcreteCatalog
}
//#endregion 🔖️Catalog

//#region 🔖️Bundle
const EXTENSION_ID: &str = "process-extension-concrete";
const HOST_APP_ID: &str = "process3d-play";

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `async fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request asking the SDK
// owner to revert those two (plus `TopicContribution::new`) to sync directly, matching the sibling
// reversion already applied to `ExtensionBundle::new`/`.extends`/`.depends_on` in that same impl block.
fn bundle() -> ExtensionBundle {
    let catalog = ConcreteCatalog;
    let bundle = ExtensionBundle::new(EXTENSION_ID, "Process Concrete Machines", "0.1.0").extends("process");
    let bundle = semio_framework::io::resolve_ready(bundle.mode(semio_framework_plugin::ExecutionMode::Declarative));
    semio_framework::io::resolve_ready(bundle.contributes_topic(
        "process.machines",
        serde_json::json!({
            "appId": HOST_APP_ID,
            "moduleId": catalog.catalog_id(),
            "label": catalog.label(),
            "iconId": catalog.icon_id(),
            "machinesJson": serde_json::to_string(&catalog.machines()).unwrap_or_default(),
        }),
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
        let json = serde_json::to_string(&machines).expect("serialize");
        let parsed: Vec<WorkshopMachine> = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, machines);
    }

    #[semio_framework_async_macros::async_test]
    async fn catalog_has_concrete_identity() {
        let catalog = ConcreteCatalog;
        assert_eq!(catalog.catalog_id(), "concrete");
        assert_eq!(catalog.label(), "Concrete");
    }
}
//#endregion 🧪️Tests
