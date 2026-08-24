//! 🦀️ EN 1991 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 12 (the unregistered-vocabulary sweep). Recorded
//! no-oracle decision `en1991-1-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `s.norm.en1991` is a
//! semio-native artifact with no third-party reader or writer, so the `oracle` handlers here
//! read the committed, independently handcrafted per-kind specification vectors literally — no
//! recomputation, no reimplementation of mutation semantics — while `subject` drives this
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
//! ⚖️ WHERE THE ASSERTIONS LIVE. A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has
//! none — so the comparison profile never gets two sides to compare. Every law this case claims
//! is therefore asserted IN ROLE inside the subject handlers, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module (`law::mutation_is_observable`,
//! `law::inverse_restores`, `law::round_trip_preserves`, `law::carrier_is_exact`) that the
//! stdio mutation cases use, reached through the `oracleHostPackages` entry this plugin
//! declares in `✏️s/🔌️plugins/📕️norm/🧪️oracle/🔣️component.json`. The oracle handlers still
//! assert what a committed vector can prove on its own: that an `applied` vector genuinely
//! moves the document and a `rejected` one genuinely does not.
//!
//! 🌉️ HOW THE FIXTURES REACH TYPED VALUES. The generated test host links only
//! `semio-repo-test-host`, the stdio law crate and — behind `sut` — this plugin's own crate;
//! `serde`, `serde_json` and this crate's `protocol`/`store`/`vcs` extern-crate aliases are all
//! unreachable from here. The subset's own production code therefore exports the bridges
//! (`decode_en1991_snapshot_json`/`encode_en1991_snapshot_json`,
//! `decode_en1991_dsl`/`encode_en1991_dsl`, `decode_en1991_pack`/`encode_en1991_pack` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`;
//! `decode_en1991_mutation_json`, `apply_en1991_mutation`, `inverse_en1991_mutation` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`), whose
//! signatures name only reachable types. Both roles read the SAME committed bytes — the oracle
//! role via `include_str!`, the subject role by decoding that same text — so a fixture can
//! never drift away from a Rust literal transcribed beside it, because there is none.
//!
//! 🚧️ The Rust SUBJECT phase cannot run at the time of writing: `semio-s-plugin-norm` does not
//! compile (a concurrent session is mid-flight removing gratuitous `async fn` wrappers across
//! the crate), and `semio-framework-os-kernel` is red for the same reason. The subject half is
//! written against the SYNC trait surface the fixture tests in this crate already call
//! (`Mutation::diff`, `MutationDiff::apply`, `Mutation::inverse`, `ArtifactDsl`,
//! `ArtifactPack`) rather than against the plugin's async wrappers, and is `sut`-gated so the
//! oracle-only run never links it.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Mirrors `En1991Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
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
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🗣️retail-hydrocarbon-fire.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🎒️retail-hydrocarbon-fire.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — this IS the independently handcrafted evidence the no-oracle
/// decision rests on, never recomputed. One `include_str!` per committed file: the oracle role
/// answers with `before`/`after`, the subject role decodes all four.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-area-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🧪️tests/enlarges-loaded-area-to-360-m2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🧪️tests/enlarges-loaded-area-to-360-m2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🧪️tests/enlarges-loaded-area-to-360-m2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🧪️tests/enlarges-loaded-area-to-360-m2/🎯️outcome/🔣️component.json"),
        ),
        "change-category" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🧪️tests/reclassifies-imposed-load-to-category-d/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🧪️tests/reclassifies-imposed-load-to-category-d/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🧪️tests/reclassifies-imposed-load-to-category-d/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🧪️tests/reclassifies-imposed-load-to-category-d/🎯️outcome/🔣️component.json"),
        ),
        "change-annex" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🧪️tests/switches-national-annex-to-en/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🧪️tests/switches-national-annex-to-en/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🧪️tests/switches-national-annex-to-en/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🧪️tests/switches-national-annex-to-en/🎯️outcome/🔣️component.json"),
        ),
        "change-self-weight-material" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🧪️tests/switches-self-weight-material-to-structural-steel/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🧪️tests/switches-self-weight-material-to-structural-steel/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🧪️tests/switches-self-weight-material-to-structural-steel/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🧪️tests/switches-self-weight-material-to-structural-steel/🎯️outcome/🔣️component.json"),
        ),
        "change-self-weight-thickness-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🧪️tests/thickens-self-weight-layer-to-0-375-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🧪️tests/thickens-self-weight-layer-to-0-375-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🧪️tests/thickens-self-weight-layer-to-0-375-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🧪️tests/thickens-self-weight-layer-to-0-375-m/🎯️outcome/🔣️component.json"),
        ),
        "change-assumed-gk-kn-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🧪️tests/raises-assumed-gk-to-7-5-kn-m2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🧪️tests/raises-assumed-gk-to-7-5-kn-m2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🧪️tests/raises-assumed-gk-to-7-5-kn-m2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🧪️tests/raises-assumed-gk-to-7-5-kn-m2/🎯️outcome/🔣️component.json"),
        ),
        "change-fire-curve" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🧪️tests/switches-fire-curve-to-hydrocarbon/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🧪️tests/switches-fire-curve-to-hydrocarbon/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🧪️tests/switches-fire-curve-to-hydrocarbon/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🧪️tests/switches-fire-curve-to-hydrocarbon/🎯️outcome/🔣️component.json"),
        ),
        "change-fire-resistance-min" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🧪️tests/extends-fire-resistance-to-120-min/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🧪️tests/extends-fire-resistance-to-120-min/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🧪️tests/extends-fire-resistance-to-120-min/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🧪️tests/extends-fire-resistance-to-120-min/🎯️outcome/🔣️component.json"),
        ),
        "change-fire-member-capacity-c" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🧪️tests/raises-fire-member-capacity-to-700-c/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🧪️tests/raises-fire-member-capacity-to-700-c/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🧪️tests/raises-fire-member-capacity-to-700-c/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🧪️tests/raises-fire-member-capacity-to-700-c/🎯️outcome/🔣️component.json"),
        ),
        "change-snow-zone" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🧪️tests/moves-site-to-snow-zone-3/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🧪️tests/moves-site-to-snow-zone-3/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🧪️tests/moves-site-to-snow-zone-3/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🧪️tests/moves-site-to-snow-zone-3/🎯️outcome/🔣️component.json"),
        ),
        "change-snow-altitude-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🧪️tests/lifts-snow-altitude-to-780-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🧪️tests/lifts-snow-altitude-to-780-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🧪️tests/lifts-snow-altitude-to-780-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🧪️tests/lifts-snow-altitude-to-780-m/🎯️outcome/🔣️component.json"),
        ),
        "change-en-sk-kn-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🧪️tests/raises-en-characteristic-snow-load-to-1-25-kn-m2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🧪️tests/raises-en-characteristic-snow-load-to-1-25-kn-m2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🧪️tests/raises-en-characteristic-snow-load-to-1-25-kn-m2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🧪️tests/raises-en-characteristic-snow-load-to-1-25-kn-m2/🎯️outcome/🔣️component.json"),
        ),
        "change-wind-zone" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🧪️tests/moves-site-to-wind-zone-4/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🧪️tests/moves-site-to-wind-zone-4/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🧪️tests/moves-site-to-wind-zone-4/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🧪️tests/moves-site-to-wind-zone-4/🎯️outcome/🔣️component.json"),
        ),
        "change-en-vbms" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🧪️tests/raises-en-basic-wind-speed-to-30-m-s/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🧪️tests/raises-en-basic-wind-speed-to-30-m-s/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🧪️tests/raises-en-basic-wind-speed-to-30-m-s/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🧪️tests/raises-en-basic-wind-speed-to-30-m-s/🎯️outcome/🔣️component.json"),
        ),
        "change-delta-tk" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🧪️tests/raises-thermal-delta-tk-to-45-k/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🧪️tests/raises-thermal-delta-tk-to-45-k/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🧪️tests/raises-thermal-delta-tk-to-45-k/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🧪️tests/raises-thermal-delta-tk-to-45-k/🎯️outcome/🔣️component.json"),
        ),
        "change-construction-activity" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🧪️tests/switches-construction-activity-to-concreting/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🧪️tests/switches-construction-activity-to-concreting/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🧪️tests/switches-construction-activity-to-concreting/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🧪️tests/switches-construction-activity-to-concreting/🎯️outcome/🔣️component.json"),
        ),
        "change-accidental-mass-t" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🧪️tests/lightens-impact-vehicle-to-12-5-t/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🧪️tests/lightens-impact-vehicle-to-12-5-t/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🧪️tests/lightens-impact-vehicle-to-12-5-t/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🧪️tests/lightens-impact-vehicle-to-12-5-t/🎯️outcome/🔣️component.json"),
        ),
        "change-accidental-speed-km-h" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🧪️tests/lowers-impact-speed-to-50-km-h/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🧪️tests/lowers-impact-speed-to-50-km-h/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🧪️tests/lowers-impact-speed-to-50-km-h/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🧪️tests/lowers-impact-speed-to-50-km-h/🎯️outcome/🔣️component.json"),
        ),
        "change-bridge-lane" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🧪️tests/widens-carriageway-to-3-notional-lanes/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🧪️tests/widens-carriageway-to-3-notional-lanes/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🧪️tests/widens-carriageway-to-3-notional-lanes/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🧪️tests/widens-carriageway-to-3-notional-lanes/🎯️outcome/🔣️component.json"),
        ),
        "change-bridge-span-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🧪️tests/lengthens-bridge-span-to-36-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🧪️tests/lengthens-bridge-span-to-36-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🧪️tests/lengthens-bridge-span-to-36-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🧪️tests/lengthens-bridge-span-to-36-m/🎯️outcome/🔣️component.json"),
        ),
        "change-bridge-lane-width-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🧪️tests/widens-notional-lane-to-3-5-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🧪️tests/widens-notional-lane-to-3-5-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🧪️tests/widens-notional-lane-to-3-5-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🧪️tests/widens-notional-lane-to-3-5-m/🎯️outcome/🔣️component.json"),
        ),
        "change-bridge-moment-resistance-knm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🧪️tests/raises-bridge-moment-resistance-to-4500-knm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🧪️tests/raises-bridge-moment-resistance-to-4500-knm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🧪️tests/raises-bridge-moment-resistance-to-4500-knm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🧪️tests/raises-bridge-moment-resistance-to-4500-knm/🎯️outcome/🔣️component.json"),
        ),
        "change-crane-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🧪️tests/upgrades-crane-to-class-hc3/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🧪️tests/upgrades-crane-to-class-hc3/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🧪️tests/upgrades-crane-to-class-hc3/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🧪️tests/upgrades-crane-to-class-hc3/🎯️outcome/🔣️component.json"),
        ),
        "change-hoist-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🧪️tests/upgrades-hoist-to-class-hc4/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🧪️tests/upgrades-hoist-to-class-hc4/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🧪️tests/upgrades-hoist-to-class-hc4/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🧪️tests/upgrades-hoist-to-class-hc4/🎯️outcome/🔣️component.json"),
        ),
        "change-hoisting-speed-ms" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🧪️tests/speeds-hoisting-to-1-25-m-s/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🧪️tests/speeds-hoisting-to-1-25-m-s/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🧪️tests/speeds-hoisting-to-1-25-m-s/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🧪️tests/speeds-hoisting-to-1-25-m-s/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-bulk-density-kn-m3" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🧪️tests/raises-silo-bulk-density-to-10-5-kn-m3/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🧪️tests/raises-silo-bulk-density-to-10-5-kn-m3/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🧪️tests/raises-silo-bulk-density-to-10-5-kn-m3/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🧪️tests/raises-silo-bulk-density-to-10-5-kn-m3/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-height-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🧪️tests/raises-silo-to-18-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🧪️tests/raises-silo-to-18-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🧪️tests/raises-silo-to-18-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🧪️tests/raises-silo-to-18-m/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-hydraulic-radius-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🧪️tests/widens-silo-hydraulic-radius-to-2-25-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🧪️tests/widens-silo-hydraulic-radius-to-2-25-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🧪️tests/widens-silo-hydraulic-radius-to-2-25-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🧪️tests/widens-silo-hydraulic-radius-to-2-25-m/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-mu" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🧪️tests/raises-silo-wall-friction-mu-to-0-625/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🧪️tests/raises-silo-wall-friction-mu-to-0-625/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🧪️tests/raises-silo-wall-friction-mu-to-0-625/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🧪️tests/raises-silo-wall-friction-mu-to-0-625/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-k" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🧪️tests/raises-silo-lateral-pressure-ratio-k-to-0-625/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🧪️tests/raises-silo-lateral-pressure-ratio-k-to-0-625/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🧪️tests/raises-silo-lateral-pressure-ratio-k-to-0-625/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🧪️tests/raises-silo-lateral-pressure-ratio-k-to-0-625/🎯️outcome/🔣️component.json"),
        ),
        "change-cs" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🧪️tests/raises-size-factor-cs-to-1-125/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🧪️tests/raises-size-factor-cs-to-1-125/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🧪️tests/raises-size-factor-cs-to-1-125/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🧪️tests/raises-size-factor-cs-to-1-125/🎯️outcome/🔣️component.json"),
        ),
        "change-cd" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🧪️tests/lowers-dynamic-factor-cd-to-0-875/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🧪️tests/lowers-dynamic-factor-cd-to-0-875/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🧪️tests/lowers-dynamic-factor-cd-to-0-875/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🧪️tests/lowers-dynamic-factor-cd-to-0-875/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-en1991-1: no committed fixture is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}

/// 🎯️ The status the committed `🎯️outcome/🔣️component.json` declares for one kind — `applied` or
/// `rejected` — read out of the committed file rather than transcribed beside it, so the contract a
/// row is held to cannot drift away from the vector that states it.
fn committed_status(kind: &str) -> String {
    let (_before, _mutation, _after, outcome) = fixture_text(kind);
    canonical(outcome).str("status")
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally. The one law a
/// committed pair can carry on its own is asserted here in role — an `applied` vector must MOVE the
/// document and a `rejected` vector must leave it identical — so a placeholder fixture that changed
/// nothing could not sit in this table unnoticed.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, after, _outcome) = fixture_text(kind);
        let (base, projection) = (canonical(before), canonical(after));
        match committed_status(kind).as_str() {
            "applied" => law::mutation_is_observable(kind, &projection, &base, &[])?,
            "rejected" if law::divergence(&projection, &base).is_some() => {
                return Err(format!("mutate-{kind}: the committed outcome declares this vector rejected, so its after-snapshot must be identical to its before-snapshot"));
            }
            "rejected" => {}
            other => return Err(format!("mutate-{kind}: unknown committed outcome status {other:?}")),
        }
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), projection))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started. The inverse LAW itself cannot be
/// asserted from the committed vectors alone (nothing here computes an inverse), which is precisely
/// why it is asserted in the subject handler below instead.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_norm::artifacts::en1991::standards::v1::subsets::any::schema::mutations::{apply_en1991_mutation, decode_en1991_mutation_json, inverse_en1991_mutation, En1991Mutation};
    use semio_s_plugin_norm::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::{decode_en1991_dsl, decode_en1991_pack, decode_en1991_snapshot_json, encode_en1991_dsl, encode_en1991_pack, encode_en1991_snapshot_json, En1991Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
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
            let steps = inverse_en1991_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1991_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
            }
            law::inverse_restores(kind, &projection(&current)?, &original)?;
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            Ok(Outcome::with_raw(original.to_string().into_bytes(), original))
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
        let projection = projection(&parsed)?;
        law::round_trip_preserves(&projection(&repacked)?, &projection)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. `identity-round-trip` is
/// deliberately subject-only: the reference answer for every other scenario is a committed JSON
/// document the oracle role can read literally, but the real artifact is committed as DSL and pack bytes
/// ONLY, and turning those into a document needs this subset's own codec — which the oracle-only
/// build must not link.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
