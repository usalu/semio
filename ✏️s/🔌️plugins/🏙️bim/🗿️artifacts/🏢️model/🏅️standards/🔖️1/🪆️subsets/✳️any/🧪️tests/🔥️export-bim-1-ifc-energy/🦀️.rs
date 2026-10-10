//! 🔥️ BIM export case `export-bim-1-ifc-energy`, Rust adapter (subject role only). The IfcOpenShell audit lives in `🐍️.py` beside this file: it opens the committed IFC 2x3 and IFC4 files of the house, checks the thermal property
//! sets against the standard templates and restates the rules from the snapshot. This adapter answers the same scenarios from the subject: it exports the committed snapshot in the schema the scenario names and reports the thermal
//! table of the written document (the thermal sets of every product with their IFC types and values, the `DerivedRows` and `Conditions` rows, the thermal rows of the window and door types).
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::ifc::{energy::thermal_report, Schema};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

    fn table(ctx: &Context, schema: Schema) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = thermal_report(schema, &snapshot)?;
        let projection = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), projection))
    }

    /// 🔥️ The thermal table of the IFC 2x3 export.
    pub fn export_ifc_energy_2x3(ctx: &Context) -> Result<Outcome, String> {
        table(ctx, Schema::Ifc2x3)
    }

    /// 🔥️ The thermal table of the IFC4 export.
    pub fn export_ifc_energy_4(ctx: &Context) -> Result<Outcome, String> {
        table(ctx, Schema::Ifc4)
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
        built = built.subject("export-ifc-energy-2x3", subject::export_ifc_energy_2x3).subject("export-ifc-energy-4", subject::export_ifc_energy_4);
    }
    built
}
//#endregion 🔖️Registration
