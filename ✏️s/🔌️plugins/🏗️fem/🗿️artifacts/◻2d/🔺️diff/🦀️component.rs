//! 🔺️ Fem2d artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::fem2d::diff::schema::{Fem2dNodesDelta, Fem2dElementsDelta, Fem2dRegionsDelta, Fem2dMaterialsDelta, Fem2dSectionsDelta, Fem2dSupportsDelta, Fem2dLoadCasesDelta, Fem2dCombinationsDelta, Fem2dDiff};
use crate::artifacts::fem2d::schema::Fem2dArtifact;
use crate::artifacts::fem2d::{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport, Fem2dSnapshot, FemRegion, element_id};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

//#region 🔖️Collections
pub(crate) trait HasId {
    fn id(&self) -> &str;
}

impl HasId for FemNode { fn id(&self) -> &str { &self.id } }
impl HasId for FemElement { fn id(&self) -> &str { element_id(self) } }
impl HasId for FemMaterial { fn id(&self) -> &str { &self.id } }
impl HasId for FemSection { fn id(&self) -> &str { &self.id } }
impl HasId for FemSupport { fn id(&self) -> &str { &self.id } }
impl HasId for FemLoadCase { fn id(&self) -> &str { &self.id } }
impl HasId for FemCombination { fn id(&self) -> &str { &self.id } }
impl HasId for FemRegion { fn id(&self) -> &str { &self.id } }

pub(crate) fn index_of<T: HasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}

fn apply_delta<T: HasId + Clone, P>(items: &[T], delta: &P) -> Vec<T>
where
    P: DeltaAccess<T>,
{
    let mut next = items.to_vec();
    for id in delta.removed() {
        next.retain(|item| item.id() != id);
    }
    for item in delta.added() {
        if let Some(pos) = next.iter().position(|existing| existing.id() == item.id()) {
            next[pos] = item.clone();
        } else {
            next.push(item.clone());
        }
    }
    for (id, item) in delta.patched() {
        if let Some(pos) = next.iter().position(|existing| existing.id() == id) {
            next[pos] = item.clone();
        }
    }
    if let Some(order) = delta.reordered() {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (item.id().to_string(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

trait DeltaAccess<T: HasId + Clone> {
    fn added(&self) -> &[T];
    fn removed(&self) -> &[String];
    fn patched(&self) -> Vec<(String, T)>;
    fn reordered(&self) -> Option<&[String]>;
}

macro_rules! impl_delta_access {
    ($delta:ty, $item:ty, $entry:ty) => {
        impl DeltaAccess<$item> for $delta {
            fn added(&self) -> &[$item] { &self.added }
            fn removed(&self) -> &[String] { &self.removed }
            fn patched(&self) -> Vec<(String, $item)> {
                self.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect()
            }
            fn reordered(&self) -> Option<&[String]> { self.reordered.as_deref() }
        }
    };
}
impl_delta_access!(Fem2dNodesDelta, FemNode, Fem2dNodesPatchEntry);
impl_delta_access!(Fem2dElementsDelta, FemElement, Fem2dElementsPatchEntry);
impl_delta_access!(Fem2dRegionsDelta, FemRegion, Fem2dRegionsPatchEntry);
impl_delta_access!(Fem2dMaterialsDelta, FemMaterial, Fem2dMaterialsPatchEntry);
impl_delta_access!(Fem2dSectionsDelta, FemSection, Fem2dSectionsPatchEntry);
impl_delta_access!(Fem2dSupportsDelta, FemSupport, Fem2dSupportsPatchEntry);
impl_delta_access!(Fem2dLoadCasesDelta, FemLoadCase, Fem2dLoadCasesPatchEntry);
impl_delta_access!(Fem2dCombinationsDelta, FemCombination, Fem2dCombinationsPatchEntry);

//#endregion 🔖️Collections

//#region 🔖️Apply
impl Fem2dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &Fem2dArtifact) -> Fem2dArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(delta) = &self.nodes {
            next.nodes = apply_delta(&next.nodes, delta);
        }
        if let Some(delta) = &self.elements {
            next.elements = apply_delta(&next.elements, delta);
        }
        if let Some(delta) = &self.regions {
            next.regions = apply_delta(&next.regions, delta);
        }
        if let Some(delta) = &self.materials {
            next.materials = apply_delta(&next.materials, delta);
        }
        if let Some(delta) = &self.sections {
            next.sections = apply_delta(&next.sections, delta);
        }
        if let Some(delta) = &self.supports {
            next.supports = apply_delta(&next.supports, delta);
        }
        if let Some(delta) = &self.load_cases {
            next.load_cases = apply_delta(&next.load_cases, delta);
        }
        if let Some(delta) = &self.combinations {
            next.combinations = apply_delta(&next.combinations, delta);
        }
        if let Some(value) = &self.analysis { next.analysis = value.clone(); }
        if let Some(value) = &self.result_source_id { next.result_source_id = value.clone(); }
        if let Some(value) = &self.result_mode { next.result_mode = value.clone(); }
        if let Some(value) = self.result_mode_index { next.result_mode_index = value; }
        if let Some(value) = &self.camera { next.camera = value.clone(); }
        if let Some(value) = &self.locale { next.locale = value.clone(); }
        if let Some(value) = &self.solver_results_json { next.solver_results_json = value.clone(); }
        if let Some(value) = &self.mesh_preview_json { next.mesh_preview_json = value.clone(); }
        next
    }
}

impl MutationDiff<Fem2dSnapshot> for Fem2dDiff {
    fn apply(&self, snapshot: &Fem2dSnapshot) -> Fem2dSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(delta) = &self.nodes {
            next.nodes = apply_delta(&next.nodes, delta);
        }
        if let Some(delta) = &self.elements {
            next.elements = apply_delta(&next.elements, delta);
        }
        if let Some(delta) = &self.regions {
            next.regions = apply_delta(&next.regions, delta);
        }
        if let Some(delta) = &self.materials {
            next.materials = apply_delta(&next.materials, delta);
        }
        if let Some(delta) = &self.sections {
            next.sections = apply_delta(&next.sections, delta);
        }
        if let Some(delta) = &self.supports {
            next.supports = apply_delta(&next.supports, delta);
        }
        if let Some(delta) = &self.load_cases {
            next.load_cases = apply_delta(&next.load_cases, delta);
        }
        if let Some(delta) = &self.combinations {
            next.combinations = apply_delta(&next.combinations, delta);
        }
        if let Some(value) = &self.analysis { next.analysis = value.clone(); }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(analysis);
        take!(result_source_id);
        take!(result_mode);
        take!(result_mode_index);
        take!(camera);
        take!(locale);
        take!(solver_results_json);
        take!(mesh_preview_json);
        merge_delta(&mut self.nodes, other.nodes);
        merge_delta(&mut self.elements, other.elements);
        merge_delta(&mut self.regions, other.regions);
        merge_delta(&mut self.materials, other.materials);
        merge_delta(&mut self.sections, other.sections);
        merge_delta(&mut self.supports, other.supports);
        merge_delta(&mut self.load_cases, other.load_cases);
        merge_delta(&mut self.combinations, other.combinations);
    }
}

fn merge_delta<D: DeltaMerge>(dst: &mut Option<D>, src: Option<D>) {
    match (dst.as_mut(), src) {
        (Some(d), Some(s)) => d.merge_from(s),
        (None, Some(s)) => *dst = Some(s),
        _ => {}
    }
}

trait DeltaMerge {
    fn merge_from(&mut self, other: Self);
}

macro_rules! impl_merge {
    ($t:ty) => {
        impl DeltaMerge for $t {
            fn merge_from(&mut self, other: Self) {
                self.added.extend(other.added);
                self.removed.extend(other.removed);
                self.patched.extend(other.patched);
                if other.reordered.is_some() {
                    self.reordered = other.reordered;
                }
            }
        }
    };
}
    impl_merge!(Fem2dNodesDelta);
    impl_merge!(Fem2dElementsDelta);
    impl_merge!(Fem2dRegionsDelta);
    impl_merge!(Fem2dMaterialsDelta);
    impl_merge!(Fem2dSectionsDelta);
    impl_merge!(Fem2dSupportsDelta);
    impl_merge!(Fem2dLoadCasesDelta);
    impl_merge!(Fem2dCombinationsDelta);
//#endregion 🔖️Apply

//#region 🔖️Constructors

/// 🏗️ Set-node field delta.
pub fn diff_set_node(index: usize, item: FemNode, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dNodesPatchEntry;
    let id = item.id().to_string();
    let delta = if base.nodes.iter().any(|existing| existing.id() == id) {
        Fem2dNodesDelta {
            patched: vec![Fem2dNodesPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.nodes.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem2dNodesDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem2dDiff { nodes: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-node field delta.
pub fn diff_remove_node(id: String) -> Fem2dDiff {
    Fem2dDiff {
        nodes: Some(Fem2dNodesDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-element field delta.
pub fn diff_set_element(index: usize, item: FemElement, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dElementsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.elements.iter().any(|existing| existing.id() == id) {
        Fem2dElementsDelta {
            patched: vec![Fem2dElementsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.elements.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem2dElementsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem2dDiff { elements: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-element field delta.
pub fn diff_remove_element(id: String) -> Fem2dDiff {
    Fem2dDiff {
        elements: Some(Fem2dElementsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-region field delta.
pub fn diff_set_region(index: usize, item: FemRegion, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dRegionsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.regions.iter().any(|existing| existing.id() == id) {
        Fem2dRegionsDelta {
            patched: vec![Fem2dRegionsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.regions.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem2dRegionsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem2dDiff { regions: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-region field delta.
pub fn diff_remove_region(id: String) -> Fem2dDiff {
    Fem2dDiff {
        regions: Some(Fem2dRegionsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-material field delta.
pub fn diff_set_material(index: usize, item: FemMaterial, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dMaterialsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.materials.iter().any(|existing| existing.id() == id) {
        Fem2dMaterialsDelta {
            patched: vec![Fem2dMaterialsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.materials.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem2dMaterialsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem2dDiff { materials: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-material field delta.
pub fn diff_remove_material(id: String) -> Fem2dDiff {
    Fem2dDiff {
        materials: Some(Fem2dMaterialsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-section field delta.
pub fn diff_set_section(index: usize, item: FemSection, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dSectionsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.sections.iter().any(|existing| existing.id() == id) {
        Fem2dSectionsDelta {
            patched: vec![Fem2dSectionsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.sections.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem2dSectionsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem2dDiff { sections: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-section field delta.
pub fn diff_remove_section(id: String) -> Fem2dDiff {
    Fem2dDiff {
        sections: Some(Fem2dSectionsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-support field delta.
pub fn diff_set_support(index: usize, item: FemSupport, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dSupportsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.supports.iter().any(|existing| existing.id() == id) {
        Fem2dSupportsDelta {
            patched: vec![Fem2dSupportsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.supports.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem2dSupportsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem2dDiff { supports: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-support field delta.
pub fn diff_remove_support(id: String) -> Fem2dDiff {
    Fem2dDiff {
        supports: Some(Fem2dSupportsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-load_case field delta.
pub fn diff_set_load_case(index: usize, item: FemLoadCase, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dLoadCasesPatchEntry;
    let id = item.id().to_string();
    let delta = if base.load_cases.iter().any(|existing| existing.id() == id) {
        Fem2dLoadCasesDelta {
            patched: vec![Fem2dLoadCasesPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.load_cases.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem2dLoadCasesDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem2dDiff { load_cases: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-load_case field delta.
pub fn diff_remove_load_case(id: String) -> Fem2dDiff {
    Fem2dDiff {
        load_cases: Some(Fem2dLoadCasesDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-combination field delta.
pub fn diff_set_combination(index: usize, item: FemCombination, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dCombinationsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.combinations.iter().any(|existing| existing.id() == id) {
        Fem2dCombinationsDelta {
            patched: vec![Fem2dCombinationsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.combinations.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem2dCombinationsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem2dDiff { combinations: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-combination field delta.
pub fn diff_remove_combination(id: String) -> Fem2dDiff {
    Fem2dDiff {
        combinations: Some(Fem2dCombinationsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Analysis settings field delta.
pub fn diff_set_analysis(settings: FemAnalysisSettings) -> Fem2dDiff {
    Fem2dDiff { analysis: Some(settings), ..Default::default() }
}

/// 🏗️ Whole-snapshot replacement field delta.
pub fn diff_set_snapshot(snapshot: Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff {
        artifact: Some(Box::new(Fem2dArtifact::from_snapshot(snapshot))),
        ..Default::default()
    }
}
//#endregion 🔖️Constructors
