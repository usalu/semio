//! 📜️ Wires artifact — native `.wires` DSL text codec (ticket
//! `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` design.md §1 CORRECTION: the native codec is
//! one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>/`, unsplit —
//! relocated here verbatim from `🧬️schema/📸️snapshot/📝️text`, taking `impl store::ArtifactDsl for
//! WiresSnapshot` with it from `🧬️schema/📸️snapshot/🦀️component.rs`'s former `🔖️HandcraftedArtifactCodecs`
//! region — `🧬️schema` now keeps only the `WiresSnapshot` type). `content` is a composed
//! `store::ArtifactChild<SemioGraphSnapshot>`, which has no `dsl::DslRecord` derive support, so this
//! hand-rolls the whole codec. `WiresMutation`'s own op-text grammar is unaffected
//! (`#[derive(dsl::DslEnum)]`, in `crate::artifacts::wires::op`).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::wires::{wires_working_scene, WiresSnapshot};
use dsl::DslValue;

/// 📄️ The `metabolism` example, handcrafted in the `.wires` DSL — source of truth for every
/// "metabolism" example call site (`setActiveExample`, `.example` manifest registration, tests).
pub const REASONING_WIRES_EXAMPLE_METABOLISM_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

//#region 🔖️TextPrimitives
/// 🧪️ Real hex-encoded text primitives — one `key=<hex>` line per field (`📓️migration-recipe.md`
/// §2's convention), duplicated locally rather than imported across facets (keeps this file
/// independently compilable, matching `✳️graph`'s own `🔖️GraphPrimitives` precedent in stdio).
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

/// ⚠️ Serializes/deserializes `DslValue` DIRECTLY (`serde_json::to_string`/`from_str::<DslValue>`),
/// never via the `dsl_to_json`/`serde_json::Value` intermediate `crate::artifacts::wires::schema`'s
/// `fixture_json_string`/`dsl_to_json` use elsewhere: `serde_json::Value::Object` normalizes key
/// order (alphabetical, no `preserve_order` feature), which silently reordered `wires_fixture`'s
/// object keys on every round trip and broke `DslValue::Object`'s (order-sensitive, `Vec`-backed)
/// `PartialEq` — a real bug this pass's round-trip tests caught (not just latent risk). `DslValue`'s
/// own hand-written `Serialize`/`Deserialize` impl (`dsl_value_serde.rs`) preserves entry order
/// end-to-end, so encoding/decoding it directly (bypassing `serde_json::Value` entirely) is lossless.
async fn enc_dsl(value: &DslValue) -> String {
    hex_encode(serde_json::to_string(value).unwrap_or_default().as_bytes())
}
async fn dec_dsl(s: &str) -> Result<DslValue, String> {
    let bytes = hex_decode(s)?;
    let text = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    serde_json::from_str::<DslValue>(&text).map_err(|e| e.to_string())
}
async fn enc_dsl_list(values: &[DslValue]) -> String {
    hex_encode(serde_json::to_string(values).unwrap_or_default().as_bytes())
}
async fn dec_dsl_list(s: &str) -> Result<Vec<DslValue>, String> {
    let bytes = hex_decode(s)?;
    let text = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    serde_json::from_str::<Vec<DslValue>>(&text).map_err(|e| e.to_string())
}

async fn to_text_error(message: String) -> store::TextError {
    store::TextError::new(message, dsl::TextSpan::at(1, 1))
}

/// 📄️ The real structured body: `wires=<hex>` / `nodes=[<hex>...]` / `edges=[<hex>...]` /
/// `camera=<hex>` / `meta=<hex>` — five lines, each independently hex-decodable.
async fn print_wires_snapshot_body(snapshot: &WiresSnapshot) -> String {
    let scene = wires_working_scene(snapshot);
    format!("wires={}\nnodes={}\nedges={}\ncamera={}\nmeta={}", enc_dsl(&snapshot.wires_fixture), enc_dsl_list(&scene.nodes), enc_dsl_list(&scene.edges), enc_dsl(&snapshot.camera), enc_dsl(&snapshot.meta),)
}

async fn parse_wires_snapshot_body(body: &str) -> Result<WiresSnapshot, store::TextError> {
    let mut wires_fixture = None;
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut camera = None;
    let mut meta = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("wires=") {
            wires_fixture = Some(dec_dsl(rest).map_err(to_text_error)?);
        } else if let Some(rest) = line.strip_prefix("nodes=") {
            nodes = dec_dsl_list(rest).map_err(to_text_error)?;
        } else if let Some(rest) = line.strip_prefix("edges=") {
            edges = dec_dsl_list(rest).map_err(to_text_error)?;
        } else if let Some(rest) = line.strip_prefix("camera=") {
            camera = Some(dec_dsl(rest).map_err(to_text_error)?);
        } else if let Some(rest) = line.strip_prefix("meta=") {
            meta = Some(dec_dsl(rest).map_err(to_text_error)?);
        } else {
            return Err(to_text_error(format!("wires snapshot: unknown line {line:?}")));
        }
    }
    let content = crate::artifacts::wires::wires_content_child_handle_and_cache(nodes, edges);
    Ok(WiresSnapshot { wires_fixture: wires_fixture.ok_or_else(|| to_text_error("wires snapshot: missing wires line".into()))?, content, camera: camera.unwrap_or_else(crate::artifacts::wires::empty_camera), meta: meta.unwrap_or(DslValue::Null) })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ P6 handcrafted `ArtifactDsl` (derive no longer emits this trait once `content` drops to a
/// composed `ArtifactChild` — see this file's module doc).
impl store::ArtifactDsl for WiresSnapshot {
    const EXTENSION: &'static str = "wires";
    async fn envelope_id() -> &'static str {
        "reasoning.wires"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_wires_snapshot_body(body)
    }
    async fn print_dsl(&self) -> String {
        let body = print_wires_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📖️ Parses `.wires` DSL text into a `WiresSnapshot`.
pub async fn parse_dsl(text: &str) -> Result<WiresSnapshot, store::TextError> {
    <WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `WiresSnapshot` back to `.wires` DSL text.
pub async fn print_dsl(document: &WiresSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::{empty_wires_snapshot, wires_working_board};

    async fn populated() -> WiresSnapshot {
        let mut snapshot = empty_wires_snapshot();
        let node = dsl::to_dsl_value(&serde_json::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 1.0, "y": 2.0, "radius": 24.0, "text": "Alpha", "handles": [] })).unwrap();
        snapshot = store::apply_mutation(&snapshot, &crate::artifacts::wires::mutations::create_node(node)).expect("valid mutation").0;
        snapshot
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_round_trip_empty_document() {
        let document = crate::artifacts::wires::empty_wires_snapshot();
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_round_trip_metabolism_fixture() {
        let document = crate::artifacts::wires::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
        assert_eq!(document.wires_fixture.get("identities").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
        assert_eq!(document.wires_fixture.get("relationships").and_then(|value| value.as_array()).map(|items| items.len()), Some(9));
        assert_eq!(crate::artifacts::wires::wires_working_board(&document).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
        let reparsed = parse_dsl(&print_dsl(&document)).expect("metabolism dsl round trip");
        assert_eq!(crate::artifacts::wires::wires_working_board(&reparsed).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips_empty() {
        let snapshot = empty_wires_snapshot();
        let text = <WiresSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let back = <WiresSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(back.wires_fixture, snapshot.wires_fixture);
        assert_eq!(wires_working_board(&back), wires_working_board(&snapshot));
    }

    /// ⚖️ codec_retention_law: a populated snapshot (real node content, not just the default) survives
    /// BOTH codecs — this is what a bare-handle-only codec would silently fail (see this file's module
    /// doc, `dag`'s bug writeup).
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law_carries_real_node_content_not_just_the_handle() {
        let snapshot = populated();
        let text = <WiresSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let back_text = <WiresSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(wires_working_board(&back_text).get("nodes").and_then(|v| v.as_array()).map(|a| a.len()), Some(1), "node content must survive a FRESH decode, not just round-trip in-process");
        let bytes = <WiresSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let back_pack = <WiresSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(wires_working_board(&back_pack).get("nodes").and_then(|v| v.as_array()).map(|a| a.len()), Some(1));
    }
}
//#endregion 🧪️Tests
