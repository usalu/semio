//! 🏗️ BIM export case `export-bim-1-ifc4`, Rust adapter (subject role only). The IfcOpenShell measurement lives in `🐍️.py` beside this file: it opens the committed IFC4 file, validates it and counts, locates and
//! tessellates what it finds. This adapter answers the same scenarios from the subject: it exports the committed model as IFC4 and reports the same table from its own Part-21 document and base quantities (class
//! counts, containment per storey, net volume of every exactly measurable element, annotations, classification tables, type property sets); the round-trip scenarios report the file the subject writes after importing
//! its own export, and the stepped scenario the file of a job that was cancelled once.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::editor::bim::commands::export_model::{Advance, ExportJob};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::ifc::projection::{projection_in, report};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::ifc::{codec, export_ifc4, Schema};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::import::ifc::import_ifc4;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
    use semio_s_artifact_bim_model::ModelSnapshot;

    fn snapshot(ctx: &Context) -> Result<ModelSnapshot, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let bytes = ctx.input_bytes(&uri)?;
        decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)
    }

    fn outcome(table: String) -> Result<Outcome, String> {
        let parsed = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), parsed))
    }

    /// 🏗️ `{schema, counts, containment, volumes, annotations, classifications, type_properties}` of the IFC4 export of the committed model.
    pub fn export(ctx: &Context) -> Result<Outcome, String> {
        outcome(projection_in(Schema::Ifc4, &snapshot(ctx)?).to_json())
    }

    /// 🔁️ The same report for the file written after the subject imported its own export.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let model = snapshot(ctx)?;
        let (first, _) = export_ifc4(&model)?;
        let (back, _) = import_ifc4(&first)?;
        let (second, _) = export_ifc4(&back)?;
        outcome(report(&back, &codec::decode_ifc4(&second)?).to_json())
    }

    /// ⏳️ The house report of the file the stepped IFC4 export job writes: a first job is cancelled half-way, a second one runs to its end on the same session and its file is read back.
    pub fn stepped(ctx: &Context) -> Result<Outcome, String> {
        let model = snapshot(ctx)?;
        let mut cancelled = ExportJob::with_steps("ifc4", Some(43), 2);
        for _ in 0..2 {
            cancelled.advance(&model).map_err(|fault| format!("{fault:?}"))?;
        }
        cancelled.cancel(&model);
        let mut job = ExportJob::with_steps("ifc4", Some(43), 64);
        let output = loop {
            if let Advance::Done(output) = job.advance(&model).map_err(|fault| format!("{fault:?}"))? {
                break output;
            }
        };
        outcome(report(&model, &codec::decode_ifc4(output.data.as_bytes())?).to_json())
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
        for id in ["export-ifc4-house", "export-ifc4-psets", "export-ifc4-ceilings", "export-ifc4-notated", "export-ifc4-ramps", "export-ifc4-wall-depth"] {
            built = built.subject(id, subject::export);
        }
        built = built.subject("roundtrip-ifc4-house", subject::round_trip).subject("roundtrip-ifc4-psets", subject::round_trip).subject("stepped-ifc4-house", subject::stepped);
    }
    built
}
//#endregion 🔖️Registration
