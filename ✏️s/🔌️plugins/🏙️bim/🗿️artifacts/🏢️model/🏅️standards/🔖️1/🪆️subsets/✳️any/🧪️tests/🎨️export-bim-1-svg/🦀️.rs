//! 🎨️ BIM export case `export-bim-1-svg`, Rust adapter (subject role only). The lxml + shapely measurement lives in `🐍️.py` beside this file: it opens the committed SVG file and
//! counts, reads and measures what it finds. This adapter answers the same scenario from the subject: it infers the view linework of the committed house and reports the same table from them
//! and from the sheet frames the writer snaps its coordinates with (view group, kind, scale, region, line and text counts, paths per style class, arcs, straight poche area and line length).
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::svg::projection::projection;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

    fn export(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let bytes = ctx.input_bytes(&uri)?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = projection(&snapshot).to_json();
        let parsed = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), parsed))
    }

    /// 🎨️ `{width, height, views}` of the SVG export of the committed house.
    pub fn export_svg_house(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 🪑️ The same report for the components room, with the symbols of every component and routed element by model id.
    pub fn export_svg_components(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
    }

    /// 🪧️ The same report for the annotated room, with its annotation layer counted and measured per kind.
    pub fn export_svg_notated(ctx: &Context) -> Result<Outcome, String> {
        export(ctx)
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
        built = built.subject("export-svg-house", subject::export_svg_house).subject("export-svg-notated", subject::export_svg_notated).subject("export-svg-components", subject::export_svg_components);
    }
    built
}
//#endregion 🔖️Registration
