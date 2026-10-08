//! 📸️ Persisted BIM model snapshot: authored parameters only. Nothing derivable is stored: a storey keeps its
//! height and level (never its elevation), a wall keeps its axis, type and constraints (never its height).

use schema::ArtifactSchema;
use std::collections::BTreeMap;

#[path = "💠️values/🦀️.rs"]
pub mod values;

#[path = "🧱️entities/🦀️.rs"]
pub mod entities;

pub use entities::*;
pub use values::*;

#[path = "✅️validity/🦀️.rs"]
pub mod validity;

pub use validity::*;

//#region 🔖️Snapshot
/// 📸️ Complete model document: every collection is keyed by a stable element id, in canonical (sorted) order.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[artifact_schema(id = "s.bim.model")]
#[dsl(extension = "bim")]
#[dsl(layout = "lines")]
pub struct ModelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[dsl(block)]
    pub project: Project,
    #[state(artifact)]
    #[value(default)]
    pub materials: BTreeMap<String, Material>,
    #[state(artifact)]
    #[value(default)]
    pub wall_types: BTreeMap<String, WallType>,
    #[state(artifact)]
    #[value(default)]
    pub slab_types: BTreeMap<String, SlabType>,
    #[state(artifact)]
    #[value(default)]
    pub roof_types: BTreeMap<String, RoofType>,
    #[state(artifact)]
    #[value(default)]
    pub column_types: BTreeMap<String, ColumnType>,
    #[state(artifact)]
    #[value(default)]
    pub beam_types: BTreeMap<String, BeamType>,
    #[state(artifact)]
    #[value(default)]
    pub window_types: BTreeMap<String, WindowType>,
    #[state(artifact)]
    #[value(default)]
    pub door_types: BTreeMap<String, DoorType>,
    #[state(artifact)]
    #[value(default)]
    pub sites: BTreeMap<String, Site>,
    #[state(artifact)]
    #[value(default)]
    pub buildings: BTreeMap<String, Building>,
    #[state(artifact)]
    #[value(default)]
    pub storeys: BTreeMap<String, Storey>,
    #[state(artifact)]
    #[value(default)]
    pub grids: BTreeMap<String, GridLine>,
    #[state(artifact)]
    #[value(default)]
    pub walls: BTreeMap<String, Wall>,
    #[state(artifact)]
    #[value(default)]
    pub curtain_walls: BTreeMap<String, CurtainWall>,
    #[state(artifact)]
    #[value(default)]
    pub columns: BTreeMap<String, Column>,
    #[state(artifact)]
    #[value(default)]
    pub beams: BTreeMap<String, Beam>,
    #[state(artifact)]
    #[value(default)]
    pub slabs: BTreeMap<String, Slab>,
    #[state(artifact)]
    #[value(default)]
    pub roofs: BTreeMap<String, Roof>,
    #[state(artifact)]
    #[value(default)]
    pub openings: BTreeMap<String, Opening>,
    #[state(artifact)]
    #[value(default)]
    pub stairs: BTreeMap<String, Stair>,
    #[state(artifact)]
    #[value(default)]
    pub railings: BTreeMap<String, Railing>,
    #[state(artifact)]
    #[value(default)]
    pub spaces: BTreeMap<String, Space>,
    #[state(artifact)]
    #[value(default)]
    pub properties: BTreeMap<String, PropertySet>,
    #[state(artifact)]
    #[value(default)]
    pub classifications: BTreeMap<String, Classification>,
}

impl Default for ModelSnapshot {
    fn default() -> Self {
        Self {
            schema: crate::BIM_MODEL_DOCUMENT_SCHEMA.into(),
            project: Project { name: String::new(), description: String::new(), author: String::new(), organization: String::new(), phase_names: Vec::new() },
            materials: BTreeMap::new(),
            wall_types: BTreeMap::new(),
            slab_types: BTreeMap::new(),
            roof_types: BTreeMap::new(),
            column_types: BTreeMap::new(),
            beam_types: BTreeMap::new(),
            window_types: BTreeMap::new(),
            door_types: BTreeMap::new(),
            sites: BTreeMap::new(),
            buildings: BTreeMap::new(),
            storeys: BTreeMap::new(),
            grids: BTreeMap::new(),
            walls: BTreeMap::new(),
            curtain_walls: BTreeMap::new(),
            columns: BTreeMap::new(),
            beams: BTreeMap::new(),
            slabs: BTreeMap::new(),
            roofs: BTreeMap::new(),
            openings: BTreeMap::new(),
            stairs: BTreeMap::new(),
            railings: BTreeMap::new(),
            spaces: BTreeMap::new(),
            properties: BTreeMap::new(),
            classifications: BTreeMap::new(),
        }
    }
}
//#endregion 🔖️Snapshot
