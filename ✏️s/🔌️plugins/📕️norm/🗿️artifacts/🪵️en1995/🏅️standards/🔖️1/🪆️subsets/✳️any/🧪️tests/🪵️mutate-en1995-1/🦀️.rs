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
    "change-reliability-class",
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
    "change-uls-equation",
    "insert-case",
    "remove-case",
    "change-case-label-en",
    "change-case-label-de",
    "change-case-entry-id",
    "change-case-number",
    "change-case-text",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-placement",
    "remove-placement",
    "insert-deck",
    "remove-deck",
    "change-deck-lamination-product",
    "change-deck-bar-product",
    "change-deck-span",
    "change-deck-deck-width",
    "change-deck-deck-depth",
    "change-deck-lamination-width",
    "change-deck-bar-spacing",
    "change-deck-bar-diameter",
    "change-deck-plate-side",
    "change-deck-initial-prestress",
    "change-deck-loss-relaxation",
    "change-deck-loss-creep",
    "change-deck-loss-moisture",
    "change-deck-bar-resistance",
    "change-deck-shear-stress",
    "change-deck-compression-c90",
    "change-deck-wheel-load",
    "change-deck-wheel-contact",
    "insert-diaphragm",
    "remove-diaphragm",
    "change-diaphragm-kind",
    "change-diaphragm-method",
    "change-diaphragm-panel-width",
    "change-diaphragm-wall-height",
    "change-diaphragm-fastener-spacing",
    "change-diaphragm-panels",
    "change-diaphragm-fastener-capacity",
    "change-diaphragm-shear-ed",
    "change-diaphragm-hold-down",
    "change-diaphragm-force",
    "change-diaphragm-length",
    "change-diaphragm-shear-resistance",
    "insert-truss",
    "remove-truss",
    "change-truss-kind",
    "change-truss-product",
    "change-truss-span",
    "change-truss-height",
    "change-truss-bays",
    "change-truss-chord-width",
    "change-truss-chord-depth",
    "change-truss-web-width",
    "change-truss-web-depth",
    "change-truss-chord-force",
    "change-truss-restraint-length",
    "change-truss-bracing-capacity",
    "insert-bracing",
    "remove-bracing",
    "change-bracing-kind",
    "change-bracing-member",
    "change-bracing-stabilised-count",
    "change-bracing-spacing",
    "change-bracing-length",
    "change-bracing-brace-width",
    "change-bracing-brace-depth",
    "change-bracing-compression",
    "change-bracing-resistance",
    "change-bracing-stiffness",
    "insert-panel-layer",
    "remove-panel-layer",
    "change-panel-layer-thickness",
    "change-panel-layer-orientation",
    "change-panel-layer-grade",
    "insert-fastener-position",
    "remove-fastener-position",
    "change-fastener-position-x",
    "change-fastener-position-y",
    "insert-taper",
    "remove-taper",
    "change-taper-member",
    "change-taper-kind",
    "change-taper-taper-angle",
    "change-taper-apex-depth",
    "change-taper-inner-radius",
    "change-taper-lamella",
    "change-taper-apex-angle",
    "change-taper-side",
    "change-taper-rods",
    "change-taper-rod-resistance",
    "insert-cutout",
    "remove-cutout",
    "change-cutout-member",
    "change-cutout-kind",
    "change-cutout-face",
    "change-cutout-distance",
    "change-cutout-length",
    "change-cutout-depth",
    "change-cutout-centre-height",
    "change-cutout-slope",
    "change-cutout-rods",
    "change-cutout-rod-resistance",
    "insert-fastener",
    "remove-fastener",
    "change-fastener-connection",
    "change-fastener-member",
    "change-fastener-columns",
    "change-fastener-lines",
    "change-fastener-spacing-par",
    "change-fastener-spacing-perp",
    "change-fastener-end-distance",
    "change-fastener-edge-distance",
    "change-fastener-offset-x",
    "change-fastener-offset-y",
    "change-fastener-load-angle",
    "insert-panel",
    "remove-panel",
    "change-panel-strength-class",
    "change-panel-span",
    "change-panel-width",
    "change-panel-load",
    "change-panel-fire-duration",
    "insert-floor",
    "remove-floor",
    "change-floor-member",
    "change-floor-span",
    "change-floor-width",
    "change-floor-joist-spacing",
    "change-floor-ei-l",
    "change-floor-ei-b",
    "change-floor-mass",
    "change-floor-damping",
    "change-floor-permanent-load",
    "change-floor-imposed-load",
    "change-floor-psi2",
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
/// 🗃️ The committed before, mutation, after and outcome fixture texts of every registered mutation kind, read literally via `include_str!`.
#[cfg(feature = "sut")]
const FIXTURE_TEXTS: &[(&str, (&str, &str, &str, &str))] = &[
    ("change-annex", (
        include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.outcome.json"),
    )),
    ("change-reliability-class", (
        include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.outcome.json"),
    )),
    ("insert-member", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.outcome.json"),
    )),
    ("insert-member-dupe", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/⛔dupe/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/⛔dupe/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/⛔dupe/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/⛔dupe/🔣️.outcome.json"),
    )),
    ("remove-member", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-label-en", (
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-member-label-en/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-label-de", (
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-member-label-de/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-role", (
        include_str!("../🧫️fixtures/🧬️mutations/🎯️change-member-role/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🎯️change-member-role/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🎯️change-member-role/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🎯️change-member-role/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-strength-class", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-member-strength-class/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-service-class", (
        include_str!("../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌧️change-member-service-class/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-support", (
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-member-support/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-member-support/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-member-support/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-member-support/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-b", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-b/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-b/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-b/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-b/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-h", (
        include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-span", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-span/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-span/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-span/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-span/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-support-length", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-support-length/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-bearing-length", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-bearing-length/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-buckling-length-y", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-y/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-y/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-y/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-y/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-buckling-length-z", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-z/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-z/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-z/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-buckling-length-z/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-restraint-spacing", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-restraint-spacing/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-restraint-spacing/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-restraint-spacing/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-restraint-spacing/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-notch-depth", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-notch-depth/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-notch-distance", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-notch-distance/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-mass-kg-per-m", (
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-mass-kg-per-m2", (
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m2/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m2/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m2/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-mass-kg-per-m2/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-damping-xi", (
        include_str!("../🧫️fixtures/🧬️mutations/🌊️change-member-damping-xi/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌊️change-member-damping-xi/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌊️change-member-damping-xi/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌊️change-member-damping-xi/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-fire-duration", (
        include_str!("../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔥️change-member-fire-duration/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-bridge-n-obs", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-n-obs/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-bridge-tl-years", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-tl-years/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-bridge-beta", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-beta/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-bridge-a", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-a/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-bridge-b", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-bridge-b/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-bridge-crowd", (
        include_str!("../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🚶️change-member-bridge-crowd/✅apply/🔣️.outcome.json"),
    )),
    ("insert-member-action", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/🔣️.outcome.json"),
    )),
    ("remove-member-action", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-action-kind", (
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-member-action-kind/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-action-category", (
        include_str!("../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏢️change-member-action-category/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-load-duration", (
        include_str!("../🧫️fixtures/🧬️mutations/⏳️change-member-load-duration/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⏳️change-member-load-duration/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⏳️change-member-load-duration/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⏳️change-member-load-duration/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-action-q-line", (
        include_str!("../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⬇️change-member-action-q-line/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-action-f-point", (
        include_str!("../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⬇️change-member-action-f-point/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-action-mk", (
        include_str!("../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⤴️change-member-action-mk/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-action-vk", (
        include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-action-vk/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-action-nk", (
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-nk/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-action-ntk", (
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-ntk/✅apply/🔣️.outcome.json"),
    )),
    ("change-member-action-fc90-k", (
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-action-fc90-k/✅apply/🔣️.outcome.json"),
    )),
    ("insert-connection", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection/✅apply/🔣️.outcome.json"),
    )),
    ("insert-connection-dupe", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection/⛔dupe/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection/⛔dupe/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection/⛔dupe/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection/⛔dupe/🔣️.outcome.json"),
    )),
    ("remove-connection", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-connection/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-connection/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-connection/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-connection/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-label-en", (
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-connection-label-en/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-label-de", (
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-connection-label-de/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-fastener-type", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-fastener-type/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-strength-class", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-connection-strength-class/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-service-class", (
        include_str!("../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌧️change-connection-service-class/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-diameter", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-diameter/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-number", (
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-number/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-rows", (
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-rows/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-spacing", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-spacing/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-edge-distance", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-edge-distance/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-end-distance", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-end-distance/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-t1", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-t1/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-t2", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-t2/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-steel-plate", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-steel-plate/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-plate-thickness", (
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-plate-thickness/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-plate-thickness/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-plate-thickness/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/↔️change-connection-plate-thickness/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-shear-planes", (
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-connection-shear-planes/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-fuk", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-connection-fuk/✅apply/🔣️.outcome.json"),
    )),
    ("insert-connection-action", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection-action/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection-action/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection-action/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-connection-action/✅apply/🔣️.outcome.json"),
    )),
    ("remove-connection-action", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-connection-action/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-connection-action/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-connection-action/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-connection-action/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-action-kind", (
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⚖️change-connection-action-kind/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-load-duration", (
        include_str!("../🧫️fixtures/🧬️mutations/⏳️change-connection-load-duration/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⏳️change-connection-load-duration/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⏳️change-connection-load-duration/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/⏳️change-connection-load-duration/✅apply/🔣️.outcome.json"),
    )),
    ("change-connection-action-fk", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-connection-action-fk/✅apply/🔣️.outcome.json"),
    )),
    ("change-uls-equation", (
        include_str!("../🧫️fixtures/🧬️mutations/🧮️change-uls-equation/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧮️change-uls-equation/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧮️change-uls-equation/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧮️change-uls-equation/✅apply/🔣️.outcome.json"),
    )),
    ("insert-case", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-case/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-case/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-case/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-case/✅apply/🔣️.outcome.json"),
    )),
    ("remove-case", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-case/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-case/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-case/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-case/✅apply/🔣️.outcome.json"),
    )),
    ("change-case-label-en", (
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-en/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-en/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-en/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-en/✅apply/🔣️.outcome.json"),
    )),
    ("change-case-label-de", (
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-de/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-de/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-de/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-de/✅apply/🔣️.outcome.json"),
    )),
    ("change-case-entry-id", (
        include_str!("../🧫️fixtures/🧬️mutations/📇️change-case-entry-id/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📇️change-case-entry-id/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📇️change-case-entry-id/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📇️change-case-entry-id/✅apply/🔣️.outcome.json"),
    )),
    ("change-case-number", (
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-case-number/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-case-number/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-case-number/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔢️change-case-number/✅apply/🔣️.outcome.json"),
    )),
    ("change-case-text", (
        include_str!("../🧫️fixtures/🧬️mutations/🔤️change-case-text/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔤️change-case-text/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔤️change-case-text/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔤️change-case-text/✅apply/🔣️.outcome.json"),
    )),
    ("change-placement-x", (
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-x/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-x/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-x/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-x/✅apply/🔣️.outcome.json"),
    )),
    ("change-placement-y", (
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-y/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-y/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-y/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-y/✅apply/🔣️.outcome.json"),
    )),
    ("change-placement-z", (
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-z/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-z/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-z/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-z/✅apply/🔣️.outcome.json"),
    )),
    ("change-placement-yaw", (
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-yaw/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-yaw/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-yaw/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-yaw/✅apply/🔣️.outcome.json"),
    )),
    ("insert-placement", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-placement/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-placement/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-placement/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-placement/✅apply/🔣️.outcome.json"),
    )),
    ("remove-placement", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-placement/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-placement/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-placement/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-placement/✅apply/🔣️.outcome.json"),
    )),
    ("insert-deck", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-deck/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-deck/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-deck/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-deck/✅apply/🔣️.outcome.json"),
    )),
    ("remove-deck", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-deck/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-deck/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-deck/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-deck/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-lamination-product", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-lamination-product/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-lamination-product/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-lamination-product/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-lamination-product/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-bar-product", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-product/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-product/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-product/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-product/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-span", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-span/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-span/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-span/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-span/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-deck-width", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-deck-width/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-deck-width/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-deck-width/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-deck-width/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-deck-depth", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-deck-depth/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-deck-depth/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-deck-depth/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-deck-depth/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-lamination-width", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-lamination-width/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-lamination-width/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-lamination-width/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-lamination-width/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-bar-spacing", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-spacing/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-spacing/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-spacing/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-spacing/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-bar-diameter", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-diameter/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-diameter/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-diameter/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-diameter/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-plate-side", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-plate-side/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-plate-side/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-plate-side/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-plate-side/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-initial-prestress", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-initial-prestress/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-initial-prestress/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-initial-prestress/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-initial-prestress/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-loss-relaxation", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-relaxation/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-relaxation/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-relaxation/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-relaxation/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-loss-creep", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-creep/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-creep/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-creep/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-creep/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-loss-moisture", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-moisture/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-moisture/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-moisture/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-loss-moisture/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-bar-resistance", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-resistance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-resistance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-resistance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-bar-resistance/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-shear-stress", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-shear-stress/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-shear-stress/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-shear-stress/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-shear-stress/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-compression-c90", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-compression-c90/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-compression-c90/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-compression-c90/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-compression-c90/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-wheel-load", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-wheel-load/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-wheel-load/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-wheel-load/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-wheel-load/✅apply/🔣️.outcome.json"),
    )),
    ("change-deck-wheel-contact", (
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-wheel-contact/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-wheel-contact/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-wheel-contact/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🌉️change-deck-wheel-contact/✅apply/🔣️.outcome.json"),
    )),
    ("insert-diaphragm", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-diaphragm/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-diaphragm/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-diaphragm/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-diaphragm/✅apply/🔣️.outcome.json"),
    )),
    ("remove-diaphragm", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-diaphragm/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-diaphragm/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-diaphragm/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-diaphragm/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-kind", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-kind/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-kind/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-kind/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-kind/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-method", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-method/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-method/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-method/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-method/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-panel-width", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-panel-width/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-panel-width/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-panel-width/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-panel-width/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-wall-height", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-wall-height/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-wall-height/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-wall-height/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-wall-height/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-fastener-spacing", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-fastener-spacing/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-fastener-spacing/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-fastener-spacing/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-fastener-spacing/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-panels", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-panels/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-panels/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-panels/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-panels/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-fastener-capacity", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-fastener-capacity/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-fastener-capacity/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-fastener-capacity/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-fastener-capacity/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-shear-ed", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-shear-ed/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-shear-ed/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-shear-ed/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-shear-ed/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-hold-down", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-hold-down/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-hold-down/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-hold-down/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-hold-down/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-force", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-force/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-force/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-force/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-force/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-length", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-length/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-length/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-length/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-length/✅apply/🔣️.outcome.json"),
    )),
    ("change-diaphragm-shear-resistance", (
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-shear-resistance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-shear-resistance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-shear-resistance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🛡️change-diaphragm-shear-resistance/✅apply/🔣️.outcome.json"),
    )),
    ("insert-truss", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-truss/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-truss/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-truss/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-truss/✅apply/🔣️.outcome.json"),
    )),
    ("remove-truss", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-truss/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-truss/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-truss/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-truss/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-kind", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-kind/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-kind/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-kind/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-kind/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-product", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-product/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-product/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-product/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-product/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-span", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-span/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-span/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-span/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-span/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-height", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-height/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-height/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-height/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-height/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-bays", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-bays/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-bays/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-bays/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-bays/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-chord-width", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-width/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-width/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-width/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-width/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-chord-depth", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-depth/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-depth/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-depth/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-depth/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-web-width", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-web-width/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-web-width/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-web-width/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-web-width/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-web-depth", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-web-depth/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-web-depth/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-web-depth/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-web-depth/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-chord-force", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-force/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-force/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-force/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-chord-force/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-restraint-length", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-restraint-length/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-restraint-length/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-restraint-length/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-restraint-length/✅apply/🔣️.outcome.json"),
    )),
    ("change-truss-bracing-capacity", (
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-bracing-capacity/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-bracing-capacity/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-bracing-capacity/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔺️change-truss-bracing-capacity/✅apply/🔣️.outcome.json"),
    )),
    ("insert-bracing", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bracing/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bracing/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bracing/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bracing/✅apply/🔣️.outcome.json"),
    )),
    ("remove-bracing", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bracing/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bracing/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bracing/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bracing/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-kind", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-kind/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-kind/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-kind/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-kind/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-member", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-member/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-member/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-member/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-member/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-stabilised-count", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-stabilised-count/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-stabilised-count/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-stabilised-count/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-stabilised-count/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-spacing", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-spacing/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-spacing/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-spacing/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-spacing/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-length", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-length/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-length/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-length/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-length/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-brace-width", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-brace-width/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-brace-width/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-brace-width/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-brace-width/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-brace-depth", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-brace-depth/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-brace-depth/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-brace-depth/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-brace-depth/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-compression", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-compression/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-compression/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-compression/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-compression/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-resistance", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-resistance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-resistance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-resistance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-resistance/✅apply/🔣️.outcome.json"),
    )),
    ("change-bracing-stiffness", (
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-stiffness/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-stiffness/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-stiffness/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/✖️change-bracing-stiffness/✅apply/🔣️.outcome.json"),
    )),
    ("insert-panel-layer", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-panel-layer/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-panel-layer/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-panel-layer/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-panel-layer/✅apply/🔣️.outcome.json"),
    )),
    ("remove-panel-layer", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-panel-layer/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-panel-layer/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-panel-layer/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-panel-layer/✅apply/🔣️.outcome.json"),
    )),
    ("change-panel-layer-thickness", (
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-thickness/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-thickness/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-thickness/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-thickness/✅apply/🔣️.outcome.json"),
    )),
    ("change-panel-layer-orientation", (
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-orientation/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-orientation/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-orientation/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-orientation/✅apply/🔣️.outcome.json"),
    )),
    ("change-panel-layer-grade", (
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-grade/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-grade/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-grade/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-layer-grade/✅apply/🔣️.outcome.json"),
    )),
    ("insert-fastener-position", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-fastener-position/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-fastener-position/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-fastener-position/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-fastener-position/✅apply/🔣️.outcome.json"),
    )),
    ("remove-fastener-position", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-fastener-position/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-fastener-position/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-fastener-position/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-fastener-position/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-position-x", (
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-fastener-position-x/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-fastener-position-x/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-fastener-position-x/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-fastener-position-x/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-position-y", (
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-fastener-position-y/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-fastener-position-y/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-fastener-position-y/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📍️change-fastener-position-y/✅apply/🔣️.outcome.json"),
    )),
    ("insert-taper", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-taper/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-taper/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-taper/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-taper/✅apply/🔣️.outcome.json"),
    )),
    ("remove-taper", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-taper/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-taper/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-taper/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-taper/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-member", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-member/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-member/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-member/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-member/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-kind", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-kind/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-kind/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-kind/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-kind/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-taper-angle", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-taper-angle/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-taper-angle/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-taper-angle/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-taper-angle/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-apex-depth", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-apex-depth/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-apex-depth/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-apex-depth/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-apex-depth/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-inner-radius", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-inner-radius/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-inner-radius/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-inner-radius/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-inner-radius/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-lamella", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-lamella/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-lamella/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-lamella/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-lamella/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-apex-angle", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-apex-angle/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-apex-angle/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-apex-angle/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-apex-angle/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-side", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-side/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-side/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-side/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-side/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-rods", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-rods/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-rods/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-rods/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-rods/✅apply/🔣️.outcome.json"),
    )),
    ("change-taper-rod-resistance", (
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-rod-resistance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-rod-resistance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-rod-resistance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/📐️change-taper-rod-resistance/✅apply/🔣️.outcome.json"),
    )),
    ("insert-cutout", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-cutout/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-cutout/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-cutout/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-cutout/✅apply/🔣️.outcome.json"),
    )),
    ("remove-cutout", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-cutout/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-cutout/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-cutout/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-cutout/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-member", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-member/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-member/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-member/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-member/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-kind", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-kind/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-kind/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-kind/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-kind/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-face", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-face/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-face/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-face/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-face/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-distance", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-distance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-distance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-distance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-distance/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-length", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-length/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-length/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-length/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-length/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-depth", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-depth/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-depth/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-depth/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-depth/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-centre-height", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-centre-height/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-centre-height/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-centre-height/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-centre-height/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-slope", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-slope/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-slope/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-slope/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-slope/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-rods", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-rods/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-rods/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-rods/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-rods/✅apply/🔣️.outcome.json"),
    )),
    ("change-cutout-rod-resistance", (
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-rod-resistance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-rod-resistance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-rod-resistance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🕳️change-cutout-rod-resistance/✅apply/🔣️.outcome.json"),
    )),
    ("insert-fastener", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-fastener/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-fastener/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-fastener/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-fastener/✅apply/🔣️.outcome.json"),
    )),
    ("remove-fastener", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-fastener/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-fastener/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-fastener/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-fastener/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-connection", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-connection/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-connection/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-connection/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-connection/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-member", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-member/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-member/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-member/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-member/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-columns", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-columns/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-columns/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-columns/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-columns/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-lines", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-lines/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-lines/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-lines/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-lines/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-spacing-par", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-spacing-par/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-spacing-par/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-spacing-par/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-spacing-par/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-spacing-perp", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-spacing-perp/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-spacing-perp/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-spacing-perp/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-spacing-perp/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-end-distance", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-end-distance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-end-distance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-end-distance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-end-distance/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-edge-distance", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-edge-distance/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-edge-distance/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-edge-distance/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-edge-distance/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-offset-x", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-offset-x/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-offset-x/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-offset-x/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-offset-x/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-offset-y", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-offset-y/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-offset-y/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-offset-y/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-offset-y/✅apply/🔣️.outcome.json"),
    )),
    ("change-fastener-load-angle", (
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-load-angle/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-load-angle/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-load-angle/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🔩️change-fastener-load-angle/✅apply/🔣️.outcome.json"),
    )),
    ("insert-panel", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-panel/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-panel/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-panel/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-panel/✅apply/🔣️.outcome.json"),
    )),
    ("remove-panel", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-panel/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-panel/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-panel/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-panel/✅apply/🔣️.outcome.json"),
    )),
    ("change-panel-strength-class", (
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-strength-class/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-strength-class/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-strength-class/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-strength-class/✅apply/🔣️.outcome.json"),
    )),
    ("change-panel-span", (
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-span/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-span/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-span/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-span/✅apply/🔣️.outcome.json"),
    )),
    ("change-panel-width", (
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-width/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-width/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-width/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-width/✅apply/🔣️.outcome.json"),
    )),
    ("change-panel-load", (
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-load/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-load/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-load/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-load/✅apply/🔣️.outcome.json"),
    )),
    ("change-panel-fire-duration", (
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-fire-duration/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-fire-duration/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-fire-duration/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🧱️change-panel-fire-duration/✅apply/🔣️.outcome.json"),
    )),
    ("insert-floor", (
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-floor/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-floor/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-floor/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➕️insert-floor/✅apply/🔣️.outcome.json"),
    )),
    ("remove-floor", (
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-floor/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-floor/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-floor/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/➖️remove-floor/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-member", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-member/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-member/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-member/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-member/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-span", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-span/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-span/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-span/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-span/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-width", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-width/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-width/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-width/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-width/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-joist-spacing", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-joist-spacing/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-joist-spacing/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-joist-spacing/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-joist-spacing/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-ei-l", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-ei-l/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-ei-l/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-ei-l/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-ei-l/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-ei-b", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-ei-b/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-ei-b/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-ei-b/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-ei-b/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-mass", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-mass/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-mass/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-mass/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-mass/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-damping", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-damping/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-damping/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-damping/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-damping/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-permanent-load", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-permanent-load/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-permanent-load/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-permanent-load/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-permanent-load/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-imposed-load", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-imposed-load/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-imposed-load/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-imposed-load/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-imposed-load/✅apply/🔣️.outcome.json"),
    )),
    ("change-floor-psi2", (
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-psi2/✅apply/🔣️.before.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-psi2/✅apply/🔣️.mutation.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-psi2/✅apply/🔣️.after.json"),
        include_str!("../🧫️fixtures/🧬️mutations/🏠️change-floor-psi2/✅apply/🔣️.outcome.json"),
    )),
];

/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match FIXTURE_TEXTS.iter().find(|(key, _)| *key == kind) {
        Some((_, value)) => *value,
        None => {
            let other = kind;
            panic!("mutate-en1995-1: no committed fixture is registered for kind {other:?}")
        }
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}

/// 🎯️ The status the committed `🔣️.outcome.json` declares for one row — `applied`, `rejected` or
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
/// rule — so the identity scenario compares the two implementations at the carrier level rather
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
    use semio_s_artifact_norm_en1995_mutations::mutation_bridge::{apply_en1995_mutation, inverse_en1995_mutation};
    use semio_s_artifact_norm_en1995_mutations::En1995Mutation;

    use semio_s_artifact_norm_en1995_io::text::mutations::decode_en1995_mutation_json;
    use semio_s_artifact_norm_en1995_model::En1995Snapshot;
    use semio_s_artifact_norm_en1995_io::binary::snapshot::encode_en1995_pack;
    use semio_s_artifact_norm_en1995_io::binary::snapshot::decode_en1995_pack;
    use semio_s_artifact_norm_en1995_io::text::snapshot::encode_en1995_dsl;
    use semio_s_artifact_norm_en1995_io::text::snapshot::decode_en1995_dsl;
    use semio_s_artifact_norm_en1995_io::text::snapshot::decode_en1995_snapshot_json;
    use semio_s_artifact_norm_en1995_io::text::snapshot::encode_en1995_snapshot_json;
    use semio_repo_test_host::law;

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
            let steps = inverse_en1995_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
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
        let text = String::from_utf8(ctx.input_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1995 artifact is not UTF-8: {error}"))?;
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
        let twin = decode_en1995_pack(&ctx.input_bytes(super::PACK_ASSET)?)?;
        if twin != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different document than the committed text artifact", &twin, &parsed));
        }
        law::round_trip_preserves(&projection(&repacked)?, &projection(&parsed)?)?;
        Ok(Outcome::with_raw(reprinted.as_bytes().to_vec(), super::carrier_projection(&reprinted)))
    }
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
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind).expect("valid retained mutation inverse fixture"));
        }
        for row in ROWS {
            built = built.subject(&format!("mutate-{row}"), subject::mutate(row));
        }
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
