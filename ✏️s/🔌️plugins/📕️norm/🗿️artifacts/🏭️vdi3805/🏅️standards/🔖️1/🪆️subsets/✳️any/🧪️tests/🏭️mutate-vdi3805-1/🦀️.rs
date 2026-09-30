//! 🦀️ VDI 3805 exhaustive mutation case — the Rust SUBJECT adapter over the current 19-kind `Vdi3805Mutation`
//! vocabulary. Every kind applies its committed vector (`../../🧫️fixtures/🧬️mutations/<leaf>/<scenario>`) through the
//! subset's production bridges and asserts, in role, the committed after-snapshot, that the document moved and that
//! its own inverse restores the before-snapshot; `identity-round-trip` re-emits the committed carrier through the DSL, pack and JSON codecs. The reference answer is the shared Python norm engine (`../🐍️.py`).

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{digest, parse_json, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `Vdi3805Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-manufacturer-file",
    "change-limits",
    "change-correction-as-of",
    "change-strict-mode",
    "change-edition-profile",
    "remove-edition-profile",
    "add-product",
    "remove-product",
    "rename-product",
    "change-product-configuration",
    "add-geometry",
    "remove-geometry",
    "resize-geometry",
    "add-geometry-connection",
    "remove-geometry-connection",
    "change-geometry-parameters",
    "add-curve",
    "remove-curve",
    "change-curve-points",
];

/// 🗣️ The real committed VDI 3805 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🎬️demo/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-manufacturer-file" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏭️change-manufacturer-file/✏️sets-file/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏭️change-manufacturer-file/✏️sets-file/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏭️change-manufacturer-file/✏️sets-file/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏭️change-manufacturer-file/✏️sets-file/🎯️outcome/🔣️.json"),
        ),
        "change-limits" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🚧️change-limits/🛡️tightens-every/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚧️change-limits/🛡️tightens-every/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚧️change-limits/🛡️tightens-every/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚧️change-limits/🛡️tightens-every/🎯️outcome/🔣️.json"),
        ),
        "change-correction-as-of" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📅️change-correction-as-of/✏️sets-of/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📅️change-correction-as-of/✏️sets-of/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📅️change-correction-as-of/✏️sets-of/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📅️change-correction-as-of/✏️sets-of/🎯️outcome/🔣️.json"),
        ),
        "change-strict-mode" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔒️change-strict-mode/🔒️turns-strict/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔒️change-strict-mode/🔒️turns-strict/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔒️change-strict-mode/🔒️turns-strict/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔒️change-strict-mode/🔒️turns-strict/🎯️outcome/🔣️.json"),
        ),
        "change-edition-profile" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔖️change-edition-profile/✏️to-current/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔖️change-edition-profile/✏️to-current/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔖️change-edition-profile/✏️to-current/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔖️change-edition-profile/✏️to-current/🎯️outcome/🔣️.json"),
        ),
        "remove-edition-profile" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧹️remove-edition-profile/➖️removes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧹️remove-edition-profile/➖️removes/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧹️remove-edition-profile/➖️removes/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧹️remove-edition-profile/➖️removes/🎯️outcome/🔣️.json"),
        ),
        "add-product" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📦️add-product/📦️appends-vlv-80-002/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📦️add-product/📦️appends-vlv-80-002/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📦️add-product/📦️appends-vlv-80-002/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📦️add-product/📦️appends-vlv-80-002/🎯️outcome/🔣️.json"),
        ),
        "remove-product" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🗑️remove-product/🚫️removes-vlv-50-001/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗑️remove-product/🚫️removes-vlv-50-001/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗑️remove-product/🚫️removes-vlv-50-001/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🗑️remove-product/🚫️removes-vlv-50-001/🎯️outcome/🔣️.json"),
        ),
        "rename-product" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️rename-product/🏷️retitles-vlv-50/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️rename-product/🏷️retitles-vlv-50/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️rename-product/🏷️retitles-vlv-50/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🏷️rename-product/🏷️retitles-vlv-50/🎯️outcome/🔣️.json"),
        ),
        "change-product-configuration" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🎛️change-product-configuration/✏️sets/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🎛️change-product-configuration/✏️sets/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🎛️change-product-configuration/✏️sets/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🎛️change-product-configuration/✏️sets/🎯️outcome/🔣️.json"),
        ),
        "add-geometry" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧊️add-geometry/🧊️adds-the-geom-valve/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧊️add-geometry/🧊️adds-the-geom-valve/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧊️add-geometry/🧊️adds-the-geom-valve/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧊️add-geometry/🧊️adds-the-geom-valve/🎯️outcome/🔣️.json"),
        ),
        "remove-geometry" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🚮️remove-geometry/🚫️removes-the-geom/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚮️remove-geometry/🚫️removes-the-geom/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚮️remove-geometry/🚫️removes-the-geom/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🚮️remove-geometry/🚫️removes-the-geom/🎯️outcome/🔣️.json"),
        ),
        "resize-geometry" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📐️resize-geometry/📐️doubles-the-geom/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️resize-geometry/📐️doubles-the-geom/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️resize-geometry/📐️doubles-the-geom/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📐️resize-geometry/📐️doubles-the-geom/🎯️outcome/🔣️.json"),
        ),
        "add-geometry-connection" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔌️add-geometry-connection/➕️adds/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔌️add-geometry-connection/➕️adds/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔌️add-geometry-connection/➕️adds/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔌️add-geometry-connection/➕️adds/🎯️outcome/🔣️.json"),
        ),
        "remove-geometry-connection" => (
            include_str!("../../🧫️fixtures/🧬️mutations/✂️remove-geometry-connection/➖️removes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✂️remove-geometry-connection/➖️removes/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✂️remove-geometry-connection/➖️removes/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/✂️remove-geometry-connection/➖️removes/🎯️outcome/🔣️.json"),
        ),
        "change-geometry-parameters" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🧮️change-geometry-parameters/✏️sets/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧮️change-geometry-parameters/✏️sets/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧮️change-geometry-parameters/✏️sets/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🧮️change-geometry-parameters/✏️sets/🎯️outcome/🔣️.json"),
        ),
        "add-curve" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📈️add-curve/📈️adds-the-curve-dp/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📈️add-curve/📈️adds-the-curve-dp/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📈️add-curve/📈️adds-the-curve-dp/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📈️add-curve/📈️adds-the-curve-dp/🎯️outcome/🔣️.json"),
        ),
        "remove-curve" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📉️remove-curve/🚫️removes-the-curve/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📉️remove-curve/🚫️removes-the-curve/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📉️remove-curve/🚫️removes-the-curve/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📉️remove-curve/🚫️removes-the-curve/🎯️outcome/🔣️.json"),
        ),
        "change-curve-points" => (
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-curve-points/✏️sets-points/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-curve-points/✏️sets-points/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-curve-points/✏️sets-points/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/📍️change-curve-points/✏️sets-points/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-vdi3805-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_artifact_norm_vdi3805::standards::v1::subsets::any::schema::mutations::{apply_vdi3805_mutation, decode_vdi3805_mutation_json, inverse_vdi3805_mutation, Vdi3805Mutation};
    use semio_s_artifact_norm_vdi3805::standards::v1::subsets::any::schema::snapshot::{decode_vdi3805_dsl, decode_vdi3805_pack, decode_vdi3805_snapshot_json, encode_vdi3805_dsl, encode_vdi3805_pack, encode_vdi3805_snapshot_json, Vdi3805Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<Vdi3805Snapshot, String> {
        decode_vdi3805_snapshot_json(text).map_err(|error| format!("mutate-vdi3805-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<Vdi3805Mutation, String> {
        decode_vdi3805_mutation_json(text).map_err(|error| format!("mutate-vdi3805-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &Vdi3805Snapshot) -> Result<Json, String> {
        parse_json(&encode_vdi3805_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &Vdi3805Snapshot, expected: &Vdi3805Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_vdi3805_snapshot_json(got), encode_vdi3805_snapshot_json(expected))
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
            let applied = apply_vdi3805_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_vdi3805_snapshot_json(&snapshot))),
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
            let mut current = match apply_vdi3805_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_vdi3805_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_vdi3805_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed VDI 3805 artifact is not UTF-8: {error}"))?;
        let parsed = decode_vdi3805_dsl(&text)?;
        let reprinted = encode_vdi3805_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_vdi3805_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_vdi3805_pack(&encode_vdi3805_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_vdi3805_snapshot_json(&encode_vdi3805_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
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
