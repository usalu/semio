//! 🌿️ BIM export case `export-bim-1-gbxml`, Rust adapter (subject role only). The lxml + shapely + numpy audit lives in `🐍️.py` beside this file: it opens the committed gbXML files, audits them
//! and measures them. This adapter answers the same scenarios from the subject: it infers the `energy-envelope` of the committed snapshot and reports the table of the document from the plan the writer builds
//! (counts, spaces, surfaces with type, adjacent spaces, orientation, size, polygon area and construction, constructions, window types and the area per surface type).
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::gbxml::export_table;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

    /// 🌿️ The table of the gbXML document of the committed snapshot.
    pub fn export_gbxml(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = export_table(&snapshot)?;
        let projection = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls; the ids are the feature's `@id-*` tags.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for id in ["export-gbxml-box", "export-gbxml-zoning", "export-gbxml-stack", "export-gbxml-house"] {
            built = built.subject(id, subject::export_gbxml);
        }
    }
    built
}
//#endregion 🔖️Registration
