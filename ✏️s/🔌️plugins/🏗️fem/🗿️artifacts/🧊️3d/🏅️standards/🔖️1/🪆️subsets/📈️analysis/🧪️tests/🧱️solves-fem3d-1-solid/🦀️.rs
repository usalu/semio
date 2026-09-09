//! 🧱 `s.fem.fem3d` meshed-solid benchmark case — Rust SUBJECT adapter.
//!
//! The reference is `🐍️.py` beside this file: `scikit-fem`, an independent continuum finite-element
//! library that meshes the same prism its own way and solves its own 3D linear-elasticity system.
//! This adapter registers the SUBJECT half only.
//!
//! **What this file puts under test.** The whole `FemSolid` path — `resolve_geometry`'s footprint
//! triangulation, its extrusion, its prism-to-tetrahedron split and the `Tet4` elements it builds —
//! reached through the frozen `fem3d_solve_all` entry point, on the one fixture whose restraint set
//! the uniform-stress solution itself satisfies. Under the top pressure the answer is `−pH/E` to
//! machine precision on both sides; under self weight it is a five-percent verdict, which is all two
//! different meshes of one continuum honestly share.
//!
//! **Why this file decodes the snapshot itself.** Same reason as its siblings
//! `../🧮️solves-fem3d-1-benchmarks` and `../🎵️solves-fem3d-1-eigen`, and with their reader, verbatim.

use semio_repo_test_host::{Adapter, Json};

//#region 🔖️Cases
/// 🌍️ Standard gravity, in m/s². `crate::fem3d_engine::fem3d_solve_all` fixes gravity at
/// `[0.0, 0.0, -9.81]`; the same number is written down here so the closed form cannot drift.
const GRAVITY: f64 = 9.81;

/// 📏️ The self-weight verdict both implementations share. A hexahedral mesh and a tetrahedral one
/// approach `−ρgH²/2E` from their own sides; the reference lands 0.26 % below it.
const SELF_WEIGHT_TOLERANCE: f64 = 0.05;

/// 📏️ How close the subject must land to the committed reference under a uniform pressure, where
/// both a constant-strain tetrahedron and a trilinear hexahedron are exact.
const PRESSURE_TOLERANCE: f64 = 1e-6;
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
    use crate::{Fem3dSnapshot, FemAnalysisSettings, FemCombination, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
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
    use super::{decode, significant, GRAVITY, PRESSURE_TOLERANCE, SELF_WEIGHT_TOLERANCE};
    use semio_repo_test_host::{Context, Json, Outcome};
    use crate::model::Dof;

    /// 🧫️ The one declared fixture URI of this scenario's steps containing `needle`.
    fn uri_in(ctx: &Context, needle: &str) -> Result<String, String> {
        ctx.scenario
            .steps
            .iter()
            .flat_map(|(_, step)| step.split_whitespace())
            .find(|token| (token.starts_with("asset://") || token.starts_with("shared://🧱️solves-fem3d-1-solid/") || token.starts_with("shared://")) && token.contains(needle))
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
            return Err(format!("{label}: this implementation solved {produced:.12e} where the expectation is {expected:.12e}"));
        }
        Ok(())
    }

    /// 🧱️ The prism's top-face axial displacement: the mean `Tz` over every node the solid's own
    /// meshing placed at its top elevation. Those nodes are SYNTHESIZED by `resolve_geometry`
    /// (`{solidId}_m{index}`) and their ids carry no elevation, so the same resolution is asked for
    /// the positions — which is also what makes the comparison mesh-independent, since scikit-fem
    /// averages over the top face of its own, entirely different mesh.
    fn top_shortening(answer: &crate::model::StaticResult, top_nodes: &[String]) -> Result<f64, String> {
        let mut total = 0.0;
        let mut count = 0usize;
        for entry in &answer.displacements {
            if top_nodes.iter().any(|id| id == &entry.node_id) {
                total += entry.values[Dof::Tz.index()];
                count += 1;
            }
        }
        if count == 0 {
            return Err("the solved model reports no node on the solid's top face".to_string());
        }
        Ok(total / count as f64)
    }

    /// 🧱️ Solves the prism under its top pressure and under its own weight.
    pub fn prism(needle: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let document = decode::snapshot(&fixture(ctx, needle)?)?;
            let solved = crate::fem3d_engine::fem3d_solve_all(&document).map_err(|error| error.to_string())?;
            let solid = document.solids.first().ok_or_else(|| "the solid fixture declares no solid".to_string())?;
            let material = document.materials.iter().find(|material| material.id == solid.material_id).ok_or_else(|| "the solid names a material the document does not carry".to_string())?;
            let top = solid.base_z + solid.height;
            let (nodes, _elements, _solids, _supports) = crate::fem3d_engine::meshing::resolve_geometry(&document).map_err(|error| error.to_string())?;
            let top_nodes: Vec<String> = nodes.iter().filter(|node| (node.pos[2] - top).abs() < 1e-9).map(|node| node.id.clone()).collect();
            let pressure: f64 = document
                .load_cases
                .iter()
                .flat_map(|case| case.loads.iter())
                .filter_map(|load| match load {
                    crate::FemLoad::Area { pressure, .. } => Some(*pressure),
                    _ => None,
                })
                .sum();
            let closed_pressure = -pressure * solid.height / material.e;
            let closed_self = -material.rho * GRAVITY * solid.height * solid.height / (2.0 * material.e);
            let committed = reference(ctx)?;
            let displacements = committed.get("topDisplacementZ").ok_or_else(|| "the committed reference carries no topDisplacementZ".to_string())?;

            let produced_pressure = top_shortening(solved.get("pressure").ok_or_else(|| "the fixture declares a `pressure` load case".to_string())?, &top_nodes)?;
            close("solid pressure shortening against the closed form", produced_pressure, closed_pressure, PRESSURE_TOLERANCE)?;
            close("solid pressure shortening against the reference", produced_pressure, super::number(displacements, "pressure"), PRESSURE_TOLERANCE)?;

            let produced_self = top_shortening(solved.get("self").ok_or_else(|| "the fixture declares a `self` load case".to_string())?, &top_nodes)?;
            close("solid self-weight shortening against the closed form", produced_self, closed_self, SELF_WEIGHT_TOLERANCE)?;
            close("solid self-weight shortening against the reference", produced_self, super::number(displacements, "self"), SELF_WEIGHT_TOLERANCE)?;

            let projection = Json::Object(vec![
                ("model".to_string(), Json::String(needle.to_string())),
                ("closedFormZ".to_string(), Json::Object(vec![("pressure".to_string(), significant(closed_pressure)), ("self".to_string(), significant(closed_self))])),
                ("pressureShorteningZ".to_string(), significant(produced_pressure)),
                ("selfWeightWithinFivePercent".to_string(), Json::Bool((produced_self - closed_self).abs() / closed_self.abs() <= SELF_WEIGHT_TOLERANCE)),
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
        return built.subject("solid-prismatic-column", subject::prism("prismatic-solid-column"));
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = (GRAVITY, SELF_WEIGHT_TOLERANCE, PRESSURE_TOLERANCE, number as fn(&Json, &str) -> f64, flag as fn(&Json, &str) -> bool, numbers as fn(&Json) -> Vec<f64>, significant as fn(f64) -> Json);
        built
    }
}
//#endregion 🔖️Registration
