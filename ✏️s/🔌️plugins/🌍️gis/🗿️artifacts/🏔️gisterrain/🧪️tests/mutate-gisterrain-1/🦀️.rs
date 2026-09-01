//! 🏔️ `s.gis.gisterrain` exhaustive mutation case — Rust SUBJECT adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` beside this file —
//! a second implementation of the document and both typed mutations, written in Python from this
//! subset's committed snapshot schema, mutation grammar and specification vectors. This adapter
//! therefore registers the SUBJECT half only: keeping oracle registrations here would put this
//! repository's answer on both sides of the comparison.
//!
//! Two persisted fields, an `f64` exaggeration and an opaque `imported_features_json` string, plus a
//! `mesh` slot that is not content but a CONTENT-ADDRESSED child handle derived from exactly those
//! two. So both mutations are root-scalar setters with a second-order effect, and the sharp check is
//! that the handle moves with the field and converges back on undo.
//!
//! **What is compared across the two languages, and what is asserted in role.** The cross-language
//! projection is the two fields `🧬️schema/📸️snapshot/🔣️component.json` declares. The `mesh` handle's
//! `childId` is a `std::hash::DefaultHasher` digest whose value the standard library leaves
//! UNSPECIFIED, so no implementation in another language can reproduce it; it is held exactly HERE,
//! in role, by [`subject::spec_vector`] against the committed after-snapshot, alongside the
//! committed `🔺️diff` and `🎯️outcome` — every check this case already made, unchanged. The
//! `.dsl.semio` carrier's fixpoint and pack-agreement laws are likewise asserted here in role, on
//! the artifact's committed example, in [`subject::round_trip`]. No comparison profile was touched
//! and no `ignoreKeys` was added.

use semio_repo_test_host::{parse_json, Adapter, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and that file's own
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against both the enum and the manifest.
const KINDS: &[&str] = &[
    "change-exaggeration",
    "change-imported-features",
];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect, so
/// [`law::mutation_is_observable`] must not demand one of them.
/// Empty: both committed vectors move a persisted field, so both forward effects are visible.
const UNOBSERVABLE: &[&str] = &[

];

/// 🗣️ The real committed document this artifact ships as its own example.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";

/// 🧫️ The real derived terrain document: the committed example's exaggeration and `mesh` handle,
/// carrying the two REAL Liège positions the sibling `gismap` example commits. Derived once,
/// provenance recorded in the feature description, because the committed example's imported payload
/// is empty and `change-imported-features` would otherwise replace nothing with something.
#[cfg(feature = "sut")]
const DERIVED_ASSET: &str = "local://🏔️liege-terrain.snapshot.json";

/// 🗂️ The two fields `GisTerrainSnapshot` declares — the cross-language projection.
const FIELDS: &[&str] = &["exaggeration", "importedFeaturesJson"];
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ One kind's committed `(before, mutation, after, diff, outcome)` specification vector, read
/// literally via `include_str!`. This IS the independently handcrafted evidence the no-oracle decision
/// rests on — never recomputed here, never restated as a Rust literal.
struct Vector {
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    diff: &'static str,
    outcome: &'static str,
}

fn vector(kind: &str) -> Vector {
    match kind {
        "change-exaggeration" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚change-exaggeration/🧪️tests/raises-exaggeration-from-1-to-2-5/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚change-exaggeration/🧪️tests/raises-exaggeration-from-1-to-2-5/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚change-exaggeration/🧪️tests/raises-exaggeration-from-1-to-2-5/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚change-exaggeration/🧪️tests/raises-exaggeration-from-1-to-2-5/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚change-exaggeration/🧪️tests/raises-exaggeration-from-1-to-2-5/🎯️outcome/🔣️component.json"),
        },
        "change-imported-features" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥change-imported-features/🧪️tests/imports-harbor-position-descriptor/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥change-imported-features/🧪️tests/imports-harbor-position-descriptor/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥change-imported-features/🧪️tests/imports-harbor-position-descriptor/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥change-imported-features/🧪️tests/imports-harbor-position-descriptor/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥change-imported-features/🧪️tests/imports-harbor-position-descriptor/🎯️outcome/🔣️component.json"),
        },
        other => panic!("mutate-gisterrain-1: no committed specification vector is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-gisterrain-1: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, DERIVED_ASSET, DSL_ASSET, FIELDS, UNOBSERVABLE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_gis::artifacts::gisterrain::standards::v1::subsets::any::schema::mutations::gis_terrain_mutation_report_json;
    use semio_s_plugin_gis::artifacts::gisterrain::standards::v1::subsets::any::schema::snapshot::gis_terrain_identity_report_json;

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

    /// 📋️ A string member of the report, rejecting a present-but-wrong-shaped value.
    fn text(report: &Json, key: &str) -> Result<String, String> {
        match member(report, key)? {
            Json::String(value) => Ok(value.clone()),
            other => Err(format!("the report's {key:?} member is {}, not a string", other.to_string())),
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

    //#region 🔖️Projection
    /// 📤️ What parity compares: the two fields `GisTerrainSnapshot` declares. The `mesh` handle is
    /// deliberately outside it — its `childId` is a `std::hash::DefaultHasher` digest the standard
    /// library leaves unspecified — and is asserted exactly, in role, by [`spec_vector`].
    fn projection(document: &Json) -> Result<Json, String> {
        let mut entries = Vec::new();
        for name in FIELDS {
            let value = document.get(name).ok_or_else(|| format!("the document carries no {name:?} field"))?;
            entries.push(((*name).to_string(), value.clone()));
        }
        Ok(Json::Object(entries))
    }

    /// 🧭️ The one report the production bridge produces for a `(base, mutation)` pair. The bridge's
    /// third argument only feeds its `expectedSnapshot` member, which the real-document scenarios do
    /// not consult, so they pass the base for it.
    fn report_of(scenario: &str, base: &str, mutation: &str, after: &str) -> Result<Json, String> {
        parse_json(&gis_terrain_mutation_report_json(base, mutation, after).map_err(|error| format!("{scenario}: the input did not reach this subset's own codec: {error}"))?)
    }

    /// 🧫️ The declared fixture's bytes as UTF-8 text.
    fn fixture_text(ctx: &Context, uri: &str) -> Result<String, String> {
        String::from_utf8(ctx.fixture_bytes(uri)?).map_err(|error| format!("the declared fixture is not UTF-8: {error}"))
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎯️ Applies one kind to the REAL derived Liège terrain document with the parameters the feature
    /// states. The observability law is asserted here in role, so a setter that wrote the value the
    /// document already held cannot pass by agreeing with an unchanged document.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = fixture_text(ctx, DERIVED_ASSET)?;
            let report = report_of(&format!("mutate-{kind}"), &base, ctx.doc_string()?, &base)?;
            let applied = member(&report, "snapshot")?;
            let faults: Vec<String> = members(&report, "messages")?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect();
            if !faults.is_empty() {
                return Err(format!("mutate-{kind}: the feature's parameters were rejected with {faults:?}"));
            }
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            let projection = projection(applied)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ Applies one kind to the REAL derived document and then its OWN computed inverse. The
    /// projection carries BOTH documents: projecting only the restored one would make both rows
    /// project the same value and the differential would be vacuous.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = fixture_text(ctx, DERIVED_ASSET)?;
            let report = report_of(&format!("inverse-{kind}"), &base, ctx.doc_string()?, &base)?;
            let faults: Vec<String> = members(&report, "inverseMessages")?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect();
            if !faults.is_empty() {
                return Err(format!("inverse-{kind}: an inverse step was rejected with {faults:?}, so the document never got the chance to return"));
            }
            let applied = member(&report, "snapshot")?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            let restored = member(&report, "inverseSnapshot")?;
            law::inverse_restores(kind, restored, member(&report, "base")?)?;
            let projection = Json::Object(vec![("mutated".to_string(), projection(applied)?), ("restored".to_string(), projection(restored)?)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 📐️ Replays one committed handcrafted specification vector. This is where the evidence the case
    /// carried before the conversion still lives, undiminished: the applied document is held to the
    /// committed after-snapshot IN FULL — the re-derived `mesh` handle included — the produced delta
    /// to the committed `🔺️diff`, and the diagnostics to the committed `🎯️outcome`.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(kind);
            let report = report_of(&format!("spec-vector-{kind}"), committed.before, committed.mutation, committed.after)?;
            let applied = member(&report, "snapshot")?;
            if let Some(first) = law::divergence(applied, member(&report, "expectedSnapshot")?) {
                return Err(format!("spec-vector-{kind}: the applied document is not the committed after-snapshot — {first}"));
            }
            if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(committed.diff)) {
                return Err(format!("spec-vector-{kind}: the produced delta is not the committed 🔺️diff — {first}"));
            }
            declared_outcome_holds(kind, &members(&report, "messages")?, &canonical(committed.outcome))?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            law::inverse_restores(kind, member(&report, "inverseSnapshot")?, member(&report, "base")?)?;
            let projection = projection(applied)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ Two identities in one scenario, because they can only be asserted in two different places.
    ///
    /// The CARRIER identity is Rust-only and asserted here in role, on the artifact's own committed
    /// example: `law::round_trip_preserves` for the semantic half, and `law::carrier_is_exact` for
    /// the byte half — deliberately the fixpoint law rather than the wave's no-pass-through
    /// tripwire, because `store::ArtifactDsl` documents canonical `print_dsl` output as a `parse_dsl`
    /// fixpoint, so byte identity of the SECOND printing is the correct answer and anything else is
    /// the defect. The pack decoding is a separate binary codec, so agreeing on one snapshot cannot
    /// be reached by carrying text bytes across.
    ///
    /// The DOCUMENT identity is what the Python reference can also produce: the two declared fields
    /// this subset's own JSON codec reads out of the derived real document. The feature's doc string
    /// carries a `change-exaggeration` payload naming the exaggeration the document ALREADY holds,
    /// which is the only way to reach the bridge's `base` member — this subset's decode plus its
    /// `mesh` re-derivation — without applying an edit.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = fixture_text(ctx, DSL_ASSET)?;
        let report = parse_json(&gis_terrain_identity_report_json(&committed).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
        let parsed = member(&report, "parsed")?;
        law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
        law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
        if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
            return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
        }
        let derived = fixture_text(ctx, DERIVED_ASSET)?;
        let probe = report_of("identity-round-trip", &derived, ctx.doc_string()?, &derived)?;
        let base = member(&probe, "base")?;
        if let Some(first) = law::divergence(member(&probe, "snapshot")?, base) {
            return Err(format!("identity-round-trip: the doc string must name the value the derived document already holds, but applying it moved the document — {first}"));
        }
        let projection = projection(base)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
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
        return built.subject("identity-round-trip", subject::round_trip);
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = (KINDS, vector as fn(&str) -> Vector);
        built
    }
}
//#endregion 🔖️Registration
