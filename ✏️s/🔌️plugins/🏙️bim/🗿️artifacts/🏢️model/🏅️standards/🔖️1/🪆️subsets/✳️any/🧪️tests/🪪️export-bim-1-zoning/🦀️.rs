//! 🏘️ BIM export case `export-bim-1-zoning`, Rust adapter (subject role only). The IfcOpenShell reading lives in `🐍️.py` beside this file: it opens the committed IFC file and reads every zone, area scheme and covering.
//! This adapter answers the same scenario from the subject: the report of the committed zoned house built from the snapshot and its inference.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::ifc::zoning::report_json;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

    /// 🏘️ `{zones, schemes, coverings}` of the committed zoned house.
    pub fn export_zoning_zoned(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = report_json(&snapshot)?;
        let parsed = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), parsed))
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
        built = built.subject("export-zoning-zoned", subject::export_zoning_zoned);
    }
    built
}
//#endregion 🔖️Registration
