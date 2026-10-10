//! 🧗️ BIM inference case `solids-bim-1-wall-depth-three`, Rust adapter (subject role only). The three.js measurement of the blessed meshes lives in `🟦️.ts` beside this file. This adapter answers the
//! same scenario from the subject's own `ModelInference`: it infers the `element-solids` field from the committed attic snapshot and reports volume, area, bounds and triangle count of every solid
//! that has a committed mesh.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
    use semio_s_artifact_bim_model::ModelInference;

    fn row(solid: &ElementSolid) -> String {
        let (min, max) = (solid.bounds.min, solid.bounds.max);
        format!(
            "{{\"volume\":{:?},\"area\":{:?},\"bounds\":{{\"min\":[{:?},{:?},{:?}],\"max\":[{:?},{:?},{:?}]}},\"triangles\":{}}}",
            solid.volume,
            solid.area,
            min.x,
            min.y,
            min.z,
            max.x,
            max.y,
            max.z,
            solid.triangle_count()
        )
    }

    /// 🧊️ `{element id: {volume, area, bounds, triangles}}` of the inferred solids of the committed meshes.
    pub fn wall_depth(ctx: &Context) -> Result<Outcome, String> {
        let snapshot_uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let meshes_uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("🧊️meshes")).ok_or_else(|| "the scenario names no meshes".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&snapshot_uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let committed = ctx.input_json(&meshes_uri)?;
        let wanted: Vec<String> = committed.get("meshes").and_then(|meshes| meshes.as_object()).map(|meshes| meshes.keys().cloned().collect()).ok_or_else(|| "the case has no meshes".to_string())?;
        let inferred = ModelInference::infer(&snapshot).map_err(|error| error.to_string())?;
        let rows: Vec<String> = wanted.iter().filter_map(|id| inferred.element_solids.get(id).map(|solid| format!("\"{id}\":{}", row(solid)))).collect();
        let text = format!("{{{}}}", rows.join(","));
        let projection = parse_json(&text)?;
        Ok(Outcome::with_raw(text.into_bytes(), projection))
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
        built = built.subject("wall-depth-three", subject::wall_depth);
    }
    built
}
//#endregion 🔖️Registration
