//! 🎭️ BIM inference case `infer-bim-1-phases`, Rust adapter (subject role only). The third-party reproduction lives in `🐍️.py` beside this file; this adapter answers the same
//! scenario from `ModelInference` of the committed house snapshot, and the platform compares both projections under `floating-point-v1`. It registers no oracle handler: a subject
//! that re-read the exported file would be a self-comparison reporting a pass.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};

    /// 🎭️ The visibility table of the committed house.
    pub fn house(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = encode_inference_projection_json(&snapshot, "phase-visibility").ok_or_else(|| "phase-visibility is not an inference of s.bim.model".to_string())?;
        let projection = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), projection))
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
        built = built.subject("phases-house", subject::house);
    }
    built
}
//#endregion 🔖️Registration
