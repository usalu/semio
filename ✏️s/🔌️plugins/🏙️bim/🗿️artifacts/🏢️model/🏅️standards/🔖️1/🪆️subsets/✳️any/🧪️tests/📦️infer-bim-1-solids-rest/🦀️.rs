//! 🧊️ BIM inference case `infer-bim-1-solids-rest`, Rust adapter (subject role only). The shapely reproduction lives in `🐍️.py` beside this
//! file: it re-derives volume and bounds of every planar column, beam, slab, roof, stair and railing from the authored snapshot of each
//! case. This adapter answers the same scenario from the subject's own `ModelInference`: `{case: {element id: {volume, min, max}}}`.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

/// 🗂️ The fixture cases of these families.
#[allow(dead_code)]
const CASES: [&str; 9] = ["columns-profiles", "beams-profiles", "frame-tilt-joins", "slabs-holes-slope", "ceilings-holes-slope", "ceilings-meshes", "roofs-shapes", "stairs-flights", "railings-posts"];

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::CASES;
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};

    /// 🧊️ `{case: {element id: {volume, min, max}}}` for every planar element of every case the scenario names.
    pub fn solids_rest(ctx: &Context) -> Result<Outcome, String> {
        let uris = ctx.step_input_uris();
        let mut tables = Vec::new();
        for case in CASES {
            let Some(uri) = uris.iter().find(|uri| uri.trim_end_matches('/').rsplit('/').nth(1).is_some_and(|folder| folder.ends_with(case))) else { continue };
            let text = String::from_utf8(ctx.input_bytes(uri)?).map_err(|error| format!("the committed case {case} is not UTF-8: {error}"))?;
            let document = parse_json(&text)?;
            let snapshot = decode_model_snapshot_json(&document.get("snapshot").ok_or_else(|| format!("the case {case} has no snapshot"))?.to_string())?;
            let table = encode_inference_projection_json(&snapshot, "frame-solids").ok_or_else(|| "frame-solids is not an inference of s.bim.model".to_string())?;
            tables.push(format!("\"{case}\":{table}"));
        }
        let table = format!("{{{}}}", tables.join(","));
        let projection = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls; the id is the feature's `@id-*` tag.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("solids-rest", subject::solids_rest);
    }
    built
}
//#endregion 🔖️Registration
