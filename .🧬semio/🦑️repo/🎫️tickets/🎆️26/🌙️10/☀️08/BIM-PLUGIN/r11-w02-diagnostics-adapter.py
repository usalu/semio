import sys
d = sys.argv[1]
def patch(path, pairs):
    s = open(path, encoding='utf-8', newline='').read()
    for old, new in pairs:
        assert s.count(old) == 1, (path, old, s.count(old))
        s = s.replace(old, new)
    open(path + ".new", 'w', encoding='utf-8', newline='').write(s)
patch(d + "/🦀️.rs", [
('''    /// ⚠️ The adjudicated findings of the model.
    pub fn diagnostics(ctx: &Context) -> Result<Outcome, String> {
        infer(ctx, "diagnostics")
    }
''', '''    /// ⚠️ The adjudicated findings of the model.
    pub fn diagnostics(ctx: &Context) -> Result<Outcome, String> {
        infer(ctx, "diagnostics")
    }

    fn snapshot_of(ctx: &Context) -> Result<semio_s_artifact_bim_model::ModelSnapshot, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)
    }

    /// 🧾️ The adjudicated findings read back from the JSON export of the model.
    pub fn export_json(ctx: &Context) -> Result<Outcome, String> {
        let text = json::export_diagnostics(&snapshot_of(ctx)?).map_err(|error| error.to_string())?;
        let table = json::adjudicated_json(&text)?;
        let parsed = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), parsed))
    }

    /// 📊️ Every finding read back from the CSV export of the model.
    pub fn export_csv(ctx: &Context) -> Result<Outcome, String> {
        let text = csv::export_diagnostics(&snapshot_of(ctx)?).map_err(|error| error.to_string())?;
        let table = csv::findings_json(&text);
        let parsed = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), parsed))
    }
'''),
('''    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};
''', '''    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::{csv, json};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};
'''),
('''.subject("diagnostics-defects", subject::diagnostics);''', '''.subject("diagnostics-defects", subject::diagnostics).subject("diagnostics-export-json", subject::export_json).subject("diagnostics-export-csv", subject::export_csv);'''),
('''//! beside this file; this adapter answers the same four scenarios from''', '''//! beside this file; this adapter answers the same scenarios (the plan metrics and findings, and the JSON and CSV exports of the findings) from'''),
])
