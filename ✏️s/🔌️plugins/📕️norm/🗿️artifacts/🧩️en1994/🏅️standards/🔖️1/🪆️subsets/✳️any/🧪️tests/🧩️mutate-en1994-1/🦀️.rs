//! 🦀️ EN 1994 exhaustive mutation case — the Rust SUBJECT adapter over the current 25-kind `En1994Mutation`
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
    "change-structure-kind",
    "change-steel-fy-pa",
    "change-fire-rating",
    "change-insulation-thickness-m",
    "change-fatigue-detail",
    "insert-beam",
    "remove-beam",
    "change-beam-action-q-area-pa",
    "change-beam-stud-spacing-m",
    "change-beam-span-m",
    "change-beam-slab-thickness-m",
    "change-beam-stud-diameter-m",
    "change-beam-stud-count",
    "change-beam-stud-fu-pa",
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
        "change-annex" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌐️switches-national/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌐️switches-national/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌐️switches-national/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌐️switches-national/🎯️outcome/🔣️.json"),
        ),
        "change-structure-kind" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏗️change-structure-kind/✏️to-bridge/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏗️change-structure-kind/✏️to-bridge/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏗️change-structure-kind/✏️to-bridge/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏗️change-structure-kind/✏️to-bridge/🎯️outcome/🔣️.json"),
        ),
        "change-steel-fy-pa" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-steel-fy-pa/🏋️upgrades-fy/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-steel-fy-pa/🏋️upgrades-fy/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-steel-fy-pa/🏋️upgrades-fy/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏋️change-steel-fy-pa/🏋️upgrades-fy/🎯️outcome/🔣️.json"),
        ),
        "change-fire-rating" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-fire-rating/🔥️upgrades-to-r90/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-fire-rating/🔥️upgrades-to-r90/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-fire-rating/🔥️upgrades-to-r90/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔥️change-fire-rating/🔥️upgrades-to-r90/🎯️outcome/🔣️.json"),
        ),
        "change-insulation-thickness-m" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✏️to-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✏️to-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✏️to-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✏️to-0/🎯️outcome/🔣️.json"),
        ),
        "change-fatigue-detail" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔁️change-fatigue-detail/✏️to-flange/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔁️change-fatigue-detail/✏️to-flange/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔁️change-fatigue-detail/✏️to-flange/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔁️change-fatigue-detail/✏️to-flange/🎯️outcome/🔣️.json"),
        ),
        "insert-beam" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-beam/➕️inserts-beam/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-beam/➕️inserts-beam/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-beam/➕️inserts-beam/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕️insert-beam/➕️inserts-beam/🎯️outcome/🔣️.json"),
        ),
        "remove-beam" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-beam/➖️removes-beam/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-beam/➖️removes-beam/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-beam/➖️removes-beam/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖️remove-beam/➖️removes-beam/🎯️outcome/🔣️.json"),
        ),
        "change-beam-action-q-area-pa" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🌀️change-beam-action-q-area-pa/✏️sets/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌀️change-beam-action-q-area-pa/✏️sets/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌀️change-beam-action-q-area-pa/✏️sets/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🌀️change-beam-action-q-area-pa/✏️sets/🎯️outcome/🔣️.json"),
        ),
        "change-beam-stud-spacing-m" => (
            include_str!("../../🧫️fixtures/🧬️mutations/✂️change-beam-stud-spacing-m/✏️to-0-15/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✂️change-beam-stud-spacing-m/✏️to-0-15/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✂️change-beam-stud-spacing-m/✏️to-0-15/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✂️change-beam-stud-spacing-m/✏️to-0-15/🎯️outcome/🔣️.json"),
        ),
        "change-beam-span-m" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-beam-span-m/📏️sets-span-10m/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-beam-span-m/📏️sets-span-10m/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-beam-span-m/📏️sets-span-10m/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏️change-beam-span-m/📏️sets-span-10m/🎯️outcome/🔣️.json"),
        ),
        "change-beam-slab-thickness-m" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧱change-beam-slab-thickness-m/✏️to-0-16/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱change-beam-slab-thickness-m/✏️to-0-16/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱change-beam-slab-thickness-m/✏️to-0-16/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧱change-beam-slab-thickness-m/✏️to-0-16/🎯️outcome/🔣️.json"),
        ),
        "change-beam-stud-diameter-m" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⭕️change-beam-stud-diameter-m/✏️to-0-022/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⭕️change-beam-stud-diameter-m/✏️to-0-022/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⭕️change-beam-stud-diameter-m/✏️to-0-022/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⭕️change-beam-stud-diameter-m/✏️to-0-022/🎯️outcome/🔣️.json"),
        ),
        "change-beam-stud-count" => (
            include_str!("../../🧫️fixtures/🧬️mutations/#️⃣change-beam-stud-count/#️⃣sets-count/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/#️⃣change-beam-stud-count/#️⃣sets-count/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/#️⃣change-beam-stud-count/#️⃣sets-count/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/#️⃣change-beam-stud-count/#️⃣sets-count/🎯️outcome/🔣️.json"),
        ),
        "change-beam-stud-fu-pa" => (
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-beam-stud-fu-pa/💪️sets-fu/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-beam-stud-fu-pa/💪️sets-fu/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-beam-stud-fu-pa/💪️sets-fu/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/💪️change-beam-stud-fu-pa/💪️sets-fu/🎯️outcome/🔣️.json"),
        ),
        "change-beam-transverse-as" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-beam-transverse-as/↔️sets-as/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-beam-transverse-as/↔️sets-as/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-beam-transverse-as/↔️sets-as/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↔️change-beam-transverse-as/↔️sets-as/🎯️outcome/🔣️.json"),
        ),
        "change-beam-construction" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🛠️change-beam-construction/✏️sets/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛠️change-beam-construction/✏️sets/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛠️change-beam-construction/✏️sets/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🛠️change-beam-construction/✏️sets/🎯️outcome/🔣️.json"),
        ),
        "insert-column" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➗️insert-column/➗️inserts-column/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➗️insert-column/➗️inserts-column/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➗️insert-column/➗️inserts-column/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➗️insert-column/➗️inserts-column/🎯️outcome/🔣️.json"),
        ),
        "remove-column" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⛔️remove-column/⛔️removes-column/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⛔️remove-column/⛔️removes-column/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⛔️remove-column/⛔️removes-column/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⛔️remove-column/⛔️removes-column/🎯️outcome/🔣️.json"),
        ),
        "change-column-action-force-n" => (
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-column-action-force-n/⬇️sets-n/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-column-action-force-n/⬇️sets-n/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-column-action-force-n/⬇️sets-n/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/⬇️change-column-action-force-n/⬇️sets-n/🎯️outcome/🔣️.json"),
        ),
        "change-column-kind" => (
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-column-kind/↪️sets-kind/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-column-kind/↪️sets-kind/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-column-kind/↪️sets-kind/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/↪️change-column-kind/↪️sets-kind/🎯️outcome/🔣️.json"),
        ),
        "insert-slab" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➕insert-slab/➕inserts-slab/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕insert-slab/➕inserts-slab/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕insert-slab/➕inserts-slab/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➕insert-slab/➕inserts-slab/🎯️outcome/🔣️.json"),
        ),
        "remove-slab" => (
            include_str!("../../🧫️fixtures/🧬️mutations/➖remove-slab/➖removes-slab/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖remove-slab/➖removes-slab/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖remove-slab/➖removes-slab/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/➖remove-slab/➖removes-slab/🎯️outcome/🔣️.json"),
        ),
        "change-slab-action-q-area-pa" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-slab-action-q-area-pa/✏️sets/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-slab-action-q-area-pa/✏️sets/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-slab-action-q-area-pa/✏️sets/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️change-slab-action-q-area-pa/✏️sets/🎯️outcome/🔣️.json"),
        ),
        "change-slab-thickness-m" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📏change-slab-thickness-m/📏sets-thickness/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏change-slab-thickness-m/📏sets-thickness/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏change-slab-thickness-m/📏sets-thickness/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📏change-slab-thickness-m/📏sets-thickness/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-en1994-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_en1994::standards::v1::subsets::any::schema::mutations::{apply_en1994_mutation, decode_en1994_mutation_json, inverse_en1994_mutation, En1994Mutation};
    use semio_s_artifact_norm_en1994::standards::v1::subsets::any::schema::snapshot::{decode_en1994_dsl, decode_en1994_pack, decode_en1994_snapshot_json, encode_en1994_dsl, encode_en1994_pack, encode_en1994_snapshot_json, En1994Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
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
            let applied = apply_en1994_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_en1994_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1994_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1994_mutation(&mutation, &base);
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1994 artifact is not UTF-8: {error}"))?;
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
        let twin = decode_en1994_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
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
