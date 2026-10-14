//! 🛡️ The payload validations a noun's `create-`/`replace-` twins share, the foreign-key
//! resolutions `create-load-case` and `add-load` share, and the referential-integrity lookups every
//! guarded `delete-` runs — written ONCE, here, below every kind that runs them.
//!
//! Why here and not in each leaf: the twins are only meaningful if they agree, and a transcription
//! per leaf is exactly how they drifted apart (a `create-element` that resolved four references
//! next to a `replace-element` that resolved none). Each leaf now spends one line per rule.
//!
//! 🚦️ Level discipline. `mutation.duplicate-id` and `mutation.invariant` are `Fatal`: they say the PAYLOAD is
//! wrong, so no merge policy may absorb them and no later base can make them right. `mutation.target-missing`,
//! `mutation.target-referenced` and `mutation.target-mismatch` (a `replace-` whose record renames the target it
//! selects) are `Error`: they say this BASE cannot host the payload as it stands.
//!
//! @see 📓️w13-fem2d-semantics.md — the per-kind rule table these guards implement.

use crate::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use crate::standards::v1::subsets::any::schema::structure::{element_id, load_id, FemAnalysisSettings, FemCombination, FemElement, FemLoad, FemMaterial, FemNode, FemRegion, FemSection};
use crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

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
