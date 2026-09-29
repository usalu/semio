"""🏗️ fem (P2-X): a replacement that renames its target and a delete that would orphan a live reference both break a
document rule → `Invariant` (Fatal / Error, levels kept), addressed at the target then its referrers as before. The guard
helpers lose the text they only ever fed the dropped message (their noun/label/blocker/message parameters; fem3d's
`*_breach` checks answer `bool` instead of a sentence), and every caller follows."""
import re

from rsargs import drop_argument

F2 = "🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/"
F3 = "🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/"
G2 = F2 + "🌐️any/🧬️schema/🧬️mutations/🦀️.rs"
G3 = F3 + "🌐️any/🧬️schema/🧬️mutations/🦀️.rs"

OVERRIDES = {(G2, 153): {"skip": True}, (G2, 169): {"skip": True}, (G3, 109): {"skip": True}, (G3, 117): {"skip": True}}

EDITS = [
    (G2, """/// 🚦️ Level discipline. `mutation.duplicate-id`, `mutation.id-mismatch` and `mutation.invariant`
/// are `Fatal`: they say the PAYLOAD is wrong, so no merge policy may absorb them and no later base
/// can make them right. `mutation.target-missing` and `mutation.target-referenced` are `Error`:
/// they say this BASE cannot host the payload, which a different base may well be able to.""",
     """/// 🚦️ Level discipline. `mutation.duplicate-id` and the `mutation.invariant` of a malformed value or a
/// renaming replacement are `Fatal`: they say the PAYLOAD is wrong, so no merge policy may absorb them
/// and no later base can make them right. `mutation.target-missing` and the `mutation.invariant` of a
/// delete that would orphan a reference are `Error`: they say this BASE cannot host the payload, which
/// a different base may well be able to."""),
    (G2, """    fn invariant(target: impl IntoIterator<Item = String>, message: String) -> Rejection {""",
     """    fn invariant(target: impl IntoIterator<Item = String>) -> Rejection {"""),
    (G2, """    fn missing(target: String, message: String) -> Rejection {""", """    fn missing(target: String) -> Rejection {"""),
    (G2, """    pub fn identity_matches(noun: &str, target: &str, replacement: &str) -> Option<Rejection> {
        (target != replacement).then(|| {
            protocol::MutationOutcome::fatal(
                "mutation.id-mismatch",
                format!("A replace-{noun} selects \\"{target}\\" but carries a record identified \\"{replacement}\\"; a replacement may not rename its target."),
                [target.to_string(), replacement.to_string()],
            )
        })
    }""",
     """    pub fn identity_matches(target: &str, replacement: &str) -> Option<Rejection> {
        (target != replacement).then(|| protocol::MutationOutcome::fatal(protocol::MutationCode::Invariant, [target.to_string(), replacement.to_string()]))
    }"""),
    (G2, """    pub fn referenced(noun: &str, blocker: &str, target: &str, referrers: Vec<String>) -> Option<Rejection> {
        (!referrers.is_empty()).then(|| {
            let count = referrers.len();
            let listed = referrers.join(", ");
            let mut address = vec![target.to_string()];
            address.extend(referrers);
            protocol::MutationOutcome::error("mutation.target-referenced", format!("{noun} \\"{target}\\" is still referenced by {count} {blocker}(s): {listed}."), address)
        })
    }""",
     """    pub fn referenced(target: &str, referrers: Vec<String>) -> Option<Rejection> {
        (!referrers.is_empty()).then(|| {
            let mut address = vec![target.to_string()];
            address.extend(referrers);
            protocol::MutationOutcome::error(protocol::MutationCode::Invariant, address)
        })
    }"""),
    (G3, """/// 🚫️ The `mutation.target-referenced` refusal — an `Error`, the same level and empty-diff shape
/// `mutation.target-missing` carries, addressed at the target FOLLOWED BY every referrer that keeps
/// it alive so a caller can offer to release them. Sibling of `🔋️energy`'s `mutation.target-in-use`
/// fault, raised here as a mutation message because fem3d refuses inside the diff builder.
pub fn target_referenced(label: &str, id: &str, referrers: Vec<String>) -> protocol::MutationOutcome<Fem3dDiff> {
    let listed = referrers.iter().map(|referrer| format!("\\"{referrer}\\"")).collect::<Vec<_>>().join(", ");
    let mut target = vec![id.to_string()];
    target.extend(referrers);
    protocol::MutationOutcome::error("mutation.target-referenced", format!("{label} \\"{id}\\" is still referenced by {listed}."), target)
}

/// 🪪️ The `mutation.id-mismatch` refusal — a `replace-` selects its target by `id` and carries a
/// whole new record; a new record under a DIFFERENT id would silently rename the row and orphan
/// every reference to it, so it is a `Fatal` identity breach, the level `mutation.duplicate-id`
/// already uses for the other half of the identity contract.
pub fn id_mismatch(label: &str, id: &str, new_id: &str) -> protocol::MutationOutcome<Fem3dDiff> {
    protocol::MutationOutcome::fatal("mutation.id-mismatch", format!("{label} \\"{id}\\" cannot be renamed to \\"{new_id}\\" by a replace."), [id.to_string(), new_id.to_string()])
}""",
     """/// 🚫️ The referenced-target refusal — a `mutation.invariant` at `Error`, the same level and empty-diff
/// shape `mutation.target-missing` carries, addressed at the target FOLLOWED BY every referrer that
/// keeps it alive so a caller can offer to release them.
pub fn target_referenced(id: &str, referrers: Vec<String>) -> protocol::MutationOutcome<Fem3dDiff> {
    let mut target = vec![id.to_string()];
    target.extend(referrers);
    protocol::MutationOutcome::error(protocol::MutationCode::Invariant, target)
}

/// 🪪️ The renaming-replacement refusal — a `replace-` selects its target by `id` and carries a
/// whole new record; a new record under a DIFFERENT id would silently rename the row and orphan
/// every reference to it, so it is a `Fatal` `mutation.invariant`, the level `mutation.duplicate-id`
/// already uses for the other half of the identity contract.
pub fn id_mismatch(id: &str, new_id: &str) -> protocol::MutationOutcome<Fem3dDiff> {
    protocol::MutationOutcome::fatal(protocol::MutationCode::Invariant, [id.to_string(), new_id.to_string()])
}"""),
    (G3, "pub fn invariant(message: String, target: Vec<String>) -> protocol::MutationOutcome<Fem3dDiff> {",
     "pub fn invariant(target: Vec<String>) -> protocol::MutationOutcome<Fem3dDiff> {"),
    (G3, """    let missing = match load {
        FemLoad::Nodal { node_id, .. } => (!base.nodes.iter().any(|node| &node.id == node_id)).then(|| ("Node", node_id.clone())),
        FemLoad::MemberUdl { element_id: id, .. } => (!base.elements.iter().any(|element| element_id(element) == id)).then(|| ("Element", id.clone())),
        FemLoad::Area { solid_id, .. } => (!base.solids.iter().any(|solid| &solid.id == solid_id)).then(|| ("Solid", solid_id.clone())),
    };
    missing.map(|(label, id)| protocol::MutationOutcome::error(protocol::MutationCode::TargetMissing, [id]))""",
     """    let missing = match load {
        FemLoad::Nodal { node_id, .. } => (!base.nodes.iter().any(|node| &node.id == node_id)).then(|| node_id.clone()),
        FemLoad::MemberUdl { element_id: id, .. } => (!base.elements.iter().any(|element| element_id(element) == id)).then(|| id.clone()),
        FemLoad::Area { solid_id, .. } => (!base.solids.iter().any(|solid| &solid.id == solid_id)).then(|| solid_id.clone()),
    };
    missing.map(|id| protocol::MutationOutcome::error(protocol::MutationCode::TargetMissing, [id]))"""),
    (G3, """pub fn node_breach(node: &FemNode) -> Option<String> {
    (!(node.x.is_finite() && node.y.is_finite() && node.z.is_finite())).then(|| format!("Node \\"{}\\" must sit at a finite position, got ({}, {}, {}).", node.id, node.x, node.y, node.z))
}""", """pub fn node_breach(node: &FemNode) -> bool {
    !(node.x.is_finite() && node.y.is_finite() && node.z.is_finite())
}"""),
    (G3, """pub fn material_breach(material: &FemMaterial) -> Option<String> {
    if !(material.e.is_finite() && material.g.is_finite() && material.nu.is_finite() && material.rho.is_finite()) {
        return Some(format!("Material \\"{}\\" must carry finite properties.", material.id));
    }
    if material.e <= 0.0 || material.g <= 0.0 || material.rho <= 0.0 {
        return Some(format!("Material \\"{}\\" must carry a positive e, g and rho, got e={}, g={}, rho={}.", material.id, material.e, material.g, material.rho));
    }
    (!(-1.0 < material.nu && material.nu < 0.5)).then(|| format!("Material \\"{}\\" must carry a Poisson ratio in (-1, 0.5), got {}.", material.id, material.nu))
}""", """pub fn material_breach(material: &FemMaterial) -> bool {
    !(material.e.is_finite() && material.g.is_finite() && material.nu.is_finite() && material.rho.is_finite())
        || material.e <= 0.0
        || material.g <= 0.0
        || material.rho <= 0.0
        || !(-1.0 < material.nu && material.nu < 0.5)
}"""),
    (G3, """pub fn section_breach(section: &FemSection) -> Option<String> {
    if !(section.area.is_finite() && section.iy.is_finite() && section.iz.is_finite() && section.j.is_finite()) {
        return Some(format!("Section \\"{}\\" must carry finite properties.", section.id));
    }
    (section.area <= 0.0 || section.iy <= 0.0 || section.iz <= 0.0 || section.j <= 0.0)
        .then(|| format!("Section \\"{}\\" must carry a positive area, iy, iz and j, got area={}, iy={}, iz={}, j={}.", section.id, section.area, section.iy, section.iz, section.j))
}""", """pub fn section_breach(section: &FemSection) -> bool {
    !(section.area.is_finite() && section.iy.is_finite() && section.iz.is_finite() && section.j.is_finite())
        || section.area <= 0.0
        || section.iy <= 0.0
        || section.iz <= 0.0
        || section.j <= 0.0
}"""),
    (G3, """pub fn solid_breach(solid: &FemSolid) -> Option<String> {
    if solid.outline.len() < 3 {
        return Some(format!("Solid \\"{}\\" needs at least three outline points, got {}.", solid.id, solid.outline.len()));
    }
    if !solid.outline.iter().all(|point| point[0].is_finite() && point[1].is_finite()) {
        return Some(format!("Solid \\"{}\\" must carry a finite outline.", solid.id));
    }
    if ring_area(&solid.outline).abs() <= 0.0 {
        return Some(format!("Solid \\"{}\\" has a degenerate outline of zero area.", solid.id));
    }
    if !(solid.base_z.is_finite() && solid.height.is_finite() && solid.mesh_size.is_finite()) {
        return Some(format!("Solid \\"{}\\" must carry a finite baseZ, height and meshSize.", solid.id));
    }
    if solid.height <= 0.0 {
        return Some(format!("Solid \\"{}\\" must be extruded by a positive height, got {}.", solid.id, solid.height));
    }
    if solid.layers < 1 {
        return Some(format!("Solid \\"{}\\" must be meshed through at least one layer, got {}.", solid.id, solid.layers));
    }
    if solid.mesh_size <= 0.0 {
        return Some(format!("Solid \\"{}\\" must carry a positive mesh size, got {}.", solid.id, solid.mesh_size));
    }
    for hole in &solid.holes {
        if hole.len() < 3 || ring_area(hole).abs() <= 0.0 {
            return Some(format!("Solid \\"{}\\" carries a degenerate hole.", solid.id));
        }
        if !hole.iter().all(|point| point[0].is_finite() && point[1].is_finite() && ring_contains(&solid.outline, *point)) {
            return Some(format!("Solid \\"{}\\" carries a hole that leaves its outline.", solid.id));
        }
    }
    None
}""", """pub fn solid_breach(solid: &FemSolid) -> bool {
    solid.outline.len() < 3
        || !solid.outline.iter().all(|point| point[0].is_finite() && point[1].is_finite())
        || ring_area(&solid.outline).abs() <= 0.0
        || !(solid.base_z.is_finite() && solid.height.is_finite() && solid.mesh_size.is_finite())
        || solid.height <= 0.0
        || solid.layers < 1
        || solid.mesh_size <= 0.0
        || solid.holes.iter().any(|hole| hole.len() < 3 || ring_area(hole).abs() <= 0.0 || !hole.iter().all(|point| point[0].is_finite() && point[1].is_finite() && ring_contains(&solid.outline, *point)))
}"""),
    (G3, """pub fn load_breach(load: &FemLoad) -> Option<String> {
    let (id, finite) = match load {
        FemLoad::Nodal { id, value, .. } => (id, value.is_finite()),
        FemLoad::MemberUdl { id, wx, wy, wz, .. } => (id, wx.is_finite() && wy.is_finite() && wz.is_finite()),
        FemLoad::Area { id, pressure, .. } => (id, pressure.is_finite()),
    };
    (!finite).then(|| format!("Load \\"{id}\\" must carry finite magnitudes."))
}""", """pub fn load_breach(load: &FemLoad) -> bool {
    !match load {
        FemLoad::Nodal { value, .. } => value.is_finite(),
        FemLoad::MemberUdl { wx, wy, wz, .. } => wx.is_finite() && wy.is_finite() && wz.is_finite(),
        FemLoad::Area { pressure, .. } => pressure.is_finite(),
    }
}"""),
    (G3, """pub fn combination_breach(combination: &FemCombination) -> Option<String> {
    (!combination.terms.values().all(|factor| factor.is_finite())).then(|| format!("Combination \\"{}\\" must carry finite factors.", combination.id))
}""", """pub fn combination_breach(combination: &FemCombination) -> bool {
    !combination.terms.values().all(|factor| factor.is_finite())
}"""),
    (G3, """pub fn analysis_breach(settings: &FemAnalysisSettings) -> Option<String> {
    if settings.modal_count < 1 || settings.buckling_count < 1 {
        return Some(format!("Analysis settings need at least one modal and one buckling factor, got {} and {}.", settings.modal_count, settings.buckling_count));
    }
    (!(settings.deformation_scale.is_finite() && settings.deformation_scale > 0.0)).then(|| format!("Analysis settings need a finite positive deformation scale, got {}.", settings.deformation_scale))
}""", """pub fn analysis_breach(settings: &FemAnalysisSettings) -> bool {
    settings.modal_count < 1 || settings.buckling_count < 1 || !(settings.deformation_scale.is_finite() && settings.deformation_scale > 0.0)
}"""),
    (F3 + "🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🔺️diff/🦀️.rs", "//! 🔗️ No `mutation.target-referenced` guard: a combination",
     "//! 🔗️ No referrer guard (`mutation.invariant`): a combination"),
    (F3 + "🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🔺️diff/🦀️.rs", "//! 🛡️ No `mutation.target-referenced` guard: a support",
     "//! 🛡️ No referrer guard (`mutation.invariant`): a support"),
    (F2 + "🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🔺️diff/🦀️.rs", "//! `mutation.target-referenced` branch, and for a structural",
     "//! referrer (`mutation.invariant`) branch, and for a structural"),
]


def fem2d_guards(text: str) -> str:
    """✂️ The fem2d guards module's own callers of its private `invariant`/`missing`: no message argument."""
    return drop_argument(drop_argument(text, "invariant", 1, 2), "missing", 1, 2)


def fem2d_callers(text: str) -> str:
    """✂️ fem2d guard callers: no noun to `identity_matches`, no noun/blocker to `referenced`."""
    text = drop_argument(text, "identity_matches", 0, 3)
    text = drop_argument(text, "referenced", 0, 4)
    return drop_argument(text, "referenced", 0, 3)


def fem3d_callers(text: str) -> str:
    """✂️ fem3d guard callers: a breach check is a `bool`, `invariant` takes the target only, no label to `id_mismatch` or
    `target_referenced`."""
    text = re.sub(r"if let Some\(breach\) = (\w+_breach)\(", r"if \1(", text)
    text = re.sub(r"\binvariant\(breach, ", "invariant(", text)
    text = drop_argument(text, "id_mismatch", 0, 3)
    return drop_argument(text, "target_referenced", 0, 3)


def rust_files(scope: str) -> list[str]:
    from pathlib import Path
    root = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2/✏️s/🔌️plugins")
    return sorted(str(path.relative_to(root)) for path in (root / scope).rglob("🦀️.rs") if "target" not in path.parts)


TRANSFORMS = [(G2, fem2d_guards)] + [(rel, fem2d_callers) for rel in rust_files(F2)] + [(rel, fem3d_callers) for rel in rust_files(F3)]

DOC_RENAMES = [(F2, {"mutation.id-mismatch": "Invariant", "mutation.target-referenced": "Invariant"}),
               (F3, {"mutation.id-mismatch": "Invariant", "mutation.target-referenced": "Invariant"})]
RENAMES = DOC_RENAMES

EDITS += [
    (G2, """        for (at, hole) in region.holes.iter().enumerate() {""", """        for hole in &region.holes {"""),
    (G2, """            if let Some(loose) = hole.iter().find(|point| !point_in_ring(**point, &region.outline)) {""",
     """            if hole.iter().any(|point| !point_in_ring(*point, &region.outline)) {"""),
    (G2, """        let (finite, magnitude) = match load {
            FemLoad::Nodal { value, .. } => (value.is_finite(), *value),
            FemLoad::MemberUdl { wx, wy, .. } => (wx.is_finite() && wy.is_finite(), *wx),
            FemLoad::Area { pressure, .. } => (pressure.is_finite(), *pressure),
        };""", """        let finite = match load {
            FemLoad::Nodal { value, .. } => value.is_finite(),
            FemLoad::MemberUdl { wx, wy, .. } => wx.is_finite() && wy.is_finite(),
            FemLoad::Area { pressure, .. } => pressure.is_finite(),
        };"""),
    (G2, """        combination.terms.iter().find(|term| !term.factor.is_finite()).map(|term| invariant([combination.id.clone()]))""",
     """        (!combination.terms.iter().all(|term| term.factor.is_finite())).then(|| invariant([combination.id.clone()]))"""),
]
