//! 🦀️ EN 1994 exhaustive mutation case — the Rust SUBJECT adapter over the current 31-kind `En1994Mutation`
//! vocabulary. Every kind applies its committed vector (`../../🧫️fixtures/🧬️mutations/<leaf>/<scenario>`) through the
//! subset's production bridges and asserts, in role, the committed after-snapshot, that the document moved and that
//! its own inverse restores the before-snapshot; `identity-round-trip` re-emits the committed carrier through the DSL,
//! pack and JSON codecs. The reference answer is the shared Python norm engine (`../🐍️.py`).

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{digest, parse_json, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `En1994Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-annex",
    "change-reliability-class",
    "change-structure-kind",
    "change-steel-fy-pa",
    "change-fire-rating",
    "change-insulation-thickness-m",
    "change-fatigue-detail",
    "insert-beam",
    "remove-beam",
    "change-beam-action-q-area-pa",
    "change-beam-span-m",
    "change-beam-slab-thickness-m",
    "change-beam-transverse-as",
    "change-beam-construction",
    "insert-column",
    "remove-column",
    "change-column-action-force-n",
    "change-column-kind",
    "insert-slab",
    "remove-slab",
    "change-slab-action-q-area-pa",
    "change-slab-thickness-m",
    "change-beam-spacing-m",
    "change-beam-support",
    "change-beam-rebar-product",
    "change-beam-concrete-f-ck-pa",
    "change-beam-concrete-e-cm-pa",
    "change-beam-ltb-length-m",
    "change-beam-hogging-rebar-m2-per-m",
    "change-beam-bar-spacing-m",
    "change-beam-crack-width-limit-m",
    "change-beam-fatigue-cycles",
    "change-beam-steel-designation",
    "change-beam-steel-height-m",
    "change-beam-steel-width-m",
    "change-beam-steel-web-thickness-m",
    "change-beam-steel-tf-m",
    "change-beam-steel-area-m2",
    "change-beam-steel-w-pl-ym3",
    "change-beam-steel-second-moment-m4",
    "change-beam-steel-shear-area-m2",
    "change-beam-steel-root-radius-m",
    "change-beam-sheeting-profile",
    "change-beam-sheeting-height-m",
    "change-beam-sheeting-rib-width-m",
    "change-beam-sheeting-thickness-m",
    "change-beam-sheeting-ribs-parallel",
    "change-beam-action-kind",
    "change-beam-action-category",
    "change-beam-action-stage",
    "change-beam-action-point-force-n",
    "change-beam-action-delta-sigma-pa",
    "change-beam-action-delta-tau-pa",
    "change-column-length-m",
    "change-column-outer-size-m",
    "change-column-wall-thickness-m",
    "change-column-steel-area-m2",
    "change-column-steel-fy-pa",
    "change-column-concrete-area-m2",
    "change-column-concrete-f-ck-pa",
    "change-column-rebar-area-m2",
    "change-column-rebar-f-yk-pa",
    "change-column-second-moment-m4",
    "change-column-buckling-curve",
    "change-column-rebar-product",
    "change-column-action-kind",
    "change-column-action-category",
    "change-column-action-stage",
    "change-column-action-moment-nm",
    "change-slab-span-m",
    "change-slab-support",
    "change-slab-f-ck-pa",
    "change-slab-m-factor",
    "change-slab-k-factor",
    "change-slab-reinforcement-m2-per-m",
    "change-slab-sheeting-profile",
    "change-slab-sheeting-height-m",
    "change-slab-sheeting-rib-width-m",
    "change-slab-sheeting-thickness-m",
    "change-slab-sheeting-ribs-parallel",
    "change-slab-action-kind",
    "change-slab-action-category",
    "change-slab-action-stage",
    "change-slab-action-point-force-n",
    "change-slab-action-delta-sigma-pa",
    "change-slab-action-delta-tau-pa",
    "change-beam-adjacent-span-m",
    "change-beam-patch-bearing-length-m",
    "change-beam-bridge-lanes",
    "change-column-introduced-force-n",
    "change-beam-cracked-length-fraction",
    "change-column-section-height-m",
    "change-column-section-width-m",
    "change-column-section-tw-m",
    "change-column-section-tf-m",
    "change-column-rebar-cover-m",
    "change-beam-stud-cross-spacing-m",
    "change-beam-stud-edge-distance-m",
    "change-beam-transverse-as-top",
    "change-beam-end-plate-width-m",
    "change-beam-end-plate-height-m",
    "change-beam-end-zone-length-m",
    "change-stud-fu-pa",
    "change-stud-height-m",
    "change-stud-diameter-m",
    "change-stud-xm",
    "insert-beam-action",
    "remove-beam-action",
    "insert-column-action",
    "remove-column-action",
    "insert-slab-action",
    "remove-slab-action",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-placement",
    "remove-placement",
    "insert-stud",
    "remove-stud",
    "insert-case",
    "remove-case",
    "change-case-number",
    "change-case-text",
    "change-case-label-en",
    "change-case-label-de",
    "change-case-entry-id",
    "change-beam-action-xm",
    "change-slab-action-xm",
    "change-slab-action-ym",
];

/// ⛔️ The refusal witnesses — `<kind>-<slug>` rows whose committed `🎯️outcome` declares the refusal and its code.
/// Registered for `mutate-` only: a refused mutation leaves nothing to undo.
#[cfg(feature = "sut")]
const ROWS: &[&str] = &[
    "remove-column-middle",
    "remove-slab-middle",
    "remove-beam-middle",
];

/// 🗣️ The real committed EN 1994 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🌉️composite-bridge-girder/🎒️.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "remove-column-middle" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛔️remove-column/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛔️remove-column/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛔️remove-column/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛔️remove-column/🧭️middle/🔣️.outcome.json"),
        ),
        "remove-slab-middle" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slab/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slab/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slab/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slab/🧭️middle/🔣️.outcome.json"),
        ),
        "remove-beam-middle" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-beam/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-beam/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-beam/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-beam/🧭️middle/🔣️.outcome.json"),
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
        "change-structure-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-structure-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-structure-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-structure-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-structure-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-steel-fy-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-steel-fy-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-steel-fy-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-steel-fy-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-steel-fy-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-fire-rating" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-rating/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-rating/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-rating/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-fire-rating/✅apply/🔣️.outcome.json"),
        ),
        "change-insulation-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-fatigue-detail" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-fatigue-detail/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-fatigue-detail/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-fatigue-detail/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-fatigue-detail/✅apply/🔣️.outcome.json"),
        ),
        "insert-beam" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-beam/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-beam/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-beam/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-beam/✅apply/🔣️.outcome.json"),
        ),
        "remove-beam" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-beam/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-beam/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-beam/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-beam/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-action-q-area-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-beam-action-q-area-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-beam-action-q-area-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-beam-action-q-area-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-beam-action-q-area-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-span-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-span-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-span-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-span-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-span-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-slab-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-beam-slab-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-beam-slab-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-beam-slab-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-beam-slab-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-transverse-as" => (
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-transverse-as/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-transverse-as/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-transverse-as/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-transverse-as/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-construction" => (
            include_str!("../🧫️fixtures/🧬️mutations/🛠️change-beam-construction/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛠️change-beam-construction/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛠️change-beam-construction/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛠️change-beam-construction/✅apply/🔣️.outcome.json"),
        ),
        "insert-column" => (
            include_str!("../🧫️fixtures/🧬️mutations/➗️insert-column/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➗️insert-column/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➗️insert-column/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➗️insert-column/✅apply/🔣️.outcome.json"),
        ),
        "remove-column" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛔️remove-column/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛔️remove-column/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛔️remove-column/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛔️remove-column/✅apply/🔣️.outcome.json"),
        ),
        "change-column-action-force-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-column-action-force-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-column-action-force-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-column-action-force-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-column-action-force-n/✅apply/🔣️.outcome.json"),
        ),
        "change-column-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/↪️change-column-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↪️change-column-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↪️change-column-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↪️change-column-kind/✅apply/🔣️.outcome.json"),
        ),
        "insert-slab" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slab/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slab/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slab/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slab/✅apply/🔣️.outcome.json"),
        ),
        "remove-slab" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slab/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slab/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slab/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slab/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-action-q-area-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-slab-action-q-area-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-slab-action-q-area-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-slab-action-q-area-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-slab-action-q-area-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏change-slab-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏change-slab-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏change-slab-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏change-slab-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-spacing-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-spacing-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-spacing-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-spacing-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-spacing-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-support" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪢️change-beam-support/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪢️change-beam-support/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪢️change-beam-support/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪢️change-beam-support/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-rebar-product" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-beam-rebar-product/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-beam-rebar-product/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-beam-rebar-product/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-beam-rebar-product/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-concrete-f-ck-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-beam-concrete-f-ck-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-beam-concrete-f-ck-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-beam-concrete-f-ck-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-beam-concrete-f-ck-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-concrete-e-cm-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-concrete-e-cm-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-concrete-e-cm-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-concrete-e-cm-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-concrete-e-cm-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-ltb-length-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-beam-ltb-length-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-beam-ltb-length-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-beam-ltb-length-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-beam-ltb-length-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-hogging-rebar-m2-per-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-beam-hogging-rebar-m2-per-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-beam-hogging-rebar-m2-per-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-beam-hogging-rebar-m2-per-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-beam-hogging-rebar-m2-per-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-bar-spacing-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-bar-spacing-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-bar-spacing-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-bar-spacing-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-bar-spacing-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-crack-width-limit-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-beam-crack-width-limit-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-beam-crack-width-limit-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-beam-crack-width-limit-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-beam-crack-width-limit-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-fatigue-cycles" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-fatigue-cycles/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-fatigue-cycles/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-fatigue-cycles/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-fatigue-cycles/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-designation" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-beam-steel-designation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-beam-steel-designation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-beam-steel-designation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-beam-steel-designation/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-beam-steel-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-beam-steel-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-beam-steel-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-beam-steel-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-steel-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-steel-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-steel-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-steel-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-web-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-steel-web-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-steel-web-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-steel-web-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-steel-web-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-tf-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-steel-tf-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-steel-tf-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-steel-tf-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-beam-steel-tf-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬛️change-beam-steel-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬛️change-beam-steel-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬛️change-beam-steel-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬛️change-beam-steel-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-w-pl-ym3" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-beam-steel-w-pl-ym3/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-beam-steel-w-pl-ym3/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-beam-steel-w-pl-ym3/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-beam-steel-w-pl-ym3/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-second-moment-m4" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-beam-steel-second-moment-m4/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-beam-steel-second-moment-m4/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-beam-steel-second-moment-m4/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-beam-steel-second-moment-m4/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-shear-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/✂️change-beam-steel-shear-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✂️change-beam-steel-shear-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✂️change-beam-steel-shear-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✂️change-beam-steel-shear-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-steel-root-radius-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-beam-steel-root-radius-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-beam-steel-root-radius-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-beam-steel-root-radius-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-beam-steel-root-radius-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-sheeting-profile" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-profile/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-profile/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-profile/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-profile/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-sheeting-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-sheeting-rib-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-rib-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-rib-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-rib-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-rib-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-sheeting-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-beam-sheeting-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-sheeting-ribs-parallel" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-beam-sheeting-ribs-parallel/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-beam-sheeting-ribs-parallel/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-beam-sheeting-ribs-parallel/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-beam-sheeting-ribs-parallel/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-action-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-beam-action-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-beam-action-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-beam-action-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-beam-action-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-action-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-beam-action-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-beam-action-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-beam-action-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-beam-action-category/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-action-stage" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-beam-action-stage/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-beam-action-stage/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-beam-action-stage/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-beam-action-stage/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-action-point-force-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-beam-action-point-force-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-beam-action-point-force-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-beam-action-point-force-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-beam-action-point-force-n/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-action-delta-sigma-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-action-delta-sigma-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-action-delta-sigma-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-action-delta-sigma-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-action-delta-sigma-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-action-delta-tau-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-action-delta-tau-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-action-delta-tau-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-action-delta-tau-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-beam-action-delta-tau-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-column-length-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-column-length-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-column-length-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-column-length-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-column-length-m/✅apply/🔣️.outcome.json"),
        ),
        "change-column-outer-size-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬜️change-column-outer-size-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬜️change-column-outer-size-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬜️change-column-outer-size-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬜️change-column-outer-size-m/✅apply/🔣️.outcome.json"),
        ),
        "change-column-wall-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-column-wall-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-column-wall-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-column-wall-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-column-wall-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-column-steel-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬛️change-column-steel-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬛️change-column-steel-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬛️change-column-steel-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬛️change-column-steel-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-column-steel-fy-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-column-steel-fy-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-column-steel-fy-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-column-steel-fy-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-column-steel-fy-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-column-concrete-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-column-concrete-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-column-concrete-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-column-concrete-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-column-concrete-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-column-concrete-f-ck-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-column-concrete-f-ck-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-column-concrete-f-ck-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-column-concrete-f-ck-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-column-concrete-f-ck-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-column-rebar-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-column-rebar-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-column-rebar-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-column-rebar-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-column-rebar-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-column-rebar-f-yk-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-column-rebar-f-yk-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-column-rebar-f-yk-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-column-rebar-f-yk-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-column-rebar-f-yk-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-column-second-moment-m4" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-column-second-moment-m4/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-column-second-moment-m4/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-column-second-moment-m4/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-column-second-moment-m4/✅apply/🔣️.outcome.json"),
        ),
        "change-column-buckling-curve" => (
            include_str!("../🧫️fixtures/🧬️mutations/📉️change-column-buckling-curve/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📉️change-column-buckling-curve/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📉️change-column-buckling-curve/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📉️change-column-buckling-curve/✅apply/🔣️.outcome.json"),
        ),
        "change-column-rebar-product" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-column-rebar-product/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-column-rebar-product/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-column-rebar-product/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-column-rebar-product/✅apply/🔣️.outcome.json"),
        ),
        "change-column-action-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-column-action-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-column-action-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-column-action-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-column-action-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-column-action-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-column-action-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-column-action-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-column-action-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-column-action-category/✅apply/🔣️.outcome.json"),
        ),
        "change-column-action-stage" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-column-action-stage/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-column-action-stage/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-column-action-stage/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-column-action-stage/✅apply/🔣️.outcome.json"),
        ),
        "change-column-action-moment-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-column-action-moment-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-column-action-moment-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-column-action-moment-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-column-action-moment-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-span-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-slab-span-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-slab-span-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-slab-span-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-slab-span-m/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-support" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪢️change-slab-support/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪢️change-slab-support/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪢️change-slab-support/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪢️change-slab-support/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-f-ck-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-slab-f-ck-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-slab-f-ck-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-slab-f-ck-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨️change-slab-f-ck-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-m-factor" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-slab-m-factor/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-slab-m-factor/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-slab-m-factor/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-slab-m-factor/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-k-factor" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-slab-k-factor/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-slab-k-factor/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-slab-k-factor/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-slab-k-factor/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-reinforcement-m2-per-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-slab-reinforcement-m2-per-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-slab-reinforcement-m2-per-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-slab-reinforcement-m2-per-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-slab-reinforcement-m2-per-m/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-sheeting-profile" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-profile/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-profile/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-profile/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-profile/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-sheeting-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-sheeting-rib-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-rib-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-rib-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-rib-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-rib-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-sheeting-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-slab-sheeting-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-sheeting-ribs-parallel" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-slab-sheeting-ribs-parallel/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-slab-sheeting-ribs-parallel/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-slab-sheeting-ribs-parallel/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-slab-sheeting-ribs-parallel/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-action-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-slab-action-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-slab-action-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-slab-action-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-slab-action-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-action-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-slab-action-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-slab-action-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-slab-action-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-slab-action-category/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-action-stage" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-slab-action-stage/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-slab-action-stage/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-slab-action-stage/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-slab-action-stage/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-action-point-force-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-slab-action-point-force-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-slab-action-point-force-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-slab-action-point-force-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-slab-action-point-force-n/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-action-delta-sigma-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-slab-action-delta-sigma-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-slab-action-delta-sigma-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-slab-action-delta-sigma-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-slab-action-delta-sigma-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-action-delta-tau-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-slab-action-delta-tau-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-slab-action-delta-tau-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-slab-action-delta-tau-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔁️change-slab-action-delta-tau-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-adjacent-span-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-adjacent-span-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-adjacent-span-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-adjacent-span-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-adjacent-span-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-patch-bearing-length-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-beam-patch-bearing-length-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-beam-patch-bearing-length-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-beam-patch-bearing-length-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-beam-patch-bearing-length-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-bridge-lanes" => (
            include_str!("../🧫️fixtures/🧬️mutations/🛣️change-beam-bridge-lanes/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛣️change-beam-bridge-lanes/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛣️change-beam-bridge-lanes/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛣️change-beam-bridge-lanes/✅apply/🔣️.outcome.json"),
        ),
        "change-column-introduced-force-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-column-introduced-force-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-column-introduced-force-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-column-introduced-force-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-column-introduced-force-n/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-cracked-length-fraction" => (
            include_str!("../🧫️fixtures/🧬️mutations/〰️change-beam-cracked-length-fraction/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/〰️change-beam-cracked-length-fraction/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/〰️change-beam-cracked-length-fraction/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/〰️change-beam-cracked-length-fraction/✅apply/🔣️.outcome.json"),
        ),
        "change-column-section-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-column-section-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-column-section-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-column-section-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-column-section-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-column-section-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-column-section-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-column-section-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-column-section-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-column-section-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-column-section-tw-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/▮change-column-section-tw-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/▮change-column-section-tw-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/▮change-column-section-tw-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/▮change-column-section-tw-m/✅apply/🔣️.outcome.json"),
        ),
        "change-column-section-tf-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/▬change-column-section-tf-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/▬change-column-section-tf-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/▬change-column-section-tf-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/▬change-column-section-tf-m/✅apply/🔣️.outcome.json"),
        ),
        "change-column-rebar-cover-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔘change-column-rebar-cover-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔘change-column-rebar-cover-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔘change-column-rebar-cover-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔘change-column-rebar-cover-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-stud-cross-spacing-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-stud-cross-spacing-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-stud-cross-spacing-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-stud-cross-spacing-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-beam-stud-cross-spacing-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-stud-edge-distance-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-stud-edge-distance-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-stud-edge-distance-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-stud-edge-distance-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-beam-stud-edge-distance-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-transverse-as-top" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-beam-transverse-as-top/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-beam-transverse-as-top/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-beam-transverse-as-top/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕸️change-beam-transverse-as-top/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-end-plate-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬜️change-beam-end-plate-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬜️change-beam-end-plate-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬜️change-beam-end-plate-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬜️change-beam-end-plate-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-end-plate-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-beam-end-plate-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-beam-end-plate-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-beam-end-plate-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-beam-end-plate-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-end-zone-length-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-beam-end-zone-length-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-beam-end-zone-length-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-beam-end-zone-length-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-beam-end-zone-length-m/✅apply/🔣️.outcome.json"),
        ),
        "change-stud-fu-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-fu-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-fu-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-fu-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-fu-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-stud-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-stud-diameter-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-diameter-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-diameter-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-diameter-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-diameter-m/✅apply/🔣️.outcome.json"),
        ),
        "change-stud-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➡️change-stud-xm/✅apply/🔣️.outcome.json"),
        ),
        "insert-beam-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-beam-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-beam-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-beam-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-beam-action/✅apply/🔣️.outcome.json"),
        ),
        "remove-beam-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-beam-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-beam-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-beam-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-beam-action/✅apply/🔣️.outcome.json"),
        ),
        "insert-column-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️insert-column-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️insert-column-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️insert-column-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️insert-column-action/✅apply/🔣️.outcome.json"),
        ),
        "remove-column-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️remove-column-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️remove-column-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️remove-column-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️remove-column-action/✅apply/🔣️.outcome.json"),
        ),
        "insert-slab-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-slab-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-slab-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-slab-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-slab-action/✅apply/🔣️.outcome.json"),
        ),
        "remove-slab-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️remove-slab-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️remove-slab-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️remove-slab-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️remove-slab-action/✅apply/🔣️.outcome.json"),
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
        "insert-placement" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-placement/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-placement/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-placement/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-placement/✅apply/🔣️.outcome.json"),
        ),
        "remove-placement" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-placement/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-placement/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-placement/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-placement/✅apply/🔣️.outcome.json"),
        ),
        "insert-stud" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-stud/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-stud/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-stud/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-stud/✅apply/🔣️.outcome.json"),
        ),
        "remove-stud" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-stud/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-stud/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-stud/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-stud/✅apply/🔣️.outcome.json"),
        ),
        "change-beam-action-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-beam-action-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-beam-action-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-beam-action-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-beam-action-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-action-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-slab-action-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-slab-action-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-slab-action-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-slab-action-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-slab-action-ym" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-slab-action-ym/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-slab-action-ym/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-slab-action-ym/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-slab-action-ym/✅apply/🔣️.outcome.json"),
        ),
        "insert-case" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-case/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-case/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-case/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-case/✅apply/🔣️.outcome.json"),
        ),
        "remove-case" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-case/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-case/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-case/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-case/✅apply/🔣️.outcome.json"),
        ),
        "change-case-number" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔢️change-case-number/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔢️change-case-number/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔢️change-case-number/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔢️change-case-number/✅apply/🔣️.outcome.json"),
        ),
        "change-case-text" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔤️change-case-text/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔤️change-case-text/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔤️change-case-text/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔤️change-case-text/✅apply/🔣️.outcome.json"),
        ),
        "change-case-label-en" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-en/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-en/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-en/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-en/✅apply/🔣️.outcome.json"),
        ),
        "change-case-label-de" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-de/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-de/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-de/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-case-label-de/✅apply/🔣️.outcome.json"),
        ),
        "change-case-entry-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/📇️change-case-entry-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📇️change-case-entry-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📇️change-case-entry-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📇️change-case-entry-id/✅apply/🔣️.outcome.json"),
        ),
        other => panic!("mutate-en1994-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_en1994_mutations::En1994Mutation;
    use semio_s_artifact_norm_en1994_mutations::mutation_bridge::{apply_en1994_mutation, inverse_en1994_mutation};

    use semio_s_artifact_norm_en1994_io::text::mutations::decode_en1994_mutation_json;
    use semio_s_artifact_norm_en1994_model::En1994Snapshot;
    use semio_s_artifact_norm_en1994_io::binary::snapshot::encode_en1994_pack;
    use semio_s_artifact_norm_en1994_io::binary::snapshot::decode_en1994_pack;
    use semio_s_artifact_norm_en1994_io::text::snapshot::encode_en1994_dsl;
    use semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl;
    use semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_snapshot_json;
    use semio_s_artifact_norm_en1994_io::text::snapshot::encode_en1994_snapshot_json;
    use semio_repo_test_host::law;

    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1994Snapshot, String> {
        decode_en1994_snapshot_json(text).map_err(|error| format!("mutate-en1994-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1994Mutation, String> {
        decode_en1994_mutation_json(text).map_err(|error| format!("mutate-en1994-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1994Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1994_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1994Snapshot, expected: &En1994Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1994_snapshot_json(got), encode_en1994_snapshot_json(expected))
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
            let applied = apply_en1994_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected" | "no-op", Ok((snapshot, messages))) if !messages.iter().any(|message| message.ends_with(&format!(":{code}"))) => return Err(format!("mutate-{kind}: the committed vector declares {status} with {code}, yet it raised {messages:?} — the document came back as {}", encode_en1994_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1994_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1994_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1994_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.input_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1994 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1994_dsl(&text)?;
        let reprinted = encode_en1994_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1994_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1994_pack(&encode_en1994_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1994_snapshot_json(&encode_en1994_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1994_pack(&ctx.input_bytes(super::PACK_ASSET)?)?;
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
