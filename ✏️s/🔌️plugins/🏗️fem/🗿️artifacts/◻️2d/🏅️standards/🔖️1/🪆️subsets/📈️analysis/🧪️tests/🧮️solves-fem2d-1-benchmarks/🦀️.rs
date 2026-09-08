//! 🧮️ `s.fem.fem2d` solver-benchmark case — Rust SUBJECT adapter.
//!
//! This case is the one fem2d case that judges ANALYSIS RESULTS rather than the model document, and
//! it is how the `fem2d-1-mutate` third-party-oracle debt recorded in
//! `../../../🌐️any/🔮️oracle/🔣️.json` is discharged for the twenty-two kinds no mesh carrier could
//! witness. The reference is `🐍️.py` beside this file: `anastruct`, a third-party 2D structural
//! analysis package, plus `scipy.linalg` for the two eigenproblems. This adapter registers the
//! SUBJECT half only — registering an oracle handler here would put this repository's answer on both
//! sides of the comparison.
//!
//! **What production surface is exercised.** Everything runs through the artifact's own bridges in
//! `../../../🌐️any/🧬️schema/🧬️mutations/🦀️.rs`: `fem2d_analysis_report_json` (whole-document
//! linear static), `fem2d_mutated_analysis_report_json` (production diff/apply, then solve),
//! `fem2d_modal_report_json` and `fem2d_buckling_report_json`. Those bridges compute the three
//! projection axes from the document, so the two implementations order the same values the same way
//! without either one being told the order.
//!
//! **What is asserted HERE, in role.** The committed SI reference values in
//! `🧫️fixtures/📊️expected.results.json` are held to relatively, against each case's own peak
//! response rather than an absolute metre; the closed forms each benchmark declares are held to
//! directly; and each mutation kind's declared effect — `changes` or `invariant` — is held against
//! the base model's own answer. The cross-language projection carries the scale-normalised result of
//! every case and combination, so the differential covers every number, not only the ones this file
//! names.

use semio_repo_test_host::{Adapter, Json};

//#region 🔖️Corpus
/// 🧫️ `(scenario suffix, committed snapshot)` per benchmark, in the order the feature states them.
const BENCHMARKS: &[(&str, &str)] = &[
    ("steel-cantilever", "📏️steel-cantilever.snapshot.json"),
    ("steel-simple-beam", "📐️steel-simple-beam.snapshot.json"),
    ("concrete-two-span", "🌉️concrete-two-span.snapshot.json"),
    ("timber-frame-members", "🪵️timber-frame-members.snapshot.json"),
    ("steel-frame-base", "🏢️steel-frame-base.snapshot.json"),
    ("steel-columns", "🏛️steel-columns.snapshot.json"),
];

/// 🏷️ Every typed fem2d mutation kind, in the artifact's own catalog order. Duplicated rather than
/// imported because the oracle-only build must not link the subject crate; the catalog's own
/// `kinds_match_the_enum_and_the_catalog` keeps that declaration honest.
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
//#endregion 🔖️Corpus

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::Json;
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_fem_2d::standards::v1::subsets::any::schema::mutations::{fem2d_analysis_report_json, fem2d_buckling_report_json, fem2d_modal_report_json, fem2d_mutated_analysis_report_json};

    //#region 🔖️Read
    /// 🧫️ The one declared fixture URI of this scenario's steps containing `needle`.
    fn uri_in(ctx: &Context, needle: &str) -> Result<String, String> {
        ctx.scenario
            .steps
            .iter()
            .flat_map(|(_, step)| step.split_whitespace())
            .find(|token| (token.starts_with("local://") || token.starts_with("asset://") || token.starts_with("shared://")) && token.contains(needle))
            .map(|token| token.to_string())
            .ok_or_else(|| format!("scenario {} declares no fixture URI containing {needle:?}", ctx.scenario.id))
    }

    /// 🧫️ The declared fixture's bytes as UTF-8 text.
    fn text(ctx: &Context, needle: &str) -> Result<String, String> {
        let uri = uri_in(ctx, needle)?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the declared fixture {uri} is not UTF-8: {error}"))
    }

    /// 🧫️ The declared fixture parsed as JSON.
    fn json(ctx: &Context, needle: &str) -> Result<Json, String> {
        parse_json(&text(ctx, needle)?)
    }

    /// 📋️ One named member, named in the error when it is absent rather than defaulted to empty.
    fn member<'a>(value: &'a Json, key: &str) -> Result<&'a Json, String> {
        value.get(key).ok_or_else(|| format!("the value carries no {key:?} member"))
    }

    /// 📋️ One member read as an array.
    fn items<'a>(value: &'a Json, key: &str) -> Result<&'a Vec<Json>, String> {
        match member(value, key)? {
            Json::Array(entries) => Ok(entries),
            other => Err(format!("the {key:?} member is {}, not an array", other.to_string())),
        }
    }

    /// 🔢️ One number, refusing any other shape rather than reading it as zero.
    fn number(value: &Json) -> Result<f64, String> {
        match value {
            Json::Number(number) => Ok(*number),
            other => Err(format!("expected a number, found {}", other.to_string())),
        }
    }

    /// 🔢️ One indexed element of an array value.
    fn nth(value: &Json, index: usize) -> Result<&Json, String> {
        match value {
            Json::Array(entries) => entries.get(index).ok_or_else(|| format!("index {index} is past the end of a {}-entry array", entries.len())),
            other => Err(format!("expected an array, found {}", other.to_string())),
        }
    }

    /// 🔤️ One string VALUE — `Json::str` reads a member of an object, which a bare string is not.
    fn label(value: &Json) -> Result<String, String> {
        match value {
            Json::String(text) => Ok(text.clone()),
            other => Err(format!("expected a string, found {}", other.to_string())),
        }
    }

    /// 🔤️ Every string of a string array.
    fn strings(value: &Json, key: &str) -> Result<Vec<String>, String> {
        items(value, key)?
            .iter()
            .map(|entry| match entry {
                Json::String(text) => Ok(text.clone()),
                other => Err(format!("expected a string, found {}", other.to_string())),
            })
            .collect()
    }
    //#endregion 🔖️Read

    //#region 🔖️Compare
    /// 📏️ `|a − b|` measured against the larger of the pair and a stated absolute floor — the same
    /// relative measure the reference uses, so both sides judge agreement the same way.
    fn relative(left: f64, right: f64, floor: f64) -> f64 {
        (left - right).abs() / left.abs().max(right.abs()).max(floor)
    }

    /// 🎯️ Holds one produced value to the committed reference value.
    fn near(produced: f64, committed: f64, floor: f64, tolerance: f64, what: &str) -> Result<(), String> {
        let error = relative(produced, committed, floor);
        if error > tolerance {
            return Err(format!("{what}: this engine produced {produced:.12}, the committed third-party reference says {committed:.12} (relative {error:.3e} > {tolerance:.3e})"));
        }
        Ok(())
    }

    /// 🚨️ The bridge reports a mechanism as `error` and no cases at all; anything else is a solve.
    fn solved(report: &Json, what: &str) -> Result<(), String> {
        match report.get("error") {
            Some(Json::String(message)) => Err(format!("{what}: the engine refused to solve — {message}")),
            _ => Ok(()),
        }
    }
    //#endregion 🔖️Compare

    //#region 🔖️Project
    /// 📏️ The four normalisation decades the committed reference file carries for one model.
    fn scales(entry: &Json) -> Result<[f64; 4], String> {
        let value = member(entry, "scales")?;
        Ok([number(member(value, "translation")?)?, number(member(value, "rotation")?)?, number(member(value, "force")?)?, number(member(value, "moment")?)?])
    }

    /// 📤️ The scale-normalised cross-language projection, in the axes order the bridge computed.
    fn projection(report: &Json, scales: [f64; 4]) -> Result<Json, String> {
        let reactions = strings(report, "reactions")?;
        let mut cases = Vec::new();
        for case in items(report, "cases")? {
            let displacements = items(case, "displacements")?
                .iter()
                .map(|values| {
                    Ok(Json::Array(vec![
                        Json::Number(number(nth(values, 0)?)? / scales[0]),
                        Json::Number(number(nth(values, 1)?)? / scales[0]),
                        Json::Number(number(nth(values, 2)?)? / scales[1]),
                    ]))
                })
                .collect::<Result<Vec<Json>, String>>()?;
            let held = items(case, "reactions")?
                .iter()
                .zip(reactions.iter())
                .map(|(value, label)| Ok(Json::Number(number(value)? / if label.ends_with(".Rz") { scales[3] } else { scales[2] })))
                .collect::<Result<Vec<Json>, String>>()?;
            let elements = items(case, "elements")?
                .iter()
                .map(|values| {
                    Ok(Json::Array(vec![
                        Json::Number(number(nth(values, 0)?)? / scales[2]),
                        Json::Number(number(nth(values, 1)?)? / scales[2]),
                        Json::Number(number(nth(values, 2)?)? / scales[3]),
                        Json::Number(number(nth(values, 3)?)? / scales[2]),
                        Json::Number(number(nth(values, 4)?)? / scales[2]),
                        Json::Number(number(nth(values, 5)?)? / scales[3]),
                    ]))
                })
                .collect::<Result<Vec<Json>, String>>()?;
            cases.push(Json::Object(vec![
                ("id".to_string(), member(case, "id")?.clone()),
                ("displacements".to_string(), Json::Array(displacements)),
                ("reactions".to_string(), Json::Array(held)),
                ("elements".to_string(), Json::Array(elements)),
            ]));
        }
        Ok(Json::Object(vec![
            ("nodes".to_string(), member(report, "nodes")?.clone()),
            ("reactions".to_string(), member(report, "reactions")?.clone()),
            ("members".to_string(), member(report, "members")?.clone()),
            ("cases".to_string(), Json::Array(cases)),
        ]))
    }

    /// 📊️ The compact per-case digest the committed reference file pins: peak translation, peak
    /// rotation, the two reaction resultants, peak moment and peak axial force, in SI units.
    fn summary(report: &Json) -> Result<Vec<(String, [f64; 6])>, String> {
        let reactions = strings(report, "reactions")?;
        let mut digest = Vec::new();
        for case in items(report, "cases")? {
            let mut values = [0.0_f64; 6];
            for entry in items(case, "displacements")? {
                values[0] = values[0].max(number(nth(entry, 0)?)?.abs()).max(number(nth(entry, 1)?)?.abs());
                values[1] = values[1].max(number(nth(entry, 2)?)?.abs());
            }
            for (entry, label) in items(case, "reactions")?.iter().zip(reactions.iter()) {
                if label.ends_with(".Tx") {
                    values[2] += number(entry)?;
                } else if label.ends_with(".Ty") {
                    values[3] += number(entry)?;
                }
            }
            for entry in items(case, "elements")? {
                values[4] = values[4].max(number(nth(entry, 2)?)?.abs()).max(number(nth(entry, 5)?)?.abs());
                values[5] = values[5].max(number(nth(entry, 0)?)?.abs()).max(number(nth(entry, 3)?)?.abs());
            }
            digest.push((label(member(case, "id")?)?, values));
        }
        Ok(digest)
    }

    /// 📊️ The six digest field names, in the order the committed reference file writes them.
    const DIGEST: [&str; 6] = ["peakTranslation", "peakRotation", "reactionFx", "reactionFy", "peakMoment", "peakAxial"];
    //#endregion 🔖️Project

    //#region 🔖️Handlers
    /// 🧮️ Solves one committed benchmark snapshot through the production bridge and holds it to the
    /// committed third-party reference values and to whatever closed form the model carries.
    pub fn solves(fixture: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let document = text(ctx, fixture)?;
            let report = parse_json(&fem2d_analysis_report_json(&document)?)?;
            solved(&report, fixture)?;
            let expected = json(ctx, "expected.results")?;
            let entry = member(member(&expected, "fixtures")?, fixture)?;
            let nodes = strings(&report, "nodes")?;
            let labels = strings(&report, "reactions")?;
            let members = strings(&report, "members")?;
            let committed_cases = member(entry, "cases")?;
            for case in items(&report, "cases")? {
                let id = label(member(case, "id")?)?;
                let committed = committed_cases.get(&id).ok_or_else(|| format!("{fixture}: the committed reference carries no case {id:?}"))?;
                let displacements = items(case, "displacements")?;
                let translation = displacements.iter().try_fold(1e-18_f64, |peak, entry| Ok::<f64, String>(peak.max(number(nth(entry, 0)?)?.abs()).max(number(nth(entry, 1)?)?.abs())))?;
                let rotation = displacements.iter().try_fold(translation * 1e-6, |peak, entry| Ok::<f64, String>(peak.max(number(nth(entry, 2)?)?.abs())))?;
                for (at, node) in nodes.iter().enumerate() {
                    let produced = nth(&displacements[at], 0)?;
                    let held = member(member(committed, "displacements")?, node)?;
                    near(number(produced)?, number(nth(held, 0)?)?, translation, 1e-9, &format!("{fixture}/{id}: {node}.Tx"))?;
                    near(number(nth(&displacements[at], 1)?)?, number(nth(held, 1)?)?, translation, 1e-9, &format!("{fixture}/{id}: {node}.Ty"))?;
                    near(number(nth(&displacements[at], 2)?)?, number(nth(held, 2)?)?, rotation, 1e-9, &format!("{fixture}/{id}: {node}.Rz"))?;
                }
                let held_reactions = member(committed, "reactions")?;
                let force = labels.iter().try_fold(1e-12_f64, |peak, label| Ok::<f64, String>(peak.max(number(member(held_reactions, label)?)?.abs())))?;
                for (at, label) in labels.iter().enumerate() {
                    near(number(nth(member(case, "reactions")?, at)?)?, number(member(held_reactions, label)?)?, force, 1e-9, &format!("{fixture}/{id}: reaction {label}"))?;
                }
                let held_elements = member(committed, "elements")?;
                for (at, name) in members.iter().enumerate() {
                    let produced = nth(member(case, "elements")?, at)?;
                    let held = member(held_elements, name)?;
                    for slot in 0..6 {
                        let floor = if slot == 2 || slot == 5 { force * 10.0 } else { force };
                        near(number(nth(produced, slot)?)?, number(nth(held, slot)?)?, floor, 1e-9, &format!("{fixture}/{id}: {name}[{slot}]"))?;
                    }
                }
            }
            for claim in items(entry, "closedForm")? {
                let case = items(&report, "cases")?.iter().find(|case| member(case, "id").and_then(label).map(|id| id == claim.str("case")).unwrap_or(false)).cloned().ok_or_else(|| format!("{fixture}: the closed form names an unsolved case {:?}", claim.str("case")))?;
                let value = claim.get("value").map(number).transpose()?.ok_or_else(|| format!("{fixture}: a closed form carries no value"))?;
                let produced = if claim.str("quantity") == "displacement" {
                    let at = nodes.iter().position(|node| node == &claim.str("node")).ok_or_else(|| format!("{fixture}: the closed form names an unknown node"))?;
                    let slot = match claim.str("dof").as_str() {
                        "Tx" => 0,
                        "Ty" => 1,
                        _ => 2,
                    };
                    number(nth(nth(member(&case, "displacements")?, at)?, slot)?)?
                } else {
                    let mut total = 0.0;
                    for wanted in strings(claim, "reactions")? {
                        let at = labels.iter().position(|label| label == &wanted).ok_or_else(|| format!("{fixture}: the closed form names an unknown reaction {wanted:?}"))?;
                        total += number(nth(member(&case, "reactions")?, at)?)?;
                    }
                    total
                };
                let tolerance = claim.get("tolerance").map(number).transpose()?.unwrap_or(1e-9);
                near(produced, value, value.abs(), tolerance, &format!("{fixture}: {}", claim.str("what")))?;
            }
            let projected = projection(&report, scales(entry)?)?;
            Ok(Outcome::with_raw(projected.to_string().into_bytes(), projected))
        }
    }

    /// 🧬️ Applies one mutation kind to the base frame through PRODUCTION dispatch, solves what it
    /// left behind, and holds the answer to the committed digest, to the kind's declared effect and,
    /// through the projection, to the reference's own solve of the committed post-mutation snapshot.
    pub fn solves_after(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = text(ctx, "steel-frame-base")?;
            let corpus = json(ctx, "mutated.snapshots")?;
            let entry = member(member(&corpus, "kinds")?, kind)?;
            let report = parse_json(&fem2d_mutated_analysis_report_json(&base, &member(entry, "mutation")?.to_string())?)?;
            solved(&report, kind)?;
            let expected = json(ctx, "expected.results")?;
            let committed = member(member(&expected, "mutated")?, kind)?;
            let produced = summary(&report)?;
            let held = member(committed, "summary")?;
            for (id, values) in &produced {
                let digest = held.get(id).ok_or_else(|| format!("{kind}: the committed reference carries no case {id:?}"))?;
                let floor = DIGEST.iter().try_fold(1e-12_f64, |peak, name| Ok::<f64, String>(peak.max(number(member(digest, name)?)?.abs())))?;
                for (slot, name) in DIGEST.iter().enumerate() {
                    near(values[slot], number(member(digest, name)?)?, floor * 1e-9, 1e-9, &format!("{kind}/{id}: {name}"))?;
                }
            }
            let baseline = parse_json(&fem2d_analysis_report_json(&base)?)?;
            solved(&baseline, "the base frame")?;
            let before = summary(&baseline)?;
            let shared: Vec<&(String, [f64; 6])> = produced.iter().filter(|(id, _)| before.iter().any(|(other, _)| other == id)).collect();
            let moved = before.len() != produced.len()
                || shared.iter().any(|(id, values)| {
                    let was = before.iter().find(|(other, _)| other == id).map(|(_, values)| *values).unwrap_or([0.0; 6]);
                    (0..6).any(|slot| relative(values[slot], was[slot], 1e-12) > 1e-9)
                });
            let effect = label(member(entry, "effect")?)?;
            if effect == "changes" && !moved {
                return Err(format!("{kind}: the feature declares this kind changes the analysis, but every case answers exactly as the base model does"));
            }
            if effect == "invariant" && moved {
                return Err(format!("{kind}: the feature declares this kind leaves the frame analysis untouched, but a case moved"));
            }
            let projected = projection(&report, scales(committed)?)?;
            Ok(Outcome::with_raw(projected.to_string().into_bytes(), projected))
        }
    }

    /// 🎵️ The cantilever's natural frequencies, against the committed third-party reference.
    pub fn modal(ctx: &Context) -> Result<Outcome, String> {
        let report = parse_json(&fem2d_modal_report_json(&text(ctx, "steel-cantilever")?)?)?;
        solved(&report, "modal")?;
        let expected = member(&json(ctx, "expected.results")?, "modal")?.clone();
        let scale = number(member(&expected, "scale")?)?;
        let committed = items(&expected, "frequenciesHz")?.clone();
        let produced = items(&report, "frequenciesHz")?.clone();
        if produced.len() != committed.len() {
            return Err(format!("modal: the engine returned {} frequencies, the committed reference carries {}", produced.len(), committed.len()));
        }
        let mut projected = Vec::new();
        for (at, value) in produced.iter().enumerate() {
            let held = number(&committed[at])?;
            near(number(value)?, held, held.abs(), 0.02, &format!("modal mode {}", at + 1))?;
            projected.push(Json::Number(number(value)? / scale));
        }
        let payload = Json::Object(vec![("frequenciesHz".to_string(), Json::Array(projected))]);
        Ok(Outcome::with_raw(payload.to_string().into_bytes(), payload))
    }

    /// 🏛️ Each column's lowest buckling factor, against the committed third-party reference.
    pub fn buckling(ctx: &Context) -> Result<Outcome, String> {
        let report = parse_json(&fem2d_buckling_report_json(&text(ctx, "steel-columns")?)?)?;
        solved(&report, "buckling")?;
        let expected = member(&json(ctx, "expected.results")?, "buckling")?.clone();
        let scale = number(member(&expected, "scale")?)?;
        let committed = member(&expected, "factors")?.clone();
        let produced = member(&report, "factors")?.clone();
        let Json::Object(entries) = &produced else {
            return Err("buckling: the bridge did not report a factor map".to_string());
        };
        let mut projected: Vec<(String, Json)> = Vec::new();
        for (id, value) in entries {
            let held = number(member(&committed, id)?)?;
            near(number(value)?, held, held.abs(), 0.02, &format!("buckling {id}"))?;
            projected.push((id.clone(), Json::Number(number(value)? / scale)));
        }
        projected.sort_by(|left, right| left.0.cmp(&right.0));
        let payload = Json::Object(vec![("factors".to_string(), Json::Object(projected))]);
        Ok(Outcome::with_raw(payload.to_string().into_bytes(), payload))
    }

    /// 🚨️ A cantilever with its only support deleted is a mechanism, and the engine must refuse it
    /// rather than answer with a displacement.
    pub fn mechanism(ctx: &Context) -> Result<Outcome, String> {
        let supported = json(ctx, "steel-cantilever")?;
        let Json::Object(entries) = &supported else {
            return Err("the cantilever fixture is not an object".to_string());
        };
        let released = Json::Object(entries.iter().map(|(key, value)| if key == "supports" { (key.clone(), Json::Array(Vec::new())) } else { (key.clone(), value.clone()) }).collect());
        let held = parse_json(&fem2d_analysis_report_json(&supported.to_string())?)?;
        solved(&held, "the supported cantilever")?;
        let loose = parse_json(&fem2d_analysis_report_json(&released.to_string())?)?;
        if loose.get("error").is_none() {
            return Err("a cantilever with every support deleted was expected to be refused as singular, but the engine answered with a solve".to_string());
        }
        let payload = Json::Object(vec![("supported".to_string(), Json::String("regular".to_string())), ("released".to_string(), Json::String("singular".to_string()))]);
        Ok(Outcome::with_raw(payload.to_string().into_bytes(), payload))
    }

    /// 🪵️ The derived frame substructure is the committed real-world document minus its regions and
    /// area loads, and the region it drops touches the frame at exactly one node.
    pub fn substructure(ctx: &Context) -> Result<Outcome, String> {
        let committed = json(ctx, "timber-portal-frame")?;
        let derived = json(ctx, "timber-frame-members")?;
        for name in ["nodes", "elements", "materials", "sections", "supports"] {
            if member(&derived, name)?.to_string() != member(&committed, name)?.to_string() {
                return Err(format!("the derived frame substructure changed {name}, which the derivation may not touch"));
            }
        }
        if !items(&derived, "regions")?.is_empty() || !items(&derived, "combinations")?.is_empty() {
            return Err("the derived frame substructure must carry no region and no combination".to_string());
        }
        let cases: Vec<String> = items(&derived, "loadCases")?.iter().map(|case| case.str("id")).collect();
        if cases != vec!["snow".to_string()] {
            return Err(format!("the derived frame substructure keeps the committed frame-only case, it kept {cases:?}"));
        }
        let mut positions = Vec::new();
        for node in items(&committed, "nodes")? {
            positions.push((number(member(node, "x")?)?, number(member(node, "y")?)?));
        }
        let mut attachments: Vec<(String, usize)> = Vec::new();
        for region in items(&committed, "regions")? {
            let mut touching = 0;
            for vertex in items(region, "outline")? {
                let (x, y) = (number(nth(vertex, 0)?)?, number(nth(vertex, 1)?)?);
                if positions.iter().any(|(nx, ny)| (nx - x).abs() < 1e-9 && (ny - y).abs() < 1e-9) {
                    touching += 1;
                }
            }
            attachments.push((region.str("id"), touching));
        }
        match attachments.iter().find(|(id, _)| id.as_str() == "slab_spare") {
            Some((_, 1)) => {}
            other => return Err(format!("slab_spare was expected to touch the frame at exactly one node, it touches {other:?}")),
        }
        let payload = Json::Object(vec![
            ("regionAttachments".to_string(), Json::Object(attachments.iter().map(|(id, count)| (id.clone(), Json::Number(*count as f64))).collect())),
            ("cases".to_string(), Json::Array(cases.into_iter().map(Json::String).collect())),
        ]);
        Ok(Outcome::with_raw(payload.to_string().into_bytes(), payload))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id. SUBJECT only:
/// the reference for every scenario here is the third-party solver in `🐍️.py` beside this file.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        let mut built = built;
        for (scenario, fixture) in BENCHMARKS {
            built = built.subject(&format!("solves-{scenario}"), subject::solves(*fixture));
        }
        for kind in KINDS {
            built = built.subject(&format!("solves-after-{kind}"), subject::solves_after(*kind));
        }
        built = built.subject("modal-cantilever-frequencies", subject::modal);
        built = built.subject("buckling-column-factors", subject::buckling);
        built = built.subject("refuses-a-mechanism", subject::mechanism);
        built = built.subject("derives-the-timber-frame-substructure", subject::substructure);
        return built;
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = (BENCHMARKS, KINDS);
        built
    }
}
//#endregion 🔖️Registration
