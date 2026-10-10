//! 🔋️ BIM export case `export-bim-1-energy`, Rust adapter (subject role only). The jsonschema + numpy + shapely measurement lives in `🐍️.py` beside this file: it opens the committed JSON export of the energy model
//! of a room, validates and audits it and measures the table again from its polygons. This adapter answers the same scenario from the subject: it infers the `energy-envelope` of the committed room and reports
//! the table of what the export keeps of it (the spaces with conditions, floor area, volume and surfaces merged by kind, boundary, neighbour and sector, and the totals of every scope).
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};

    /// 🔋️ The table of the energy export of the committed case.
    pub fn export_energy(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = encode_inference_projection_json(&snapshot, "energy-export").ok_or_else(|| "energy-export is not a projection of s.bim.model".to_string())?;
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
        built = built.subject("energy-export-room", subject::export_energy).subject("energy-export-pair", subject::export_energy).subject("energy-export-stack", subject::export_energy);
    }
    built
}
//#endregion 🔖️Registration
