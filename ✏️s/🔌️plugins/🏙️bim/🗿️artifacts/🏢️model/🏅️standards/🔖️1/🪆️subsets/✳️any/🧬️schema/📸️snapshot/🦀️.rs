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

#[path = "🕰️phases/🦀️.rs"]
pub mod phases;

#[path = "📋️schedule-kit/🦀️.rs"]
pub mod schedule_kit;

#[path = "⚓️anchors/🦀️.rs"]
pub mod anchors;

pub use anchors::*;

#[path = "🏷️property-kit/🦀️.rs"]
pub mod property_kit;

pub use property_kit::*;

#[path = "🖼️views/🦀️.rs"]
pub mod views;

pub use views::*;

#[path = "📄️sheets/🦀️.rs"]
pub mod sheets;

pub use sheets::*;

#[path = "🔥️conditions/🦀️.rs"]
pub mod conditions;

pub use conditions::*;

#[path = "🤝️coordination/🦀️.rs"]
pub mod coordination;

pub use coordination::*;

#[path = "🦴️structure/🦀️.rs"]
pub mod structure;
pub use structure::*;

//#region 🔖️Snapshot
/// 📸️ Complete model document: every collection is keyed by a stable element id, in canonical (sorted) order.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[artifact_schema(id = "s.bim.model")]
#[dsl(extension = "bim")]
#[dsl(layout = "lines")]
pub struct ModelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub supports: BTreeMap<String, StructuralSupport>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub load_cases: BTreeMap<String, LoadCase>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub loads: BTreeMap<String, StructuralLoad>,
    #[state(artifact)]
    #[dsl(block)]
    pub project: Project,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub option_groups: BTreeMap<String, OptionGroup>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub design_options: BTreeMap<String, DesignOption>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub worksets: BTreeMap<String, Workset>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub element_options: BTreeMap<String, ElementMembership>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub element_worksets: BTreeMap<String, ElementMembership>,
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
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub curtain_wall_types: BTreeMap<String, CurtainWallType>,
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
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub curtain_panel_overrides: BTreeMap<String, CurtainPanelOverride>,
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
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ramps: BTreeMap<String, Ramp>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ceiling_types: BTreeMap<String, CeilingType>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ceilings: BTreeMap<String, Ceiling>,
    #[state(artifact)]
    #[value(default)]
    pub spaces: BTreeMap<String, Space>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub zones: BTreeMap<String, Zone>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub area_schemes: BTreeMap<String, AreaScheme>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub space_conditions: BTreeMap<String, SpaceConditions>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub views: BTreeMap<String, View>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sheets: BTreeMap<String, Sheet>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub viewports: BTreeMap<String, Viewport>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sheet_revisions: BTreeMap<String, SheetRevision>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub clash_sets: BTreeMap<String, ClashSet>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub rules: BTreeMap<String, Rule>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub issues: BTreeMap<String, Issue>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub issue_comments: BTreeMap<String, IssueComment>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub dimensions: BTreeMap<String, Dimension>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub tags: BTreeMap<String, Tag>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub text_notes: BTreeMap<String, TextNote>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub leaders: BTreeMap<String, Leader>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotation_styles: BTreeMap<String, AnnotationStyle>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub wall_sweeps: BTreeMap<String, WallSweep>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub schedules: BTreeMap<String, Schedule>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub families: BTreeMap<String, Family>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub family_parameters: BTreeMap<String, FamilyParameter>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub family_solids: BTreeMap<String, FamilySolid>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub components: BTreeMap<String, Component>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub component_overrides: BTreeMap<String, ComponentOverride>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub mep_elements: BTreeMap<String, MepElement>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub property_templates: BTreeMap<String, PropertyTemplate>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub classification_systems: BTreeMap<String, ClassificationSystem>,
    #[state(artifact)]
    #[value(default)]
    pub properties: BTreeMap<String, PropertySet>,
    #[state(artifact)]
    #[value(default)]
    pub classifications: BTreeMap<String, ClassificationSet>,
}

impl Default for ModelSnapshot {
    fn default() -> Self {
        Self {
            schema: crate::BIM_MODEL_DOCUMENT_SCHEMA.into(),
            supports: BTreeMap::new(),
            load_cases: BTreeMap::new(),
            loads: BTreeMap::new(),
            project: Project { name: String::new(), description: String::new(), author: String::new(), organization: String::new(), phase_names: Vec::new() },
            option_groups: BTreeMap::new(),
            design_options: BTreeMap::new(),
            worksets: BTreeMap::new(),
            element_options: BTreeMap::new(),
            element_worksets: BTreeMap::new(),
            materials: BTreeMap::new(),
            wall_types: BTreeMap::new(),
            slab_types: BTreeMap::new(),
            roof_types: BTreeMap::new(),
            column_types: BTreeMap::new(),
            beam_types: BTreeMap::new(),
            window_types: BTreeMap::new(),
            door_types: BTreeMap::new(),
            curtain_wall_types: BTreeMap::new(),
            sites: BTreeMap::new(),
            buildings: BTreeMap::new(),
            storeys: BTreeMap::new(),
            grids: BTreeMap::new(),
            walls: BTreeMap::new(),
            curtain_walls: BTreeMap::new(),
            curtain_panel_overrides: BTreeMap::new(),
            columns: BTreeMap::new(),
            beams: BTreeMap::new(),
            slabs: BTreeMap::new(),
            roofs: BTreeMap::new(),
            openings: BTreeMap::new(),
            stairs: BTreeMap::new(),
            railings: BTreeMap::new(),
            ramps: BTreeMap::new(),
            ceiling_types: BTreeMap::new(),
            ceilings: BTreeMap::new(),
            spaces: BTreeMap::new(),
            zones: BTreeMap::new(),
            area_schemes: BTreeMap::new(),
            space_conditions: BTreeMap::new(),
            views: BTreeMap::new(),
            sheets: BTreeMap::new(),
            viewports: BTreeMap::new(),
            sheet_revisions: BTreeMap::new(),
            clash_sets: BTreeMap::new(),
            rules: BTreeMap::new(),
            issues: BTreeMap::new(),
            issue_comments: BTreeMap::new(),
            dimensions: BTreeMap::new(),
            tags: BTreeMap::new(),
            text_notes: BTreeMap::new(),
            leaders: BTreeMap::new(),
            annotation_styles: BTreeMap::new(),
            wall_sweeps: BTreeMap::new(),
            schedules: BTreeMap::new(),
            families: BTreeMap::new(),
            family_parameters: BTreeMap::new(),
            family_solids: BTreeMap::new(),
            components: BTreeMap::new(),
            component_overrides: BTreeMap::new(),
            mep_elements: BTreeMap::new(),
            property_templates: BTreeMap::new(),
            classification_systems: BTreeMap::new(),
            properties: BTreeMap::new(),
            classifications: BTreeMap::new(),
        }
    }
}
//#endregion 🔖️Snapshot
