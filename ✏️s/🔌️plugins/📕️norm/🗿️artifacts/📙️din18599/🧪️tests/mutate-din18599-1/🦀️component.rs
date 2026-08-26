//! 🦀️ DIN V 18599 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14 (the no-oracle conversion). The recorded
//! no-oracle decision `din18599-1-mutation-semantics` is gone from
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, because a reference now
//! exists to compare against: `s.norm.din18599` is a
//! semio-native artifact with no third-party reader or writer, so its reference is a second
//! IMPLEMENTATION: the independent Python `🐍️component.py` beside this file, registered as the
//! oracle `din18599-1-python-independent`. This adapter is the SUBJECT half only — it drives this
//! repository's own `apply_din18599_mutation` over the full 13-kind `Din18599Mutation`
//! vocabulary.
//!
//! Twelve document-root scalars — use class, heated area, occupants, the transmission and
//! ventilation heat-transfer coefficients H_T and H_V, internal and solar gains, system losses,
//! renewable yield, the annual primary-energy limit, the energy carrier and the reference Q_p —
//! each with its own `change-<field>` kind, plus ONE `update-climate`.
//!
//! ⚖️ WHERE THE ASSERTIONS LIVE. Every law this case claims is asserted IN ROLE inside the
//! subject handlers as well as being compared against the oracle's answer, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module (`law::mutation_is_observable`,
//! `law::inverse_restores`, `law::round_trip_preserves`, `law::carrier_is_exact`) that the
//! stdio mutation cases use, reached through the `oracleHostPackages` entry this plugin
//! declares in `✏️s/🔌️plugins/📕️norm/🧪️oracle/🔣️component.json`. What `parity` adds on top is the
//! one thing a single implementation can never provide: that a second implementation, written in
//! another language from the same written specification, reaches the same document.
//!
//! 🌉️ HOW THE FIXTURES REACH TYPED VALUES. The generated test host links only
//! `semio-repo-test-host`, the stdio law crate and — behind `sut` — this plugin's own crate;
//! `serde`, `serde_json` and this crate's `protocol`/`store`/`vcs` extern-crate aliases are all
//! unreachable from here. The subset's own production code therefore exports the bridges
//! (`decode_din18599_snapshot_json`/`encode_din18599_snapshot_json`,
//! `decode_din18599_dsl`/`encode_din18599_dsl`, `decode_din18599_pack`/`encode_din18599_pack`
//! in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`;
//! `decode_din18599_mutation_json`, `apply_din18599_mutation`, `inverse_din18599_mutation` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`), whose
//! signatures name only reachable types. This side reaches the committed vectors through
//! `include_str!` and the Python side through the `asset://` URIs the feature declares, so both
//! read the SAME committed bytes and neither holds a Rust or Python literal transcribed beside
//! them that could drift from what the other one read.
//!
//! 🚧️ The Rust SUBJECT phase still cannot run: `semio-s-plugin-norm` does not compile (671 errors
//! at the time of writing — a concurrent session is mid-flight across ~2000 files of this plugin,
//! removing gratuitous `async fn` wrappers). `parity` therefore has nothing to compare the oracle
//! against YET; the moment the crate is green it does, with no further change here. The subject half is
//! written against the SYNC trait surface the fixture tests in this crate already call
//! (`Mutation::diff`, `MutationDiff::apply`, `Mutation::inverse`, `ArtifactDsl`,
//! `ArtifactPack`) rather than against the plugin's async wrappers, and is `sut`-gated so the
//! oracle-only run never links it.

use semio_repo_test_host::{digest, parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Mirrors `Din18599Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-use-class",
    "change-heated-area-m2",
    "change-occupants",
    "change-ht",
    "change-hv",
    "change-internal-gains-wm2",
    "change-solar-gains-kwh",
    "change-system-losses-kwh",
    "change-renewable-kwh",
    "change-annual-limit-kwh",
    "change-energy-carrier",
    "change-reference-qp-kwh",
    "update-climate",
];

/// 🗣️ The real committed DIN V 18599 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-use-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-use-class/🧪️tests/reclassifies-the-building-as-an-office/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-use-class/🧪️tests/reclassifies-the-building-as-an-office/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-use-class/🧪️tests/reclassifies-the-building-as-an-office/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-use-class/🧪️tests/reclassifies-the-building-as-an-office/🎯️outcome/🔣️component.json"),
        ),
        "change-heated-area-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-heated-area-m2/🧪️tests/extends-the-heated-area-to-160-m2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-heated-area-m2/🧪️tests/extends-the-heated-area-to-160-m2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-heated-area-m2/🧪️tests/extends-the-heated-area-to-160-m2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-heated-area-m2/🧪️tests/extends-the-heated-area-to-160-m2/🎯️outcome/🔣️component.json"),
        ),
        "change-occupants" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-occupants/🧪️tests/raises-the-occupancy-to-six-people/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-occupants/🧪️tests/raises-the-occupancy-to-six-people/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-occupants/🧪️tests/raises-the-occupancy-to-six-people/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-occupants/🧪️tests/raises-the-occupancy-to-six-people/🎯️outcome/🔣️component.json"),
        ),
        "change-ht" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-ht/🧪️tests/raises-the-transmission-loss-coefficient-to-118-w-per-k/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-ht/🧪️tests/raises-the-transmission-loss-coefficient-to-118-w-per-k/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-ht/🧪️tests/raises-the-transmission-loss-coefficient-to-118-w-per-k/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-ht/🧪️tests/raises-the-transmission-loss-coefficient-to-118-w-per-k/🎯️outcome/🔣️component.json"),
        ),
        "change-hv" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-hv/🧪️tests/raises-the-ventilation-loss-coefficient-to-52-25-w-per-k/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-hv/🧪️tests/raises-the-ventilation-loss-coefficient-to-52-25-w-per-k/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-hv/🧪️tests/raises-the-ventilation-loss-coefficient-to-52-25-w-per-k/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-hv/🧪️tests/raises-the-ventilation-loss-coefficient-to-52-25-w-per-k/🎯️outcome/🔣️component.json"),
        ),
        "change-internal-gains-wm2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-internal-gains-wm2/🧪️tests/raises-the-internal-gains-to-5-w-per-m2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-internal-gains-wm2/🧪️tests/raises-the-internal-gains-to-5-w-per-m2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-internal-gains-wm2/🧪️tests/raises-the-internal-gains-to-5-w-per-m2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-internal-gains-wm2/🧪️tests/raises-the-internal-gains-to-5-w-per-m2/🎯️outcome/🔣️component.json"),
        ),
        "change-solar-gains-kwh" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦥change-solar-gains-kwh/🧪️tests/raises-the-annual-solar-gains-to-132-kwh/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦥change-solar-gains-kwh/🧪️tests/raises-the-annual-solar-gains-to-132-kwh/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦥change-solar-gains-kwh/🧪️tests/raises-the-annual-solar-gains-to-132-kwh/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦥change-solar-gains-kwh/🧪️tests/raises-the-annual-solar-gains-to-132-kwh/🎯️outcome/🔣️component.json"),
        ),
        "change-system-losses-kwh" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦦change-system-losses-kwh/🧪️tests/cuts-the-system-losses-to-450-kwh/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦦change-system-losses-kwh/🧪️tests/cuts-the-system-losses-to-450-kwh/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦦change-system-losses-kwh/🧪️tests/cuts-the-system-losses-to-450-kwh/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦦change-system-losses-kwh/🧪️tests/cuts-the-system-losses-to-450-kwh/🎯️outcome/🔣️component.json"),
        ),
        "change-renewable-kwh" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦨change-renewable-kwh/🧪️tests/raises-the-on-site-renewable-yield-to-2250-kwh/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦨change-renewable-kwh/🧪️tests/raises-the-on-site-renewable-yield-to-2250-kwh/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦨change-renewable-kwh/🧪️tests/raises-the-on-site-renewable-yield-to-2250-kwh/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦨change-renewable-kwh/🧪️tests/raises-the-on-site-renewable-yield-to-2250-kwh/🎯️outcome/🔣️component.json"),
        ),
        "change-annual-limit-kwh" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦡change-annual-limit-kwh/🧪️tests/tightens-the-annual-primary-energy-limit-to-6000-kwh/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦡change-annual-limit-kwh/🧪️tests/tightens-the-annual-primary-energy-limit-to-6000-kwh/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦡change-annual-limit-kwh/🧪️tests/tightens-the-annual-primary-energy-limit-to-6000-kwh/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦡change-annual-limit-kwh/🧪️tests/tightens-the-annual-primary-energy-limit-to-6000-kwh/🎯️outcome/🔣️component.json"),
        ),
        "change-energy-carrier" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-energy-carrier/🧪️tests/switches-the-energy-carrier-to-an-electric-heat-pump/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-energy-carrier/🧪️tests/switches-the-energy-carrier-to-an-electric-heat-pump/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-energy-carrier/🧪️tests/switches-the-energy-carrier-to-an-electric-heat-pump/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-energy-carrier/🧪️tests/switches-the-energy-carrier-to-an-electric-heat-pump/🎯️outcome/🔣️component.json"),
        ),
        "change-reference-qp-kwh" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-reference-qp-kwh/🧪️tests/lowers-the-reference-building-primary-energy-to-8750-kwh/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-reference-qp-kwh/🧪️tests/lowers-the-reference-building-primary-energy-to-8750-kwh/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-reference-qp-kwh/🧪️tests/lowers-the-reference-building-primary-energy-to-8750-kwh/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-reference-qp-kwh/🧪️tests/lowers-the-reference-building-primary-energy-to-8750-kwh/🎯️outcome/🔣️component.json"),
        ),
        "update-climate" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘update-climate/🧪️tests/refuses-a-negative-january-irradiance/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘update-climate/🧪️tests/refuses-a-negative-january-irradiance/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘update-climate/🧪️tests/refuses-a-negative-january-irradiance/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘update-climate/🧪️tests/refuses-a-negative-january-irradiance/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-din18599-1: no committed fixture is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}

/// 🎯️ The status the committed `🎯️outcome/🔣️component.json` declares for one kind — `applied` or
/// `rejected` — read out of the committed file rather than transcribed beside it, so the contract a
/// row is held to cannot drift away from the vector that states it.
#[cfg(feature = "sut")]
fn committed_status(kind: &str) -> String {
    let (_before, _mutation, _after, outcome) = fixture_text(kind);
    canonical(outcome).str("status")
}
//#endregion 🔖️Fixtures

//#region 🔖️Carrier
/// 🧵️ The canonical carrier bytes as a comparable projection: the envelope preamble, every body line
/// as written, and the digest and length of what was emitted. `.dsl.semio` has no grammar document in
/// this repository — the committed `📖️component.grammar.semio` is the repository-wide `payload = OCTET+`
/// placeholder — so the identity scenario compares the two implementations at the carrier level rather
/// than mapping carrier tokens onto the snapshot's enum spellings, a mapping nothing states. The
/// independent Python implementation builds the identical shape from ITS re-emission, and `digest` is
/// the coordinator's own sha256, so the two languages' bytes are directly comparable.
#[cfg(feature = "sut")]
fn carrier_projection(text: &str) -> Json {
    let (preamble, body) = text.split_once('\n').unwrap_or((text, ""));
    let body = body.strip_suffix('\n').unwrap_or(body);
    let lines = if body.is_empty() { Vec::new() } else { body.split('\n').map(|line| Json::String(line.to_string())).collect::<Vec<Json>>() };
    Json::Object(vec![
        ("preamble".to_string(), Json::String(preamble.to_string())),
        ("lines".to_string(), Json::Array(lines)),
        ("dslDigest".to_string(), Json::String(digest(text.as_bytes()))),
        ("dslLength".to_string(), Json::Number(text.as_bytes().len() as f64)),
    ])
}
//#endregion 🔖️Carrier

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_norm::artifacts::din18599::standards::v1::subsets::any::schema::mutations::{apply_din18599_mutation, decode_din18599_mutation_json, inverse_din18599_mutation, Din18599Mutation};
    use semio_s_plugin_norm::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::{decode_din18599_dsl, decode_din18599_pack, decode_din18599_snapshot_json, encode_din18599_dsl, encode_din18599_pack, encode_din18599_snapshot_json, Din18599Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<Din18599Snapshot, String> {
        decode_din18599_snapshot_json(text).map_err(|error| format!("mutate-din18599-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<Din18599Mutation, String> {
        decode_din18599_mutation_json(text).map_err(|error| format!("mutate-din18599-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &Din18599Snapshot) -> Result<Json, String> {
        parse_json(&encode_din18599_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &Din18599Snapshot, expected: &Din18599Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_din18599_snapshot_json(got), encode_din18599_snapshot_json(expected))
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot, under whichever contract the committed `🎯️outcome` declares: an `applied`
    /// vector must be accepted without a diagnostic and must move the projection (`law::
    /// mutation_is_observable`), a `rejected` one must raise a diagnostic and leave the document
    /// bit-identical. A handler that merely returned `Ok` would report a pass having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let status = super::committed_status(kind);
            let applied = apply_din18599_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_din18599_snapshot_json(&snapshot))),
                ("rejected", Ok((snapshot, _messages))) => snapshot,
                ("rejected", Err(_error)) => base.clone(),
                (other, _) => return Err(format!("mutate-{kind}: unknown committed outcome status {other:?}")),
            };
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied document does not match the committed after-snapshot"), &current, &expected));
            }
            let (base_projection, mutated) = (projection(&base)?, projection(&current)?);
            if status == "applied" {
                law::mutation_is_observable(kind, &mutated, &base_projection, &[])?;
            } else if law::divergence(&mutated, &base_projection).is_some() {
                return Err(disagreement(&format!("mutate-{kind}: a rejected mutation must leave the document untouched"), &current, &base));
            }
            Ok(Outcome::with_raw(mutated.to_string().into_bytes(), mutated))
        }
    }

    /// ↩️ The metamorphic inverse law, asserted in role through `law::inverse_restores`: applying the
    /// kind and then its OWN computed inverse must restore the committed before-snapshot exactly —
    /// collection POSITION included, not merely membership. A kind the committed outcome declares
    /// `applied` must additionally produce a non-empty inverse, because a mutation that changes the
    /// document and reports nothing to undo silently breaks the event-sourced undo history.
    /// The projection carries BOTH the mutated and the restored document: projecting only the restored
    /// one would make every row of the table project the same value and the differential vacuous.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let original = projection(&base)?;
            let mut current = match apply_din18599_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_din18599_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_din18599_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
            }
            let restored = projection(&current)?;
            law::inverse_restores(kind, &restored, &original)?;
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            let projection = Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), restored)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real committed document through every encoding it has. The DSL carrier is deliberately
    /// byte-preserving here — the committed file IS this printer's own canonical output — so
    /// `law::carrier_is_exact` is the correct half of the identity law and the usual
    /// no-byte-pass-through inequality would be the wrong claim. What proves the document was PARSED
    /// rather than copied is the agreement of three independently written codecs: the hand-written
    /// DSL grammar, the hand-written binary pack protocol, and the JSON projection. A shortcut that
    /// handed back its input bytes could not survive the pack leg.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed DIN V 18599 artifact is not UTF-8: {error}"))?;
        let parsed = decode_din18599_dsl(&text)?;
        let reprinted = encode_din18599_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_din18599_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_din18599_pack(&encode_din18599_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_din18599_snapshot_json(&encode_din18599_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        law::round_trip_preserves(&projection(&repacked)?, &projection(&parsed)?)?;
        Ok(Outcome::with_raw(reprinted.as_bytes().to_vec(), carrier_projection(&reprinted)))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. SUBJECT role only: the reference
/// answer now comes from the independent Python implementation registered as this subset's oracle, and
/// registering this repository's own answer on the oracle side as well would compare it with
/// itself.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
