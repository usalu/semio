//! 🦀️ BIM model exhaustive mutation case, Rust adapter. The subject role asserts its law through the shared
//! `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law/🦀️.rs` module before it returns. The subject half is `sut`-gated because the generated host
//! links this repository's crate only for the subject role. No oracle handler is registered: a third-party reproduction of the
//! mutation semantics is registered by the oracle wave, never by an adapter that re-reads what the subject just produced.

use semio_repo_test_host::Adapter;

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `KINDS` of `ModelMutation` (`../../🧬️schema/🧬️mutations/🦀️.rs`) so the oracle-only build never links the subject crate.
const KINDS: &[&str] = &[
    "set-curtain-wall",
    "delete-curtain-wall",
    "create-curtain-wall",
    "split-wall",
    "flip-wall",
    "set-wall-location",
    "set-wall-type-of",
    "set-wall-base-offset",
    "set-wall-axis",
    "create-site",
    "delete-site",
    "create-building",
    "delete-building",
    "create-storey",
    "rename-storey",
    "set-storey-height",
    "set-storey-cut-height",
    "set-element-phase",
    "set-element-storey",
    "set-storey-level",
    "delete-storey",
    "create-wall",
    "delete-wall",
    "set-wall-top",
    "set-project-info",
    "set-site",
    "set-building",
    "create-grid-line",
    "delete-grid-line",
    "set-grid-line",
    "create-column-type",
    "delete-column-type",
    "set-column-type",
    "create-beam-type",
    "delete-beam-type",
    "set-beam-type",
    "create-window-type",
    "delete-window-type",
    "set-window-type",
    "create-door-type",
    "delete-door-type",
    "set-door-type",
    "create-material",
    "delete-material",
    "set-material",
    "create-wall-type",
    "delete-wall-type",
    "set-wall-type",
    "create-slab-type",
    "delete-slab-type",
    "set-slab-type",
    "create-roof-type",
    "delete-roof-type",
    "set-roof-type",
    "create-column",
    "delete-column",
    "set-column",
    "create-beam",
    "delete-beam",
    "set-beam",
    "create-railing",
    "delete-railing",
    "set-railing",
    "create-ramp",
    "set-ramp",
    "delete-ramp",
    "create-space",
    "delete-space",
    "set-space",
    "create-zone",
    "set-zone",
    "delete-zone",
    "create-area-scheme",
    "set-area-scheme",
    "delete-area-scheme",
    "create-slab",
    "delete-slab",
    "set-slab-boundary",
    "set-slab",
    "create-ceiling-type",
    "delete-ceiling-type",
    "set-ceiling-type",
    "create-ceiling",
    "create-dimension",
    "delete-dimension",
    "set-dimension",
    "create-tag",
    "delete-tag",
    "set-tag",
    "create-text-note",
    "delete-text-note",
    "set-text-note",
    "create-leader",
    "delete-leader",
    "set-leader",
    "create-annotation-style",
    "delete-annotation-style",
    "set-annotation-style",
    "delete-ceiling",
    "set-ceiling-boundary",
    "set-ceiling",
    "create-roof",
    "delete-roof",
    "set-roof-footprint",
    "set-roof-shape",
    "create-opening",
    "delete-opening",
    "move-opening",
    "set-opening",
    "create-stair",
    "delete-stair",
    "set-stair",
    "move-elements",
    "rotate-elements",
    "place-elements",
    "delete-elements",
    "rename-element",
    "set-element-property",
    "remove-element-property",
    "set-element-classification",
    "remove-element-classification",
    "create-schedule",
    "set-schedule",
    "delete-schedule",
    "create-view",
    "set-view",
    "delete-view",
    "set-wall-end-join",
    "copy-elements",
    "mirror-elements",
    "array-elements",
    "align-elements",
    "offset-wall",
    "trim-extend-wall",
    "split-slab",
    "split-beam",
];

/// 📄️ The plugin's own committed real DSL artifact: the only input that can carry evidence about the text grammar.
const DSL_ASSET: &str = "asset://🎬️demo/🗣️.dsl.semio";
//#endregion 🔖️Vocabulary

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::DSL_ASSET;
    use semio_repo_test_host::law::{carrier_is_exact, inverse_restores, mutation_is_observable, round_trip_preserves};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_bim_model::mutations::{apply_model_mutation, inverse_model_mutation, ModelMutation};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::mutations::decode_model_mutation_json;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_model_projection_json, parse_dsl, print_dsl};
    use semio_s_artifact_bim_model::ModelSnapshot;

    fn base(ctx: &Context, spec: &Json) -> Result<ModelSnapshot, String> {
        let bytes = ctx.input_bytes(&spec.str("before"))?;
        decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed before-snapshot is not UTF-8: {error}"))?)
    }

    fn mutation(spec: &Json) -> Result<ModelMutation, String> {
        let params = spec.get("params").ok_or_else(|| "the scenario doc string carries no params member".to_string())?;
        decode_model_mutation_json(&params.to_string())
    }

    fn projection(snapshot: &ModelSnapshot) -> Result<Json, String> {
        parse_json(&encode_model_projection_json(snapshot))
    }

    //#region 🔖️Handlers
    /// 👁️ Every declared kind must MOVE the surface the scenario is compared through; the exemption list is empty.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx, &spec)?;
        let mutated = apply_model_mutation(&base, &mutation(&spec)?).map_err(|error| format!("mutate-{kind}: the mutation did not apply: {error}"))?;
        let after = projection(&mutated)?;
        mutation_is_observable(&kind, &after, &projection(&base)?, &[])?;
        Ok(Outcome::with_raw(after.to_string().into_bytes(), after))
    }

    /// ↩️ Applying the kind and then its OWN computed inverse (replayed storage-reversed, as the store does) lands back on the base.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx, &spec)?;
        let mutation = mutation(&spec)?;
        let mut current = apply_model_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: the forward mutation did not apply: {error}"))?;
        for step in inverse_model_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: {error}"))?.iter().rev() {
            current = apply_model_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step did not apply: {error}"))?;
        }
        let restored = projection(&current)?;
        inverse_restores(&kind, &restored, &projection(&base)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }

    /// 🔁️ The identity law on the real committed DSL bytes; the carrier is this codec's own output, so it must be reproduced exactly.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.input_bytes(DSL_ASSET)?;
        let committed = String::from_utf8(input.clone()).map_err(|error| format!("the committed BIM artifact is not UTF-8: {error}"))?;
        let decoded = parse_dsl(&committed).map_err(|error| format!("identity-round-trip: the committed BIM artifact does not parse: {error:?}"))?;
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
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario id, mirroring the `Examples` tables.
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
