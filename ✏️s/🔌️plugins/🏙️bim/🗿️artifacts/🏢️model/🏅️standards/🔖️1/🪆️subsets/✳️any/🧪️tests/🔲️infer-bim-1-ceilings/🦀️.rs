//! 🔲️ BIM inference case `infer-bim-1-ceilings`, Rust adapter (subject role only). The shapely reproduction lives in `🐍️.py` beside this file; this adapter answers the same scenario from
//! `ModelInference` of the very same committed snapshot: the take-off of every ceiling (areas, perimeter, volumes, mass), the span of its solid and the z of its underside at the probe the
//! oracle names (the first vertex moved 5 % towards the mean of the vertices). It registers no oracle handler: a subject that re-read the committed expectation would be a self-comparison.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::element_solids::ceilings::{span, underside_at};
    use semio_s_artifact_bim_model::{ModelInference, Point2};

    fn number(value: f64) -> String {
        format!("{value:?}")
    }

    fn probe(boundary: &[semio_s_artifact_bim_model::Vertex]) -> Point2 {
        let count = boundary.len() as f64;
        let (mean_x, mean_y) = (boundary.iter().map(|vertex| vertex.point.x).sum::<f64>() / count, boundary.iter().map(|vertex| vertex.point.y).sum::<f64>() / count);
        let first = boundary[0].point;
        Point2 { x: first.x + 0.05 * (mean_x - first.x), y: first.y + 0.05 * (mean_y - first.y) }
    }

    /// 🔲️ `{ceiling id: row | null}` of the take-off and the span of every ceiling of the committed snapshot.
    pub fn takeoff(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let inferred = ModelInference::infer(&snapshot).map_err(|error| error.to_string())?;
        let mut rows = Vec::new();
        for (id, ceiling) in &snapshot.ceilings {
            let (Some(solid), Some(quantity), Some(level)) = (inferred.element_solids.get(id).filter(|solid| !solid.is_empty()), inferred.quantities.elements.get(id), inferred.storey_levels.get(&ceiling.storey)) else {
                rows.push(format!("\"{id}\":null"));
                continue;
            };
            let _ = solid;
            let (bottom, top) = span(&snapshot, ceiling, level);
            let under = underside_at(&snapshot, ceiling, level, probe(&ceiling.boundary)).map_or_else(|| "null".to_string(), number);
            rows.push(format!(
                "\"{id}\":{{\"gross_area\":{},\"net_area\":{},\"surface_area\":{},\"perimeter\":{},\"width\":{},\"gross_volume\":{},\"net_volume\":{},\"mass\":{},\"top_z\":{},\"bottom_z\":{},\"underside_z\":{under}}}",
                number(quantity.gross_area),
                number(quantity.net_area),
                number(quantity.surface_area),
                number(quantity.perimeter),
                number(quantity.width),
                number(quantity.gross_volume),
                number(quantity.net_volume),
                number(quantity.mass),
                number(top),
                number(bottom),
            ));
        }
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
        built = built.subject("ceilings-takeoff", subject::takeoff);
    }
    built
}
//#endregion 🔖️Registration
