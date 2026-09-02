//! 🗺️ `s.gis.gismap` exhaustive mutation case — Rust SUBJECT adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` beside this file —
//! a second implementation of the document and all twelve typed mutations, written in Python from
//! this subset's committed schema, grammar and specification vectors. This adapter therefore
//! registers the SUBJECT half only: keeping oracle registrations here would put this repository's
//! answer on both sides of the comparison, which is the precise failure the platform exists to
//! prevent.
//!
//! Twelve kinds are four verbs over three PARALLEL id-keyed collections, and the parallelism is a
//! specification — `📓️derivation-rules.md`'s per-id-keyed-collection recipe — not a copy. A
//! `MapFeature` is an id plus an opaque `dsl::DslValue` the artifact never interprets, so
//! `replace-<noun>-data` swaps a whole untyped value. Order is meaningful and both the real-document
//! parameters and the three committed reorder vectors exercise different displacements.
//!
//! **What is compared across the two languages, and what is asserted in role.** The cross-language
//! projection is the three `x-semio-state: artifact` collections the committed JSON Schema declares.
//! `drawing` and `value` are composed children content-addressed with `std::hash::DefaultHasher`,
//! whose output the standard library leaves UNSPECIFIED, so no second implementation in another
//! language can reproduce them. They are still held exactly here, in role, by
//! [`subject::spec_vector`] against the committed after-snapshot — together with the committed
//! `🔺️diff`, which pins WHICH fields the mutation was allowed to touch, and the committed
//! `🎯️outcome`. Nothing was relaxed by the conversion: no comparison profile changed, no
//! `ignoreKeys` was added, and the diff and outcome checks this case already made are all still here.

use semio_repo_test_host::{parse_json, Adapter, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and that file's own
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against both the enum and the manifest.
const KINDS: &[&str] = &[
    "create-position",
    "delete-position",
    "replace-position-data",
    "reorder-positions",
    "create-route",
    "delete-route",
    "replace-route-data",
    "reorder-routes",
    "create-region",
    "delete-region",
    "replace-region-data",
    "reorder-regions",
];

/// 🗂️ The three collections `GisMapSnapshot` declares, in schema order — the cross-language projection.
const COLLECTIONS: &[&str] = &["positions", "routes", "regions"];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect, so
/// [`law::mutation_is_observable`] must not demand one of them.
/// Empty: all twelve committed vectors move a feature collection, so every forward effect is visible.
const UNOBSERVABLE: &[&str] = &[];

/// 🗣️ The real committed document this artifact ships as its own example — a Liège fragment with the
/// Institut de Botanique and Lycée Block 3000 at their true coordinates.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";

/// 🧫️ That same document with three regions added, each the axis-aligned envelope of geometry
/// already inside it. Derived once, provenance recorded in the feature description, because the
/// committed example carries no regions and three of the twelve kinds address an existing one.
#[cfg(feature = "sut")]
const DERIVED_ASSET: &str = "local://🗺️liege-with-derived-regions.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ One kind's committed `(before, mutation, after, diff, outcome)` specification vector, read
/// literally via `include_str!`. The Python reference reads the same three snapshot and mutation
/// files through the plan's declared fixtures; the `🔺️diff` and `🎯️outcome` members are checked on
/// this side only, because they are Rust-side report shapes rather than parts of the document.
struct Vector {
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    diff: &'static str,
    outcome: &'static str,
}

fn vector(kind: &str) -> Vector {
    match kind {
        "create-position" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/🧪️tests/adds-lighthouse-position-after-harbor/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/🧪️tests/adds-lighthouse-position-after-harbor/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/🧪️tests/adds-lighthouse-position-after-harbor/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/🧪️tests/adds-lighthouse-position-after-harbor/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/🧪️tests/adds-lighthouse-position-after-harbor/🎯️outcome/🔣️.json"),
        },
        "delete-position" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-position/🧪️tests/removes-lighthouse-position/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-position/🧪️tests/removes-lighthouse-position/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-position/🧪️tests/removes-lighthouse-position/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-position/🧪️tests/removes-lighthouse-position/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-position/🧪️tests/removes-lighthouse-position/🎯️outcome/🔣️.json"),
        },
        "replace-position-data" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/🧪️tests/rewrites-harbor-position-payload/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/🧪️tests/rewrites-harbor-position-payload/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/🧪️tests/rewrites-harbor-position-payload/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/🧪️tests/rewrites-harbor-position-payload/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/🧪️tests/rewrites-harbor-position-payload/🎯️outcome/🔣️.json"),
        },
        "reorder-positions" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/🧪️tests/moves-harbor-position-to-end/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/🧪️tests/moves-harbor-position-to-end/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/🧪️tests/moves-harbor-position-to-end/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/🧪️tests/moves-harbor-position-to-end/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/🧪️tests/moves-harbor-position-to-end/🎯️outcome/🔣️.json"),
        },
        "create-route" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️create-route/🧪️tests/adds-tram-route-after-ferry/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️create-route/🧪️tests/adds-tram-route-after-ferry/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️create-route/🧪️tests/adds-tram-route-after-ferry/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️create-route/🧪️tests/adds-tram-route-after-ferry/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️create-route/🧪️tests/adds-tram-route-after-ferry/🎯️outcome/🔣️.json"),
        },
        "delete-route" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-route/🧪️tests/removes-tram-route/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-route/🧪️tests/removes-tram-route/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-route/🧪️tests/removes-tram-route/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-route/🧪️tests/removes-tram-route/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-route/🧪️tests/removes-tram-route/🎯️outcome/🔣️.json"),
        },
        "replace-route-data" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-route-data/🧪️tests/rewrites-ferry-route-payload/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-route-data/🧪️tests/rewrites-ferry-route-payload/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-route-data/🧪️tests/rewrites-ferry-route-payload/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-route-data/🧪️tests/rewrites-ferry-route-payload/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-route-data/🧪️tests/rewrites-ferry-route-payload/🎯️outcome/🔣️.json"),
        },
        "reorder-routes" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭reorder-routes/🧪️tests/moves-bus-route-to-front/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭reorder-routes/🧪️tests/moves-bus-route-to-front/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭reorder-routes/🧪️tests/moves-bus-route-to-front/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭reorder-routes/🧪️tests/moves-bus-route-to-front/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭reorder-routes/🧪️tests/moves-bus-route-to-front/🎯️outcome/🔣️.json"),
        },
        "create-region" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐create-region/🧪️tests/adds-old-town-region-after-harbor-district/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐create-region/🧪️tests/adds-old-town-region-after-harbor-district/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐create-region/🧪️tests/adds-old-town-region-after-harbor-district/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐create-region/🧪️tests/adds-old-town-region-after-harbor-district/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐create-region/🧪️tests/adds-old-town-region-after-harbor-district/🎯️outcome/🔣️.json"),
        },
        "delete-region" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-region/🧪️tests/removes-old-town-region/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-region/🧪️tests/removes-old-town-region/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-region/🧪️tests/removes-old-town-region/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-region/🧪️tests/removes-old-town-region/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-region/🧪️tests/removes-old-town-region/🎯️outcome/🔣️.json"),
        },
        "replace-region-data" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-region-data/🧪️tests/rewrites-harbor-district-region-payload/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-region-data/🧪️tests/rewrites-harbor-district-region-payload/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-region-data/🧪️tests/rewrites-harbor-district-region-payload/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-region-data/🧪️tests/rewrites-harbor-district-region-payload/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-region-data/🧪️tests/rewrites-harbor-district-region-payload/🎯️outcome/🔣️.json"),
        },
        "reorder-regions" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-regions/🧪️tests/moves-park-region-between-2-districts/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-regions/🧪️tests/moves-park-region-between-2-districts/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-regions/🧪️tests/moves-park-region-between-2-districts/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-regions/🧪️tests/moves-park-region-between-2-districts/🔺️diff/🔣️.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-regions/🧪️tests/moves-park-region-between-2-districts/🎯️outcome/🔣️.json"),
        },
        other => panic!("mutate-gismap-1: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, COLLECTIONS, DERIVED_ASSET, DSL_ASSET, UNOBSERVABLE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::mutations::gis_map_mutation_report_json;
    use semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::snapshot::gis_map_identity_report_json;
    use semio_s_plugin_stdio_test_oracle::law;

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
                return Err(format!("spec-vector-{kind}: the vector declares a rejection with code {expected:?}, the implementation raised {codes:?}"));
            }
            if !levels.iter().any(|level| level == "error" || level == "fatal") {
                return Err(format!("spec-vector-{kind}: the vector declares a rejection, but the implementation raised it at {levels:?} — a rejection is at least an error"));
            }
            let path = strings(outcome, "path");
            let target = strings(&produced[0], "target");
            if !path.is_empty() && target != path {
                return Err(format!("spec-vector-{kind}: the vector declares the offending address {path:?}, the implementation reported {target:?}"));
            }
            return Ok(());
        }
        let expected: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).collect();
        if codes != expected {
            return Err(format!("spec-vector-{kind}: the vector declares the diagnostics {expected:?}, the implementation raised {codes:?}"));
        }
        match levels.iter().find(|level| level.as_str() == "error" || level.as_str() == "fatal") {
            Some(level) => Err(format!("spec-vector-{kind}: the vector declares an applied outcome, but the implementation raised a {level}")),
            None => Ok(()),
        }
    }
    //#endregion 🔖️Report

    //#region 🔖️Projection
    /// 📤️ What parity compares: the three `x-semio-state: artifact` collections the committed JSON
    /// Schema declares. The composed `drawing` and `value` handles are deliberately outside it —
    /// their `childId` is a `std::hash::DefaultHasher` digest the standard library leaves
    /// unspecified — and are asserted exactly, in role, by [`spec_vector`] instead.
    fn projection(document: &Json) -> Result<Json, String> {
        let mut entries = Vec::new();
        for name in COLLECTIONS {
            let value = document.get(name).ok_or_else(|| format!("the document carries no {name:?} collection"))?;
            entries.push(((*name).to_string(), value.clone()));
        }
        Ok(Json::Object(entries))
    }

    /// 🧭️ The one report the production bridge produces for a `(base, mutation)` pair. The bridge's
    /// third argument only feeds its `expectedSnapshot` member, which the real-document scenarios do
    /// not consult, so they pass the base for it.
    fn report_of(kind: &str, base: &str, mutation: &str, after: &str) -> Result<Json, String> {
        parse_json(&gis_map_mutation_report_json(base, mutation, after).map_err(|error| format!("{kind}: the input did not reach this subset's own codec: {error}"))?)
    }

    /// 📖️ The real committed carrier document, parsed by this subset's own DSL codec into the JSON
    /// text the mutation bridge takes.
    fn parsed_document(uri: &str, ctx: &Context) -> Result<String, String> {
        let committed = String::from_utf8(ctx.fixture_bytes(uri)?).map_err(|error| format!("the declared document is not UTF-8: {error}"))?;
        let report = parse_json(&gis_map_identity_report_json(&committed).map_err(|error| format!("the declared document did not reach this subset's own codec: {error}"))?)?;
        Ok(member(&report, "parsed")?.to_string())
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎯️ Applies one kind to the REAL Liège document with the parameters the feature states, and
    /// answers with the three declared collections. The observability law is asserted here in role,
    /// so a mutation that quietly did nothing cannot pass by agreeing with an unchanged document.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = parsed_document(DERIVED_ASSET, ctx)?;
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

    /// ↩️ Applies one kind to the REAL Liège document and then its OWN computed inverse. The
    /// projection carries BOTH documents: projecting only the restored one would make all twelve rows
    /// project the same value and the differential would be vacuous.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = parsed_document(DERIVED_ASSET, ctx)?;
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
    /// committed after-snapshot IN FULL — composed `drawing` and `value` handles included — the
    /// produced delta to the committed `🔺️diff`, and the diagnostics to the committed `🎯️outcome`.
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

    /// 🔁️ The real committed document through this subset's own two codecs. The semantic half is
    /// `law::round_trip_preserves`: parsing, printing back and parsing again must not move the
    /// projection. The byte half is `law::carrier_is_exact` rather than the wave's usual
    /// no-pass-through tripwire, and deliberately so — `store::ArtifactDsl`'s own documented LAW is that
    /// canonical `print_dsl` output is a `parse_dsl` fixpoint, so the correct answer for a second
    /// printing IS byte identity and anything else is the defect. The pack decoding is a separate
    /// binary codec, so agreeing on one snapshot cannot be reached by carrying text bytes across.
    /// The projection is what the Python reference read out of the SAME committed bytes.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let report = parse_json(&gis_map_identity_report_json(&committed).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
        let parsed = member(&report, "parsed")?;
        law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
        law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
        if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
            return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
        }
        let projection = projection(parsed)?;
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
