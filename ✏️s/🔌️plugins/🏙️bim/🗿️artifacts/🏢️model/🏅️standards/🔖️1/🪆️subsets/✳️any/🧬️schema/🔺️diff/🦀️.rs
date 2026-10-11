//! 🔺️ BIM model diff: a sparse per-collection keyed delta (`Created | Deleted | Replaced | Patched` per id) plus a
//! project patch. The central applier is the only caller of [`MutationDiff::apply`]; `between` exists for
//! sync and import only and is never called from a mutation leaf.

use crate::standards::v1::subsets::any::schema::snapshot::{entities::*, values::*, ModelSnapshot};
use crate::{StructuralSupport, LoadCase, StructuralLoad, StructuralLocation, Restraints};
use protocol::{ApplyCapability, DiffAlgebra, DiffRegions, MutationApplyError, MutationApplyResult, MutationDiff, TouchedPaths};
use schema::ArtifactSchema;
use std::collections::BTreeMap;

/// ➕️ Later slot wins: `merge_slot!(&mut earlier, later)` writes `later` over `earlier` when it names a value.
macro_rules! merge_slot {
    ($slot:expr, $later:expr) => {
        if let later @ Some(_) = $later {
            *$slot = later;
        }
    };
}

#[path = "🩹️patches/🦀️.rs"]
pub mod patches;

pub use patches::*;

//#region 🔖️PatchAlgebra
/// 🩹 A sparse field patch of one record type: absent fields are untouched.
pub trait Patch<T>: Clone + Default + PartialEq {
    /// ✍️ The record after the patch is written over `base`.
    fn write(&self, base: &T) -> T;
    /// 🔁️ The patch that restores exactly the fields this patch names to their `base` values.
    fn restoring(&self, base: &T) -> Self;
    /// ➕️ Composes `later` over `self`: the later value of a field wins.
    fn merge(&mut self, later: Self);
    /// ✂️ This patch without the fields that already hold `base`'s value: exactly what really changes.
    fn minimal(&self, base: &T) -> Self;
    /// 📍️ The field names this patch writes.
    fn touched(&self) -> Vec<String>;
    /// 🕳️ Whether the patch names no field.
    fn is_empty(&self) -> bool;
}

/// 🎯️ An explicitly assigned optional value: keeps "set to none" distinct from "untouched" on the wire.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Assigned<T> {
    pub value: T,
}

impl<T> Assigned<T> {
    /// 🏗️ Wraps the assigned value.
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

/// ✍️ The patched value of a plain field.
pub fn take<T: Clone>(slot: &Option<T>, base: &T) -> T {
    slot.as_ref().unwrap_or(base).clone()
}

/// ✍️ The patched value of an optional field.
pub fn take_assigned<T: Clone>(slot: &Option<Assigned<Option<T>>>, base: &Option<T>) -> Option<T> {
    slot.as_ref().map_or_else(|| base.clone(), |assigned| assigned.value.clone())
}

/// 🔁️ The restoring slot of a plain field.
pub fn restore<T: Clone>(slot: &Option<T>, base: &T) -> Option<T> {
    slot.as_ref().map(|_| base.clone())
}

/// 🔁️ The restoring slot of an optional field.
pub fn restore_assigned<T: Clone>(slot: &Option<Assigned<Option<T>>>, base: &Option<T>) -> Option<Assigned<Option<T>>> {
    slot.as_ref().map(|_| Assigned::new(base.clone()))
}

/// ✂️ The slot of a plain field, dropped when it restates `base`.
pub fn changed<T: Clone + PartialEq>(slot: &Option<T>, base: &T) -> Option<T> {
    slot.as_ref().filter(|value| *value != base).cloned()
}

/// ✂️ The slot of an optional field, dropped when it restates `base`.
pub fn changed_assigned<T: Clone + PartialEq>(slot: &Option<Assigned<Option<T>>>, base: &Option<T>) -> Option<Assigned<Option<T>>> {
    slot.as_ref().filter(|assigned| assigned.value != *base).cloned()
}

/// 🧭️ The slot naming `to` when it differs from `from`.
pub fn differs<T: Clone + PartialEq>(from: &T, to: &T) -> Option<T> {
    (from != to).then(|| to.clone())
}

/// 🧭️ The optional-field slot naming `to` when it differs from `from`.
pub fn differs_assigned<T: Clone + PartialEq>(from: &Option<T>, to: &Option<T>) -> Option<Assigned<Option<T>>> {
    (from != to).then(|| Assigned::new(to.clone()))
}

/// 🏷️ Sparse patch of one element's property sets: property set → property → value (`None` removes).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct PropertySetPatch {
    pub assigned: BTreeMap<String, BTreeMap<String, Option<PropertyValue>>>,
}

impl Patch<PropertySet> for PropertySetPatch {
    fn write(&self, base: &PropertySet) -> PropertySet {
        let mut next = base.clone();
        for (set, properties) in &self.assigned {
            for (name, value) in properties {
                match value {
                    Some(value) => {
                        next.entry(set.clone()).or_default().insert(name.clone(), value.clone());
                    }
                    None => {
                        if let Some(group) = next.get_mut(set) {
                            group.remove(name);
                            if group.is_empty() {
                                next.remove(set);
                            }
                        }
                    }
                }
            }
        }
        next
    }

    fn restoring(&self, base: &PropertySet) -> Self {
        let assigned = self.assigned.iter().map(|(set, properties)| (set.clone(), properties.keys().map(|name| (name.clone(), base.get(set).and_then(|group| group.get(name)).cloned())).collect())).collect();
        Self { assigned }
    }

    fn merge(&mut self, later: Self) {
        for (set, properties) in later.assigned {
            self.assigned.entry(set).or_default().extend(properties);
        }
    }

    fn minimal(&self, base: &PropertySet) -> Self {
        let assigned = self
            .assigned
            .iter()
            .map(|(set, properties)| (set.clone(), properties.iter().filter(|(name, value)| base.get(set).and_then(|group| group.get(*name)) != value.as_ref()).map(|(name, value)| (name.clone(), value.clone())).collect::<BTreeMap<_, _>>()))
            .filter(|(_, properties)| !properties.is_empty())
            .collect();
        Self { assigned }
    }

    fn touched(&self) -> Vec<String> {
        self.assigned.keys().cloned().collect()
    }

    fn is_empty(&self) -> bool {
        self.assigned.values().all(BTreeMap::is_empty)
    }
}

/// 🗂️ Sparse patch of one element's classifications: classification system → code (`None` removes the classification of that system).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ClassificationSetPatch {
    pub assigned: BTreeMap<String, Option<String>>,
}

impl Patch<ClassificationSet> for ClassificationSetPatch {
    fn write(&self, base: &ClassificationSet) -> ClassificationSet {
        let mut next = base.clone();
        for (system, code) in &self.assigned {
            match code {
                Some(code) => {
                    next.insert(system.clone(), code.clone());
                }
                None => {
                    next.remove(system);
                }
            }
        }
        next
    }

    fn restoring(&self, base: &ClassificationSet) -> Self {
        Self { assigned: self.assigned.keys().map(|system| (system.clone(), base.get(system).cloned())).collect() }
    }

    fn merge(&mut self, later: Self) {
        self.assigned.extend(later.assigned);
    }

    fn minimal(&self, base: &ClassificationSet) -> Self {
        Self { assigned: self.assigned.iter().filter(|(system, code)| base.get(*system) != code.as_ref()).map(|(system, code)| (system.clone(), code.clone())).collect() }
    }

    fn touched(&self) -> Vec<String> {
        self.assigned.keys().cloned().collect()
    }

    fn is_empty(&self) -> bool {
        self.assigned.is_empty()
    }
}
//#endregion 🔖️PatchAlgebra

//#region 🔖️KeyedDelta
/// 🧩 One keyed change of a collection.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "entry")]
pub enum Entry<T, P> {
    Created(T),
    Deleted,
    Replaced(T),
    Patched(P),
}

/// 🧩 Keyed delta of one id-keyed collection: at most one entry per id, in canonical id order.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct KeyedDelta<T, P>(pub BTreeMap<String, Entry<T, P>>);

impl<T, P> Default for KeyedDelta<T, P> {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl<T: Clone + PartialEq, P: Patch<T>> KeyedDelta<T, P> {
    /// 🏗️ A delta of one entry.
    pub fn one(id: impl Into<String>, entry: Entry<T, P>) -> Self {
        Self(BTreeMap::from([(id.into(), entry)]))
    }

    /// 🕳️ Whether the delta names no entry.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// ✔️ Validates every entry against `base` and returns the next collection; refuses with the offending id as target.
    pub fn write_into(&self, base: &BTreeMap<String, T>) -> Result<BTreeMap<String, T>, MutationApplyError> {
        for (id, entry) in &self.0 {
            match (entry, base.contains_key(id)) {
                (Entry::Created(_), true) => return Err(MutationApplyError::new("mutation.apply.duplicate-id", "created id already exists").at([id.as_str()])),
                (Entry::Deleted | Entry::Replaced(_) | Entry::Patched(_), false) => return Err(MutationApplyError::new("mutation.apply.missing-target", "changed id does not exist").at([id.as_str()])),
                _ => {}
            }
        }
        let untouched = base.iter().filter(|(id, _)| !self.0.contains_key(*id)).map(|(id, record)| (id.clone(), record.clone()));
        let written = self.0.iter().filter_map(|(id, entry)| match entry {
            Entry::Created(record) | Entry::Replaced(record) => Some((id.clone(), record.clone())),
            Entry::Deleted => None,
            Entry::Patched(patch) => Some((id.clone(), patch.write(&base[id]))),
        });
        Ok(untouched.chain(written).collect())
    }

    /// ➕️ The sequential composition of `later` after `self`, per key: created∘patched keeps created, created∘deleted cancels,
    /// deleted∘created becomes replaced, replaced∘patched keeps replaced, patched∘patched merges, anything∘deleted deletes.
    pub fn then(self, later: Self) -> Self {
        let mut rows = self.0;
        for (id, entry) in later.0 {
            let composed = match (rows.remove(&id), entry) {
                (None, entry) => Some(entry),
                (Some(Entry::Created(_)), Entry::Deleted) => None,
                (Some(Entry::Created(_)), Entry::Created(record) | Entry::Replaced(record)) => Some(Entry::Created(record)),
                (Some(Entry::Created(record)), Entry::Patched(patch)) => Some(Entry::Created(patch.write(&record))),
                (Some(Entry::Deleted), Entry::Created(record) | Entry::Replaced(record)) => Some(Entry::Replaced(record)),
                (Some(Entry::Deleted | Entry::Patched(_)), Entry::Deleted) | (Some(Entry::Replaced(_)), Entry::Deleted) => Some(Entry::Deleted),
                (Some(Entry::Deleted), Entry::Patched(_)) => Some(Entry::Deleted),
                (Some(Entry::Replaced(_) | Entry::Patched(_)), Entry::Created(record) | Entry::Replaced(record)) => Some(Entry::Replaced(record)),
                (Some(Entry::Replaced(record)), Entry::Patched(patch)) => Some(Entry::Replaced(patch.write(&record))),
                (Some(Entry::Patched(mut earlier)), Entry::Patched(patch)) => {
                    earlier.merge(patch);
                    Some(Entry::Patched(earlier))
                }
            };
            if let Some(composed) = composed {
                rows.insert(id, composed);
            }
        }
        Self(rows)
    }

    /// 🔁️ The negative delta against `base`: created → deleted, deleted → created with the base record, replaced → replaced with
    /// the base record, patched → the patch restoring exactly the patched fields.
    pub fn inverse(&self, base: &BTreeMap<String, T>) -> Self {
        Self(
            self.0
                .iter()
                .filter_map(|(id, entry)| {
                    let inverse = match entry {
                        Entry::Created(_) => Entry::Deleted,
                        Entry::Deleted => Entry::Created(base.get(id)?.clone()),
                        Entry::Replaced(_) => Entry::Replaced(base.get(id)?.clone()),
                        Entry::Patched(patch) => Entry::Patched(patch.restoring(base.get(id)?)),
                    };
                    Some((id.clone(), inverse))
                })
                .collect(),
        )
    }

    /// 📍️ Region paths of the delta: `<collection>/<id>` for structural entries, `<collection>/<id>/<field>` for patches.
    pub fn region_paths(&self, collection: &str) -> Vec<String> {
        self.0
            .iter()
            .flat_map(|(id, entry)| match entry {
                Entry::Patched(patch) => patch.touched().into_iter().map(|field| format!("{collection}/{id}/{field}")).collect::<Vec<_>>(),
                _ => vec![format!("{collection}/{id}")],
            })
            .collect()
    }
}

fn combine<T: Clone + PartialEq, P: Patch<T>>(target: Option<KeyedDelta<T, P>>, later: Option<KeyedDelta<T, P>>) -> Option<KeyedDelta<T, P>> {
    let composed = match (target, later) {
        (Some(prior), Some(later)) => Some(prior.then(later)),
        (None, Some(later)) => Some(later),
        (target, None) => target,
    };
    composed.filter(|delta| !delta.is_empty())
}

fn combine_patch<T, P: Patch<T>>(target: Option<P>, later: Option<P>) -> Option<P> {
    let composed = match (target, later) {
        (Some(mut prior), Some(later)) => {
            prior.merge(later);
            Some(prior)
        }
        (None, Some(later)) => Some(later),
        (target, None) => target,
    };
    composed.filter(|patch| !<P as Patch<T>>::is_empty(patch))
}

fn non_empty<T: Clone + PartialEq, P: Patch<T>>(delta: KeyedDelta<T, P>) -> Option<KeyedDelta<T, P>> {
    (!delta.is_empty()).then_some(delta)
}
//#endregion 🔖️KeyedDelta

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
