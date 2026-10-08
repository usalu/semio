//! 🏗️ BIM export case `export-bim-1-ifc`, Rust adapter (subject role only). The IfcOpenShell measurement lives in `🐍️.py` beside this file: it opens the committed IFC file and counts,
//! locates and tessellates what it finds. This adapter answers the same scenario from the subject: it exports the committed house and reports the same table from its own Part-21 document
//! and base quantities (class counts, containment per storey, net volume of every exactly measurable element).
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

/// 📸️ The committed house model.
const SNAPSHOT: &str = "shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json";

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::SNAPSHOT;
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::ifc::projection::projection;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

    /// 🏗️ `{schema, counts, containment, volumes}` of the export of the committed house.
    pub fn export_ifc_house(ctx: &Context) -> Result<Outcome, String> {
        let bytes = ctx.input_bytes(SNAPSHOT)?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = projection(&snapshot).to_json();
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
        built = built.subject("export-ifc-house", subject::export_ifc_house);
    }
    built
}
//#endregion 🔖️Registration
