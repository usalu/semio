//! 🧊️ BIM inference case `infer-bim-1-solids-three`, Rust adapter (subject role only). The three.js measurement of the blessed meshes lives in `🟦️.ts` beside
//! this file. This adapter answers the same scenario from the subject's own `ModelInference`: for every committed case it infers the `element-solids` field
//! from the committed snapshot and reports volume, area, bounds and triangle count of every solid.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

/// 📸️ The committed cases, in the order of the scenario.
/// 📁️ (projection key, fixture directory) of the committed cases, in the order of the scenario.
const CASES: [(&str, &str); 7] = [("straight-openings", "🚪️straight-openings"), ("room-joins", "🧩️room-joins"), ("curtain-grid", "🏬️curtain-grid"), ("ceilings-meshes", "🪵️ceilings-meshes"), ("frame-tilt-joins", "📐️frame-tilt-joins"), ("curtain-overrides", "🪟️curtain-overrides"), ("components-mep", "🪑️components-mep")];

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::CASES;
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::model_graph::registry;

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

    /// 🧊️ `{case: {element id: {volume, area, bounds, triangles}}}` of the inferred solids of every committed case.
    pub fn element_solids(ctx: &Context) -> Result<Outcome, String> {
        let mut cases = Vec::new();
        for (name, directory) in CASES {
            let document = ctx.input_json(&format!("shared://💡️inferences/🧊️element-solids/{directory}/🔣️.json"))?;
            let snapshot = decode_model_snapshot_json(&document.get("snapshot").ok_or_else(|| format!("{name}: the case has no snapshot"))?.to_string())?;
            let solids = registry::try_with_inference(None, &snapshot, |inferred| inferred.element_solids.clone()).map_err(|error| error.to_string())?;
            let rows: Vec<String> = solids.iter().map(|(id, solid)| format!("\"{id}\":{}", row(solid))).collect();
            cases.push(format!("\"{name}\":{{{}}}", rows.join(",")));
        }
        let text = format!("{{{}}}", cases.join(","));
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
        built = built.subject("element-solids-three", subject::element_solids);
    }
    built
}
//#endregion 🔖️Registration
