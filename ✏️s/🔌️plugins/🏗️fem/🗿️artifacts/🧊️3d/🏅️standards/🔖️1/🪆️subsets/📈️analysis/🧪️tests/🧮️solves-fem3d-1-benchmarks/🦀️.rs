//! 🧮 `s.fem.fem3d` analysis-benchmark case — Rust SUBJECT adapter.
//!
//! The reference is `🐍️.py` beside this file: `PyNiteFEA`, an independent 3D frame and truss
//! finite-element solver, driven from the same committed models. This adapter registers the SUBJECT
//! half only — registering an oracle handler here as well would put this repository's answer on both
//! sides of the comparison.
//!
//! **What this file puts under test.** `crate::fem3d_engine::fem3d_solve_all` — the frozen entry
//! point the plugin's own results window calls — on real structures, and on the model each typed
//! mutation produces. The mutation itself goes through PRODUCTION dispatch
//! (`fem3d_mutation_report_json`) and the applied model is held to the committed after-model before
//! anything is solved, so a scenario cannot pass by solving the wrong document.
//!
//! **Why this file decodes the snapshot itself.** The subject under test here is the SOLVER, not the
//! document codec: what the DSL, pack and store encodings do with these bytes is the business of the
//! `🌐️any` round-trip cases, which own it. A small literal reader keeps this case's failure modes
//! about stiffness and equilibrium rather than about grammar, and keeps the generated host's
//! dependency graph to the two crates it already links.

use semio_repo_test_host::{Adapter, Json};

//#region 🔖️Kinds
/// 🏷️ Every kind of this artifact's vocabulary except `create-`/`delete-`/`replace-solid`, whose
/// subject is geometry and whose oracles are `three-fem3d-mesh-reader` and
/// `manifold-fem3d-mesh-measure` — the twenty-two a frame solver can genuinely adjudicate.
/// Duplicated, not imported, because the oracle-only build must not link the subject crate.
const KINDS: &[&str] = &[
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
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
];

/// 🏗️ The real-world models this case solves whole, by the needle its scenarios name them with.
const MODELS: &[&str] = &["steel-frame", "space-frame", "roof-truss"];

/// 📏️ The closed-form scenarios, as `(scenario suffix, fixture needle, tip degree of freedom)`.
/// `Tz` bends the cantilever about its local y axis (`iy`), `Ty` about its local z axis (`iz`).
const CLOSED_FORM: &[(&str, &str, &str)] = &[("cantilever-local-z", "cantilever-tip-load-local-z", "Tz"), ("cantilever-local-y", "cantilever-tip-load-local-y", "Ty")];

/// 📏️ The relative agreement demanded of the committed third-party reference for a static answer.
/// Two direct double-precision factorisations of one linear system disagree at the `1e-12` scale;
/// `1e-6` is a millionfold margin over that and still far below any modelling defect.
const STATIC_TOLERANCE: f64 = 1e-6;

/// 🔢️ The six degrees of freedom in this artifact's own index order.
const DOFS: [&str; 6] = ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"];
//#endregion 🔖️Kinds

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
/// 🔢️ The same text, with a component that is zero only up to round-off snapped to zero. A resultant
/// that cancels by symmetry lands on `-2.5e-11` in one solver and `+8.1e-12` in another; six
/// significant figures OF ROUND-OFF is noise, not agreement. Anything a billionth of its own group's
/// largest component is therefore reported as the zero it is.
fn significant_relative(value: f64, scale: f64) -> Json {
    if value.abs() <= 1e-9 * scale {
        return Json::String("0.00000e+0".to_string());
    }
    significant(value)
}
//#endregion 🔖️Json

//#region 🔖️Decode
#[cfg(feature = "sut")]
mod decode {
    use super::{flag, number, numbers};
    use semio_repo_test_host::Json;
    use semio_s_plugin_fem::artifacts::fem3d::{Fem3dSnapshot, FemAnalysisSettings, FemCombination, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
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
    use super::{decode, number, numbers, significant, significant_relative, DOFS, STATIC_TOLERANCE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_fem::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::fem3d_mutation_report_json;
    use semio_s_plugin_fem::artifacts::fem3d::Fem3dSnapshot;
    use semio_s_plugin_fem::model::{Dof, StaticResult};
    use semio_s_plugin_stdio_test_oracle::law;
    use std::collections::BTreeMap;

    //#region 🔖️Plan
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

    /// 🧫️ The declared fixture's bytes as UTF-8 text — needed where a production entry point takes
    /// the document as text rather than as a value.
    fn fixture_text(ctx: &Context, needle: &str) -> Result<String, String> {
        let uri = uri_in(ctx, needle)?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the declared fixture {uri} is not UTF-8: {error}"))
    }

    /// 🔮️ This scenario's slice of the committed third-party reference — the answers `🐍️.py`
    /// produced with PyNite and `🔨️run-fem3d-oracle.py` committed.
    fn reference(ctx: &Context) -> Result<Json, String> {
        let committed = fixture(ctx, "expected.results")?;
        committed
            .get("scenarios")
            .and_then(|scenarios| scenarios.get(&ctx.scenario.id))
            .and_then(|scenario| scenario.get("reference"))
            .cloned()
            .ok_or_else(|| format!("the committed reference carries no entry for scenario {}", ctx.scenario.id))
    }
    //#endregion 🔖️Plan

    //#region 🔖️Solve
    /// 🚀️ Solves one document's LINE-ELEMENT sub-model — `solids` emptied and every `area` load
    /// dropped, the projection this case's feature declares and its reference applies to the same
    /// bytes — through the frozen `fem3d_solve_all` entry point.
    fn solve(document: &Fem3dSnapshot) -> Result<BTreeMap<String, StaticResult>, String> {
        let mut projected = document.clone();
        projected.solids.clear();
        for case in &mut projected.load_cases {
            case.loads.retain(|load| !matches!(load, semio_s_plugin_fem::artifacts::fem3d::FemLoad::Area { .. }));
        }
        let solved = semio_s_plugin_fem::fem3d_engine::fem3d_solve_all(&projected).map_err(|error| error.to_string())?;
        Ok(solved.into_iter().collect())
    }

    /// 📐️ The compared projection: each case's answer as five scale-carrying scalars at six
    /// significant figures, exactly as `🐍️.py`'s own `reduction` computes them.
    fn reduction(answers: &BTreeMap<String, StaticResult>) -> Json {
        let mut cases = Vec::new();
        for (name, answer) in answers {
            let mut squared = 0.0;
            let (mut translation, mut rotation, mut axial) = (0.0f64, 0.0f64, 0.0f64);
            let mut ordered: Vec<&semio_s_plugin_fem::model::NodeDisplacement> = answer.displacements.iter().collect();
            ordered.sort_by(|left, right| left.node_id.cmp(&right.node_id));
            for entry in ordered {
                for (index, value) in entry.values.iter().enumerate() {
                    squared += value * value;
                    if index < 3 {
                        translation = translation.max(value.abs());
                    } else {
                        rotation = rotation.max(value.abs());
                    }
                }
            }
            let mut force = [0.0f64; 3];
            for reaction in &answer.reactions {
                if reaction.dof.index() < 3 {
                    force[reaction.dof.index()] += reaction.value;
                }
            }
            for (_, element) in &answer.elements {
                if let semio_s_plugin_fem::model::ElementResult::Bar { n } = element {
                    axial = axial.max(n.abs());
                }
            }
            let scale = force.iter().fold(0.0f64, |largest, component| largest.max(component.abs()));
            cases.push((
                name.clone(),
                Json::Object(vec![
                    ("displacementNorm".to_string(), significant(squared.sqrt())),
                    ("maxTranslation".to_string(), significant(translation)),
                    ("maxRotation".to_string(), significant(rotation)),
                    ("reactionForceSum".to_string(), Json::Array(force.iter().map(|component| significant_relative(*component, scale)).collect())),
                    ("maxAxial".to_string(), significant(axial)),
                ]),
            ));
        }
        Json::Object(cases)
    }

    /// 🔍️ Holds the produced answer to the committed third-party reference, field for field.
    fn agrees_with_reference(scenario: &str, answers: &BTreeMap<String, StaticResult>, reference: &Json) -> Result<(), String> {
        let cases = reference.get("cases").ok_or_else(|| format!("{scenario}: the committed reference carries no cases"))?;
        let expected_names: Vec<String> = match cases {
            Json::Object(entries) => entries.iter().map(|(name, _)| name.clone()).collect(),
            other => return Err(format!("{scenario}: the committed reference's cases member is {}, not an object", other.to_string())),
        };
        let mut produced_names: Vec<String> = answers.keys().cloned().collect();
        produced_names.sort();
        let mut sorted_expected = expected_names.clone();
        sorted_expected.sort();
        if produced_names != sorted_expected {
            return Err(format!("{scenario}: the reference answers {sorted_expected:?}, this implementation answered {produced_names:?}"));
        }
        for name in &expected_names {
            let expected = cases.get(name).ok_or_else(|| format!("{scenario}: no reference for case {name}"))?;
            let answer = answers.get(name).ok_or_else(|| format!("{scenario}: no answer for case {name}"))?;
            let displacements = expected.get("displacements").ok_or_else(|| format!("{scenario}: case {name} carries no reference displacements"))?;
            let scale = displacement_scale(displacements).max(1e-12);
            for entry in &answer.displacements {
                let expected_values = numbers(displacements.get(&entry.node_id).ok_or_else(|| format!("{scenario}: case {name} has no reference displacement for node {}", entry.node_id))?);
                for (index, value) in entry.values.iter().enumerate() {
                    let target = expected_values.get(index).copied().unwrap_or(0.0);
                    if (value - target).abs() / scale > STATIC_TOLERANCE {
                        return Err(format!("{scenario}: case {name} node {} {} is {value:.12e}, the reference says {target:.12e}", entry.node_id, DOFS[index]));
                    }
                }
            }
            let reactions = expected.get("reactions").ok_or_else(|| format!("{scenario}: case {name} carries no reference reactions"))?;
            let reaction_scale = member_scale(reactions).max(1e-9);
            for reaction in &answer.reactions {
                let key = format!("{}|{}", reaction.node_id, DOFS[reaction.dof.index()]);
                let target = number(reactions, &key);
                if reactions.get(&key).is_none() {
                    return Err(format!("{scenario}: case {name} reports a reaction at {key} the reference does not"));
                }
                if (reaction.value - target).abs() / reaction_scale > STATIC_TOLERANCE {
                    return Err(format!("{scenario}: case {name} reaction {key} is {:.12e}, the reference says {target:.12e}", reaction.value));
                }
            }
            let axial = expected.get("axial").cloned().unwrap_or(Json::Object(Vec::new()));
            let axial_scale = member_scale(&axial).max(1e-9);
            for (id, element) in &answer.elements {
                if let semio_s_plugin_fem::model::ElementResult::Bar { n } = element {
                    let target = number(&axial, id);
                    if axial.get(id).is_none() {
                        return Err(format!("{scenario}: case {name} reports a bar force for {id} the reference does not"));
                    }
                    if (*n - target).abs() / axial_scale > STATIC_TOLERANCE {
                        return Err(format!("{scenario}: case {name} bar {id} carries {n:.12e}, the reference says {target:.12e}"));
                    }
                }
            }
        }
        Ok(())
    }

    /// 📏️ The largest magnitude in a reference displacement map — the scale a relative comparison
    /// needs so a component that is zero by symmetry is not measured against itself.
    fn displacement_scale(displacements: &Json) -> f64 {
        match displacements {
            Json::Object(entries) => entries.iter().flat_map(|(_, values)| numbers(values)).fold(0.0f64, |largest, value| largest.max(value.abs())),
            _ => 0.0,
        }
    }

    /// 📏️ The largest magnitude in a flat reference map of scalars.
    fn member_scale(values: &Json) -> f64 {
        match values {
            Json::Object(entries) => entries.iter().fold(0.0f64, |largest, (_, value)| match value {
                Json::Number(found) => largest.max(found.abs()),
                _ => largest,
            }),
            _ => 0.0,
        }
    }

    /// 🔬️ How the answer moved between two models: which combinations changed, which are untouched,
    /// which appeared and which are gone — the mechanism both implementations must report alike.
    fn mechanism(before: &BTreeMap<String, StaticResult>, after: &BTreeMap<String, StaticResult>) -> Json {
        let mut names: Vec<String> = before.keys().chain(after.keys()).cloned().collect();
        names.sort();
        names.dedup();
        let mut moved = Vec::new();
        for name in names {
            let verdict = match (before.get(&name), after.get(&name)) {
                (None, Some(_)) => "appeared".to_string(),
                (Some(_), None) => "disappeared".to_string(),
                (Some(left), Some(right)) => {
                    if left.displacements.len() != right.displacements.len() {
                        "reshaped".to_string()
                    } else {
                        let mut scale = 1e-30f64;
                        let mut worst = 0.0f64;
                        let mut same_nodes = true;
                        for entry in &left.displacements {
                            match right.displacements.iter().find(|other| other.node_id == entry.node_id) {
                                Some(other) => {
                                    for (index, value) in entry.values.iter().enumerate() {
                                        scale = scale.max(value.abs()).max(other.values[index].abs());
                                        worst = worst.max((value - other.values[index]).abs());
                                    }
                                }
                                None => same_nodes = false,
                            }
                        }
                        if !same_nodes {
                            "reshaped".to_string()
                        } else if worst / scale < 1e-9 {
                            "unchanged".to_string()
                        } else {
                            "changed".to_string()
                        }
                    }
                }
                (None, None) => "absent".to_string(),
            };
            moved.push((name, Json::String(verdict)));
        }
        Json::Object(moved)
    }
    //#endregion 🔖️Solve

    //#region 🔖️Handlers
    /// 📊️ Solves one committed real-world model whole, and holds it to the committed reference.
    pub fn static_model(needle: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let document = decode::snapshot(&fixture(ctx, needle)?)?;
            let answers = solve(&document)?;
            agrees_with_reference(&format!("static-{needle}"), &answers, &reference(ctx)?)?;
            let projection = Json::Object(vec![("model".to_string(), Json::String(needle.to_string())), ("cases".to_string(), reduction(&answers))]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 📏️ A tip-loaded cantilever, held to `PL³/3EI`, `PL²/2EI` and `PL` as well as to the reference.
    pub fn cantilever(needle: &'static str, axis: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let document = decode::snapshot(&fixture(ctx, needle)?)?;
            let answers = solve(&document)?;
            agrees_with_reference(&format!("closed-form-{needle}"), &answers, &reference(ctx)?)?;
            let (material, section, length) = (&document.materials[0], &document.sections[0], document.nodes[1].x);
            let inertia = if axis == "Tz" { section.iy } else { section.iz };
            let load = match &document.load_cases[0].loads[0] {
                semio_s_plugin_fem::artifacts::fem3d::FemLoad::Nodal { value, .. } => *value,
                other => return Err(format!("the cantilever fixture's load is {other:?}, not a nodal force")),
            };
            let index = DOFS.iter().position(|name| *name == axis).unwrap_or(2);
            let answer = answers.get("tip").ok_or_else(|| "the cantilever fixture declares a `tip` load case".to_string())?;
            let tip = answer.displacements.iter().find(|entry| entry.node_id == "tip").ok_or_else(|| "the solved model carries no `tip` node".to_string())?;
            close(&format!("closed-form-{needle} tip deflection"), tip.values[index], load * length.powi(3) / (3.0 * material.e * inertia), 1e-9)?;
            close(&format!("closed-form-{needle} base moment"), reaction_of(answer, "root", if axis == "Tz" { Dof::Ry } else { Dof::Rz })?.abs(), load.abs() * length, 1e-9)?;
            close(&format!("closed-form-{needle} base shear"), reaction_of(answer, "root", if axis == "Tz" { Dof::Tz } else { Dof::Ty })?, -load, 1e-9)?;
            let projection = Json::Object(vec![("model".to_string(), Json::String(needle.to_string())), ("cases".to_string(), reduction(&answers))]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🌀️ A tip-twisted cantilever, held to `TL/GJ`.
    pub fn torsion(needle: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let document = decode::snapshot(&fixture(ctx, needle)?)?;
            let answers = solve(&document)?;
            agrees_with_reference(&format!("closed-form-{needle}"), &answers, &reference(ctx)?)?;
            let (material, section, length) = (&document.materials[0], &document.sections[0], document.nodes[1].x);
            let torque = match &document.load_cases[0].loads[0] {
                semio_s_plugin_fem::artifacts::fem3d::FemLoad::Nodal { value, .. } => *value,
                other => return Err(format!("the torsion fixture's load is {other:?}, not a nodal moment")),
            };
            let answer = answers.get("tip").ok_or_else(|| "the torsion fixture declares a `tip` load case".to_string())?;
            let tip = answer.displacements.iter().find(|entry| entry.node_id == "tip").ok_or_else(|| "the solved model carries no `tip` node".to_string())?;
            close("closed-form-torsion tip twist", tip.values[Dof::Rx.index()], torque * length / (material.g * section.j), 1e-9)?;
            close("closed-form-torsion base torque", reaction_of(answer, "root", Dof::Rx)?, -torque, 1e-9)?;
            let projection = Json::Object(vec![("model".to_string(), Json::String(needle.to_string())), ("cases".to_string(), reduction(&answers))]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🌉️ A simply supported beam under a UDL, held to `5wL⁴/384EI` and `wL/2`.
    pub fn simply_supported(needle: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let document = decode::snapshot(&fixture(ctx, needle)?)?;
            let answers = solve(&document)?;
            agrees_with_reference(&format!("closed-form-{needle}"), &answers, &reference(ctx)?)?;
            let (material, section, length) = (&document.materials[0], &document.sections[0], document.nodes[2].x);
            let w = match &document.load_cases[0].loads[0] {
                semio_s_plugin_fem::artifacts::fem3d::FemLoad::MemberUdl { wz, .. } => *wz,
                other => return Err(format!("the simply supported fixture's load is {other:?}, not a member UDL")),
            };
            let answer = answers.get("udl").ok_or_else(|| "the simply supported fixture declares a `udl` load case".to_string())?;
            let mid = answer.displacements.iter().find(|entry| entry.node_id == "mid").ok_or_else(|| "the solved model carries no `mid` node".to_string())?;
            close("simply supported midspan deflection", mid.values[Dof::Tz.index()], 5.0 * w * length.powi(4) / (384.0 * material.e * section.iy), 2e-3)?;
            close("simply supported left reaction", reaction_of(answer, "left", Dof::Tz)?, -w * length / 2.0, 1e-9)?;
            close("simply supported right reaction", reaction_of(answer, "right", Dof::Tz)?, -w * length / 2.0, 1e-9)?;
            let projection = Json::Object(vec![("model".to_string(), Json::String(needle.to_string())), ("cases".to_string(), reduction(&answers))]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🦠️ Applies one typed edit through PRODUCTION dispatch, holds the applied model to the
    /// committed after-model, then solves what it became and reports how the answer moved.
    pub fn mutate_solve(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base_text = fixture_text(ctx, "mutation-base")?;
            let after_text = fixture_text(ctx, &format!("mutation-after-{kind}"))?;
            let report = parse_json(&fem3d_mutation_report_json(&base_text, ctx.doc_string()?, &after_text).map_err(|error| format!("mutate-solve-{kind}: the input did not reach this subset's own codec: {error}"))?)?;
            let applied = report.get("snapshot").ok_or_else(|| format!("mutate-solve-{kind}: the report carries no snapshot"))?;
            let faults: Vec<String> = report
                .array("messages")
                .iter()
                .filter(|message| {
                    let level = message.str("level");
                    level == "error" || level == "fatal"
                })
                .map(|message| message.str("code"))
                .collect();
            if !faults.is_empty() {
                return Err(format!("mutate-solve-{kind}: the feature's parameters were rejected with {faults:?}"));
            }
            if let Some(first) = law::divergence(applied, report.get("expectedSnapshot").ok_or_else(|| format!("mutate-solve-{kind}: the report carries no expectedSnapshot"))?) {
                return Err(format!("mutate-solve-{kind}: the applied model is not the committed after-model — {first}"));
            }
            // 🧭️Production dispatch has just been held to the committed after-model, member for
            // member, by `law::divergence` above — so what is solved below is that model, read from
            // the committed bytes whose shape this case's own reader is written against rather than
            // from the report's re-encoding, which the `🌐️any` round-trip cases own.
            let before = solve(&decode::snapshot(&fixture(ctx, "mutation-base")?)?)?;
            let document = decode::snapshot(&fixture(ctx, &format!("mutation-after-{kind}"))?)?;
            let after = solve(&document)?;
            agrees_with_reference(&format!("mutate-solve-{kind}"), &after, &reference(ctx)?.get("after").cloned().map(|cases| Json::Object(vec![("cases".to_string(), cases)])).ok_or_else(|| format!("mutate-solve-{kind}: the committed reference carries no after"))?)?;
            let projection = Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("after".to_string(), reduction(&after)), ("mechanism".to_string(), mechanism(&before, &after))]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ⚖️ One reaction value, named rather than defaulted — an absent reaction would otherwise read
    /// as a satisfied zero.
    fn reaction_of(answer: &StaticResult, node_id: &str, dof: Dof) -> Result<f64, String> {
        answer.reactions.iter().find(|reaction| reaction.node_id == node_id && reaction.dof == dof).map(|reaction| reaction.value).ok_or_else(|| format!("the solved model reports no reaction at {node_id} {dof:?}"))
    }

    /// 📏️ Relative agreement with a closed form, asserted in role.
    fn close(label: &str, produced: f64, expected: f64, tolerance: f64) -> Result<(), String> {
        let scale = expected.abs().max(1e-30);
        if (produced - expected).abs() / scale > tolerance {
            return Err(format!("{label}: this implementation solved {produced:.12e} where the closed form is {expected:.12e}"));
        }
        Ok(())
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id. SUBJECT only:
/// the reference for every scenario here is `🐍️.py`'s PyNite model beside this file, and registering
/// an oracle handler as well would put this repository's answer on both sides.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        let mut built = built;
        for needle in MODELS {
            built = built.subject(&format!("static-{needle}"), subject::static_model(*needle));
        }
        for entry in CLOSED_FORM {
            built = built.subject(&format!("closed-form-{}", entry.0), subject::cantilever(entry.1, entry.2));
        }
        built = built.subject("closed-form-cantilever-torsion", subject::torsion("cantilever-tip-torsion"));
        built = built.subject("closed-form-simply-supported-udl", subject::simply_supported("simply-supported-udl"));
        for kind in KINDS {
            built = built.subject(&format!("mutate-solve-{kind}"), subject::mutate_solve(*kind));
        }
        return built;
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = (KINDS, MODELS, CLOSED_FORM, DOFS, STATIC_TOLERANCE, number as fn(&Json, &str) -> f64, flag as fn(&Json, &str) -> bool, numbers as fn(&Json) -> Vec<f64>, significant as fn(f64) -> Json, significant_relative as fn(f64, f64) -> Json);
        built
    }
}
//#endregion 🔖️Registration
