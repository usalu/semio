//! 🦀️ EN 1995 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14 (the no-oracle conversion). The recorded
//! no-oracle decision `en1995-1-mutation-semantics` is gone from
//! `../../🔮️oracles/🔣️.json`, because a reference now
//! exists to compare against: `s.norm.en1995` is a
//! semio-native artifact with no third-party reader or writer, so its reference is a second
//! IMPLEMENTATION: the independent Python `🐍️.py` beside this file, registered as the
//! oracle `en1995-1-python-independent`. This adapter is the SUBJECT half only — it drives this
//! repository's own `apply_en1995_mutation` over the full 66-kind `En1995Mutation` vocabulary.
//!
//! The vocabulary is hierarchical: `change-annex`; insert/remove plus one `change-member-<field>`
//! per scalar of an id-addressed member; insert/remove plus one `change-member-action-<field>` per
//! scalar of a member's id-addressed characteristic action; and the same three shapes for
//! connections and their fastener actions. Every vector starts from the default floor-beam document
//! and is written by the production JSON codec (`EN1995_REGEN_MUTATION_VECTORS=1`), and
//! `committed_mutation_vectors_replay_for_every_kind` in the crate replays each one.
//!
//! ⚖️ WHERE THE ASSERTIONS LIVE. Every law this case claims is asserted IN ROLE inside the
//! subject handlers as well as being compared against the oracle's answer, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law` module (`law::mutation_is_observable`,
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
//! (`decode_en1995_snapshot_json`/`encode_en1995_snapshot_json`,
//! `decode_en1995_dsl`/`encode_en1995_dsl`, `decode_en1995_pack`/`encode_en1995_pack` in
//! `../../🧬️schema/📸️snapshot/🦀️.rs`;
//! `decode_en1995_mutation_json`, `apply_en1995_mutation`, `inverse_en1995_mutation` in
//! `../../🧬️schema/🧬️mutations/🦀️.rs`), whose
//! signatures name only reachable types. This side reaches the committed vectors through
//! `include_str!` and the Python side through the `asset://` URIs the feature declares, so both
//! read the SAME committed bytes and neither holds a Rust or Python literal transcribed beside
//! them that could drift from what the other one read.
//!
//! 🚧️ The subject half is `sut`-gated so the oracle-only run never links the subject crate; the
//! bridges it calls are exercised inside the crate by `bridges_round_trip_apply_and_invert_every_kind`.

use semio_repo_test_host::{digest, parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Mirrors `En1995Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-annex",
    "insert-member",
    "remove-member",
    "change-member-label-en",
    "change-member-label-de",
    "change-member-role",
    "change-member-strength-class",
    "change-member-service-class",
    "change-member-support",
    "change-member-b",
    "change-member-h",
    "change-member-span",
    "change-member-support-length",
    "change-member-bearing-length",
    "change-member-buckling-y",
    "change-member-buckling-z",
    "change-member-lateral-restraint",
    "change-member-notch-depth",
    "change-member-notch-distance",
    "change-member-m-crit",
    "change-member-mass-per-m",
    "change-member-mass-per-m2",
    "change-member-damping",
    "change-member-fire-duration",
    "change-member-bridge-n-obs",
    "change-member-bridge-tl-years",
    "change-member-bridge-beta",
    "change-member-bridge-a",
    "change-member-bridge-b",
    "change-member-bridge-crowd",
    "insert-member-action",
    "remove-member-action",
    "change-member-action-kind",
    "change-member-action-category",
    "change-member-action-load-duration",
    "change-member-action-q-line",
    "change-member-action-f-point",
    "change-member-action-mk",
    "change-member-action-vk",
    "change-member-action-nk",
    "change-member-action-ntk",
    "change-member-action-fc90-k",
    "insert-connection",
    "remove-connection",
    "change-connection-label-en",
    "change-connection-label-de",
    "change-connection-fastener-type",
    "change-connection-strength-class",
    "change-connection-service-class",
    "change-connection-diameter",
    "change-connection-number",
    "change-connection-rows",
    "change-connection-spacing",
    "change-connection-edge-distance",
    "change-connection-end-distance",
    "change-connection-t1",
    "change-connection-t2",
    "change-connection-steel-plate",
    "change-connection-steel-plate-thickness",
    "change-connection-shear-planes",
    "change-connection-fuk",
    "insert-connection-action",
    "remove-connection-action",
    "change-connection-action-kind",
    "change-connection-action-load-duration",
    "change-connection-action-fk",
];

/// 🗣️ The real committed EN 1995 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏠️glulam-floor-beam/🏠️glulam-floor-beam/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏠️glulam-floor-beam/🎒️.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-annex" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍️switches-from-the-german-na-to-the-recommended-en-annex/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍️switches-from-the-german-na-to-the-recommended-en-annex/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍️switches-from-the-german-na-to-the-recommended-en-annex/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍️switches-from-the-german-na-to-the-recommended-en-annex/🎯️outcome/🔣️.json"),
        ),
        "insert-member" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/➕️inserts-a-member-at-end/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/➕️inserts-a-member-at-end/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/➕️inserts-a-member-at-end/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/➕️inserts-a-member-at-end/🎯️outcome/🔣️.json"),
        ),
        "remove-member" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member/➖️removes-the-first-member/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member/➖️removes-the-first-member/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member/➖️removes-the-first-member/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member/➖️removes-the-first-member/🎯️outcome/🔣️.json"),
        ),
        "change-member-label-en" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✏️sets-labelEn/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✏️sets-labelEn/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✏️sets-labelEn/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✏️sets-labelEn/🎯️outcome/🔣️.json"),
        ),
        "change-member-label-de" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✏️sets-labelDe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✏️sets-labelDe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✏️sets-labelDe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✏️sets-labelDe/🎯️outcome/🔣️.json"),
        ),
        "change-member-role" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🎯️change-member-role/✏️sets-role/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🎯️change-member-role/✏️sets-role/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🎯️change-member-role/✏️sets-role/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🎯️change-member-role/✏️sets-role/🎯️outcome/🔣️.json"),
        ),
        "change-member-strength-class" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✏️sets-strengthClass/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✏️sets-strengthClass/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✏️sets-strengthClass/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✏️sets-strengthClass/🎯️outcome/🔣️.json"),
        ),
        "change-member-service-class" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✏️sets-serviceClass/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✏️sets-serviceClass/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✏️sets-serviceClass/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✏️sets-serviceClass/🎯️outcome/🔣️.json"),
        ),
        "change-member-support" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-member-support/✏️sets-support/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-member-support/✏️sets-support/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-member-support/✏️sets-support/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-member-support/✏️sets-support/🎯️outcome/🔣️.json"),
        ),
        "change-member-b" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-b/✏️sets-bM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-b/✏️sets-bM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-b/✏️sets-bM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-b/✏️sets-bM/🎯️outcome/🔣️.json"),
        ),
        "change-member-h" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-h/✏️sets-hM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-h/✏️sets-hM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-h/✏️sets-hM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-h/✏️sets-hM/🎯️outcome/🔣️.json"),
        ),
        "change-member-span" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-span/✏️sets-spanM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-span/✏️sets-spanM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-span/✏️sets-spanM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-span/✏️sets-spanM/🎯️outcome/🔣️.json"),
        ),
        "change-member-support-length" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✏️sets-supportLengthM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✏️sets-supportLengthM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✏️sets-supportLengthM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✏️sets-supportLengthM/🎯️outcome/🔣️.json"),
        ),
        "change-member-bearing-length" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✏️sets-bearingLengthM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✏️sets-bearingLengthM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✏️sets-bearingLengthM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✏️sets-bearingLengthM/🎯️outcome/🔣️.json"),
        ),
        "change-member-buckling-y" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-y/✏️sets-bucklingLengthYM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-y/✏️sets-bucklingLengthYM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-y/✏️sets-bucklingLengthYM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-y/✏️sets-bucklingLengthYM/🎯️outcome/🔣️.json"),
        ),
        "change-member-buckling-z" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-z/✏️sets-bucklingLengthZM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-z/✏️sets-bucklingLengthZM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-z/✏️sets-bucklingLengthZM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-z/✏️sets-bucklingLengthZM/🎯️outcome/🔣️.json"),
        ),
        "change-member-lateral-restraint" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-lateral-restraint/✏️sets-lateralRestraintSpacingM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-lateral-restraint/✏️sets-lateralRestraintSpacingM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-lateral-restraint/✏️sets-lateralRestraintSpacingM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-lateral-restraint/✏️sets-lateralRestraintSpacingM/🎯️outcome/🔣️.json"),
        ),
        "change-member-notch-depth" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✏️sets-notchDepthM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✏️sets-notchDepthM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✏️sets-notchDepthM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✏️sets-notchDepthM/🎯️outcome/🔣️.json"),
        ),
        "change-member-notch-distance" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✏️sets-notchDistanceM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✏️sets-notchDistanceM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✏️sets-notchDistanceM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✏️sets-notchDistanceM/🎯️outcome/🔣️.json"),
        ),
        "change-member-m-crit" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚠️change-member-m-crit/✏️sets-mCritNm/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚠️change-member-m-crit/✏️sets-mCritNm/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚠️change-member-m-crit/✏️sets-mCritNm/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚠️change-member-m-crit/✏️sets-mCritNm/🎯️outcome/🔣️.json"),
        ),
        "change-member-mass-per-m" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-per-m/✏️sets-massKgPerM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-per-m/✏️sets-massKgPerM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-per-m/✏️sets-massKgPerM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-per-m/✏️sets-massKgPerM/🎯️outcome/🔣️.json"),
        ),
        "change-member-mass-per-m2" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-per-m2/✏️sets-massKgPerM2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-per-m2/✏️sets-massKgPerM2/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-per-m2/✏️sets-massKgPerM2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-per-m2/✏️sets-massKgPerM2/🎯️outcome/🔣️.json"),
        ),
        "change-member-damping" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌊️change-member-damping/✏️sets-dampingXi/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌊️change-member-damping/✏️sets-dampingXi/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌊️change-member-damping/✏️sets-dampingXi/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌊️change-member-damping/✏️sets-dampingXi/🎯️outcome/🔣️.json"),
        ),
        "change-member-fire-duration" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✏️sets-fireDurationS/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✏️sets-fireDurationS/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✏️sets-fireDurationS/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✏️sets-fireDurationS/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-n-obs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✏️sets-bridgeNObs/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✏️sets-bridgeNObs/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✏️sets-bridgeNObs/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✏️sets-bridgeNObs/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-tl-years" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✏️sets-bridgeTLYears/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✏️sets-bridgeTLYears/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✏️sets-bridgeTLYears/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✏️sets-bridgeTLYears/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-beta" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✏️sets-bridgeBeta/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✏️sets-bridgeBeta/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✏️sets-bridgeBeta/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✏️sets-bridgeBeta/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-a" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✏️sets-bridgeA/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✏️sets-bridgeA/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✏️sets-bridgeA/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✏️sets-bridgeA/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-b" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✏️sets-bridgeB/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✏️sets-bridgeB/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✏️sets-bridgeB/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✏️sets-bridgeB/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-crowd" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✏️sets-bridgeCrowdPerM2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✏️sets-bridgeCrowdPerM2/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✏️sets-bridgeCrowdPerM2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✏️sets-bridgeCrowdPerM2/🎯️outcome/🔣️.json"),
        ),
        "insert-member-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/➕️inserts-an-action-at-end-of-member/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/➕️inserts-an-action-at-end-of-member/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/➕️inserts-an-action-at-end-of-member/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/➕️inserts-an-action-at-end-of-member/🎯️outcome/🔣️.json"),
        ),
        "remove-member-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/➖️removes-the-first-action-of-member/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/➖️removes-the-first-action-of-member/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/➖️removes-the-first-action-of-member/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/➖️removes-the-first-action-of-member/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-kind" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✏️sets-kind/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✏️sets-kind/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✏️sets-kind/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✏️sets-kind/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-category" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✏️sets-category/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✏️sets-category/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✏️sets-category/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✏️sets-category/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-load-duration" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-member-action-load-duration/✏️sets-loadDuration/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-member-action-load-duration/✏️sets-loadDuration/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-member-action-load-duration/✏️sets-loadDuration/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-member-action-load-duration/✏️sets-loadDuration/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-q-line" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✏️sets-qLineNPerM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✏️sets-qLineNPerM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✏️sets-qLineNPerM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✏️sets-qLineNPerM/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-f-point" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✏️sets-fPointN/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✏️sets-fPointN/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✏️sets-fPointN/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✏️sets-fPointN/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-mk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✏️sets-mKNm/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✏️sets-mKNm/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✏️sets-mKNm/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✏️sets-mKNm/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-vk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✏️sets-vKN/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✏️sets-vKN/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✏️sets-vKN/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✏️sets-vKN/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-nk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✏️sets-nKN/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✏️sets-nKN/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✏️sets-nKN/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✏️sets-nKN/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-ntk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✏️sets-nTKN/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✏️sets-nTKN/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✏️sets-nTKN/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✏️sets-nTKN/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-fc90-k" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✏️sets-fC90KN/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✏️sets-fC90KN/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✏️sets-fC90KN/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✏️sets-fC90KN/🎯️outcome/🔣️.json"),
        ),
        "insert-connection" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/➕️inserts-a-connection-at-end/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/➕️inserts-a-connection-at-end/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/➕️inserts-a-connection-at-end/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/➕️inserts-a-connection-at-end/🎯️outcome/🔣️.json"),
        ),
        "remove-connection" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection/➖️removes-the-first-connection/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection/➖️removes-the-first-connection/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection/➖️removes-the-first-connection/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection/➖️removes-the-first-connection/🎯️outcome/🔣️.json"),
        ),
        "change-connection-label-en" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✏️sets-labelEn/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✏️sets-labelEn/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✏️sets-labelEn/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✏️sets-labelEn/🎯️outcome/🔣️.json"),
        ),
        "change-connection-label-de" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✏️sets-labelDe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✏️sets-labelDe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✏️sets-labelDe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✏️sets-labelDe/🎯️outcome/🔣️.json"),
        ),
        "change-connection-fastener-type" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✏️sets-fastenerType/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✏️sets-fastenerType/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✏️sets-fastenerType/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✏️sets-fastenerType/🎯️outcome/🔣️.json"),
        ),
        "change-connection-strength-class" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✏️sets-strengthClass/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✏️sets-strengthClass/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✏️sets-strengthClass/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✏️sets-strengthClass/🎯️outcome/🔣️.json"),
        ),
        "change-connection-service-class" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✏️sets-serviceClass/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✏️sets-serviceClass/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✏️sets-serviceClass/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✏️sets-serviceClass/🎯️outcome/🔣️.json"),
        ),
        "change-connection-diameter" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✏️sets-diameterM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✏️sets-diameterM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✏️sets-diameterM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✏️sets-diameterM/🎯️outcome/🔣️.json"),
        ),
        "change-connection-number" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✏️sets-number/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✏️sets-number/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✏️sets-number/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✏️sets-number/🎯️outcome/🔣️.json"),
        ),
        "change-connection-rows" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✏️sets-rows/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✏️sets-rows/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✏️sets-rows/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✏️sets-rows/🎯️outcome/🔣️.json"),
        ),
        "change-connection-spacing" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✏️sets-spacingM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✏️sets-spacingM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✏️sets-spacingM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✏️sets-spacingM/🎯️outcome/🔣️.json"),
        ),
        "change-connection-edge-distance" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✏️sets-edgeDistanceM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✏️sets-edgeDistanceM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✏️sets-edgeDistanceM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✏️sets-edgeDistanceM/🎯️outcome/🔣️.json"),
        ),
        "change-connection-end-distance" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✏️sets-endDistanceM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✏️sets-endDistanceM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✏️sets-endDistanceM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✏️sets-endDistanceM/🎯️outcome/🔣️.json"),
        ),
        "change-connection-t1" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✏️sets-t1M/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✏️sets-t1M/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✏️sets-t1M/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✏️sets-t1M/🎯️outcome/🔣️.json"),
        ),
        "change-connection-t2" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✏️sets-t2M/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✏️sets-t2M/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✏️sets-t2M/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✏️sets-t2M/🎯️outcome/🔣️.json"),
        ),
        "change-connection-steel-plate" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✏️sets-steelPlate/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✏️sets-steelPlate/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✏️sets-steelPlate/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✏️sets-steelPlate/🎯️outcome/🔣️.json"),
        ),
        "change-connection-steel-plate-thickness" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-steel-plate-thickness/✏️sets-steelPlateThicknessM/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-steel-plate-thickness/✏️sets-steelPlateThicknessM/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-steel-plate-thickness/✏️sets-steelPlateThicknessM/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-steel-plate-thickness/✏️sets-steelPlateThicknessM/🎯️outcome/🔣️.json"),
        ),
        "change-connection-shear-planes" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✏️sets-shearPlanes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✏️sets-shearPlanes/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✏️sets-shearPlanes/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✏️sets-shearPlanes/🎯️outcome/🔣️.json"),
        ),
        "change-connection-fuk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✏️sets-fUK/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✏️sets-fUK/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✏️sets-fUK/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✏️sets-fUK/🎯️outcome/🔣️.json"),
        ),
        "insert-connection-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection-action/➕️inserts-an-action-at-end-of-connection/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection-action/➕️inserts-an-action-at-end-of-connection/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection-action/➕️inserts-an-action-at-end-of-connection/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection-action/➕️inserts-an-action-at-end-of-connection/🎯️outcome/🔣️.json"),
        ),
        "remove-connection-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection-action/➖️removes-the-first-action-of-connection/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection-action/➖️removes-the-first-action-of-connection/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection-action/➖️removes-the-first-action-of-connection/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection-action/➖️removes-the-first-action-of-connection/🎯️outcome/🔣️.json"),
        ),
        "change-connection-action-kind" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✏️sets-kind/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✏️sets-kind/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✏️sets-kind/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✏️sets-kind/🎯️outcome/🔣️.json"),
        ),
        "change-connection-action-load-duration" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-connection-action-load-duration/✏️sets-loadDuration/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-connection-action-load-duration/✏️sets-loadDuration/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-connection-action-load-duration/✏️sets-loadDuration/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-connection-action-load-duration/✏️sets-loadDuration/🎯️outcome/🔣️.json"),
        ),
        "change-connection-action-fk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✏️sets-fKN/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✏️sets-fKN/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✏️sets-fKN/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✏️sets-fKN/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-en1995-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_en1995::standards::v1::subsets::any::schema::mutations::{apply_en1995_mutation, decode_en1995_mutation_json, inverse_en1995_mutation, En1995Mutation};
    use semio_s_artifact_norm_en1995::standards::v1::subsets::any::schema::snapshot::{decode_en1995_dsl, decode_en1995_pack, decode_en1995_snapshot_json, encode_en1995_dsl, encode_en1995_pack, encode_en1995_snapshot_json, En1995Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1995Snapshot, String> {
        decode_en1995_snapshot_json(text).map_err(|error| format!("mutate-en1995-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1995Mutation, String> {
        decode_en1995_mutation_json(text).map_err(|error| format!("mutate-en1995-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1995Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1995_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1995Snapshot, expected: &En1995Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1995_snapshot_json(got), encode_en1995_snapshot_json(expected))
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
            let applied = apply_en1995_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_en1995_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1995_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1995_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1995_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1995 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1995_dsl(&text)?;
        let reprinted = encode_en1995_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1995_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1995_pack(&encode_en1995_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1995_snapshot_json(&encode_en1995_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1995_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
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
