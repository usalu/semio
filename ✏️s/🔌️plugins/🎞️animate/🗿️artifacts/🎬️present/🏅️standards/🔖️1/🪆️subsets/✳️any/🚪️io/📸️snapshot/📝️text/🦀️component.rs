//! 🗣️ Animate present artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::present::{PresentSnapshot, PRESENT_DOCUMENT_SCHEMA};

/// 📄️ The handcrafted `.present` DSL-text fixture — a multi-tile deck exercising every field
/// (including the optional `source-aspect`), embedded at compile time as the permanent proof that
/// the checked-in fixture still parses and round trips.
pub const PRESENT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.present` DSL text into a `PresentSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PresentSnapshot, store::TextError> {
    <PresentSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PresentSnapshot` back to `.present` DSL text.
pub fn print_dsl(deck: &PresentSnapshot) -> String {
    store::ArtifactDsl::print_dsl(deck)
}

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors writer's/cad's own `enc_child`/`dec_child`) — a
/// handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened via `to_uri()`),
/// never the child's own content. Text-only (the binary facet uses its own LEB128-length-prefixed
/// scheme, see `../💾️binary/🦀️component.rs`).
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
fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
fn print_present_snapshot_body(s: &PresentSnapshot) -> String {
    format!("schema={}\npresentation={}\nanimation={}", enc_str(&s.schema), enc_child(&s.presentation), enc_child(&s.animation))
}
fn parse_present_snapshot_body(body: &str) -> Result<PresentSnapshot, String> {
    let mut schema = None;
    let mut presentation = None;
    let mut animation = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("presentation=") {
            presentation = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("animation=") {
            animation = Some(dec_child(rest)?);
        } else {
            return Err(format!("present snapshot: unknown line {line:?}"));
        }
    }
    Ok(PresentSnapshot {
        schema: schema.ok_or_else(|| "present snapshot: missing schema line".to_string())?,
        presentation: presentation.ok_or_else(|| "present snapshot: missing presentation line".to_string())?,
        animation: animation.ok_or_else(|| "present snapshot: missing animation line".to_string())?,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️HandcraftedArtifactDsl
/// 🚪️ Moved from `../../../🧬️schema/📸️snapshot/🦀️component.rs` per design.md's CORRECTION — the
/// native codec sits directly under `🚪️io/<facet>/<representation>/`, unsplit (one bidirectional
/// trait impl, not an import/export mirror).
impl store::ArtifactDsl for PresentSnapshot {
    const EXTENSION: &'static str = "present";
    fn envelope_id() -> &'static str {
        PRESENT_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_present_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_present_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::default_present_snapshot;
    use crate::artifacts::present::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
    use store::os_store::test_support;

    #[test]
    fn dsl_round_trip_default_present_snapshot() {
        test_support::assert_dsl_round_trip(&default_present_snapshot());
        test_support::assert_dsl_pack_equivalence(&default_present_snapshot());
    }

    #[test]
    fn dsl_round_trip_present_deck_with_tiles() {
        let deck = default_present_snapshot();
        let (source, _) = crate::artifacts::present::present_working_scene(&deck);
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let deck = crate::artifacts::present::present_snapshot_with_tiles(&source, &tiles);
        test_support::assert_dsl_round_trip(&deck);
        test_support::assert_dsl_pack_equivalence(&deck);
    }

    #[test]
    fn present_dsl_round_trips_bundled_default_example() {
        let deck = parse_dsl(PRESENT_EXAMPLE_TEXT).expect("🎞️default.present must parse");
        test_support::assert_dsl_round_trip(&deck);
        test_support::assert_dsl_pack_equivalence(&deck);
    }
}
//#endregion 🧪️Tests
