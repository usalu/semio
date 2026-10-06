//! 🦀️ EN 1993 exhaustive mutation case — the Rust SUBJECT adapter over the current 49-kind `En1993Mutation`
//! vocabulary. Every kind applies its committed vector (`../../🧫️fixtures/🧬️mutations/<leaf>/<scenario>`) through the
//! subset's production bridges and asserts, in role, the committed after-snapshot, that the document moved and that
//! its own inverse restores the before-snapshot; `identity-round-trip` re-emits the committed carrier through the DSL,
//! pack and JSON codecs. The reference answer is the shared Python norm engine (`../🐍️.py`).

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{digest, parse_json, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `En1993Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-annex",
    "update-member-properties",
    "update-fire-inputs",
    "update-cold-formed-inputs",
    "update-stainless-inputs",
    "update-plated-inputs",
    "update-silo-shell-inputs",
    "update-bolt-inputs",
    "update-weld-inputs",
    "update-fatigue-inputs",
    "update-through-thickness-inputs",
    "update-tension-component-inputs",
    "update-hss-inputs",
    "update-bridge-inputs",
    "update-tower-inputs",
    "update-pile-inputs",
    "update-crane-inputs",
    "insert-material",
    "remove-material",
    "insert-section",
    "remove-section",
    "insert-member",
    "remove-member",
    "insert-load-case",
    "remove-load-case",
    "insert-member-action",
    "remove-member-action",
    "insert-joint",
    "remove-joint",
    "insert-fatigue-detail",
    "remove-fatigue-detail",
    "insert-fire-exposure",
    "remove-fire-exposure",
    "insert-cold-formed-member",
    "remove-cold-formed-member",
    "insert-plated-panel",
    "remove-plated-panel",
    "insert-silo-shell",
    "remove-silo-shell",
    "insert-tension-component",
    "remove-tension-component",
    "insert-bridge-fatigue",
    "remove-bridge-fatigue",
    "insert-tower-leg",
    "remove-tower-leg",
    "insert-pile",
    "remove-pile",
    "insert-crane-runway",
    "remove-crane-runway",
];

/// ⛔️ The refusal witnesses — `<kind>-<slug>` rows whose committed `🎯️outcome` declares the refusal and its code.
/// Registered for `mutate-` only: a refused mutation leaves nothing to undo.
#[cfg(feature = "sut")]
const ROWS: &[&str] = &[
    "insert-material-dupe",
    "insert-section-dupe",
    "insert-member-dupe",
    "insert-load-case-dupe",
    "insert-member-action-dupe",
    "insert-joint-dupe",
    "insert-fatigue-detail-dupe",
    "insert-fire-exposure-dupe",
    "insert-cold-formed-member-dupe",
    "insert-plated-panel-dupe",
    "insert-silo-shell-dupe",
    "insert-tension-component-dupe",
    "insert-bridge-fatigue-dupe",
    "insert-tower-leg-dupe",
    "insert-pile-dupe",
    "insert-crane-runway-dupe",
];

/// 🗣️ The real committed EN 1993 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🔩️high-strength-connection/🎒️.pack.semio";
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
        "update-member-properties" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📊️update-member-properties/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📊️update-member-properties/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📊️update-member-properties/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📊️update-member-properties/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-fire-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️update-fire-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️update-fire-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️update-fire-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️update-fire-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-cold-formed-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🥶️update-cold-formed-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🥶️update-cold-formed-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🥶️update-cold-formed-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🥶️update-cold-formed-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-stainless-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/✨️update-stainless-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✨️update-stainless-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✨️update-stainless-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✨️update-stainless-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-plated-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️update-plated-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️update-plated-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️update-plated-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️update-plated-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-silo-shell-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛢️update-silo-shell-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢️update-silo-shell-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢️update-silo-shell-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢️update-silo-shell-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-bolt-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️update-bolt-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️update-bolt-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️update-bolt-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔩️update-bolt-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-weld-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧲️update-weld-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧲️update-weld-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧲️update-weld-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧲️update-weld-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-fatigue-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔁️update-fatigue-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔁️update-fatigue-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔁️update-fatigue-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔁️update-fatigue-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-through-thickness-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↕️update-through-thickness-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️update-through-thickness-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️update-through-thickness-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↕️update-through-thickness-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-tension-component-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🪢️update-tension-component-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪢️update-tension-component-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪢️update-tension-component-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪢️update-tension-component-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-hss-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⬜️update-hss-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬜️update-hss-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬜️update-hss-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬜️update-hss-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-bridge-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️update-bridge-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️update-bridge-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️update-bridge-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉️update-bridge-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-tower-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🗼️update-tower-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼️update-tower-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼️update-tower-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼️update-tower-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-pile-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🪵️update-pile-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪵️update-pile-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪵️update-pile-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪵️update-pile-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "update-crane-inputs" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏗️update-crane-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏗️update-crane-inputs/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏗️update-crane-inputs/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏗️update-crane-inputs/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-material" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-material/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-material/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-material/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-material/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-material-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-material/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-material/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-material/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-material/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-material" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-material/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-material/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-material/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-material/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-section" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-section/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-section/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-section/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-section/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-section-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-section/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-section/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-section/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-section/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-section" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-section/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-section/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-section/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-section/✅apply/🎯️outcome/🔣️.json"),
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
        "insert-load-case" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-load-case/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-load-case/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-load-case/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-load-case/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-load-case-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-load-case/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-load-case/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-load-case/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-load-case/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-load-case" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-member-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-member-action-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-member-action/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-member-action" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-member-action/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-joint" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-joint/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-joint/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-joint/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-joint/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-joint-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-joint/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-joint/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-joint/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-joint/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-joint" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-joint/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-joint/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-joint/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-joint/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-fatigue-detail" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fatigue-detail/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fatigue-detail/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fatigue-detail/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fatigue-detail/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-fatigue-detail-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fatigue-detail/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fatigue-detail/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fatigue-detail/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fatigue-detail/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-fatigue-detail" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-fatigue-detail/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-fatigue-detail/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-fatigue-detail/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-fatigue-detail/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-fire-exposure" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-fire-exposure-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-fire-exposure" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-fire-exposure/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-fire-exposure/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-fire-exposure/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-fire-exposure/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-cold-formed-member" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-cold-formed-member/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-cold-formed-member/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-cold-formed-member/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-cold-formed-member/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-cold-formed-member-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-cold-formed-member/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-cold-formed-member/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-cold-formed-member/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-cold-formed-member/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-cold-formed-member" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-cold-formed-member/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-cold-formed-member/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-cold-formed-member/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-cold-formed-member/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-plated-panel" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-plated-panel/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-plated-panel/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-plated-panel/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-plated-panel/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-plated-panel-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-plated-panel/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-plated-panel/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-plated-panel/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-plated-panel/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-plated-panel" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-plated-panel/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-plated-panel/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-plated-panel/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-plated-panel/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-silo-shell" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-silo-shell/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-silo-shell/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-silo-shell/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-silo-shell/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-silo-shell-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-silo-shell/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-silo-shell/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-silo-shell/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-silo-shell/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-silo-shell" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo-shell/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo-shell/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo-shell/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo-shell/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-tension-component" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tension-component/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tension-component/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tension-component/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tension-component/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-tension-component-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tension-component/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tension-component/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tension-component/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tension-component/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-tension-component" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tension-component/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tension-component/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tension-component/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tension-component/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-bridge-fatigue" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-bridge-fatigue/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-bridge-fatigue/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-bridge-fatigue/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-bridge-fatigue/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-bridge-fatigue-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-bridge-fatigue/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-bridge-fatigue/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-bridge-fatigue/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-bridge-fatigue/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-bridge-fatigue" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge-fatigue/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge-fatigue/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge-fatigue/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge-fatigue/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-tower-leg" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tower-leg/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tower-leg/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tower-leg/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tower-leg/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-tower-leg-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tower-leg/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tower-leg/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tower-leg/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-tower-leg/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-tower-leg" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower-leg/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower-leg/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower-leg/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower-leg/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-pile" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-pile/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-pile/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-pile/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-pile/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-pile-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-pile/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-pile/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-pile/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-pile/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-pile" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-pile/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-pile/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-pile/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-pile/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-crane-runway" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-crane-runway/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-crane-runway/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-crane-runway/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-crane-runway/✅apply/🎯️outcome/🔣️.json"),
        ),
        "insert-crane-runway-dupe" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-crane-runway/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-crane-runway/⛔dupe/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-crane-runway/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-crane-runway/⛔dupe/🎯️outcome/🔣️.json"),
        ),
        "remove-crane-runway" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-crane-runway/✅apply/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-crane-runway/✅apply/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-crane-runway/✅apply/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-crane-runway/✅apply/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-en1993-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_en1993::standards::v1::subsets::any::schema::mutations::{apply_en1993_mutation, inverse_en1993_mutation, En1993Mutation};
    use semio_s_artifact_norm_en1993::standards::v1::subsets::any::io::text::mutations::{decode_en1993_mutation_json};
    use semio_s_artifact_norm_en1993::standards::v1::subsets::any::schema::snapshot::{En1993Snapshot};
    use semio_s_artifact_norm_en1993::standards::v1::subsets::any::io::binary::snapshot::{encode_en1993_pack};
    use semio_s_artifact_norm_en1993::standards::v1::subsets::any::io::binary::snapshot::{decode_en1993_pack};
    use semio_s_artifact_norm_en1993::standards::v1::subsets::any::io::text::snapshot::{encode_en1993_dsl};
    use semio_s_artifact_norm_en1993::standards::v1::subsets::any::io::text::snapshot::{decode_en1993_dsl};
    use semio_s_artifact_norm_en1993::standards::v1::subsets::any::io::text::snapshot::{decode_en1993_snapshot_json};
    use semio_s_artifact_norm_en1993::standards::v1::subsets::any::io::text::snapshot::{encode_en1993_snapshot_json};
    use semio_repo_test_host::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1993Snapshot, String> {
        decode_en1993_snapshot_json(text).map_err(|error| format!("mutate-en1993-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1993Mutation, String> {
        decode_en1993_mutation_json(text).map_err(|error| format!("mutate-en1993-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1993Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1993_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1993Snapshot, expected: &En1993Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1993_snapshot_json(got), encode_en1993_snapshot_json(expected))
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
            let applied = apply_en1993_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected" | "no-op", Ok((snapshot, messages))) if !messages.iter().any(|message| message.ends_with(&format!(":{code}"))) => return Err(format!("mutate-{kind}: the committed vector declares {status} with {code}, yet it raised {messages:?} — the document came back as {}", encode_en1993_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1993_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1993_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1993_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.input_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1993 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1993_dsl(&text)?;
        let reprinted = encode_en1993_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1993_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1993_pack(&encode_en1993_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1993_snapshot_json(&encode_en1993_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1993_pack(&ctx.input_bytes(super::PACK_ASSET)?)?;
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
