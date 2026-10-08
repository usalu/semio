//! ⚡️ FEM 2D artifact — semantic mutation dispatch enum + laws (constitutional: op). Every variant
//! wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Fem2dSnapshot, Fem2dMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.

use crate::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use crate::Fem2dSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the fem2d document, derived per
/// `📓️derivation-rules.md` from `Fem2dSnapshot`'s shape (8 id-keyed collections + one inseparable
/// analysis-settings facet). Every generic `Set*`/`Remove*` variant this facet used to carry —
/// including the banned `SetSnapshot` whole-document-replace variant — is gone; whole-document
/// replace is not an in-history mutation at all (routed through `Effect::LoadDocument`, see
/// `editor::fem2d::reset_document_effect`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
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
    ReplaceNode(replace_node::ReplaceNode),
    ReplaceLoad(replace_load::ReplaceLoad),
    ChangeLoadCaseName(change_load_case_name::ChangeLoadCaseName),
    ReplaceCombination(replace_combination::ReplaceCombination),
    MoveSelection(move_selection::MoveSelection),
}
//#endregion 🔖️Mutations

//#region 🔖️LeafImports
/// 🌉️ Brings every triad leaf's `mutation` submodule into this file's own scope (declared as
/// siblings back in `🦀️.rs`, not inside this file) — required for the dispatch enum's bare
/// `create_node::CreateNode`-style variant field paths above to resolve.
use super::add_load;
use super::change_load_case_name;
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
use super::replace_combination;
use super::replace_element;
use super::replace_load;
use super::replace_material;
use super::replace_node;
use super::move_selection;
use super::replace_region;
use super::replace_section;
use super::replace_support;
use super::update_analysis_settings;
//#endregion 🔖️LeafImports

pub type Fem2dEnvelope = ArtifactEnvelope<Fem2dSnapshot, Fem2dMutation>;
pub type Fem2dStore = ArtifactStore<Fem2dSnapshot, Fem2dMutation>;

//#region 🔖️GenericDelegates
pub fn inverse_fem2d_mutation(snapshot: &Fem2dSnapshot, mutation: &Fem2dMutation) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(snapshot)?

    })
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
/// 🚦️ Level discipline. `mutation.duplicate-id` and `mutation.invariant` are `Fatal`: they say the PAYLOAD is
/// wrong, so no merge policy may absorb them and no later base can make them right. `mutation.target-missing`,
/// `mutation.target-referenced` and `mutation.target-mismatch` (a `replace-` whose record renames the target it
/// selects) are `Error`: they say this BASE cannot host the payload as it stands.
///
/// @see 📓️w13-fem2d-semantics.md — the per-kind rule table these guards implement.
pub mod guards {
    use crate::standards::v1::subsets::any::schema::diff::Fem2dDiff;
    use crate::{element_id, load_id, Fem2dSnapshot, FemAnalysisSettings, FemCombination, FemElement, FemLoad, FemMaterial, FemNode, FemRegion, FemSection};

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
        (target != replacement).then(|| {
            protocol::MutationOutcome::error(
                "mutation.target-mismatch",
                format!("A replace-{noun} selects \"{target}\" but carries a record identified \"{replacement}\"; a replacement may not rename its target."),
                [target.to_string(), replacement.to_string()],
            )
        })
    }

    /// 🔗️ A `delete-` may not orphan a live reference. The diagnostic addresses the target FIRST
    /// and then every referrer, so a caller can offer to re-point or remove them.
    pub fn referenced(noun: &str, blocker: &str, target: &str, referrers: Vec<String>) -> Option<Rejection> {
        (!referrers.is_empty()).then(|| {
            let count = referrers.len();
            let listed = referrers.join(", ");
            let mut address = vec![target.to_string()];
            address.extend(referrers);
            protocol::MutationOutcome::error("mutation.target-referenced", format!("{noun} \"{target}\" is still referenced by {count} {blocker}(s): {listed}."), address)
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
        base.load_cases.iter().flat_map(|case| case.loads.iter().filter(move |load| matches!(load, FemLoad::MemberUdl { element_id: referenced, .. } if referenced == target)).map(move |load| format!("{}/{}", case.id, load_id(load)))).collect()
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
        base.load_cases.iter().flat_map(|case| case.loads.iter().filter(move |load| matches!(load, FemLoad::Area { region_id, .. } if region_id == target)).map(move |load| format!("{}/{}", case.id, load_id(load)))).collect()
    }

    /// 🔎️ Every combination weighting this load case.
    pub fn load_case_referrers(base: &Fem2dSnapshot, target: &str) -> Vec<String> {
        base.combinations.iter().filter(|combination| combination.terms.iter().any(|term| term.case_id == target)).map(|combination| combination.id.clone()).collect()
    }

    /// 🔗️ Every term of a combination resolves to a load case or to ANOTHER combination — the same
    /// resolution whether the combination is brand new (`create-combination`) or replaces an
    /// existing one (`replace-combination`). `id` is the identity the payload is being stored
    /// under, which `replace-` selects and `create-` carries.
    ///
    /// 🪞️ ANOTHER, never itself. A term citing `id` is refused as `mutation.invariant` (Fatal) and
    /// not as `mutation.target-missing` (Error), because the cycle is a property of the PAYLOAD: no
    /// base can host a combination that superposes itself, so no later base may absorb it. Reading
    /// the self-term as a plain reference is exactly the gap this closes — under `replace-` the
    /// target DOES exist in `base.combinations`, so the resolution below would have accepted it,
    /// and `create-` only refused it by accident, as an id its own base did not carry yet. It is
    /// the same self-exclusion `combination_referrers` applies when it asks who blocks a delete.
    pub fn combination_term_references(base: &Fem2dSnapshot, id: &str, combination: &FemCombination) -> Option<Rejection> {
        combination.terms.iter().find_map(|term| {
            if term.case_id == id {
                return Some(invariant([id.to_string()], format!("Combination \"{id}\" may not weight itself.")));
            }
            (!base.load_cases.iter().any(|case| case.id == term.case_id) && !base.combinations.iter().any(|other| other.id == term.case_id)).then(|| missing(term.case_id.clone(), format!("Load case or combination \"{}\" does not exist.", term.case_id)))
        })
    }

    /// ⚖️ A combination weights its cases by real numbers — a non-finite factor turns the superposed
    /// load vector into NaN long before anything downstream can say which edit caused it.
    pub fn combination_factors(combination: &FemCombination) -> Option<Rejection> {
        combination.terms.iter().find(|term| !term.factor.is_finite()).map(|term| invariant([combination.id.clone()], format!("Combination \"{}\" carries a non-finite factor on case \"{}\".", combination.id, term.case_id)))
    }

    /// 🏋️ A load's magnitudes are real numbers — a non-finite force, line load or pressure poisons
    /// every right-hand side it enters, exactly as a non-finite coordinate poisons the stiffness.
    pub fn load_magnitudes(load: &FemLoad) -> Option<Rejection> {
        let (finite, magnitude) = match load {
            FemLoad::Nodal { value, .. } => (value.is_finite(), *value),
            FemLoad::MemberUdl { wx, wy, .. } => (wx.is_finite() && wy.is_finite(), *wx),
            FemLoad::Area { pressure, .. } => (pressure.is_finite(), *pressure),
        };
        (!finite).then(|| invariant([load_id(load).to_string()], format!("Load \"{}\" carries a non-finite magnitude ({magnitude}).", load_id(load))))
    }

    /// 🔎️ Every OTHER combination nesting this one (a self-term cannot block its own deletion).
    pub fn combination_referrers(base: &Fem2dSnapshot, target: &str) -> Vec<String> {
        base.combinations.iter().filter(|combination| combination.id != target && combination.terms.iter().any(|term| term.case_id == target)).map(|combination| combination.id.clone()).collect()
    }
}
//#endregion 🛡️Guards

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests


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
    "replace-node",
    "replace-load",
    "change-load-case-name",
    "replace-combination",
    "move-selection",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge


/// 🔢️ The three planar degrees of freedom this artifact's 2D elements can carry, as the wire spells
/// them. A `Beam` contributes all three at each end, a `Bar` only the two translations, and a node no
/// element touches carries no equation at all — which is why a support on such a node is inert.
pub(crate) const PLANAR_DOFS: [&str; 3] = ["Tx", "Ty", "Rz"];
















//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
