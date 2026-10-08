//! 🗺️ BIM inference case `infer-bim-1-plan-and-diagnostics`, Rust adapter (subject role only). The third-party reproduction lives in `🐍️.py`
//! beside this file; this adapter answers the same four scenarios from `ModelInference` of the very same committed snapshots, and the
//! platform compares both projections under `floating-point-v1`. It registers no oracle handler: a subject that re-read the committed
//! expectation would be a self-comparison reporting a pass.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};

    fn infer(ctx: &Context, slug: &str) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let bytes = ctx.input_bytes(&uri)?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = encode_inference_projection_json(&snapshot, slug).ok_or_else(|| format!("{slug} is not an inference of s.bim.model"))?;
        let projection = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), projection))
    }

    /// 🗺️ The plan measures of every storey.
    pub fn plan_metrics(ctx: &Context) -> Result<Outcome, String> {
        infer(ctx, "plan-metrics")
    }

    /// ⚠️ The adjudicated findings of the model.
    pub fn diagnostics(ctx: &Context) -> Result<Outcome, String> {
        infer(ctx, "diagnostics")
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
        built = built.subject("plan-metrics-house", subject::plan_metrics).subject("plan-metrics-curved", subject::plan_metrics).subject("diagnostics-clean", subject::diagnostics).subject("diagnostics-defects", subject::diagnostics);
    }
    built
}
//#endregion 🔖️Registration
