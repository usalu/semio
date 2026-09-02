//! 🦀️ EN 1991 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14 (the no-oracle conversion). The recorded
//! no-oracle decision `en1991-1-mutation-semantics` is gone from
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`, because a reference now
//! exists to compare against: `s.norm.en1991` is a
//! semio-native artifact with no third-party reader or writer, so its reference is a second
//! IMPLEMENTATION: the independent Python `🐍️component.py` beside this file, registered as the
//! oracle `en1991-1-python-independent`. This adapter is the SUBJECT half only — it drives this
//! repository's own `apply_en1991_mutation` over the full 32-kind `En1991Mutation` vocabulary.
//!
//! Thirty-two document-root scalars, one `change-<field>` each, spanning the whole of Eurocode
//! 1: loaded area and imposed-load category, national annex, self-weight (material and layer
//! thickness, plus an assumed characteristic value), fire (curve, required resistance, member
//! capacity), snow (zone, altitude, characteristic load), wind (zone, basic speed), thermal
//! delta T, construction activity, accidental impact (vehicle mass and speed), bridge traffic
//! (notional lanes, span, lane width, moment resistance), crane and hoist classes with hoisting
//! speed, silo bulk material (density, height, hydraulic radius, wall friction mu, lateral
//! pressure ratio K) and the size and dynamic factors c_s and c_d.
//!
//! ⚖️ WHERE THE ASSERTIONS LIVE. Every law this case claims is asserted IN ROLE inside the
//! subject handlers as well as being compared against the oracle's answer, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module (`law::mutation_is_observable`,
//! `law::inverse_restores`, `law::round_trip_preserves`, `law::carrier_is_exact`) that the
//! stdio mutation cases use, reached through the `oracleHostPackages` entry this plugin
//! declares in `✏️s/🔌️plugins/📕️norm/🔣️oracle.json`. What `parity` adds on top is the
//! one thing a single implementation can never provide: that a second implementation, written in
//! another language from the same written specification, reaches the same document.
//!
//! 🌉️ HOW THE FIXTURES REACH TYPED VALUES. The generated test host links only
//! `semio-repo-test-host`, the stdio law crate and — behind `sut` — this plugin's own crate;
//! `serde`, `serde_json` and this crate's `protocol`/`store`/`vcs` extern-crate aliases are all
//! unreachable from here. The subset's own production code therefore exports the bridges
//! (`decode_en1991_snapshot_json`/`encode_en1991_snapshot_json`,
//! `decode_en1991_dsl`/`encode_en1991_dsl`, `decode_en1991_pack`/`encode_en1991_pack` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`;
//! `decode_en1991_mutation_json`, `apply_en1991_mutation`, `inverse_en1991_mutation` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`), whose
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
/// 🏷️ Mirrors `En1991Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-area-m2",
    "change-category",
    "change-annex",
    "change-self-weight-material",
    "change-self-weight-thickness-m",
    "change-assumed-gk-kn-m2",
    "change-fire-curve",
    "change-fire-resistance-min",
    "change-fire-member-capacity-c",
    "change-snow-zone",
    "change-snow-altitude-m",
    "change-en-sk-kn-m2",
    "change-wind-zone",
    "change-en-vbms",
    "change-delta-tk",
    "change-construction-activity",
    "change-accidental-mass-t",
    "change-accidental-speed-km-h",
    "change-bridge-lane",
    "change-bridge-span-m",
    "change-bridge-lane-width-m",
    "change-bridge-moment-resistance-knm",
    "change-crane-class",
    "change-hoist-class",
    "change-hoisting-speed-ms",
    "change-silo-bulk-density-kn-m3",
    "change-silo-height-m",
    "change-silo-hydraulic-radius-m",
    "change-silo-mu",
    "change-silo-k",
    "change-cs",
    "change-cd",
];

/// 🗣️ The real committed EN 1991 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🧪️retail-hydrocarbon-fire/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🎒️.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-area-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🧪️tests/enlarges-loaded-area-to-360-m2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🧪️tests/enlarges-loaded-area-to-360-m2/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🧪️tests/enlarges-loaded-area-to-360-m2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🧪️tests/enlarges-loaded-area-to-360-m2/🎯️outcome/🔣️.json"),
        ),
        "change-category" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🧪️tests/reclassifies-imposed-load-to-category-d/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🧪️tests/reclassifies-imposed-load-to-category-d/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🧪️tests/reclassifies-imposed-load-to-category-d/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🧪️tests/reclassifies-imposed-load-to-category-d/🎯️outcome/🔣️.json"),
        ),
        "change-annex" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🧪️tests/switches-national-annex-to-en/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🧪️tests/switches-national-annex-to-en/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🧪️tests/switches-national-annex-to-en/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🧪️tests/switches-national-annex-to-en/🎯️outcome/🔣️.json"),
        ),
        "change-self-weight-material" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🧪️tests/switches-self-weight-material-to-structural-steel/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🧪️tests/switches-self-weight-material-to-structural-steel/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🧪️tests/switches-self-weight-material-to-structural-steel/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🧪️tests/switches-self-weight-material-to-structural-steel/🎯️outcome/🔣️.json"),
        ),
        "change-self-weight-thickness-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🧪️tests/thickens-self-weight-layer-to-0-375-m/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🧪️tests/thickens-self-weight-layer-to-0-375-m/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🧪️tests/thickens-self-weight-layer-to-0-375-m/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🧪️tests/thickens-self-weight-layer-to-0-375-m/🎯️outcome/🔣️.json"),
        ),
        "change-assumed-gk-kn-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🧪️tests/raises-assumed-gk-to-7-5-kn-m2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🧪️tests/raises-assumed-gk-to-7-5-kn-m2/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🧪️tests/raises-assumed-gk-to-7-5-kn-m2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🧪️tests/raises-assumed-gk-to-7-5-kn-m2/🎯️outcome/🔣️.json"),
        ),
        "change-fire-curve" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🧪️tests/switches-fire-curve-to-hydrocarbon/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🧪️tests/switches-fire-curve-to-hydrocarbon/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🧪️tests/switches-fire-curve-to-hydrocarbon/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🧪️tests/switches-fire-curve-to-hydrocarbon/🎯️outcome/🔣️.json"),
        ),
        "change-fire-resistance-min" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🧪️tests/extends-fire-resistance-to-120-min/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🧪️tests/extends-fire-resistance-to-120-min/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🧪️tests/extends-fire-resistance-to-120-min/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🧪️tests/extends-fire-resistance-to-120-min/🎯️outcome/🔣️.json"),
        ),
        "change-fire-member-capacity-c" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🧪️tests/raises-fire-member-capacity-to-700-c/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🧪️tests/raises-fire-member-capacity-to-700-c/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🧪️tests/raises-fire-member-capacity-to-700-c/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🧪️tests/raises-fire-member-capacity-to-700-c/🎯️outcome/🔣️.json"),
        ),
        "change-snow-zone" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🧪️tests/moves-site-to-snow-zone-3/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🧪️tests/moves-site-to-snow-zone-3/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🧪️tests/moves-site-to-snow-zone-3/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🧪️tests/moves-site-to-snow-zone-3/🎯️outcome/🔣️.json"),
        ),
        "change-snow-altitude-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🧪️tests/lifts-snow-altitude-to-780-m/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🧪️tests/lifts-snow-altitude-to-780-m/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🧪️tests/lifts-snow-altitude-to-780-m/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🧪️tests/lifts-snow-altitude-to-780-m/🎯️outcome/🔣️.json"),
        ),
        "change-en-sk-kn-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🧪️tests/raises-en-characteristic-snow-load-to-1-25-kn-m2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🧪️tests/raises-en-characteristic-snow-load-to-1-25-kn-m2/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🧪️tests/raises-en-characteristic-snow-load-to-1-25-kn-m2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🧪️tests/raises-en-characteristic-snow-load-to-1-25-kn-m2/🎯️outcome/🔣️.json"),
        ),
        "change-wind-zone" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🧪️tests/moves-site-to-wind-zone-4/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🧪️tests/moves-site-to-wind-zone-4/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🧪️tests/moves-site-to-wind-zone-4/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🧪️tests/moves-site-to-wind-zone-4/🎯️outcome/🔣️.json"),
        ),
        "change-en-vbms" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🧪️tests/raises-en-basic-wind-speed-to-30-m-s/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🧪️tests/raises-en-basic-wind-speed-to-30-m-s/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🧪️tests/raises-en-basic-wind-speed-to-30-m-s/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🧪️tests/raises-en-basic-wind-speed-to-30-m-s/🎯️outcome/🔣️.json"),
        ),
        "change-delta-tk" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🧪️tests/raises-thermal-delta-tk-to-45-k/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🧪️tests/raises-thermal-delta-tk-to-45-k/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🧪️tests/raises-thermal-delta-tk-to-45-k/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🧪️tests/raises-thermal-delta-tk-to-45-k/🎯️outcome/🔣️.json"),
        ),
        "change-construction-activity" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🧪️tests/switches-construction-activity-to-concreting/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🧪️tests/switches-construction-activity-to-concreting/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🧪️tests/switches-construction-activity-to-concreting/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🧪️tests/switches-construction-activity-to-concreting/🎯️outcome/🔣️.json"),
        ),
        "change-accidental-mass-t" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🧪️tests/lightens-impact-vehicle-to-12-5-t/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🧪️tests/lightens-impact-vehicle-to-12-5-t/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🧪️tests/lightens-impact-vehicle-to-12-5-t/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🧪️tests/lightens-impact-vehicle-to-12-5-t/🎯️outcome/🔣️.json"),
        ),
        "change-accidental-speed-km-h" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🧪️tests/lowers-impact-speed-to-50-km-h/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🧪️tests/lowers-impact-speed-to-50-km-h/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🧪️tests/lowers-impact-speed-to-50-km-h/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🧪️tests/lowers-impact-speed-to-50-km-h/🎯️outcome/🔣️.json"),
        ),
        "change-bridge-lane" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🧪️tests/widens-carriageway-to-3-notional-lanes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🧪️tests/widens-carriageway-to-3-notional-lanes/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🧪️tests/widens-carriageway-to-3-notional-lanes/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🧪️tests/widens-carriageway-to-3-notional-lanes/🎯️outcome/🔣️.json"),
        ),
        "change-bridge-span-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🧪️tests/lengthens-bridge-span-to-36-m/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🧪️tests/lengthens-bridge-span-to-36-m/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🧪️tests/lengthens-bridge-span-to-36-m/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🧪️tests/lengthens-bridge-span-to-36-m/🎯️outcome/🔣️.json"),
        ),
        "change-bridge-lane-width-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🧪️tests/widens-notional-lane-to-3-5-m/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🧪️tests/widens-notional-lane-to-3-5-m/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🧪️tests/widens-notional-lane-to-3-5-m/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🧪️tests/widens-notional-lane-to-3-5-m/🎯️outcome/🔣️.json"),
        ),
        "change-bridge-moment-resistance-knm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🧪️tests/raises-bridge-moment-resistance-to-4500-knm/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🧪️tests/raises-bridge-moment-resistance-to-4500-knm/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🧪️tests/raises-bridge-moment-resistance-to-4500-knm/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🧪️tests/raises-bridge-moment-resistance-to-4500-knm/🎯️outcome/🔣️.json"),
        ),
        "change-crane-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🧪️tests/upgrades-crane-to-class-hc3/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🧪️tests/upgrades-crane-to-class-hc3/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🧪️tests/upgrades-crane-to-class-hc3/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🧪️tests/upgrades-crane-to-class-hc3/🎯️outcome/🔣️.json"),
        ),
        "change-hoist-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🧪️tests/upgrades-hoist-to-class-hc4/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🧪️tests/upgrades-hoist-to-class-hc4/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🧪️tests/upgrades-hoist-to-class-hc4/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🧪️tests/upgrades-hoist-to-class-hc4/🎯️outcome/🔣️.json"),
        ),
        "change-hoisting-speed-ms" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🧪️tests/speeds-hoisting-to-1-25-m-s/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🧪️tests/speeds-hoisting-to-1-25-m-s/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🧪️tests/speeds-hoisting-to-1-25-m-s/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🧪️tests/speeds-hoisting-to-1-25-m-s/🎯️outcome/🔣️.json"),
        ),
        "change-silo-bulk-density-kn-m3" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🧪️tests/raises-silo-bulk-density-to-10-5-kn-m3/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🧪️tests/raises-silo-bulk-density-to-10-5-kn-m3/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🧪️tests/raises-silo-bulk-density-to-10-5-kn-m3/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🧪️tests/raises-silo-bulk-density-to-10-5-kn-m3/🎯️outcome/🔣️.json"),
        ),
        "change-silo-height-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🧪️tests/raises-silo-to-18-m/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🧪️tests/raises-silo-to-18-m/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🧪️tests/raises-silo-to-18-m/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🧪️tests/raises-silo-to-18-m/🎯️outcome/🔣️.json"),
        ),
        "change-silo-hydraulic-radius-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🧪️tests/widens-silo-hydraulic-radius-to-2-25-m/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🧪️tests/widens-silo-hydraulic-radius-to-2-25-m/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🧪️tests/widens-silo-hydraulic-radius-to-2-25-m/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🧪️tests/widens-silo-hydraulic-radius-to-2-25-m/🎯️outcome/🔣️.json"),
        ),
        "change-silo-mu" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🧪️tests/raises-silo-wall-friction-mu-to-0-625/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🧪️tests/raises-silo-wall-friction-mu-to-0-625/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🧪️tests/raises-silo-wall-friction-mu-to-0-625/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🧪️tests/raises-silo-wall-friction-mu-to-0-625/🎯️outcome/🔣️.json"),
        ),
        "change-silo-k" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🧪️tests/raises-silo-lateral-pressure-ratio-k-to-0-625/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🧪️tests/raises-silo-lateral-pressure-ratio-k-to-0-625/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🧪️tests/raises-silo-lateral-pressure-ratio-k-to-0-625/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🧪️tests/raises-silo-lateral-pressure-ratio-k-to-0-625/🎯️outcome/🔣️.json"),
        ),
        "change-cs" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🧪️tests/raises-size-factor-cs-to-1-125/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🧪️tests/raises-size-factor-cs-to-1-125/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🧪️tests/raises-size-factor-cs-to-1-125/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🧪️tests/raises-size-factor-cs-to-1-125/🎯️outcome/🔣️.json"),
        ),
        "change-cd" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🧪️tests/lowers-dynamic-factor-cd-to-0-875/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🧪️tests/lowers-dynamic-factor-cd-to-0-875/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🧪️tests/lowers-dynamic-factor-cd-to-0-875/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🧪️tests/lowers-dynamic-factor-cd-to-0-875/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-en1991-1: no committed fixture is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}

/// 🎯️ The status the committed `🎯️outcome/🔣️.json` declares for one kind — `applied` or
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
/// this repository — the committed `📖️.grammar.semio` is the repository-wide `payload = OCTET+`
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
    use semio_s_plugin_norm::artifacts::en1991::standards::v1::subsets::any::schema::mutations::{apply_en1991_mutation, decode_en1991_mutation_json, inverse_en1991_mutation, En1991Mutation};
    use semio_s_plugin_norm::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::{decode_en1991_dsl, decode_en1991_pack, decode_en1991_snapshot_json, encode_en1991_dsl, encode_en1991_pack, encode_en1991_snapshot_json, En1991Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1991Snapshot, String> {
        decode_en1991_snapshot_json(text).map_err(|error| format!("mutate-en1991-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1991Mutation, String> {
        decode_en1991_mutation_json(text).map_err(|error| format!("mutate-en1991-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1991Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1991_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1991Snapshot, expected: &En1991Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1991_snapshot_json(got), encode_en1991_snapshot_json(expected))
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
            let applied = apply_en1991_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_en1991_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1991_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1991_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1991_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1991 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1991_dsl(&text)?;
        let reprinted = encode_en1991_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1991_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1991_pack(&encode_en1991_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1991_snapshot_json(&encode_en1991_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1991_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
        if twin != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different document than the committed text artifact", &twin, &parsed));
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
