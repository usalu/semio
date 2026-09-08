//! 🧩️ Process metal machine catalog extension — contributes metal-shop machines to `process3d-play`.

use semio_framework_plugin::ExtensionBundle;
use semio_s_artifact_process_process3d::{Capability, CapabilityParameter, CapabilityRule, MachineCatalog, MeasureRecipe, StockQuantity, WorkshopMachine};

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
    fn catalog_id(&self) -> &str {
        "metal"
    }

    fn label(&self) -> &str {
        "Metal"
    }

    fn icon_id(&self) -> &str {
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

pub fn catalog() -> MetalCatalog {
    MetalCatalog
}
//#endregion 🔖️Catalog

//#region 🔖️Bundle
const EXTENSION_ID: &str = "process-extension-metal";
const HOST_APP_ID: &str = "process3d-play";

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request asking the SDK
// owner to revert those two (plus `TopicContribution::new`) to sync directly, matching the sibling
// reversion already applied to `ExtensionBundle::new`/`.extends`/`.depends_on` in that same impl block.
fn bundle() -> ExtensionBundle {
    let catalog = MetalCatalog;
    let bundle = ExtensionBundle::new(EXTENSION_ID, "Process Metal Machines", "0.1.0").extends("process");
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
