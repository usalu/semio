//! 🧮️ BIM inference case `infer-bim-1-quantities`, Rust adapter (subject role only). The third-party reproduction lives in `🐍️.py` beside this
//! file; this adapter answers the same scenario from `ModelInference` of the very same committed snapshot, and the platform compares both
//! projections under `floating-point-v1`. It registers no oracle handler: a subject that re-read the committed expectation would be a
//! self-comparison reporting a pass.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::mutations::apply_model_mutation;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::mutations::decode_model_mutation_json;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};
    use semio_s_artifact_bim_model::ModelSnapshot;

    fn snapshot(ctx: &Context) -> Result<ModelSnapshot, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)
    }

    fn table(snapshot: &ModelSnapshot) -> Result<Outcome, String> {
        let table = encode_inference_projection_json(snapshot, "quantities").ok_or_else(|| "quantities is not an inference of s.bim.model".to_string())?;
        let projection = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), projection))
    }

    /// 🧮️ The take-off table of the building model.
    pub fn building(ctx: &Context) -> Result<Outcome, String> {
        table(&snapshot(ctx)?)
    }

    /// 🛗️ The take-off table of the model after its committed `set-element-storey` mutation was applied by the central applier.
    pub fn storey_move(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("🦠️mutation")).ok_or_else(|| "the scenario names no mutation".to_string())?;
        let mutation = decode_model_mutation_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed mutation is not UTF-8: {error}"))?)?;
        let moved = apply_model_mutation(&snapshot(ctx)?, &mutation).map_err(|error| format!("the storey move did not apply: {error}"))?;
        table(&moved)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls; ids are the feature's `@id-*` tags.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("quantities-building", subject::building).subject("quantities-storey-move", subject::storey_move);
    }
    built
}
//#endregion 🔖️Registration
