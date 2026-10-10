//! 🏗️ BIM export case `export-bim-1-ifc`, Rust adapter (subject role only). The IfcOpenShell measurement lives in `🐍️.py` beside this file: it opens the committed IFC file and counts,
//! locates and tessellates what it finds. This adapter answers the same scenario from the subject: it exports the committed house and reports the same table from its own Part-21 document
//! and base quantities (class counts, containment per storey, net volume of every exactly measurable element), the classification tables and the type-level property sets; for the library scenario it reports
//! the IFC4 template library of the committed psets model.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::ifc::projection::projection;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

    fn export(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let bytes = ctx.input_bytes(&uri)?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = projection(&snapshot).to_json();
        let parsed = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), parsed))
    }

    /// 🏗️ `{schema, counts, containment, volumes, annotations}` of the export of the committed house.
    pub fn export_ifc_house(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 🏗️ The same report for the frame model: the `IfcColumn`, `IfcBeam`, `IfcCurtainWall`, `IfcMember`, `IfcPlate`, `IfcDoor` and `IfcWindow` counts and the net volume of every leaning column, arc, inclined and joined beam.
    pub fn export_ifc_frame(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 🪧️ The same report for the annotated room: every `IfcAnnotation` by name.
    pub fn export_ifc_notated(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 🔲️ The same report for the ceilings model: the `IfcCovering` count, the containment and the kernel volume of every flat ceiling.
    pub fn export_ifc_ceilings(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 🛝️ The same report for the ramps model: the `IfcRamp` and `IfcRampFlight` counts, the landing slabs and the railings the ramps aggregate.
    pub fn export_ifc_ramps(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 🧗️ The same report for the attic: the `IfcWall` and `IfcMember` counts, the `IfcRelConnectsElements` of the attached walls and the containment of every sweep run.
    pub fn export_ifc_wall_depth(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 🪑️ The same report for the components model: the `IfcFurnishingElement`, `IfcFlowTerminal`, `IfcBuildingElementProxy` and `IfcFlowSegment` counts, the type objects, the `IfcSystem` groups and the net volume of every component and MEP element.
    pub fn export_ifc_components(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 🏷️ The same report for the psets model: the classification tables with their parents and attached codes and the typed property sets of every type object.
    pub fn export_ifc_psets(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 📚️ The report of the IFC4 template library of the committed psets model: counts, property set templates with their simple property templates, classification systems with their parent chains and the declarations.
    pub fn export_ifc_library(ctx: &Context) -> Result<Outcome, String> {
        use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::ifc::ifc4::{library_report, library_to_part21};
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = library_report(&library_to_part21(&snapshot).0);
        let parsed = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), parsed))
    }

    /// ⏳️ The house report of the file the stepped export job writes: a first job is cancelled half-way, a second one runs to its end on the same session and its file is read back.
    pub fn export_ifc_stepped(ctx: &Context) -> Result<Outcome, String> {
        use semio_s_artifact_bim_model::editor::bim::commands::export_model::{Advance, ExportJob};
        use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::ifc::{codec, projection::report};
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let mut cancelled = ExportJob::with_steps("ifc2x3", Some(41), 2);
        for _ in 0..2 {
            cancelled.advance(&snapshot).map_err(|fault| format!("{fault:?}"))?;
        }
        cancelled.cancel(&snapshot);
        let mut job = ExportJob::with_steps("ifc2x3", Some(41), 64);
        let output = loop {
            if let Advance::Done(output) = job.advance(&snapshot).map_err(|fault| format!("{fault:?}"))? {
                break output;
            }
        };
        let document = codec::decode_document(output.data.as_bytes())?;
        let table = report(&snapshot, &document).to_json();
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
        built = built.subject("export-ifc-house", subject::export_ifc_house).subject("export-ifc-frame", subject::export_ifc_frame).subject("export-ifc-notated", subject::export_ifc_notated).subject("export-ifc-ceilings", subject::export_ifc_ceilings).subject("export-ifc-ramps", subject::export_ifc_ramps).subject("export-ifc-stepped", subject::export_ifc_stepped).subject("export-ifc-components", subject::export_ifc_components).subject("export-ifc-psets", subject::export_ifc_psets).subject("export-ifc-library", subject::export_ifc_library).subject("export-ifc-wall-depth", subject::export_ifc_wall_depth);
    }
    built
}
//#endregion 🔖️Registration
