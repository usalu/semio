//! 🦀️ Animate PRESENT exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`, wave 12 (the gate hole that exempted every vocabulary
//! with no catalog).
//!
//! Recorded no-oracle decision `present-figure-deck-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`): `s.animate.present` is a
//! semio-NATIVE artifact with no third-party reader or writer in any ecosystem, so this adapter
//! registers NO oracle handler at all. That is deliberate and it is the honest shape: a recorded
//! no-oracle case is never dispatched in the oracle role, and an oracle handler that could only
//! re-read what the subject just produced would be a stub reporting a pass. All evidence therefore
//! lives in the SUBJECT role below, where each handler asserts its law through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` module before it returns.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only
//! for the subject role (fleet brief §5.3).

use semio_repo_test_host::Adapter;

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `PresentMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) — duplicated rather
/// than imported so the oracle-only build never links the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &["resize-source-frame", "replace-source", "create-tile", "delete-tile", "delete-tiles", "rename-tile", "resize-tile-crop", "reorder-tiles", "replace-tiles"];

/// 📄️ The plugin's own committed real deck artifact, read where the domain already keeps it.
const DECK_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";

/// 🧫️ The three tiles the composed presentation child is seeded with — see this fixture's own
/// `_provenance` member for which committed leaf payload each one came from.
const BASE_TILES: &str = "local://🎞️base-tiles.json";
//#endregion 🔖️Vocabulary

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{BASE_TILES, DECK_ASSET};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_animate::artifacts::present::dsl::{parse_dsl, print_dsl};
    use semio_s_plugin_animate::artifacts::present::mutations::{apply_present_mutation, decode_present_mutation_json, encode_present_projection_json, inverse_present_mutation, PresentMutation};
    use semio_s_plugin_animate::artifacts::present::{present_snapshot_with_tiles, present_working_scene, FigureTileDraft, FigureTileFrame, PresentSnapshot};
    use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, mutation_is_observable, round_trip_preserves};

    //#region 🔖️CommittedInput
    fn number(value: &Json, key: &str) -> Result<f64, String> {
        match value.get(key) {
            Some(Json::Number(found)) => Ok(*found),
            _ => Err(format!("the committed base scene carries no numeric {key:?}")),
        }
    }

    fn text(value: &Json, key: &str) -> Result<String, String> {
        match value.get(key) {
            Some(Json::String(found)) => Ok(found.clone()),
            _ => Err(format!("the committed base scene carries no string {key:?}")),
        }
    }

    fn frame(value: &Json) -> Result<FigureTileFrame, String> {
        Ok(FigureTileFrame { x: number(value, "x")?, y: number(value, "y")?, width: number(value, "width")?, height: number(value, "height")? })
    }

    fn committed_tiles(ctx: &Context) -> Result<Vec<FigureTileDraft>, String> {
        let fixture = ctx.fixture_json(BASE_TILES)?;
        let mut tiles = Vec::new();
        for entry in fixture.array("tiles") {
            let crop = entry.get("crop").ok_or_else(|| "a committed base tile carries no crop".to_string())?;
            tiles.push(FigureTileDraft { id: text(&entry, "id")?, name: text(&entry, "name")?, crop: frame(crop)? });
        }
        if tiles.is_empty() {
            return Err("the committed base scene declares no tiles, so no id-keyed kind could address anything".into());
        }
        Ok(tiles)
    }

    /// 📄️ The real committed artifact, decoded by production's own reader, with its composed
    /// presentation child re-minted around the committed base tiles. The SOURCE is never a literal
    /// here: it is whatever the committed artifact itself decodes to.
    fn base(ctx: &Context) -> Result<PresentSnapshot, String> {
        let bytes = ctx.fixture_bytes(DECK_ASSET)?;
        let committed = String::from_utf8(bytes).map_err(|error| format!("the committed deck artifact is not UTF-8: {error}"))?;
        let decoded = parse_dsl(&committed).map_err(|error| format!("the committed deck artifact does not parse: {error:?}"))?;
        let (source, _) = present_working_scene(&decoded);
        Ok(present_snapshot_with_tiles(&source, &committed_tiles(ctx)?))
    }

    fn mutation(spec: &Json) -> Result<PresentMutation, String> {
        let params = spec.get("params").ok_or_else(|| "the scenario doc string carries no params member".to_string())?;
        decode_present_mutation_json(&params.to_string())
    }
    //#endregion 🔖️CommittedInput

    //#region 🔖️Projection
    /// ⚖️ The subset's OWN semantic projection, read back through production's
    /// `encode_present_projection_json` and reparsed with the platform's dependency-free reader.
    /// Comparison belongs to the owner, never to an adapter, so nothing about what this document
    /// means is decided in this file.
    fn projection(snapshot: &PresentSnapshot) -> Result<Json, String> {
        parse_json(&encode_present_projection_json(snapshot))
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 👁️ The observability law: every declared kind must MOVE the very surface the scenario is
    /// compared through. The exemption list is EMPTY — no kind of this vocabulary is allowed to pass
    /// without having been observed.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx)?;
        let mutated = apply_present_mutation(&base, &mutation(&spec)?).map_err(|error| format!("mutate-{kind}: the mutation did not apply: {error}"))?;
        let after = projection(&mutated)?;
        mutation_is_observable(&kind, &after, &projection(&base)?, &[])?;
        Ok(Outcome::with_raw(after.to_string().into_bytes(), after))
    }

    /// ↩️ The inverse law, asserted in role: applying the kind and then its OWN computed inverse
    /// must land back on the base document's projection exactly. The steps are replayed in REVERSE
    /// order, which is this subset's own convention — `🧬️mutations/🦀️component.rs`'s `round_trip`
    /// test helper reverses before replaying.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx)?;
        let mutation = mutation(&spec)?;
        let mut current = apply_present_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: the forward mutation did not apply: {error}"))?;
        let mut undo = inverse_present_mutation(&base, &mutation);
        undo.reverse();
        for step in &undo {
            current = apply_present_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step did not apply: {error}"))?;
        }
        let restored = projection(&current)?;
        inverse_restores(&kind, &restored, &projection(&base)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }

    /// 🔁️ The identity law on the real committed bytes. The carrier is deliberately byte-exact:
    /// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` is this codec's OWN output — a semio-native
    /// envelope no foreign writer ever produced — so reproducing it exactly is the correct answer
    /// and anything else is codec or fixture drift. That is why `carrier_is_exact` stands here in
    /// place of the wave's usual "output must differ from input" tripwire.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.fixture_bytes(DECK_ASSET)?;
        let committed = String::from_utf8(input.clone()).map_err(|error| format!("the committed deck artifact is not UTF-8: {error}"))?;
        let decoded = parse_dsl(&committed).map_err(|error| format!("identity-round-trip: the committed deck artifact does not parse: {error:?}"))?;
        let printed = print_dsl(&decoded);
        carrier_is_exact(printed.as_bytes(), &input)?;
        let reparsed = parse_dsl(&printed).map_err(|error| format!("identity-round-trip: this codec's own output does not parse back: {error:?}"))?;
        let after = projection(&reparsed)?;
        round_trip_preserves(&after, &projection(&decoded)?)?;
        Ok(Outcome::with_raw(printed.into_bytes(), after))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        let _ = kind;
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
