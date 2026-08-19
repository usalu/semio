//! 🔺️ Fem3d artifact — sparse field-delta diff codec and apply/absorb.


use crate::artifacts::fem3d::schema::diff::{Fem3dNodesDelta, Fem3dElementsDelta, Fem3dMaterialsDelta, Fem3dSectionsDelta, Fem3dSolidsDelta, Fem3dSupportsDelta, Fem3dLoadCasesDelta, Fem3dCombinationsDelta, Fem3dDiff};
use crate::artifacts::fem3d::schema::Fem3dArtifact;
use crate::artifacts::fem3d::{FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport, Fem3dSnapshot, FemSolid, element_id};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Collections
pub(crate) trait HasId {
    async fn id(&self) -> &str;
}

impl HasId for FemNode { fn id(&self) -> &str { &self.id } }
impl HasId for FemElement { fn id(&self) -> &str { element_id(self) } }
impl HasId for FemMaterial { fn id(&self) -> &str { &self.id } }
impl HasId for FemSection { fn id(&self) -> &str { &self.id } }
impl HasId for FemSupport { fn id(&self) -> &str { &self.id } }
impl HasId for FemLoadCase { fn id(&self) -> &str { &self.id } }
impl HasId for FemCombination { fn id(&self) -> &str { &self.id } }
impl HasId for FemSolid { fn id(&self) -> &str { &self.id } }

async fn apply_delta<T: HasId + Clone, P>(
    items: &[T],
    delta: &P,
) -> protocol::MutationApplyResult<Vec<T>>
where
    P: DeltaAccess<T>,
{
    for (index, id) in delta.removed().iter().enumerate() {
        if !items.iter().any(|item| item.id() == id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.missing-target",
                "removed item does not exist",
            )
            .at(["removed".to_string(), index.to_string()]));
        }
        if delta.removed()[..index].contains(id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "item is removed more than once",
            )
            .at(["removed".to_string(), index.to_string()]));
        }
    }
    for (index, item) in delta.added().iter().enumerate() {
        if items.iter().any(|existing| existing.id() == item.id())
            || delta.added()[..index]
                .iter()
                .any(|existing| existing.id() == item.id())
        {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "added item identity already exists",
            )
            .at(["added".to_string(), index.to_string()]));
        }
    }
    let patched = delta.patched();
    for (index, (id, item)) in patched.iter().enumerate() {
        if !items.iter().any(|existing| existing.id() == id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.missing-target",
                "patched item does not exist",
            )
            .at(["patched".to_string(), index.to_string()]));
        }
        if delta.removed().contains(id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.conflicting-target",
                "item cannot be removed and patched",
            )
            .at(["patched".to_string(), index.to_string()]));
        }
        if patched[..index].iter().any(|(prior, _)| prior == id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "item is patched more than once",
            )
            .at(["patched".to_string(), index.to_string()]));
        }
        if item.id() != id
            && (items.iter().any(|existing| existing.id() == item.id())
                || delta.added().iter().any(|added| added.id() == item.id()))
        {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "patched item identity collides with another item",
            )
            .at(["patched".to_string(), index.to_string()]));
        }
    }
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
    for (id, item) in patched {
        if let Some(pos) = next.iter().position(|existing| existing.id() == id) {
            next[pos] = item.clone();
        }
    }
    let mut resulting_ids = std::collections::HashSet::new();
    if !next.iter().all(|item| resulting_ids.insert(item.id())) {
        return Err(protocol::MutationApplyError::new(
            "mutation.apply.duplicate-target",
            "resulting collection contains duplicate identities",
        )
        .at(["identities"]));
    }
    if let Some(order) = delta.reordered() {
        if order.len() != next.len()
            || order.iter().enumerate().any(|(index, id)| {
                order[..index].contains(id) || !next.iter().any(|item| item.id() == id)
            })
        {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.invalid-order",
                "reorder must be a complete unique permutation",
            )
            .at(["reordered"]));
        }
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (item.id().to_string(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            ordered.push(by_id.remove(id).ok_or_else(|| {
                protocol::MutationApplyError::new(
                    "mutation.apply.missing-target",
                    "reordered item does not exist",
                )
                .at(["reordered".to_string(), id.clone()])
            })?);
        }
        next = ordered;
    }
    Ok(next)
}

trait DeltaAccess<T: HasId + Clone> {
    async fn added(&self) -> &[T];
    async fn removed(&self) -> &[String];
    async fn patched(&self) -> Vec<(String, T)>;
    async fn reordered(&self) -> Option<&[String]>;
}

macro_rules! impl_delta_access {
    ($delta:ty, $item:ty, $entry:ty) => {
        impl DeltaAccess<$item> for $delta {
            async fn added(&self) -> &[$item] { &self.added }
            async fn removed(&self) -> &[String] { &self.removed }
            async fn patched(&self) -> Vec<(String, $item)> {
                self.patched.iter().map(|e| (e.id.clone(), e.item.clone())).collect()
            }
            async fn reordered(&self) -> Option<&[String]> { self.reordered.as_deref() }
        }
    };
}
impl_delta_access!(Fem3dNodesDelta, FemNode, Fem3dNodesPatchEntry);
impl_delta_access!(Fem3dElementsDelta, FemElement, Fem3dElementsPatchEntry);
impl_delta_access!(Fem3dMaterialsDelta, FemMaterial, Fem3dMaterialsPatchEntry);
impl_delta_access!(Fem3dSectionsDelta, FemSection, Fem3dSectionsPatchEntry);
impl_delta_access!(Fem3dSolidsDelta, FemSolid, Fem3dSolidsPatchEntry);
impl_delta_access!(Fem3dSupportsDelta, FemSupport, Fem3dSupportsPatchEntry);
impl_delta_access!(Fem3dLoadCasesDelta, FemLoadCase, Fem3dLoadCasesPatchEntry);
impl_delta_access!(Fem3dCombinationsDelta, FemCombination, Fem3dCombinationsPatchEntry);

//#endregion 🔖️Collections

//#region 🔖️Apply
impl Fem3dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &Fem3dArtifact) -> protocol::MutationApplyResult<Fem3dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(delta) = &self.nodes {
                next.nodes = apply_delta(&next.nodes, delta)
                    .map_err(|error| error.under(["nodes"]))?;
            }
            if let Some(delta) = &self.elements {
                next.elements = apply_delta(&next.elements, delta)
                    .map_err(|error| error.under(["elements"]))?;
            }
            if let Some(delta) = &self.materials {
                next.materials = apply_delta(&next.materials, delta)
                    .map_err(|error| error.under(["materials"]))?;
            }
            if let Some(delta) = &self.sections {
                next.sections = apply_delta(&next.sections, delta)
                    .map_err(|error| error.under(["sections"]))?;
            }
            if let Some(delta) = &self.solids {
                next.solids = apply_delta(&next.solids, delta)
                    .map_err(|error| error.under(["solids"]))?;
            }
            if let Some(delta) = &self.supports {
                next.supports = apply_delta(&next.supports, delta)
                    .map_err(|error| error.under(["supports"]))?;
            }
            if let Some(delta) = &self.load_cases {
                next.load_cases = apply_delta(&next.load_cases, delta)
                    .map_err(|error| error.under(["loadCases"]))?;
            }
            if let Some(delta) = &self.combinations {
                next.combinations = apply_delta(&next.combinations, delta)
                    .map_err(|error| error.under(["combinations"]))?;
            }
            if let Some(value) = &self.analysis { next.analysis = value.clone(); }
            if let Some(value) = &self.result_source_id { next.result_source_id = value.clone(); }
            if let Some(value) = &self.result_mode { next.result_mode = value.clone(); }
            if let Some(value) = self.result_mode_index { next.result_mode_index = value; }
            if let Some(value) = &self.camera { next.camera = value.clone(); }
            if let Some(value) = &self.solver_results_json { next.solver_results_json = value.clone(); }
            if let Some(value) = &self.mesh_preview_json { next.mesh_preview_json = value.clone(); }
            next
        })
    }
}

impl MutationDiff<Fem3dSnapshot> for Fem3dDiff {
    async fn apply(&self, snapshot: &Fem3dSnapshot) -> protocol::MutationApplyResult<Fem3dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(delta) = &self.nodes {
                next.nodes = apply_delta(&next.nodes, delta)
                    .map_err(|error| error.under(["nodes"]))?;
            }
            if let Some(delta) = &self.elements {
                next.elements = apply_delta(&next.elements, delta)
                    .map_err(|error| error.under(["elements"]))?;
            }
            if let Some(delta) = &self.materials {
                next.materials = apply_delta(&next.materials, delta)
                    .map_err(|error| error.under(["materials"]))?;
            }
            if let Some(delta) = &self.sections {
                next.sections = apply_delta(&next.sections, delta)
                    .map_err(|error| error.under(["sections"]))?;
            }
            if let Some(delta) = &self.solids {
                next.solids = apply_delta(&next.solids, delta)
                    .map_err(|error| error.under(["solids"]))?;
            }
            if let Some(delta) = &self.supports {
                next.supports = apply_delta(&next.supports, delta)
                    .map_err(|error| error.under(["supports"]))?;
            }
            if let Some(delta) = &self.load_cases {
                next.load_cases = apply_delta(&next.load_cases, delta)
                    .map_err(|error| error.under(["loadCases"]))?;
            }
            if let Some(delta) = &self.combinations {
                next.combinations = apply_delta(&next.combinations, delta)
                    .map_err(|error| error.under(["combinations"]))?;
            }
            if let Some(value) = &self.analysis { next.analysis = value.clone(); }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
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
        take!(solver_results_json);
        take!(mesh_preview_json);
        merge_delta(&mut self.nodes, other.nodes);
        merge_delta(&mut self.elements, other.elements);
        merge_delta(&mut self.materials, other.materials);
        merge_delta(&mut self.sections, other.sections);
        merge_delta(&mut self.solids, other.solids);
        merge_delta(&mut self.supports, other.supports);
        merge_delta(&mut self.load_cases, other.load_cases);
        merge_delta(&mut self.combinations, other.combinations);
    }
}

async fn merge_delta<D: DeltaMerge>(dst: &mut Option<D>, src: Option<D>) {
    match (dst.as_mut(), src) {
        (Some(d), Some(s)) => d.merge_from(s),
        (None, Some(s)) => *dst = Some(s),
        _ => {}
    }
}

trait DeltaMerge {
    async fn merge_from(&mut self, other: Self);
}

macro_rules! impl_merge {
    ($t:ty) => {
        impl DeltaMerge for $t {
            async fn merge_from(&mut self, other: Self) {
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
    impl_merge!(Fem3dNodesDelta);
    impl_merge!(Fem3dElementsDelta);
    impl_merge!(Fem3dMaterialsDelta);
    impl_merge!(Fem3dSectionsDelta);
    impl_merge!(Fem3dSolidsDelta);
    impl_merge!(Fem3dSupportsDelta);
    impl_merge!(Fem3dLoadCasesDelta);
    impl_merge!(Fem3dCombinationsDelta);
//#endregion 🔖️Apply

//#region 🔖️Constructors

/// 🏗️ Set-node field delta.
pub async fn diff_set_node(index: usize, item: FemNode, base: &Fem3dSnapshot) -> Fem3dDiff {
    use crate::artifacts::fem3d::schema::diff::Fem3dNodesPatchEntry;
    let id = item.id().to_string();
    let delta = if base.nodes.iter().any(|existing| existing.id() == id) {
        Fem3dNodesDelta {
            patched: vec![Fem3dNodesPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.nodes.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem3dNodesDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem3dDiff { nodes: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-node field delta.
pub async fn diff_remove_node(id: String) -> Fem3dDiff {
    Fem3dDiff {
        nodes: Some(Fem3dNodesDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-element field delta.
pub async fn diff_set_element(index: usize, item: FemElement, base: &Fem3dSnapshot) -> Fem3dDiff {
    use crate::artifacts::fem3d::schema::diff::Fem3dElementsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.elements.iter().any(|existing| existing.id() == id) {
        Fem3dElementsDelta {
            patched: vec![Fem3dElementsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.elements.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem3dElementsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem3dDiff { elements: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-element field delta.
pub async fn diff_remove_element(id: String) -> Fem3dDiff {
    Fem3dDiff {
        elements: Some(Fem3dElementsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-material field delta.
pub async fn diff_set_material(index: usize, item: FemMaterial, base: &Fem3dSnapshot) -> Fem3dDiff {
    use crate::artifacts::fem3d::schema::diff::Fem3dMaterialsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.materials.iter().any(|existing| existing.id() == id) {
        Fem3dMaterialsDelta {
            patched: vec![Fem3dMaterialsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.materials.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem3dMaterialsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem3dDiff { materials: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-material field delta.
pub async fn diff_remove_material(id: String) -> Fem3dDiff {
    Fem3dDiff {
        materials: Some(Fem3dMaterialsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-section field delta.
pub async fn diff_set_section(index: usize, item: FemSection, base: &Fem3dSnapshot) -> Fem3dDiff {
    use crate::artifacts::fem3d::schema::diff::Fem3dSectionsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.sections.iter().any(|existing| existing.id() == id) {
        Fem3dSectionsDelta {
            patched: vec![Fem3dSectionsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.sections.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem3dSectionsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem3dDiff { sections: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-section field delta.
pub async fn diff_remove_section(id: String) -> Fem3dDiff {
    Fem3dDiff {
        sections: Some(Fem3dSectionsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-solid field delta.
pub async fn diff_set_solid(index: usize, item: FemSolid, base: &Fem3dSnapshot) -> Fem3dDiff {
    use crate::artifacts::fem3d::schema::diff::Fem3dSolidsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.solids.iter().any(|existing| existing.id() == id) {
        Fem3dSolidsDelta {
            patched: vec![Fem3dSolidsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.solids.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem3dSolidsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem3dDiff { solids: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-solid field delta.
pub async fn diff_remove_solid(id: String) -> Fem3dDiff {
    Fem3dDiff {
        solids: Some(Fem3dSolidsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-support field delta.
pub async fn diff_set_support(index: usize, item: FemSupport, base: &Fem3dSnapshot) -> Fem3dDiff {
    use crate::artifacts::fem3d::schema::diff::Fem3dSupportsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.supports.iter().any(|existing| existing.id() == id) {
        Fem3dSupportsDelta {
            patched: vec![Fem3dSupportsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.supports.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem3dSupportsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem3dDiff { supports: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-support field delta.
pub async fn diff_remove_support(id: String) -> Fem3dDiff {
    Fem3dDiff {
        supports: Some(Fem3dSupportsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-load_case field delta.
pub async fn diff_set_load_case(index: usize, item: FemLoadCase, base: &Fem3dSnapshot) -> Fem3dDiff {
    use crate::artifacts::fem3d::schema::diff::Fem3dLoadCasesPatchEntry;
    let id = item.id().to_string();
    let delta = if base.load_cases.iter().any(|existing| existing.id() == id) {
        Fem3dLoadCasesDelta {
            patched: vec![Fem3dLoadCasesPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.load_cases.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem3dLoadCasesDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem3dDiff { load_cases: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-load_case field delta.
pub async fn diff_remove_load_case(id: String) -> Fem3dDiff {
    Fem3dDiff {
        load_cases: Some(Fem3dLoadCasesDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Set-combination field delta.
pub async fn diff_set_combination(index: usize, item: FemCombination, base: &Fem3dSnapshot) -> Fem3dDiff {
    use crate::artifacts::fem3d::schema::diff::Fem3dCombinationsPatchEntry;
    let id = item.id().to_string();
    let delta = if base.combinations.iter().any(|existing| existing.id() == id) {
        Fem3dCombinationsDelta {
            patched: vec![Fem3dCombinationsPatchEntry { id, item }],
            ..Default::default()
        }
    } else {
        let mut order: Vec<String> = base.combinations.iter().map(|existing| existing.id().to_string()).collect();
        let at = index.min(order.len());
        order.insert(at, id);
        Fem3dCombinationsDelta {
            added: vec![item],
            reordered: Some(order),
            ..Default::default()
        }
    };
    Fem3dDiff { combinations: Some(delta), ..Default::default() }
}

/// 🏗️ Remove-combination field delta.
pub async fn diff_remove_combination(id: String) -> Fem3dDiff {
    Fem3dDiff {
        combinations: Some(Fem3dCombinationsDelta { removed: vec![id], ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Analysis settings field delta.
pub async fn diff_set_analysis(settings: FemAnalysisSettings) -> Fem3dDiff {
    Fem3dDiff { analysis: Some(settings), ..Default::default() }
}

/// 🏗️ Whole-snapshot replacement field delta.
pub async fn diff_set_snapshot(snapshot: Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff {
        artifact: Some(Box::new(Fem3dArtifact::from_snapshot(snapshot))),
        ..Default::default()
    }
}
//#endregion 🔖️Constructors
