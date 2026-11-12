//! 📸️ Persisted BIM model snapshot: authored parameters only. Nothing derivable is stored: a storey keeps its
//! height and level (never its elevation), a wall keeps its axis, type and constraints (never its height).

use schema::ArtifactSchema;
use semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap;


#[path = "🧱️entities/🦀️.rs"]
pub mod entities;

pub use entities::*;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Project, PropertySet, ClassificationSet};

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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone)]
#[artifact_schema(id = "s.bim.model")]
#[dsl(extension = "bim")]
#[dsl(layout = "lines")]
pub struct ModelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub supports: RetainedOrderedMap<String, StructuralSupport>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub load_cases: RetainedOrderedMap<String, LoadCase>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub loads: RetainedOrderedMap<String, StructuralLoad>,
    #[state(artifact)]
    #[dsl(block)]
    pub project: Project,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub option_groups: RetainedOrderedMap<String, OptionGroup>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub design_options: RetainedOrderedMap<String, DesignOption>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub worksets: RetainedOrderedMap<String, Workset>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub element_options: RetainedOrderedMap<String, ElementMembership>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub element_worksets: RetainedOrderedMap<String, ElementMembership>,
    #[state(artifact)]
    #[value(default)]
    pub materials: RetainedOrderedMap<String, Material>,
    #[state(artifact)]
    #[value(default)]
    pub wall_types: RetainedOrderedMap<String, WallType>,
    #[state(artifact)]
    #[value(default)]
    pub slab_types: RetainedOrderedMap<String, SlabType>,
    #[state(artifact)]
    #[value(default)]
    pub roof_types: RetainedOrderedMap<String, RoofType>,
    #[state(artifact)]
    #[value(default)]
    pub column_types: RetainedOrderedMap<String, ColumnType>,
    #[state(artifact)]
    #[value(default)]
    pub beam_types: RetainedOrderedMap<String, BeamType>,
    #[state(artifact)]
    #[value(default)]
    pub window_types: RetainedOrderedMap<String, WindowType>,
    #[state(artifact)]
    #[value(default)]
    pub door_types: RetainedOrderedMap<String, DoorType>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub curtain_wall_types: RetainedOrderedMap<String, CurtainWallType>,
    #[state(artifact)]
    #[value(default)]
    pub sites: RetainedOrderedMap<String, Site>,
    #[state(artifact)]
    #[value(default)]
    pub buildings: RetainedOrderedMap<String, Building>,
    #[state(artifact)]
    #[value(default)]
    pub storeys: RetainedOrderedMap<String, Storey>,
    #[state(artifact)]
    #[value(default)]
    pub grids: RetainedOrderedMap<String, GridLine>,
    #[state(artifact)]
    #[value(default)]
    pub walls: RetainedOrderedMap<String, Wall>,
    #[state(artifact)]
    #[value(default)]
    pub curtain_walls: RetainedOrderedMap<String, CurtainWall>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub curtain_panel_overrides: RetainedOrderedMap<String, CurtainPanelOverride>,
    #[state(artifact)]
    #[value(default)]
    pub columns: RetainedOrderedMap<String, Column>,
    #[state(artifact)]
    #[value(default)]
    pub beams: RetainedOrderedMap<String, Beam>,
    #[state(artifact)]
    #[value(default)]
    pub slabs: RetainedOrderedMap<String, Slab>,
    #[state(artifact)]
    #[value(default)]
    pub roofs: RetainedOrderedMap<String, Roof>,
    #[state(artifact)]
    #[value(default)]
    pub openings: RetainedOrderedMap<String, Opening>,
    #[state(artifact)]
    #[value(default)]
    pub stairs: RetainedOrderedMap<String, Stair>,
    #[state(artifact)]
    #[value(default)]
    pub railings: RetainedOrderedMap<String, Railing>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub ramps: RetainedOrderedMap<String, Ramp>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub ceiling_types: RetainedOrderedMap<String, CeilingType>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub ceilings: RetainedOrderedMap<String, Ceiling>,
    #[state(artifact)]
    #[value(default)]
    pub spaces: RetainedOrderedMap<String, Space>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub zones: RetainedOrderedMap<String, Zone>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub area_schemes: RetainedOrderedMap<String, AreaScheme>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub space_conditions: RetainedOrderedMap<String, SpaceConditions>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub views: RetainedOrderedMap<String, View>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub sheets: RetainedOrderedMap<String, Sheet>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub viewports: RetainedOrderedMap<String, Viewport>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub sheet_revisions: RetainedOrderedMap<String, SheetRevision>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub clash_sets: RetainedOrderedMap<String, ClashSet>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub rules: RetainedOrderedMap<String, Rule>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub issues: RetainedOrderedMap<String, Issue>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub issue_comments: RetainedOrderedMap<String, IssueComment>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub dimensions: RetainedOrderedMap<String, Dimension>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub tags: RetainedOrderedMap<String, Tag>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub text_notes: RetainedOrderedMap<String, TextNote>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub leaders: RetainedOrderedMap<String, Leader>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub annotation_styles: RetainedOrderedMap<String, AnnotationStyle>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub wall_sweeps: RetainedOrderedMap<String, WallSweep>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub schedules: RetainedOrderedMap<String, Schedule>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub families: RetainedOrderedMap<String, Family>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub family_parameters: RetainedOrderedMap<String, FamilyParameter>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub family_solids: RetainedOrderedMap<String, FamilySolid>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub components: RetainedOrderedMap<String, Component>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub component_overrides: RetainedOrderedMap<String, ComponentOverride>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub mep_elements: RetainedOrderedMap<String, MepElement>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub property_templates: RetainedOrderedMap<String, PropertyTemplate>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RetainedOrderedMap::is_empty")]
    pub classification_systems: RetainedOrderedMap<String, ClassificationSystem>,
    #[state(artifact)]
    #[value(default)]
    pub properties: RetainedOrderedMap<String, PropertySet>,
    #[state(artifact)]
    #[value(default)]
    pub classifications: RetainedOrderedMap<String, ClassificationSet>,
}

impl Default for ModelSnapshot {
    fn default() -> Self {
        Self {
            schema: semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA.into(),
            supports: RetainedOrderedMap::new(),
            load_cases: RetainedOrderedMap::new(),
            loads: RetainedOrderedMap::new(),
            project: Project { name: String::new(), description: String::new(), author: String::new(), organization: String::new(), phase_names: Vec::new() },
            option_groups: RetainedOrderedMap::new(),
            design_options: RetainedOrderedMap::new(),
            worksets: RetainedOrderedMap::new(),
            element_options: RetainedOrderedMap::new(),
            element_worksets: RetainedOrderedMap::new(),
            materials: RetainedOrderedMap::new(),
            wall_types: RetainedOrderedMap::new(),
            slab_types: RetainedOrderedMap::new(),
            roof_types: RetainedOrderedMap::new(),
            column_types: RetainedOrderedMap::new(),
            beam_types: RetainedOrderedMap::new(),
            window_types: RetainedOrderedMap::new(),
            door_types: RetainedOrderedMap::new(),
            curtain_wall_types: RetainedOrderedMap::new(),
            sites: RetainedOrderedMap::new(),
            buildings: RetainedOrderedMap::new(),
            storeys: RetainedOrderedMap::new(),
            grids: RetainedOrderedMap::new(),
            walls: RetainedOrderedMap::new(),
            curtain_walls: RetainedOrderedMap::new(),
            curtain_panel_overrides: RetainedOrderedMap::new(),
            columns: RetainedOrderedMap::new(),
            beams: RetainedOrderedMap::new(),
            slabs: RetainedOrderedMap::new(),
            roofs: RetainedOrderedMap::new(),
            openings: RetainedOrderedMap::new(),
            stairs: RetainedOrderedMap::new(),
            railings: RetainedOrderedMap::new(),
            ramps: RetainedOrderedMap::new(),
            ceiling_types: RetainedOrderedMap::new(),
            ceilings: RetainedOrderedMap::new(),
            spaces: RetainedOrderedMap::new(),
            zones: RetainedOrderedMap::new(),
            area_schemes: RetainedOrderedMap::new(),
            space_conditions: RetainedOrderedMap::new(),
            views: RetainedOrderedMap::new(),
            sheets: RetainedOrderedMap::new(),
            viewports: RetainedOrderedMap::new(),
            sheet_revisions: RetainedOrderedMap::new(),
            clash_sets: RetainedOrderedMap::new(),
            rules: RetainedOrderedMap::new(),
            issues: RetainedOrderedMap::new(),
            issue_comments: RetainedOrderedMap::new(),
            dimensions: RetainedOrderedMap::new(),
            tags: RetainedOrderedMap::new(),
            text_notes: RetainedOrderedMap::new(),
            leaders: RetainedOrderedMap::new(),
            annotation_styles: RetainedOrderedMap::new(),
            wall_sweeps: RetainedOrderedMap::new(),
            schedules: RetainedOrderedMap::new(),
            families: RetainedOrderedMap::new(),
            family_parameters: RetainedOrderedMap::new(),
            family_solids: RetainedOrderedMap::new(),
            components: RetainedOrderedMap::new(),
            component_overrides: RetainedOrderedMap::new(),
            mep_elements: RetainedOrderedMap::new(),
            property_templates: RetainedOrderedMap::new(),
            classification_systems: RetainedOrderedMap::new(),
            properties: RetainedOrderedMap::new(),
            classifications: RetainedOrderedMap::new(),
        }
    }
}
//#endregion 🔖️Snapshot
