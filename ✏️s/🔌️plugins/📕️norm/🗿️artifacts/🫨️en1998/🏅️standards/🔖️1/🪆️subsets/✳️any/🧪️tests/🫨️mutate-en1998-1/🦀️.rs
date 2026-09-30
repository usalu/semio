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
            include_str!("../../🧫️fixtures/🧬️mutations/📎️change-annex/🌍️switches-to-en/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📎️change-annex/🌍️switches-to-en/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📎️change-annex/🌍️switches-to-en/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📎️change-annex/🌍️switches-to-en/🎯️outcome/🔣️.json"),
        ),
        "update-site" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/🌋️zone-3-soft-soil/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/🌋️zone-3-soft-soil/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/🌋️zone-3-soft-soil/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌚️update-site/🌋️zone-3-soft-soil/🎯️outcome/🔣️.json"),
        ),
        "insert-building" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/🏢️adds-an-annex/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/🏢️adds-an-annex/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/🏢️adds-an-annex/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-building/🏢️adds-an-annex/🎯️outcome/🔣️.json"),
        ),
        "remove-building" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-building/🏚️drops-the-office/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-building/🏚️drops-the-office/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-building/🏚️drops-the-office/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-building/🏚️drops-the-office/🎯️outcome/🔣️.json"),
        ),
        "change-system-v-rd-n" => (
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/💪️stronger-y/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/💪️stronger-y/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/💪️stronger-y/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-system-v-rd-n/💪️stronger-y/🎯️outcome/🔣️.json"),
        ),
        "change-storey-permanent-gk-n" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/⚖️heavier/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/⚖️heavier/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/⚖️heavier/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⚖️change-storey-permanent-gk-n/⚖️heavier/🎯️outcome/🔣️.json"),
        ),
        "change-storey-stiffness-x" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/📐️softer/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/📐️softer/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/📐️softer/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-storey-stiffness-x/📐️softer/🎯️outcome/🔣️.json"),
        ),
        "change-storey-drift-xm" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/📏️more-drift/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/📏️more-drift/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/📏️more-drift/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-storey-drift-xm/📏️more-drift/🎯️outcome/🔣️.json"),
        ),
        "change-building-plan-regular" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/🧭️twist/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/🧭️twist/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/🧭️twist/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧭️change-building-plan-regular/🧭️twist/🎯️outcome/🔣️.json"),
        ),
        "change-elevation-regular" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/🏙️irregular/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/🏙️irregular/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/🏙️irregular/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-elevation-regular/🏙️irregular/🎯️outcome/🔣️.json"),
        ),
        "change-member-detailing" => (
            include_str!("../../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅️beam-unfit/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅️beam-unfit/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅️beam-unfit/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✅️change-member-detailing/✅️beam-unfit/🎯️outcome/🔣️.json"),
        ),
        "change-masonry-wall-ratio" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/🧱️four-pct/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/🧱️four-pct/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/🧱️four-pct/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️change-masonry-wall-ratio/🧱️four-pct/🎯️outcome/🔣️.json"),
        ),
        "insert-bridge" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/🌉️adds-a-viaduct/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/🌉️adds-a-viaduct/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/🌉️adds-a-viaduct/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌉insert-bridge/🌉️adds-a-viaduct/🎯️outcome/🔣️.json"),
        ),
        "change-bridge-v-rd-n" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/🛑️stronger-pier/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/🛑️stronger-pier/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/🛑️stronger-pier/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛑️change-bridge-v-rd-n/🛑️stronger-pier/🎯️outcome/🔣️.json"),
        ),
        "insert-assessment" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/🔧️adds-a-kl3-check/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/🔧️adds-a-kl3-check/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/🔧️adds-a-kl3-check/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔧insert-assessment/🔧️adds-a-kl3-check/🎯️outcome/🔣️.json"),
        ),
        "change-assessment-rkn" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/🏋️stronger/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/🏋️stronger/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/🏋️stronger/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-assessment-rkn/🏋️stronger/🎯️outcome/🔣️.json"),
        ),
        "insert-silo" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/🫙️adds-a-grain-silo/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/🫙️adds-a-grain-silo/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/🫙️adds-a-grain-silo/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🫙insert-silo/🫙️adds-a-grain-silo/🎯️outcome/🔣️.json"),
        ),
        "insert-tank" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/🛢️adds-a-water-tank/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/🛢️adds-a-water-tank/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/🛢️adds-a-water-tank/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛢insert-tank/🛢️adds-a-water-tank/🎯️outcome/🔣️.json"),
        ),
        "insert-foundation" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/🪨️adds-a-pad/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/🪨️adds-a-pad/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/🪨️adds-a-pad/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🪨insert-foundation/🪨️adds-a-pad/🎯️outcome/🔣️.json"),
        ),
        "insert-retaining-wall" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/🧱️adds-a-wall/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/🧱️adds-a-wall/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/🧱️adds-a-wall/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱️insert-retaining-wall/🧱️adds-a-wall/🎯️outcome/🔣️.json"),
        ),
        "insert-tower" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/🏭️adds-a-chimney/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/🏭️adds-a-chimney/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/🏭️adds-a-chimney/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗼insert-tower/🏭️adds-a-chimney/🎯️outcome/🔣️.json"),
        ),
        "change-tower-m-rd-nm" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/↪️stronger-base/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/↪️stronger-base/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/↪️stronger-base/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-tower-m-rd-nm/↪️stronger-base/🎯️outcome/🔣️.json"),
        ),
        "remove-bridge" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge/🌉️drops-the-viaduct/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge/🌉️drops-the-viaduct/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge/🌉️drops-the-viaduct/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-bridge/🌉️drops-the-viaduct/🎯️outcome/🔣️.json"),
        ),
        "remove-assessment" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-assessment/🔧️drops-the-check/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-assessment/🔧️drops-the-check/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-assessment/🔧️drops-the-check/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-assessment/🔧️drops-the-check/🎯️outcome/🔣️.json"),
        ),
        "remove-silo" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo/🫙️drops-the-silo/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo/🫙️drops-the-silo/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo/🫙️drops-the-silo/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-silo/🫙️drops-the-silo/🎯️outcome/🔣️.json"),
        ),
        "remove-tank" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tank/🛢️drops-the-tank/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tank/🛢️drops-the-tank/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tank/🛢️drops-the-tank/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tank/🛢️drops-the-tank/🎯️outcome/🔣️.json"),
        ),
        "remove-foundation" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-foundation/🪨️drops-the-pad/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-foundation/🪨️drops-the-pad/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-foundation/🪨️drops-the-pad/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-foundation/🪨️drops-the-pad/🎯️outcome/🔣️.json"),
        ),
        "remove-retaining-wall" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/🧱️drops-it/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/🧱️drops-it/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/🧱️drops-it/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-retaining-wall/🧱️drops-it/🎯️outcome/🔣️.json"),
        ),
        "remove-tower" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower/🗼️drops-the-tower/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower/🗼️drops-the-tower/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower/🗼️drops-the-tower/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-tower/🗼️drops-the-tower/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-en1998-1: no committed fixture is registered for kind {other:?}"),
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
            let applied = apply_en1998_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_en1998_snapshot_json(&snapshot))),
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
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
