//! 🧊️ BIM inference case `infer-bim-1-wall-solids`, Rust adapter (subject role only). The IfcOpenShell reproduction lives in `🐍️.py` beside
//! this file: it builds every straight wall from the committed layout and measures `z` extent and volume with its geometry kernel. This
//! adapter answers the same scenario from the subject's own `ModelInference`: for every straight wall `base_z`, `top_z` and `volume`.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

/// 📸️ The committed oracle house.
const SNAPSHOT: &str = "shared://💡️inferences/🏠️house/📸️snapshot/🔣️.json";

/// 🧊️ The committed case with windows, doors, voids and layers.
#[cfg(feature = "sut")]
const OPENINGS_CASE: &str = "shared://💡️inferences/🧊️element-solids/🚪️straight-openings/🔣️.json";

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{OPENINGS_CASE, SNAPSHOT};
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_inference_projection_json};

    /// 🧊️ `{wall id: {base_z, top_z, volume}}` of every wall whose axis is a straight line.
    pub fn wall_solids(ctx: &Context) -> Result<Outcome, String> {
        let bytes = ctx.input_bytes(SNAPSHOT)?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = encode_inference_projection_json(&snapshot, "wall-solids").ok_or_else(|| "wall-solids is not an inference of s.bim.model".to_string())?;
        let projection = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), projection))
    }

    /// 🧊️ `{wall id: {volume, z_min, z_max}}` of every straight wall of the committed `straight-openings` case, from the inferred `element-solids` (openings subtracted).
    pub fn wall_solids_openings(ctx: &Context) -> Result<Outcome, String> {
        use semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::model_graph::registry;
        let document = ctx.input_json(OPENINGS_CASE)?;
        let snapshot = decode_model_snapshot_json(&document.get("snapshot").ok_or_else(|| "the case has no snapshot".to_string())?.to_string())?;
        let solids = registry::try_with_inference(None, &snapshot, |inferred| inferred.element_solids.clone()).map_err(|error| error.to_string())?;
        let rows: Vec<String> = snapshot
            .walls
            .iter()
            .filter(|(_, wall)| matches!(wall.axis, semio_s_artifact_bim_model::Axis::Line { .. }))
            .filter_map(|(id, _)| solids.get(id).map(|solid| format!("\"{id}\":{{\"volume\":{:?},\"z_min\":{:?},\"z_max\":{:?}}}", solid.volume, solid.bounds.min.z, solid.bounds.max.z)))
            .collect();
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
        built = built.subject("wall-solids", subject::wall_solids).subject("wall-solids-openings", subject::wall_solids_openings);
    }
    built
}
//#endregion 🔖️Registration
