//! 🔺️ BIM model diff: a sparse per-collection keyed delta (`Created | Deleted | Replaced | Patched` per id) plus a
//! project patch. The central applier is the only caller of [`MutationDiff::apply`]; `between` exists for
//! sync and import only and is never called from a mutation leaf.

use crate::standards::v1::subsets::any::schema::snapshot::{entities::*, values::*, ModelSnapshot};
use crate::standards::v1::subsets::any::schema::snapshot::{StructuralSupport, LoadCase, StructuralLoad, StructuralLocation, Restraints};
use protocol::{ApplyCapability, DiffAlgebra, DiffRegions, MutationApplyError, MutationApplyResult, MutationDiff, TouchedPaths};
use schema::ArtifactSchema;
use std::collections::BTreeMap;

#[path = "🧮️algebra/🦀️.rs"]
pub mod algebra;
#[path = "🩹️patches/🦀️.rs"]
pub mod patches;

pub use algebra::*;
pub use patches::*;

//#region 🔖️ModelDiff
macro_rules! collections {
    ($callback:ident) => {
        $callback! {
            option_groups: OptionGroup, OptionGroupPatch;
            design_options: DesignOption, DesignOptionPatch;
            worksets: Workset, WorksetPatch;
            element_options: ElementMembership, ElementMembershipPatch;
            element_worksets: ElementMembership, ElementMembershipPatch;
            supports: StructuralSupport, StructuralSupportPatch;
            load_cases: LoadCase, LoadCasePatch;
            loads: StructuralLoad, StructuralLoadPatch;
            materials: Material, MaterialPatch;
            wall_types: WallType, WallTypePatch;
            slab_types: SlabType, SlabTypePatch;
            roof_types: RoofType, RoofTypePatch;
            column_types: ColumnType, ColumnTypePatch;
            beam_types: BeamType, BeamTypePatch;
            window_types: WindowType, WindowTypePatch;
            door_types: DoorType, DoorTypePatch;
            curtain_wall_types: CurtainWallType, CurtainWallTypePatch;
            sites: Site, SitePatch;
            buildings: Building, BuildingPatch;
            storeys: Storey, StoreyPatch;
            grids: GridLine, GridLinePatch;
            walls: Wall, WallPatch;
            curtain_walls: CurtainWall, CurtainWallPatch;
            curtain_panel_overrides: CurtainPanelOverride, CurtainPanelOverridePatch;
            columns: Column, ColumnPatch;
            beams: Beam, BeamPatch;
            slabs: Slab, SlabPatch;
            roofs: Roof, RoofPatch;
            openings: Opening, OpeningPatch;
            stairs: Stair, StairPatch;
            railings: Railing, RailingPatch;
            ramps: Ramp, RampPatch;
            ceiling_types: CeilingType, CeilingTypePatch;
            ceilings: Ceiling, CeilingPatch;
            spaces: Space, SpacePatch;
            zones: Zone, ZonePatch;
            area_schemes: AreaScheme, AreaSchemePatch;
            space_conditions: SpaceConditions, SpaceConditionsPatch;
            views: View, ViewPatch;
            sheets: Sheet, SheetPatch;
            viewports: Viewport, ViewportPatch;
            sheet_revisions: SheetRevision, SheetRevisionPatch;
            clash_sets: ClashSet, ClashSetPatch;
            rules: Rule, RulePatch;
            issues: Issue, IssuePatch;
            issue_comments: IssueComment, IssueCommentPatch;
            dimensions: Dimension, DimensionPatch;
            tags: Tag, TagPatch;
            text_notes: TextNote, TextNotePatch;
            leaders: Leader, LeaderPatch;
            annotation_styles: AnnotationStyle, AnnotationStylePatch;
            wall_sweeps: WallSweep, WallSweepPatch;
            schedules: Schedule, SchedulePatch;
            families: Family, FamilyPatch;
            family_parameters: FamilyParameter, FamilyParameterPatch;
            family_solids: FamilySolid, FamilySolidPatch;
            components: Component, ComponentPatch;
            component_overrides: ComponentOverride, ComponentOverridePatch;
            mep_elements: MepElement, MepElementPatch;
            property_templates: PropertyTemplate, PropertyTemplatePatch;
            classification_systems: ClassificationSystem, ClassificationSystemPatch;
            properties: PropertySet, PropertySetPatch;
            classifications: ClassificationSet, ClassificationSetPatch;
        }
    };
}

macro_rules! model_diff {
    ($($field:ident : $entity:ty, $patch:ty;)*) => {
        /// 🔺️ Sparse typed diff of a [`ModelSnapshot`]: one optional keyed delta per collection, absent = untouched.
        #[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
        #[value(default)]
        #[artifact_schema(id = "s.bim.model")]
        pub struct ModelDiff {
            #[state(artifact)]
            #[value(skip_serializing_if = "Option::is_none")]
            pub project: Option<ProjectPatch>,
            $(
                #[state(artifact)]
                #[value(skip_serializing_if = "Option::is_none")]
                pub $field: Option<KeyedDelta<$entity, $patch>>,
            )*
        }

        impl MutationDiff<ModelSnapshot> for ModelDiff {
            fn apply(&self, base: &ModelSnapshot, _capability: ApplyCapability) -> MutationApplyResult<ModelSnapshot> {
                let mut next = base.clone();
                if let Some(patch) = &self.project {
                    next.project = patch.write(&base.project);
                }
                $(
                    if let Some(delta) = &self.$field {
                        next.$field = delta.write_into(&base.$field).map_err(|error| error.under([stringify!($field)]))?;
                    }
                )*
                Ok(next)
            }

            fn absorb(&mut self, other: Self) {
                self.project = combine_patch::<Project, ProjectPatch>(self.project.take(), other.project);
                $( self.$field = combine(self.$field.take(), other.$field); )*
            }
        }

        impl DiffAlgebra<ModelSnapshot> for ModelDiff {
            fn inverse(&self, base: &ModelSnapshot) -> Self {
                Self {
                    project: self.project.as_ref().map(|patch| patch.restoring(&base.project)).filter(|patch| !<ProjectPatch as Patch<Project>>::is_empty(patch)),
                    $( $field: self.$field.as_ref().map(|delta| delta.inverse(&base.$field)).and_then(non_empty), )*
                }
            }

            fn is_empty(&self) -> bool {
                self.project.as_ref().is_none_or(<ProjectPatch as Patch<Project>>::is_empty) $( && self.$field.as_ref().is_none_or(KeyedDelta::is_empty) )*
            }
        }

        impl DiffRegions for ModelDiff {
            fn touches(&self) -> TouchedPaths {
                let mut paths = Vec::new();
                if let Some(patch) = &self.project {
                    paths.extend(patch.touched().into_iter().map(|field| format!("project/{field}")));
                }
                $( if let Some(delta) = &self.$field { paths.extend(delta.region_paths(stringify!($field))); } )*
                TouchedPaths::new(paths)
            }
        }

        impl ModelDiff {
            $(
                /// 🏗️ A diff carrying one keyed entry of this collection.
                pub fn $field(id: impl Into<String>, entry: Entry<$entity, $patch>) -> Self {
                    Self { $field: Some(KeyedDelta::one(id, entry)), ..Self::default() }
                }
            )*
        }
    };
}

collections!(model_diff);
//#endregion 🔖️ModelDiff

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
