#!/usr/bin/env python3
"""Rewrite FEM root/diff/engine/glue and bulk-rename Document→Snapshot."""
from __future__ import annotations
from pathlib import Path
import re

FEM = Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem')
ART2 = FEM / '🗿️artifacts' / '◻2d'
ART3 = FEM / '🗿️artifacts' / '🧊️3d'

def patch_root_2d():
    p = ART2 / '🦀️component.rs'
    text = p.read_text()
    # Remove Fem2dDocument struct + codecs region; keep helpers; add reexports
    # Find /// 🧾️ Persistent fem-2d document
    start = text.find('/// 🧾️ Persistent fem-2d document')
    end = text.find('// #endregion 🔖️Document')
    if start < 0 or end < 0:
        raise SystemExit('cannot find Fem2dDocument block')
    replacement = '''/// 📸️ `Fem2dSnapshot` lives in `📸️snapshot/🧬️schema` — re-exported here for crate consumers.
pub use crate::artifacts::fem2d::snapshot::schema::Fem2dSnapshot;
pub use crate::artifacts::fem2d::schema::Fem2dArtifact;

'''
    text = text[:start] + replacement + text[end:]
    # FemCamera stays (used by config)
    p.write_text(text)
    print('patched root 2d')

def patch_root_3d():
    p = ART3 / '🦀️component.rs'
    text = p.read_text()
    start = text.find('/// 🧾️ Persistent fem-3d document')
    end = text.find('// #endregion 🔖️Document')
    if start < 0 or end < 0:
        raise SystemExit('cannot find Fem3dDocument block')
    replacement = '''/// 📸️ `Fem3dSnapshot` lives in `📸️snapshot/🧬️schema` — re-exported here for crate consumers.
pub use crate::artifacts::fem3d::snapshot::schema::Fem3dSnapshot;
pub use crate::artifacts::fem3d::schema::Fem3dArtifact;

'''
    text = text[:start] + replacement + text[end:]
    p.write_text(text)
    print('patched root 3d')

DIFF_RUNTIME_2D = r'''//! 🔺️ FEM 2D artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::fem2d::diff::schema::{
    Fem2dCombinationsDelta, Fem2dDiff, Fem2dElementsDelta, Fem2dLoadCasesDelta, Fem2dMaterialsDelta,
    Fem2dNodesDelta, Fem2dRegionsDelta, Fem2dSectionsDelta, Fem2dSupportsDelta,
};
use crate::artifacts::fem2d::schema::Fem2dArtifact;
use crate::artifacts::fem2d::{element_id, Fem2dSnapshot, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport, FemCombination};
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
impl HasId for FemRegion { fn id(&self) -> &str { &self.id } }
impl HasId for FemCombination { fn id(&self) -> &str { &self.id } }

pub(crate) fn index_of<T: HasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}

fn apply_identified_delta<T: HasId + Clone>(
    items: &[T],
    added: &[T],
    removed: &[String],
    patched: &[(String, T)],
    reordered: Option<&[String]>,
) -> Vec<T> {
    let mut next = items.to_vec();
    for id in removed {
        next.retain(|item| item.id() != id);
    }
    for item in added {
        if let Some(pos) = next.iter().position(|existing| existing.id() == item.id()) {
            next[pos] = item.clone();
        } else {
            next.push(item.clone());
        }
    }
    for (id, item) in patched {
        if let Some(pos) = next.iter().position(|existing| existing.id() == id) {
            next[pos] = item.clone();
        }
    }
    if let Some(order) = reordered {
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
            next.nodes = apply_identified_delta(&next.nodes, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.elements {
            next.elements = apply_identified_delta(&next.elements, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.regions {
            next.regions = apply_identified_delta(&next.regions, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.materials {
            next.materials = apply_identified_delta(&next.materials, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.sections {
            next.sections = apply_identified_delta(&next.sections, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.supports {
            next.supports = apply_identified_delta(&next.supports, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.load_cases {
            next.load_cases = apply_identified_delta(&next.load_cases, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.combinations {
            next.combinations = apply_identified_delta(&next.combinations, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
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
            next.nodes = apply_identified_delta(&next.nodes, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.elements {
            next.elements = apply_identified_delta(&next.elements, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.regions {
            next.regions = apply_identified_delta(&next.regions, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.materials {
            next.materials = apply_identified_delta(&next.materials, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.sections {
            next.sections = apply_identified_delta(&next.sections, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.supports {
            next.supports = apply_identified_delta(&next.supports, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.load_cases {
            next.load_cases = apply_identified_delta(&next.load_cases, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
        }
        if let Some(delta) = &self.combinations {
            next.combinations = apply_identified_delta(&next.combinations, &delta.added, &delta.removed, &delta.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect::<Vec<_>>(), delta.reordered.as_deref());
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
        for (dst, src) in [
            (&mut self.nodes, other.nodes),
            // handled below individually due to types
        ] { let _ = (dst, src); }
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

fn merge_delta<D>(dst: &mut Option<D>, src: Option<D>)
where
    D: Default,
    D: mergeable::Mergeable,
{
    match (dst.as_mut(), src) {
        (Some(d), Some(s)) => d.merge(s),
        (d, Some(s)) => *d = Some(s),
        _ => {}
    }
}

mod mergeable {
    use super::*;
    pub trait Mergeable {
        fn merge(&mut self, other: Self);
    }
    macro_rules! impl_merge {
        ($t:ty) => {
            impl Mergeable for $t {
                fn merge(&mut self, other: Self) {
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
}
//#endregion 🔖️Apply

//#region 🔖️Constructors
fn upsert_delta<T: HasId + Clone, D, PE>(
    index: usize,
    item: T,
    base: &[T],
    make: impl FnOnce(Vec<T>, Vec<String>, Vec<PE>, Option<Vec<String>>) -> D,
    patch_entry: impl FnOnce(String, T) -> PE,
) -> D {
    let id = item.id().to_string();
    if base.iter().any(|existing| existing.id() == id) {
        make(vec![], vec![], vec![patch_entry(id, item)], None)
    } else {
        let mut order: Vec<String> = base.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        make(vec![item], vec![], vec![], Some(order))
    }
}

/// 🏗️ Set-node field delta.
pub fn diff_set_node(index: usize, node: FemNode, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dNodesPatchEntry;
    Fem2dDiff {
        nodes: Some(upsert_delta(index, node, &base.nodes, |added, removed, patched, reordered| Fem2dNodesDelta { added, removed, patched, reordered }, |id, item| Fem2dNodesPatchEntry { id, item })),
        ..Default::default()
    }
}
pub fn diff_remove_node(id: String) -> Fem2dDiff {
    Fem2dDiff { nodes: Some(Fem2dNodesDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_element(index: usize, element: FemElement, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dElementsPatchEntry;
    Fem2dDiff {
        elements: Some(upsert_delta(index, element, &base.elements, |added, removed, patched, reordered| Fem2dElementsDelta { added, removed, patched, reordered }, |id, item| Fem2dElementsPatchEntry { id, item })),
        ..Default::default()
    }
}
pub fn diff_remove_element(id: String) -> Fem2dDiff {
    Fem2dDiff { elements: Some(Fem2dElementsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_material(index: usize, material: FemMaterial, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dMaterialsPatchEntry;
    Fem2dDiff {
        materials: Some(upsert_delta(index, material, &base.materials, |a,r,p,o| Fem2dMaterialsDelta { added:a, removed:r, patched:p, reordered:o }, |id, item| Fem2dMaterialsPatchEntry { id, item })),
        ..Default::default()
    }
}
pub fn diff_remove_material(id: String) -> Fem2dDiff {
    Fem2dDiff { materials: Some(Fem2dMaterialsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_section(index: usize, section: FemSection, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dSectionsPatchEntry;
    Fem2dDiff {
        sections: Some(upsert_delta(index, section, &base.sections, |a,r,p,o| Fem2dSectionsDelta { added:a, removed:r, patched:p, reordered:o }, |id, item| Fem2dSectionsPatchEntry { id, item })),
        ..Default::default()
    }
}
pub fn diff_remove_section(id: String) -> Fem2dDiff {
    Fem2dDiff { sections: Some(Fem2dSectionsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_support(index: usize, support: FemSupport, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dSupportsPatchEntry;
    Fem2dDiff {
        supports: Some(upsert_delta(index, support, &base.supports, |a,r,p,o| Fem2dSupportsDelta { added:a, removed:r, patched:p, reordered:o }, |id, item| Fem2dSupportsPatchEntry { id, item })),
        ..Default::default()
    }
}
pub fn diff_remove_support(id: String) -> Fem2dDiff {
    Fem2dDiff { supports: Some(Fem2dSupportsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_load_case(index: usize, load_case: FemLoadCase, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dLoadCasesPatchEntry;
    Fem2dDiff {
        load_cases: Some(upsert_delta(index, load_case, &base.load_cases, |a,r,p,o| Fem2dLoadCasesDelta { added:a, removed:r, patched:p, reordered:o }, |id, item| Fem2dLoadCasesPatchEntry { id, item })),
        ..Default::default()
    }
}
pub fn diff_remove_load_case(id: String) -> Fem2dDiff {
    Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_region(index: usize, region: FemRegion, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dRegionsPatchEntry;
    Fem2dDiff {
        regions: Some(upsert_delta(index, region, &base.regions, |a,r,p,o| Fem2dRegionsDelta { added:a, removed:r, patched:p, reordered:o }, |id, item| Fem2dRegionsPatchEntry { id, item })),
        ..Default::default()
    }
}
pub fn diff_remove_region(id: String) -> Fem2dDiff {
    Fem2dDiff { regions: Some(Fem2dRegionsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_combination(index: usize, combination: FemCombination, base: &Fem2dSnapshot) -> Fem2dDiff {
    use crate::artifacts::fem2d::diff::schema::Fem2dCombinationsPatchEntry;
    Fem2dDiff {
        combinations: Some(upsert_delta(index, combination, &base.combinations, |a,r,p,o| Fem2dCombinationsDelta { added:a, removed:r, patched:p, reordered:o }, |id, item| Fem2dCombinationsPatchEntry { id, item })),
        ..Default::default()
    }
}
pub fn diff_remove_combination(id: String) -> Fem2dDiff {
    Fem2dDiff { combinations: Some(Fem2dCombinationsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_analysis(settings: crate::artifacts::fem2d::FemAnalysisSettings) -> Fem2dDiff {
    Fem2dDiff { analysis: Some(settings), ..Default::default() }
}
pub fn diff_set_snapshot(snapshot: Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { artifact: Some(Box::new(Fem2dArtifact::from_snapshot(snapshot))), ..Default::default() }
}
//#endregion 🔖️Constructors
'''

    # The apply_identified_delta with collect inside call is inefficient and may have borrow issues.
    # Simplify: rewrite apply helpers without temporary collect in call.
    # Actually the temporary Vec is fine in Rust.
    
    (ART2 / '🔺️diff' / '🦀️component.rs').write_text(DIFF_RUNTIME_2D)
    print('wrote diff runtime 2d')

patch_root_2d()
patch_root_3d()
# Write 2d diff - but fix the apply_identified_delta call to avoid lifetime issues by helper
print('phase rewire partial done - continuing in next script for 3d diff + bulk rename')
