//! ⚡️ FEM 2D artifact — semantic mutation dispatch enum + laws (constitutional: op). Every variant
//! wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Fem2dSnapshot, Fem2dMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.

use crate::artifacts::fem2d::diff::Fem2dDiff;
use crate::artifacts::fem2d::Fem2dSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the fem2d document, derived per
/// `📓️derivation-rules.md` from `Fem2dSnapshot`'s shape (8 id-keyed collections + one inseparable
/// analysis-settings facet). Every generic `Set*`/`Remove*` variant this facet used to carry —
/// including the banned `SetSnapshot` whole-document-replace variant — is gone; whole-document
/// replace is not an in-history mutation at all (routed through `Effect::LoadDocument`, see
/// `Fem2dPlayApp::whole_document_operation` returning `None` now and `editor::fem2d::reset_document_effect`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Fem2dSnapshot, diff = Fem2dDiff, schema = "fem.fem2d")]
pub enum Fem2dMutation {
    CreateNode(create_node::CreateNode),
    DeleteNode(delete_node::DeleteNode),
    CreateElement(create_element::CreateElement),
    DeleteElement(delete_element::DeleteElement),
    ReplaceElement(replace_element::ReplaceElement),
    CreateMaterial(create_material::CreateMaterial),
    DeleteMaterial(delete_material::DeleteMaterial),
    ReplaceMaterial(replace_material::ReplaceMaterial),
    CreateSection(create_section::CreateSection),
    DeleteSection(delete_section::DeleteSection),
    ReplaceSection(replace_section::ReplaceSection),
    CreateSupport(create_support::CreateSupport),
    DeleteSupport(delete_support::DeleteSupport),
    ReplaceSupport(replace_support::ReplaceSupport),
    CreateRegion(create_region::CreateRegion),
    DeleteRegion(delete_region::DeleteRegion),
    ReplaceRegion(replace_region::ReplaceRegion),
    CreateLoadCase(create_load_case::CreateLoadCase),
    DeleteLoadCase(delete_load_case::DeleteLoadCase),
    AddLoad(add_load::AddLoad),
    RemoveLoad(remove_load::RemoveLoad),
    ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight),
    CreateCombination(create_combination::CreateCombination),
    DeleteCombination(delete_combination::DeleteCombination),
    UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings),
}
//#endregion 🔖️Mutations

//#region 🔖️LeafImports
/// 🌉️ Brings every triad leaf's `mutation` submodule into this file's own scope (declared as
/// siblings back in `🦀️.rs`, not inside this file) — required for the dispatch enum's bare
/// `create_node::CreateNode`-style variant field paths above to resolve.
use super::add_load;
use super::change_load_case_self_weight;
use super::create_combination;
use super::create_element;
use super::create_load_case;
use super::create_material;
use super::create_node;
use super::create_region;
use super::create_section;
use super::create_support;
use super::delete_combination;
use super::delete_element;
use super::delete_load_case;
use super::delete_material;
use super::delete_node;
use super::delete_region;
use super::delete_section;
use super::delete_support;
use super::remove_load;
use super::replace_element;
use super::replace_material;
use super::replace_region;
use super::replace_section;
use super::replace_support;
use super::update_analysis_settings;
//#endregion 🔖️LeafImports

pub type Fem2dEnvelope = ArtifactEnvelope<Fem2dSnapshot, Fem2dMutation>;
pub type Fem2dStore = ArtifactStore<Fem2dSnapshot, Fem2dMutation>;

//#region 🔖️GenericDelegates
/// 🌉️ Thin delegates to the derive-generated `protocol::Mutation` impl — kept because
/// `🏗️builder/🦀️.rs` (an artifact-generic caller, no per-variant knowledge) and
/// `📝️text/🦀️.rs`'s re-export both call these by name.
pub fn apply_fem2d_mutation(snapshot: &mut Fem2dSnapshot, mutation: &Fem2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(snapshot, mutation)?;

    *snapshot = next;
    Ok(())
}

pub fn inverse_fem2d_mutation(snapshot: &Fem2dSnapshot, mutation: &Fem2dMutation) -> Vec<Fem2dMutation> {
    mutation.inverse(snapshot)
}
//#endregion 🔖️GenericDelegates

//#region 🛡️Guards
/// 🛡️ The payload validations a noun's `create-`/`replace-` twins share, the foreign-key
/// resolutions `create-load-case` and `add-load` share, and the referential-integrity lookups every
/// guarded `delete-` runs — written ONCE, here, beside the dispatch enum.
///
/// Why here and not in each leaf: the twins are only meaningful if they agree, and a transcription
/// per leaf is exactly how they drifted apart (a `create-element` that resolved four references
/// next to a `replace-element` that resolved none). Each leaf now spends one line per rule.
///
/// 🚦️ Level discipline. `mutation.duplicate-id`, `mutation.id-mismatch` and `mutation.invariant`
/// are `Fatal`: they say the PAYLOAD is wrong, so no merge policy may absorb them and no later base
/// can make them right. `mutation.target-missing` and `mutation.target-referenced` are `Error`:
/// they say this BASE cannot host the payload, which a different base may well be able to.
///
/// @see 📓️w13-fem2d-semantics.md — the per-kind rule table these guards implement.
pub mod guards {
    use crate::artifacts::fem2d::diff::Fem2dDiff;
    use crate::artifacts::fem2d::{element_id, load_id, Fem2dSnapshot, FemAnalysisSettings, FemElement, FemLoad, FemMaterial, FemNode, FemRegion, FemSection};

    type Rejection = protocol::MutationOutcome<Fem2dDiff>;

    /// 🚫️ `Fatal` — the payload itself is inadmissible, on this base or any other.
    fn invariant(target: impl IntoIterator<Item = String>, message: String) -> Rejection {
        protocol::MutationOutcome::fatal("mutation.invariant", message, target)
    }

    /// 🚫️ `Error` — this base cannot host the payload.
    fn missing(target: String, message: String) -> Rejection {
        protocol::MutationOutcome::error("mutation.target-missing", message, [target])
    }

    /// 🪪️ A `replace-` selects its target by `id` and carries a whole record that also carries an
    /// `id`; letting the two differ is a silent RENAME that orphans every referrer, so it is
    /// refused. Renaming a record is `delete-` + `create-` + re-pointing the referrers.
    pub fn identity_matches(noun: &str, target: &str, replacement: &str) -> Option<Rejection> {
        (target != replacement).then(|| protocol::MutationOutcome::fatal("mutation.id-mismatch", format!("A replace-{noun} selects \"{target}\" but carries a record identified \"{replacement}\"; a replacement may not rename its target."), [target.to_string(), replacement.to_string()]))
    }

    /// 🔗️ A `delete-` may not orphan a live reference. The diagnostic addresses the target FIRST
    /// and then every referrer, so a caller can offer to re-point or remove them.
    pub fn referenced(noun: &str, blocker: &str, target: &str, referrers: Vec<String>) -> Option<Rejection> {
        (!referrers.is_empty()).then(|| {
            let listed = referrers.join(", ");
            let mut address = vec![target.to_string()];
            address.extend(referrers.iter().cloned());
            protocol::MutationOutcome::error("mutation.target-referenced", format!("{noun} \"{target}\" is still referenced by {} {blocker}(s): {listed}.", referrers.len()), address)
        })
    }

    /// 📍️ A node's coordinates are real numbers — a non-finite ordinate cannot be meshed, drawn or
    /// solved, and would silently poison every stiffness matrix it enters.
    pub fn node_geometry(node: &FemNode) -> Option<Rejection> {
        (!node.x.is_finite() || !node.y.is_finite()).then(|| invariant([node.id.clone()], format!("Node \"{}\" carries a non-finite coordinate ({}, {}).", node.id, node.x, node.y)))
    }

    /// 🧱️ Isotropic-elasticity bounds: a positive Young's modulus and density, and a Poisson ratio
    /// inside the thermodynamically admissible open interval (-1, 0.5) — at 0.5 the material is
    /// incompressible and the plane-stress/plane-strain constitutive matrix is singular.
    pub fn material_plausibility(material: &FemMaterial) -> Option<Rejection> {
        let id = &material.id;
        if !material.e.is_finite() || !material.nu.is_finite() || !material.rho.is_finite() {
            return Some(invariant([id.clone()], format!("Material \"{id}\" carries a non-finite property.")));
        }
        if material.e <= 0.0 {
            return Some(invariant([id.clone()], format!("Material \"{id}\" needs a positive Young's modulus, got {}.", material.e)));
        }
        if material.rho <= 0.0 {
            return Some(invariant([id.clone()], format!("Material \"{id}\" needs a positive density, got {}.", material.rho)));
        }
        if !(material.nu > -1.0 && material.nu < 0.5) {
            return Some(invariant([id.clone()], format!("Material \"{id}\" needs a Poisson ratio in (-1, 0.5), got {}.", material.nu)));
        }
        None
    }

    /// 📏️ A cross-section carries a positive area and a positive strong-axis second moment of area;
    /// either at or below zero makes the member's axial or bending stiffness zero or negative.
    pub fn section_plausibility(section: &FemSection) -> Option<Rejection> {
        let id = &section.id;
        if !section.area.is_finite() || !section.iy.is_finite() {
            return Some(invariant([id.clone()], format!("Section \"{id}\" carries a non-finite property.")));
        }
        if section.area <= 0.0 {
            return Some(invariant([id.clone()], format!("Section \"{id}\" needs a positive area, got {}.", section.area)));
        }
        if section.iy <= 0.0 {
            return Some(invariant([id.clone()], format!("Section \"{id}\" needs a positive second moment of area, got {}.", section.iy)));
        }
        None
    }

    /// 📐️ Twice the shoelace sum, halved — the signed area of a closed ring, sign carrying winding.
    fn signed_area(ring: &[[f64; 2]]) -> f64 {
        let mut total = 0.0;
        for (at, point) in ring.iter().enumerate() {
            let next = ring[(at + 1) % ring.len()];
            total += point[0] * next[1] - next[0] * point[1];
        }
        total / 2.0
    }

    /// 🎯️ Crossing-number containment with the boundary counted as INSIDE, so a hole is allowed to
    /// touch the outline it is cut from (a notch) but not to leave it.
    fn point_in_ring(point: [f64; 2], ring: &[[f64; 2]]) -> bool {
        const ON_EDGE: f64 = 1e-12;
        for (at, start) in ring.iter().enumerate() {
            let end = ring[(at + 1) % ring.len()];
            let cross = (end[0] - start[0]) * (point[1] - start[1]) - (end[1] - start[1]) * (point[0] - start[0]);
            let within = point[0] >= start[0].min(end[0]) - ON_EDGE && point[0] <= start[0].max(end[0]) + ON_EDGE && point[1] >= start[1].min(end[1]) - ON_EDGE && point[1] <= start[1].max(end[1]) + ON_EDGE;
            if cross.abs() <= ON_EDGE && within {
                return true;
            }
        }
        let mut inside = false;
        for (at, start) in ring.iter().enumerate() {
            let end = ring[(at + 1) % ring.len()];
            if (start[1] > point[1]) != (end[1] > point[1]) {
                let crossing = start[0] + (point[1] - start[1]) * (end[0] - start[0]) / (end[1] - start[1]);
                if point[0] < crossing {
                    inside = !inside;
                }
            }
        }
        inside
    }

    /// 🗺️ A meshable region: an outline that is a real polygon enclosing real area, a positive
    /// thickness and mesh size, and every hole a real polygon lying inside that outline. Without
    /// this the failure only surfaces far downstream, inside `fem2d_engine::meshing`, as an empty
    /// or self-overlapping triangulation that no diagnostic points back at this edit.
    pub fn region_geometry(region: &FemRegion) -> Option<Rejection> {
        let id = &region.id;
        if region.outline.len() < 3 {
            return Some(invariant([id.clone()], format!("Region \"{id}\" needs an outline of at least three points, got {}.", region.outline.len())));
        }
        if region.outline.iter().any(|point| !point[0].is_finite() || !point[1].is_finite()) {
            return Some(invariant([id.clone()], format!("Region \"{id}\" carries a non-finite outline coordinate.")));
        }
        if signed_area(&region.outline) == 0.0 {
            return Some(invariant([id.clone()], format!("Region \"{id}\" has a degenerate outline enclosing zero area.")));
        }
        if !region.thickness.is_finite() || region.thickness <= 0.0 {
            return Some(invariant([id.clone()], format!("Region \"{id}\" needs a positive thickness, got {}.", region.thickness)));
        }
        if !region.mesh_size.is_finite() || region.mesh_size <= 0.0 {
            return Some(invariant([id.clone()], format!("Region \"{id}\" needs a positive mesh size, got {}.", region.mesh_size)));
        }
        for (at, hole) in region.holes.iter().enumerate() {
            if hole.len() < 3 {
                return Some(invariant([id.clone()], format!("Region \"{id}\" hole {at} needs at least three points, got {}.", hole.len())));
            }
            if hole.iter().any(|point| !point[0].is_finite() || !point[1].is_finite()) {
                return Some(invariant([id.clone()], format!("Region \"{id}\" hole {at} carries a non-finite coordinate.")));
            }
            if signed_area(hole) == 0.0 {
                return Some(invariant([id.clone()], format!("Region \"{id}\" hole {at} is degenerate.")));
            }
            if let Some(loose) = hole.iter().find(|point| !point_in_ring(**point, &region.outline)) {
                return Some(invariant([id.clone()], format!("Region \"{id}\" hole {at} leaves the outline at ({}, {}).", loose[0], loose[1])));
            }
        }
        None
    }

    /// ⚙️ At least one mode of each family — a zero count asks the solver for an empty spectrum —
    /// and a real, positive deformation exaggeration, which the viewport multiplies displacements by.
    pub fn analysis_bounds(settings: &FemAnalysisSettings) -> Option<Rejection> {
        if settings.modal_count < 1 {
            return Some(invariant(Vec::new(), format!("Analysis settings need at least one modal mode, got {}.", settings.modal_count)));
        }
        if settings.buckling_count < 1 {
            return Some(invariant(Vec::new(), format!("Analysis settings need at least one buckling mode, got {}.", settings.buckling_count)));
        }
        if !settings.deformation_scale.is_finite() || settings.deformation_scale <= 0.0 {
            return Some(invariant(Vec::new(), format!("Analysis settings need a positive deformation scale, got {}.", settings.deformation_scale)));
        }
        None
    }

    /// 🔗️ The one node a support names.
    pub fn node_reference(base: &Fem2dSnapshot, node_id: &str) -> Option<Rejection> {
        (!base.nodes.iter().any(|node| node.id == node_id)).then(|| missing(node_id.to_string(), format!("Node \"{node_id}\" does not exist.")))
    }

    /// 🔗️ The one material a region names.
    pub fn material_reference(base: &Fem2dSnapshot, material_id: &str) -> Option<Rejection> {
        (!base.materials.iter().any(|material| material.id == material_id)).then(|| missing(material_id.to_string(), format!("Material \"{material_id}\" does not exist.")))
    }

    /// 🔗️ The four foreign keys every element carries, resolved in declaration order.
    pub fn element_references(base: &Fem2dSnapshot, element: &FemElement) -> Option<Rejection> {
        let (start, end, material_id, section_id) = match element {
            FemElement::Bar { start, end, material_id, section_id, .. } | FemElement::Beam { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
        };
        if let Some(rejection) = node_reference(base, start) {
            return Some(rejection);
        }
        if let Some(rejection) = node_reference(base, end) {
            return Some(rejection);
        }
        if let Some(rejection) = material_reference(base, material_id) {
            return Some(rejection);
        }
        if !base.sections.iter().any(|section| &section.id == section_id) {
            return Some(missing(section_id.clone(), format!("Section \"{section_id}\" does not exist.")));
        }
        None
    }

    /// 🔗️ The one target a load variant carries — the same resolution whether the load arrives
    /// inside a new case (`create-load-case`) or is attached to an existing one (`add-load`).
    pub fn load_reference(base: &Fem2dSnapshot, load: &FemLoad) -> Option<Rejection> {
        match load {
            FemLoad::Nodal { node_id, .. } => node_reference(base, node_id),
            FemLoad::MemberUdl { element_id: referenced, .. } => (!base.elements.iter().any(|element| element_id(element) == referenced)).then(|| missing(referenced.clone(), format!("Element \"{referenced}\" does not exist."))),
            FemLoad::Area { region_id, .. } => (!base.regions.iter().any(|region| &region.id == region_id)).then(|| missing(region_id.clone(), format!("Region \"{region_id}\" does not exist."))),
        }
    }

    /// 🔎️ Every member UDL naming this element, as `"<case>/<load>"`.
    pub fn element_referrers(base: &Fem2dSnapshot, target: &str) -> Vec<String> {
        base.load_cases.iter().flat_map(|case| case.loads.iter().filter_map(move |load| matches!(load, FemLoad::MemberUdl { element_id: referenced, .. } if referenced == target).then(|| format!("{}/{}", case.id, load_id(load))))).collect()
    }

    /// 🔎️ Every element and region naming this material.
    pub fn material_referrers(base: &Fem2dSnapshot, target: &str) -> Vec<String> {
        let elements = base.elements.iter().filter(|element| match element {
            FemElement::Bar { material_id, .. } | FemElement::Beam { material_id, .. } => material_id == target,
        });
        elements.map(|element| element_id(element).to_string()).chain(base.regions.iter().filter(|region| region.material_id == target).map(|region| region.id.clone())).collect()
    }

    /// 🔎️ Every element naming this cross-section.
    pub fn section_referrers(base: &Fem2dSnapshot, target: &str) -> Vec<String> {
        base.elements
            .iter()
            .filter(|element| match element {
                FemElement::Bar { section_id, .. } | FemElement::Beam { section_id, .. } => section_id == target,
            })
            .map(|element| element_id(element).to_string())
            .collect()
    }

    /// 🔎️ Every area load naming this region, as `"<case>/<load>"`.
    pub fn region_referrers(base: &Fem2dSnapshot, target: &str) -> Vec<String> {
        base.load_cases.iter().flat_map(|case| case.loads.iter().filter_map(move |load| matches!(load, FemLoad::Area { region_id, .. } if region_id == target).then(|| format!("{}/{}", case.id, load_id(load))))).collect()
    }

    /// 🔎️ Every combination weighting this load case.
    pub fn load_case_referrers(base: &Fem2dSnapshot, target: &str) -> Vec<String> {
        base.combinations.iter().filter(|combination| combination.terms.iter().any(|term| term.case_id == target)).map(|combination| combination.id.clone()).collect()
    }

    /// 🔎️ Every OTHER combination nesting this one (a self-term cannot block its own deletion).
    pub fn combination_referrers(base: &Fem2dSnapshot, target: &str) -> Vec<String> {
        base.combinations.iter().filter(|combination| combination.id != target && combination.terms.iter().any(|term| term.case_id == target)).map(|combination| combination.id.clone()).collect()
    }
}
//#endregion 🛡️Guards

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem2d::{element_id, load_id, FemAnalysisSettings, FemCombination, FemCombinationTerm, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
    use protocol::MutationDiff;
    use protocol::SemanticMutation;

    // #region 🔖️Fixtures
    fn simply_supported_beam_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }, FemMaterial { id: "timber_spare".into(), name: "GL24h".into(), e: 11.5e9, nu: 0.3, rho: 420.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }, FemSection { id: "shs_spare".into(), name: "SHS 100x5".into(), area: 0.00184, iy: 2.79e-6 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    fn rectangle_region_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "c0".into(), x: 0.0, y: 0.0 }, FemNode { id: "c1".into(), x: 4.0, y: 0.0 }, FemNode { id: "c2".into(), x: 4.0, y: 2.0 }, FemNode { id: "c3".into(), x: 0.0, y: 2.0 }],
            elements: vec![],
            regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️OpRoundTrip
    fn round_trip(snapshot: &Fem2dSnapshot, operation: &Fem2dMutation) -> Fem2dSnapshot {
        let forward = vcs::apply_mutation(snapshot, operation).expect("valid mutation").0;
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            restored = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation").0;
        }
        assert_eq!(&restored, snapshot, "inverse() must restore the pre-mutation document");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn node_create_and_delete_round_trip() {
        let base = Fem2dSnapshot::default();
        let node = FemNode { id: "n1".into(), x: 1.0, y: 2.0 };
        let after_create = round_trip(&base, &Fem2dMutation::CreateNode(create_node::CreateNode { node: node.clone() }));
        assert_eq!(after_create.nodes, vec![node.clone()]);
        round_trip(&after_create, &Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: node.id }));
    }

    #[semio_framework_async_macros::async_test]
    async fn element_create_replace_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let updated = FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
        let after_replace = round_trip(&base, &Fem2dMutation::ReplaceElement(replace_element::ReplaceElement { id: "e1".into(), new_element: Box::new(updated) }));
        assert_eq!(element_id(&after_replace.elements[0]), "e1");
        let new_element = FemElement::Bar { id: "e2".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
        let after_create = round_trip(&after_replace, &Fem2dMutation::CreateElement(create_element::CreateElement { element: Box::new(new_element) }));
        round_trip(&after_create, &Fem2dMutation::DeleteElement(delete_element::DeleteElement { id: "e2".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn material_create_replace_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let replaced = FemMaterial { id: "steel".into(), name: "Steel Updated".into(), e: 200e9, nu: 0.3, rho: 7900.0 };
        let after_replace = round_trip(&base, &Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "steel".into(), new_material: replaced }));
        // 🔗️ Deletes the TRAILING SPARE, not `steel`: since this ticket's referential-integrity wave a
        // `delete-material` naming a grade seven members still point at is refused with
        // `mutation.target-referenced` (see `guards::material_referrers`), and a trailing record is
        // also the only one whose `create-`-shaped inverse round-trips to a byte-identical vec order.
        round_trip(&after_replace, &Fem2dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: "timber_spare".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn section_create_replace_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let replaced = FemSection { id: "ipe300".into(), name: "IPE300 Updated".into(), area: 0.01, iy: 1e-4 };
        let after_replace = round_trip(&base, &Fem2dMutation::ReplaceSection(replace_section::ReplaceSection { id: "ipe300".into(), new_section: replaced }));
        // 🔗️ Same reason as the material twin above: `ipe300` is still carried by beam `e1`, so its
        // deletion is now `mutation.target-referenced`; the trailing spare is the deletable one.
        round_trip(&after_replace, &Fem2dMutation::DeleteSection(delete_section::DeleteSection { id: "shs_spare".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn support_create_replace_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let replaced = FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Ty] };
        round_trip(&base, &Fem2dMutation::ReplaceSupport(replace_support::ReplaceSupport { id: "s1".into(), new_support: replaced }));
        // 🧮️ Deletes the LAST support: `apply_delta`'s `added` handling (`↩️inverse` recreates via
        // `create-support`, which has no `index` field) re-appends at the end of the collection, so
        // only a last-position delete round-trips to a byte-identical vec order — id-keyed collections
        // with no display order (📓️derivation-rules.md rule 2) don't guarantee position preservation
        // for a non-last delete+recreate, matching `create_support`'s fem3d sibling precedent.
        round_trip(&base, &Fem2dMutation::DeleteSupport(delete_support::DeleteSupport { id: "s2".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn region_create_replace_and_delete_round_trip() {
        let base = rectangle_region_doc();
        let updated = FemRegion { id: "r1".into(), name: "slab v2".into(), outline: vec![[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.03, material_id: "steel".into(), mesh_size: 0.5 };
        let after_replace = round_trip(&base, &Fem2dMutation::ReplaceRegion(replace_region::ReplaceRegion { id: "r1".into(), new_region: updated }));
        assert_eq!(after_replace.regions[0].thickness, 0.03);
        round_trip(&after_replace, &Fem2dMutation::DeleteRegion(delete_region::DeleteRegion { id: "r1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn load_case_create_and_delete_round_trip() {
        let base = simply_supported_beam_doc();
        let load_case = FemLoadCase { id: "wind".into(), name: "Wind Load".into(), loads: vec![], self_weight: false };
        let after_create = round_trip(&base, &Fem2dMutation::CreateLoadCase(create_load_case::CreateLoadCase { load_case }));
        round_trip(&after_create, &Fem2dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: "wind".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_load_and_remove_load_round_trip() {
        let base = simply_supported_beam_doc();
        let load = FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: -900.0 };
        let after_add = round_trip(&base, &Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "dead".into(), load: Box::new(load.clone()) }));
        assert_eq!(after_add.load_cases[0].loads.len(), 2);
        round_trip(&after_add, &Fem2dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: "dead".into(), load_id: load_id(&load).to_string() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_load_case_self_weight_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: "dead".into(), new_self_weight: true }));
    }

    #[semio_framework_async_macros::async_test]
    async fn combination_create_and_delete_round_trip() {
        let mut base = simply_supported_beam_doc();
        base.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] });
        let combination = FemCombination { id: "sls".into(), name: "SLS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.0 }] };
        let after_create = round_trip(&base, &Fem2dMutation::CreateCombination(create_combination::CreateCombination { combination }));
        round_trip(&after_create, &Fem2dMutation::DeleteCombination(delete_combination::DeleteCombination { id: "sls".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn analysis_settings_update_round_trips() {
        let base = simply_supported_beam_doc();
        let settings = FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 25.0 };
        round_trip(&base, &Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings }));
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_target_inverse_and_diff_are_no_ops() {
        let base = Fem2dSnapshot::default();
        assert!(Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: "ghost".into() }).inverse(&base).is_empty());
        assert!(Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "ghost".into(), new_material: FemMaterial { id: "ghost".into(), name: "x".into(), e: 1.0, nu: 0.3, rho: 1.0 } }).inverse(&base).is_empty());
        assert!(Fem2dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: "ghost".into(), load_id: "ghost".into() }).inverse(&base).is_empty());
        assert_eq!(*Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "ghost".into(), load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: 1.0 }) }).diff(&base).diff(), Fem2dDiff::default());
    }
    // #endregion 🔖️OpRoundTrip

    // #region 🔖️OpText
    #[semio_framework_async_macros::async_test]
    async fn fem2d_op_text_round_trips_every_variant() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateNode(create_node::CreateNode { node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: "n1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateElement(create_element::CreateElement {
            element: Box::new(FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }),
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ReplaceElement(replace_element::ReplaceElement {
            id: "e1".into(),
            new_element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteElement(delete_element::DeleteElement { id: "e1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateMaterial(create_material::CreateMaterial {
            material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial {
            id: "steel".into(),
            new_material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: "steel".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateSection(create_section::CreateSection {
            section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ReplaceSection(replace_section::ReplaceSection {
            id: "ipe300".into(),
            new_section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteSection(delete_section::DeleteSection { id: "ipe300".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateSupport(create_support::CreateSupport {
            support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteSupport(delete_support::DeleteSupport { id: "s1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateRegion(create_region::CreateRegion {
            region: FemRegion {
                id: "r1".into(),
                name: "Slab".into(),
                outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]],
                holes: vec![vec![[1.0, 1.0], [2.0, 1.0], [2.0, 1.5]]],
                thickness: 0.02,
                material_id: "steel".into(),
                mesh_size: 0.5,
            },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteRegion(delete_region::DeleteRegion { id: "r1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateLoadCase(create_load_case::CreateLoadCase {
            load_case: FemLoadCase {
                id: "dead".into(),
                name: "Dead Load".into(),
                loads: vec![
                    FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: -1000.0 },
                    FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: -5000.0 },
                    FemLoad::Area { id: "l3".into(), region_id: "r1".into(), pressure: 800.0 },
                ],
                self_weight: true,
            },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: "dead".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::AddLoad(add_load::AddLoad {
            case_id: "dead".into(),
            load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: -1000.0 }),
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: "dead".into(), load_id: "l1".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: "dead".into(), new_self_weight: true }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::CreateCombination(create_combination::CreateCombination {
            combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] },
        }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::DeleteCombination(delete_combination::DeleteCombination { id: "uls".into() }));
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings {
            settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 },
        }));
    }
    // #endregion 🔖️OpText

    // #region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn mutation_law_create_node_inverse_and_diff_absorb() {
        let base = Fem2dSnapshot::default();
        let mutation = Fem2dMutation::CreateNode(create_node::CreateNode { node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).diff().clone();
        let after = d1.apply(&base).expect("valid mutation diff");
        let d2 = Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: "none".into(), new_self_weight: true }).diff(&after).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_law_replace_material_inverse() {
        let base = simply_supported_beam_doc();
        let mutation = Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "steel".into(), new_material: FemMaterial { id: "steel".into(), name: "Steel 2".into(), e: 200e9, nu: 0.3, rho: 7900.0 } });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_law_add_load_inverse_and_diff_absorb() {
        let base = simply_supported_beam_doc();
        // 🔗️ A member UDL on the beam this document actually carries: since this ticket's wave
        // `add-load` resolves the load's own target exactly as `create-load-case` does, so the
        // area pressure over the region `r1` this fixture never had would now be refused.
        let mutation = Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "dead".into(), load: Box::new(FemLoad::MemberUdl { id: "l9".into(), element_id: "e1".into(), wx: 0.0, wy: -400.0 }) });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).diff().clone();
        let after = d1.apply(&base).expect("valid mutation diff");
        let d2 = Fem2dMutation::DeleteCombination(delete_combination::DeleteCombination { id: "none".into() }).diff(&after).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn every_mutation_registers_a_semantic_descriptor() {
        register_fem2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        let kinds = <Fem2dMutation as SemanticMutation<Fem2dSnapshot>>::kinds();
        assert_eq!(kinds.len(), 25, "every semantic mutation kind must be registered exactly once");
        for descriptor in kinds {
            assert!(protocol::is_approved_verb(descriptor.verb), "verb '{}' must be in APPROVED_VERBS", descriptor.verb);
        }
    }
    // #endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// one `assert_missing_target_is_error`/Fatal check per verb family this facet implements
    /// (create/delete/replace/add/remove/change).
    #[semio_framework_async_macros::async_test]
    async fn create_node_duplicate_id_is_fatal() {
        let base = simply_supported_beam_doc();
        let existing_id = base.nodes.first().unwrap().id.clone();
        let outcome = Fem2dMutation::CreateNode(create_node::CreateNode { node: FemNode { id: existing_id, x: 0.0, y: 0.0 } }).diff(&base);
        protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_support_missing_node_is_error() {
        let base = Fem2dSnapshot::default();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem2dMutation::CreateSupport(create_support::CreateSupport { support: FemSupport { id: "s1".into(), node_id: "ghost".into(), fixed: vec![] } })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_node_missing_target_is_error() {
        let base = Fem2dSnapshot::default();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: "ghost".into() })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_material_missing_target_is_error() {
        let base = simply_supported_beam_doc();
        protocol::os_spr::testkit::assert_missing_target_is_error(
            &base,
            &Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "ghost".into(), new_material: FemMaterial { id: "ghost".into(), name: "x".into(), e: 1.0, nu: 0.3, rho: 1.0 } }),
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn add_load_missing_target_is_error() {
        let base = simply_supported_beam_doc();
        protocol::os_spr::testkit::assert_missing_target_is_error(
            &base,
            &Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "ghost".into(), load: Box::new(FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: FemDof::Ty, value: 1.0 }) }),
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_load_missing_target_is_error() {
        let base = simply_supported_beam_doc();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem2dMutation::RemoveLoad(remove_load::RemoveLoad { case_id: "ghost".into(), load_id: "ghost".into() })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn change_load_case_self_weight_missing_target_is_error() {
        let base = simply_supported_beam_doc();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: "ghost".into(), new_self_weight: true })).await;
    }
    //#endregion 🔖️OutcomeLaws

    //#region 🛡️GuardLaws
    /// 🧾️ The refusal a guard raised, as `(code, level, target)` — every assertion below reads it.
    fn refusal(base: &Fem2dSnapshot, mutation: &Fem2dMutation) -> (String, protocol::Severity, Vec<String>) {
        let outcome = mutation.diff(base);
        assert_eq!(outcome.diff(), &Fem2dDiff::default(), "a refusing diff builder must carry the empty diff, never a half-built delta");
        let messages = outcome.messages();
        assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
        (messages[0].code.0.clone(), messages[0].level, messages[0].target.clone())
    }

    /// 🪪️ A `replace-` may not rename its target — that would orphan every referrer silently.
    #[semio_framework_async_macros::async_test]
    async fn replace_material_rename_is_id_mismatch() {
        let base = simply_supported_beam_doc();
        let renamed = FemMaterial { id: "steel_v2".into(), name: "Steel S355".into(), e: 210e9, nu: 0.3, rho: 7850.0 };
        let (code, level, target) = refusal(&base, &Fem2dMutation::ReplaceMaterial(replace_material::ReplaceMaterial { id: "steel".into(), new_material: renamed }));
        assert_eq!(code, "mutation.id-mismatch");
        assert_eq!(level, protocol::Severity::Fatal, "a rename through a replace is an identity breach no merge policy may absorb");
        assert_eq!(target, vec!["steel".to_string(), "steel_v2".to_string()], "the diagnostic addresses the selected id first and the impostor second");
    }

    /// 🔗️ A `delete-` refuses while referrers exist, and names every one of them.
    #[semio_framework_async_macros::async_test]
    async fn delete_material_still_referenced_is_error() {
        let base = simply_supported_beam_doc();
        let (code, level, target) = refusal(&base, &Fem2dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: "steel".into() }));
        assert_eq!(code, "mutation.target-referenced");
        assert_eq!(level, protocol::Severity::Error, "another base may well have no referrers, so this is an Error, not a Fatal");
        assert_eq!(target, vec!["steel".to_string(), "e1".to_string()], "the target comes first, then every referrer");
    }

    /// 🔗️ The same law through the load lane: a case a combination still weights cannot leave.
    #[semio_framework_async_macros::async_test]
    async fn delete_load_case_still_combined_is_error() {
        let mut base = simply_supported_beam_doc();
        base.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] });
        let (code, _, target) = refusal(&base, &Fem2dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: "dead".into() }));
        assert_eq!(code, "mutation.target-referenced");
        assert_eq!(target, vec!["dead".to_string(), "uls".to_string()]);
    }

    /// 🧱️ A Poisson ratio at or above 0.5 makes the plane constitutive matrix singular.
    #[semio_framework_async_macros::async_test]
    async fn create_material_implausible_poisson_is_fatal() {
        let base = simply_supported_beam_doc();
        let implausible = FemMaterial { id: "rubber".into(), name: "Rubber".into(), e: 1e7, nu: 0.5, rho: 1100.0 };
        let (code, level, target) = refusal(&base, &Fem2dMutation::CreateMaterial(create_material::CreateMaterial { material: implausible }));
        assert_eq!(code, "mutation.invariant");
        assert_eq!(level, protocol::Severity::Fatal, "an inadmissible property is wrong on every base, so no merge policy may absorb it");
        assert_eq!(target, vec!["rubber".to_string()]);
    }

    /// 📏️ A zero area gives the member zero axial stiffness.
    #[semio_framework_async_macros::async_test]
    async fn create_section_zero_area_is_fatal() {
        let base = simply_supported_beam_doc();
        let implausible = FemSection { id: "void".into(), name: "Void".into(), area: 0.0, iy: 1e-5 };
        let (code, level, _) = refusal(&base, &Fem2dMutation::CreateSection(create_section::CreateSection { section: implausible }));
        assert_eq!(code, "mutation.invariant");
        assert_eq!(level, protocol::Severity::Fatal);
    }

    /// 🕳️ A hole must be cut FROM the outline, not float beside it.
    #[semio_framework_async_macros::async_test]
    async fn create_region_hole_outside_outline_is_fatal() {
        let base = rectangle_region_doc();
        let loose = FemRegion {
            id: "r2".into(),
            name: "Loose hole".into(),
            outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]],
            holes: vec![vec![[9.0, 9.0], [10.0, 9.0], [10.0, 10.0]]],
            thickness: 0.02,
            material_id: "steel".into(),
            mesh_size: 0.5,
        };
        let (code, level, target) = refusal(&base, &Fem2dMutation::CreateRegion(create_region::CreateRegion { region: loose }));
        assert_eq!(code, "mutation.invariant");
        assert_eq!(level, protocol::Severity::Fatal);
        assert_eq!(target, vec!["r2".to_string()]);
    }

    /// 📐️ An outline of two points encloses nothing there is anything to mesh.
    #[semio_framework_async_macros::async_test]
    async fn create_region_degenerate_outline_is_fatal() {
        let base = rectangle_region_doc();
        let degenerate = FemRegion { id: "r3".into(), name: "Line".into(), outline: vec![[0.0, 0.0], [4.0, 0.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 0.5 };
        let (code, _, _) = refusal(&base, &Fem2dMutation::CreateRegion(create_region::CreateRegion { region: degenerate }));
        assert_eq!(code, "mutation.invariant");
    }

    /// ⚙️ Asking for zero modes asks the solver for an empty spectrum.
    #[semio_framework_async_macros::async_test]
    async fn update_analysis_settings_zero_modes_is_fatal() {
        let base = simply_supported_beam_doc();
        let settings = FemAnalysisSettings { modal_count: 0, buckling_count: 3, deformation_scale: 50.0 };
        let (code, level, target) = refusal(&base, &Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings }));
        assert_eq!(code, "mutation.invariant");
        assert_eq!(level, protocol::Severity::Fatal);
        assert!(target.is_empty(), "the analysis facet has no id to address, got {target:?}");
    }

    /// 🔗️ `add-load` and `create-load-case` are two doors into the same collection, and now agree.
    #[semio_framework_async_macros::async_test]
    async fn add_load_dangling_node_is_error() {
        let base = simply_supported_beam_doc();
        let load = FemLoad::Nodal { id: "l9".into(), node_id: "ghost".into(), dof: FemDof::Ty, value: -1000.0 };
        let through_add = refusal(&base, &Fem2dMutation::AddLoad(add_load::AddLoad { case_id: "dead".into(), load: Box::new(load.clone()) }));
        let case = FemLoadCase { id: "seismic".into(), name: "Seismic".into(), loads: vec![load], self_weight: false };
        let through_create = refusal(&base, &Fem2dMutation::CreateLoadCase(create_load_case::CreateLoadCase { load_case: case }));
        assert_eq!(through_add, through_create, "the same load on the same missing node must be refused identically through both verbs");
        assert_eq!(through_add.0, "mutation.target-missing");
        assert_eq!(through_add.2, vec!["ghost".to_string()]);
    }

    /// 🔗️ `replace-element` re-resolves all four foreign keys, exactly as `create-element` does.
    #[semio_framework_async_macros::async_test]
    async fn replace_element_dangling_section_is_error() {
        let base = simply_supported_beam_doc();
        let dangling = FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ghost".into() };
        let (code, level, target) = refusal(&base, &Fem2dMutation::ReplaceElement(replace_element::ReplaceElement { id: "e1".into(), new_element: Box::new(dangling) }));
        assert_eq!(code, "mutation.target-missing");
        assert_eq!(level, protocol::Severity::Error);
        assert_eq!(target, vec!["ghost".to_string()]);
    }
    //#endregion 🛡️GuardLaws
}
// #endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `Fem2dMutation` variant, in declaration order — the vocabulary the `fem2d-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `mutate-fem2d-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &[
    "create-node",
    "delete-node",
    "create-element",
    "delete-element",
    "replace-element",
    "create-material",
    "delete-material",
    "replace-material",
    "create-section",
    "delete-section",
    "replace-section",
    "create-support",
    "delete-support",
    "replace-support",
    "create-region",
    "delete-region",
    "replace-region",
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no third-party codec or `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `Fem2dMutation` nor
/// `Fem2dSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn fem2d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<Fem2dSnapshot, String> {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: Fem2dMutation = dsl::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <Fem2dMutation as Mutation<Fem2dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <Fem2dMutation as Mutation<Fem2dSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Fem2dMutation as Mutation<Fem2dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = dsl::DslValue::object([
    ("base".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&base))),
    ("expectedSnapshot".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&expected))),
    ("snapshot".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&applied))),
    ("diff".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(forward.diff()))),
    ("messages".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(forward.messages()))),
    ("inverseSteps".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&inverse))),
    ("inverseSnapshot".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&undone))),
    ("inverseMessages".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&inverse_messages))),
    ]);
    Ok(dsl::json::to_json_string(&report))
}

/// 🔢️ The three planar degrees of freedom this artifact's 2D elements can carry, as the wire spells
/// them. A `Beam` contributes all three at each end, a `Bar` only the two translations, and a node no
/// element touches carries no equation at all — which is why a support on such a node is inert.
const PLANAR_DOFS: [&str; 3] = ["Tx", "Ty", "Rz"];

/// 🗂️ The three projection axes of an analysis report, computed from the document so that two
/// implementations order the same values the same way: every node a `bar`/`beam` element references
/// in document order, every restrained-and-active `(node, dof)` pair in that same order, and the
/// `bar`/`beam` members in document order. Regions are outside the projection — a meshed continuum
/// is not something a frame solver can express, and this subset's region geometry already has its own
/// two third-party oracles.
fn fem2d_frame_axes(doc: &Fem2dSnapshot) -> (Vec<String>, Vec<(String, String)>, Vec<String>) {
    let ends = |element: &crate::artifacts::fem2d::FemElement| match element {
        crate::artifacts::fem2d::FemElement::Bar { start, end, .. } => (start.clone(), end.clone(), false),
        crate::artifacts::fem2d::FemElement::Beam { start, end, .. } => (start.clone(), end.clone(), true),
    };
    let mut active: Vec<(String, [bool; 3])> = Vec::new();
    for element in &doc.elements {
        let (start, end, bends) = ends(element);
        for node in [start, end] {
            match active.iter().position(|(id, _)| id == &node) {
                Some(at) => {
                    active[at].1 = [true, true, active[at].1[2] || bends];
                }
                None => active.push((node, [true, true, bends])),
            }
        }
    }
    let nodes: Vec<String> = doc.nodes.iter().map(|node| node.id.clone()).filter(|id| active.iter().any(|(node, _)| node == id)).collect();
    let mut pairs = Vec::new();
    for node in &nodes {
        let flags = active.iter().find(|(id, _)| id == node).map(|(_, flags)| *flags).unwrap_or([false; 3]);
        for (at, name) in PLANAR_DOFS.iter().enumerate() {
            let held = doc.supports.iter().any(|support| &support.node_id == node && support.fixed.iter().any(|dof| fem2d_dof_name(*dof) == *name));
            if flags[at] && held {
                pairs.push((node.clone(), (*name).to_string()));
            }
        }
    }
    let members: Vec<String> = doc.elements.iter().map(crate::artifacts::fem2d::element_id).map(str::to_string).collect();
    (nodes, pairs, members)
}

/// 🔤️ A document degree of freedom as the wire spells it.
fn fem2d_dof_name(dof: crate::artifacts::fem2d::FemDof) -> &'static str {
    match dof {
        crate::artifacts::fem2d::FemDof::Tx => "Tx",
        crate::artifacts::fem2d::FemDof::Ty => "Ty",
        crate::artifacts::fem2d::FemDof::Tz => "Tz",
        crate::artifacts::fem2d::FemDof::Rx => "Rx",
        crate::artifacts::fem2d::FemDof::Ry => "Ry",
        crate::artifacts::fem2d::FemDof::Rz => "Rz",
    }
}

/// 📤️ One solved case, projected onto the three axes.
fn fem2d_case_value(result: &crate::model::StaticResult, nodes: &[String], pairs: &[(String, String)], members: &[String]) -> dsl::DslValue {
    let displacements = nodes
        .iter()
        .map(|node| {
            let found = result.displacements.iter().find(|entry| &entry.node_id == node);
            let values = found.map(|entry| entry.values).unwrap_or([0.0; 6]);
            dsl::DslValue::Array(vec![dsl::DslValue::float(values[0]), dsl::DslValue::float(values[1]), dsl::DslValue::float(values[5])])
        })
        .collect();
    let reactions = pairs
        .iter()
        .map(|(node, dof)| {
            let value = result.reactions.iter().find(|entry| &entry.node_id == node && fem2d_dof_name_of(entry.dof) == dof.as_str()).map(|entry| entry.value).unwrap_or(0.0);
            dsl::DslValue::float(value)
        })
        .collect();
    let elements = members
        .iter()
        .map(|member| {
            let six = match result.elements.iter().find(|(id, _)| id == member).map(|(_, value)| value) {
                Some(crate::model::ElementResult::Bar { n }) => [*n, 0.0, 0.0, *n, 0.0, 0.0],
                Some(crate::model::ElementResult::Beam { stations }) => match (stations.first(), stations.last()) {
                    (Some(head), Some(tail)) => [head.n, head.v, head.m, tail.n, tail.v, tail.m],
                    _ => [0.0; 6],
                },
                _ => [0.0; 6],
            };
            dsl::DslValue::Array(six.iter().map(|value| dsl::DslValue::float(*value)).collect())
        })
        .collect();
    dsl::DslValue::Object(vec![
        ("displacements".to_string(), dsl::DslValue::Array(displacements)),
        ("reactions".to_string(), dsl::DslValue::Array(reactions)),
        ("elements".to_string(), dsl::DslValue::Array(elements)),
    ])
}

/// 🔤️ A solver degree of freedom as the wire spells it.
fn fem2d_dof_name_of(dof: crate::model::Dof) -> &'static str {
    match dof {
        crate::model::Dof::Tx => "Tx",
        crate::model::Dof::Ty => "Ty",
        crate::model::Dof::Tz => "Tz",
        crate::model::Dof::Rx => "Rx",
        crate::model::Dof::Ry => "Ry",
        crate::model::Dof::Rz => "Rz",
    }
}

/// 🧮️ One JSON report of the linear-static analysis of `snapshot_json` — the production surface the
/// `🧮️solves-fem2d-1-benchmarks` case's third-party solver oracle is compared against.
///
/// Values are SI and in this artifact's own frame: metres, newtons, newton-metres, radians; `x`
/// right and `y` up; `rz` counter-clockwise positive; a reaction is the force the support applies TO
/// the structure; a member's axial force is tension-positive. Cases come first in document order,
/// then combinations. A model whose stiffness matrix is singular reports `error` and no cases at all,
/// which is the only honest answer for a mechanism.
///
/// @see ../../../📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks/🥒️.feature
pub fn fem2d_analysis_report_json(snapshot_json: &str) -> Result<String, String> {
    let doc: Fem2dSnapshot = dsl::json::from_json_str(snapshot_json).map_err(|error| error.to_string())?;
    let (nodes, pairs, members) = fem2d_frame_axes(&doc);
    let axes = |value: dsl::DslValue| {
        dsl::DslValue::Object(vec![
            ("nodes".to_string(), dsl::DslValue::Array(nodes.iter().map(|node| dsl::DslValue::String(node.clone())).collect())),
            ("reactions".to_string(), dsl::DslValue::Array(pairs.iter().map(|(node, dof)| dsl::DslValue::String(format!("{node}.{dof}"))).collect())),
            ("members".to_string(), dsl::DslValue::Array(members.iter().map(|member| dsl::DslValue::String(member.clone())).collect())),
            ("cases".to_string(), value),
        ])
    };
    match crate::fem2d_engine::fem2d_solve_all(&doc) {
        Err(error) => Ok(dsl::json::to_json_string(&dsl::DslValue::Object(vec![("error".to_string(), dsl::DslValue::String(error.to_string()))]))),
        Ok(results) => {
            let mut cases = Vec::new();
            for id in doc.load_cases.iter().map(|case| case.id.clone()).chain(doc.combinations.iter().map(|combination| combination.id.clone())) {
                let Some(result) = results.get(&id) else {
                    return Err(format!("fem2d_solve_all returned no result for {id:?}"));
                };
                let mut entry = vec![("id".to_string(), dsl::DslValue::String(id.clone()))];
                if let dsl::DslValue::Object(fields) = fem2d_case_value(result, &nodes, &pairs, &members) {
                    entry.extend(fields);
                }
                cases.push(dsl::DslValue::Object(entry));
            }
            Ok(dsl::json::to_json_string(&axes(dsl::DslValue::Array(cases))))
        }
    }
}

/// 🧬️ Applies one typed mutation to `base_json` through the SAME production diff/apply path
/// `fem2d_mutation_report_json` uses, then reports the analysis of what it left behind — the
/// mutate-then-solve half of `🧮️solves-fem2d-1-benchmarks`.
pub fn fem2d_mutated_analysis_report_json(base_json: &str, mutation_json: &str) -> Result<String, String> {
    let base: Fem2dSnapshot = dsl::json::from_json_str(base_json).map_err(|error| error.to_string())?;
    let mutation: Fem2dMutation = dsl::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let outcome = <Fem2dMutation as Mutation<Fem2dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let faults: Vec<String> = outcome.messages().iter().filter(|message| matches!(message.level, dsl::Severity::Error | dsl::Severity::Fatal)).map(|message| format!("{:?}", message.code)).collect();
    if !faults.is_empty() {
        return Err(format!("the mutation was rejected with {faults:?}"));
    }
    fem2d_analysis_report_json(&dsl::json::to_json_string(&applied))
}

/// 🎵️ One JSON report of the modal analysis of `snapshot_json` — the natural frequencies in hertz,
/// as many as the document's own `analysis.modalCount` asks for.
pub fn fem2d_modal_report_json(snapshot_json: &str) -> Result<String, String> {
    let doc: Fem2dSnapshot = dsl::json::from_json_str(snapshot_json).map_err(|error| error.to_string())?;
    match crate::fem2d_engine::modal_buckling::fem2d_modal(&doc) {
        Err(error) => Ok(dsl::json::to_json_string(&dsl::DslValue::Object(vec![("error".to_string(), dsl::DslValue::String(error.to_string()))]))),
        Ok(result) => Ok(dsl::json::to_json_string(&dsl::DslValue::Object(vec![(
            "frequenciesHz".to_string(),
            dsl::DslValue::Array(result.frequencies_hz.iter().take(doc.analysis.modal_count as usize).map(|value| dsl::DslValue::float(*value)).collect()),
        )]))),
    }
}

/// 🏛️ One JSON report of the linear-buckling analysis of `snapshot_json` — the lowest load factor of
/// every load case the document declares, keyed by case id.
pub fn fem2d_buckling_report_json(snapshot_json: &str) -> Result<String, String> {
    let doc: Fem2dSnapshot = dsl::json::from_json_str(snapshot_json).map_err(|error| error.to_string())?;
    let mut factors = Vec::new();
    for case in &doc.load_cases {
        match crate::fem2d_engine::modal_buckling::fem2d_buckling(&doc, &case.id) {
            Err(error) => return Ok(dsl::json::to_json_string(&dsl::DslValue::Object(vec![("error".to_string(), dsl::DslValue::String(error.to_string()))]))),
            Ok(result) => match result.factors.first() {
                Some(value) => factors.push((case.id.clone(), dsl::DslValue::float(*value))),
                None => return Ok(dsl::json::to_json_string(&dsl::DslValue::Object(vec![("error".to_string(), dsl::DslValue::String(format!("no buckling factor for {}", case.id)))]))),
            },
        }
    }
    Ok(dsl::json::to_json_string(&dsl::DslValue::Object(vec![("factors".to_string(), dsl::DslValue::Object(factors))])))
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
mod kinds_conformance {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of them must appear in the committed oracle
    /// manifest's catalog. The framework never parses Rust, so this is what keeps the declaration
    /// honest in both directions at once.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🔮️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsConformance
