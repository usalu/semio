//! 🪜️ BIM inference case `infer-bim-1-levels-and-wall-heights`, Rust adapter (subject role only). The third-party reproduction lives in
//! `🐍️.py` beside this file; this adapter answers the same two scenarios from `ModelInference` of the very same committed snapshot, and the
//! platform compares both projections under `floating-point-v1`. It registers no oracle handler: a subject that re-read the committed
//! expectation would be a self-comparison reporting a pass.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

/// 📸️ The committed oracle house, the only input of both scenarios.
const SNAPSHOT: &str = "shared://💡️inferences/🏠️house/📸️snapshot/🔣️.json";

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::SNAPSHOT;
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};

    fn infer(ctx: &Context, slug: &str) -> Result<Outcome, String> {
        let bytes = ctx.input_bytes(SNAPSHOT)?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = encode_inference_projection_json(&snapshot, slug).ok_or_else(|| format!("{slug} is not an inference of s.bim.model"))?;
        let projection = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), projection))
    }

    /// 🪜️ The storey elevation table.
    pub fn storey_levels(ctx: &Context) -> Result<Outcome, String> {
        infer(ctx, "storey-levels")
    }

    /// 🧱️ The wall layout table.
    pub fn wall_layout(ctx: &Context) -> Result<Outcome, String> {
        infer(ctx, "wall-layout")
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
        built = built.subject("storey-levels", subject::storey_levels).subject("wall-layout", subject::wall_layout);
    }
    built
}
//#endregion 🔖️Registration
