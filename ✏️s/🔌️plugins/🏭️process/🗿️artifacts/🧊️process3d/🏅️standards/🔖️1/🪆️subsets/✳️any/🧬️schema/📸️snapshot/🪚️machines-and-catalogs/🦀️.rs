//! 🪚️ Machines and catalogs of the process3d snapshot: the stock quantity, measure kind and recipe, the capability rules and parameters, the workshop machine with its patch, the workshop, the generic machine set, the machine catalog trait with its closed dispatch enum (closed in the module of the trait because of macro textual scoping), the contributed catalog and the machines topic payload. See [`super`].

use super::document_records::{ProcessMeasure, WorkingSolid};
use protocol::{Identified, Patchable};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Workshop
/// 📏️ A stock dimension a capability rule checks against a capability's own parameter value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum StockQuantity {
    #[default]
    Width,
    Depth,
    Height,
    MaxDimension,
    MinDimension,
}

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]`'s enum path only supports
/// internally-tagged (`#[value(tag = "…")]`) representations, but `StockQuantity` is a plain
/// unit-only "string enum" — serde's own default (untagged bare-string) representation for an
/// enum with no `#[serde(...)]` attribute at all — so the wire shape here is just the bare variant
/// name, matching what `Serialize`/`Deserialize` already produce for this type today.
impl semio_framework_value::ToValue for StockQuantity {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let name = match self {
            StockQuantity::Width => "width",
            StockQuantity::Depth => "depth",
            StockQuantity::Height => "height",
            StockQuantity::MaxDimension => "maxDimension",
            StockQuantity::MinDimension => "minDimension",
        };
        semio_framework_value::DslValue::String(name.to_string())
    }
}

impl semio_framework_value::FromValue for StockQuantity {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::String(s) => match s.as_str() {
                "width" => Ok(StockQuantity::Width),
                "depth" => Ok(StockQuantity::Depth),
                "height" => Ok(StockQuantity::Height),
                "maxDimension" => Ok(StockQuantity::MaxDimension),
                "minDimension" => Ok(StockQuantity::MinDimension),
                other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown StockQuantity variant `{other}`"))),
            },
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🪚️ Which kernel geometry effect a capability produces — `ProcessMeasure`'s three shapes are the
/// fixed, small vocabulary every machine capability ultimately maps onto via its `MeasureRecipe`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MeasureKind {
    Cut,
    Drill,
    Attach,
}

/// ✅️ "the named stock quantity must be at least/at most the named capability parameter's value (±
/// margin)" — a capability's rules are ANDed together, e.g. a crosscut capability needs stock width
/// AND height above the blade diameter.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum CapabilityRule {
    Min {
        quantity: StockQuantity,
        parameter: String,
        #[dsl(unit = "m")]
        margin: f64,
    },
    Max {
        quantity: StockQuantity,
        parameter: String,
        #[dsl(unit = "m")]
        margin: f64,
    },
}

/// 🔧️ One named numeric parameter of a capability (e.g. blade diameter) — workshop-editable, and
/// referenced by id from the capability's own `MeasureRecipe`/`CapabilityRule`s.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct CapabilityParameter {
    pub id: String,
    pub label: String,
    #[dsl(unit = "m")]
    pub value: f64,
}

/// 🪚️ How a capability's parameters build a kernel `ProcessMeasure` — every field names a
/// `Capability::parameters` entry by id, resolved at measure-build time; `measure_kind()` derives the
/// fixed Cut/Drill/Attach effect so it never needs to be stored redundantly alongside the recipe.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "recipe", rename_all = "camelCase")]
pub enum MeasureRecipe {
    /// ✂️ A disc-shaped cut tool sized from a blade `diameter` and `kerf` (tool thickness).
    DiscCut { diameter: String, kerf: String },
    /// ✂️ A blade-shaped cut tool sized from `kerf` (width), cut `length` (depth), and cut `depth` (height).
    BladeCut { kerf: String, length: String, depth: String },
    /// ✂️ A square pocket cut tool sized from a `diameter` (width/depth) and `depth` (height).
    PocketCut { diameter: String, depth: String },
    /// 🕳️ A cylindrical bore sized from `radius` and `depth`.
    BoreDrill { radius: String, depth: String },
    /// 🔩️ A cylindrical additive component sized from `radius` and `length` (height).
    CylinderAttach { radius: String, length: String },
    /// 🔩️ A box-shaped additive component sized from `width`, `depth`, and `height`.
    BoxAttach { width: String, depth: String, height: String },
}

impl MeasureRecipe {
    pub fn measure_kind(&self) -> MeasureKind {
        match self {
            MeasureRecipe::DiscCut { .. } | MeasureRecipe::BladeCut { .. } | MeasureRecipe::PocketCut { .. } => MeasureKind::Cut,
            MeasureRecipe::BoreDrill { .. } => MeasureKind::Drill,
            MeasureRecipe::CylinderAttach { .. } | MeasureRecipe::BoxAttach { .. } => MeasureKind::Attach,
        }
    }
}

/// 🌉️ Hand `dsl::DslField` impl — `MeasureRecipe` is a `DslEnum` (`DslVariants` only), and
/// `Capability::recipe` is a REQUIRED, never-optional field that must stay a bare `MeasureRecipe`.
/// 🏷️ A tagged enum stored in one field is a one-statement `Shape::Statements` keyed by its variant.
include!("../../../🚪️io/🪶️sqlite/📸️snapshot/🪆️native-fields/🦀️.rs");

controlled_tagged_variant_field!(MeasureRecipe, WorkingSolid, ProcessMeasure);

/// 🪚️ One thing a machine can do; every capability turns into a step: `recipe` fixes the geometric
/// effect and how it's sized, `parameters` size the tool, `rules` gate legality against the stock.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct Capability {
    pub id: String,
    pub label: String,
    pub icon_id: String,
    pub recipe: MeasureRecipe,
    #[value(default)]
    pub parameters: Vec<CapabilityParameter>,
    #[value(default)]
    #[dsl(statements, block)]
    pub rules: Vec<CapabilityRule>,
}

/// 🛠️ A machine in the document's workshop — an embedded snapshot, never a reference; consistent with
/// `StepOrigin`'s never-resolve invariant (see its doc comment), and robust to catalog drift: editing
/// or removing an installed catalog can never retroactively change an already-configured workshop.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WorkshopMachine {
    pub id: String,
    pub label: String,
    pub icon_id: String,
    /// 🏷️ Which installed catalog this snapshot was seeded from — informational only, never resolved
    /// (a machine stays fully usable after its source catalog is uninstalled).
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[value(default)]
    pub capabilities: Vec<Capability>,
}

impl Identified<String> for WorkshopMachine {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Sparse edit for a `WorkshopMachine` — `None` fields are left untouched.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WorkshopMachinePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<Capability>>,
}

impl Patchable<WorkshopMachinePatch> for WorkshopMachine {
    fn apply_patch(&mut self, patch: &WorkshopMachinePatch) {
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(icon_id) = &patch.icon_id {
            self.icon_id = icon_id.clone();
        }
        if let Some(capabilities) = &patch.capabilities {
            self.capabilities = capabilities.clone();
        }
    }
}

/// 🏭️ The document's configured workshop: the machines available to build steps from.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct Workshop {
    #[value(default)]
    pub machines: Vec<WorkshopMachine>,
}

impl Default for Workshop {
    fn default() -> Self {
        Self { machines: generic_machines() }
    }
}

/// 📦️ The three built-in generic machines (saw/drill/attacher), reproducing the same default tool
/// sizes `ProcessMeasure` used before capabilities existed — pure data with no catalog dependency, so
/// every document (including ones deserialized without a `workshop` field) always has a working
/// workshop and the utility bar's click-to-place cut/drill/attach never dead-ends.
pub fn generic_machines() -> Vec<WorkshopMachine> {
    vec![
        WorkshopMachine {
            id: "saw".into(),
            label: "Generic Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "cut".into(),
                label: "Cut".into(),
                icon_id: "scissors".into(),
                recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "length".into(), depth: "depth".into() },
                parameters: vec![
                    CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.05 },
                    CapabilityParameter { id: "length".into(), label: "Length".into(), value: 0.5 },
                    CapabilityParameter { id: "depth".into(), label: "Depth".into(), value: 0.5 },
                ],
                rules: Vec::new(),
            }],
        },
        WorkshopMachine {
            id: "drill".into(),
            label: "Generic Drill".into(),
            icon_id: "circle-dot".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "drill".into(),
                label: "Drill".into(),
                icon_id: "circle-dot".into(),
                recipe: MeasureRecipe::BoreDrill { radius: "radius".into(), depth: "depth".into() },
                parameters: vec![CapabilityParameter { id: "radius".into(), label: "Radius".into(), value: 0.05 }, CapabilityParameter { id: "depth".into(), label: "Depth".into(), value: 0.3 }],
                rules: Vec::new(),
            }],
        },
        WorkshopMachine {
            id: "attacher".into(),
            label: "Generic Attacher".into(),
            icon_id: "plus".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "attach".into(),
                label: "Attach".into(),
                icon_id: "plus".into(),
                recipe: MeasureRecipe::CylinderAttach { radius: "radius".into(), length: "length".into() },
                parameters: vec![CapabilityParameter { id: "radius".into(), label: "Radius".into(), value: 0.03 }, CapabilityParameter { id: "length".into(), label: "Length".into(), value: 0.2 }],
                rules: Vec::new(),
            }],
        },
    ]
}

/// 🧩️ A machine catalog: contributes machines (with capabilities) the workshop configurator can
/// install into a document's workshop. Implemented by the built-in generic/domain catalogs
/// (`crate::schema::{GenericCatalog, MetalCatalog, WoodCatalog, RoboticCatalog,
/// ConcreteCatalog}`) and by the app's runtime-contributed `ContributedMachineCatalog`
/// (`crate::editor::process3d::ContributedMachineCatalog`) — closed into `MachineCatalogs` below
/// (O1: dyn dispatch is banned from trait-method return position; R11: closed implementor set ⇒
/// `dyn_enum_close!`, never a box).
// 🚫️async: E1 pure — every implementor (5 in `schema`, `editor`'s `ContributedMachineCatalog`, plus
// each `process-extension-*` crate's local catalog) is a zero-suspension struct-literal/field
// accessor; every caller (`editor::installed_catalogs`/`catalog_machine`, each extension's `bundle()`)
// already consumes these unawaited — see R9.
#[dyn_enum]
pub trait MachineCatalog {
    fn catalog_id(&self) -> &str;
    fn label(&self) -> &str;
    fn icon_id(&self) -> &str;
    fn machines(&self) -> Vec<WorkshopMachine>;
}

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The closed set of `MachineCatalog` implementors. Closed HERE, in the same module as `#[dyn_enum]`
    /// (not at the `editor::installed_catalogs` call site that gathers them): the generated
    /// `__semio_dispatch_MachineCatalog!` is `#[macro_export]`ed, and rustc rejects any reference to a
    /// `macro_export`ed macro produced by expansion IN THE SAME CRATE via an absolute/`crate::`-qualified
    /// path (rust-lang/rust#52234) — verified directly against real rustc: even the documented
    /// `use crate::__semio_dispatch_MachineCatalog;` cross-module recipe (`📓️terra-dyn-enum-macro-report.md`
    /// finding 1) still hits this error when the closing site is a genuine sibling module tree (`editor` is
    /// a top-level sibling of `artifacts`, not a descendant of `process3d`), because that `use` is itself an
    /// absolute path. Only a BARE invocation in the trait's OWN literal module resolves it, via ordinary
    /// `macro_rules!` textual scoping — so the enum lives here, and `editor::installed_catalogs` imports
    /// `MachineCatalogs` like any other type.
    pub enum MachineCatalogs: MachineCatalog {
        Generic(crate::GenericCatalog),
        Metal(crate::MetalCatalog),
        Wood(crate::WoodCatalog),
        Robotic(crate::RoboticCatalog),
        Concrete(crate::ConcreteCatalog),
        Contributed(crate::snapshot::ContributedMachineCatalog),
    }
}
//#endregion 🔖️Workshop

//#region 🔧️Behavior
/// 🧩️ One hot-installed machine catalog deserialized from the `"process.machines"` topic contribution.
/// 🔓️ `pub` (not private): a variant payload of `crate::MachineCatalogs`
/// (closed in the trait's own module, not here — see that enum's doc comment for why), which is
/// itself `pub` — a variant field can never be less visible than the enum wrapping it.
#[derive(Clone)]
pub struct ContributedMachineCatalog {
    pub catalog_id: String,
    pub(super) label: String,
    icon_id: String,
    machines: Vec<WorkshopMachine>,
}

impl ContributedMachineCatalog {
    /// 🧩️ Seals one contributed catalog from the identity and machine list the topic contribution decoded.
    pub fn new(catalog_id: String, label: String, icon_id: String, machines: Vec<WorkshopMachine>) -> Self {
        Self { catalog_id, label, icon_id, machines }
    }
}

impl MachineCatalog for ContributedMachineCatalog {
    fn catalog_id(&self) -> &str {
        &self.catalog_id
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn icon_id(&self) -> &str {
        &self.icon_id
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        self.machines.clone()
    }
}
//#endregion 🔧️Behavior

//#region 🔧️Behavior
//#region 🔖️ProcessMachinesTopicPayload
/// 🗂️ `topic_contribution.payload` shape for the `"process.machines"` topic — the sole shape
/// `contributed_machine_catalogs` decodes. See `TopicContribution` in
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`.
#[derive(semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ProcessMachinesTopicPayload {
    app_id: String,
    module_id: String,
    pub(super) label: String,
    icon_id: String,
    machines_json: String,
}
//#endregion 🔖️ProcessMachinesTopicPayload
//#endregion 🔧️Behavior
