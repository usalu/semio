//! 🦀️ SHOOTING exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`, wave 12 (the gate hole that exempted every vocabulary
//! with no catalog).
//!
//! Recorded no-oracle decision `shooting-render-scene-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`): `s.shooting.shooting` is a
//! semio-NATIVE artifact, and the 3D-scene interchange formats that come closest (glTF 2.0, USD,
//! Collada) model no SHOT at all — eleven of the thirty-one kinds address one — so this adapter
//! registers NO oracle handler. That is deliberate and it is the honest shape: a recorded no-oracle
//! case is never dispatched in the oracle role, and an oracle handler that could only re-read what
//! the subject just produced would be a stub reporting a pass. All evidence therefore lives in the
//! SUBJECT role below, where each handler asserts its law through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` module before it returns.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only
//! for the subject role (fleet brief §5.3).

use semio_repo_test_host::Adapter;

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `ShootingMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) — duplicated rather
/// than imported so the oracle-only build never links the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "create-asset",
    "delete-asset",
    "rename-asset",
    "change-asset-url",
    "reorder-assets",
    "drag-assets",
    "rotate-assets",
    "scale-assets",
    "create-shot",
    "delete-shot",
    "rename-shot",
    "change-shot-width",
    "change-shot-height",
    "change-shot-format",
    "change-shot-shape",
    "reorder-shots",
    "replace-shot-camera",
    "create-saved-camera",
    "delete-saved-camera",
    "rename-saved-camera",
    "replace-saved-camera-view",
    "reorder-saved-cameras",
    "set-active-shot",
    "set-active-asset",
    "change-scene-sun-enabled",
    "change-scene-sun-azimuth",
    "change-scene-sun-elevation",
    "change-scene-sun-intensity",
    "change-scene-ambient-intensity",
    "change-scene-shadow-enabled",
    "change-scene-material-roughness",
];

/// 📸️ The one before-snapshot all thirty-one of this vocabulary's committed leaf fixtures share, read
/// where the domain already keeps it rather than copied into a thirty-second place.
const BASE_SNAPSHOT: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-asset/🧪️tests/renames-asset-hero-to-lead/📸️snapshot/⬅️before/🔣️component.json";

/// 📄️ The plugin's own committed real DSL artifact — the only input that can carry evidence about
/// the handcrafted block/table text grammar.
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Vocabulary

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{BASE_SNAPSHOT, DSL_ASSET};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_shooting::artifacts::shooting::dsl::{parse_dsl, print_dsl};
    use semio_s_plugin_shooting::artifacts::shooting::mutations::{apply_shooting_mutation, decode_shooting_mutation_json, decode_shooting_snapshot_json, encode_shooting_projection_json, inverse_shooting_mutation, ShootingMutation};
    use semio_s_plugin_shooting::artifacts::shooting::ShootingSnapshot;
    use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, mutation_is_observable, round_trip_preserves};

    //#region 🔖️CommittedInput
    /// 📸️ The committed render scene, decoded by production's own reader. Nothing about it is
    /// authored here — it is the very document the thirty-one leaf fixtures were written against.
    fn base(ctx: &Context) -> Result<ShootingSnapshot, String> {
        let bytes = ctx.fixture_bytes(BASE_SNAPSHOT)?;
        let committed = String::from_utf8(bytes).map_err(|error| format!("the committed before-snapshot is not UTF-8: {error}"))?;
        decode_shooting_snapshot_json(&committed)
    }

    fn mutation(spec: &Json) -> Result<ShootingMutation, String> {
        let params = spec.get("params").ok_or_else(|| "the scenario doc string carries no params member".to_string())?;
        decode_shooting_mutation_json(&params.to_string())
    }

    /// ⚖️ The subset's OWN semantic projection, read back through production's
    /// `encode_shooting_projection_json` and reparsed with the platform's dependency-free reader.
    /// Comparison belongs to the owner, never to an adapter, so nothing about what this document
    /// means is decided in this file.
    fn projection(snapshot: &ShootingSnapshot) -> Result<Json, String> {
        parse_json(&encode_shooting_projection_json(snapshot))
    }
    //#endregion 🔖️CommittedInput

    //#region 🔖️Handlers
    /// 👁️ The observability law: every declared kind must MOVE the very surface the scenario is
    /// compared through. The exemption list is EMPTY — no kind of this vocabulary is allowed to pass
    /// without having been observed.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx)?;
        let mutated = apply_shooting_mutation(&base, &mutation(&spec)?).map_err(|error| format!("mutate-{kind}: the mutation did not apply: {error}"))?;
        let after = projection(&mutated)?;
        mutation_is_observable(&kind, &after, &projection(&base)?, &[])?;
        Ok(Outcome::with_raw(after.to_string().into_bytes(), after))
    }

    /// ↩️ The inverse law, asserted in role: applying the kind and then its OWN computed inverse must
    /// land back on the committed scene's projection exactly. The steps are replayed in the order the
    /// vocabulary emits them, which is this subset's own convention —
    /// `🧬️mutations/🦀️component.rs`'s `round_trip` test helper replays `backwards` forward, unlike its
    /// `present` and `sequence` siblings which reverse.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx)?;
        let mutation = mutation(&spec)?;
        let mut current = apply_shooting_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: the forward mutation did not apply: {error}"))?;
        for step in &inverse_shooting_mutation(&base, &mutation) {
            current = apply_shooting_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step did not apply: {error}"))?;
        }
        let restored = projection(&current)?;
        inverse_restores(&kind, &restored, &projection(&base)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }

    /// 🔁️ The identity law on the real committed DSL bytes. The carrier is deliberately byte-exact:
    /// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` is this codec's OWN output — a semio-native
    /// envelope no foreign writer ever produced — so reproducing it exactly is the correct answer and
    /// anything else is codec or fixture drift. That is why `carrier_is_exact` stands here in place
    /// of the wave's usual "output must differ from input" tripwire.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.fixture_bytes(DSL_ASSET)?;
        let committed = String::from_utf8(input.clone()).map_err(|error| format!("the committed shooting artifact is not UTF-8: {error}"))?;
        let decoded = parse_dsl(&committed).map_err(|error| format!("identity-round-trip: the committed shooting artifact does not parse: {error:?}"))?;
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
