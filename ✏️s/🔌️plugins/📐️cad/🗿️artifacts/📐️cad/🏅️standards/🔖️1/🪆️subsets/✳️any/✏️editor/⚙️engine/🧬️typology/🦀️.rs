//! 🧬️ CAD typology + model-definition registry — the Rust twin of `⚙️engine/🧬️typology/🟦️.ts` and of
//! the typology/model-definition/attribute half of the spatial kernel's
//! `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts`.
//!
//! 🧭️ Schema-first: every row is parsed from the SAME shipped JSON asset the TS side globs
//! (`📚️examples/🖼️assets/🏗️modelDefinitions/*/{🔣️modelDefinition,🗂️typologies/*/🔣️typology,🏷️attributeDefinitions/*}.json`),
//! embedded with `include_str!` exactly as `⚙️engine/🕹️interaction/🦀️.rs` embeds the interaction
//! specs — there is no glob/registrar step on this target, so `📔️registry`/`🏃️runtime` have no twin
//! (see the packet report `📓️w2f-cad-spatial-editor-wgpu.md` §2).
//!
//! 🎨️ `resolve_typology_style` reproduces the TS auto-style derivation bit for bit (FNV-1a over the
//! typology id → golden-angle hue → HSL→hex), so a typology with no authored `style` block paints the
//! same colour on every target.

use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::HashMap;
use std::sync::OnceLock;

//#region 🔖️Kinds
/// 🧭️ Framework + kernel sub-element selection kinds (`GeometryEntityKind | object | geometry | attribute`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModelEntityKind {
    Anchor,
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    Object,
    Geometry,
    Attribute,
}

impl ModelEntityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Anchor => "anchor",
            Self::Vertex => "vertex",
            Self::Edge => "edge",
            Self::Wire => "wire",
            Self::Face => "face",
            Self::Shell => "shell",
            Self::Solid => "solid",
            Self::Object => "object",
            Self::Geometry => "geometry",
            Self::Attribute => "attribute",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "anchor" => Self::Anchor,
            "vertex" => Self::Vertex,
            "edge" => Self::Edge,
            "wire" => Self::Wire,
            "face" => Self::Face,
            "shell" => Self::Shell,
            "solid" => Self::Solid,
            "object" => Self::Object,
            "geometry" => Self::Geometry,
            "attribute" => Self::Attribute,
            _ => return None,
        })
    }
}

/// 🧱️ Primitive entity kinds selectable on kernel geometry (excludes typology `object` rows).
pub const PRIMITIVE_MODEL_ENTITY_KINDS: &[ModelEntityKind] = &[
    ModelEntityKind::Anchor,
    ModelEntityKind::Vertex,
    ModelEntityKind::Edge,
    ModelEntityKind::Wire,
    ModelEntityKind::Face,
    ModelEntityKind::Shell,
    ModelEntityKind::Solid,
];

/// 🧱️ The three primitive slots a typology may bind its object row to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypologyPrimitiveKind {
    Solid,
    Surface,
    Curve,
}

impl TypologyPrimitiveKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Surface => "surface",
            Self::Curve => "curve",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "solid" => Self::Solid,
            "surface" => Self::Surface,
            "curve" => Self::Curve,
            _ => return None,
        })
    }
}
//#endregion 🔖️Kinds

//#region 🔖️Assets
/// 🎨️ Authored pattern block on a typology `style` (every field optional — the auto style fills in).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TypologyPatternSpec {
    #[value(default)]
    pub kind: Option<String>,
    #[value(default)]
    pub direction: Option<f64>,
    #[value(default)]
    pub spacing: Option<f64>,
    #[value(default)]
    pub line_width: Option<f64>,
    #[value(default)]
    pub color: Option<String>,
}

/// 🎨️ Authored `style` block on a typology asset.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TypologyStyleSpec {
    #[value(default)]
    pub color: Option<String>,
    #[value(default)]
    pub edge_color: Option<String>,
    #[value(default)]
    pub opacity: Option<f64>,
    #[value(default)]
    pub pattern: Option<TypologyPatternSpec>,
}

/// 🗂️ `spatial.typology` asset row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TypologySpec {
    pub id: String,
    pub version: String,
    pub label: String,
    #[value(default)]
    pub description: Option<String>,
    #[value(default)]
    pub primitive_kinds: Vec<String>,
    #[value(default)]
    pub style: Option<TypologyStyleSpec>,
    #[value(default)]
    pub actions: Vec<String>,
    #[value(default)]
    pub interactions: Vec<String>,
}

impl TypologySpec {
    pub fn primitive_kind_set(&self) -> Vec<TypologyPrimitiveKind> {
        self.primitive_kinds.iter().filter_map(|raw| TypologyPrimitiveKind::parse(raw)).collect()
    }
}

/// 🗺️ `spatial.modelDefinition` asset row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ModelDefinitionManifest {
    pub id: String,
    pub version: String,
    #[value(default)]
    pub label: Option<String>,
    #[value(default)]
    pub description: Option<String>,
    #[value(default)]
    pub default: bool,
    #[value(default)]
    pub base_object_typology: Option<String>,
    #[value(default)]
    pub kernel_typologies: HashMap<String, String>,
    #[value(default)]
    pub kinds: Vec<String>,
}

/// 🧲️ `geometrySelector` block on an attribute definition.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AttributeGeometrySelector {
    #[value(default)]
    pub kinds: Vec<String>,
}

/// 🏷️ `spatial.attribute` asset row — only the fields the selection-kind derivation reads.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AttributeDefinitionSpec {
    pub id: String,
    pub version: String,
    #[value(default)]
    pub label: Option<String>,
    #[value(default)]
    pub field: Option<String>,
    #[value(default)]
    pub targets: Vec<String>,
    #[value(default)]
    pub geometry_selector: Option<AttributeGeometrySelector>,
}

const RAW_MODEL_DEFINITION_ASSETS: &[&str] = &[
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📏️aec.building.structure.fem.line/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🗺️aec.building.structure.fem.surface/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🧊️aec.building.structure.fem.solid/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🧱️aec.building.concrete/🔣️modelDefinition.json"),
];

const RAW_TYPOLOGY_ASSETS: &[(&str, &str)] = &[
    ("aec.building.structure.classic", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🗂️typologies/🏛️ReinforcedConcreteColumn/🔣️typology-20ac89.json")),
    ("aec.building.structure.classic", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🗂️typologies/🚧️ReinforcedConcreteInternalWall/🔣️typology-87a300.json")),
    ("aec.building.structure.classic", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🗂️typologies/🛡️ReinforcedConcreteExternalWall/🔣️typology-9f56a1.json")),
    ("aec.building.structure.classic", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building.structure.classic/🗂️typologies/🧱️OneWayReinforcedConcreteSlab/🔣️typology-6a95a6.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/⬆️Ceiling/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🏛️Column/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🏠️Roof/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🚧️Railing/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🚪️Door/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🛡️Wall/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🧱️Slab/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🪜️Stair/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🪟️Window/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🪨️Foundation/🔣️typology.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🪵️Beam/🔣️typology.json")),
    ("aec.building.structure.fem.line", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📏️aec.building.structure.fem.line/🗂️typologies/📏️LineElement/🔣️typology-3ad563.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/➡️ExtrudeCurve/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/⭕️Circle/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/〰️ControlPointCurve/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌀️Sweep1/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌊️InterpolateCurve/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌐️Sphere/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌙️Arc/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌪️Sweep2/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🎢️Loft/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/📏️Line/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/📐️Polyline/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/📦️Box/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🕸️NetworkSurface/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🗺️Plane/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🥫️Cylinder/🔣️typology.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🏠️Roof/🔣️typology.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🐚️Hull/🔣️typology.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🧱️ExternalWall/🔣️typology.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🪟️Windows/🔣️typology.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🪨️BasePlate/🔣️typology.json")),
    ("aec.building.structure.fem.surface", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🗺️aec.building.structure.fem.surface/🗂️typologies/🗺️SurfaceElement/🔣️typology-b35a36.json")),
    ("aec.building.structure.fem.solid", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🧊️aec.building.structure.fem.solid/🗂️typologies/🧊️SolidElement/🔣️typology-4e128f.json")),
];

const RAW_ATTRIBUTE_DEFINITION_ASSETS: &[(&str, &str)] = &[
    ("aec.building.structure", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🌡️uvalue.json")),
    ("aec.building.structure", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🌦️exposure.json")),
    ("aec.building.structure", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🔆️gvalue.json")),
    ("aec.building.structure", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🔗️bondable.json")),
    ("aec.building.structure", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🚪️opening.json")),
    ("aec.building.structure", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🪨️material.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🏷️attributeDefinitions/🚪️opening.json")),
    ("aec.building", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🏷️attributeDefinitions/🪨️material.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🏷️attributeDefinitions/🌡️uvalue.json")),
    ("aec.building.energy", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🏷️attributeDefinitions/🔆️gvalue.json")),
];

fn parsed_model_definitions() -> &'static [ModelDefinitionManifest] {
    static ROWS: OnceLock<Vec<ModelDefinitionManifest>> = OnceLock::new();
    ROWS.get_or_init(|| RAW_MODEL_DEFINITION_ASSETS.iter().filter_map(|raw| protocol::json::from_json_str::<ModelDefinitionManifest>(raw).ok()).collect())
}

fn parsed_typologies() -> &'static [(&'static str, TypologySpec)] {
    static ROWS: OnceLock<Vec<(&'static str, TypologySpec)>> = OnceLock::new();
    ROWS.get_or_init(|| RAW_TYPOLOGY_ASSETS.iter().filter_map(|(owner, raw)| protocol::json::from_json_str::<TypologySpec>(raw).ok().map(|spec| (*owner, spec))).collect())
}

fn parsed_attribute_definitions() -> &'static [(&'static str, AttributeDefinitionSpec)] {
    static ROWS: OnceLock<Vec<(&'static str, AttributeDefinitionSpec)>> = OnceLock::new();
    ROWS.get_or_init(|| RAW_ATTRIBUTE_DEFINITION_ASSETS.iter().filter_map(|(owner, raw)| protocol::json::from_json_str::<AttributeDefinitionSpec>(raw).ok().map(|spec| (*owner, spec))).collect())
}
//#endregion 🔖️Assets

//#region 🔖️Catalog
/// 📚️ Every shipped model-definition manifest.
pub fn list_model_definition_manifests() -> &'static [ModelDefinitionManifest] {
    parsed_model_definitions()
}

/// 🧭️ Default geometry-edit model definition id (manifest `default: true`, else the first row).
pub fn default_model_definition_id() -> &'static str {
    static ID: OnceLock<String> = OnceLock::new();
    ID.get_or_init(|| {
        let rows = parsed_model_definitions();
        rows.iter().find(|row| row.default).or_else(|| rows.first()).map(|row| row.id.clone()).unwrap_or_default()
    })
}

/// 🪪️ Kernel typology ids per primitive kind on a manifest, or `None` when the manifest declares none.
pub fn kernel_typology_ids(model_definition_id: &str) -> Option<&'static HashMap<String, String>> {
    let manifest = parsed_model_definitions().iter().find(|row| row.id == model_definition_id)?;
    if manifest.kernel_typologies.is_empty() {
        return None;
    }
    Some(&manifest.kernel_typologies)
}

/// 🧭️ True when the definition is geometry edit (`ModelDefinition`) rather than typology objects.
pub fn is_shape_model_definition(model_definition_id: Option<&str>) -> bool {
    match model_definition_id {
        None => true,
        Some(id) => kernel_typology_ids(id).is_some(),
    }
}

/// 🧲️ True when the definition exposes kernel-geometry pick targets — every definition does.
pub fn model_definition_uses_geometry_picking(_model_definition_id: &str) -> bool {
    true
}

/// 📚️ Every shipped typology row.
pub fn list_model_definition_typologies() -> impl Iterator<Item = &'static TypologySpec> {
    parsed_typologies().iter().map(|(_, spec)| spec)
}

/// 📚️ Loads a typology by stable `id`.
pub fn load_typology(typology: &str) -> Option<&'static TypologySpec> {
    parsed_typologies().iter().find(|(_, spec)| spec.id == typology).map(|(_, spec)| spec)
}

/// 🧭️ Model definition that owns a typology asset.
pub fn model_definition_id_for_typology(typology_id: &str) -> Option<&'static str> {
    parsed_typologies().iter().find(|(_, spec)| spec.id == typology_id).map(|(owner, _)| *owner)
}

/// 📚️ Typologies whose asset file lives under `model_definition_id`.
pub fn list_typologies_for_model_definition(model_definition_id: &str) -> Vec<&'static TypologySpec> {
    parsed_typologies().iter().filter(|(owner, _)| *owner == model_definition_id).map(|(_, spec)| spec).collect()
}

/// 📚️ Resolves the typology whose `interactions` list includes `interaction_id`.
pub fn typology_for_interaction(interaction_id: &str) -> Option<&'static TypologySpec> {
    parsed_typologies().iter().find(|(_, spec)| spec.interactions.iter().any(|id| id == interaction_id)).map(|(_, spec)| spec)
}

/// 📚️ Attribute definitions declared under one model definition.
pub fn list_attribute_definitions_for_model_definition(model_definition_id: &str) -> Vec<&'static AttributeDefinitionSpec> {
    let mut rows: Vec<&'static AttributeDefinitionSpec> = parsed_attribute_definitions().iter().filter(|(owner, _)| *owner == model_definition_id).map(|(_, spec)| spec).collect();
    rows.sort_by(|a, b| a.id.cmp(&b.id));
    rows
}

/// 🧭️ Selection entity kinds available while a model definition is active (primitives + objects +
/// anything an attribute definition targets), ordered primitives-first exactly as the TS twin.
pub fn model_definition_selection_entity_kinds(model_definition_id: &str) -> Vec<ModelEntityKind> {
    let mut kinds: Vec<ModelEntityKind> = PRIMITIVE_MODEL_ENTITY_KINDS.to_vec();
    kinds.push(ModelEntityKind::Object);
    for defn in list_attribute_definitions_for_model_definition(model_definition_id) {
        let selector: &[String] = defn.geometry_selector.as_ref().map_or(&[], |selector| selector.kinds.as_slice());
        for raw in defn.targets.iter().chain(selector.iter()) {
            if let Some(kind) = ModelEntityKind::parse(raw) {
                if !kinds.contains(&kind) {
                    kinds.push(kind);
                }
            }
        }
    }
    let mut ordered: Vec<ModelEntityKind> = PRIMITIVE_MODEL_ENTITY_KINDS.iter().copied().filter(|kind| kinds.contains(kind)).collect();
    for kind in kinds {
        if !PRIMITIVE_MODEL_ENTITY_KINDS.contains(&kind) && !ordered.contains(&kind) {
            ordered.push(kind);
        }
    }
    ordered
}

/// 👁️ Typology ids declared on the active model definition (sorted, TS `localeCompare` on ASCII ids).
pub fn model_definition_typology_ids(model_definition_id: Option<&str>) -> Vec<String> {
    let fallback: &str = default_model_definition_id();
    let mut ids: Vec<String> = list_typologies_for_model_definition(model_definition_id.unwrap_or(fallback)).into_iter().map(|row| row.id.clone()).collect();
    ids.sort();
    ids
}
//#endregion 🔖️Catalog

//#region 🔖️Style
/// 🎨️ Pattern half of a resolved typology style.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedTypologyPattern {
    pub kind: String,
    pub direction: f64,
    pub spacing: f64,
    pub line_width: f64,
    pub color: String,
}

/// 🎨️ A typology's display style with every optional authored field filled from the auto style.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedTypologyStyle {
    pub color: String,
    pub edge_color: String,
    pub opacity: f64,
    pub pattern: ResolvedTypologyPattern,
}

fn hash_typology_id(typology: &str) -> u32 {
    let mut h: u32 = 2_166_136_261;
    for unit in typology.encode_utf16() {
        h ^= u32::from(unit);
        h = h.wrapping_mul(16_777_619);
    }
    h
}

fn hsl_to_hex(h: f64, s: f64, l: f64) -> String {
    let hue = ((h % 360.0) + 360.0) % 360.0;
    let sat = s.clamp(0.0, 1.0);
    let lit = l.clamp(0.0, 1.0);
    let c = (1.0 - (2.0 * lit - 1.0).abs()) * sat;
    let x = c * (1.0 - (((hue / 60.0) % 2.0) - 1.0).abs());
    let m = lit - c / 2.0;
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    format!("#{:02x}{:02x}{:02x}", js_round((r + m) * 255.0), js_round((g + m) * 255.0), js_round((b + m) * 255.0))
}

/// 🔢️ `Math.round` semantics (half away from zero upward), not Rust's half-away-from-zero — the two
/// agree for the non-negative channel values here, and the explicit form keeps the port auditable.
fn js_round(v: f64) -> u32 {
    (v + 0.5).floor().max(0.0).min(255.0) as u32
}

fn darken_hex_color(hex: &str, amount: f64) -> String {
    let raw = hex.trim_start_matches('#');
    if raw.len() != 6 {
        return hex.to_string();
    }
    let channel = |from: usize| u32::from_str_radix(&raw[from..from + 2], 16).ok();
    let (Some(r), Some(g), Some(b)) = (channel(0), channel(2), channel(4)) else {
        return hex.to_string();
    };
    let scale = 1.0 - amount;
    let byte = |v: u32| js_round(f64::from(v) * scale);
    format!("#{:02x}{:02x}{:02x}", byte(r), byte(g), byte(b))
}

fn auto_typology_style(typology: &str) -> ResolvedTypologyStyle {
    let hue = (f64::from(hash_typology_id(typology)) * 137.508) % 360.0;
    let color = hsl_to_hex(hue, 0.58, 0.52);
    ResolvedTypologyStyle {
        edge_color: darken_hex_color(&color, 0.32),
        opacity: 0.72,
        pattern: ResolvedTypologyPattern { kind: "none".into(), direction: 0.0, spacing: 0.35, line_width: 0.03, color: darken_hex_color(&color, 0.18) },
        color,
    }
}

fn merge_typology_style(typology: &str, authored: Option<&TypologyStyleSpec>) -> ResolvedTypologyStyle {
    let base = auto_typology_style(typology);
    let pattern = authored.and_then(|style| style.pattern.as_ref());
    let fill = authored.and_then(|style| style.color.clone()).unwrap_or_else(|| base.color.clone());
    ResolvedTypologyStyle {
        edge_color: authored.and_then(|style| style.edge_color.clone()).unwrap_or_else(|| darken_hex_color(&fill, 0.32)),
        opacity: authored.and_then(|style| style.opacity).unwrap_or(base.opacity),
        pattern: ResolvedTypologyPattern {
            kind: pattern.and_then(|p| p.kind.clone()).unwrap_or(base.pattern.kind),
            direction: pattern.and_then(|p| p.direction).unwrap_or(base.pattern.direction),
            spacing: pattern.and_then(|p| p.spacing).unwrap_or(base.pattern.spacing),
            line_width: pattern.and_then(|p| p.line_width).unwrap_or(base.pattern.line_width),
            color: pattern.and_then(|p| p.color.clone()).unwrap_or_else(|| darken_hex_color(&fill, 0.18)),
        },
        color: fill,
    }
}

/// 🎨️ Resolves the display style for a typology (deterministic auto fallback + authored override).
pub fn resolve_typology_style(typology: &str) -> ResolvedTypologyStyle {
    merge_typology_style(typology, load_typology(typology).and_then(|spec| spec.style.as_ref()))
}

/// 🎨️ Stable cache key for renderer material/pattern reuse — the TS field order, byte for byte.
pub fn typology_style_cache_key(style: &ResolvedTypologyStyle) -> String {
    let p = &style.pattern;
    format!("{}|{}|{}|{}|{}|{}|{}|{}", style.color, style.edge_color, js_number(style.opacity), p.kind, js_number(p.direction), js_number(p.spacing), js_number(p.line_width), p.color)
}

/// 🔢️ `String(number)` for the finite values a style carries: whole floats lose their `.0`.
pub fn js_number(v: f64) -> String {
    if v == v.trunc() && v.abs() < 1e21 {
        format!("{}", v as i64)
    } else {
        let mut text = format!("{v}");
        if text.contains('e') {
            text = format!("{v:?}");
        }
        text
    }
}
//#endregion 🔖️Style

//#region 🔖️Labels
/// 🏷️ PascalCase object name from a typology label (`External Wall` → `ExternalWall`).
pub fn typology_object_pascal_from_label(label: &str) -> String {
    label
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect()
}

/// 🏷️ Short typology label for play chrome (`Base Plate` → `BasePlate`).
pub fn spatial_typology_toggle_label(typology_id: &str, label: Option<&str>) -> String {
    if let Some(label) = label.filter(|text| !text.trim().is_empty()) {
        return typology_object_pascal_from_label(label);
    }
    let tail = typology_id.rsplit('.').next().unwrap_or(typology_id);
    let spaced: String = tail.chars().map(|c| if matches!(c, '.' | '_' | '-') { ' ' } else { c }).collect();
    typology_object_pascal_from_label(&spaced)
}
//#endregion 🔖️Labels

//#region 🔖️Construct
/// 🧭️ The three `construct*From*` mode actions and the one `construct*` interaction of a typology.
#[derive(Clone, Debug, PartialEq)]
pub struct TypologyConstructKit {
    pub typology: String,
    pub interaction: String,
    pub construct_from_2_points_and_height: String,
    pub construct_from_curve_and_height: String,
    pub construct_from_surface: String,
}

/// 🧭️ Stable asset ids derived from a typology id + label.
pub fn typology_construct_asset_ids(typology: &str, label: &str) -> TypologyConstructKit {
    let parts: Vec<&str> = typology.split('.').collect();
    let prefix = if parts.len() > 1 { format!("{}.", parts[..parts.len() - 1].join(".")) } else { String::new() };
    let pascal = typology_object_pascal_from_label(label);
    TypologyConstructKit {
        typology: typology.to_string(),
        interaction: format!("{prefix}construct{pascal}"),
        construct_from_2_points_and_height: format!("{prefix}construct{pascal}From2PointsAndHeight"),
        construct_from_curve_and_height: format!("{prefix}construct{pascal}FromCurveAndHeight"),
        construct_from_surface: format!("{prefix}construct{pascal}FromSurface"),
    }
}

fn typology_construct_is_surface_primary(typology_id: &str) -> bool {
    typology_id.ends_with(".baseplate")
}

/// 🧭️ `construct*` action ids declared on a typology (`surface`-primary typologies ship surface only).
pub fn typology_construct_mode_action_ids(typology_id: &str, label: &str) -> Vec<String> {
    let ids = typology_construct_asset_ids(typology_id, label);
    if typology_construct_is_surface_primary(typology_id) {
        return vec![ids.construct_from_surface];
    }
    vec![ids.construct_from_2_points_and_height, ids.construct_from_curve_and_height, ids.construct_from_surface]
}

/// 🏗️ True when a typology ships exactly one construct interaction and its mode `construct*` actions.
pub fn typology_has_native_construct_kit(typology: &TypologySpec) -> bool {
    let ids = typology_construct_asset_ids(&typology.id, &typology.label);
    let mut expected = typology_construct_mode_action_ids(&typology.id, &typology.label);
    expected.sort();
    let mut actual = typology.actions.clone();
    actual.sort();
    typology.interactions.len() == 1 && typology.interactions[0] == ids.interaction && actual == expected
}

/// 🏗️ Typologies in a model definition that expose the native construct interaction.
pub fn list_constructable_typologies_for_model_definition(model_definition_id: &str) -> Vec<&'static TypologySpec> {
    list_typologies_for_model_definition(model_definition_id).into_iter().filter(|spec| typology_has_native_construct_kit(spec)).collect()
}

/// 🧭️ Maps each typology construct interaction id to its mode actions (not the interaction id).
pub fn typology_construct_kit_by_interaction(interaction_id: &str) -> Option<TypologyConstructKit> {
    list_model_definition_typologies().map(|spec| typology_construct_asset_ids(&spec.id, &spec.label)).find(|kit| kit.interaction == interaction_id)
}
//#endregion 🔖️Construct

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
