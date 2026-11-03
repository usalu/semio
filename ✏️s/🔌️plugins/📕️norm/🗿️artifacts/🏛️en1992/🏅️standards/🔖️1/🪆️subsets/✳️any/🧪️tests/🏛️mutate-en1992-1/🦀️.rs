//! 🦀️ EN 1992 exhaustive mutation case — the Rust SUBJECT adapter over the current 149-kind `En1992Mutation`
//! vocabulary. Every kind applies its committed vector (`../../🧫️fixtures/🧬️mutations/<leaf>/<scenario>`) through the
//! subset's production bridges and asserts, in role, the committed after-snapshot, that the document moved and that
//! its own inverse restores the before-snapshot; `identity-round-trip` re-emits the committed carrier through the DSL,
//! pack and JSON codecs. The reference answer is the shared Python norm engine (`../🐍️.py`).

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{digest, parse_json, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `En1992Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-annex",
    "change-reliability-class",
    "change-title",
    "change-design-working-life",
    "change-delta-c-dev",
    "change-cement-type",
    "change-concrete-f-ck",
    "change-reinforcement-f-yk",
    "insert-member",
    "remove-member",
    "change-member-order",
    "change-member-width",
    "change-member-height",
    "change-member-effective-depth",
    "change-member-cover",
    "change-member-exposure",
    "change-member-span",
    "change-member-stirrup-spacing",
    "change-member-axis-distance",
    "change-member-fire-rating",
    "change-bar-layer-count",
    "change-bar-layer-diameter",
    "change-action-mk",
    "change-action-nk",
    "change-action-vk",
    "insert-anchor",
    "remove-anchor",
    "change-anchor-h-ef",
    "change-anchor-as",
    "change-member-bridge-delta-sigma-s",
    "change-member-bridge-sigma-c",
    "change-member-liquid-sr-max",
    "change-member-liquid-f-ct-eff",
    "change-member-liquid-rho-p-eff",
    "change-member-liquid-sigma-s",
    "change-member-hd-over-h",
    "change-member-udl",
    "change-member-buckling-length",
    "change-member-relative-humidity",
    "change-member-bearing-width",
    "change-concrete-name",
    "change-reinforcement-name",
    "change-reinforcement-es",
    "change-reinforcement-ductility",
    "change-reinforcement-k",
    "change-reinforcement-eps-uk",
    "change-prestress-steel-name",
    "change-prestress-steel-f-pk",
    "change-prestress-steel-fp01k",
    "change-member-label-en",
    "change-member-label-de",
    "change-member-kind",
    "change-member-concrete-grade-id",
    "change-member-rebar-grade-id",
    "change-member-prestress-steel-id",
    "change-member-support",
    "change-member-stirrups-diameter",
    "change-member-stirrups-legs",
    "change-member-punch-column-width",
    "change-member-punch-column-depth",
    "change-member-punch-col-position",
    "change-member-punching-asw",
    "change-member-prestress-force",
    "change-member-prestress-area",
    "change-member-prestress-ecc",
    "change-member-prestress-loss-ratio",
    "change-member-fire-slab-system",
    "change-member-use-fem",
    "change-member-deflection-sensitive",
    "change-bar-layer-position",
    "change-bar-layer-anchorage-length",
    "change-bar-layer-lap-length",
    "change-bar-layer-bond-condition",
    "change-bar-layer-aggregate-size",
    "change-bar-layer-lap-share",
    "change-action-kind",
    "change-action-category",
    "change-action-source",
    "change-action-gk-line",
    "change-action-qk-line",
    "change-action-point-force",
    "change-action-tk",
    "change-action-vk-punch",
    "change-section-point-y",
    "change-section-point-z",
    "change-bar-point-layer-id",
    "change-bar-point-y",
    "change-bar-point-z",
    "change-bar-point-diameter",
    "change-tendon-point-x",
    "change-tendon-point-y",
    "change-tendon-point-z",
    "change-anchor-cracked",
    "change-anchor-f-uk",
    "change-anchor-f-yk",
    "change-anchor-d",
    "change-anchor-c1",
    "change-anchor-f-ck",
    "change-anchor-head-diameter",
    "change-anchor-action-kind",
    "change-anchor-action-category",
    "change-anchor-action-source",
    "change-anchor-action-gk-line",
    "change-anchor-action-qk-line",
    "change-anchor-action-point-force",
    "change-anchor-action-mk",
    "change-anchor-action-nk",
    "change-anchor-action-vk",
    "change-anchor-action-tk",
    "change-anchor-action-vk-punch",
    "insert-concrete-grade",
    "remove-concrete-grade",
    "insert-reinforcement-grade",
    "remove-reinforcement-grade",
    "insert-prestress-steel",
    "remove-prestress-steel",
    "insert-bar-layer",
    "remove-bar-layer",
    "insert-action",
    "remove-action",
    "insert-section-point",
    "remove-section-point",
    "insert-bar-point",
    "remove-bar-point",
    "insert-tendon-point",
    "remove-tendon-point",
    "insert-anchor-action",
    "remove-anchor-action",
    "insert-case",
    "remove-case",
    "change-case-number",
    "change-case-text",
    "change-case-label-en",
    "change-case-label-de",
    "change-case-entry-id",
    "change-member-tightness",
    "clear-member-tightness",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-placement",
    "remove-placement",
    "change-bar-point-length",
    "change-bar-point-bend",
    "change-member-punching-beta-method",
    "change-member-punching-moment-c1",
    "change-member-punching-moment-c2",
    "change-opening-x",
    "change-opening-y",
    "change-opening-width",
    "change-opening-depth",
    "insert-opening",
    "remove-opening",
    "change-stud-row-angle",
    "change-stud-row-count",
    "change-stud-row-spacing-first",
    "change-stud-row-spacing-radial",
    "change-stud-row-diameter",
    "insert-stud-row",
    "remove-stud-row",
];

/// ⛔️ The refusal witnesses — `<kind>-<slug>` rows whose committed `🎯️outcome` declares the refusal and its code.
/// Registered for `mutate-` only: a refused mutation leaves nothing to undo.
#[cfg(feature = "sut")]
const ROWS: &[&str] = &[
    "insert-anchor-dupe",
    "remove-anchor-gone",
    "remove-member-middle",
    "change-member-order-middle",
    "remove-anchor-middle",
];

/// 🗣️ The real committed EN 1992 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🛢️liquid-retaining-fem-anchor/🎒️.pack.semio";
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
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/🧭️middle/🔣️.outcome.json"),
        ),
        "change-member-order-middle" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔀️change-member-order/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔀️change-member-order/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔀️change-member-order/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔀️change-member-order/🧭️middle/🔣️.outcome.json"),
        ),
        "remove-anchor-middle" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/🧭️middle/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/🧭️middle/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/🧭️middle/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/🧭️middle/🔣️.outcome.json"),
        ),
        "change-annex" => (
        "change-reliability-class" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔣️.outcome.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.outcome.json"),
        ),
        "change-title" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-title/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-title/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-title/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏷️change-title/✅apply/🔣️.outcome.json"),
        ),
        "change-design-working-life" => (
            include_str!("../🧫️fixtures/🧬️mutations/📅️change-design-working-life/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📅️change-design-working-life/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📅️change-design-working-life/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📅️change-design-working-life/✅apply/🔣️.outcome.json"),
        ),
        "change-delta-c-dev" => (
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-delta-c-dev/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-delta-c-dev/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-delta-c-dev/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📏️change-delta-c-dev/✅apply/🔣️.outcome.json"),
        ),
        "change-cement-type" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧪change-cement-type/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪change-cement-type/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪change-cement-type/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧪change-cement-type/✅apply/🔣️.outcome.json"),
        ),
        "change-concrete-f-ck" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-concrete-f-ck/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-concrete-f-ck/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-concrete-f-ck/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱change-concrete-f-ck/✅apply/🔣️.outcome.json"),
        ),
        "change-reinforcement-f-yk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩change-reinforcement-f-yk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩change-reinforcement-f-yk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩change-reinforcement-f-yk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩change-reinforcement-f-yk/✅apply/🔣️.outcome.json"),
        ),
        "insert-member" => (
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➕️insert-member/✅apply/🔣️.outcome.json"),
        ),
        "remove-member" => (
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/➖️remove-member/✅apply/🔣️.outcome.json"),
        ),
        "change-member-order" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔀️change-member-order/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔀️change-member-order/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔀️change-member-order/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔀️change-member-order/✅apply/🔣️.outcome.json"),
        ),
        "change-member-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↔️change-member-width/✅apply/🔣️.outcome.json"),
        ),
        "change-member-height" => (
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-height/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-height/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-height/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↕️change-member-height/✅apply/🔣️.outcome.json"),
        ),
        "change-member-effective-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-member-effective-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-member-effective-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-member-effective-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-member-effective-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-member-cover" => (
            include_str!("../🧫️fixtures/🧬️mutations/🛡️change-member-cover/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛡️change-member-cover/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛡️change-member-cover/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🛡️change-member-cover/✅apply/🔣️.outcome.json"),
        ),
        "change-member-exposure" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌦️change-member-exposure/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌦️change-member-exposure/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌦️change-member-exposure/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌦️change-member-exposure/✅apply/🔣️.outcome.json"),
        ),
        "change-member-span" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-span/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-span/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-span/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌉️change-member-span/✅apply/🔣️.outcome.json"),
        ),
        "change-member-stirrup-spacing" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪢change-member-stirrup-spacing/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪢change-member-stirrup-spacing/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪢change-member-stirrup-spacing/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪢change-member-stirrup-spacing/✅apply/🔣️.outcome.json"),
        ),
        "change-member-axis-distance" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔥change-member-axis-distance/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥change-member-axis-distance/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥change-member-axis-distance/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥change-member-axis-distance/✅apply/🔣️.outcome.json"),
        ),
        "change-member-fire-rating" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-member-fire-rating/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-member-fire-rating/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-member-fire-rating/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔥️change-member-fire-rating/✅apply/🔣️.outcome.json"),
        ),
        "change-member-tightness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🚿change-member-tightness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🚿change-member-tightness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🚿change-member-tightness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🚿change-member-tightness/✅apply/🔣️.outcome.json"),
        ),
        "clear-member-tightness" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧽clear-member-tightness/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧽clear-member-tightness/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧽clear-member-tightness/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧽clear-member-tightness/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-layer-count" => (
            include_str!("../🧫️fixtures/🧬️mutations/#️⃣change-bar-layer-count/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/#️⃣change-bar-layer-count/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/#️⃣change-bar-layer-count/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/#️⃣change-bar-layer-count/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-layer-diameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕change-bar-layer-diameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕change-bar-layer-diameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕change-bar-layer-diameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕change-bar-layer-diameter/✅apply/🔣️.outcome.json"),
        ),
        "change-action-mk" => (
            include_str!("../🧫️fixtures/🧬️mutations/⤴️change-action-mk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⤴️change-action-mk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⤴️change-action-mk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⤴️change-action-mk/✅apply/🔣️.outcome.json"),
        ),
        "change-action-nk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-nk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-nk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-nk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-nk/✅apply/🔣️.outcome.json"),
        ),
        "change-action-vk" => (
            include_str!("../🧫️fixtures/🧬️mutations/↘️change-action-vk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↘️change-action-vk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↘️change-action-vk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/↘️change-action-vk/✅apply/🔣️.outcome.json"),
        ),
        "insert-anchor" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️insert-anchor/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️insert-anchor/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️insert-anchor/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️insert-anchor/✅apply/🔣️.outcome.json"),
        ),
        "insert-anchor-dupe" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️insert-anchor/⛔dupe/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️insert-anchor/⛔dupe/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️insert-anchor/⛔dupe/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️insert-anchor/⛔dupe/🔣️.outcome.json"),
        ),
        "remove-anchor" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/✅apply/🔣️.outcome.json"),
        ),
        "remove-anchor-gone" => (
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/❓gone/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/❓gone/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/❓gone/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-anchor/❓gone/🔣️.outcome.json"),
        ),
        "change-anchor-h-ef" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍change-anchor-h-ef/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍change-anchor-h-ef/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍change-anchor-h-ef/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍change-anchor-h-ef/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-as" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧷change-anchor-as/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧷change-anchor-as/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧷change-anchor-as/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧷change-anchor-as/✅apply/🔣️.outcome.json"),
        ),
        "change-member-bridge-delta-sigma-s" => (
            include_str!("../🧫️fixtures/🧬️mutations/🎢change-member-bridge-delta-sigma-s/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎢change-member-bridge-delta-sigma-s/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎢change-member-bridge-delta-sigma-s/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🎢change-member-bridge-delta-sigma-s/✅apply/🔣️.outcome.json"),
        ),
        "change-member-bridge-sigma-c" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌁change-member-bridge-sigma-c/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌁change-member-bridge-sigma-c/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌁change-member-bridge-sigma-c/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌁change-member-bridge-sigma-c/✅apply/🔣️.outcome.json"),
        ),
        "change-member-liquid-sr-max" => (
            include_str!("../🧫️fixtures/🧬️mutations/🌊change-member-liquid-sr-max/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌊change-member-liquid-sr-max/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌊change-member-liquid-sr-max/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🌊change-member-liquid-sr-max/✅apply/🔣️.outcome.json"),
        ),
        "change-member-liquid-f-ct-eff" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧊change-member-liquid-f-ct-eff/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧊change-member-liquid-f-ct-eff/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧊change-member-liquid-f-ct-eff/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧊change-member-liquid-f-ct-eff/✅apply/🔣️.outcome.json"),
        ),
        "change-member-liquid-rho-p-eff" => (
            include_str!("../🧫️fixtures/🧬️mutations/🫧change-member-liquid-rho-p-eff/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫧change-member-liquid-rho-p-eff/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫧change-member-liquid-rho-p-eff/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🫧change-member-liquid-rho-p-eff/✅apply/🔣️.outcome.json"),
        ),
        "change-member-liquid-sigma-s" => (
            include_str!("../🧫️fixtures/🧬️mutations/💦change-member-liquid-sigma-s/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💦change-member-liquid-sigma-s/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💦change-member-liquid-sigma-s/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💦change-member-liquid-sigma-s/✅apply/🔣️.outcome.json"),
        ),
        "change-member-hd-over-h" => (
            include_str!("../🧫️fixtures/🧬️mutations/🚰change-member-hd-over-h/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🚰change-member-hd-over-h/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🚰change-member-hd-over-h/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🚰change-member-hd-over-h/✅apply/🔣️.outcome.json"),
        ),
        "change-member-udl" => (
            include_str!("../🧫️fixtures/🧬️mutations/🪜change-member-udl/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪜change-member-udl/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪜change-member-udl/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🪜change-member-udl/✅apply/🔣️.outcome.json"),
        ),
        "change-member-buckling-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵change-member-buckling-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵change-member-buckling-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵change-member-buckling-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵change-member-buckling-length/✅apply/🔣️.outcome.json"),
        ),
        "change-member-relative-humidity" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧change-member-relative-humidity/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧change-member-relative-humidity/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧change-member-relative-humidity/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧change-member-relative-humidity/✅apply/🔣️.outcome.json"),
        ),
        "change-member-bearing-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/💧change-member-bearing-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧change-member-bearing-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧change-member-bearing-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/💧change-member-bearing-width/✅apply/🔣️.outcome.json"),
        ),
        "change-concrete-name" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-concrete-name/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-concrete-name/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-concrete-name/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️change-concrete-name/✅apply/🔣️.outcome.json"),
        ),
        "change-reinforcement-name" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-name/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-name/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-name/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-name/✅apply/🔣️.outcome.json"),
        ),
        "change-reinforcement-es" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-es/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-es/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-es/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-es/✅apply/🔣️.outcome.json"),
        ),
        "change-reinforcement-ductility" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-ductility/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-ductility/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-ductility/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-ductility/✅apply/🔣️.outcome.json"),
        ),
        "change-reinforcement-k" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-k/✅apply/🔣️.outcome.json"),
        ),
        "change-reinforcement-eps-uk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-eps-uk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-eps-uk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-eps-uk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-reinforcement-eps-uk/✅apply/🔣️.outcome.json"),
        ),
        "change-prestress-steel-name" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-name/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-name/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-name/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-name/✅apply/🔣️.outcome.json"),
        ),
        "change-prestress-steel-f-pk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-f-pk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-f-pk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-f-pk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-f-pk/✅apply/🔣️.outcome.json"),
        ),
        "change-prestress-steel-fp01k" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-fp01k/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-fp01k/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-fp01k/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️change-prestress-steel-fp01k/✅apply/🔣️.outcome.json"),
        ),
        "change-member-label-en" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-label-en/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-label-en/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-label-en/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-label-en/✅apply/🔣️.outcome.json"),
        ),
        "change-member-label-de" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-label-de/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-label-de/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-label-de/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-label-de/✅apply/🔣️.outcome.json"),
        ),
        "change-member-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-member-concrete-grade-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-concrete-grade-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-concrete-grade-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-concrete-grade-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-concrete-grade-id/✅apply/🔣️.outcome.json"),
        ),
        "change-member-rebar-grade-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-rebar-grade-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-rebar-grade-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-rebar-grade-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-rebar-grade-id/✅apply/🔣️.outcome.json"),
        ),
        "change-member-prestress-steel-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-steel-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-steel-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-steel-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-steel-id/✅apply/🔣️.outcome.json"),
        ),
        "change-member-support" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-support/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-support/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-support/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-support/✅apply/🔣️.outcome.json"),
        ),
        "change-member-stirrups-diameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-stirrups-diameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-stirrups-diameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-stirrups-diameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-stirrups-diameter/✅apply/🔣️.outcome.json"),
        ),
        "change-member-stirrups-legs" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-stirrups-legs/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-stirrups-legs/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-stirrups-legs/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-stirrups-legs/✅apply/🔣️.outcome.json"),
        ),
        "change-member-punch-column-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-column-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-column-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-column-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-column-width/✅apply/🔣️.outcome.json"),
        ),
        "change-member-punch-column-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-column-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-column-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-column-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-column-depth/✅apply/🔣️.outcome.json"),
        ),
        "change-member-punch-col-position" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-col-position/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-col-position/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-col-position/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punch-col-position/✅apply/🔣️.outcome.json"),
        ),
        "change-member-punching-asw" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-asw/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-asw/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-asw/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-asw/✅apply/🔣️.outcome.json"),
        ),
        "change-member-prestress-force" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-force/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-force/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-force/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-force/✅apply/🔣️.outcome.json"),
        ),
        "change-member-prestress-area" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-area/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-area/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-area/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-area/✅apply/🔣️.outcome.json"),
        ),
        "change-member-prestress-ecc" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-ecc/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-ecc/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-ecc/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-ecc/✅apply/🔣️.outcome.json"),
        ),
        "change-member-prestress-loss-ratio" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-loss-ratio/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-loss-ratio/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-loss-ratio/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-prestress-loss-ratio/✅apply/🔣️.outcome.json"),
        ),
        "change-member-fire-slab-system" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-fire-slab-system/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-fire-slab-system/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-fire-slab-system/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-fire-slab-system/✅apply/🔣️.outcome.json"),
        ),
        "change-member-use-fem" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-use-fem/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-use-fem/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-use-fem/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-use-fem/✅apply/🔣️.outcome.json"),
        ),
        "change-member-deflection-sensitive" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-deflection-sensitive/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-deflection-sensitive/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-deflection-sensitive/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-deflection-sensitive/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-layer-position" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-position/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-position/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-position/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-position/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-layer-anchorage-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-anchorage-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-anchorage-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-anchorage-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-anchorage-length/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-layer-lap-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-lap-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-lap-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-lap-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-lap-length/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-layer-bond-condition" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-bond-condition/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-bond-condition/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-bond-condition/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-bond-condition/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-layer-aggregate-size" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-aggregate-size/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-aggregate-size/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-aggregate-size/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-aggregate-size/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-layer-lap-share" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-lap-share/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-lap-share/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-lap-share/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️change-bar-layer-lap-share/✅apply/🔣️.outcome.json"),
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
        "change-action-point-force" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-point-force/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-point-force/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-point-force/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-point-force/✅apply/🔣️.outcome.json"),
        ),
        "change-action-tk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-tk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-tk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-tk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-tk/✅apply/🔣️.outcome.json"),
        ),
        "change-action-vk-punch" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vk-punch/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vk-punch/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vk-punch/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-action-vk-punch/✅apply/🔣️.outcome.json"),
        ),
        "change-section-point-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-point-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-point-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-point-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-point-y/✅apply/🔣️.outcome.json"),
        ),
        "change-section-point-z" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-point-z/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-point-z/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-point-z/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️change-section-point-z/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-point-layer-id" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-layer-id/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-layer-id/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-layer-id/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-layer-id/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-point-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-y/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-point-z" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-z/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-z/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-z/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-z/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-point-diameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-diameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-diameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-diameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-diameter/✅apply/🔣️.outcome.json"),
        ),
        "change-tendon-point-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-x/✅apply/🔣️.outcome.json"),
        ),
        "change-tendon-point-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-y/✅apply/🔣️.outcome.json"),
        ),
        "change-tendon-point-z" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-z/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-z/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-z/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️change-tendon-point-z/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-cracked" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-cracked/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-cracked/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-cracked/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-cracked/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-f-uk" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-uk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-uk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-uk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-uk/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-f-yk" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-yk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-yk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-yk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-yk/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-d" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-d/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-d/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-d/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-d/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-c1" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-c1/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-c1/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-c1/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-c1/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-f-ck" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-ck/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-ck/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-ck/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-f-ck/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-head-diameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-head-diameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-head-diameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-head-diameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⚓️change-anchor-head-diameter/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-kind" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-kind/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-kind/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-kind/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-kind/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-category" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-category/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-category/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-category/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-category/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-source" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-source/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-source/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-source/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-source/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-gk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-gk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-gk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-gk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-gk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-qk-line" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-qk-line/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-qk-line/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-qk-line/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-qk-line/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-point-force" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-point-force/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-point-force/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-point-force/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-point-force/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-mk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-mk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-mk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-mk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-mk/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-nk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-nk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-nk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-nk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-nk/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-vk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-vk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-vk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-vk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-vk/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-tk" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-tk/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-tk/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-tk/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-tk/✅apply/🔣️.outcome.json"),
        ),
        "change-anchor-action-vk-punch" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-vk-punch/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-vk-punch/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-vk-punch/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️change-anchor-action-vk-punch/✅apply/🔣️.outcome.json"),
        ),
        "insert-concrete-grade" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-concrete-grade/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-concrete-grade/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-concrete-grade/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️insert-concrete-grade/✅apply/🔣️.outcome.json"),
        ),
        "remove-concrete-grade" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧱️remove-concrete-grade/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️remove-concrete-grade/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️remove-concrete-grade/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧱️remove-concrete-grade/✅apply/🔣️.outcome.json"),
        ),
        "insert-reinforcement-grade" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️insert-reinforcement-grade/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️insert-reinforcement-grade/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️insert-reinforcement-grade/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️insert-reinforcement-grade/✅apply/🔣️.outcome.json"),
        ),
        "remove-reinforcement-grade" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️remove-reinforcement-grade/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️remove-reinforcement-grade/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️remove-reinforcement-grade/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️remove-reinforcement-grade/✅apply/🔣️.outcome.json"),
        ),
        "insert-prestress-steel" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️insert-prestress-steel/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️insert-prestress-steel/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️insert-prestress-steel/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️insert-prestress-steel/✅apply/🔣️.outcome.json"),
        ),
        "remove-prestress-steel" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧵️remove-prestress-steel/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️remove-prestress-steel/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️remove-prestress-steel/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧵️remove-prestress-steel/✅apply/🔣️.outcome.json"),
        ),
        "insert-bar-layer" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️insert-bar-layer/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️insert-bar-layer/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️insert-bar-layer/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️insert-bar-layer/✅apply/🔣️.outcome.json"),
        ),
        "remove-bar-layer" => (
            include_str!("../🧫️fixtures/🧬️mutations/⭕️remove-bar-layer/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️remove-bar-layer/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️remove-bar-layer/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/⭕️remove-bar-layer/✅apply/🔣️.outcome.json"),
        ),
        "insert-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-action/✅apply/🔣️.outcome.json"),
        ),
        "remove-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-action/✅apply/🔣️.outcome.json"),
        ),
        "insert-section-point" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️insert-section-point/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️insert-section-point/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️insert-section-point/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️insert-section-point/✅apply/🔣️.outcome.json"),
        ),
        "remove-section-point" => (
            include_str!("../🧫️fixtures/🧬️mutations/📐️remove-section-point/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️remove-section-point/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️remove-section-point/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📐️remove-section-point/✅apply/🔣️.outcome.json"),
        ),
        "insert-bar-point" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-bar-point/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-bar-point/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-bar-point/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️insert-bar-point/✅apply/🔣️.outcome.json"),
        ),
        "remove-bar-point" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-bar-point/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-bar-point/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-bar-point/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️remove-bar-point/✅apply/🔣️.outcome.json"),
        ),
        "insert-tendon-point" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧬️insert-tendon-point/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️insert-tendon-point/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️insert-tendon-point/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️insert-tendon-point/✅apply/🔣️.outcome.json"),
        ),
        "remove-tendon-point" => (
            include_str!("../🧫️fixtures/🧬️mutations/🧬️remove-tendon-point/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️remove-tendon-point/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️remove-tendon-point/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🧬️remove-tendon-point/✅apply/🔣️.outcome.json"),
        ),
        "insert-anchor-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-anchor-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-anchor-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-anchor-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️insert-anchor-action/✅apply/🔣️.outcome.json"),
        ),
        "remove-anchor-action" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-anchor-action/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-anchor-action/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-anchor-action/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏋️remove-anchor-action/✅apply/🔣️.outcome.json"),
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
        "change-bar-point-length" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-length/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-length/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-length/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-length/✅apply/🔣️.outcome.json"),
        ),
        "change-bar-point-bend" => (
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-bend/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-bend/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-bend/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/📍️change-bar-point-bend/✅apply/🔣️.outcome.json"),
        ),
        "change-member-punching-beta-method" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-beta-method/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-beta-method/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-beta-method/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-beta-method/✅apply/🔣️.outcome.json"),
        ),
        "change-member-punching-moment-c1" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-moment-c1/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-moment-c1/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-moment-c1/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-moment-c1/✅apply/🔣️.outcome.json"),
        ),
        "change-member-punching-moment-c2" => (
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-moment-c2/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-moment-c2/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-moment-c2/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🏛️change-member-punching-moment-c2/✅apply/🔣️.outcome.json"),
        ),
        "change-opening-x" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-x/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-x/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-x/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-x/✅apply/🔣️.outcome.json"),
        ),
        "change-opening-y" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-y/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-y/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-y/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-y/✅apply/🔣️.outcome.json"),
        ),
        "change-opening-width" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-width/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-width/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-width/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-width/✅apply/🔣️.outcome.json"),
        ),
        "change-opening-depth" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-depth/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-depth/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-depth/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️change-opening-depth/✅apply/🔣️.outcome.json"),
        ),
        "insert-opening" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️insert-opening/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️insert-opening/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️insert-opening/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️insert-opening/✅apply/🔣️.outcome.json"),
        ),
        "remove-opening" => (
            include_str!("../🧫️fixtures/🧬️mutations/🕳️remove-opening/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️remove-opening/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️remove-opening/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🕳️remove-opening/✅apply/🔣️.outcome.json"),
        ),
        "change-stud-row-angle" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-angle/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-angle/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-angle/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-angle/✅apply/🔣️.outcome.json"),
        ),
        "change-stud-row-count" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-count/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-count/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-count/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-count/✅apply/🔣️.outcome.json"),
        ),
        "change-stud-row-spacing-first" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-spacing-first/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-spacing-first/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-spacing-first/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-spacing-first/✅apply/🔣️.outcome.json"),
        ),
        "change-stud-row-spacing-radial" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-spacing-radial/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-spacing-radial/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-spacing-radial/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-spacing-radial/✅apply/🔣️.outcome.json"),
        ),
        "change-stud-row-diameter" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-diameter/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-diameter/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-diameter/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️change-stud-row-diameter/✅apply/🔣️.outcome.json"),
        ),
        "insert-stud-row" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️insert-stud-row/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️insert-stud-row/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️insert-stud-row/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️insert-stud-row/✅apply/🔣️.outcome.json"),
        ),
        "remove-stud-row" => (
            include_str!("../🧫️fixtures/🧬️mutations/🔩️remove-stud-row/✅apply/🔣️.before.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️remove-stud-row/✅apply/🔣️.mutation.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️remove-stud-row/✅apply/🔣️.after.json"),
            include_str!("../🧫️fixtures/🧬️mutations/🔩️remove-stud-row/✅apply/🔣️.outcome.json"),
        ),
        other => panic!("mutate-en1992-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_en1992_mutations::En1992Mutation;
    use semio_s_artifact_norm_en1992_mutations::mutation_bridge::{apply_en1992_mutation, inverse_en1992_mutation};

    use semio_s_artifact_norm_en1992_io::text::mutations::decode_en1992_mutation_json;
    use semio_s_artifact_norm_en1992_model::En1992Snapshot;
    use semio_s_artifact_norm_en1992_io::binary::snapshot::encode_en1992_pack;
    use semio_s_artifact_norm_en1992_io::binary::snapshot::decode_en1992_pack;
    use semio_s_artifact_norm_en1992_io::text::snapshot::encode_en1992_dsl;
    use semio_s_artifact_norm_en1992_io::text::snapshot::decode_en1992_dsl;
    use semio_s_artifact_norm_en1992_io::text::snapshot::decode_en1992_snapshot_json;
    use semio_s_artifact_norm_en1992_io::text::snapshot::encode_en1992_snapshot_json;
    use semio_repo_test_host::law;

    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1992Snapshot, String> {
        decode_en1992_snapshot_json(text).map_err(|error| format!("mutate-en1992-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1992Mutation, String> {
        decode_en1992_mutation_json(text).map_err(|error| format!("mutate-en1992-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1992Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1992_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1992Snapshot, expected: &En1992Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1992_snapshot_json(got), encode_en1992_snapshot_json(expected))
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
            let applied = apply_en1992_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected" | "no-op", Ok((snapshot, messages))) if !messages.iter().any(|message| message.ends_with(&format!(":{code}"))) => return Err(format!("mutate-{kind}: the committed vector declares {status} with {code}, yet it raised {messages:?} — the document came back as {}", encode_en1992_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1992_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1992_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1992_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.input_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1992 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1992_dsl(&text)?;
        let reprinted = encode_en1992_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1992_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1992_pack(&encode_en1992_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1992_snapshot_json(&encode_en1992_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1992_pack(&ctx.input_bytes(super::PACK_ASSET)?)?;
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
