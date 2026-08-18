//! 📜️ Sequence artifact — native `.sequence` DSL text codec (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §1 CORRECTION: the native codec is
//! one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>/`, unsplit —
//! relocated here from `🧬️schema/📸️snapshot/📝️text` verbatim, `🧬️schema` keeps only the
//! `SequenceSnapshot` type). Carries the grammar doc-string, the real `store::ArtifactDsl for
//! SequenceSnapshot` impl, example text, and round-trip laws. Ticket
//! `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`) dropped the old
//! `SequenceEdgeDsl` unified-`dsl::Wire` mirror here — the snapshot no longer embeds `edges`
//! structurally in its own text grammar at all (only the opaque composed `content` handle), so a
//! per-edge DSL mirror has nothing left to mirror.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::sequence::{SequenceContentChild, SequenceSnapshot};

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `📐️cad`/`✒️writer`'s own `enc_child`/`dec_child`)
/// — a handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened via
/// `to_uri()`), never the child's own content. `SequenceSnapshot` no longer derives
/// `dsl::DslRecord` (the composed child has no reachable `DslField` impl from this crate) — this
/// facet hand-rolls the whole `ArtifactDsl`/`ArtifactPack` codec instead.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
fn enc_child(c: &SequenceContentChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_child(s: &str) -> Result<SequenceContentChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives
//#region 🔖️TextPrimitives
fn print_sequence_snapshot_body(s: &SequenceSnapshot) -> String {
    format!("schema={}\ncontent={}", enc_str(&s.schema), enc_child(&s.content))
}
fn parse_sequence_snapshot_body(body: &str) -> Result<SequenceSnapshot, String> {
    let mut schema = None;
    let mut content = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("content=") {
            content = Some(dec_child(rest)?);
        } else {
            return Err(format!("sequence snapshot: unknown line {line:?}"));
        }
    }
    Ok(SequenceSnapshot {
        schema: schema.ok_or_else(|| "sequence snapshot: missing schema line".to_string())?,
        content: content.ok_or_else(|| "sequence snapshot: missing content line".to_string())?,
    })
}
//#endregion 🔖️TextPrimitives
//#region 🔖️ArtifactDslCodec
impl store::ArtifactDsl for SequenceSnapshot {
    const EXTENSION: &'static str = "sequence";
    fn envelope_id() -> &'static str {
        "sequence.sequence"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_sequence_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_sequence_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️ArtifactDslCodec


//#region 🔖️Example
/// 📄️ The handcrafted `.sequence` DSL-text fixture (regenerated from `default_snapshot()`'s canonical
/// print form) — the permanent proof that the checked-in fixture still parses and round trips, not a
/// one-time migration script.
pub const SEQUENCE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.sequence` DSL text into a `SequenceSnapshot`.
pub fn parse_dsl(text: &str) -> Result<SequenceSnapshot, store::TextError> {
    <SequenceSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `SequenceSnapshot` back to `.sequence` DSL text.
pub fn print_dsl(snapshot: &SequenceSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️Example

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_snapshot, SequenceStep, SlotRef, StepParams};

    #[test]
    fn dsl_round_trips_default_snapshot() {
        store::os_store::test_support::assert_dsl_round_trip(&default_snapshot());
    }

    #[test]
    fn default_sequence_example_dsl_round_trips() {
        let snapshot = parse_dsl(SEQUENCE_EXAMPLE_TEXT).expect("🎬️default.sequence must parse");
        store::os_store::test_support::assert_dsl_round_trip(&snapshot);
    }

    #[test]
    fn dsl_round_trips_snapshot_with_slots_and_nested_params() {
        use neural_engine::{Atom, Dictionary, Value};
        let mut fixture = default_snapshot().to_fixture();
        fixture.steps.push(SequenceStep {
            id: "step-3".into(),
            kind: "control.if".into(),
            params: StepParams::new().insert("flag", Value::Atom(Atom::Boolean(true))),
            x: 560.0,
            y: 0.0,
            slot: None,
            collapsed: true,
        });
        fixture.steps.push(SequenceStep {
            id: "step-4".into(),
            kind: "log.print".into(),
            params: StepParams::new()
                .insert("message", Value::Atom(Atom::String("nested \"quote\" and \\ backslash".into())))
                .insert("meta", Value::Dictionary(Dictionary::new().insert("count", Value::Atom(Atom::Integer(-3))).insert("ratio", Value::Atom(Atom::Decimal(2.5))))),
            x: 560.0,
            y: 160.0,
            slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
            collapsed: false,
        });
        let snapshot = SequenceSnapshot::from_fixture(fixture);
        store::os_store::test_support::assert_dsl_round_trip(&snapshot);
    }
}
//#endregion 🧪️Tests
