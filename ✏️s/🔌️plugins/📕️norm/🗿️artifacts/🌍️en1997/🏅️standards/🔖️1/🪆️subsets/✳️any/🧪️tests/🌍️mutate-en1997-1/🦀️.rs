//! 🦀️ EN 1997 exhaustive mutation case — the Rust SUBJECT adapter over the current 160-kind `En1997Mutation`
//! vocabulary. Every kind applies its committed vector (`../../🧫️fixtures/🧬️mutations/<leaf>/<scenario>`) through the
//! subset's production bridges and asserts, in role, the committed after-snapshot, that the document moved and that
//! its own inverse restores the before-snapshot; `identity-round-trip` re-emits the committed carrier through the DSL,
//! pack and JSON codecs. The reference answer is the shared Python norm engine (`../🐍️.py`).

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{digest, parse_json, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `En1997Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-annex",
    "change-geotechnical-category",
    "change-design-situation",
    "change-design-approach",
    "change-groundwater-level",
    "change-investigation-depth",
    "change-footing-width",
    "change-footing-embedment",
    "change-pile-length",
    "change-pile-count",
    "change-wall-base-width",
    "change-slope-angle",
    "change-layer-phi-prime",
    "change-layer-oedometric-modulus",
    "insert-layer",
    "remove-layer",
    "insert-footing",
    "remove-footing",
    "insert-pile",
    "remove-pile",
    "insert-retaining-wall",
    "remove-retaining-wall",
    "insert-slope",
    "remove-slope",
    "insert-uplift-case",
    "remove-uplift-case",
    "insert-anchor",
    "remove-anchor",
    "insert-embedded-wall",
    "remove-embedded-wall",
    "insert-pile-group",
    "remove-pile-group",
    "insert-lab-test",
    "remove-lab-test",
    "insert-layer-water",
    "remove-layer-water",
    "insert-characteristic-basis",
    "remove-characteristic-basis",
    "insert-ground-investigation",
    "remove-ground-investigation",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-placement",
    "remove-placement",
    "change-layer-soil-type",
    "change-layer-depth-top",
    "change-layer-depth-bottom",
    "change-layer-gamma",
    "change-layer-gamma-prime",
    "change-layer-cohesion-effective",
    "change-layer-cohesion-undrained",
    "change-layer-poisson-ratio",
    "change-layer-cpt-qc",
    "change-layer-spt-n",
    "change-footing-length",
    "change-footing-base-incl-deg",
    "change-footing-settlement-limit",
    "change-load-case-design-situation",
    "change-load-case-vertical-perm",
    "change-load-case-vertical-var",
    "change-load-case-horiz-permanent",
    "change-load-case-horiz-variable",
    "change-load-case-moment-permanent",
    "change-load-case-moment-variable",
    "change-pile-pile-type",
    "change-pile-diameter",
    "change-pile-alpha-s",
    "change-pile-unit-shaft-resistance",
    "change-pile-unit-base-resistance",
    "change-pile-compression-permanent",
    "change-pile-compression-variable",
    "change-pile-tension-permanent",
    "change-pile-tension-variable",
    "change-profile-shaft-resistance",
    "change-profile-base-resistance",
    "change-wall-height",
    "change-wall-embedment",
    "change-wall-stem-thickness",
    "change-wall-backfill-phi-deg",
    "change-wall-backfill-gamma",
    "change-wall-wall-friction-deg",
    "change-wall-earth-pressure-mode",
    "change-wall-wall-movement",
    "change-wall-ocr",
    "change-wall-concrete-gamma",
    "change-wall-surcharge",
    "change-wall-vertical-permanent",
    "change-wall-horizontal-permanent",
    "change-slope-height",
    "change-slope-length",
    "change-slope-governing-layer-id",
    "change-slope-circle-centre-x",
    "change-slope-circle-centre-z",
    "change-slope-circle-radius",
    "change-uplift-perm-stabilizing",
    "change-uplift-perm-destabilizing",
    "change-uplift-var-destabilizing",
    "change-uplift-pore-pressure",
    "change-uplift-total-stress",
    "change-anchor-wall-id",
    "change-anchor-tendon",
    "change-anchor-head-depth",
    "change-anchor-inclination-deg",
    "change-anchor-free-length",
    "change-anchor-fixed-length",
    "change-anchor-fixed-diameter",
    "change-anchor-bond-stress",
    "change-anchor-design-force",
    "change-anchor-tendon-yield-force",
    "change-anchor-min-free-length",
    "change-anchor-test-factor",
    "change-anchor-test-pull-out-res",
    "change-emb-wall-kind",
    "change-emb-wall-retained-height",
    "change-emb-wall-embedment",
    "change-emb-wall-prop-depth",
    "change-emb-wall-thickness",
    "change-emb-wall-backfill-gamma",
    "change-emb-wall-phi-deg",
    "change-emb-wall-delta-active-deg",
    "change-emb-wall-delta-passive-deg",
    "change-emb-wall-surcharge",
    "change-emb-wall-undr-strength",
    "change-pile-group-pile-id",
    "change-pile-group-rows",
    "change-pile-group-columns",
    "change-pile-group-spacing",
    "change-pile-group-cap-vertical",
    "change-pile-group-cap-moment-x",
    "change-pile-group-cap-moment-y",
    "change-pile-group-single-res",
    "change-pile-group-shaft-coh-sum",
    "change-pile-group-base-cohesion",
    "change-lab-test-layer-id",
    "change-lab-test-kind",
    "change-lab-test-void-ratio",
    "change-lab-test-preload-stress",
    "change-lab-test-drainage-height",
    "change-lab-test-consol-time",
    "change-lab-point-normal-stress",
    "change-lab-point-shear-stress",
    "change-water-layer-id",
    "change-water-piezometric-depth",
    "change-water-permeability",
    "change-water-head-difference",
    "change-water-flow-length",
    "change-structure-id",
    "insert-load-case",
    "remove-load-case",
    "insert-test-profile",
    "remove-test-profile",
    "insert-anchor-test",
    "remove-anchor-test",
    "insert-lab-point",
    "remove-lab-point",
    "insert-ground-investigation-reading",
    "remove-ground-investigation-reading",
    "insert-case",
    "remove-case",
    "change-case-number",
    "change-case-text",
    "change-case-label-en",
    "change-case-label-de",
    "change-case-entry-id",
    "change-slope-width",
    "change-pile-bending-stiffness-ei",
    "change-pile-horiz-bed-modulus",
    "change-pile-lateral-head-load",
    "change-wall-seismic-kh",
    "change-wall-seismic-kv",
    "change-pile-elastic-modulus",
    "change-pile-settling-thickness",
    "change-pile-neg-friction-beta",
    "change-pile-dynamic-test-count",
    "change-pile-dynamic-res-mean",
    "change-pile-dynamic-res-min",
    "change-pile-hammer-weight",
    "change-pile-drop-height",
    "change-pile-set-per-blow",
    "change-pile-elastic-compression",
    "change-pile-hammer-efficiency",
    "change-wall-reinforcement-spacing",
    "change-wall-reinforcement-width",
    "change-wall-rebar-friction",
    "change-lab-test-compression-index",
    "change-lab-test-swelling-index",
    "change-lab-test-stress-increment",
    "change-lab-test-sec-compr-index",
    "change-lab-test-design-life",
    "change-water-well-radius",
    "change-water-well-water-depth",
    "change-water-influence-radius",
    "change-water-well-capacity",
    "change-pile-concrete-class",
    "change-ground-investigation-position-x",
    "change-ground-investigation-position-y",
    "change-ground-investigation-level",
    "change-ground-investigation-depth",
    "change-ground-investigation-kind",
    "change-ground-reading-depth",
    "change-ground-reading-measurement",
    "change-characteristic-basis-layer-id",
    "change-characteristic-basis-parameter",
    "change-characteristic-basis-basis",
    "change-layer-cpt-nkt",
];

/// ⛔️ The refusal witnesses — `<kind>-<slug>` rows whose committed `🎯️outcome` declares the refusal and its code.
/// Registered for `mutate-` only: a refused mutation leaves nothing to undo.
#[cfg(feature = "sut")]
const ROWS: &[&str] = &[
    "insert-layer-dupe",
    "insert-footing-dupe",
    "insert-pile-dupe",
    "remove-footing-middle",
    "remove-layer-middle",
    "remove-pile-middle",
    "insert-retaining-wall-dupe",
    "insert-slope-dupe",
    "insert-uplift-case-dupe",
    "insert-anchor-dupe",
    "insert-embedded-wall-dupe",
    "insert-pile-group-dupe",
    "insert-lab-test-dupe",
    "insert-layer-water-dupe",
    "insert-characteristic-basis-dupe",
    "insert-ground-investigation-dupe",
];

/// 🗣️ The real committed EN 1997 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🎬️demo/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🎬️demo/📦️.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "remove-footing-middle" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-footing/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-footing/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-footing/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-footing/🧭️middle/🔣️.outcome.json"),
        ),
        "remove-layer-middle" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-layer/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-layer/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-layer/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-layer/🧭️middle/🔣️.outcome.json"),
        ),
        "remove-pile-middle" => (
            include_str!("../🧫️fixtures/🧬️mutations/📤remove-pile/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📤remove-pile/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📤remove-pile/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📤remove-pile/🧭️middle/🔣️.outcome.json"),
        ),
        "change-annex" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.outcome.json"),
        ),
        "change-geotechnical-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗂️change-geotechnical-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗂️change-geotechnical-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗂️change-geotechnical-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗂️change-geotechnical-category/✅apply/🔣️.outcome.json"),
        ),
        "change-design-situation" => (
            include_str!("../🧫️fixtures/🧬️mutations/📅️change-design-situation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📅️change-design-situation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📅️change-design-situation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📅️change-design-situation/✅apply/🔣️.outcome.json"),
        ),
        "change-design-approach" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-design-approach/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-design-approach/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-design-approach/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-design-approach/✅apply/🔣️.outcome.json"),
        ),
        "change-groundwater-level" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧change-groundwater-level/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧change-groundwater-level/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧change-groundwater-level/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧change-groundwater-level/✅apply/🔣️.outcome.json"),
        ),
        "change-investigation-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔎️change-investigation-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔎️change-investigation-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔎️change-investigation-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔎️change-investigation-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-footing-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-footing-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-footing-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-footing-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-footing-width/✅apply/🔣️.outcome.json"),
        ),
        "change-footing-embedment" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-footing-embedment/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-footing-embedment/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-footing-embedment/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬇️change-footing-embedment/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-pile-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-pile-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-pile-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-pile-length/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-count" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔢change-pile-count/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔢change-pile-count/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔢change-pile-count/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔢change-pile-count/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-base-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-wall-base-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-wall-base-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-wall-base-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-wall-base-width/✅apply/🔣️.outcome.json"),
        ),
        "change-slope-angle" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-angle/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-angle/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-angle/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-angle/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-phi-prime" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-layer-phi-prime/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-layer-phi-prime/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-layer-phi-prime/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-layer-phi-prime/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-oedometric-modulus" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-layer-oedometric-modulus/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-layer-oedometric-modulus/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-layer-oedometric-modulus/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌀️change-layer-oedometric-modulus/✅apply/🔣️.outcome.json"),
        ),
        "insert-layer" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-layer/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-layer/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-layer/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-layer/✅apply/🔣️.outcome.json"),
        ),
        "insert-layer-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-layer/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-layer/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-layer/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-layer/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-layer" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-layer/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-layer/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-layer/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-layer/✅apply/🔣️.outcome.json"),
        ),
        "insert-footing" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-footing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-footing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-footing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-footing/✅apply/🔣️.outcome.json"),
        ),
        "insert-footing-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-footing/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-footing/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-footing/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-footing/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-footing" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-footing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-footing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-footing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-footing/✅apply/🔣️.outcome.json"),
        ),
        "insert-pile" => (
            include_str!("../🧫️fixtures/🧬️mutations/📥insert-pile/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📥insert-pile/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📥insert-pile/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📥insert-pile/✅apply/🔣️.outcome.json"),
        ),
        "insert-pile-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/📥insert-pile/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📥insert-pile/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📥insert-pile/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📥insert-pile/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-pile" => (
            include_str!("../🧫️fixtures/🧬️mutations/📤remove-pile/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📤remove-pile/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📤remove-pile/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📤remove-pile/✅apply/🔣️.outcome.json"),
        ),
        "insert-retaining-wall" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-retaining-wall/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-retaining-wall/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-retaining-wall/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-retaining-wall/✅apply/🔣️.outcome.json"),
        ),
        "insert-retaining-wall-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-retaining-wall/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-retaining-wall/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-retaining-wall/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-retaining-wall/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-retaining-wall" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-retaining-wall/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-retaining-wall/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-retaining-wall/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-retaining-wall/✅apply/🔣️.outcome.json"),
        ),
        "insert-slope" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slope/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slope/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slope/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slope/✅apply/🔣️.outcome.json"),
        ),
        "insert-slope-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slope/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slope/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slope/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-slope/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-slope" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slope/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slope/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slope/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-slope/✅apply/🔣️.outcome.json"),
        ),
        "insert-uplift-case" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-uplift-case/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-uplift-case/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-uplift-case/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-uplift-case/✅apply/🔣️.outcome.json"),
        ),
        "insert-uplift-case-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-uplift-case/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-uplift-case/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-uplift-case/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-uplift-case/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-uplift-case" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-uplift-case/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-uplift-case/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-uplift-case/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-uplift-case/✅apply/🔣️.outcome.json"),
        ),
        "insert-anchor" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-anchor/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-anchor/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-anchor/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-anchor/✅apply/🔣️.outcome.json"),
        ),
        "insert-anchor-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-anchor/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-anchor/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-anchor/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-anchor/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-anchor" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-anchor/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-anchor/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-anchor/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-anchor/✅apply/🔣️.outcome.json"),
        ),
        "insert-embedded-wall" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-embedded-wall/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-embedded-wall/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-embedded-wall/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-embedded-wall/✅apply/🔣️.outcome.json"),
        ),
        "insert-embedded-wall-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-embedded-wall/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-embedded-wall/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-embedded-wall/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-embedded-wall/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-embedded-wall" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-embedded-wall/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-embedded-wall/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-embedded-wall/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-embedded-wall/✅apply/🔣️.outcome.json"),
        ),
        "insert-pile-group" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-pile-group/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-pile-group/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-pile-group/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-pile-group/✅apply/🔣️.outcome.json"),
        ),
        "insert-pile-group-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-pile-group/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-pile-group/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-pile-group/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-pile-group/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-pile-group" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-pile-group/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-pile-group/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-pile-group/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-pile-group/✅apply/🔣️.outcome.json"),
        ),
        "insert-lab-test" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-lab-test/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-lab-test/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-lab-test/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-lab-test/✅apply/🔣️.outcome.json"),
        ),
        "insert-lab-test-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-lab-test/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-lab-test/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-lab-test/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-lab-test/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-lab-test" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-lab-test/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-lab-test/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-lab-test/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-lab-test/✅apply/🔣️.outcome.json"),
        ),
        "insert-layer-water" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-layer-water/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-layer-water/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-layer-water/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-layer-water/✅apply/🔣️.outcome.json"),
        ),
        "insert-layer-water-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-layer-water/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-layer-water/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-layer-water/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-layer-water/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-characteristic-basis" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-characteristic-basis/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-characteristic-basis/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-characteristic-basis/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-characteristic-basis/✅apply/🔣️.outcome.json"),
        ),
        "insert-characteristic-basis-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-characteristic-basis/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-characteristic-basis/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-characteristic-basis/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-characteristic-basis/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-ground-investigation" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ground-investigation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ground-investigation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ground-investigation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ground-investigation/✅apply/🔣️.outcome.json"),
        ),
        "insert-ground-investigation-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ground-investigation/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ground-investigation/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ground-investigation/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕insert-ground-investigation/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-layer-water" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-layer-water/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-layer-water/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-layer-water/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-layer-water/✅apply/🔣️.outcome.json"),
        ),
        "remove-characteristic-basis" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-characteristic-basis/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-characteristic-basis/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-characteristic-basis/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-characteristic-basis/✅apply/🔣️.outcome.json"),
        ),
        "remove-ground-investigation" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-ground-investigation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-ground-investigation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-ground-investigation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖remove-ground-investigation/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-soil-type" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-soil-type/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-soil-type/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-soil-type/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-soil-type/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-depth-top" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-depth-top/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-depth-top/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-depth-top/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-depth-top/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-depth-bottom" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-depth-bottom/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-depth-bottom/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-depth-bottom/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-depth-bottom/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-gamma" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-gamma/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-gamma/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-gamma/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-gamma/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-gamma-prime" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-gamma-prime/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-gamma-prime/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-gamma-prime/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-gamma-prime/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-cohesion-effective" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cohesion-effective/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cohesion-effective/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cohesion-effective/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cohesion-effective/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-cohesion-undrained" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cohesion-undrained/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cohesion-undrained/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cohesion-undrained/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cohesion-undrained/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-poisson-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-poisson-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-poisson-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-poisson-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-poisson-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-cpt-qc" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cpt-qc/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cpt-qc/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cpt-qc/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cpt-qc/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-spt-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-spt-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-spt-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-spt-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-spt-n/✅apply/🔣️.outcome.json"),
        ),
        "change-footing-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-length/✅apply/🔣️.outcome.json"),
        ),
        "change-footing-base-incl-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-base-incl-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-base-incl-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-base-incl-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-base-incl-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-footing-settlement-limit" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-settlement-limit/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-settlement-limit/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-settlement-limit/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-footing-settlement-limit/✅apply/🔣️.outcome.json"),
        ),
        "change-load-case-design-situation" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-design-situation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-design-situation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-design-situation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-design-situation/✅apply/🔣️.outcome.json"),
        ),
        "change-load-case-vertical-perm" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-vertical-perm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-vertical-perm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-vertical-perm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-vertical-perm/✅apply/🔣️.outcome.json"),
        ),
        "change-load-case-vertical-var" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-vertical-var/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-vertical-var/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-vertical-var/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-vertical-var/✅apply/🔣️.outcome.json"),
        ),
        "change-load-case-horiz-permanent" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-horiz-permanent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-horiz-permanent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-horiz-permanent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-horiz-permanent/✅apply/🔣️.outcome.json"),
        ),
        "change-load-case-horiz-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-horiz-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-horiz-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-horiz-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-horiz-variable/✅apply/🔣️.outcome.json"),
        ),
        "change-load-case-moment-permanent" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-moment-permanent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-moment-permanent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-moment-permanent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-moment-permanent/✅apply/🔣️.outcome.json"),
        ),
        "change-load-case-moment-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-moment-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-moment-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-moment-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-load-case-moment-variable/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-pile-type" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-pile-type/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-pile-type/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-pile-type/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-pile-type/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-diameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-diameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-diameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-diameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-diameter/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-alpha-s" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-alpha-s/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-alpha-s/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-alpha-s/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-alpha-s/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-unit-shaft-resistance" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-unit-shaft-resistance/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-unit-shaft-resistance/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-unit-shaft-resistance/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-unit-shaft-resistance/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-unit-base-resistance" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-unit-base-resistance/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-unit-base-resistance/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-unit-base-resistance/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-unit-base-resistance/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-compression-permanent" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-compression-permanent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-compression-permanent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-compression-permanent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-compression-permanent/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-compression-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-compression-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-compression-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-compression-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-compression-variable/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-tension-permanent" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-tension-permanent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-tension-permanent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-tension-permanent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-tension-permanent/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-tension-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-tension-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-tension-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-tension-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-tension-variable/✅apply/🔣️.outcome.json"),
        ),
        "change-profile-shaft-resistance" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-profile-shaft-resistance/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-profile-shaft-resistance/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-profile-shaft-resistance/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-profile-shaft-resistance/✅apply/🔣️.outcome.json"),
        ),
        "change-profile-base-resistance" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-profile-base-resistance/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-profile-base-resistance/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-profile-base-resistance/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-profile-base-resistance/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-height" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-height/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-height/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-height/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-height/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-embedment" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-embedment/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-embedment/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-embedment/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-embedment/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-stem-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-stem-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-stem-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-stem-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-stem-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-backfill-phi-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-backfill-phi-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-backfill-phi-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-backfill-phi-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-backfill-phi-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-backfill-gamma" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-backfill-gamma/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-backfill-gamma/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-backfill-gamma/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-backfill-gamma/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-wall-friction-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-wall-friction-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-wall-friction-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-wall-friction-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-wall-friction-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-earth-pressure-mode" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-earth-pressure-mode/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-earth-pressure-mode/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-earth-pressure-mode/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-earth-pressure-mode/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-wall-movement" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-wall-movement/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-wall-movement/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-wall-movement/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-wall-movement/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-ocr" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-ocr/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-ocr/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-ocr/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-ocr/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-concrete-gamma" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-concrete-gamma/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-concrete-gamma/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-concrete-gamma/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-concrete-gamma/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-surcharge" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-surcharge/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-surcharge/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-surcharge/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-surcharge/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-vertical-permanent" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-vertical-permanent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-vertical-permanent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-vertical-permanent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-vertical-permanent/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-horizontal-permanent" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-horizontal-permanent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-horizontal-permanent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-horizontal-permanent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-horizontal-permanent/✅apply/🔣️.outcome.json"),
        ),
        "change-slope-height" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-height/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-height/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-height/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-height/✅apply/🔣️.outcome.json"),
        ),
        "change-slope-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-length/✅apply/🔣️.outcome.json"),
        ),
        "change-slope-governing-layer-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-governing-layer-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-governing-layer-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-governing-layer-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-governing-layer-id/✅apply/🔣️.outcome.json"),
        ),
        "change-slope-circle-centre-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-centre-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-centre-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-centre-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-centre-x/✅apply/🔣️.outcome.json"),
        ),
        "change-slope-circle-centre-z" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-centre-z/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-centre-z/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-centre-z/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-centre-z/✅apply/🔣️.outcome.json"),
        ),
        "change-slope-circle-radius" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-radius/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-radius/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-radius/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-circle-radius/✅apply/🔣️.outcome.json"),
        ),
        "change-uplift-perm-stabilizing" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-perm-stabilizing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-perm-stabilizing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-perm-stabilizing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-perm-stabilizing/✅apply/🔣️.outcome.json"),
        ),
        "change-uplift-perm-destabilizing" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-perm-destabilizing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-perm-destabilizing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-perm-destabilizing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-perm-destabilizing/✅apply/🔣️.outcome.json"),
        ),
        "change-uplift-var-destabilizing" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-var-destabilizing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-var-destabilizing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-var-destabilizing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-var-destabilizing/✅apply/🔣️.outcome.json"),
        ),
        "change-uplift-pore-pressure" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-pore-pressure/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-pore-pressure/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-pore-pressure/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-pore-pressure/✅apply/🔣️.outcome.json"),
        ),
        "change-uplift-total-stress" => (
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-total-stress/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-total-stress/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-total-stress/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⬆️change-uplift-total-stress/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-wall-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-wall-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-wall-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-wall-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-wall-id/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-tendon" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-tendon/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-tendon/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-tendon/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-tendon/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-head-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-head-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-head-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-head-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-head-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-inclination-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-inclination-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-inclination-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-inclination-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-inclination-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-free-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-free-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-free-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-free-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-free-length/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-fixed-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-fixed-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-fixed-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-fixed-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-fixed-length/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-fixed-diameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-fixed-diameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-fixed-diameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-fixed-diameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-fixed-diameter/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-bond-stress" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-bond-stress/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-bond-stress/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-bond-stress/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-bond-stress/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-design-force" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-design-force/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-design-force/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-design-force/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-design-force/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-tendon-yield-force" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-tendon-yield-force/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-tendon-yield-force/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-tendon-yield-force/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-tendon-yield-force/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-min-free-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-min-free-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-min-free-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-min-free-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-min-free-length/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-test-factor" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-test-factor/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-test-factor/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-test-factor/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-test-factor/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-test-pull-out-res" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-test-pull-out-res/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-test-pull-out-res/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-test-pull-out-res/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-test-pull-out-res/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-retained-height" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-retained-height/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-retained-height/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-retained-height/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-retained-height/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-embedment" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-embedment/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-embedment/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-embedment/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-embedment/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-prop-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-prop-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-prop-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-prop-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-prop-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-backfill-gamma" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-backfill-gamma/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-backfill-gamma/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-backfill-gamma/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-backfill-gamma/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-phi-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-phi-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-phi-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-phi-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-phi-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-delta-active-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-delta-active-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-delta-active-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-delta-active-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-delta-active-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-delta-passive-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-delta-passive-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-delta-passive-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-delta-passive-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-delta-passive-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-surcharge" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-surcharge/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-surcharge/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-surcharge/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-surcharge/✅apply/🔣️.outcome.json"),
        ),
        "change-emb-wall-undr-strength" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-undr-strength/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-undr-strength/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-undr-strength/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-emb-wall-undr-strength/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-pile-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-pile-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-pile-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-pile-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-pile-id/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-rows" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-rows/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-rows/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-rows/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-rows/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-columns" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-columns/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-columns/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-columns/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-columns/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-spacing" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-spacing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-spacing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-spacing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-spacing/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-cap-vertical" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-vertical/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-vertical/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-vertical/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-vertical/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-cap-moment-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-moment-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-moment-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-moment-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-moment-x/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-cap-moment-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-moment-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-moment-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-moment-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-cap-moment-y/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-single-res" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-single-res/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-single-res/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-single-res/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-single-res/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-shaft-coh-sum" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-shaft-coh-sum/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-shaft-coh-sum/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-shaft-coh-sum/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-shaft-coh-sum/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-group-base-cohesion" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-base-cohesion/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-base-cohesion/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-base-cohesion/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-pile-group-base-cohesion/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-layer-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-layer-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-layer-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-layer-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-layer-id/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-void-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-void-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-void-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-void-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-void-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-preload-stress" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-preload-stress/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-preload-stress/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-preload-stress/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-preload-stress/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-drainage-height" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-drainage-height/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-drainage-height/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-drainage-height/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-drainage-height/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-consol-time" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-consol-time/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-consol-time/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-consol-time/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-consol-time/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-point-normal-stress" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-point-normal-stress/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-point-normal-stress/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-point-normal-stress/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-point-normal-stress/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-point-shear-stress" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-point-shear-stress/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-point-shear-stress/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-point-shear-stress/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-point-shear-stress/✅apply/🔣️.outcome.json"),
        ),
        "change-water-layer-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-layer-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-layer-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-layer-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-layer-id/✅apply/🔣️.outcome.json"),
        ),
        "change-water-piezometric-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-piezometric-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-piezometric-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-piezometric-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-piezometric-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-water-permeability" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-permeability/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-permeability/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-permeability/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-permeability/✅apply/🔣️.outcome.json"),
        ),
        "change-water-head-difference" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-head-difference/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-head-difference/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-head-difference/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-head-difference/✅apply/🔣️.outcome.json"),
        ),
        "change-water-flow-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-flow-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-flow-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-flow-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-flow-length/✅apply/🔣️.outcome.json"),
        ),
        "change-structure-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-structure-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-structure-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-structure-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏗️change-structure-id/✅apply/🔣️.outcome.json"),
        ),
        "insert-load-case" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-load-case/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-load-case/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-load-case/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-load-case/✅apply/🔣️.outcome.json"),
        ),
        "remove-load-case" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/🔣️.outcome.json"),
        ),
        "insert-test-profile" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-test-profile/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-test-profile/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-test-profile/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-test-profile/✅apply/🔣️.outcome.json"),
        ),
        "remove-test-profile" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-test-profile/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-test-profile/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-test-profile/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-test-profile/✅apply/🔣️.outcome.json"),
        ),
        "insert-anchor-test" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-anchor-test/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-anchor-test/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-anchor-test/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-anchor-test/✅apply/🔣️.outcome.json"),
        ),
        "remove-anchor-test" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-anchor-test/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-anchor-test/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-anchor-test/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-anchor-test/✅apply/🔣️.outcome.json"),
        ),
        "insert-lab-point" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-lab-point/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-lab-point/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-lab-point/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-lab-point/✅apply/🔣️.outcome.json"),
        ),
        "remove-lab-point" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-lab-point/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-lab-point/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-lab-point/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-lab-point/✅apply/🔣️.outcome.json"),
        ),
        "insert-ground-investigation-reading" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-ground-investigation-reading/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-ground-investigation-reading/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-ground-investigation-reading/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-ground-investigation-reading/✅apply/🔣️.outcome.json"),
        ),
        "remove-ground-investigation-reading" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-ground-investigation-reading/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-ground-investigation-reading/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-ground-investigation-reading/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-ground-investigation-reading/✅apply/🔣️.outcome.json"),
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
        "change-slope-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⛰️change-slope-width/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-bending-stiffness-ei" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-bending-stiffness-ei/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-bending-stiffness-ei/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-bending-stiffness-ei/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-bending-stiffness-ei/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-horiz-bed-modulus" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-horiz-bed-modulus/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-horiz-bed-modulus/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-horiz-bed-modulus/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-horiz-bed-modulus/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-lateral-head-load" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-lateral-head-load/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-lateral-head-load/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-lateral-head-load/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-lateral-head-load/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-seismic-kh" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-seismic-kh/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-seismic-kh/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-seismic-kh/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-seismic-kh/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-seismic-kv" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-seismic-kv/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-seismic-kv/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-seismic-kv/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-seismic-kv/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-elastic-modulus" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-elastic-modulus/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-elastic-modulus/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-elastic-modulus/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-elastic-modulus/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-settling-thickness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-settling-thickness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-settling-thickness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-settling-thickness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-settling-thickness/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-neg-friction-beta" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-neg-friction-beta/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-neg-friction-beta/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-neg-friction-beta/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-neg-friction-beta/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-dynamic-test-count" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-test-count/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-test-count/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-test-count/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-test-count/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-dynamic-res-mean" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-res-mean/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-res-mean/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-res-mean/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-res-mean/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-dynamic-res-min" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-res-min/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-res-min/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-res-min/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-dynamic-res-min/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-hammer-weight" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-hammer-weight/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-hammer-weight/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-hammer-weight/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-hammer-weight/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-drop-height" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-drop-height/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-drop-height/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-drop-height/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-drop-height/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-set-per-blow" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-set-per-blow/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-set-per-blow/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-set-per-blow/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-set-per-blow/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-elastic-compression" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-elastic-compression/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-elastic-compression/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-elastic-compression/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-elastic-compression/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-hammer-efficiency" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-hammer-efficiency/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-hammer-efficiency/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-hammer-efficiency/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-hammer-efficiency/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-reinforcement-spacing" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-reinforcement-spacing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-reinforcement-spacing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-reinforcement-spacing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-reinforcement-spacing/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-reinforcement-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-reinforcement-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-reinforcement-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-reinforcement-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-reinforcement-width/✅apply/🔣️.outcome.json"),
        ),
        "change-wall-rebar-friction" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-rebar-friction/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-rebar-friction/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-rebar-friction/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗄️change-wall-rebar-friction/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-compression-index" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-compression-index/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-compression-index/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-compression-index/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-compression-index/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-swelling-index" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-swelling-index/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-swelling-index/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-swelling-index/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-swelling-index/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-stress-increment" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-stress-increment/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-stress-increment/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-stress-increment/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-stress-increment/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-sec-compr-index" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-sec-compr-index/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-sec-compr-index/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-sec-compr-index/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-sec-compr-index/✅apply/🔣️.outcome.json"),
        ),
        "change-lab-test-design-life" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-design-life/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-design-life/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-design-life/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪️change-lab-test-design-life/✅apply/🔣️.outcome.json"),
        ),
        "change-water-well-radius" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-radius/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-radius/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-radius/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-radius/✅apply/🔣️.outcome.json"),
        ),
        "change-water-well-water-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-water-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-water-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-water-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-water-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-water-influence-radius" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-influence-radius/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-influence-radius/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-influence-radius/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-influence-radius/✅apply/🔣️.outcome.json"),
        ),
        "change-water-well-capacity" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-capacity/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-capacity/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-capacity/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧️change-water-well-capacity/✅apply/🔣️.outcome.json"),
        ),
        "change-pile-concrete-class" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-concrete-class/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-concrete-class/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-concrete-class/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔄️change-pile-concrete-class/✅apply/🔣️.outcome.json"),
        ),
        "change-ground-investigation-position-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-position-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-position-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-position-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-position-x/✅apply/🔣️.outcome.json"),
        ),
        "change-ground-investigation-position-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-position-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-position-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-position-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-position-y/✅apply/🔣️.outcome.json"),
        ),
        "change-ground-investigation-level" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-level/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-level/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-level/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-level/✅apply/🔣️.outcome.json"),
        ),
        "change-ground-investigation-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-ground-investigation-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-ground-investigation-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-ground-reading-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/📈️change-ground-reading-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📈️change-ground-reading-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📈️change-ground-reading-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📈️change-ground-reading-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-ground-reading-measurement" => (
            include_str!("../🧫️fixtures/🧬️mutations/📈️change-ground-reading-measurement/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📈️change-ground-reading-measurement/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📈️change-ground-reading-measurement/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📈️change-ground-reading-measurement/✅apply/🔣️.outcome.json"),
        ),
        "change-characteristic-basis-layer-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-layer-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-layer-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-layer-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-layer-id/✅apply/🔣️.outcome.json"),
        ),
        "change-characteristic-basis-parameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-parameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-parameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-parameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-parameter/✅apply/🔣️.outcome.json"),
        ),
        "change-characteristic-basis-basis" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-basis/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-basis/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-basis/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧮️change-characteristic-basis-basis/✅apply/🔣️.outcome.json"),
        ),
        "change-layer-cpt-nkt" => (
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cpt-nkt/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cpt-nkt/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cpt-nkt/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🥞️change-layer-cpt-nkt/✅apply/🔣️.outcome.json"),
        ),
        other => panic!("mutate-en1997-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_en1997_mutations::En1997Mutation;
    use semio_s_artifact_norm_en1997_mutations::mutation_bridge::{apply_en1997_mutation, inverse_en1997_mutation};

    use semio_s_artifact_norm_en1997_io::text::mutations::decode_en1997_mutation_json;
    use semio_s_artifact_norm_en1997_model::En1997Snapshot;
    use semio_s_artifact_norm_en1997_io::binary::snapshot::encode_en1997_pack;
    use semio_s_artifact_norm_en1997_io::binary::snapshot::decode_en1997_pack;
    use semio_s_artifact_norm_en1997_io::text::snapshot::encode_en1997_dsl;
    use semio_s_artifact_norm_en1997_io::text::snapshot::decode_en1997_dsl;
    use semio_s_artifact_norm_en1997_io::text::snapshot::decode_en1997_snapshot_json;
    use semio_s_artifact_norm_en1997_io::text::snapshot::encode_en1997_snapshot_json;
    use semio_repo_test_host::law;

    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1997Snapshot, String> {
        decode_en1997_snapshot_json(text).map_err(|error| format!("mutate-en1997-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1997Mutation, String> {
        decode_en1997_mutation_json(text).map_err(|error| format!("mutate-en1997-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1997Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1997_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1997Snapshot, expected: &En1997Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1997_snapshot_json(got), encode_en1997_snapshot_json(expected))
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
            let applied = apply_en1997_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected" | "no-op", Ok((snapshot, messages))) if !messages.iter().any(|message| message.ends_with(&format!(":{code}"))) => return Err(format!("mutate-{kind}: the committed vector declares {status} with {code}, yet it raised {messages:?} — the document came back as {}", encode_en1997_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1997_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1997_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1997_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.input_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1997 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1997_dsl(&text)?;
        let reprinted = encode_en1997_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1997_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1997_pack(&encode_en1997_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1997_snapshot_json(&encode_en1997_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1997_pack(&ctx.input_bytes(super::PACK_ASSET)?)?;
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
