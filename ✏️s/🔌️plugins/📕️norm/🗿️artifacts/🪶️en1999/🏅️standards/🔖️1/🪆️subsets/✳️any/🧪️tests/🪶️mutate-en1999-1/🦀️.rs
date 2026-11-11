//! 🦀️ EN 1999 exhaustive mutation case — the Rust SUBJECT adapter over the current 216-kind `En1999Mutation`
//! vocabulary. Every kind applies its committed vector (`../../🧫️fixtures/🧬️mutations/<leaf>/<scenario>`) through the
//! subset's production bridges and asserts, in role, the committed after-snapshot, that the document moved and that
//! its own inverse restores the before-snapshot; `identity-round-trip` re-emits the committed carrier through the DSL,
//! pack and JSON codecs. The reference answer is the shared Python norm engine (`../🐍️.py`).

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{digest, parse_json, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `En1999Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-annex",
    "change-reliability-class",
    "insert-member",
    "remove-member",
    "change-member-n-ed",
    "change-member-my-ed",
    "change-member-buckling-length",
    "change-material-designation",
    "change-plate-thickness",
    "change-weld-throat",
    "change-bolt-count",
    "change-material-product-form",
    "change-section-kind",
    "change-section-height",
    "change-section-width",
    "change-section-flange-thickness",
    "change-section-web-thickness",
    "change-section-outer-diameter",
    "change-element-width",
    "change-element-outstand",
    "change-element-welded",
    "change-element-weld-position",
    "change-element-weld-process",
    "change-element-weld-t-junction",
    "change-element-weld-direction",
    "change-element-stiffener-depth",
    "change-element-stiffener-thk",
    "change-member-section-id",
    "change-member-material-id",
    "change-member-length",
    "change-member-support",
    "change-member-ltb-length",
    "change-member-c1",
    "change-member-restrained-ltb",
    "change-member-bearing-length",
    "change-member-transverse-weld",
    "change-member-t-weld-offset",
    "change-action-kind",
    "change-action-category",
    "change-action-source",
    "change-action-gk-line",
    "change-action-qk-line",
    "change-action-vyk",
    "change-action-vzk",
    "change-action-mzk",
    "change-action-torsion-k",
    "change-action-local-load-k",
    "change-conn-member-id",
    "change-conn-material-id",
    "change-conn-kind",
    "change-conn-force-method",
    "change-conn-bolts-material",
    "change-conn-bolts-diameter",
    "change-conn-bolts-edge-distance",
    "change-conn-bolts-pitch",
    "change-conn-bolts-gauge",
    "change-conn-bolts-plate-thickness",
    "change-conn-welds-filler-alloy",
    "change-conn-welds-length",
    "change-conn-welds-beta-w",
    "change-conn-welds-haz-extent",
    "change-conn-action-kind",
    "change-conn-action-category",
    "change-conn-action-source",
    "change-conn-action-gk-line",
    "change-conn-action-qk-line",
    "change-conn-action-nk",
    "change-conn-action-vyk",
    "change-conn-action-vzk",
    "change-conn-action-myk",
    "change-conn-action-mzk",
    "change-conn-action-torsion-k",
    "change-conn-action-local-load-k",
    "change-fire-member-id",
    "change-fire-theta-a",
    "change-fire-duration-s",
    "change-fatigue-member-id",
    "change-fatigue-detail-category",
    "change-fatigue-delta-sigma-c",
    "change-fatigue-delta-sigma-ed",
    "change-fatigue-n-cycles",
    "change-fatigue-m1",
    "change-fatigue-m2",
    "change-sheet-material-id",
    "change-sheet-thickness",
    "change-sheet-width",
    "change-sheet-span",
    "change-sheet-welded",
    "change-sheet-action-kind",
    "change-sheet-action-category",
    "change-sheet-action-source",
    "change-sheet-action-gk-line",
    "change-sheet-action-qk-line",
    "change-sheet-action-nk",
    "change-sheet-action-vyk",
    "change-sheet-action-vzk",
    "change-sheet-action-myk",
    "change-sheet-action-mzk",
    "change-sheet-action-torsion-k",
    "change-sheet-action-local-load-k",
    "change-shell-material-id",
    "change-shell-radius",
    "change-shell-thickness",
    "change-shell-length",
    "change-shell-welded",
    "change-shell-fabrication-class",
    "change-shell-action-kind",
    "change-shell-action-category",
    "change-shell-action-source",
    "change-shell-action-gk-line",
    "change-shell-action-qk-line",
    "change-shell-action-nk",
    "change-shell-action-vyk",
    "change-shell-action-vzk",
    "change-shell-action-myk",
    "change-shell-action-mzk",
    "change-shell-action-torsion-k",
    "change-shell-action-local-load-k",
    "change-weld-run-member-id",
    "change-weld-run-element-id",
    "change-weld-run-start",
    "change-weld-run-length",
    "change-weld-run-offset",
    "change-weld-run-throat",
    "change-weld-run-process",
    "change-weld-run-t-junction",
    "change-weld-run-direction",
    "change-weld-run-haz-extent",
    "change-vertex-section-id",
    "change-vertex-ring",
    "change-vertex-element-id",
    "change-vertex-y",
    "change-vertex-z",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-weld-run",
    "remove-weld-run",
    "insert-outline-vertex",
    "remove-outline-vertex",
    "insert-placement",
    "remove-placement",
    "insert-material",
    "remove-material",
    "insert-section",
    "remove-section",
    "insert-connection",
    "remove-connection",
    "insert-fire-scenario",
    "remove-fire-scenario",
    "insert-fatigue-detail",
    "remove-fatigue-detail",
    "insert-cold-formed-sheet",
    "remove-cold-formed-sheet",
    "insert-shell",
    "remove-shell",
    "change-member-buckling-length-y",
    "change-member-buckling-length-t",
    "change-conn-bolts-bolts-per-row",
    "change-category-member-id",
    "change-category-element-id",
    "change-category-detail-category",
    "change-category-detail-type",
    "change-category-location-x",
    "change-category-m1",
    "change-category-m2",
    "change-patch-subject-id",
    "change-patch-position",
    "change-patch-x",
    "change-patch-bearing-length",
    "change-patch-web-height",
    "change-patch-web-angle",
    "change-patch-corner-radius",
    "change-patch-load",
    "change-ring-shell-id",
    "change-ring-x",
    "change-ring-depth",
    "change-ring-thickness",
    "change-ring-internal",
    "insert-fatigue-category",
    "remove-fatigue-category",
    "insert-patch-load",
    "remove-patch-load",
    "insert-ring-stiffener",
    "remove-ring-stiffener",
    "insert-sheet-profile",
    "remove-sheet-profile",
    "insert-fire-insulation",
    "remove-fire-insulation",
    "insert-member-action",
    "remove-member-action",
    "insert-connection-action",
    "remove-connection-action",
    "insert-section-element",
    "remove-section-element",
    "insert-sheet-action",
    "remove-sheet-action",
    "insert-shell-action",
    "remove-shell-action",
    "change-transverse-girder-id",
    "change-sheet-profile-sheet-id",
    "change-fire-insulation-fire-id",
    "change-transverse-position",
    "change-transverse-width",
    "change-transverse-thickness",
    "change-conn-load-eccentricity",
    "change-conn-hole-stagger",
    "change-sheet-profile-web-height",
    "change-sheet-profile-bottom-flange",
    "change-sheet-profile-top-flange",
    "change-sheet-profile-pitch",
    "change-fire-insulation-density",
    "change-fire-insulation-specific-heat",
    "change-fire-insulation-conductivity",
    "change-fire-insulation-thickness",
    "change-transverse-sides",
    "insert-transverse-stiffener",
    "remove-transverse-stiffener",
];

/// ⛔️ The refusal witnesses — `<kind>-<slug>` rows whose committed `🎯️outcome` declares the refusal and its code.
/// Registered for `mutate-` only: a refused mutation leaves nothing to undo.
#[cfg(feature = "sut")]
const ROWS: &[&str] = &[
    "insert-member-dupe",
    "remove-member-middle",
    "insert-weld-run-dupe",
    "insert-outline-vertex-dupe",
    "insert-placement-dupe",
    "insert-material-dupe",
    "insert-section-dupe",
    "insert-connection-dupe",
    "insert-fire-scenario-dupe",
    "insert-fatigue-detail-dupe",
    "insert-cold-formed-sheet-dupe",
    "insert-shell-dupe",
    "insert-fatigue-category-dupe",
    "insert-patch-load-dupe",
    "insert-ring-stiffener-dupe",
    "insert-sheet-profile-dupe",
    "insert-fire-insulation-dupe",
    "insert-member-action-dupe",
    "insert-connection-action-dupe",
    "insert-section-element-dupe",
    "insert-sheet-action-dupe",
    "insert-shell-action-dupe",
    "insert-transverse-stiffener-dupe",
];

/// 🗣️ The real committed EN 1999 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏠️aluminium-roof-purlin/🎒️.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "remove-member-middle" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member/🧭️middle/🔣️.outcome.json"),
        ),
        "change-annex" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.outcome.json"),
        ),
        "change-reliability-class" => (
            include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.outcome.json"),
        ),
        "insert-member" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member/✅apply/🔣️.outcome.json"),
        ),
        "insert-member-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-member" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member/✅apply/🔣️.outcome.json"),
        ),
        "change-member-n-ed" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-n-ed/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-n-ed/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-n-ed/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-member-n-ed/✅apply/🔣️.outcome.json"),
        ),
        "change-member-my-ed" => (
            include_str!("../🧫️fixtures/🧬️mutations/⤴️change-member-my-ed/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⤴️change-member-my-ed/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⤴️change-member-my-ed/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⤴️change-member-my-ed/✅apply/🔣️.outcome.json"),
        ),
        "change-member-buckling-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-member-buckling-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-member-buckling-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-member-buckling-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-member-buckling-length/✅apply/🔣️.outcome.json"),
        ),
        "change-material-designation" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚗️change-material-designation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚗️change-material-designation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚗️change-material-designation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚗️change-material-designation/✅apply/🔣️.outcome.json"),
        ),
        "change-plate-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-plate-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-plate-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-plate-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-plate-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-throat" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-weld-throat/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-weld-throat/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-weld-throat/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-weld-throat/✅apply/🔣️.outcome.json"),
        ),
        "change-bolt-count" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩change-bolt-count/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩change-bolt-count/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩change-bolt-count/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩change-bolt-count/✅apply/🔣️.outcome.json"),
        ),
        "change-material-product-form" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚗️change-material-product-form/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚗️change-material-product-form/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚗️change-material-product-form/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚗️change-material-product-form/✅apply/🔣️.outcome.json"),
        ),
        "change-section-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-section-height" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-height/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-height/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-height/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-height/✅apply/🔣️.outcome.json"),
        ),
        "change-section-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-width/✅apply/🔣️.outcome.json"),
        ),
        "change-section-flange-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-flange-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-flange-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-flange-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-flange-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-section-web-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-web-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-web-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-web-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-web-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-section-outer-diameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-outer-diameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-outer-diameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-outer-diameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-outer-diameter/✅apply/🔣️.outcome.json"),
        ),
        "change-element-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-width/✅apply/🔣️.outcome.json"),
        ),
        "change-element-outstand" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-outstand/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-outstand/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-outstand/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-outstand/✅apply/🔣️.outcome.json"),
        ),
        "change-element-welded" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-welded/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-welded/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-welded/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-welded/✅apply/🔣️.outcome.json"),
        ),
        "change-element-weld-position" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-position/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-position/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-position/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-position/✅apply/🔣️.outcome.json"),
        ),
        "change-element-weld-process" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-process/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-process/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-process/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-process/✅apply/🔣️.outcome.json"),
        ),
        "change-element-weld-t-junction" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-t-junction/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-t-junction/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-t-junction/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-t-junction/✅apply/🔣️.outcome.json"),
        ),
        "change-element-weld-direction" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-direction/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-direction/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-direction/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-weld-direction/✅apply/🔣️.outcome.json"),
        ),
        "change-element-stiffener-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-stiffener-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-stiffener-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-stiffener-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-stiffener-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-element-stiffener-thk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-stiffener-thk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-stiffener-thk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-stiffener-thk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-element-stiffener-thk/✅apply/🔣️.outcome.json"),
        ),
        "change-member-section-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-section-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-section-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-section-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-section-id/✅apply/🔣️.outcome.json"),
        ),
        "change-member-material-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-material-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-material-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-material-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-material-id/✅apply/🔣️.outcome.json"),
        ),
        "change-member-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-length/✅apply/🔣️.outcome.json"),
        ),
        "change-member-support" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-support/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-support/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-support/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-support/✅apply/🔣️.outcome.json"),
        ),
        "change-member-ltb-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-ltb-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-ltb-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-ltb-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-ltb-length/✅apply/🔣️.outcome.json"),
        ),
        "change-member-c1" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-c1/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-c1/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-c1/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-c1/✅apply/🔣️.outcome.json"),
        ),
        "change-member-restrained-ltb" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-restrained-ltb/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-restrained-ltb/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-restrained-ltb/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-restrained-ltb/✅apply/🔣️.outcome.json"),
        ),
        "change-member-bearing-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-bearing-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-bearing-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-bearing-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-bearing-length/✅apply/🔣️.outcome.json"),
        ),
        "change-member-transverse-weld" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-transverse-weld/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-transverse-weld/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-transverse-weld/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-transverse-weld/✅apply/🔣️.outcome.json"),
        ),
        "change-member-t-weld-offset" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-t-weld-offset/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-t-weld-offset/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-t-weld-offset/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-t-weld-offset/✅apply/🔣️.outcome.json"),
        ),
        "change-action-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-action-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-category/✅apply/🔣️.outcome.json"),
        ),
        "change-action-source" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-source/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-source/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-source/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-source/✅apply/🔣️.outcome.json"),
        ),
        "change-action-gk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-gk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-gk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-gk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-gk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-action-qk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-qk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-qk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-qk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-qk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-action-vyk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vyk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vyk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vyk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vyk/✅apply/🔣️.outcome.json"),
        ),
        "change-action-vzk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vzk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vzk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vzk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vzk/✅apply/🔣️.outcome.json"),
        ),
        "change-action-mzk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-mzk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-mzk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-mzk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-mzk/✅apply/🔣️.outcome.json"),
        ),
        "change-action-torsion-k" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-torsion-k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-torsion-k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-torsion-k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-torsion-k/✅apply/🔣️.outcome.json"),
        ),
        "change-action-local-load-k" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-local-load-k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-local-load-k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-local-load-k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-local-load-k/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-member-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-member-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-member-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-member-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-member-id/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-material-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-material-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-material-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-material-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-material-id/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-force-method" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-force-method/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-force-method/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-force-method/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-force-method/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-bolts-material" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-material/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-material/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-material/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-material/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-bolts-diameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-diameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-diameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-diameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-diameter/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-bolts-edge-distance" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-edge-distance/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-edge-distance/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-edge-distance/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-edge-distance/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-bolts-pitch" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-pitch/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-pitch/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-pitch/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-pitch/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-bolts-gauge" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-gauge/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-gauge/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-gauge/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-gauge/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-bolts-plate-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-plate-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-plate-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-plate-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-plate-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-welds-filler-alloy" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-filler-alloy/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-filler-alloy/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-filler-alloy/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-filler-alloy/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-welds-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-length/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-welds-beta-w" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-beta-w/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-beta-w/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-beta-w/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-beta-w/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-welds-haz-extent" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-haz-extent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-haz-extent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-haz-extent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-welds-haz-extent/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-category/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-source" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-source/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-source/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-source/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-source/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-gk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-gk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-gk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-gk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-gk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-qk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-qk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-qk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-qk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-qk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-nk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-nk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-nk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-nk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-nk/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-vyk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-vyk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-vyk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-vyk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-vyk/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-vzk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-vzk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-vzk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-vzk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-vzk/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-myk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-myk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-myk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-myk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-myk/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-mzk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-mzk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-mzk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-mzk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-mzk/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-torsion-k" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-torsion-k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-torsion-k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-torsion-k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-torsion-k/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-action-local-load-k" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-local-load-k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-local-load-k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-local-load-k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-action-local-load-k/✅apply/🔣️.outcome.json"),
        ),
        "change-fire-member-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-member-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-member-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-member-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-member-id/✅apply/🔣️.outcome.json"),
        ),
        "change-fire-theta-a" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-theta-a/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-theta-a/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-theta-a/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-theta-a/✅apply/🔣️.outcome.json"),
        ),
        "change-fire-duration-s" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-duration-s/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-duration-s/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-duration-s/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-duration-s/✅apply/🔣️.outcome.json"),
        ),
        "change-fatigue-member-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-member-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-member-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-member-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-member-id/✅apply/🔣️.outcome.json"),
        ),
        "change-fatigue-detail-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-detail-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-detail-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-detail-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-detail-category/✅apply/🔣️.outcome.json"),
        ),
        "change-fatigue-delta-sigma-c" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-delta-sigma-c/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-delta-sigma-c/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-delta-sigma-c/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-delta-sigma-c/✅apply/🔣️.outcome.json"),
        ),
        "change-fatigue-delta-sigma-ed" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-delta-sigma-ed/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-delta-sigma-ed/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-delta-sigma-ed/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-delta-sigma-ed/✅apply/🔣️.outcome.json"),
        ),
        "change-fatigue-n-cycles" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-n-cycles/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-n-cycles/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-n-cycles/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-n-cycles/✅apply/🔣️.outcome.json"),
        ),
        "change-fatigue-m1" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-m1/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-m1/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-m1/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-m1/✅apply/🔣️.outcome.json"),
        ),
        "change-fatigue-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-fatigue-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-material-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-material-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-material-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-material-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-material-id/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-width/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-span" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-span/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-span/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-span/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-span/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-welded" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-welded/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-welded/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-welded/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-welded/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-category/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-source" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-source/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-source/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-source/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-source/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-gk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-gk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-gk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-gk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-gk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-qk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-qk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-qk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-qk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-qk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-nk" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-nk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-nk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-nk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-nk/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-vyk" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-vyk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-vyk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-vyk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-vyk/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-vzk" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-vzk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-vzk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-vzk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-vzk/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-myk" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-myk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-myk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-myk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-myk/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-mzk" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-mzk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-mzk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-mzk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-mzk/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-torsion-k" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-torsion-k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-torsion-k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-torsion-k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-torsion-k/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-action-local-load-k" => (
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-local-load-k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-local-load-k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-local-load-k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/❄️change-sheet-action-local-load-k/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-material-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-material-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-material-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-material-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-material-id/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-radius" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-radius/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-radius/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-radius/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-radius/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-length/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-welded" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-welded/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-welded/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-welded/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-welded/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-fabrication-class" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-fabrication-class/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-fabrication-class/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-fabrication-class/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-fabrication-class/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-category/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-source" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-source/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-source/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-source/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-source/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-gk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-gk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-gk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-gk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-gk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-qk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-qk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-qk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-qk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-qk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-nk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-nk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-nk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-nk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-nk/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-vyk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-vyk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-vyk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-vyk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-vyk/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-vzk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-vzk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-vzk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-vzk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-vzk/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-myk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-myk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-myk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-myk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-myk/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-mzk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-mzk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-mzk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-mzk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-mzk/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-torsion-k" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-torsion-k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-torsion-k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-torsion-k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-torsion-k/✅apply/🔣️.outcome.json"),
        ),
        "change-shell-action-local-load-k" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-local-load-k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-local-load-k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-local-load-k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙️change-shell-action-local-load-k/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-member-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-member-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-member-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-member-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-member-id/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-element-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-element-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-element-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-element-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-element-id/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-start" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-start/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-start/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-start/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-start/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-length/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-offset" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-offset/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-offset/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-offset/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-offset/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-throat" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-throat/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-throat/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-throat/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-throat/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-process" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-process/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-process/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-process/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-process/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-t-junction" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-t-junction/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-t-junction/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-t-junction/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-t-junction/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-direction" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-direction/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-direction/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-direction/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-direction/✅apply/🔣️.outcome.json"),
        ),
        "change-weld-run-haz-extent" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-haz-extent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-haz-extent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-haz-extent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-weld-run-haz-extent/✅apply/🔣️.outcome.json"),
        ),
        "change-vertex-section-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-section-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-section-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-section-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-section-id/✅apply/🔣️.outcome.json"),
        ),
        "change-vertex-ring" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-ring/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-ring/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-ring/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-ring/✅apply/🔣️.outcome.json"),
        ),
        "change-vertex-element-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-element-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-element-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-element-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-element-id/✅apply/🔣️.outcome.json"),
        ),
        "change-vertex-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-y/✅apply/🔣️.outcome.json"),
        ),
        "change-vertex-z" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-z/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-z/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-z/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔷️change-vertex-z/✅apply/🔣️.outcome.json"),
        ),
        "change-placement-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-x/✅apply/🔣️.outcome.json"),
        ),
        "change-placement-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-y/✅apply/🔣️.outcome.json"),
        ),
        "change-placement-z" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-z/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-z/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-z/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-z/✅apply/🔣️.outcome.json"),
        ),
        "change-placement-yaw" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-yaw/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-yaw/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-yaw/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-placement-yaw/✅apply/🔣️.outcome.json"),
        ),
        "insert-weld-run" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-weld-run/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-weld-run/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-weld-run/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-weld-run/✅apply/🔣️.outcome.json"),
        ),
        "remove-weld-run" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-weld-run/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-weld-run/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-weld-run/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-weld-run/✅apply/🔣️.outcome.json"),
        ),
        "insert-outline-vertex" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-outline-vertex/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-outline-vertex/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-outline-vertex/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-outline-vertex/✅apply/🔣️.outcome.json"),
        ),
        "remove-outline-vertex" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-outline-vertex/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-outline-vertex/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-outline-vertex/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-outline-vertex/✅apply/🔣️.outcome.json"),
        ),
        "insert-placement" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-placement/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-placement/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-placement/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-placement/✅apply/🔣️.outcome.json"),
        ),
        "remove-placement" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-placement/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-placement/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-placement/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-placement/✅apply/🔣️.outcome.json"),
        ),
        "insert-weld-run-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-weld-run/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-weld-run/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-weld-run/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-weld-run/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-outline-vertex-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-outline-vertex/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-outline-vertex/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-outline-vertex/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-outline-vertex/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-placement-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-placement/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-placement/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-placement/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-placement/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-material" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-material/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-material/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-material/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-material/✅apply/🔣️.outcome.json"),
        ),
        "remove-material" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-material/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-material/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-material/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-material/✅apply/🔣️.outcome.json"),
        ),
        "insert-section" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section/✅apply/🔣️.outcome.json"),
        ),
        "remove-section" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-section/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-section/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-section/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-section/✅apply/🔣️.outcome.json"),
        ),
        "insert-connection" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection/✅apply/🔣️.outcome.json"),
        ),
        "remove-connection" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-connection/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-connection/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-connection/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-connection/✅apply/🔣️.outcome.json"),
        ),
        "insert-fire-scenario" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-scenario/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-scenario/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-scenario/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-scenario/✅apply/🔣️.outcome.json"),
        ),
        "remove-fire-scenario" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fire-scenario/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fire-scenario/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fire-scenario/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fire-scenario/✅apply/🔣️.outcome.json"),
        ),
        "insert-fatigue-detail" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-detail/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-detail/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-detail/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-detail/✅apply/🔣️.outcome.json"),
        ),
        "remove-fatigue-detail" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fatigue-detail/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fatigue-detail/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fatigue-detail/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fatigue-detail/✅apply/🔣️.outcome.json"),
        ),
        "insert-cold-formed-sheet" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-cold-formed-sheet/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-cold-formed-sheet/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-cold-formed-sheet/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-cold-formed-sheet/✅apply/🔣️.outcome.json"),
        ),
        "remove-cold-formed-sheet" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-cold-formed-sheet/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-cold-formed-sheet/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-cold-formed-sheet/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-cold-formed-sheet/✅apply/🔣️.outcome.json"),
        ),
        "insert-shell" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell/✅apply/🔣️.outcome.json"),
        ),
        "remove-shell" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-shell/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-shell/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-shell/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-shell/✅apply/🔣️.outcome.json"),
        ),
        "insert-material-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-material/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-material/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-material/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-material/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-section-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-connection-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-fire-scenario-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-scenario/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-scenario/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-scenario/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-scenario/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-fatigue-detail-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-detail/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-detail/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-detail/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-detail/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-cold-formed-sheet-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-cold-formed-sheet/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-cold-formed-sheet/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-cold-formed-sheet/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-cold-formed-sheet/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-shell-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell/⛔dupe/🔣️.outcome.json"),
        ),
        "change-member-buckling-length-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-buckling-length-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-buckling-length-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-buckling-length-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-buckling-length-y/✅apply/🔣️.outcome.json"),
        ),
        "change-member-buckling-length-t" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-buckling-length-t/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-buckling-length-t/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-buckling-length-t/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-member-buckling-length-t/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-bolts-bolts-per-row" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-bolts-per-row/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-bolts-per-row/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-bolts-per-row/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔗️change-conn-bolts-bolts-per-row/✅apply/🔣️.outcome.json"),
        ),
        "change-category-member-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-member-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-member-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-member-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-member-id/✅apply/🔣️.outcome.json"),
        ),
        "change-category-element-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-element-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-element-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-element-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-element-id/✅apply/🔣️.outcome.json"),
        ),
        "change-category-detail-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-detail-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-detail-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-detail-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-detail-category/✅apply/🔣️.outcome.json"),
        ),
        "change-category-detail-type" => (
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-detail-type/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-detail-type/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-detail-type/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-detail-type/✅apply/🔣️.outcome.json"),
        ),
        "change-category-location-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-location-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-location-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-location-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-location-x/✅apply/🔣️.outcome.json"),
        ),
        "change-category-m1" => (
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-m1/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-m1/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-m1/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-m1/✅apply/🔣️.outcome.json"),
        ),
        "change-category-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯️change-category-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-patch-subject-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-subject-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-subject-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-subject-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-subject-id/✅apply/🔣️.outcome.json"),
        ),
        "change-patch-position" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-position/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-position/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-position/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-position/✅apply/🔣️.outcome.json"),
        ),
        "change-patch-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-x/✅apply/🔣️.outcome.json"),
        ),
        "change-patch-bearing-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-bearing-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-bearing-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-bearing-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-bearing-length/✅apply/🔣️.outcome.json"),
        ),
        "change-patch-web-height" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-web-height/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-web-height/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-web-height/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-web-height/✅apply/🔣️.outcome.json"),
        ),
        "change-patch-web-angle" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-web-angle/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-web-angle/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-web-angle/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-web-angle/✅apply/🔣️.outcome.json"),
        ),
        "change-patch-corner-radius" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-corner-radius/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-corner-radius/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-corner-radius/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-corner-radius/✅apply/🔣️.outcome.json"),
        ),
        "change-patch-load" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-load/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-load/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-load/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-patch-load/✅apply/🔣️.outcome.json"),
        ),
        "change-ring-shell-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-shell-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-shell-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-shell-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-shell-id/✅apply/🔣️.outcome.json"),
        ),
        "change-ring-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-x/✅apply/🔣️.outcome.json"),
        ),
        "change-ring-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-ring-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-ring-internal" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-internal/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-internal/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-internal/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-ring-internal/✅apply/🔣️.outcome.json"),
        ),
        "insert-fatigue-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-category/✅apply/🔣️.outcome.json"),
        ),
        "remove-fatigue-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fatigue-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fatigue-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fatigue-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fatigue-category/✅apply/🔣️.outcome.json"),
        ),
        "insert-patch-load" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-patch-load/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-patch-load/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-patch-load/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-patch-load/✅apply/🔣️.outcome.json"),
        ),
        "remove-patch-load" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-patch-load/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-patch-load/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-patch-load/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-patch-load/✅apply/🔣️.outcome.json"),
        ),
        "insert-ring-stiffener" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ring-stiffener/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ring-stiffener/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ring-stiffener/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ring-stiffener/✅apply/🔣️.outcome.json"),
        ),
        "remove-ring-stiffener" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-ring-stiffener/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-ring-stiffener/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-ring-stiffener/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-ring-stiffener/✅apply/🔣️.outcome.json"),
        ),
        "insert-sheet-profile" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-profile/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-profile/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-profile/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-profile/✅apply/🔣️.outcome.json"),
        ),
        "remove-sheet-profile" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-sheet-profile/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-sheet-profile/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-sheet-profile/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-sheet-profile/✅apply/🔣️.outcome.json"),
        ),
        "insert-fire-insulation" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-insulation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-insulation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-insulation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-insulation/✅apply/🔣️.outcome.json"),
        ),
        "remove-fire-insulation" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fire-insulation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fire-insulation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fire-insulation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-fire-insulation/✅apply/🔣️.outcome.json"),
        ),
        "insert-fatigue-category-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-category/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-category/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-category/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fatigue-category/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-patch-load-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-patch-load/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-patch-load/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-patch-load/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-patch-load/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-ring-stiffener-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ring-stiffener/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ring-stiffener/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ring-stiffener/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ring-stiffener/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-sheet-profile-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-profile/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-profile/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-profile/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-profile/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-fire-insulation-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-insulation/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-insulation/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-insulation/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-fire-insulation/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-member-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member-action/✅apply/🔣️.outcome.json"),
        ),
        "remove-member-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-member-action/✅apply/🔣️.outcome.json"),
        ),
        "insert-connection-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection-action/✅apply/🔣️.outcome.json"),
        ),
        "remove-connection-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-connection-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-connection-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-connection-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-connection-action/✅apply/🔣️.outcome.json"),
        ),
        "insert-section-element" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section-element/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section-element/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section-element/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section-element/✅apply/🔣️.outcome.json"),
        ),
        "remove-section-element" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-section-element/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-section-element/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-section-element/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-section-element/✅apply/🔣️.outcome.json"),
        ),
        "insert-sheet-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-action/✅apply/🔣️.outcome.json"),
        ),
        "remove-sheet-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-sheet-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-sheet-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-sheet-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-sheet-action/✅apply/🔣️.outcome.json"),
        ),
        "insert-shell-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell-action/✅apply/🔣️.outcome.json"),
        ),
        "remove-shell-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-shell-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-shell-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-shell-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-shell-action/✅apply/🔣️.outcome.json"),
        ),
        "insert-member-action-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member-action/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member-action/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member-action/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-member-action/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-connection-action-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection-action/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection-action/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection-action/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-connection-action/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-section-element-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section-element/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section-element/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section-element/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-section-element/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-sheet-action-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-action/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-action/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-action/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-sheet-action/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-shell-action-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell-action/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell-action/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell-action/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-shell-action/⛔dupe/🔣️.outcome.json"),
        ),
        "change-transverse-girder-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-girder-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-girder-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-girder-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-girder-id/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-profile-sheet-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-sheet-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-sheet-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-sheet-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-sheet-id/✅apply/🔣️.outcome.json"),
        ),
        "change-fire-insulation-fire-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-fire-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-fire-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-fire-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-fire-id/✅apply/🔣️.outcome.json"),
        ),
        "change-transverse-position" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-position/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-position/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-position/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-position/✅apply/🔣️.outcome.json"),
        ),
        "change-transverse-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-width/✅apply/🔣️.outcome.json"),
        ),
        "change-transverse-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-load-eccentricity" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-conn-load-eccentricity/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-conn-load-eccentricity/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-conn-load-eccentricity/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-conn-load-eccentricity/✅apply/🔣️.outcome.json"),
        ),
        "change-conn-hole-stagger" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-conn-hole-stagger/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-conn-hole-stagger/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-conn-hole-stagger/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-conn-hole-stagger/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-profile-web-height" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-web-height/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-web-height/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-web-height/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-web-height/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-profile-bottom-flange" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-bottom-flange/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-bottom-flange/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-bottom-flange/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-bottom-flange/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-profile-top-flange" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-top-flange/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-top-flange/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-top-flange/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-top-flange/✅apply/🔣️.outcome.json"),
        ),
        "change-sheet-profile-pitch" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-pitch/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-pitch/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-pitch/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-sheet-profile-pitch/✅apply/🔣️.outcome.json"),
        ),
        "change-fire-insulation-density" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-density/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-density/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-density/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-density/✅apply/🔣️.outcome.json"),
        ),
        "change-fire-insulation-specific-heat" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-specific-heat/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-specific-heat/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-specific-heat/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-specific-heat/✅apply/🔣️.outcome.json"),
        ),
        "change-fire-insulation-conductivity" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-conductivity/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-conductivity/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-conductivity/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-conductivity/✅apply/🔣️.outcome.json"),
        ),
        "change-fire-insulation-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-fire-insulation-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-transverse-sides" => (
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-sides/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-sides/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-sides/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🦴️change-transverse-sides/✅apply/🔣️.outcome.json"),
        ),
        "insert-transverse-stiffener" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-transverse-stiffener/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-transverse-stiffener/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-transverse-stiffener/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-transverse-stiffener/✅apply/🔣️.outcome.json"),
        ),
        "remove-transverse-stiffener" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-transverse-stiffener/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-transverse-stiffener/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-transverse-stiffener/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-transverse-stiffener/✅apply/🔣️.outcome.json"),
        ),
        "insert-transverse-stiffener-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-transverse-stiffener/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-transverse-stiffener/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-transverse-stiffener/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-transverse-stiffener/⛔dupe/🔣️.outcome.json"),
        ),
        other => panic!("mutate-en1999-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_en1999_mutations::En1999Mutation;
    use semio_s_artifact_norm_en1999_mutations::mutation_bridge::{apply_en1999_mutation, inverse_en1999_mutation};

    use semio_s_artifact_norm_en1999_io::text::mutations::decode_en1999_mutation_json;
    use semio_s_artifact_norm_en1999_model::En1999Snapshot;
    use semio_s_artifact_norm_en1999_io::binary::snapshot::encode_en1999_pack;
    use semio_s_artifact_norm_en1999_io::binary::snapshot::decode_en1999_pack;
    use semio_s_artifact_norm_en1999_io::text::snapshot::encode_en1999_dsl;
    use semio_s_artifact_norm_en1999_io::text::snapshot::decode_en1999_dsl;
    use semio_s_artifact_norm_en1999_io::text::snapshot::decode_en1999_snapshot_json;
    use semio_s_artifact_norm_en1999_io::text::snapshot::encode_en1999_snapshot_json;
    use semio_repo_test_host::law;

    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1999Snapshot, String> {
        decode_en1999_snapshot_json(text).map_err(|error| format!("mutate-en1999-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1999Mutation, String> {
        decode_en1999_mutation_json(text).map_err(|error| format!("mutate-en1999-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1999Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1999_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1999Snapshot, expected: &En1999Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1999_snapshot_json(got), encode_en1999_snapshot_json(expected))
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
            let applied = apply_en1999_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected" | "no-op", Ok((snapshot, messages))) if !messages.iter().any(|message| message.ends_with(&format!(":{code}"))) => return Err(format!("mutate-{kind}: the committed vector declares {status} with {code}, yet it raised {messages:?} — the document came back as {}", encode_en1999_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1999_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1999_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1999_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.input_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1999 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1999_dsl(&text)?;
        let reprinted = encode_en1999_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1999_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1999_pack(&encode_en1999_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1999_snapshot_json(&encode_en1999_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1999_pack(&ctx.input_bytes(super::PACK_ASSET)?)?;
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
