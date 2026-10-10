//! 🖨️ BIM export case `export-bim-1-sheets-svg`, Rust adapter (subject role only). The lxml + shapely measurement lives in `🐍️.py` beside this file: it opens the committed SVG files of the room's
//! sheets and measures them. This adapter answers the same scenario from the subject: it infers the `sheet-layout` of the committed room and reports the table of the files from it and from the marks
//! the writer draws (paper, size, clip window and scale of every viewport, sorted text runs of the title block and the revision table).
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};

    /// 🖨️ The table of the SVG files of the committed room.
    pub fn export_sheets_svg_room(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = encode_inference_projection_json(&snapshot, "sheet-svg").ok_or_else(|| "sheet-svg is not a projection of s.bim.model".to_string())?;
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
        built = built.subject("export-sheets-svg-room", subject::export_sheets_svg_room);
    }
    built
}
//#endregion 🔖️Registration
