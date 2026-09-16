//! 🎮️ CAD interaction statechart — a generic interpreter over every `spatial.interaction` JSON asset
//! under `📚️examples/🖼️assets/🏗️modelDefinitions/*/🕹️interactions/` (60 assets across eight model
//! definitions, all embedded at build time), plus the commit-action runner mapping each spec's
//! `commit.operation.action` onto the document (`CommitOutcome`). There is no hand-written
//! statechart any more: the four `aec.building` placements that used to be bespoke are the
//! `placeWall`/`placeBeam`/`placeColumn`/`placeSlab` assets like every other interaction.

use crate::standards::v1::subsets::any::io::geometry_import::{CadObject, CadPrimitiveSlot};
use crate::{evaluate_expr, CadPaneId, DisplayItemSpec, Effect, ExprEnv, ExprPathRoot, ExprPathSegment, ExprPathTarget, InteractionSpec};

use protocol::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::{Brep, BrepKernel};
use std::collections::HashMap;
use std::sync::OnceLock;

//#region 🔖️Types
/// 🧮️ Arbitrary interaction-statechart scratch data — genuinely opaque JSON with no `dsl`/`value`
/// shape (per `CadConfig`'s own doc comment on `engagement_session_json`), so `ToValue`/`FromValue`
/// are hand-written here rather than derived: `#[value(transparent)]` would need
/// `HashMap<String, DslValue>: ToValue + FromValue`, which the blanket derive path does not cover
/// for a bare `HashMap` wrapper. Builds/reads the `DslValue::Object` tree directly — no
/// `serde_json` bridging (this used to route through `serde_json::Value` before this crate's own
/// `DslValue` object variant existed).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CadEngagementContext(pub HashMap<String, DslValue>);

impl protocol::ToValue for CadEngagementContext {
    /// 🔤️ Keys are emitted SORTED: `engagement_session_json` is compared byte-for-byte against its
    /// persisted twin by `snapshot_of` (the checkpoint-transition guard), and a `HashMap`'s
    /// iteration order differs between two encodes of the same context — which rejected every
    /// `engagementInput` keystroke once a session held two or more context fields.
    fn to_value(&self) -> DslValue {
        let mut entries: Vec<(&String, &DslValue)> = self.0.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        DslValue::object(entries.into_iter().map(|(k, v)| (k.clone(), v.clone())))
    }
}

impl protocol::FromValue for CadEngagementContext {
    fn from_value(value: DslValue) -> Result<Self, protocol::ValueError> {
        match value {
            DslValue::Object(entries) => Ok(Self(entries.into_iter().collect())),
            DslValue::Null => Ok(Self::default()),
            other => Err(protocol::ValueError::new(format!("expected an object for CadEngagementContext, got {other:?}"))),
        }
    }
}

impl std::ops::Deref for CadEngagementContext {
    type Target = HashMap<String, DslValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CadEngagementContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadEngagementScratch {
    pub interaction_id: String,
    pub state: String,
    pub context: CadEngagementContext,
    pub pane: CadPaneId,
    #[value(default)]
    pub last_response: Option<String>,
}

#[derive(Clone, Debug)]
pub struct KeyedTransition {
    pub key: String,
    pub label: String,
    pub event_kind: String,
}

#[derive(Clone, Debug)]
pub struct InteractionCatalogEntry {
    pub id: String,
    pub label: String,
    pub key: String,
    pub model_definition_id: String,
    pub produces_typology: String,
}
//#endregion 🔖️Types

//#region 🔖️Registry
/// `(modelDefinitionId, raw JSON)` for every `🕹️interactions/*.json` asset embedded at build time —
/// pinned against the on-disk tree by `every_interaction_asset_on_disk_parses_as_interaction_spec`.
const RAW_INTERACTION_ASSETS: &[(&str, &str)] = &[
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🌙️arc.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🟨️area.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/➖️booleanDifference.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/✖️booleanIntersection.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/➕️booleanUnion.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/📦️box.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🪓️chamfer.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/⭕️circle.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/〰️constructCurve.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🗺️constructSurface.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🎛️controlPointCurve.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/📋️copy.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/⚓️createAnchor.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🥫️cylinder.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/💥️explode.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🚀️extrudeCrv.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🧱️extrudeWire.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🍥️fillet.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🌊️interpolateCurve.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🔗️join.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/📏️length.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🖊️line.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🎢️loft.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🪞️mirror.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🚚️move.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🕸️networkSrf.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🧭️offsetSurface.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/✈️plane.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/📈️polyline.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🔃️rotate.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/↕️scale1d.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🪗️scale3d.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🌐️sphere.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/✂️split.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🌀️sweep1.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🌪️sweep2.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/✏️trim.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🪵️placeBeam.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/⬆️placeCeiling.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🏛️placeColumn.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🚪️placeDoor.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🪨️placeFoundation.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🚧️placeRailing.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🏠️placeRoof.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🧱️placeSlab.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🪜️placeStair.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🛡️placeWall.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🪟️placeWindow.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🧱️constructBasePlate.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🚧️constructExternalWall.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🚢️constructHull.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🏠️constructRoof.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🪟️constructWindows.json")),
    (
        "aec.building.structure.classic",
        include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🕹️interactions/🧱️constructOneWayRe-72a083.json"),
    ),
    (
        "aec.building.structure.classic",
        include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🕹️interactions/🏛️constructReinforc-411bd6.json"),
    ),
    (
        "aec.building.structure.classic",
        include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🕹️interactions/🛡️constructReinforc-c38891.json"),
    ),
    (
        "aec.building.structure.classic",
        include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🕹️interactions/🚧️constructReinforc-e8fc67.json"),
    ),
    (
        "aec.building.structure.fem.line",
        include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📏️aec.building.structure.fem.line/🕹️interactions/🔣️constructLineElem-0d404b.json"),
    ),
    (
        "aec.building.structure.fem.solid",
        include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🧊️aec.building.structure.fem.solid/🕹️interactions/🔣️constructSolidEle-105046.json"),
    ),
    (
        "aec.building.structure.fem.surface",
        include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🗺️aec.building.structure.fem.surface/🕹️interactions/🔣️constructSurfaceE-7da86f.json"),
    ),
];

fn parse_interaction_spec(raw: &str) -> Option<InteractionSpec> {
    protocol::json::from_json_str(raw).ok()
}

/// 📚️ Every asset parsed ONCE per process — the specs are large (the box alone is 63 KiB of
/// JSON) and every pointer move used to re-parse all of them.
fn parsed_specs() -> &'static [(&'static str, InteractionSpec)] {
    static SPECS: OnceLock<Vec<(&'static str, InteractionSpec)>> = OnceLock::new();
    SPECS.get_or_init(|| RAW_INTERACTION_ASSETS.iter().filter_map(|(model_def, raw)| parse_interaction_spec(raw).map(|spec| (*model_def, spec))).collect())
}

fn spec_by_id(id: &str) -> Option<&'static InteractionSpec> {
    parsed_specs().iter().find(|(_, spec)| spec.id == id).map(|(_, spec)| spec)
}

fn catalog() -> &'static [InteractionCatalogEntry] {
    static CATALOG: OnceLock<Vec<InteractionCatalogEntry>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        parsed_specs()
            .iter()
            .map(|(model_def, spec)| InteractionCatalogEntry {
                id: spec.id.clone(),
                label: spec.label.clone().unwrap_or_else(|| spec.id.clone()),
                key: spec.key.clone().unwrap_or_default(),
                model_definition_id: (*model_def).to_string(),
                produces_typology: spec.produces.typology.clone().unwrap_or_default(),
            })
            .collect()
    })
}
//#endregion 🔖️Registry

//#region 🔖️Catalog
pub fn list_interactions_for_model_definition(model_definition_id: &str) -> Vec<&'static InteractionCatalogEntry> {
    catalog().iter().filter(|entry| entry.model_definition_id == model_definition_id).collect()
}

pub fn resolve_interaction_key(input: &str, model_definition_id: &str) -> Option<&'static InteractionCatalogEntry> {
    let trimmed = input.trim().to_lowercase();
    catalog().iter().find(|entry| entry.model_definition_id == model_definition_id && (entry.key == trimmed || entry.id.eq_ignore_ascii_case(&trimmed) || entry.id.to_lowercase().ends_with(&format!(".{trimmed}"))))
}

pub fn interaction_by_id(id: &str) -> Option<&'static InteractionCatalogEntry> {
    catalog().iter().find(|entry| entry.id == id)
}
//#endregion 🔖️Catalog

//#region 🔖️Statechart
fn vec3_json(point: [f64; 3]) -> DslValue {
    DslValue::Array(vec![DslValue::float(point[0]), DslValue::float(point[1]), DslValue::float(point[2])])
}

fn parse_vec3(value: &DslValue) -> Option<[f64; 3]> {
    let array = value.as_array()?;
    if array.len() < 3 {
        return None;
    }
    Some([array[0].as_f64()?, array[1].as_f64()?, array[2].as_f64()?])
}

pub fn start_session(interaction_id: &str, pane: CadPaneId) -> Option<CadEngagementScratch> {
    let spec = spec_by_id(interaction_id)?;
    Some(CadEngagementScratch { interaction_id: spec.id.clone(), state: spec.machine.initial.clone(), context: CadEngagementContext(HashMap::new()), pane, last_response: None })
}

pub fn keyed_transitions(session: &CadEngagementScratch) -> Vec<KeyedTransition> {
    let Some(spec) = spec_by_id(&session.interaction_id) else {
        return Vec::new();
    };
    let Some(state) = spec.state(&session.state) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for handler in &state.on {
        for transition in &handler.transitions {
            if let Some(key) = &transition.key {
                out.push(KeyedTransition { key: key.clone(), label: transition.label.clone().unwrap_or_else(|| handler.event.clone()), event_kind: handler.event.clone() });
            }
        }
    }
    out
}

pub fn can_commit(session: &CadEngagementScratch) -> bool {
    let Some(spec) = spec_by_id(&session.interaction_id) else {
        return false;
    };
    // 🏁️ `commit.fromStates` names the commit-ready state; 20 assets declare `ready` while their
    // machine ends in a `final` `committed` state instead. A machine that has reached a final state
    // has nothing left to transition to, so a from-state the machine never defines resolves to it.
    let named_from_state = spec.commit.from_states.iter().any(|state| state == &session.state);
    let dangling_from_states = spec.commit.from_states.iter().all(|state| spec.state(state).is_none());
    let at_final = spec.state(&session.state).is_some_and(|state| state.r#final || state.on.is_empty());
    if !named_from_state && !(dangling_from_states && at_final) {
        return false;
    }
    match &spec.commit.when {
        None => true,
        Some(guard_name) => {
            let env = ExprEnv { context: &session.context.0, event: None };
            spec.guard(guard_name, &env)
        }
    }
}

fn context_target_field(target: &ExprPathTarget) -> Option<&str> {
    if target.root != ExprPathRoot::Context {
        return None;
    }
    match target.segments.as_slice() {
        [ExprPathSegment::Field { name }] => Some(name.as_str()),
        _ => None,
    }
}

/// Wraps a raw event payload into the shape the JSON specs' `event.*` path expressions expect:
/// `pointer.down`/`pointer.move` read `event.point`, `set.*` events read `event.value`. Callers
/// (both `lib.rs`'s command handlers and this module's own tests) pass raw values (a `[x,y,z]`
/// array, a bare number) for brevity — already-wrapped objects pass through unchanged.
fn normalize_event_payload(event_kind: &str, payload: Option<&DslValue>) -> Option<DslValue> {
    let payload = payload?;
    if payload.as_object().is_some() {
        return Some(payload.clone());
    }
    if event_kind == "pointer.down" || event_kind == "pointer.move" {
        return Some(DslValue::object([("point".to_string(), payload.clone())]));
    }
    if event_kind.starts_with("set.") {
        return Some(DslValue::object([("value".to_string(), payload.clone())]));
    }
    Some(payload.clone())
}

/// Executes an `action` effect by name.
///
/// `command.addPoint` (used by sphere/circle/etc.) records a named point into a
/// `context[field][key]` map. `box.aabbFromDiagonalCorners` (box's default diagonal-mode second
/// click) derives `context.origin`/`context.corner` — the axis-aligned min/max of `context.diagA`
/// and `event.point` — which `hasValidBox` and the commit params then read.
///
/// The remaining `box.*` rubber-band helpers and selection-driven actions (used only by box's
/// advanced cube/3-point/center sub-modes and by selection-based utilities) are a documented
/// follow-up; they no-operation here rather than error.
fn run_named_action_effect(context: &mut HashMap<String, DslValue>, payload: Option<&DslValue>, action: &str, params: &HashMap<String, DslValue>) {
    match action {
        // 📍️ Keyed: `context[field][key] = point` (a map of named points). Unkeyed (polyline /
        // control-point / interpolate curves): `context[field]` is the ORDERED point list and the
        // point is appended — the specs seed that field with `[]`, so a map is never fabricated.
        "command.addPoint" => {
            let field = params.get("field").and_then(|value| value.as_str()).unwrap_or("points").to_string();
            let key = params.get("key").and_then(|value| value.as_str()).map(str::to_string);
            let point = params.get("point").cloned().unwrap_or(DslValue::Null);
            match key {
                Some(key) => {
                    let entry = context.entry(field).or_insert_with(|| DslValue::object(Vec::new()));
                    if entry.as_object().is_none() {
                        *entry = DslValue::object(Vec::new());
                    }
                    if let DslValue::Object(object) = entry {
                        if let Some(existing) = object.iter_mut().find(|(k, _)| *k == key) {
                            existing.1 = point;
                        } else {
                            object.push((key, point));
                        }
                    }
                }
                None => {
                    let entry = context.entry(field).or_insert_with(|| DslValue::Array(Vec::new()));
                    if !matches!(entry, DslValue::Array(_)) {
                        *entry = DslValue::Array(Vec::new());
                    }
                    if let DslValue::Array(points) = entry {
                        points.push(point);
                    }
                }
            }
        }
        // 🎯️ `context[field] = targets` — the framework `"cad"` selection the engagement commands
        // inject as a `selection.changed` event (see `inject_selection`).
        "command.addSelection" | "command.setSelection" => {
            let field = params.get("field").and_then(|value| value.as_str()).unwrap_or("targets").to_string();
            let targets = params.get("targets").cloned().unwrap_or_else(|| DslValue::Array(Vec::new()));
            context.insert(field, targets);
        }
        // 🧭️ Rubber-band cursor for a constrained move: `vertical` keeps the pick's x/y and reads the
        // height off the pointer, `normal`/`free` follow the ground pick as-is.
        "command.constrainMoveCursor" => {
            let Some(point) = params.get("point").and_then(parse_vec3).or_else(|| payload.and_then(|value| value.get("point")).and_then(parse_vec3)) else { return };
            let mode = context.get("moveMode").and_then(|value| value.as_str()).unwrap_or("free").to_string();
            let from = context.get("points").and_then(|points| points.get("from")).and_then(parse_vec3);
            let cursor = match (mode.as_str(), from) {
                ("vertical", Some(from)) => [from[0], from[1], point[2]],
                _ => point,
            };
            context.insert("cursor".into(), vec3_json(cursor));
        }
        // 📦️ The session has no object geometry in hand; the pick that preceded `confirm` is the
        // anchor a bbox centre would have approximated, so the origin falls back to the world origin.
        "command.selectionBboxCenter" => {
            let field = params.get("field").and_then(|value| value.as_str()).unwrap_or("from").to_string();
            let point = context.get("cursor").and_then(parse_vec3).unwrap_or([0.0, 0.0, 0.0]);
            let entry = context.entry("points".into()).or_insert_with(|| DslValue::object(Vec::new()));
            if let DslValue::Object(object) = entry {
                object.retain(|(k, _)| *k != field);
                object.push((field, vec3_json(point)));
            }
        }
        "command.undoPick" => {
            let field = params.get("field").and_then(|value| value.as_str()).unwrap_or("points").to_string();
            let clear: Vec<String> = params.get("clearKeys").and_then(|value| value.as_array()).map(|keys| keys.iter().filter_map(|key| key.as_str().map(str::to_string)).collect()).unwrap_or_default();
            if let Some(DslValue::Object(object)) = context.get_mut(&field) {
                object.retain(|(k, _)| !clear.iter().any(|key| key == k));
            }
        }
        "box.aabbFromDiagonalCorners" => {
            let diag_a = context.get("diagA").and_then(parse_vec3);
            let second = payload.and_then(|value| value.get("point")).and_then(parse_vec3);
            if let (Some(a), Some(b)) = (diag_a, second) {
                let origin = [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])];
                let corner = [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])];
                context.insert("origin".into(), vec3_json(origin));
                context.insert("corner".into(), vec3_json(corner));
            }
        }
        _ => {}
    }
}

fn apply_effect(session: &mut CadEngagementScratch, payload: Option<&DslValue>, effect: &Effect, raised: &mut Vec<String>) {
    let empty_vars = HashMap::new();
    match effect {
        Effect::Assign { target, value } => {
            if let Some(field) = context_target_field(target) {
                let env = ExprEnv { context: &session.context.0, event: payload };
                let evaluated = evaluate_expr(value, &env, &empty_vars);
                session.context.insert(field.to_string(), evaluated);
            }
        }
        Effect::Clear { target } => {
            if let Some(field) = context_target_field(target) {
                session.context.remove(field);
            }
        }
        Effect::Append { target, value } => {
            if let Some(field) = context_target_field(target) {
                let env = ExprEnv { context: &session.context.0, event: payload };
                let evaluated = evaluate_expr(value, &env, &empty_vars);
                let entry = session.context.entry(field.to_string()).or_insert_with(|| DslValue::Array(Vec::new()));
                if let DslValue::Array(array) = entry {
                    array.push(evaluated);
                } else {
                    *entry = DslValue::Array(vec![evaluated]);
                }
            }
        }
        Effect::Raise { event } => raised.push(event.clone()),
        Effect::Action { action, params, .. } => {
            let env = ExprEnv { context: &session.context.0, event: payload };
            let evaluated: HashMap<String, DslValue> = params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, &env, &empty_vars))).collect();
            run_named_action_effect(&mut session.context, payload, action, &evaluated);
        }
        // Emit/OpenTransaction/CommitTransaction/RollbackTransaction/RequestPreview/KernelQuery/
        // ResolveEditable/SetDiagnostic/ClearDiagnostic/InteractionCall are not yet interpreted —
        // InteractionCall (nested sub-interaction composition) is a documented follow-up used only
        // by the curve-drawing sub-flow (`mode.curve`); the primary `mode.2points` flow doesn't
        // depend on it. The others have no observable effect on committed geometry.
        _ => {}
    }
}

fn apply_event_generic(session: &mut CadEngagementScratch, event_kind: &str, raw_payload: Option<&DslValue>, depth: u8) -> bool {
    if depth > 8 {
        return false;
    }
    let Some(spec) = spec_by_id(&session.interaction_id) else {
        return false;
    };
    let Some(state) = spec.state(&session.state) else {
        return false;
    };
    let Some(handler) = state.on.iter().find(|handler| handler.event == event_kind) else {
        return false;
    };
    let normalized = normalize_event_payload(event_kind, raw_payload);
    let payload = normalized.as_ref();
    let chosen = handler.transitions.iter().find(|transition| match &transition.guard {
        None => true,
        Some(name) => {
            let env = ExprEnv { context: &session.context.0, event: payload };
            spec.guard(name, &env)
        }
    });
    let Some(transition) = chosen else {
        return false;
    };
    let mut raised = Vec::new();
    for effect in &transition.effects {
        apply_effect(session, payload, effect, &mut raised);
    }
    if let Some(target) = &transition.target {
        session.state = target.clone();
    }
    session.last_response = Some("OK".into());
    for raised_event in raised {
        apply_event_generic(session, &raised_event, None, depth + 1);
    }
    true
}

pub fn apply_event(session: &mut CadEngagementScratch, event_kind: &str, payload: Option<&DslValue>) -> bool {
    apply_event_generic(session, event_kind, payload, 0)
}

/// States where a numeric-only line commits the pending height (premigration `tryCommitNumericEntry`).
const NUMERIC_ENTRY_STATES: &[&str] = &["first_corner_height", "two_points_height", "slab_height", "column_height", "radius", "curve_height"];

/// 🔢️ The `set.*` event a bare number means in `session`'s current state — the spec's own
/// `interaction.scalarEntry` row for the state first, then the state's declared `set.*` handler
/// (`set.radius` in a `radius` state, `set.angle`, `set.factor`, …), then the legacy height states.
pub fn numeric_entry_event(session: &CadEngagementScratch) -> Option<String> {
    if let Some(spec) = spec_by_id(&session.interaction_id) {
        if let Some(entry) = spec.interaction.scalar_entry.iter().find(|entry| entry.state == session.state) {
            return Some(entry.event.clone());
        }
        if let Some(state) = spec.state(&session.state) {
            if let Some(handler) = state.on.iter().find(|handler| handler.event.starts_with("set.")) {
                return Some(handler.event.clone());
            }
        }
    }
    NUMERIC_ENTRY_STATES.contains(&session.state.as_str()).then(|| "set.height".to_string())
}

/// 🎯️ True when the current state listens for `selection.changed` — the engagement commands then
/// feed it the live `"cad"` domain selection before their own event.
pub fn accepts_selection(session: &CadEngagementScratch) -> bool {
    spec_by_id(&session.interaction_id).and_then(|spec| spec.state(&session.state).map(|state| state.on.iter().any(|handler| handler.event == "selection.changed"))).unwrap_or(false)
}

/// 🎯️ Feeds the framework selection into the session as one `selection.changed` event (targets =
/// the domain's object ids), only where the current state declares that handler.
pub fn inject_selection(session: &mut CadEngagementScratch, ids: &[String]) -> bool {
    if ids.is_empty() || !accepts_selection(session) {
        return false;
    }
    // 🎯️ Each target is `{ id, kind }` — the shape the specs' entity guards read (`curves[0].id`).
    let targets = DslValue::Array(ids.iter().map(|id| DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("kind".to_string(), DslValue::String("object".into()))])).collect());
    apply_event(session, "selection.changed", Some(&DslValue::object([("targets".to_string(), targets)])))
}

/// 🪧️ The prompt the current state shows: its `selection.prompt`, else the display `label` item in
/// the `prompt` role, else the state name spelled out.
pub fn state_prompt(session: &CadEngagementScratch) -> String {
    if let Some(spec) = spec_by_id(&session.interaction_id) {
        if let Some(prompt) = spec.state(&session.state).and_then(|state| state.selection.as_ref()).and_then(|selection| selection.prompt.clone()) {
            return prompt;
        }
        if let Some(text) = spec.display.states.iter().find(|state| state.state == session.state).and_then(|state| {
            state.items.iter().find_map(|item| match item {
                DisplayItemSpec::Label { role, text, .. } if role.as_deref() == Some("prompt") => Some(text.clone()),
                _ => None,
            })
        }) {
            return text;
        }
    }
    session.state.replace('_', " ")
}

fn strip_prefix_ignore_case<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    if text.len() < prefix.len() {
        return None;
    }
    let (head, tail) = text.split_at(prefix.len());
    head.eq_ignore_ascii_case(prefix).then_some(tail)
}

/// Parses a REPL command line into an `(event_kind, payload)` pair.
///
/// `current_state` is the active engagement session's state (if any) — required to disambiguate a
/// bare numeric line (e.g. `"3.5"`) as a height commit only while a numeric-entry state is active,
/// mirroring premigration's `trySubmitLine` numeric-entry step.
pub fn parse_repl_line(line: &str, current_state: Option<&str>) -> Option<(String, Option<DslValue>)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Legacy raw forms (still used by the wgpu renderer's REPL, which does not PascalCase drafts).
    if let Some(rest) = trimmed.strip_prefix("set.height ") {
        return rest.trim().parse::<f64>().ok().map(|height| ("set.height".into(), Some(DslValue::float(height))));
    }
    if let Some(rest) = trimmed.strip_prefix("dist ") {
        return rest.trim().parse::<f64>().ok().map(|distance| ("set.distance".into(), Some(DslValue::float(distance))));
    }
    // Normalized forms: the React shell's engagement input PascalCases every draft (no separators),
    // so `set.height 3.5` arrives as `SetHeight3.5` (framework/renderer/react `Engagement.applyDraft`
    // via `normalizeEngagementCommandText`).
    if let Some(rest) = strip_prefix_ignore_case(trimmed, "SetHeight") {
        if let Ok(height) = rest.parse::<f64>() {
            return Some(("set.height".into(), Some(DslValue::float(height))));
        }
    }
    if let Some(rest) = strip_prefix_ignore_case(trimmed, "Dist") {
        if let Ok(distance) = rest.parse::<f64>() {
            return Some(("set.distance".into(), Some(DslValue::float(distance))));
        }
    }
    // `SetRadius2.5` / `SetAngle45` / `SetFactor2` — every `set.<field>` scalar event the specs
    // declare, PascalCased by the shell exactly like `SetHeight`.
    if let Some(rest) = strip_prefix_ignore_case(trimmed, "Set") {
        let digits = rest.find(|character: char| character.is_ascii_digit() || character == '-' || character == '.').unwrap_or(rest.len());
        let (field, number) = rest.split_at(digits);
        if !field.is_empty() && field.chars().all(|character| character.is_ascii_alphabetic()) {
            if let Ok(value) = number.parse::<f64>() {
                return Some((format!("set.{}", field.to_ascii_lowercase()), Some(DslValue::float(value))));
            }
        }
    }
    // Bare numeric entry commits height while a numeric-entry state is active.
    if current_state.is_some_and(|state| NUMERIC_ENTRY_STATES.contains(&state)) {
        if let Ok(height) = trimmed.parse::<f64>() {
            return Some(("set.height".into(), Some(DslValue::float(height))));
        }
    }
    Some((trimmed.into(), None))
}

/// 🔢️ `parse_repl_line` with the session's own numeric-entry event: a bare number in a state whose
/// spec declares a `set.*` handler becomes that event (`radius` → `set.radius`).
pub fn parse_repl_line_for(line: &str, session: Option<&CadEngagementScratch>) -> Option<(String, Option<DslValue>)> {
    if let (Some(session), Ok(value)) = (session, line.trim().parse::<f64>()) {
        if let Some(event) = numeric_entry_event(session) {
            return Some((event, Some(DslValue::float(value))));
        }
    }
    parse_repl_line(line, session.map(|session| session.state.as_str()))
}
//#endregion 🔖️Statechart

//#region 🔖️CommitRunner
fn commit_primitive_box(kernel: &mut Brep, params: &HashMap<String, DslValue>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let corner_a = params.get("cornerA").and_then(parse_vec3)?;
    let corner_b = params.get("cornerB").and_then(parse_vec3)?;
    let height = params.get("height").and_then(|value| value.as_f64()).unwrap_or(1.0);
    let width = (corner_b[0] - corner_a[0]).abs().max(0.05);
    let depth = (corner_b[1] - corner_a[1]).abs().max(0.05);
    let solid = kernel.box_prim(width, depth, height.max(0.05)).ok()?;
    Some(CadObject {
        id: next_id("object"),
        label: format!("Box {}", label_count + 1),
        typology: "spatial.shape.primitive.box".into(),
        visible: true,
        locked: false,
        origin: corner_a,
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        mesh_url: None,
        extent: Some([width, depth, height.max(0.05)]),
        solid_handle: Some(solid.0.clone()),
        primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
    })
}

/// Generic commit for the "2 points + height" family shared by every `aec.building.energy`,
/// `aec.building.structure.classic`, and `aec.building.structure.fem.*` construction interaction
/// (`commit.operation.action` ending in `From2PointsAndHeight`/`FromSurface`) — differentiated only
/// by the `typology` commit param.
fn commit_from_2_points_and_height(kernel: &mut Brep, params: &HashMap<String, DslValue>, label: &str, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let typology = params.get("typology").and_then(|value| value.as_str()).unwrap_or("").to_string();
    let lower = typology.to_lowercase();
    let point_a = params.get("pointA").and_then(parse_vec3)?;
    let height = params.get("height").and_then(|value| value.as_f64()).unwrap_or(3.0);

    if lower.contains("column") {
        let radius = 0.25;
        let solid = kernel.cylinder_prim(radius, height.max(0.05)).ok()?;
        return Some(CadObject {
            id: next_id("object"),
            label: format!("{label} {}", label_count + 1),
            typology,
            visible: true,
            locked: false,
            origin: point_a,
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: Some([radius * 2.0, radius * 2.0, height.max(0.05)]),
            solid_handle: Some(solid.0.clone()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
        });
    }

    let point_b = params.get("pointB").and_then(parse_vec3)?;
    let span = ((point_b[0] - point_a[0]).powi(2) + (point_b[1] - point_a[1]).powi(2)).sqrt().max(0.5);
    let (width, depth, solid_height) = if lower.contains("wall") {
        (span, 0.2, height.max(0.05))
    } else if lower.contains("windows") {
        (span, 0.05, height.max(0.05))
    } else {
        // slab / baseplate / roof / hull / fem elements: flat footprint extruded by `height`.
        let w = (point_b[0] - point_a[0]).abs().max(0.5);
        let d = (point_b[1] - point_a[1]).abs().max(0.5);
        (w, d, height.max(0.05))
    };
    let solid = kernel.box_prim(width, depth, solid_height).ok()?;
    Some(CadObject {
        id: next_id("object"),
        label: format!("{label} {}", label_count + 1),
        typology,
        visible: true,
        locked: false,
        origin: point_a,
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        mesh_url: None,
        extent: Some([width, depth, solid_height]),
        solid_handle: Some(solid.0.clone()),
        primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
    })
}

/// 🧱️ One solid object sized by `extent` at `origin` — the persisted shape every committed solid takes
/// (`CadObjectSpec` carries typology + placement + extent; `typology_brep_mesh` rebuilds the kernel
/// primitive from those at render time, so the ephemeral handle minted here only seeds the first
/// tessellation).
fn solid_object(kernel: &mut Brep, typology: &str, label: String, origin: [f64; 3], extent: [f64; 3], orientation: [f64; 4], next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let extent = [extent[0].max(0.02), extent[1].max(0.02), extent[2].max(0.02)];
    let solid = match crate::standards::v1::subsets::any::schema::inferences::typology_mesh_kind(typology) {
        "cylinder" => kernel.cylinder_prim(extent[0].max(extent[1]) * 0.5, extent[2]).ok()?,
        "sphere" => kernel.sphere_prim(extent[0].max(extent[1]).max(extent[2]) * 0.5).ok()?,
        _ => kernel.box_prim(extent[0], extent[1], extent[2]).ok()?,
    };
    Some(CadObject {
        id: next_id("object"),
        label,
        typology: typology.into(),
        visible: true,
        locked: false,
        origin,
        orientation: Some(orientation),
        scale: None,
        mesh_url: None,
        extent: Some(extent),
        solid_handle: Some(solid.0.clone()),
        primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
    })
}

/// 🧭️ Unit quaternion (x, y, z, w) rotating +X onto `direction` — yaw about Z then pitch about the
/// rotated Y, which is how a bar-shaped solid (beam, railing, line thickness) follows a segment.
pub fn quaternion_from_x_to(direction: [f64; 3]) -> [f64; 4] {
    let length = (direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2]).sqrt();
    if length <= f64::EPSILON {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let yaw = direction[1].atan2(direction[0]);
    let pitch = -(direction[2] / length).asin();
    let (sy, cy) = (yaw * 0.5).sin_cos();
    let (sp, cp) = (pitch * 0.5).sin_cos();
    // q = q_yaw(Z) * q_pitch(Y)
    [-sy * sp, cy * sp, sy * cp, cy * cp]
}

/// 📏️ A bar of `thickness × thickness` cross-section from `p0` to `p1` — the persisted shape of a
/// line-like result (a `curve.line`, a placed beam/railing, one polyline segment).
fn bar_object(kernel: &mut Brep, typology: &str, label: String, p0: [f64; 3], p1: [f64; 3], thickness: [f64; 2], next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let direction = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let length = (direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2]).sqrt();
    if length <= 1e-6 {
        return None;
    }
    solid_object(kernel, typology, label, p0, [length, thickness[0], thickness[1]], quaternion_from_x_to(direction), next_id)
}

fn context_points_list(context: &HashMap<String, DslValue>) -> Vec<[f64; 3]> {
    context.get("points").and_then(|value| value.as_array()).map(|points| points.iter().filter_map(parse_vec3).collect()).unwrap_or_default()
}

fn context_named_point(context: &HashMap<String, DslValue>, key: &str) -> Option<[f64; 3]> {
    context.get("points").and_then(|points| points.get(key)).and_then(parse_vec3)
}

fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// 📈️ One bar per consecutive point pair (closed polylines add the closing segment) — a polyline is
/// persisted as N segment solids sharing one label prefix, the shape `CadObjectSpec` can carry today.
fn polyline_objects(kernel: &mut Brep, typology: &str, label: &str, points: &[[f64; 3]], closed: bool, label_count: usize, next_id: &impl Fn(&str) -> String) -> Vec<CadObject> {
    let mut objects = Vec::new();
    let mut pairs: Vec<([f64; 3], [f64; 3])> = points.windows(2).map(|pair| (pair[0], pair[1])).collect();
    if closed && points.len() > 2 {
        pairs.push((points[points.len() - 1], points[0]));
    }
    for (index, (p0, p1)) in pairs.into_iter().enumerate() {
        if let Some(object) = bar_object(kernel, typology, format!("{label} {}.{}", label_count + 1, index + 1), p0, p1, [0.02, 0.02], next_id) {
            objects.push(object);
        }
    }
    objects
}

/// 🎯️ What a committed session asks the document to do.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CommitOutcome {
    /// 🧱️ New objects for the session's pane.
    Objects(Vec<CadObject>),
    /// 🚚️ Move the selected `targets` by `delta`.
    Move { targets: Vec<String>, delta: [f64; 3] },
    /// 📋️ Duplicate the selected `targets`, the copies offset by `delta`.
    Copy { targets: Vec<String>, delta: [f64; 3] },
    /// 🌀️ Rotate the selected `targets` about `axis` by `angle` (radians).
    Rotate { targets: Vec<String>, axis: [f64; 3], angle: f64 },
    /// ⚖️ Scale the selected `targets` by `factors`.
    Scale { targets: Vec<String>, factors: [f64; 3] },
    /// 🚧️ A commit action the persisted object model cannot represent yet (booleans, fillets, lofts,
    /// sweeps, edits on existing topology) — the session still closes, and the HUD says why.
    Unsupported(String),
}

/// 🎯️ The selected object ids a `command.addSelection` stored under `targets` — `{ id, kind }`
/// records from `inject_selection`, or bare id strings from a hand-authored payload.
fn context_targets(context: &HashMap<String, DslValue>) -> Vec<String> {
    context
        .get("targets")
        .and_then(|value| value.as_array())
        .map(|targets| targets.iter().filter_map(|target| target.as_str().map(str::to_string).or_else(|| target.get("id").and_then(|id| id.as_str()).map(str::to_string))).collect())
        .unwrap_or_default()
}

/// `command.finish` dispatches by the commit's `resultKind` param, reading whatever context fields
/// that interaction's machine populated (`points.<key>`, `radius`, `height`, `angle`, …).
fn commit_command_finish(kernel: &mut Brep, params: &HashMap<String, DslValue>, context: &HashMap<String, DslValue>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CommitOutcome> {
    let result_kind = params.get("resultKind").and_then(|value| value.as_str())?;
    let radius_of = |center: [f64; 3]| context.get("radius").and_then(|value| value.as_f64()).or_else(|| context_named_point(context, "radiusPoint").map(|point| distance(point, center))).unwrap_or(1.0).max(0.02);
    match result_kind {
        "sphere" => {
            let center = context_named_point(context, "center")?;
            let radius = radius_of(center);
            let object = solid_object(kernel, "spatial.shape.solid.sphere", format!("Sphere {}", label_count + 1), center, [radius * 2.0; 3], [0.0, 0.0, 0.0, 1.0], next_id)?;
            Some(CommitOutcome::Objects(vec![object]))
        }
        "cylinder" => {
            let base = context_named_point(context, "base")?;
            let radius = radius_of(base);
            let height = context.get("height").and_then(|value| value.as_f64()).or_else(|| context_named_point(context, "end").map(|end| (end[2] - base[2]).abs().max(distance(end, base)))).unwrap_or(1.0).max(0.02);
            let object = solid_object(kernel, "spatial.shape.solid.cylinder", format!("Cylinder {}", label_count + 1), base, [radius * 2.0, radius * 2.0, height], [0.0, 0.0, 0.0, 1.0], next_id)?;
            Some(CommitOutcome::Objects(vec![object]))
        }
        "plane" => {
            let a = context_named_point(context, "cornerA")?;
            let b = context_named_point(context, "cornerB")?;
            let origin = [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])];
            let object = solid_object(kernel, "spatial.shape.surface.plane", format!("Plane {}", label_count + 1), origin, [(b[0] - a[0]).abs(), (b[1] - a[1]).abs(), 0.02], [0.0, 0.0, 0.0, 1.0], next_id)?;
            Some(CommitOutcome::Objects(vec![object]))
        }
        "circle" | "arc" => {
            let center = context_named_point(context, "center")?;
            let radius = if result_kind == "arc" { context_named_point(context, "start").map(|start| distance(start, center)).unwrap_or(1.0).max(0.02) } else { radius_of(center) };
            // ⭕️ A circle/arc is persisted as its closed polygonal polyline (32 segments, arcs the swept
            // share) — thin bars are the only curve shape `CadObjectSpec` carries today.
            let sweep = if result_kind == "arc" {
                let start = context_named_point(context, "start")?;
                let start_angle = (start[1] - center[1]).atan2(start[0] - center[0]);
                let end_angle = context.get("angle").and_then(|value| value.as_f64()).map(|degrees| start_angle + degrees.to_radians()).or_else(|| context_named_point(context, "end").map(|end| (end[1] - center[1]).atan2(end[0] - center[0])))?;
                let mut sweep = end_angle - start_angle;
                if sweep <= 0.0 {
                    sweep += std::f64::consts::TAU;
                }
                (start_angle, sweep)
            } else {
                (0.0, std::f64::consts::TAU)
            };
            let segments = ((sweep.1 / std::f64::consts::TAU) * 32.0).ceil().max(2.0) as usize;
            let points: Vec<[f64; 3]> = (0..=segments).map(|index| {
                let angle = sweep.0 + sweep.1 * (index as f64 / segments as f64);
                [center[0] + radius * angle.cos(), center[1] + radius * angle.sin(), center[2]]
            }).collect();
            let (typology, label) = if result_kind == "arc" { ("spatial.shape.curve.arc", "Arc") } else { ("spatial.shape.curve.circle", "Circle") };
            let objects = polyline_objects(kernel, typology, label, &points, false, label_count, &next_id);
            (!objects.is_empty()).then_some(CommitOutcome::Objects(objects))
        }
        "curve" | "interpolateCurve" => {
            let points = context_points_list(context);
            let (typology, label) = if result_kind == "curve" { ("spatial.shape.curve.control-point-curve", "Curve") } else { ("spatial.shape.curve.interpolate-curve", "Interpolated curve") };
            let objects = polyline_objects(kernel, typology, label, &points, false, label_count, &next_id);
            (!objects.is_empty()).then_some(CommitOutcome::Objects(objects))
        }
        "mirror" => {
            // 🪞️ Mirror across the vertical plane through the two picked points: a copy whose origin is
            // reflected; the mirrored solid keeps its own extent (the persisted model has no handedness).
            let targets = context_targets(context);
            let a = context_named_point(context, "mirrorStart")?;
            let b = context_named_point(context, "mirrorEnd")?;
            let normal = [-(b[1] - a[1]), b[0] - a[0], 0.0];
            let length = (normal[0] * normal[0] + normal[1] * normal[1]).sqrt();
            if length <= 1e-9 || targets.is_empty() {
                return Some(CommitOutcome::Unsupported(format!("command.finish/{result_kind}")));
            }
            let unit = [normal[0] / length, normal[1] / length, 0.0];
            // The reflection of the selection's anchor is expressed as one copy offset — the persisted
            // origin moves twice its signed distance to the plane along the normal.
            let anchor = context.get("cursor").and_then(parse_vec3).unwrap_or(a);
            let signed = (anchor[0] - a[0]) * unit[0] + (anchor[1] - a[1]) * unit[1];
            Some(CommitOutcome::Copy { targets, delta: [-2.0 * signed * unit[0], -2.0 * signed * unit[1], 0.0] })
        }
        other => Some(CommitOutcome::Unsupported(format!("command.finish/{other}"))),
    }
}

/// 🎯️ Runs the spec's `commit.operation` against the session context — every commit family the
/// model-definition assets declare: `primitive.createBoxFromCorners`, the `*From2PointsAndHeight`/
/// `*FromSurface` constructions, `curve.line`/`curve.polyline`, `command.finish` result kinds, and
/// the `transform.*` verbs on the injected selection.
pub(crate) fn commit_session(kernel: &mut Brep, session: &CadEngagementScratch, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CommitOutcome> {
    let spec = spec_by_id(&session.interaction_id)?;
    let env = ExprEnv { context: &session.context.0, event: None };
    let empty_vars = HashMap::new();
    let params: HashMap<String, DslValue> = spec.commit.operation.params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, &env, &empty_vars))).collect();
    let action = spec.commit.operation.action.as_str();
    let label = spec.label.clone().unwrap_or_else(|| spec.id.clone());
    let typology = spec.produces.typology.clone().unwrap_or_else(|| format!("spatial.shape.{}", spec.id));
    let context = &session.context.0;
    if action == "primitive.createBoxFromCorners" {
        return commit_primitive_box(kernel, &params, label_count, next_id).map(|object| CommitOutcome::Objects(vec![object]));
    }
    if action.ends_with("From2PointsAndHeight") || action.ends_with("FromSurface") {
        return commit_from_2_points_and_height(kernel, &params, &label, label_count, next_id).map(|object| CommitOutcome::Objects(vec![object]));
    }
    if action == "command.finish" {
        return commit_command_finish(kernel, &params, context, label_count, next_id);
    }
    if action == "curve.line" {
        let p0 = params.get("p0").and_then(parse_vec3).or_else(|| context_named_point(context, "start"))?;
        let p1 = params.get("p1").and_then(parse_vec3).or_else(|| context_named_point(context, "end"))?;
        let lower = typology.to_lowercase();
        let thickness = if lower.contains("beam") { [0.3, 0.3] } else if lower.contains("railing") { [0.05, 1.0] } else { [0.02, 0.02] };
        return bar_object(kernel, &typology, format!("{label} {}", label_count + 1), p0, p1, thickness, next_id).map(|object| CommitOutcome::Objects(vec![object]));
    }
    if action == "curve.polyline" {
        let points = context_points_list(context);
        let closed = context.get("closed").and_then(|value| value.as_bool()).unwrap_or(false);
        let objects = polyline_objects(kernel, &typology, &label, &points, closed, label_count, &next_id);
        return (!objects.is_empty()).then_some(CommitOutcome::Objects(objects));
    }
    if action == "transform.move" || action == "transform.copy" {
        let targets = context_targets(context);
        let from = context_named_point(context, "from")?;
        let to = context_named_point(context, "to")?;
        let delta = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
        return Some(if action == "transform.copy" { CommitOutcome::Copy { targets, delta } } else { CommitOutcome::Move { targets, delta } });
    }
    if action == "transform.rotate" {
        let targets = context_targets(context);
        let center = context_named_point(context, "center")?;
        let angle = context.get("angle").and_then(|value| value.as_f64()).map(f64::to_radians).or_else(|| {
            let a = context_named_point(context, "referenceA")?;
            let b = context_named_point(context, "referenceB")?;
            Some((b[1] - center[1]).atan2(b[0] - center[0]) - (a[1] - center[1]).atan2(a[0] - center[0]))
        })?;
        return Some(CommitOutcome::Rotate { targets, axis: [0.0, 0.0, 1.0], angle });
    }
    if action == "transform.scale3d" || action == "transform.scale1d" {
        let targets = context_targets(context);
        let factor = context.get("factor").and_then(|value| value.as_f64()).or_else(|| {
            let origin = context_named_point(context, "origin")?;
            let a = context_named_point(context, "axisPoint").or_else(|| context_named_point(context, "referenceA"))?;
            let b = context_named_point(context, "referenceB")?;
            let base = distance(a, origin);
            (base > 1e-9).then(|| distance(b, origin) / base)
        })?;
        let factors = if action == "transform.scale1d" { [factor, 1.0, 1.0] } else { [factor; 3] };
        return Some(CommitOutcome::Scale { targets, factors });
    }
    Some(CommitOutcome::Unsupported(action.to_string()))
}

/// 🧱️ The first object a committed session produces — the shape the single-object tests read.
#[cfg(test)]
pub(crate) fn commit_object(kernel: &mut Brep, session: &CadEngagementScratch, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    match commit_session(kernel, session, label_count, next_id)? {
        CommitOutcome::Objects(objects) => objects.into_iter().next(),
        _ => None,
    }
}
//#endregion 🔖️CommitRunner

//#region 🔖️Preview
fn opt_string_value(value: &Option<String>) -> DslValue {
    value.clone().map_or(DslValue::Null, DslValue::String)
}

fn display_item_to_json(item: &DisplayItemSpec, env: &ExprEnv<'_>, vars: &HashMap<String, DslValue>) -> Option<DslValue> {
    match item {
        DisplayItemSpec::Point { id, role, position, .. } => {
            let position = evaluate_expr(position, env, vars);
            if position.is_null() {
                return None;
            }
            Some(DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("kind".to_string(), DslValue::String("point".into())), ("role".to_string(), opt_string_value(role)), ("position".to_string(), position)]))
        }
        DisplayItemSpec::Label { id, role, text, position, .. } => {
            let position = evaluate_expr(position, env, vars);
            Some(DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("kind".to_string(), DslValue::String("label".into())), ("role".to_string(), opt_string_value(role)), ("text".to_string(), DslValue::String(text.clone())), ("position".to_string(), position)]))
        }
        DisplayItemSpec::Segment { id, role, from, to, .. } => {
            let from = evaluate_expr(from, env, vars);
            let to = evaluate_expr(to, env, vars);
            if from.is_null() || to.is_null() {
                return None;
            }
            Some(DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("kind".to_string(), DslValue::String("segment".into())), ("role".to_string(), opt_string_value(role)), ("from".to_string(), from), ("to".to_string(), to)]))
        }
        DisplayItemSpec::LinearHandle { id, role, axis, origin, .. } => {
            let origin = evaluate_expr(origin, env, vars);
            if origin.is_null() {
                return None;
            }
            Some(DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("kind".to_string(), DslValue::String("linear-handle".into())), ("role".to_string(), opt_string_value(role)), ("axis".to_string(), vec3_json(*axis)), ("origin".to_string(), origin)]))
        }
        DisplayItemSpec::BoxPreview { id, role, corner_a, corner_b, height, .. } => {
            let corner_a = evaluate_expr(corner_a, env, vars);
            let corner_b = evaluate_expr(corner_b, env, vars);
            if corner_a.is_null() || corner_b.is_null() {
                return None;
            }
            let height = evaluate_expr(height, env, vars);
            Some(DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("kind".to_string(), DslValue::String("box-preview".into())), ("role".to_string(), opt_string_value(role)), ("cornerA".to_string(), corner_a), ("cornerB".to_string(), corner_b), ("height".to_string(), height)]))
        }
        DisplayItemSpec::EntityHighlight { id, role, geometry_entity_kind, entity_id, .. } => {
            let entity_id = evaluate_expr(entity_id, env, vars);
            if entity_id.is_null() {
                return None;
            }
            Some(DslValue::object([
                ("id".to_string(), DslValue::String(id.clone())),
                ("kind".to_string(), DslValue::String("entity-highlight".into())),
                ("role".to_string(), opt_string_value(role)),
                ("geometryEntityKind".to_string(), DslValue::String(geometry_entity_kind.clone())),
                ("entityId".to_string(), entity_id),
            ]))
        }
        DisplayItemSpec::Curve { id, role, .. } => Some(DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("kind".to_string(), DslValue::String("curve".into())), ("role".to_string(), opt_string_value(role))])),
        DisplayItemSpec::Mesh { id, role, .. } => Some(DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("kind".to_string(), DslValue::String("mesh".into())), ("role".to_string(), opt_string_value(role))])),
        DisplayItemSpec::Preview { id, role, preview_kind, params, .. } => {
            let evaluated_params: Vec<(String, DslValue)> = params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, env, vars))).collect();
            Some(DslValue::object([
                ("id".to_string(), DslValue::String(id.clone())),
                ("kind".to_string(), DslValue::String("preview".into())),
                ("role".to_string(), opt_string_value(role)),
                ("previewKind".to_string(), opt_string_value(preview_kind)),
                ("params".to_string(), DslValue::Object(evaluated_params)),
            ]))
        }
    }
}

pub fn preview_display_items(session: &CadEngagementScratch) -> Vec<DslValue> {
    let Some(spec) = spec_by_id(&session.interaction_id) else {
        return Vec::new();
    };
    let Some(display_state) = spec.display.states.iter().find(|state| state.state == session.state) else {
        return Vec::new();
    };
    let env = ExprEnv { context: &session.context.0, event: None };
    let empty_vars = HashMap::new();
    display_state.items.iter().filter_map(|item| display_item_to_json(item, &env, &empty_vars)).collect()
}
//#endregion 🔖️Preview

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
