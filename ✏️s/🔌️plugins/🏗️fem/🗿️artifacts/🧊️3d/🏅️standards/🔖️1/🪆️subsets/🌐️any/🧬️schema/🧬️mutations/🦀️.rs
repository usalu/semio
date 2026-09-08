//! ⚡️ FEM 3D artifact — semantic mutation dispatch enum + laws (constitutional: op). Every variant
//! wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Fem3dSnapshot, Fem3dMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.

use crate::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use crate::Fem3dSnapshot;
use crate::{element_id, load_id, FemAnalysisSettings, FemElement, FemLoad, FemMaterial, FemNode, FemSection, FemSolid};
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the fem3d document, derived per
/// `📓️derivation-rules.md` from `Fem3dSnapshot`'s shape (8 id-keyed collections + one inseparable
/// analysis-settings facet). Every generic `Set*`/`Remove*` variant this facet used to carry —
/// including the banned `SetSnapshot` whole-document-replace variant — is gone; whole-document
/// replace is not an in-history mutation at all (routed through `ArtifactStore::reset` /
/// `Effect::LoadDocument`, see `Fem3dPlayApp::whole_document_operation` returning `None` now and
/// `editor::fem3d::reset_document_effect`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Fem3dSnapshot, diff = Fem3dDiff, schema = "fem.fem3d")]
pub enum Fem3dMutation {
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
    CreateSolid(create_solid::CreateSolid),
    DeleteSolid(delete_solid::DeleteSolid),
    ReplaceSolid(replace_solid::ReplaceSolid),
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
use super::create_section;
use super::create_solid;
use super::create_support;
use super::delete_combination;
use super::delete_element;
use super::delete_load_case;
use super::delete_material;
use super::delete_node;
use super::delete_section;
use super::delete_solid;
use super::delete_support;
use super::remove_load;
use super::replace_element;
use super::replace_material;
use super::replace_section;
use super::replace_solid;
use super::replace_support;
use super::update_analysis_settings;
//#endregion 🔖️LeafImports

//#region 🛡️Guards
/// 🚫️ The `mutation.target-referenced` refusal — an `Error`, the same level and empty-diff shape
/// `mutation.target-missing` carries, addressed at the target FOLLOWED BY every referrer that keeps
/// it alive so a caller can offer to release them. Sibling of `🔋️energy`'s `mutation.target-in-use`
/// fault, raised here as a mutation message because fem3d refuses inside the diff builder.
pub fn target_referenced(label: &str, id: &str, referrers: Vec<String>) -> protocol::MutationOutcome<Fem3dDiff> {
    let listed = referrers.iter().map(|referrer| format!("\"{referrer}\"")).collect::<Vec<_>>().join(", ");
    let mut target = vec![id.to_string()];
    target.extend(referrers);
    protocol::MutationOutcome::error("mutation.target-referenced", format!("{label} \"{id}\" is still referenced by {listed}."), target)
}

/// 🪪️ The `mutation.id-mismatch` refusal — a `replace-` selects its target by `id` and carries a
/// whole new record; a new record under a DIFFERENT id would silently rename the row and orphan
/// every reference to it, so it is a `Fatal` identity breach, the level `mutation.duplicate-id`
/// already uses for the other half of the identity contract.
pub fn id_mismatch(label: &str, id: &str, new_id: &str) -> protocol::MutationOutcome<Fem3dDiff> {
    protocol::MutationOutcome::fatal("mutation.id-mismatch", format!("{label} \"{id}\" cannot be renamed to \"{new_id}\" by a replace."), [id.to_string(), new_id.to_string()])
}

/// 🧨️ The `mutation.invariant` refusal — the repo-wide code for a value or geometry breach
/// (`🔱️trinity`'s `create-edge`/`move-node`, `📸️remodel`'s `update-geo-params`), `Fatal` there and
/// `Fatal` here: a document that took the value would not be solvable at all.
pub fn invariant(message: String, target: Vec<String>) -> protocol::MutationOutcome<Fem3dDiff> {
    protocol::MutationOutcome::fatal("mutation.invariant", message, target)
}

/// 🔗️ Every record id that would dangle if `id` left `nodes` — elements, then supports, then nodal
/// loads, each in document order.
pub fn node_referrers(base: &Fem3dSnapshot, id: &str) -> Vec<String> {
    let mut found: Vec<String> = base
        .elements
        .iter()
        .filter(|element| match element {
            FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => start == id || end == id,
        })
        .map(|element| element_id(element).to_string())
        .collect();
    found.extend(base.supports.iter().filter(|support| support.node_id == id).map(|support| support.id.clone()));
    found.extend(base.load_cases.iter().flat_map(|case| case.loads.iter()).filter(|load| matches!(load, FemLoad::Nodal { node_id, .. } if node_id == id)).map(|load| load_id(load).to_string()));
    found
}

/// 🔗️ Every member UDL that would dangle if `id` left `elements`.
pub fn element_referrers(base: &Fem3dSnapshot, id: &str) -> Vec<String> {
    base.load_cases.iter().flat_map(|case| case.loads.iter()).filter(|load| matches!(load, FemLoad::MemberUdl { element_id, .. } if element_id == id)).map(|load| load_id(load).to_string()).collect()
}

/// 🔗️ Every element that would dangle if `id` left `sections`.
pub fn section_referrers(base: &Fem3dSnapshot, id: &str) -> Vec<String> {
    base.elements
        .iter()
        .filter(|element| match element {
            FemElement::Bar { section_id, .. } | FemElement::Frame { section_id, .. } => section_id == id,
        })
        .map(|element| element_id(element).to_string())
        .collect()
}

/// 🔗️ Every area load that would dangle if `id` left `solids`.
pub fn solid_referrers(base: &Fem3dSnapshot, id: &str) -> Vec<String> {
    base.load_cases.iter().flat_map(|case| case.loads.iter()).filter(|load| matches!(load, FemLoad::Area { solid_id, .. } if solid_id == id)).map(|load| load_id(load).to_string()).collect()
}

/// 🔗️ Every element and solid that would dangle if `id` left `materials`.
pub fn material_referrers(base: &Fem3dSnapshot, id: &str) -> Vec<String> {
    let mut found: Vec<String> = base
        .elements
        .iter()
        .filter(|element| match element {
            FemElement::Bar { material_id, .. } | FemElement::Frame { material_id, .. } => material_id == id,
        })
        .map(|element| element_id(element).to_string())
        .collect();
    found.extend(base.solids.iter().filter(|solid| solid.material_id == id).map(|solid| solid.id.clone()));
    found
}

/// 🔗️ Every combination that would lose a weighted term if `id` left `load_cases`.
pub fn load_case_referrers(base: &Fem3dSnapshot, id: &str) -> Vec<String> {
    base.combinations.iter().filter(|combination| combination.terms.contains_key(id)).map(|combination| combination.id.clone()).collect()
}

/// 🔩️ Resolves an element's four foreign keys in the declared order — start, end, material,
/// section — so `create-element` and `replace-element` accept exactly the same references.
pub fn resolve_element(base: &Fem3dSnapshot, element: &FemElement) -> Option<protocol::MutationOutcome<Fem3dDiff>> {
    let (start, end, material_id, section_id) = match element {
        FemElement::Bar { start, end, material_id, section_id, .. } | FemElement::Frame { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
    };
    if !base.nodes.iter().any(|node| &node.id == start) {
        return Some(protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{start}\" does not exist."), [start.clone()]));
    }
    if !base.nodes.iter().any(|node| &node.id == end) {
        return Some(protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{end}\" does not exist."), [end.clone()]));
    }
    if !base.materials.iter().any(|material| &material.id == material_id) {
        return Some(protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{material_id}\" does not exist."), [material_id.clone()]));
    }
    if !base.sections.iter().any(|section| &section.id == section_id) {
        return Some(protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{section_id}\" does not exist."), [section_id.clone()]));
    }
    None
}

/// 🏋️ Resolves the node, element or solid a load hangs on, so `add-load` accepts exactly the
/// references `create-load-case` accepts for the same payload.
pub fn resolve_load(base: &Fem3dSnapshot, load: &FemLoad) -> Option<protocol::MutationOutcome<Fem3dDiff>> {
    let missing = match load {
        FemLoad::Nodal { node_id, .. } => (!base.nodes.iter().any(|node| &node.id == node_id)).then(|| ("Node", node_id.clone())),
        FemLoad::MemberUdl { element_id: id, .. } => (!base.elements.iter().any(|element| element_id(element) == id)).then(|| ("Element", id.clone())),
        FemLoad::Area { solid_id, .. } => (!base.solids.iter().any(|solid| &solid.id == solid_id)).then(|| ("Solid", solid_id.clone())),
    };
    missing.map(|(label, id)| protocol::MutationOutcome::error("mutation.target-missing", format!("{label} \"{id}\" does not exist."), [id]))
}

/// 📍️ A node must sit somewhere the solver can assemble — three finite coordinates.
pub fn node_breach(node: &FemNode) -> Option<String> {
    (!(node.x.is_finite() && node.y.is_finite() && node.z.is_finite())).then(|| format!("Node \"{}\" must sit at a finite position, got ({}, {}, {}).", node.id, node.x, node.y, node.z))
}

/// 🧱️ Linear-elastic isotropy: positive moduli and density, and a Poisson ratio inside the
/// thermodynamically admissible open interval (-1, 0.5) — 0.5 itself is incompressible and makes
/// the `Tet4` constitutive matrix singular.
pub fn material_breach(material: &FemMaterial) -> Option<String> {
    if !(material.e.is_finite() && material.g.is_finite() && material.nu.is_finite() && material.rho.is_finite()) {
        return Some(format!("Material \"{}\" must carry finite properties.", material.id));
    }
    if material.e <= 0.0 || material.g <= 0.0 || material.rho <= 0.0 {
        return Some(format!("Material \"{}\" must carry a positive e, g and rho, got e={}, g={}, rho={}.", material.id, material.e, material.g, material.rho));
    }
    (!(-1.0 < material.nu && material.nu < 0.5)).then(|| format!("Material \"{}\" must carry a Poisson ratio in (-1, 0.5), got {}.", material.id, material.nu))
}

/// 📐️ A cross-section with a non-positive area or second moment cannot carry axial, bending or
/// torsional stiffness at all — the element stiffness matrix would be singular.
pub fn section_breach(section: &FemSection) -> Option<String> {
    if !(section.area.is_finite() && section.iy.is_finite() && section.iz.is_finite() && section.j.is_finite()) {
        return Some(format!("Section \"{}\" must carry finite properties.", section.id));
    }
    (section.area <= 0.0 || section.iy <= 0.0 || section.iz <= 0.0 || section.j <= 0.0).then(|| format!("Section \"{}\" must carry a positive area, iy, iz and j, got area={}, iy={}, iz={}, j={}.", section.id, section.area, section.iy, section.iz, section.j))
}

/// 📏️ Twice the signed area of a closed ring (the shoelace sum, halved) — negative when the ring
/// winds clockwise, which `resolve_geometry` accepts either way.
fn ring_area(ring: &[[f64; 2]]) -> f64 {
    (0..ring.len()).map(|at| ring[at][0] * ring[(at + 1) % ring.len()][1] - ring[(at + 1) % ring.len()][0] * ring[at][1]).sum::<f64>() / 2.0
}

/// 🎯️ Crossing-count containment. A point exactly ON an edge is left undefined, as the classical
/// ray cast leaves it; every committed hole sits strictly inside its outline.
fn ring_contains(ring: &[[f64; 2]], point: [f64; 2]) -> bool {
    let mut inside = false;
    for at in 0..ring.len() {
        let (corner, other) = (ring[at], ring[(at + 1) % ring.len()]);
        if (corner[1] > point[1]) != (other[1] > point[1]) && point[0] < corner[0] + (point[1] - corner[1]) / (other[1] - corner[1]) * (other[0] - corner[0]) {
            inside = !inside;
        }
    }
    inside
}

/// 🧊️ The footprint `crate::fem3d_engine::meshing::resolve_geometry` has to be able to extrude and
/// tet-split: a closed non-degenerate ring, a positive extrusion through at least one layer, a
/// positive mesh size, and every hole strictly inside the outline. Refused HERE rather than at
/// solve time so a document never reaches the mesher already unmeshable.
pub fn solid_breach(solid: &FemSolid) -> Option<String> {
    if solid.outline.len() < 3 {
        return Some(format!("Solid \"{}\" needs at least three outline points, got {}.", solid.id, solid.outline.len()));
    }
    if !solid.outline.iter().all(|point| point[0].is_finite() && point[1].is_finite()) {
        return Some(format!("Solid \"{}\" must carry a finite outline.", solid.id));
    }
    if ring_area(&solid.outline).abs() <= 0.0 {
        return Some(format!("Solid \"{}\" has a degenerate outline of zero area.", solid.id));
    }
    if !(solid.base_z.is_finite() && solid.height.is_finite() && solid.mesh_size.is_finite()) {
        return Some(format!("Solid \"{}\" must carry a finite baseZ, height and meshSize.", solid.id));
    }
    if solid.height <= 0.0 {
        return Some(format!("Solid \"{}\" must be extruded by a positive height, got {}.", solid.id, solid.height));
    }
    if solid.layers < 1 {
        return Some(format!("Solid \"{}\" must be meshed through at least one layer, got {}.", solid.id, solid.layers));
    }
    if solid.mesh_size <= 0.0 {
        return Some(format!("Solid \"{}\" must carry a positive mesh size, got {}.", solid.id, solid.mesh_size));
    }
    for hole in &solid.holes {
        if hole.len() < 3 || ring_area(hole).abs() <= 0.0 {
            return Some(format!("Solid \"{}\" carries a degenerate hole.", solid.id));
        }
        if !hole.iter().all(|point| point[0].is_finite() && point[1].is_finite() && ring_contains(&solid.outline, *point)) {
            return Some(format!("Solid \"{}\" carries a hole that leaves its outline.", solid.id));
        }
    }
    None
}

/// ⚙️ Analysis bounds: an eigen solve for fewer than one mode or one buckling factor has nothing to
/// return, and a non-positive deformation scale collapses or mirrors the results view.
pub fn analysis_breach(settings: &FemAnalysisSettings) -> Option<String> {
    if settings.modal_count < 1 || settings.buckling_count < 1 {
        return Some(format!("Analysis settings need at least one modal and one buckling factor, got {} and {}.", settings.modal_count, settings.buckling_count));
    }
    (!(settings.deformation_scale.is_finite() && settings.deformation_scale > 0.0)).then(|| format!("Analysis settings need a finite positive deformation scale, got {}.", settings.deformation_scale))
}
//#endregion 🛡️Guards

pub type Fem3dEnvelope = ArtifactEnvelope<Fem3dSnapshot, Fem3dMutation>;
pub type Fem3dStore = ArtifactStore<Fem3dSnapshot, Fem3dMutation>;

//#region 🔖️GenericDelegates
/// 🌉️ Thin delegates to the derive-generated `protocol::Mutation` impl — kept because
/// `🏗️builder/🦀️.rs` (an artifact-generic caller, no per-variant knowledge) and
/// `📝️text/🦀️.rs`'s re-export both call these by name.
pub fn apply_fem3d_mutation(snapshot: &mut Fem3dSnapshot, mutation: &Fem3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(snapshot, mutation)?;

    *snapshot = next;
    Ok(())
}

pub fn inverse_fem3d_mutation(snapshot: &Fem3dSnapshot, mutation: &Fem3dMutation) -> Vec<Fem3dMutation> {
    mutation.inverse(snapshot)
}
//#endregion 🔖️GenericDelegates

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `Fem3dMutation` variant, in declaration order — the vocabulary the `fem3d-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `mutate-fem3d-1` exhaustive test case measures
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
    "create-solid",
    "delete-solid",
    "replace-solid",
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
/// `protocol`/`store` extern-crate aliases are private — so neither `Fem3dMutation` nor
/// `Fem3dSnapshot` can be named there, and hand-transcribing either into a Rust literal
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
pub fn fem3d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<Fem3dSnapshot, String> {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: Fem3dMutation = dsl::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <Fem3dMutation as Mutation<Fem3dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <Fem3dMutation as Mutation<Fem3dSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Fem3dMutation as Mutation<Fem3dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
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
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
