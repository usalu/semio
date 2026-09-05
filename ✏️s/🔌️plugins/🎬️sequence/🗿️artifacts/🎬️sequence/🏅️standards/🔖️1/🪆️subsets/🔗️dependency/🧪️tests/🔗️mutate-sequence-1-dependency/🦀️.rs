//! 🦀️ SEQUENCE dependency-vocabulary exhaustive mutation case — Rust adapter. Relocated out of the
//! artifact-level `mutate-sequence-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION` so
//! the edge vocabulary has a subset-owned test.
//!
//! Recorded no-oracle decision `sequence-step-graph-mutation-semantics`
//! (`../../../✳️any/🔮️oracle/🔣️.json`): `s.sequence.sequence` is a semio-NATIVE artifact with no
//! third-party reader or writer in any ecosystem, so this adapter registers NO oracle handler at
//! all. All evidence lives in the SUBJECT role below, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` module.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only
//! for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Vocabulary
/// 🏷️ This subset's own slice of `SequenceMutation::KINDS`
/// (`../../🧬️schema/🧬️mutations/🦀️.rs`) — duplicated rather than imported so the oracle-only
/// build never links the subject crate.
const KINDS: &[&str] = &["connect-steps", "disconnect-steps"];

/// 📄️ This subset's own local copy of the plugin's committed real sequence artifact.
const SEQUENCE_ASSET: &str = "local://🗣️.dsl.semio";

/// 🧫️ The three steps and one edge the composed content child is seeded with — see this fixture's
/// own `_provenance` member for which committed leaf payload each value came from.
const BASE_SCENE: &str = "local://🎬️base-scene.json";
//#endregion 🔖️Vocabulary

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{BASE_SCENE, SEQUENCE_ASSET};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_sequence::artifacts::sequence::dsl::parse_dsl;
    use semio_s_plugin_sequence::artifacts::sequence::mutations::{apply_sequence_mutation, decode_sequence_mutation_json, decode_sequence_scene_json, encode_sequence_projection_json, inverse_sequence_mutation, SequenceMutation};
    use semio_s_plugin_sequence::artifacts::sequence::{sequence_content_child_with_owner, SequenceSnapshot};
    use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, mutation_is_observable};

    //#region 🔖️CommittedInput
    fn base(ctx: &Context) -> Result<SequenceSnapshot, String> {
        let bytes = ctx.fixture_bytes(SEQUENCE_ASSET)?;
        let committed = String::from_utf8(bytes).map_err(|error| format!("the committed sequence artifact is not UTF-8: {error}"))?;
        let mut decoded = parse_dsl(&committed).map_err(|error| format!("the committed sequence artifact does not parse: {error:?}"))?;
        let scene = String::from_utf8(ctx.fixture_bytes(BASE_SCENE)?).map_err(|error| format!("the committed base scene is not UTF-8: {error}"))?;
        let (steps, edges) = decode_sequence_scene_json(&scene)?;
        if steps.is_empty() {
            return Err("the committed base scene declares no steps, so no id-keyed kind could address anything".into());
        }
        decoded.content = sequence_content_child_with_owner(steps, edges);
        Ok(decoded)
    }

    fn mutation(spec: &Json) -> Result<SequenceMutation, String> {
        let params = spec.get("params").ok_or_else(|| "the scenario doc string carries no params member".to_string())?;
        decode_sequence_mutation_json(&params.to_string())
    }

    /// ⚖️ The subset's OWN semantic projection, read back through production's
    /// `encode_sequence_projection_json` and reparsed with the platform's dependency-free reader.
    fn projection(snapshot: &SequenceSnapshot) -> Result<Json, String> {
        parse_json(&encode_sequence_projection_json(snapshot))
    }
    //#endregion 🔖️CommittedInput

    //#region 🔖️Handlers
    /// 👁️ The observability law: every declared kind must MOVE the very surface the scenario is
    /// compared through. The exemption list is EMPTY.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx)?;
        let mutated = apply_sequence_mutation(&base, &mutation(&spec)?).map_err(|error| format!("mutate-{kind}: the mutation did not apply: {error}"))?;
        let after = projection(&mutated)?;
        mutation_is_observable(&kind, &after, &projection(&base)?, &[])?;
        Ok(Outcome::with_raw(after.to_string().into_bytes(), after))
    }

    /// ↩️ The inverse law, asserted in role: applying the kind and then its OWN computed inverse must
    /// land back on the base document's projection exactly. The steps are replayed in REVERSE order.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx)?;
        let mutation = mutation(&spec)?;
        let mut current = apply_sequence_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: the forward mutation did not apply: {error}"))?;
        let mut undo = inverse_sequence_mutation(&base, &mutation);
        undo.reverse();
        for step in &undo {
            current = apply_sequence_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step did not apply: {error}"))?;
        }
        let restored = projection(&current)?;
        inverse_restores(&kind, &restored, &projection(&base)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's two `Examples` tables exactly.
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
    built
}
//#endregion 🔖️Registration
