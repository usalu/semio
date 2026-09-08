//! 🧩️ Process wood machine catalog extension — contributes wood-shop machines to `process3d-play`.

use semio_framework_plugin::ExtensionBundle;
use semio_s_artifact_process_process3d::{Capability, CapabilityParameter, CapabilityRule, MachineCatalog, MeasureRecipe, StockQuantity, WorkshopMachine};

//#region 🔖️Catalog
pub struct WoodCatalog;

fn parameter(id: &str, label: &str, value: f64) -> CapabilityParameter {
    CapabilityParameter { id: id.into(), label: label.into(), value }
}

fn max_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Max { quantity, parameter: parameter.into(), margin }
}

fn min_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Min { quantity, parameter: parameter.into(), margin }
}

impl MachineCatalog for WoodCatalog {
    fn catalog_id(&self) -> &str {
        "wood"
    }

    fn label(&self) -> &str {
        "Wood"
    }

    fn icon_id(&self) -> &str {
        "beam"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
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

pub fn catalog() -> WoodCatalog {
    WoodCatalog
}
//#endregion 🔖️Catalog

//#region 🔖️Bundle
const EXTENSION_ID: &str = "process-extension-wood";
const HOST_APP_ID: &str = "process3d-play";

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request asking the SDK
// owner to revert those two (plus `TopicContribution::new`) to sync directly, matching the sibling
// reversion already applied to `ExtensionBundle::new`/`.extends`/`.depends_on` in that same impl block.
fn bundle() -> ExtensionBundle {
    let catalog = WoodCatalog;
    let bundle = ExtensionBundle::new(EXTENSION_ID, "Process Wood Machines", "0.1.0").extends("process");
    let bundle = bundle.mode(semio_framework_plugin::ExecutionMode::Declarative);
    bundle.contributes_topic(
        "process.machines",
        semio_framework_os_kernel::DslValue::object([
            ("appId".to_string(), semio_framework_os_kernel::DslValue::String(HOST_APP_ID.to_string())),
            ("moduleId".to_string(), semio_framework_os_kernel::DslValue::String(catalog.catalog_id().to_string())),
            ("label".to_string(), semio_framework_os_kernel::DslValue::String(catalog.label().to_string())),
            ("iconId".to_string(), semio_framework_os_kernel::DslValue::String(catalog.icon_id().to_string())),
            ("machinesJson".to_string(), semio_framework_os_kernel::DslValue::String(semio_framework_os_kernel::json::to_json_string(&catalog.machines()))),
        ]),
    )
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
