//! 💬️ The subject reports authored BIM coordination issues and enforces byte parity with their BCF export.

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
fn frame(ctx: &semio_repo_test_host::Context) -> Result<semio_repo_test_host::Outcome, String> {
    use semio_repo_test_host::{parse_json, Outcome};
    use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::bcf::{export_bcf, table_json};
    use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

    let source = String::from_utf8(ctx.input_bytes("shared://🚪️bcf/🏢️frame/model.json")?).map_err(|error| error.to_string())?;
    let model = decode_model_snapshot_json(&source)?;
    if export_bcf(&model)? != ctx.input_bytes("shared://🚪️bcf/🏢️frame/issues.bcf")? {
        return Err("The frame export differs from the committed BCF container".into());
    }
    let table = table_json(&model);
    let result = parse_json(&table)?;
    Ok(Outcome::with_raw(table.into_bytes(), result))
}

/// 🧭️ Registers the BIM frame subject.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("export-bcf-frame", frame);
    adapter
}
