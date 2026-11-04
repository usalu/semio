//! 🦀️ EN 1998 exhaustive mutation case — the Rust SUBJECT adapter over the current 29-kind `En1998Mutation`
//! vocabulary. Every kind applies its committed vector (`../../🧫️fixtures/🧬️mutations/<leaf>/<scenario>`) through the
//! subset's production bridges and asserts, in role, the committed after-snapshot, that the document moved and that
//! its own inverse restores the before-snapshot; `identity-round-trip` re-emits the committed carrier through the DSL,
//! pack and JSON codecs. The reference answer is the shared Python norm engine (`../🐍️.py`).

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{digest, parse_json, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `En1998Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-annex",
    "insert-building",
    "remove-building",
    "change-system-v-rd-n",
    "change-storey-permanent-gk-n",
    "change-storey-stiffness-x",
    "change-storey-drift-xm",
    "change-building-plan-regular",
    "change-elevation-regular",
    "change-member-detailing",
    "change-masonry-wall-ratio",
    "insert-bridge",
    "change-bridge-v-rd-n",
    "insert-assessment",
    "change-assessment-rkn",
    "insert-silo",
    "insert-tank",
    "insert-foundation",
    "insert-retaining-wall",
    "insert-tower",
    "change-tower-m-rd-nm",
    "remove-bridge",
    "remove-assessment",
    "remove-silo",
    "remove-tank",
    "remove-foundation",
    "remove-retaining-wall",
    "remove-tower",
    "change-site-s-ap-r",
    "change-site-a-gr",
    "change-site-de-ground-combo",
    "change-site-en-ground-type",
    "change-site-en-spectrum-type",
    "change-site-importance-class",
    "change-building-name",
    "change-building-plan-width-m",
    "change-building-plan-length-m",
    "change-building-t1-method",
    "change-building-t1-given-s",
    "change-building-ct",
    "change-building-drift-limit-class",
    "change-building-nu",
    "change-building-multi-resisting-systems",
    "change-building-claims-simple-masonry",
    "change-building-accidental-ecc-ratio",
    "change-system-direction",
    "change-system-system-type",
    "change-system-material",
    "change-system-ductility-class",
    "change-system-q0",
    "change-system-alpha-u-over-alpha1",
    "change-system-kw",
    "change-storey-height-m",
    "change-storey-correlated-occupancy",
    "change-storey-stiffness-y",
    "change-storey-centre-of-mass-xm",
    "change-storey-centre-of-mass-ym",
    "change-storey-centre-of-stiffness-xm",
    "change-storey-centre-of-stiffness-ym",
    "change-storey-drift-ym",
    "change-storey-shear-resistance-n",
    "change-member-material",
    "change-member-role",
    "change-member-min-dimension-m",
    "change-member-rho",
    "change-member-rho-prime",
    "change-member-omega-wd",
    "change-member-steel-section-class",
    "change-bridge-period-ratio",
    "change-bridge-fundamental-period-s",
    "change-bridge-bearing-d-rd-m",
    "change-bridge-permanent-gk-n",
    "change-bridge-correlated-occupancy",
    "change-assessment-knowledge-level",
    "change-assessment-limit-state",
    "change-assessment-analysis-method",
    "change-assessment-supported-building-id",
    "change-assessment-gamma-el",
    "change-tower-height-m",
    "change-tower-is-chimney",
    "change-tower-q-nominal",
    "change-tower-permanent-gk-n",
    "change-tower-correlated-occupancy",
    "change-silo-height-m",
    "change-silo-radius-m",
    "change-silo-permanent-gk-n",
    "change-silo-content-qk-n",
    "change-silo-content-category",
    "change-silo-filling-ratio",
    "change-silo-n-rd-n",
    "change-silo-v-rd-n",
    "change-silo-q-nominal",
    "change-tank-height-m",
    "change-tank-radius-m",
    "change-tank-permanent-gk-n",
    "change-tank-content-qk-n",
    "change-tank-content-category",
    "change-tank-filling-ratio",
    "change-tank-v-rd-n",
    "change-foundation-supported-building-id",
    "change-foundation-area-m2",
    "change-foundation-p-rd-pa",
    "change-foundation-passive-resistance-n",
    "change-foundation-k-foundation",
    "change-foundation-k-soil",
    "change-retaining-wall-height-m",
    "change-retaining-wall-phi-deg",
    "change-retaining-wall-soil-gamma",
    "change-retaining-wall-r",
    "change-retaining-wall-h-rd-n-per-m",
    "change-site-vs30-ms",
    "change-site-n-spt",
    "change-site-cu-kpa",
    "change-site-soft-clay-thickness-m",
    "change-site-soft-clay-pi",
    "change-site-liquefiable-layer",
    "change-site-alluvium-thickness-m",
    "change-site-alluvium-vsms",
    "change-site-below-alluvium-vsms",
    "change-building-setback-ratio",
    "change-building-storey-stiffness-ratio",
    "change-building-storey-stiffness-basis",
    "change-building-plan-env-gap-ratio",
    "change-building-joint-gap-m",
    "change-building-ns-weight-n",
    "change-building-ns-height-m",
    "change-building-ns-period-s",
    "change-building-ns-q",
    "change-building-ns-resistance-n",
    "change-building-col-moment-sum-nm",
    "change-building-beam-moment-sum-nm",
    "change-member-fastener-diameter-mm",
    "change-member-timber-thickness-mm",
    "change-member-wall-thickness-m",
    "change-member-effective-stiffness-ratio",
    "change-member-effective-stiffness-basis",
    "change-member-wall-poisson-ratio",
    "change-member-wall-start-xm",
    "change-member-wall-start-ym",
    "change-member-wall-length-m",
    "change-member-wall-direction-rad",
    "change-member-wall-storey-id",
    "change-member-wall-eff-height-m",
    "change-bridge-pier-height-m",
    "change-bridge-pier-moment-rd-nm",
    "change-bridge-pier-shear-rd-n",
    "change-bridge-importance-class",
    "change-bridge-vertical-period-s",
    "change-bridge-pier-cracked-inertia-ratio",
    "change-bridge-deck-length-m",
    "change-bridge-deck-ei-nm2",
    "change-assessment-chord-demand-rad",
    "change-assessment-chord-yield-rad",
    "change-assessment-chord-ult-rad",
    "change-tower-ei-nm2",
    "change-foundation-liq-n160",
    "change-foundation-liq-fines-percent",
    "change-foundation-liq-depth-m",
    "change-foundation-liq-water-depth-m",
    "change-foundation-liq-unit-wt-nm3",
    "change-foundation-liq-magnitude",
    "change-assessment-section-depth-m",
    "change-assessment-section-width-m",
    "change-assessment-eff-depth-m",
    "change-assessment-shear-span-m",
    "change-assessment-axial-force-n",
    "change-assessment-shear-force-demand-n",
    "change-assessment-as1-area-m2",
    "change-assessment-as2-area-m2",
    "change-assessment-bar-diameter-mm",
    "change-assessment-steel-yield-mpa",
    "change-assessment-fc-mpa",
    "change-assessment-strp-leg-area-m2",
    "change-assessment-strp-spacing-m",
    "change-assessment-strp-yield-mpa",
    "change-assessment-core-width-m",
    "change-assessment-core-depth-m",
    "change-assessment-mid-steel-area-m2",
    "change-assessment-rbar-count",
    "change-assessment-rbar-spacing-m",
    "change-assessment-diag-reinf-ratio",
    "change-assessment-shear-crack",
    "change-foundation-liq-layer-m",
    "change-foundation-footing-width-m",
    "change-foundation-bear-axial-n",
    "change-foundation-bear-shear-n",
    "change-foundation-bear-moment-nm",
    "change-foundation-bearing-n-max-n",
    "change-foundation-soil-phi-deg",
    "change-foundation-pile-diameter-m",
    "change-foundation-pile-length-m",
    "change-foundation-pile-modulus-pa",
    "change-foundation-pile-ks-nm3",
    "change-foundation-pile-head-shear-n",
    "change-foundation-pile-head-fixed",
    "change-foundation-pile-iface-m",
    "change-foundation-pile-up-vs-ms",
    "change-foundation-pile-low-vs-ms",
    "change-foundation-pile-moment-rd-nm",
    "change-storey-variable-category",
    "change-storey-variable-qk-n",
    "change-bridge-variable-category",
    "change-bridge-variable-qk-n",
    "change-tower-variable-category",
    "change-tower-variable-qk-n",
    "insert-storey-variable",
    "remove-storey-variable",
    "insert-member",
    "remove-member",
    "insert-storey",
    "remove-storey",
    "insert-system",
    "remove-system",
    "insert-bridge-variable",
    "remove-bridge-variable",
    "insert-tower-variable",
    "remove-tower-variable",
    "change-soil-layer-foundation-id",
    "change-soil-layer-thickness-m",
    "change-soil-unit-wt-nm3",
    "change-soil-layer-vs-ms",
    "change-soil-layer-n160",
    "change-soil-layer-fines-percent",
    "insert-soil-layer",
    "remove-soil-layer",
    "change-building-ot-base-xm",
    "change-building-ot-base-ym",
    "change-building-tiedown-nm",
    "change-diaphragm-building-id",
    "change-diaphragm-storey-id",
    "change-diaphragm-material",
    "change-diaphragm-thickness-m",
    "change-diaphragm-span-m",
    "change-diaphragm-depth-m",
    "change-diaphragm-opening-width-m",
    "change-diaphragm-shear-str-mpa",
    "change-diaphragm-chord-area-m2",
    "change-diaphragm-chord-str-mpa",
    "change-diaphragm-gamma-d",
    "change-accelerogram-building-id",
    "change-accelerogram-label",
    "change-accelerogram-samples",
    "change-accelerogram-dt-s",
    "change-accelerogram-scale-factor",
    "insert-diaphragm",
    "remove-diaphragm",
    "insert-accelerogram",
    "remove-accelerogram",
    "change-tower-base-rot-stiff-nm",
    "change-tank-shell-tm",
    "change-tank-shell-str-mpa",
    "change-pier-bridge-id",
    "change-pier-xm",
    "change-pier-height-m",
    "change-pier-section-bm",
    "change-pier-section-hm",
    "change-pier-modulus-mpa",
    "change-pier-moment-rd-nm",
    "change-pier-shear-rd-n",
    "change-bearing-bridge-id",
    "change-joint-building-id",
    "change-joint-storey-id",
    "change-joint-column-id",
    "change-joint-beam-id",
    "change-bearing-pier-id",
    "change-bearing-device",
    "change-bearing-xm",
    "change-joint-xm",
    "change-joint-ym",
    "change-bearing-stiffness-n-per-m",
    "change-bearing-d-rd-m",
    "change-bearing-seat-length-m",
    "change-joint-column-moment-rd-nm",
    "change-joint-beam-moment-rd-nm",
    "change-bearing-min-support-m",
    "insert-pier",
    "remove-pier",
    "insert-bearing",
    "insert-joint",
    "remove-bearing",
    "remove-joint",
    "change-joint-col-moment-rd-nm",
    "change-retaining-wall-water-height-m",
    "change-retaining-wall-saturated-gamma",
    "change-retaining-wall-permeability-ms",
    "change-foundation-base-friction-deg",
    "change-foundation-base-pore-pressure-pa",
];

/// ⛔️ The refusal witnesses — `<kind>-<slug>` rows whose committed `🎯️outcome` declares the refusal and its code.
/// Registered for `mutate-` only: a refused mutation leaves nothing to undo.
#[cfg(feature = "sut")]
const ROWS: &[&str] = &[
    "insert-building-dupe",
    "insert-bridge-dupe",
    "insert-assessment-dupe",
    "insert-silo-dupe",
    "insert-tank-dupe",
    "insert-foundation-dupe",
    "insert-retaining-wall-dupe",
    "insert-tower-dupe",
    "remove-assessment-middle-row",
    "remove-bridge-middle-row",
    "remove-building-middle-row",
    "remove-foundation-middle-row",
    "remove-retaining-wall-middle-row",
    "remove-silo-middle-row",
    "remove-tank-middle-row",
    "remove-tower-middle-row",
];

/// 🗣️ The real committed EN 1998 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏢️seismic-rc-frame/🎒️.pack.semio";
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
            include_str!("../🧫️fixtures/🧬️mutations/📎️change-annex/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📎️change-annex/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📎️change-annex/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📎️change-annex/✅apply/🔣️.outcome.json"),
        ),
        "insert-building" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-building/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-building/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-building/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-building/✅apply/🔣️.outcome.json"),
        ),
        "insert-building-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-building/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-building/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-building/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-building/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-building" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-building/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-building/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-building/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-building/✅apply/🔣️.outcome.json"),
        ),
        "change-system-v-rd-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-permanent-gk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-stiffness-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-drift-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-building-plan-regular" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/✅apply/🔣️.outcome.json"),
        ),
        "change-elevation-regular" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/✅apply/🔣️.outcome.json"),
        ),
        "change-member-detailing" => (
            include_str!("../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅apply/🔣️.outcome.json"),
        ),
        "change-masonry-wall-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/✅apply/🔣️.outcome.json"),
        ),
        "insert-bridge" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌉insert-bridge/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌉insert-bridge/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌉insert-bridge/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌉insert-bridge/✅apply/🔣️.outcome.json"),
        ),
        "insert-bridge-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌉insert-bridge/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌉insert-bridge/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌉insert-bridge/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌉insert-bridge/⛔dupe/🔣️.outcome.json"),
        ),
        "change-bridge-v-rd-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/✅apply/🔣️.outcome.json"),
        ),
        "insert-assessment" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔧insert-assessment/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔧insert-assessment/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔧insert-assessment/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔧insert-assessment/✅apply/🔣️.outcome.json"),
        ),
        "insert-assessment-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔧insert-assessment/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔧insert-assessment/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔧insert-assessment/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔧insert-assessment/⛔dupe/🔣️.outcome.json"),
        ),
        "change-assessment-rkn" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/✅apply/🔣️.outcome.json"),
        ),
        "insert-silo" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙insert-silo/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙insert-silo/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙insert-silo/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙insert-silo/✅apply/🔣️.outcome.json"),
        ),
        "insert-silo-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫙insert-silo/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙insert-silo/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙insert-silo/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫙insert-silo/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-tank" => (
            include_str!("../🧫️fixtures/🧬️mutations/🛢️insert-tank/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛢️insert-tank/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛢️insert-tank/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛢️insert-tank/✅apply/🔣️.outcome.json"),
        ),
        "insert-tank-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/🛢️insert-tank/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛢️insert-tank/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛢️insert-tank/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛢️insert-tank/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-foundation" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪨insert-foundation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨insert-foundation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨insert-foundation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨insert-foundation/✅apply/🔣️.outcome.json"),
        ),
        "insert-foundation-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪨insert-foundation/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨insert-foundation/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨insert-foundation/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪨insert-foundation/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-retaining-wall" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/✅apply/🔣️.outcome.json"),
        ),
        "insert-retaining-wall-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/⛔dupe/🔣️.outcome.json"),
        ),
        "insert-tower" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗼insert-tower/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗼insert-tower/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗼insert-tower/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗼insert-tower/✅apply/🔣️.outcome.json"),
        ),
        "insert-tower-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗼insert-tower/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗼insert-tower/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗼insert-tower/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗼insert-tower/⛔dupe/🔣️.outcome.json"),
        ),
        "change-tower-m-rd-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/✅apply/🔣️.outcome.json"),
        ),
        "remove-bridge" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bridge/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bridge/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bridge/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bridge/✅apply/🔣️.outcome.json"),
        ),
        "remove-assessment" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-assessment/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-assessment/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-assessment/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-assessment/✅apply/🔣️.outcome.json"),
        ),
        "remove-silo" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-silo/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-silo/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-silo/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-silo/✅apply/🔣️.outcome.json"),
        ),
        "remove-tank" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tank/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tank/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tank/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tank/✅apply/🔣️.outcome.json"),
        ),
        "remove-foundation" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-foundation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-foundation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-foundation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-foundation/✅apply/🔣️.outcome.json"),
        ),
        "remove-retaining-wall" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/✅apply/🔣️.outcome.json"),
        ),
        "remove-tower" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tower/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tower/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tower/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tower/✅apply/🔣️.outcome.json"),
        ),
        "change-site-s-ap-r" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-s-ap-r/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-s-ap-r/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-s-ap-r/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-s-ap-r/✅apply/🔣️.outcome.json"),
        ),
        "change-site-a-gr" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-a-gr/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-a-gr/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-a-gr/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-a-gr/✅apply/🔣️.outcome.json"),
        ),
        "change-site-de-ground-combo" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-de-ground-combo/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-de-ground-combo/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-de-ground-combo/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-de-ground-combo/✅apply/🔣️.outcome.json"),
        ),
        "change-site-en-ground-type" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-en-ground-type/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-en-ground-type/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-en-ground-type/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-en-ground-type/✅apply/🔣️.outcome.json"),
        ),
        "change-site-en-spectrum-type" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-en-spectrum-type/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-en-spectrum-type/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-en-spectrum-type/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-en-spectrum-type/✅apply/🔣️.outcome.json"),
        ),
        "change-site-importance-class" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-importance-class/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-importance-class/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-importance-class/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-importance-class/✅apply/🔣️.outcome.json"),
        ),
        "change-building-name" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-name/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-name/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-name/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-name/✅apply/🔣️.outcome.json"),
        ),
        "change-building-plan-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-building-plan-length-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-length-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-length-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-length-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-length-m/✅apply/🔣️.outcome.json"),
        ),
        "change-building-t1-method" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-t1-method/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-t1-method/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-t1-method/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-t1-method/✅apply/🔣️.outcome.json"),
        ),
        "change-building-t1-given-s" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-t1-given-s/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-t1-given-s/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-t1-given-s/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-t1-given-s/✅apply/🔣️.outcome.json"),
        ),
        "change-building-ct" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ct/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ct/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ct/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ct/✅apply/🔣️.outcome.json"),
        ),
        "change-building-drift-limit-class" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-drift-limit-class/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-drift-limit-class/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-drift-limit-class/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-drift-limit-class/✅apply/🔣️.outcome.json"),
        ),
        "change-building-nu" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-nu/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-nu/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-nu/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-nu/✅apply/🔣️.outcome.json"),
        ),
        "change-building-multi-resisting-systems" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-multi-resisting-systems/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-multi-resisting-systems/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-multi-resisting-systems/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-multi-resisting-systems/✅apply/🔣️.outcome.json"),
        ),
        "change-building-claims-simple-masonry" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-claims-simple-masonry/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-claims-simple-masonry/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-claims-simple-masonry/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-claims-simple-masonry/✅apply/🔣️.outcome.json"),
        ),
        "change-building-accidental-ecc-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-accidental-ecc-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-accidental-ecc-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-accidental-ecc-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-accidental-ecc-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-system-direction" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-direction/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-direction/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-direction/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-direction/✅apply/🔣️.outcome.json"),
        ),
        "change-system-system-type" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-system-type/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-system-type/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-system-type/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-system-type/✅apply/🔣️.outcome.json"),
        ),
        "change-system-material" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-material/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-material/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-material/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-material/✅apply/🔣️.outcome.json"),
        ),
        "change-system-ductility-class" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-ductility-class/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-ductility-class/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-ductility-class/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-ductility-class/✅apply/🔣️.outcome.json"),
        ),
        "change-system-q0" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-q0/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-q0/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-q0/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-q0/✅apply/🔣️.outcome.json"),
        ),
        "change-system-alpha-u-over-alpha1" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-alpha-u-over-alpha1/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-alpha-u-over-alpha1/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-alpha-u-over-alpha1/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-alpha-u-over-alpha1/✅apply/🔣️.outcome.json"),
        ),
        "change-system-kw" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-kw/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-kw/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-kw/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-system-kw/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-correlated-occupancy" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-correlated-occupancy/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-correlated-occupancy/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-correlated-occupancy/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-correlated-occupancy/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-stiffness-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-stiffness-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-stiffness-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-stiffness-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-stiffness-y/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-centre-of-mass-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-mass-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-mass-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-mass-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-mass-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-centre-of-mass-ym" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-mass-ym/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-mass-ym/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-mass-ym/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-mass-ym/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-centre-of-stiffness-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-centre-of-stiffness-ym" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-ym/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-ym/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-ym/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-ym/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-drift-ym" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-drift-ym/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-drift-ym/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-drift-ym/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-drift-ym/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-shear-resistance-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-shear-resistance-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-shear-resistance-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-shear-resistance-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-shear-resistance-n/✅apply/🔣️.outcome.json"),
        ),
        "change-member-material" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-material/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-material/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-material/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-material/✅apply/🔣️.outcome.json"),
        ),
        "change-member-role" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-role/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-role/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-role/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-role/✅apply/🔣️.outcome.json"),
        ),
        "change-member-min-dimension-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-min-dimension-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-min-dimension-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-min-dimension-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-min-dimension-m/✅apply/🔣️.outcome.json"),
        ),
        "change-member-rho" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-rho/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-rho/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-rho/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-rho/✅apply/🔣️.outcome.json"),
        ),
        "change-member-rho-prime" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-rho-prime/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-rho-prime/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-rho-prime/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-rho-prime/✅apply/🔣️.outcome.json"),
        ),
        "change-member-omega-wd" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-omega-wd/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-omega-wd/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-omega-wd/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-omega-wd/✅apply/🔣️.outcome.json"),
        ),
        "change-member-steel-section-class" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-steel-section-class/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-steel-section-class/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-steel-section-class/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-steel-section-class/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-period-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-period-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-period-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-period-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-period-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-fundamental-period-s" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-fundamental-period-s/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-fundamental-period-s/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-fundamental-period-s/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-fundamental-period-s/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-bearing-d-rd-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-bearing-d-rd-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-bearing-d-rd-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-bearing-d-rd-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-bearing-d-rd-m/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-permanent-gk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-permanent-gk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-permanent-gk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-permanent-gk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-permanent-gk-n/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-correlated-occupancy" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-correlated-occupancy/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-correlated-occupancy/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-correlated-occupancy/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-correlated-occupancy/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-knowledge-level" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-knowledge-level/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-knowledge-level/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-knowledge-level/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-knowledge-level/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-limit-state" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-limit-state/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-limit-state/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-limit-state/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-limit-state/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-analysis-method" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-analysis-method/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-analysis-method/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-analysis-method/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-analysis-method/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-supported-building-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-supported-building-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-supported-building-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-supported-building-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-supported-building-id/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-gamma-el" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-gamma-el/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-gamma-el/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-gamma-el/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-gamma-el/✅apply/🔣️.outcome.json"),
        ),
        "change-tower-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-tower-is-chimney" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-is-chimney/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-is-chimney/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-is-chimney/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-is-chimney/✅apply/🔣️.outcome.json"),
        ),
        "change-tower-q-nominal" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-q-nominal/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-q-nominal/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-q-nominal/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-q-nominal/✅apply/🔣️.outcome.json"),
        ),
        "change-tower-permanent-gk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-permanent-gk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-permanent-gk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-permanent-gk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-permanent-gk-n/✅apply/🔣️.outcome.json"),
        ),
        "change-tower-correlated-occupancy" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-correlated-occupancy/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-correlated-occupancy/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-correlated-occupancy/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-correlated-occupancy/✅apply/🔣️.outcome.json"),
        ),
        "change-silo-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-silo-radius-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-radius-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-radius-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-radius-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-radius-m/✅apply/🔣️.outcome.json"),
        ),
        "change-silo-permanent-gk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-permanent-gk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-permanent-gk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-permanent-gk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-permanent-gk-n/✅apply/🔣️.outcome.json"),
        ),
        "change-silo-content-qk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-content-qk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-content-qk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-content-qk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-content-qk-n/✅apply/🔣️.outcome.json"),
        ),
        "change-silo-content-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-content-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-content-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-content-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-content-category/✅apply/🔣️.outcome.json"),
        ),
        "change-silo-filling-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-filling-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-filling-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-filling-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-filling-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-silo-n-rd-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-n-rd-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-n-rd-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-n-rd-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-n-rd-n/✅apply/🔣️.outcome.json"),
        ),
        "change-silo-v-rd-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-v-rd-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-v-rd-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-v-rd-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-v-rd-n/✅apply/🔣️.outcome.json"),
        ),
        "change-silo-q-nominal" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-q-nominal/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-q-nominal/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-q-nominal/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-silo-q-nominal/✅apply/🔣️.outcome.json"),
        ),
        "change-tank-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-tank-radius-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-radius-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-radius-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-radius-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-radius-m/✅apply/🔣️.outcome.json"),
        ),
        "change-tank-permanent-gk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-permanent-gk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-permanent-gk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-permanent-gk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-permanent-gk-n/✅apply/🔣️.outcome.json"),
        ),
        "change-tank-content-qk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-content-qk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-content-qk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-content-qk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-content-qk-n/✅apply/🔣️.outcome.json"),
        ),
        "change-tank-content-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-content-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-content-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-content-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-content-category/✅apply/🔣️.outcome.json"),
        ),
        "change-tank-filling-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-filling-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-filling-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-filling-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-filling-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-tank-v-rd-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-v-rd-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-v-rd-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-v-rd-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-v-rd-n/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-supported-building-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-supported-building-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-supported-building-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-supported-building-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-supported-building-id/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-p-rd-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-p-rd-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-p-rd-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-p-rd-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-p-rd-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-passive-resistance-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-passive-resistance-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-passive-resistance-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-passive-resistance-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-passive-resistance-n/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-k-foundation" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-k-foundation/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-k-foundation/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-k-foundation/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-k-foundation/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-k-soil" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-k-soil/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-k-soil/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-k-soil/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-k-soil/✅apply/🔣️.outcome.json"),
        ),
        "change-retaining-wall-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-retaining-wall-phi-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-phi-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-phi-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-phi-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-phi-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-retaining-wall-soil-gamma" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-soil-gamma/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-soil-gamma/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-soil-gamma/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-soil-gamma/✅apply/🔣️.outcome.json"),
        ),
        "change-retaining-wall-r" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-r/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-r/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-r/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-r/✅apply/🔣️.outcome.json"),
        ),
        "change-retaining-wall-h-rd-n-per-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-h-rd-n-per-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-h-rd-n-per-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-h-rd-n-per-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-h-rd-n-per-m/✅apply/🔣️.outcome.json"),
        ),
        "change-site-vs30-ms" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-vs30-ms/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-vs30-ms/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-vs30-ms/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-vs30-ms/✅apply/🔣️.outcome.json"),
        ),
        "change-site-n-spt" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-n-spt/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-n-spt/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-n-spt/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-n-spt/✅apply/🔣️.outcome.json"),
        ),
        "change-site-cu-kpa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-cu-kpa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-cu-kpa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-cu-kpa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-cu-kpa/✅apply/🔣️.outcome.json"),
        ),
        "change-site-soft-clay-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-soft-clay-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-soft-clay-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-soft-clay-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-soft-clay-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-site-soft-clay-pi" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-soft-clay-pi/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-soft-clay-pi/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-soft-clay-pi/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-soft-clay-pi/✅apply/🔣️.outcome.json"),
        ),
        "change-site-liquefiable-layer" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-liquefiable-layer/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-liquefiable-layer/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-liquefiable-layer/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-liquefiable-layer/✅apply/🔣️.outcome.json"),
        ),
        "change-site-alluvium-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-alluvium-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-alluvium-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-alluvium-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-alluvium-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-site-alluvium-vsms" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-alluvium-vsms/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-alluvium-vsms/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-alluvium-vsms/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-alluvium-vsms/✅apply/🔣️.outcome.json"),
        ),
        "change-site-below-alluvium-vsms" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-below-alluvium-vsms/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-below-alluvium-vsms/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-below-alluvium-vsms/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-site-below-alluvium-vsms/✅apply/🔣️.outcome.json"),
        ),
        "change-building-setback-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-setback-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-setback-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-setback-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-setback-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-building-storey-stiffness-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-storey-stiffness-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-storey-stiffness-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-storey-stiffness-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-storey-stiffness-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-building-storey-stiffness-basis" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-storey-stiffness-basis/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-storey-stiffness-basis/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-storey-stiffness-basis/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-storey-stiffness-basis/✅apply/🔣️.outcome.json"),
        ),
        "change-building-plan-env-gap-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-env-gap-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-env-gap-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-env-gap-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-plan-env-gap-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-building-joint-gap-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-joint-gap-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-joint-gap-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-joint-gap-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-joint-gap-m/✅apply/🔣️.outcome.json"),
        ),
        "change-building-ns-weight-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-weight-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-weight-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-weight-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-weight-n/✅apply/🔣️.outcome.json"),
        ),
        "change-building-ns-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-building-ns-period-s" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-period-s/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-period-s/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-period-s/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-period-s/✅apply/🔣️.outcome.json"),
        ),
        "change-building-ns-q" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-q/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-q/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-q/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-q/✅apply/🔣️.outcome.json"),
        ),
        "change-building-ns-resistance-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-resistance-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-resistance-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-resistance-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ns-resistance-n/✅apply/🔣️.outcome.json"),
        ),
        "change-building-col-moment-sum-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-col-moment-sum-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-col-moment-sum-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-col-moment-sum-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-col-moment-sum-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-building-beam-moment-sum-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-beam-moment-sum-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-beam-moment-sum-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-beam-moment-sum-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-beam-moment-sum-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-member-fastener-diameter-mm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-fastener-diameter-mm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-fastener-diameter-mm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-fastener-diameter-mm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-fastener-diameter-mm/✅apply/🔣️.outcome.json"),
        ),
        "change-member-timber-thickness-mm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-timber-thickness-mm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-timber-thickness-mm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-timber-thickness-mm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-timber-thickness-mm/✅apply/🔣️.outcome.json"),
        ),
        "change-member-wall-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-member-effective-stiffness-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-effective-stiffness-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-effective-stiffness-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-effective-stiffness-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-effective-stiffness-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-member-effective-stiffness-basis" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-effective-stiffness-basis/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-effective-stiffness-basis/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-effective-stiffness-basis/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-effective-stiffness-basis/✅apply/🔣️.outcome.json"),
        ),
        "change-member-wall-poisson-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-poisson-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-poisson-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-poisson-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-poisson-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-member-wall-start-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-start-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-start-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-start-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-start-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-member-wall-start-ym" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-start-ym/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-start-ym/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-start-ym/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-start-ym/✅apply/🔣️.outcome.json"),
        ),
        "change-member-wall-length-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-length-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-length-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-length-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-length-m/✅apply/🔣️.outcome.json"),
        ),
        "change-member-wall-direction-rad" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-direction-rad/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-direction-rad/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-direction-rad/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-direction-rad/✅apply/🔣️.outcome.json"),
        ),
        "change-member-wall-storey-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-storey-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-storey-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-storey-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-storey-id/✅apply/🔣️.outcome.json"),
        ),
        "change-member-wall-eff-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-eff-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-eff-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-eff-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-member-wall-eff-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-pier-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-pier-moment-rd-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-moment-rd-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-moment-rd-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-moment-rd-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-moment-rd-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-pier-shear-rd-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-shear-rd-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-shear-rd-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-shear-rd-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-shear-rd-n/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-importance-class" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-importance-class/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-importance-class/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-importance-class/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-importance-class/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-vertical-period-s" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-vertical-period-s/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-vertical-period-s/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-vertical-period-s/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-vertical-period-s/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-pier-cracked-inertia-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-cracked-inertia-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-cracked-inertia-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-cracked-inertia-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-pier-cracked-inertia-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-deck-length-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-deck-length-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-deck-length-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-deck-length-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-deck-length-m/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-deck-ei-nm2" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-deck-ei-nm2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-deck-ei-nm2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-deck-ei-nm2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-deck-ei-nm2/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-chord-demand-rad" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-demand-rad/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-demand-rad/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-demand-rad/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-demand-rad/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-chord-yield-rad" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-yield-rad/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-yield-rad/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-yield-rad/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-yield-rad/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-chord-ult-rad" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-ult-rad/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-ult-rad/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-ult-rad/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-chord-ult-rad/✅apply/🔣️.outcome.json"),
        ),
        "change-tower-ei-nm2" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-ei-nm2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-ei-nm2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-ei-nm2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-ei-nm2/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-liq-n160" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-n160/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-n160/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-n160/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-n160/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-liq-fines-percent" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-fines-percent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-fines-percent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-fines-percent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-fines-percent/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-liq-depth-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-depth-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-depth-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-depth-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-depth-m/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-liq-water-depth-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-water-depth-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-water-depth-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-water-depth-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-water-depth-m/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-liq-unit-wt-nm3" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-unit-wt-nm3/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-unit-wt-nm3/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-unit-wt-nm3/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-unit-wt-nm3/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-liq-magnitude" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-magnitude/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-magnitude/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-magnitude/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-magnitude/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-section-depth-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-section-depth-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-section-depth-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-section-depth-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-section-depth-m/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-section-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-section-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-section-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-section-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-section-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-eff-depth-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-eff-depth-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-eff-depth-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-eff-depth-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-eff-depth-m/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-shear-span-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-span-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-span-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-span-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-span-m/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-axial-force-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-axial-force-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-axial-force-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-axial-force-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-axial-force-n/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-shear-force-demand-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-force-demand-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-force-demand-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-force-demand-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-force-demand-n/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-as1-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-as1-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-as1-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-as1-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-as1-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-as2-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-as2-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-as2-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-as2-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-as2-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-bar-diameter-mm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-bar-diameter-mm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-bar-diameter-mm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-bar-diameter-mm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-bar-diameter-mm/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-steel-yield-mpa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-steel-yield-mpa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-steel-yield-mpa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-steel-yield-mpa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-steel-yield-mpa/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-fc-mpa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-fc-mpa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-fc-mpa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-fc-mpa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-fc-mpa/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-strp-leg-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-leg-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-leg-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-leg-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-leg-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-strp-spacing-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-spacing-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-spacing-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-spacing-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-spacing-m/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-strp-yield-mpa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-yield-mpa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-yield-mpa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-yield-mpa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-strp-yield-mpa/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-core-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-core-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-core-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-core-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-core-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-core-depth-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-core-depth-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-core-depth-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-core-depth-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-core-depth-m/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-mid-steel-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-mid-steel-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-mid-steel-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-mid-steel-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-mid-steel-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-rbar-count" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-rbar-count/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-rbar-count/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-rbar-count/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-rbar-count/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-rbar-spacing-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-rbar-spacing-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-rbar-spacing-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-rbar-spacing-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-rbar-spacing-m/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-diag-reinf-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-diag-reinf-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-diag-reinf-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-diag-reinf-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-diag-reinf-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-assessment-shear-crack" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-crack/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-crack/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-crack/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-assessment-shear-crack/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-liq-layer-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-layer-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-layer-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-layer-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-liq-layer-m/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-footing-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-footing-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-footing-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-footing-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-footing-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-bear-axial-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-axial-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-axial-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-axial-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-axial-n/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-bear-shear-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-shear-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-shear-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-shear-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-shear-n/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-bear-moment-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-moment-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-moment-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-moment-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bear-moment-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-bearing-n-max-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bearing-n-max-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bearing-n-max-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bearing-n-max-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-bearing-n-max-n/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-soil-phi-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-soil-phi-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-soil-phi-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-soil-phi-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-soil-phi-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-diameter-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-diameter-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-diameter-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-diameter-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-diameter-m/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-length-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-length-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-length-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-length-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-length-m/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-modulus-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-modulus-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-modulus-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-modulus-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-modulus-pa/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-ks-nm3" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-ks-nm3/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-ks-nm3/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-ks-nm3/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-ks-nm3/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-head-shear-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-head-shear-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-head-shear-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-head-shear-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-head-shear-n/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-head-fixed" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-head-fixed/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-head-fixed/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-head-fixed/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-head-fixed/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-iface-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-iface-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-iface-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-iface-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-iface-m/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-up-vs-ms" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-up-vs-ms/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-up-vs-ms/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-up-vs-ms/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-up-vs-ms/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-low-vs-ms" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-low-vs-ms/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-low-vs-ms/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-low-vs-ms/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-low-vs-ms/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-pile-moment-rd-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-moment-rd-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-moment-rd-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-moment-rd-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-pile-moment-rd-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-variable-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-variable-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-variable-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-variable-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-variable-category/✅apply/🔣️.outcome.json"),
        ),
        "change-storey-variable-qk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-variable-qk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-variable-qk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-variable-qk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-storey-variable-qk-n/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-variable-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-variable-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-variable-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-variable-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-variable-category/✅apply/🔣️.outcome.json"),
        ),
        "change-bridge-variable-qk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-variable-qk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-variable-qk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-variable-qk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bridge-variable-qk-n/✅apply/🔣️.outcome.json"),
        ),
        "change-tower-variable-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-variable-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-variable-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-variable-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-variable-category/✅apply/🔣️.outcome.json"),
        ),
        "change-tower-variable-qk-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-variable-qk-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-variable-qk-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-variable-qk-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-variable-qk-n/✅apply/🔣️.outcome.json"),
        ),
        "insert-storey-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-storey-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-storey-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-storey-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-storey-variable/✅apply/🔣️.outcome.json"),
        ),
        "insert-member" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.outcome.json"),
        ),
        "insert-storey" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-storey/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-storey/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-storey/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-storey/✅apply/🔣️.outcome.json"),
        ),
        "insert-system" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-system/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-system/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-system/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-system/✅apply/🔣️.outcome.json"),
        ),
        "remove-storey-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-storey-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-storey-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-storey-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-storey-variable/✅apply/🔣️.outcome.json"),
        ),
        "remove-member" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.outcome.json"),
        ),
        "remove-storey" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-storey/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-storey/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-storey/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-storey/✅apply/🔣️.outcome.json"),
        ),
        "remove-system" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-system/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-system/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-system/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-system/✅apply/🔣️.outcome.json"),
        ),
        "insert-bridge-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bridge-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bridge-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bridge-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bridge-variable/✅apply/🔣️.outcome.json"),
        ),
        "remove-bridge-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bridge-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bridge-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bridge-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bridge-variable/✅apply/🔣️.outcome.json"),
        ),
        "insert-tower-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-tower-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-tower-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-tower-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-tower-variable/✅apply/🔣️.outcome.json"),
        ),
        "remove-tower-variable" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tower-variable/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tower-variable/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tower-variable/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-tower-variable/✅apply/🔣️.outcome.json"),
        ),
        "change-soil-layer-foundation-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-foundation-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-foundation-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-foundation-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-foundation-id/✅apply/🔣️.outcome.json"),
        ),
        "change-soil-layer-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-soil-unit-wt-nm3" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-unit-wt-nm3/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-unit-wt-nm3/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-unit-wt-nm3/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-unit-wt-nm3/✅apply/🔣️.outcome.json"),
        ),
        "change-soil-layer-vs-ms" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-vs-ms/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-vs-ms/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-vs-ms/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-vs-ms/✅apply/🔣️.outcome.json"),
        ),
        "change-soil-layer-n160" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-n160/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-n160/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-n160/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-n160/✅apply/🔣️.outcome.json"),
        ),
        "change-soil-layer-fines-percent" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-fines-percent/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-fines-percent/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-fines-percent/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-soil-layer-fines-percent/✅apply/🔣️.outcome.json"),
        ),
        "insert-soil-layer" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-soil-layer/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-soil-layer/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-soil-layer/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-soil-layer/✅apply/🔣️.outcome.json"),
        ),
        "remove-soil-layer" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-soil-layer/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-soil-layer/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-soil-layer/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-soil-layer/✅apply/🔣️.outcome.json"),
        ),
        "change-building-ot-base-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ot-base-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ot-base-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ot-base-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ot-base-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-building-ot-base-ym" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ot-base-ym/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ot-base-ym/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ot-base-ym/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-ot-base-ym/✅apply/🔣️.outcome.json"),
        ),
        "change-building-tiedown-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-tiedown-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-tiedown-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-tiedown-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-building-tiedown-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-building-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-building-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-building-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-building-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-building-id/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-storey-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-storey-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-storey-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-storey-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-storey-id/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-material" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-material/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-material/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-material/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-material/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-thickness-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-thickness-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-thickness-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-thickness-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-thickness-m/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-span-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-span-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-span-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-span-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-span-m/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-depth-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-depth-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-depth-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-depth-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-depth-m/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-opening-width-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-opening-width-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-opening-width-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-opening-width-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-opening-width-m/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-shear-str-mpa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-shear-str-mpa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-shear-str-mpa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-shear-str-mpa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-shear-str-mpa/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-chord-area-m2" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-chord-area-m2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-chord-area-m2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-chord-area-m2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-chord-area-m2/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-chord-str-mpa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-chord-str-mpa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-chord-str-mpa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-chord-str-mpa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-chord-str-mpa/✅apply/🔣️.outcome.json"),
        ),
        "change-diaphragm-gamma-d" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-gamma-d/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-gamma-d/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-gamma-d/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-diaphragm-gamma-d/✅apply/🔣️.outcome.json"),
        ),
        "change-accelerogram-building-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-building-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-building-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-building-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-building-id/✅apply/🔣️.outcome.json"),
        ),
        "change-accelerogram-label" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-label/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-label/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-label/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-label/✅apply/🔣️.outcome.json"),
        ),
        "change-accelerogram-samples" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-samples/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-samples/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-samples/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-samples/✅apply/🔣️.outcome.json"),
        ),
        "change-accelerogram-dt-s" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-dt-s/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-dt-s/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-dt-s/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-dt-s/✅apply/🔣️.outcome.json"),
        ),
        "change-accelerogram-scale-factor" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-scale-factor/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-scale-factor/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-scale-factor/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-accelerogram-scale-factor/✅apply/🔣️.outcome.json"),
        ),
        "insert-diaphragm" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-diaphragm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-diaphragm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-diaphragm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-diaphragm/✅apply/🔣️.outcome.json"),
        ),
        "remove-diaphragm" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-diaphragm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-diaphragm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-diaphragm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-diaphragm/✅apply/🔣️.outcome.json"),
        ),
        "insert-accelerogram" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-accelerogram/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-accelerogram/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-accelerogram/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-accelerogram/✅apply/🔣️.outcome.json"),
        ),
        "remove-accelerogram" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-accelerogram/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-accelerogram/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-accelerogram/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-accelerogram/✅apply/🔣️.outcome.json"),
        ),
        "change-tower-base-rot-stiff-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-base-rot-stiff-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-base-rot-stiff-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-base-rot-stiff-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tower-base-rot-stiff-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-tank-shell-tm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-shell-tm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-shell-tm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-shell-tm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-shell-tm/✅apply/🔣️.outcome.json"),
        ),
        "change-tank-shell-str-mpa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-shell-str-mpa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-shell-str-mpa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-shell-str-mpa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-tank-shell-str-mpa/✅apply/🔣️.outcome.json"),
        ),
        "change-pier-bridge-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-bridge-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-bridge-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-bridge-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-bridge-id/✅apply/🔣️.outcome.json"),
        ),
        "change-pier-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-pier-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-pier-section-bm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-section-bm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-section-bm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-section-bm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-section-bm/✅apply/🔣️.outcome.json"),
        ),
        "change-pier-section-hm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-section-hm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-section-hm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-section-hm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-section-hm/✅apply/🔣️.outcome.json"),
        ),
        "change-pier-modulus-mpa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-modulus-mpa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-modulus-mpa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-modulus-mpa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-modulus-mpa/✅apply/🔣️.outcome.json"),
        ),
        "change-pier-moment-rd-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-moment-rd-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-moment-rd-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-moment-rd-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-moment-rd-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-pier-shear-rd-n" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-shear-rd-n/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-shear-rd-n/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-shear-rd-n/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-pier-shear-rd-n/✅apply/🔣️.outcome.json"),
        ),
        "change-bearing-bridge-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-bridge-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-bridge-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-bridge-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-bridge-id/✅apply/🔣️.outcome.json"),
        ),
        "change-joint-building-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-building-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-building-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-building-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-building-id/✅apply/🔣️.outcome.json"),
        ),
        "change-joint-storey-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-storey-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-storey-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-storey-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-storey-id/✅apply/🔣️.outcome.json"),
        ),
        "change-joint-column-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-column-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-column-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-column-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-column-id/✅apply/🔣️.outcome.json"),
        ),
        "change-joint-beam-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-beam-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-beam-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-beam-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-beam-id/✅apply/🔣️.outcome.json"),
        ),
        "change-bearing-pier-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-pier-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-pier-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-pier-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-pier-id/✅apply/🔣️.outcome.json"),
        ),
        "change-bearing-device" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-device/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-device/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-device/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-device/✅apply/🔣️.outcome.json"),
        ),
        "change-bearing-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-joint-xm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-xm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-xm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-xm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-xm/✅apply/🔣️.outcome.json"),
        ),
        "change-joint-ym" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-ym/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-ym/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-ym/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-ym/✅apply/🔣️.outcome.json"),
        ),
        "change-bearing-stiffness-n-per-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-stiffness-n-per-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-stiffness-n-per-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-stiffness-n-per-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-stiffness-n-per-m/✅apply/🔣️.outcome.json"),
        ),
        "change-bearing-d-rd-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-d-rd-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-d-rd-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-d-rd-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-d-rd-m/✅apply/🔣️.outcome.json"),
        ),
        "change-bearing-seat-length-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-seat-length-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-seat-length-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-seat-length-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-seat-length-m/✅apply/🔣️.outcome.json"),
        ),
        "change-joint-column-moment-rd-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-column-moment-rd-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-column-moment-rd-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-column-moment-rd-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-column-moment-rd-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-joint-beam-moment-rd-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-beam-moment-rd-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-beam-moment-rd-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-beam-moment-rd-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-beam-moment-rd-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-bearing-min-support-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-min-support-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-min-support-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-min-support-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-bearing-min-support-m/✅apply/🔣️.outcome.json"),
        ),
        "insert-pier" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-pier/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-pier/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-pier/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-pier/✅apply/🔣️.outcome.json"),
        ),
        "remove-pier" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-pier/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-pier/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-pier/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-pier/✅apply/🔣️.outcome.json"),
        ),
        "insert-bearing" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bearing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bearing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bearing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-bearing/✅apply/🔣️.outcome.json"),
        ),
        "insert-joint" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-joint/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-joint/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-joint/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-joint/✅apply/🔣️.outcome.json"),
        ),
        "remove-bearing" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bearing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bearing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bearing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-bearing/✅apply/🔣️.outcome.json"),
        ),
        "remove-joint" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-joint/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-joint/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-joint/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-joint/✅apply/🔣️.outcome.json"),
        ),
        "change-joint-col-moment-rd-nm" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-col-moment-rd-nm/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-col-moment-rd-nm/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-col-moment-rd-nm/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-joint-col-moment-rd-nm/✅apply/🔣️.outcome.json"),
        ),
        "change-retaining-wall-water-height-m" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-water-height-m/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-water-height-m/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-water-height-m/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-water-height-m/✅apply/🔣️.outcome.json"),
        ),
        "change-retaining-wall-saturated-gamma" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-saturated-gamma/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-saturated-gamma/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-saturated-gamma/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-saturated-gamma/✅apply/🔣️.outcome.json"),
        ),
        "change-retaining-wall-permeability-ms" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-permeability-ms/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-permeability-ms/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-permeability-ms/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-retaining-wall-permeability-ms/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-base-friction-deg" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-base-friction-deg/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-base-friction-deg/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-base-friction-deg/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-base-friction-deg/✅apply/🔣️.outcome.json"),
        ),
        "change-foundation-base-pore-pressure-pa" => (
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-base-pore-pressure-pa/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-base-pore-pressure-pa/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-base-pore-pressure-pa/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/✏️change-foundation-base-pore-pressure-pa/✅apply/🔣️.outcome.json"),
        ),
        other => panic!("mutate-en1998-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_en1998_mutations::mutation_bridge::{apply_en1998_mutation, inverse_en1998_mutation};
    use semio_s_artifact_norm_en1998_mutations::En1998Mutation;

    use semio_s_artifact_norm_en1998_io::text::mutations::decode_en1998_mutation_json;
    use semio_s_artifact_norm_en1998_model::En1998Snapshot;
    use semio_s_artifact_norm_en1998_io::binary::snapshot::decode_en1998_pack;
    use semio_s_artifact_norm_en1998_io::binary::snapshot::encode_en1998_pack;
    use semio_s_artifact_norm_en1998_io::text::snapshot::encode_en1998_dsl;
    use semio_s_artifact_norm_en1998_io::text::snapshot::decode_en1998_dsl;
    use semio_s_artifact_norm_en1998_io::text::snapshot::decode_en1998_snapshot_json;
    use semio_s_artifact_norm_en1998_io::text::snapshot::encode_en1998_snapshot_json;
    use semio_repo_test_host::law;

    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1998Snapshot, String> {
        decode_en1998_snapshot_json(text).map_err(|error| format!("mutate-en1998-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1998Mutation, String> {
        decode_en1998_mutation_json(text).map_err(|error| format!("mutate-en1998-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1998Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1998_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1998Snapshot, expected: &En1998Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1998_snapshot_json(got), encode_en1998_snapshot_json(expected))
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
            let applied = apply_en1998_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected" | "no-op", Ok((snapshot, messages))) if !messages.iter().any(|message| message.ends_with(&format!(":{code}"))) => return Err(format!("mutate-{kind}: the committed vector declares {status} with {code}, yet it raised {messages:?} — the document came back as {}", encode_en1998_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1998_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1998_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1998_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.input_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1998 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1998_dsl(&text)?;
        let reprinted = encode_en1998_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1998_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1998_pack(&encode_en1998_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1998_snapshot_json(&encode_en1998_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1998_pack(&ctx.input_bytes(super::PACK_ASSET)?)?;
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
