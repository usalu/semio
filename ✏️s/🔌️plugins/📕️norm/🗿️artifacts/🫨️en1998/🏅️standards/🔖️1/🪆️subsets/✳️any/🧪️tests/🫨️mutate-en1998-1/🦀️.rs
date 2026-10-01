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
    "update-site",
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
];

/// ⛔️ The refusal witnesses — `<kind>-<slug>` rows whose committed `🎯️outcome` declares the refusal and its code.
/// Registered for `mutate-` only: a refused mutation leaves nothing to undo.
#[cfg(feature = "sut")]
const ROWS: &[&str] = &[
    "update-site-noop",
    "insert-building-dupe",
    "insert-bridge-dupe",
    "insert-assessment-dupe",
    "insert-silo-dupe",
    "insert-tank-dupe",
    "insert-foundation-dupe",
    "insert-retaining-wall-dupe",
    "insert-tower-dupe",
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
            include_str!("../../🧫️fixtures/🧬️mutations/📎️change-annex/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📎️change-annex/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📎️change-annex/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📎️change-annex/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-site" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-site-noop" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/🟰noop/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/🟰noop/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/🟰noop/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/🟰noop/🎯️outcome/🔣️.json"),
        ),
        "insert-building" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-building-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-building" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-building/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-building/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-building/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-building/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-system-v-rd-n" => (
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-storey-permanent-gk-n" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-storey-stiffness-x" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-storey-drift-xm" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-building-plan-regular" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-elevation-regular" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-member-detailing" => (
            include_str!("../../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅apply/🎯️outcome/🔣️.json"),
        ),
        "change-masonry-wall-ratio" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-bridge" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-bridge-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "change-bridge-v-rd-n" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-assessment" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-assessment-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "change-assessment-rkn" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-silo" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-silo-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "insert-tank" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-tank-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "insert-foundation" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-foundation-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "insert-retaining-wall" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-retaining-wall-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "insert-tower" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-tower-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "change-tower-m-rd-nm" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/✅apply/🎯️outcome/🔣️.json"),
        ),
        "remove-bridge" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge/✅apply/🎯️outcome/🔣️.json"),
        ),
        "remove-assessment" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-assessment/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-assessment/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-assessment/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-assessment/✅apply/🎯️outcome/🔣️.json"),
        ),
        "remove-silo" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo/✅apply/🎯️outcome/🔣️.json"),
        ),
        "remove-tank" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tank/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tank/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tank/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tank/✅apply/🎯️outcome/🔣️.json"),
        ),
        "remove-foundation" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-foundation/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-foundation/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-foundation/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-foundation/✅apply/🎯️outcome/🔣️.json"),
        ),
        "remove-retaining-wall" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/✅apply/🎯️outcome/🔣️.json"),
        ),
        "remove-tower" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower/✅apply/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-en1998-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_en1998::standards::v1::subsets::any::schema::mutations::{apply_en1998_mutation, decode_en1998_mutation_json, inverse_en1998_mutation, En1998Mutation};
    use semio_s_artifact_norm_en1998::standards::v1::subsets::any::schema::snapshot::{decode_en1998_dsl, decode_en1998_pack, decode_en1998_snapshot_json, encode_en1998_dsl, encode_en1998_pack, encode_en1998_snapshot_json, En1998Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
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
            let steps = inverse_en1998_mutation(&mutation, &base);
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1998 artifact is not UTF-8: {error}"))?;
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
        let twin = decode_en1998_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
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
