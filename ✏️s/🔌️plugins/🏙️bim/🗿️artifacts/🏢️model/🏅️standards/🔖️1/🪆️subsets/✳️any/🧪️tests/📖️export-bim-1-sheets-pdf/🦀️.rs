//! 📖️ BIM export case `export-bim-1-sheets-pdf`, Rust adapter (subject role only). The pypdf measurement lives in `🐍️.py` beside this file: it opens the committed PDF of the sheet set of the room and reads its
//! pages. This adapter answers the same scenario from the subject: it infers the `sheet-layout` of the committed room and reports the table of the file from it (one page per sheet in print order, its size in
//! millimetres to a tenth, and that its text shows the number and the title of the sheet).
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};

    /// 🖨️ The table of the PDF of the committed room.
    pub fn export_sheets_pdf_room(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = encode_inference_projection_json(&snapshot, "sheet-pdf").ok_or_else(|| "sheet-pdf is not a projection of s.bim.model".to_string())?;
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
        built = built.subject("export-sheets-pdf-room", subject::export_sheets_pdf_room);
    }
    built
}
//#endregion 🔖️Registration
