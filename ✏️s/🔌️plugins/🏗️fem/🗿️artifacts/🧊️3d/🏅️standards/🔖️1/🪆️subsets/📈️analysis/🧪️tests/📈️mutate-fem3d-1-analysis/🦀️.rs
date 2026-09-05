//! 🧊 `s.fem.fem3d` analysis mutation case — Rust SUBJECT adapter. Relocated out of the
//! artifact-level `mutate-fem3d-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION` so
//! this subset's own kinds have a subset-owned test.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` beside this file — a
//! second implementation of the structural model and this subset's typed mutations, written in
//! Python from this subset's committed snapshot schema, mutation grammar and specification vectors.
//! This adapter registers the SUBJECT half only: keeping oracle registrations here would put this
//! repository's answer on both sides of the comparison.
//!
//! **What the two roles each hold.** The cross-language projection is the whole model; this artifact
//! composes no digest-derived child, so nothing has to be held back. The committed `🔺️diff` — which
//! pins WHICH fields a mutation was allowed to touch — and the committed `🎯️outcome` are Rust-side
//! report shapes rather than parts of the document, so they stay asserted HERE, in role, in
//! [`subject::spec_vector`].

use semio_repo_test_host::{parse_json, Adapter, Json};

//#region 🔖️Kinds
/// 🏷️ This subset's own slice of `KINDS` in `../../🧬️schema/🧬️mutations/🦀️.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and that file's own
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against both the enum and the manifest.
const KINDS: &[&str] = &["update-analysis-settings"];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect, so
/// [`law::mutation_is_observable`] must not demand one of them.
/// Empty: every committed vector of this subset is forward and message-free, so every kind's effect
/// is visible.
const UNOBSERVABLE: &[&str] = &[

];

/// 🧫️ The same derived steel frame model every fem3d mutation subset case shares, as its own
/// local copy — see `../../../🌐️any/🧪️tests/🔄️round-trips-the-committed-document/🥒️.feature` for
/// the full derivation provenance.
#[cfg(feature = "sut")]
const DERIVED_ASSET: &str = "local://🧊️steel-frame.snapshot.json";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ One kind's committed `(before, mutation, after, diff, outcome)` specification vector, read
/// literally via `include_str!`. This IS the independently handcrafted evidence this case rests on —
/// never recomputed here, never restated as a Rust literal.
struct Vector {
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    diff: &'static str,
    outcome: &'static str,
}

fn vector(kind: &str) -> Vector {
    match kind {
        "update-analysis-settings" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-buckling-mode-count/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-buckling-mode-count/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-buckling-mode-count/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-buckling-mode-count/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-buckling-mode-count/🎯️outcome/🔣️.json"),
        },
        other => panic!("mutate-fem3d-1-analysis: no committed specification vector is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-fem3d-1-analysis: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, DERIVED_ASSET, UNOBSERVABLE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_fem::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::fem3d_mutation_report_json;

    //#region 🔖️Report
    /// 📋️ One member of the production bridge's report, named in the error when it is absent — never
    /// defaulted, because a silently missing member would turn every comparison below into a comparison
    /// of two empty values.
    fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
        report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
    }

    /// 📋️ An array member of the report, rejecting a present-but-wrong-shaped value rather than
    /// treating it as empty.
    fn members(report: &Json, key: &str) -> Result<Vec<Json>, String> {
        match member(report, key)? {
            Json::Array(items) => Ok(items.clone()),
            other => Err(format!("the report's {key:?} member is {}, not an array", other.to_string())),
        }
    }

    /// 📋️ A string array read as owned `String`s — an address list, either declared by a committed
    /// outcome or reported by a diagnostic.
    fn strings(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .iter()
            .map(|segment| match segment {
                Json::String(text) => text.clone(),
                other => other.to_string(),
            })
            .collect()
    }

    /// 🚦️ Normalizes a declared severity word. The committed outcome vectors are not consistent — some
    /// write `warn` where the serialized `Severity` writes `warning` — so the level is normalized before
    /// comparison while the `code`, which is a frozen closed-set identifier, is compared verbatim.
    fn level_of(word: &str) -> String {
        if word == "warn" {
            "warning".to_string()
        } else {
            word.to_string()
        }
    }

    /// 🎯️ Checks the produced diagnostics against the ones the committed `🎯️outcome` vector declares.
    /// A `rejected` vector declares one fault code and the offending address; an `applied` vector
    /// declares an ordered (possibly empty) message list and forbids anything at error level or worse.
    fn declared_outcome_holds(kind: &str, produced: &[Json], outcome: &Json) -> Result<(), String> {
        let codes: Vec<String> = produced.iter().map(|message| message.str("code")).collect();
        let levels: Vec<String> = produced.iter().map(|message| level_of(&message.str("level"))).collect();
        if outcome.str("status") == "rejected" {
            let expected = outcome.str("code");
            if codes != vec![expected.clone()] {
                return Err(format!("mutate-{kind}: the vector declares a rejection with code {expected:?}, the implementation raised {codes:?}"));
            }
            if !levels.iter().any(|level| level == "error" || level == "fatal") {
                return Err(format!("mutate-{kind}: the vector declares a rejection, but the implementation raised it at {levels:?} — a rejection is at least an error"));
            }
            let path = strings(outcome, "path");
            let target = strings(&produced[0], "target");
            if !path.is_empty() && target != path {
                return Err(format!("mutate-{kind}: the vector declares the offending address {path:?}, the implementation reported {target:?}"));
            }
            return Ok(());
        }
        let expected: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).collect();
        if codes != expected {
            return Err(format!("mutate-{kind}: the vector declares the diagnostics {expected:?}, the implementation raised {codes:?}"));
        }
        match levels.iter().find(|level| level.as_str() == "error" || level.as_str() == "fatal") {
            Some(level) => Err(format!("mutate-{kind}: the vector declares an applied outcome, but the implementation raised a {level}")),
            None => Ok(()),
        }
    }
    //#endregion 🔖️Report

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

    /// 🧫️ The declared fixture's bytes as UTF-8 text.
    fn fixture_text(ctx: &Context, needle: &str) -> Result<String, String> {
        let uri = uri_in(ctx, needle)?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the declared fixture {uri} is not UTF-8: {error}"))
    }

    /// 🔀️ Each verb writes exactly ONE of the nine members. That is the check an after-snapshot
    /// comparison cannot make on its own: an implementation that re-derived a sibling collection on
    /// every edit — renumbering ids, re-sorting sections — would still land on the right value for
    /// the member it meant to write.
    fn touches_one(scenario: &str, kind: &str, before: &Json, after: &Json) -> Result<(), String> {
        let written = match kind {
            "update-analysis-settings" => "analysis",
            "add-load" | "remove-load" | "change-load-case-self-weight" | "create-load-case" | "delete-load-case" => "loadCases",
            _ => match kind.split_once('-').map(|(_, noun)| noun).unwrap_or_default() {
                "node" => "nodes",
                "element" => "elements",
                "solid" => "solids",
                "material" => "materials",
                "section" => "sections",
                "support" => "supports",
                "combination" => "combinations",
                other => return Err(format!("{scenario}: no collection is declared for the noun {other:?}")),
            },
        };
        let moved: Vec<String> = ["nodes", "elements", "materials", "sections", "solids", "supports", "loadCases", "combinations", "analysis"]
            .iter()
            .filter(|name| before.get(name) != after.get(name))
            .map(|name| (*name).to_string())
            .collect();
        if moved != vec![written.to_string()] {
            return Err(format!("{scenario}: this verb writes {written} and nothing else, but {moved:?} moved"));
        }
        Ok(())
    }

    /// 🧭️ The one report the production bridge produces for a `(base, mutation)` pair. The bridge's
    /// third argument only feeds its `expectedSnapshot` member, which the real-model scenarios do not
    /// consult, so they pass the base for it.
    fn report_of(scenario: &str, base: &str, mutation: &str, after: &str) -> Result<Json, String> {
        parse_json(&fem3d_mutation_report_json(base, mutation, after).map_err(|error| format!("{scenario}: the input did not reach this subset's own codec: {error}"))?)
    }
    //#endregion 🔖️Plan

    //#region 🔖️Handlers
    /// 🎯️ Applies one kind to the REAL derived steel frame with the parameters the feature states,
    /// and asserts in role that it moved the model and wrote exactly one member.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = fixture_text(ctx, "steel-frame")?;
            let report = report_of(&format!("mutate-{kind}"), &base, ctx.doc_string()?, &base)?;
            let applied = member(&report, "snapshot")?;
            let faults: Vec<String> = members(&report, "messages")?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect();
            if !faults.is_empty() {
                return Err(format!("mutate-{kind}: the feature's parameters were rejected with {faults:?}"));
            }
            law::mutation_is_observable(kind, applied, member(&report, "base")?, &[])?;
            touches_one(&format!("mutate-{kind}"), kind, member(&report, "base")?, applied)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// ↩️ Applies one kind to the REAL derived frame and then EVERY step of its OWN computed inverse.
    /// The projection carries BOTH models: projecting only the restored one would make every row
    /// project the same value and the differential would be vacuous.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = fixture_text(ctx, "steel-frame")?;
            let report = report_of(&format!("inverse-{kind}"), &base, ctx.doc_string()?, &base)?;
            let faults: Vec<String> = members(&report, "inverseMessages")?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect();
            if !faults.is_empty() {
                return Err(format!("inverse-{kind}: an inverse step was rejected with {faults:?}, so the model never got the chance to return"));
            }
            let applied = member(&report, "snapshot")?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, &[])?;
            let restored = member(&report, "inverseSnapshot")?;
            law::inverse_restores(kind, restored, member(&report, "base")?)?;
            let projection = Json::Object(vec![("mutated".to_string(), applied.clone()), ("restored".to_string(), restored.clone())]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 📐️ Replays one committed handcrafted specification vector. This is where the evidence the case
    /// carried before the relocation still lives, undiminished: the applied model is held to the
    /// committed after-snapshot, the produced delta to the committed `🔺️diff`, and the diagnostics to
    /// the committed `🎯️outcome`.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(kind);
            let report = report_of(&format!("spec-vector-{kind}"), committed.before, committed.mutation, committed.after)?;
            let applied = member(&report, "snapshot")?;
            if let Some(first) = law::divergence(applied, member(&report, "expectedSnapshot")?) {
                return Err(format!("spec-vector-{kind}: the applied model is not the committed after-snapshot — {first}"));
            }
            if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(committed.diff)) {
                return Err(format!("spec-vector-{kind}: the produced delta is not the committed 🔺️diff — {first}"));
            }
            declared_outcome_holds(kind, &members(&report, "messages")?, &canonical(committed.outcome))?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            touches_one(&format!("spec-vector-{kind}"), kind, member(&report, "base")?, applied)?;
            law::inverse_restores(kind, member(&report, "inverseSnapshot")?, member(&report, "base")?)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id. SUBJECT only:
/// the reference for every scenario here is the Python implementation beside this file, and
/// registering an oracle handler as well would put this repository's answer on both sides.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        let mut built = built;
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind));
            built = built.subject(&format!("inverse-{kind}"), subject::inverse(kind));
            built = built.subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
        }
        return built;
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = (KINDS, UNOBSERVABLE, vector as fn(&str) -> Vector);
        built
    }
}
//#endregion 🔖️Registration
