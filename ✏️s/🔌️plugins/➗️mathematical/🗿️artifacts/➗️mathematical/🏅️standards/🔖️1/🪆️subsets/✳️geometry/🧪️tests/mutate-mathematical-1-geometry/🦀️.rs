//! ➗️ `s.mathematical.mathematical` geometry mutation case — Rust adapter. Relocated out of the
//! artifact-level `mutate-mathematical-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION` so
//! `✳️geometry`'s own kinds have a subset-owned test. Recorded no-oracle decision
//! `mathematical-mutation-semantics` (`../../../✳️any/🧪️oracle/🔣️.json`, which also records why
//! `petgraph` and the external CAS candidates were surveyed and DECLINED).
//!
//! ⚠️ The consequence is measured rather than described: three of this subset's four kinds have no
//! forward vector. `remove-point` and `move-point` carry REJECTION vectors
//! (`mutation.target-missing`) and `replace-points` carries an `applied`-but-`mutation.no-op`
//! vector, because this subset's own snapshot no longer holds the point cloud inline: ticket
//! UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM moved it into a composed `results` child no fixture can
//! resolve. `insert-point` is the one real vector: it seeds the empty cloud with its first point,
//! the one index-addressed verb whose inverse (`remove-point` at the same index) is exercised end
//! to end here. The three without forward evidence are listed by name in `UNOBSERVABLE` below.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner resolves
//! an oracle implementation from the feature's `@oracle-` tag and this feature has none — so every law
//! this case claims is asserted inside the SUBJECT handlers, through the shared law module
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` that the stdio subsets use. The oracle handlers
//! below still answer with the committed vector read literally, so the reference side exists the
//! moment a second producer ever does. The subject half is gated behind the generated host's `sut`
//! feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ This subset's own slice of `KINDS` in `../../🧬️schema/🧬️mutations/🦀️.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and that file's own
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against both the enum and the manifest.
const KINDS: &[&str] = &["replace-points", "insert-point", "remove-point", "move-point"];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect, so
/// [`law::mutation_is_observable`] must not demand one of them.
const UNOBSERVABLE: &[&str] = &["replace-points", "remove-point", "move-point"];

//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ One kind's committed `(before, mutation, after, diff, outcome)` specification vector, read
/// literally via `include_str!`. This IS the independently handcrafted evidence the no-oracle decision
/// rests on — never recomputed here, never restated as a Rust literal.
struct Vector {
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    /// 🔺️ `None` for a REJECTION vector, which commits no `🔺️diff` file at all — a rejected
    /// mutation produces the default delta by the frozen outcome contract's law 2, so there is nothing
    /// for the vector to pin. The handler below refuses a missing diff on any vector that is not a
    /// rejection rather than treating absence as permission to skip the check.
    diff: Option<&'static str>,
    outcome: &'static str,
}

fn vector(kind: &str) -> Vector {
    match kind {
        "replace-points" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/🎯️outcome/🔣️.json"),
        },
        "insert-point" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/🎯️outcome/🔣️.json"),
        },
        "remove-point" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/rejects-removing-a-point-from-an-empty-cloud/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/rejects-removing-a-point-from-an-empty-cloud/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/rejects-removing-a-point-from-an-empty-cloud/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/rejects-removing-a-point-from-an-empty-cloud/🎯️outcome/🔣️.json"),
        },
        "move-point" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/rejects-moving-a-point-that-is-not-in-the-cloud/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/rejects-moving-a-point-that-is-not-in-the-cloud/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/rejects-moving-a-point-that-is-not-in-the-cloud/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/rejects-moving-a-point-that-is-not-in-the-cloud/🎯️outcome/🔣️.json"),
        },
        other => panic!("mutate-mathematical-1-geometry: no committed specification vector is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-mathematical-1-geometry: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed after-snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let after = vector(kind).after;
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed before-snapshot — undoing a mutation must land back
/// exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let before = vector(kind).before;
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, UNOBSERVABLE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_mathematical::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::mathematical_mutation_report_json;

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

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to its committed before-snapshot and asserts THREE things the vector commits
    /// to: the resulting document is the committed after-snapshot, the produced delta is the committed
    /// `🔺️diff`, and the diagnostics are the ones the committed `🎯️outcome` declares. A kind the vector shows
    /// moving is additionally held to the observability law, so a mutation that quietly did nothing
    /// cannot pass by agreeing with an unchanged document.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(kind);
            let report = parse_json(&mathematical_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("mutate-{kind}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let applied = member(&report, "snapshot")?;
            let expected = member(&report, "expectedSnapshot")?;
            if let Some(first) = law::divergence(applied, expected) {
                return Err(format!("mutate-{kind}: the applied document is not the committed after-snapshot — {first}"));
            }
            match committed.diff {
                Some(diff) => {
                    if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(diff)) {
                        return Err(format!("mutate-{kind}: the produced delta is not the committed 🔺️diff — {first}"));
                    }
                }
                None if canonical(committed.outcome).str("status") == "rejected" => {}
                None => return Err(format!("mutate-{kind}: the vector commits no 🔺️diff, which only a rejection vector may omit")),
            }
            declared_outcome_holds(kind, &members(&report, "messages")?, &canonical(committed.outcome))?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must restore
    /// the committed before-snapshot exactly. Asserted in role through `law::inverse_restores`, so a
    /// divergence is reported by JSON path rather than as a bare inequality, and an inverse step that
    /// was itself rejected fails here rather than silently leaving the document where it was.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(kind);
            let report = parse_json(&mathematical_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("inverse-{kind}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let faults: Vec<String> = members(&report, "inverseMessages")?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect();
            if !faults.is_empty() {
                return Err(format!("inverse-{kind}: an inverse step was rejected with {faults:?}, so the document never got the chance to return"));
            }
            let restored = member(&report, "inverseSnapshot")?;
            law::inverse_restores(kind, restored, member(&report, "base")?)?;
            Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored.clone()))
        }
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario id,
/// so the loop mirrors the feature's `Examples` tables exactly.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built
}
//#endregion 🔖️Registration
