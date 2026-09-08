//! 🎵 `s.fem.fem3d` modal and buckling benchmark case — Rust SUBJECT adapter.
//!
//! The reference is `🐍️.py` beside this file: `scipy.linalg.eigh` over a beam `K`/`M`/`Kg` that file
//! assembles from the textbook Euler-Bernoulli matrices, plus the closed forms both implementations
//! are held to. This adapter registers the SUBJECT half only.
//!
//! **What this file puts under test.** `crate::fem3d_engine::modal_buckling::fem3d_modal` and
//! `fem3d_buckling` — and behind them this repository's own hand-rolled subspace iteration
//! (`crate::sparse::subspace_iteration`), which until this case had no external reference of any
//! kind. The kernel's own benchmarks compared it with a closed form at a TEN-percent tolerance; this
//! case demands two, on an eight-element discretisation where the reference itself lands inside
//! 0.3 %.
//!
//! **Why this file decodes the snapshot itself.** Same reason as its sibling
//! `../🧮️solves-fem3d-1-benchmarks`: the subject here is the eigensolver, not the document codec,
//! and the reader keeps this case's failure modes about frequencies rather than about grammar. The
//! reader below is that case's, verbatim — one vocabulary, one shape, read the same way twice rather
//! than differently twice.

use semio_repo_test_host::{Adapter, Json};

//#region 🔖️Cases
/// 🏛️ The four textbook effective-length cases, as `(scenario suffix, fixture needle, K)`.
const BUCKLING: &[(&str, &str, f64)] = &[
    ("column-fixed-free", "buckling-column-fixed-free", 2.0),
    ("column-pinned-pinned", "buckling-column-pinned-pinned", 1.0),
    ("column-fixed-pinned", "buckling-column-fixed-pinned", 0.7),
    ("column-fixed-fixed", "buckling-column-fixed-fixed", 0.5),
];

/// 🎵️ The first three Euler-Bernoulli cantilever eigenvalue roots, `cos β cosh β = −1`.
const BETAS: [f64; 3] = [1.8751040687, 4.6940911330, 7.8547574382];

/// 📏️ How close each implementation must independently land to the closed form. Two percent over
/// eight elements: the reference lands inside 0.3 %, so this is a demand of the discretisation, not
/// of the solver.
const CLOSED_FORM_TOLERANCE: f64 = 0.02;

/// 📏️ How close the subject must land to the committed reference's own eigenvalues. Looser than a
/// static solve's `1e-6` because a subspace iteration converges to a tolerance rather than to a
/// factorisation's last bit.
const REFERENCE_TOLERANCE: f64 = 1e-3;
//#endregion 🔖️Cases

//#region 🔖️Json
/// 🔎️ A numeric member, or zero — the committed fixtures are schema-validated before they get here.
fn number(value: &Json, key: &str) -> f64 {
    match value.get(key) {
        Some(Json::Number(found)) => *found,
        _ => 0.0,
    }
}

/// 🔎️ A boolean member, or false.
fn flag(value: &Json, key: &str) -> bool {
    matches!(value.get(key), Some(Json::Bool(true)))
}

/// 🔎️ The numbers of a JSON array value.
fn numbers(value: &Json) -> Vec<f64> {
    match value {
        Json::Array(items) => items
            .iter()
            .map(|item| match item {
                Json::Number(found) => *found,
                _ => 0.0,
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// 🔢️ One number at six significant figures, as the normalized decimal text this case's comparison
/// profile compares. Text, not a float: the two implementations must agree to six figures, and a
/// float projection would compare their last bits instead, which no two solvers share.
fn significant(value: f64) -> Json {
    if !value.is_finite() {
        return Json::String("nonfinite".to_string());
    }
    if value == 0.0 {
        return Json::String("0.00000e+0".to_string());
    }
    let mut exponent = value.abs().log10().floor() as i32;
    let mut text = format!("{:.5}", value / 10f64.powf(f64::from(exponent)));
    if text.trim_start_matches('-').starts_with("10") {
        exponent += 1;
        text = format!("{:.5}", value / 10f64.powf(f64::from(exponent)));
    }
    Json::String(format!("{text}e{exponent:+}"))
}
//#endregion 🔖️Json

//#region 🔖️Decode
#[cfg(feature = "sut")]
mod decode {
    use super::{flag, number, numbers};
    use semio_repo_test_host::Json;
    use semio_s_artifact_fem_3d::{Fem3dSnapshot, FemAnalysisSettings, FemCombination, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
    use std::collections::BTreeMap;

    /// 🔒️ One degree-of-freedom tag, as this artifact spells it on the wire.
    pub fn dof(name: &str) -> Result<FemDof, String> {
        match name {
            "Tx" => Ok(FemDof::Tx),
            "Ty" => Ok(FemDof::Ty),
            "Tz" => Ok(FemDof::Tz),
            "Rx" => Ok(FemDof::Rx),
            "Ry" => Ok(FemDof::Ry),
            "Rz" => Ok(FemDof::Rz),
            other => Err(format!("no degree of freedom is named {other:?}")),
        }
    }

    /// 📍️ A polygon ring, as `[[x, y], …]`.
    fn ring(value: &Json) -> Vec<[f64; 2]> {
        match value {
            Json::Array(points) => points
                .iter()
                .map(|point| {
                    let pair = numbers(point);
                    [pair.first().copied().unwrap_or(0.0), pair.get(1).copied().unwrap_or(0.0)]
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    /// 🏋️ One load record, across the vocabulary's three variants.
    fn load(value: &Json) -> Result<FemLoad, String> {
        match value.str("kind").as_str() {
            "nodal" => Ok(FemLoad::Nodal { id: value.str("id"), node_id: value.str("nodeId"), dof: dof(&value.str("dof"))?, value: number(value, "value") }),
            "memberUdl" => Ok(FemLoad::MemberUdl { id: value.str("id"), element_id: value.str("elementId"), wx: number(value, "wx"), wy: number(value, "wy"), wz: number(value, "wz") }),
            "area" => Ok(FemLoad::Area { id: value.str("id"), solid_id: value.str("solidId"), pressure: number(value, "pressure") }),
            other => Err(format!("no load kind is named {other:?}")),
        }
    }

    /// 📦️ A combination's `caseId → factor` terms, accepting both committed encodings of a map: the
    /// object form the shared frame fixtures carry, and the `[{caseId, factor}]` list form the
    /// specification vectors carry.
    fn terms(value: &Json) -> BTreeMap<String, f64> {
        match value.get("terms") {
            Some(Json::Object(entries)) => entries
                .iter()
                .map(|(name, factor)| {
                    (
                        name.clone(),
                        match factor {
                            Json::Number(found) => *found,
                            _ => 0.0,
                        },
                    )
                })
                .collect(),
            Some(Json::Array(items)) => items.iter().map(|term| (term.str("caseId"), number(term, "factor"))).collect(),
            _ => BTreeMap::new(),
        }
    }

    /// 📸️ The nine-member document, read literally from the committed bytes.
    pub fn snapshot(value: &Json) -> Result<Fem3dSnapshot, String> {
        let mut elements = Vec::new();
        for element in value.array("elements") {
            elements.push(match element.str("kind").as_str() {
                "frame" => FemElement::Frame { id: element.str("id"), start: element.str("start"), end: element.str("end"), material_id: element.str("materialId"), section_id: element.str("sectionId"), roll: number(&element, "roll") },
                "bar" => FemElement::Bar { id: element.str("id"), start: element.str("start"), end: element.str("end"), material_id: element.str("materialId"), section_id: element.str("sectionId") },
                other => return Err(format!("no element kind is named {other:?}")),
            });
        }
        let mut supports = Vec::new();
        for support in value.array("supports") {
            let mut fixed = Vec::new();
            for entry in support.array("fixed") {
                match entry {
                    Json::String(name) => fixed.push(dof(&name)?),
                    other => return Err(format!("a support's fixed list carries {}, not a degree-of-freedom tag", other.to_string())),
                }
            }
            supports.push(FemSupport { id: support.str("id"), node_id: support.str("nodeId"), fixed });
        }
        let mut load_cases = Vec::new();
        for case in value.array("loadCases") {
            let mut loads = Vec::new();
            for entry in case.array("loads") {
                loads.push(load(&entry)?);
            }
            load_cases.push(FemLoadCase { id: case.str("id"), name: case.str("name"), loads, self_weight: flag(&case, "selfWeight") });
        }
        let analysis = value.get("analysis").cloned().unwrap_or(Json::Null);
        Ok(Fem3dSnapshot {
            nodes: value.array("nodes").iter().map(|node| FemNode { id: node.str("id"), x: number(node, "x"), y: number(node, "y"), z: number(node, "z") }).collect(),
            elements,
            materials: value.array("materials").iter().map(|material| FemMaterial { id: material.str("id"), name: material.str("name"), e: number(material, "e"), g: number(material, "g"), nu: number(material, "nu"), rho: number(material, "rho") }).collect(),
            sections: value.array("sections").iter().map(|section| FemSection { id: section.str("id"), name: section.str("name"), area: number(section, "area"), iy: number(section, "iy"), iz: number(section, "iz"), j: number(section, "j") }).collect(),
            solids: value
                .array("solids")
                .iter()
                .map(|solid| FemSolid {
                    id: solid.str("id"),
                    name: solid.str("name"),
                    outline: ring(solid.get("outline").unwrap_or(&Json::Null)),
                    holes: solid.array("holes").iter().map(ring).collect(),
                    base_z: number(solid, "baseZ"),
                    height: number(solid, "height"),
                    layers: number(solid, "layers").max(1.0) as usize,
                    mesh_size: number(solid, "meshSize"),
                    material_id: solid.str("materialId"),
                })
                .collect(),
            supports,
            load_cases,
            combinations: value.array("combinations").iter().map(|combination| FemCombination { id: combination.str("id"), name: combination.str("name"), terms: terms(combination) }).collect(),
            analysis: FemAnalysisSettings { modal_count: number(&analysis, "modalCount").max(0.0) as usize, buckling_count: number(&analysis, "bucklingCount").max(0.0) as usize, deformation_scale: number(&analysis, "deformationScale") },
        })
    }
}
//#endregion 🔖️Decode

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{decode, significant, BETAS, CLOSED_FORM_TOLERANCE, REFERENCE_TOLERANCE};
    use semio_repo_test_host::{Context, Json, Outcome};

    /// 🧫️ The one declared fixture URI of this scenario's steps containing `needle`.
    fn uri_in(ctx: &Context, needle: &str) -> Result<String, String> {
        ctx.scenario
            .steps
            .iter()
            .flat_map(|(_, step)| step.split_whitespace())
            .find(|token| (token.starts_with("asset://") || token.starts_with("local://") || token.starts_with("shared://")) && token.contains(needle))
            .map(|token| token.to_string())
            .ok_or_else(|| format!("scenario {} declares no fixture URI containing {needle:?}", ctx.scenario.id))
    }

    /// 🧫️ The declared JSON fixture this scenario names.
    fn fixture(ctx: &Context, needle: &str) -> Result<Json, String> {
        ctx.fixture_json(&uri_in(ctx, needle)?)
    }

    /// 🔮️ This scenario's slice of the committed third-party reference.
    fn reference(ctx: &Context) -> Result<Json, String> {
        fixture(ctx, "expected.results")?
            .get("scenarios")
            .and_then(|scenarios| scenarios.get(&ctx.scenario.id))
            .and_then(|scenario| scenario.get("reference"))
            .cloned()
            .ok_or_else(|| format!("the committed reference carries no entry for scenario {}", ctx.scenario.id))
    }

    /// 📏️ Relative agreement, asserted in role.
    fn close(label: &str, produced: f64, expected: f64, tolerance: f64) -> Result<(), String> {
        if (produced - expected).abs() / expected.abs().max(1e-30) > tolerance {
            return Err(format!("{label}: this implementation solved {produced:.12e} where the reference is {expected:.12e}"));
        }
        Ok(())
    }

    /// ✅️ Whether one eigenvalue sits within the closed form's stated tolerance.
    fn within(produced: f64, expected: f64) -> bool {
        (produced - expected).abs() / expected.abs().max(1e-30) <= CLOSED_FORM_TOLERANCE
    }

    /// 🔢️ One reference number by index.
    fn nth(reference: &Json, key: &str, index: usize) -> Result<f64, String> {
        match reference.array(key).get(index) {
            Some(Json::Number(value)) => Ok(*value),
            _ => Err(format!("the committed reference carries no {key}[{index}]")),
        }
    }

    /// 🎵️ The cantilever's natural frequencies, against `βₙ²/(2πL²)·√(EI/ρA)` and the reference.
    pub fn modal(needle: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let document = decode::snapshot(&fixture(ctx, needle)?)?;
            let solved = semio_s_plugin_fem::fem3d_engine::modal_buckling::fem3d_modal(&document).map_err(|error| error.to_string())?;
            let (material, section) = (&document.materials[0], &document.sections[0]);
            let length = document.nodes[document.nodes.len() - 1].x;
            let mut closed: Vec<f64> = Vec::new();
            for beta in BETAS {
                for inertia in [section.iy, section.iz] {
                    closed.push(beta * beta / (2.0 * std::f64::consts::PI * length * length) * (material.e * inertia / (material.rho * section.area)).sqrt());
                }
            }
            closed.sort_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
            let committed = reference(ctx)?;
            let mut verdicts = Vec::new();
            for (rank, expected) in closed.iter().take(4).enumerate() {
                let produced = *solved.frequencies_hz.get(rank).ok_or_else(|| format!("modal-{needle}: this implementation returned only {} frequencies", solved.frequencies_hz.len()))?;
                close(&format!("modal-{needle} mode {}", rank + 1), produced, *expected, CLOSED_FORM_TOLERANCE)?;
                close(&format!("modal-{needle} mode {} against the reference", rank + 1), produced, nth(&committed, "frequenciesHz", rank)?, REFERENCE_TOLERANCE)?;
                verdicts.push(Json::Bool(within(produced, *expected)));
            }
            let projection = Json::Object(vec![
                ("model".to_string(), Json::String(needle.to_string())),
                ("closedFormHz".to_string(), Json::Array(closed.iter().take(4).map(|value| significant(*value)).collect())),
                ("withinTolerance".to_string(), Json::Array(verdicts)),
                ("modeCount".to_string(), Json::Number(document.analysis.modal_count as f64)),
            ]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🏛️ The column's lowest load factor, against `π²EI/(KL)²` and the reference. The fixture's
    /// reference load is exactly 1 N, so that factor IS the critical load in newtons.
    pub fn buckling(needle: &'static str, effective_length_factor: f64) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let document = decode::snapshot(&fixture(ctx, needle)?)?;
            let case_id = document.load_cases[0].id.clone();
            let solved = semio_s_plugin_fem::fem3d_engine::modal_buckling::fem3d_buckling(&document, &case_id).map_err(|error| error.to_string())?;
            let (material, section) = (&document.materials[0], &document.sections[0]);
            let height = document.nodes[document.nodes.len() - 1].z;
            let critical = std::f64::consts::PI * std::f64::consts::PI * material.e * section.iz / (effective_length_factor * height).powi(2);
            let produced = *solved.factors.first().ok_or_else(|| format!("buckling-{needle}: this implementation returned no load factor"))?;
            close(&format!("buckling-{needle} critical load"), produced, critical, CLOSED_FORM_TOLERANCE)?;
            close(&format!("buckling-{needle} against the reference"), produced, nth(&reference(ctx)?, "factors", 0)?, REFERENCE_TOLERANCE)?;
            let projection = Json::Object(vec![
                ("model".to_string(), Json::String(needle.to_string())),
                ("closedFormN".to_string(), significant(critical)),
                ("withinTolerance".to_string(), Json::Bool(within(produced, critical))),
                ("factorCount".to_string(), Json::Number(document.analysis.buckling_count as f64)),
            ]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id. SUBJECT only.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        let mut built = built;
        built = built.subject("modal-cantilever", subject::modal("modal-cantilever"));
        for entry in BUCKLING {
            built = built.subject(&format!("buckling-{}", entry.0), subject::buckling(entry.1, entry.2));
        }
        return built;
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = (BUCKLING, BETAS, CLOSED_FORM_TOLERANCE, REFERENCE_TOLERANCE, number as fn(&Json, &str) -> f64, flag as fn(&Json, &str) -> bool, numbers as fn(&Json) -> Vec<f64>, significant as fn(f64) -> Json);
        built
    }
}
//#endregion 🔖️Registration
