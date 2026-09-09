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
    use crate::standards::v1::subsets::any::schema::diff::Fem2dDiff;
    use crate::{element_id, load_id, Fem2dSnapshot, FemAnalysisSettings, FemElement, FemLoad, FemMaterial, FemNode, FemRegion, FemSection};

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
            protocol::MutationOutcome::fatal(
                "mutation.id-mismatch",
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

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

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
    let ends = |element: &crate::FemElement| match element {
        crate::FemElement::Bar { start, end, .. } => (start.clone(), end.clone(), false),
        crate::FemElement::Beam { start, end, .. } => (start.clone(), end.clone(), true),
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
        let flags = active.iter().find(|(id, _)| id == node).map_or([false; 3], |(_, flags)| *flags);
        for (at, name) in PLANAR_DOFS.iter().enumerate() {
            let held = doc.supports.iter().any(|support| &support.node_id == node && support.fixed.iter().any(|dof| fem2d_dof_name(*dof) == *name));
            if flags[at] && held {
                pairs.push((node.clone(), (*name).to_string()));
            }
        }
    }
    let members: Vec<String> = doc.elements.iter().map(crate::element_id).map(str::to_string).collect();
    (nodes, pairs, members)
}

/// 🔤️ A document degree of freedom as the wire spells it.
fn fem2d_dof_name(dof: crate::FemDof) -> &'static str {
    match dof {
        crate::FemDof::Tx => "Tx",
        crate::FemDof::Ty => "Ty",
        crate::FemDof::Tz => "Tz",
        crate::FemDof::Rx => "Rx",
        crate::FemDof::Ry => "Ry",
        crate::FemDof::Rz => "Rz",
    }
}

/// 📤️ One solved case, projected onto the three axes.
fn fem2d_case_value(result: &crate::model::StaticResult, nodes: &[String], pairs: &[(String, String)], members: &[String]) -> dsl::DslValue {
    let displacements = nodes
        .iter()
        .map(|node| {
            let found = result.displacements.iter().find(|entry| &entry.node_id == node);
            let values = found.map_or([0.0; 6], |entry| entry.values);
            dsl::DslValue::Array(vec![dsl::DslValue::float(values[0]), dsl::DslValue::float(values[1]), dsl::DslValue::float(values[5])])
        })
        .collect();
    let reactions = pairs
        .iter()
        .map(|(node, dof)| {
            let value = result.reactions.iter().find(|entry| &entry.node_id == node && fem2d_dof_name_of(entry.dof) == dof.as_str()).map_or(0.0, |entry| entry.value);
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
    dsl::DslValue::Object(vec![("displacements".to_string(), dsl::DslValue::Array(displacements)), ("reactions".to_string(), dsl::DslValue::Array(reactions)), ("elements".to_string(), dsl::DslValue::Array(elements))])
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
        Ok(result) => {
            Ok(dsl::json::to_json_string(&dsl::DslValue::Object(vec![("frequenciesHz".to_string(), dsl::DslValue::Array(result.frequencies_hz.iter().take(doc.analysis.modal_count as usize).map(|value| dsl::DslValue::float(*value)).collect()))])))
        }
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
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
