//! 🦀️ EN 1995 exhaustive mutation case — the Rust SUBJECT adapter over the current 66-kind `En1995Mutation`
//! vocabulary. Every kind applies its committed vector (`../../🧫️fixtures/🧬️mutations/<leaf>/<scenario>`) through the
//! subset's production bridges and asserts, in role, the committed after-snapshot, that the document moved and that
//! its own inverse restores the before-snapshot; `identity-round-trip` re-emits the committed carrier through the DSL,
//! pack and JSON codecs. The reference answer is the shared Python norm engine (`../🐍️.py`).

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{digest, parse_json, Json};

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
    "change-member-buckling-length-y",
    "change-member-buckling-length-z",
    "change-member-restraint-spacing",
    "change-member-notch-depth",
    "change-member-notch-distance",
    "change-member-m-crit",
    "change-member-mass-kg-per-m",
    "change-member-mass-kg-per-m2",
    "change-member-damping-xi",
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
    "change-member-load-duration",
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
    "change-connection-plate-thickness",
    "change-connection-shear-planes",
    "change-connection-fuk",
    "insert-connection-action",
    "remove-connection-action",
    "change-connection-action-kind",
    "change-connection-load-duration",
    "change-connection-action-fk",
];

/// ⛔️ The refusal witnesses — `<kind>-<slug>` rows whose committed `🎯️outcome` declares the refusal and its code.
/// Registered for `mutate-` only: a refused mutation leaves nothing to undo.
#[cfg(feature = "sut")]
const ROWS: &[&str] = &[
    "insert-member-dupe",
    "insert-connection-dupe",
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
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-member" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-member-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-member" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-label-en" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-label-de" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-role" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🎯️change-member-role/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🎯️change-member-role/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🎯️change-member-role/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🎯️change-member-role/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-strength-class" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-service-class" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-support" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-member-support/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-member-support/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-member-support/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-member-support/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-b" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-b/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-b/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-b/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-b/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-h" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-span" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-span/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-span/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-span/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-span/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-support-length" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-bearing-length" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-buckling-length-y" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-y/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-y/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-y/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-y/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-buckling-length-z" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-z/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-z/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-z/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-z/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-restraint-spacing" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-restraint-spacing/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-restraint-spacing/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-restraint-spacing/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-restraint-spacing/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-notch-depth" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-notch-distance" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-m-crit" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚠️change-member-m-crit/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚠️change-member-m-crit/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚠️change-member-m-crit/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚠️change-member-m-crit/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-mass-kg-per-m" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-mass-kg-per-m2" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m2/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m2/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m2/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m2/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-damping-xi" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌊️change-member-damping-xi/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌊️change-member-damping-xi/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌊️change-member-damping-xi/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌊️change-member-damping-xi/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-fire-duration" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-n-obs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-tl-years" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-beta" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-a" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-b" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-bridge-crowd" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-member-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/🎯️outcome/🔣️.json"),
        ),
        "remove-member-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-kind" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-category" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-load-duration" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-member-load-duration/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-member-load-duration/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-member-load-duration/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-member-load-duration/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-q-line" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-f-point" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-mk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-vk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-nk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-ntk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-action-fc90-k" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-connection" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-connection-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-connection" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-label-en" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-label-de" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-fastener-type" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-strength-class" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-service-class" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-diameter" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-number" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-rows" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-spacing" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-edge-distance" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-end-distance" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-t1" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-t2" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-steel-plate" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-plate-thickness" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-plate-thickness/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-plate-thickness/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-plate-thickness/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-connection-plate-thickness/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-shear-planes" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-fuk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-connection-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection-action/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection-action/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection-action/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-connection-action/✅apply/🎯️outcome/🔣️.json"),
        ),
        "remove-connection-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection-action/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection-action/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection-action/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-connection-action/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-action-kind" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-load-duration" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-connection-load-duration/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-connection-load-duration/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-connection-load-duration/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⏳️change-connection-load-duration/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-connection-action-fk" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✅apply/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-en1995-1: no committed fixture is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}

/// 🎯️ The status the committed `🎯️outcome/🔣️.json` declares for one row — `applied`, `rejected` or
/// `no-op` — read out of the committed file rather than transcribed beside it, so the contract a
/// row is held to cannot drift away from the vector that states it.
#[cfg(feature = "sut")]
fn committed_status(kind: &str) -> String {
    let (_before, _mutation, _after, outcome) = fixture_text(kind);
    canonical(outcome).str("status")
}

/// 🏷️ The frozen outcome code a refusal or no-op row's committed `🎯️outcome` declares; empty for `applied`.
#[cfg(feature = "sut")]
fn committed_code(kind: &str) -> String {
    let (_before, _mutation, _after, outcome) = fixture_text(kind);
    canonical(outcome).str("code")
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
    use semio_repo_test_host::law;

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
    /// 🎯️ Applies the row's mutation to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot, under whichever contract the committed `🎯️outcome` declares: an `applied`
    /// vector must be accepted without a diagnostic and must move the projection (`law::
    /// mutation_is_observable`), a `rejected` or `no-op` one must raise the committed outcome code and leave
    /// the document bit-identical. A handler that merely returned `Ok` would report a pass having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let status = super::committed_status(kind);
            let code = super::committed_code(kind);
            let applied = apply_en1995_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected" | "no-op", Ok((snapshot, messages))) if !messages.iter().any(|message| message.ends_with(&format!(":{code}"))) => return Err(format!("mutate-{kind}: the committed vector declares {status} with {code}, yet it raised {messages:?} — the document came back as {}", encode_en1995_snapshot_json(&snapshot))),
                ("rejected" | "no-op", Ok((snapshot, _messages))) => snapshot,
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
        Ok(Outcome::with_raw(reprinted.as_bytes().to_vec(), super::carrier_projection(&reprinted)))
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
        for row in ROWS {
            built = built.subject(&format!("mutate-{row}"), subject::mutate(row));
        }
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
