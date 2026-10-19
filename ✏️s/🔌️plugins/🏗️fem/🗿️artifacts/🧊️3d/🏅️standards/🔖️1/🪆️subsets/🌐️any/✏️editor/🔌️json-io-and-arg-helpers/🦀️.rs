//! 🔌️ Result json, ports and argument helpers of the FEM 3D editor: the results map json, the io ports and the dof argument decoders. See [`super`].

use super::*;

//#region 🔖️Fem3dResultsJson
/// 🎨️ Manual `semio_s_fem_model::StaticResult` -> JSON bridge for `"results:out"` (see `export_media` below)
/// — `semio_s_fem_model::StaticResult`/`ElementResult`/`Dof` don't derive `Serialize` (the `🫀️core` kernel is
/// a cross-artifact shared crate, out of scope to touch here), so this hand-rolls the same shape
/// `semio_framework_pack_json::to_json_string` would have produced, using `Dof`'s existing `{:?}` formatting. Single
/// consumer (`export_media`), so this lives here rather than in the artifact's `⚙️engine`.
fn fem3d_dof_json(dof: Dof) -> Value {
    semio_framework_pack_json::json!(format!("{dof:?}"))
}

fn fem3d_element_result_json(result: &ElementResult) -> Value {
    match result {
        ElementResult::Bar { n } => semio_framework_pack_json::json!({ "kind": "bar", "n": n }),
        ElementResult::Beam { stations } => {
            semio_framework_pack_json::json!({ "kind": "beam", "stations": stations.iter().map(|s| semio_framework_pack_json::json!({ "x": s.x, "n": s.n, "v": s.v, "m": s.m })).collect::<Vec<_>>() })
        }
        ElementResult::Plane { gauss } => {
            semio_framework_pack_json::json!({ "kind": "plane", "gauss": gauss.iter().map(|g| semio_framework_pack_json::json!({ "sxx": g.sxx, "syy": g.syy, "sxy": g.sxy, "vonMises": g.von_mises })).collect::<Vec<_>>() })
        }
        ElementResult::Plate { gauss } => {
            semio_framework_pack_json::json!({ "kind": "plate", "gauss": gauss.iter().map(|g| semio_framework_pack_json::json!({ "mx": g.mx, "my": g.my, "mxy": g.mxy })).collect::<Vec<_>>() })
        }
        ElementResult::Solid { gauss } => semio_framework_pack_json::json!({
            "kind": "solid",
            "gauss": gauss.iter().map(|g| semio_framework_pack_json::json!({ "sxx": g.sxx, "syy": g.syy, "szz": g.szz, "sxy": g.sxy, "syz": g.syz, "sxz": g.sxz, "vonMises": g.von_mises })).collect::<Vec<_>>(),
        }),
        ElementResult::Shell { gauss } => semio_framework_pack_json::json!({
            "kind": "shell",
            "gauss": gauss.iter().map(|g| semio_framework_pack_json::json!({ "nxx": g.nxx, "nyy": g.nyy, "nxy": g.nxy, "mxx": g.mxx, "myy": g.myy, "mxy": g.mxy, "vonMisesTop": g.von_mises_top, "vonMisesBottom": g.von_mises_bottom })).collect::<Vec<_>>(),
        }),
    }
}

fn fem3d_static_result_json(result: &semio_s_fem_model::StaticResult) -> Value {
    semio_framework_pack_json::json!({
        "displacements": result.displacements.iter().map(|d| semio_framework_pack_json::json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| semio_framework_pack_json::json!({ "nodeId": r.node_id, "dof": fem3d_dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| semio_framework_pack_json::json!({ "id": id, "result": fem3d_element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

pub(super) fn fem3d_results_map_json(results: &HashMap<String, semio_s_fem_model::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), fem3d_static_result_json(result))).collect())
}
//#endregion 🔖️Fem3dResultsJson

//#region 🔌️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document port pair
/// (`fem.3d` × 3D-Any) plus `geometry:in` (importing an externally authored extruded-footprint outline
/// as a new `FemSolid` — see `import_media` above) and `results:out` (every load case/combination's
/// solved `semio_s_fem_model::StaticResult`, pinned to the `computation.fem3d` artifact kind declared in
/// `semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::computation_artifact_kind` — see `export_media` above). Moved out of the
/// (now deleted) artifact `⚙️engine`: it returns `AppIo`, an app type, so it belongs here.
pub fn fem3d_io() -> AppIo {
    AppIo {
        artifact_schema: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any },
        ports: vec![fem3d_geometry_in_port(), fem3d_results_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.fem".into(), name: "FEM 3D".into(), dimension: "3d".into(), component_kind: "fem3d".into() },
    }
}

/// 🔌️ `geometry:in` — an externally authored extruded-footprint outline (polygon-with-holes,
/// base/height/layers), imported as a new `FemSolid`.
pub fn fem3d_geometry_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "geometry:in".into(),
        label: "Geometry".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any },
        kind_id: None,
        required: true,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `results:out` — every load case/combination's solved `semio_s_fem_model::StaticResult`, pinned to the
/// `computation.fem3d` artifact kind.
pub fn fem3d_results_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "results:out".into(),
        label: "Results".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        kind_id: Some("computation.fem3d".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}
//#endregion 🔌️Io

//#region 🔖️ActionArgHelpers
/// 🧭️ Parses one staged `dof` tag (`tx`…`rz`, case-insensitive) into the shared six-tag `FemDof`.
pub(super) fn fem3d_dof(value: Option<&str>) -> Option<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof> {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
    match value?.trim().to_ascii_lowercase().as_str() {
        "tx" => Some(FemDof::Tx),
        "ty" => Some(FemDof::Ty),
        "tz" => Some(FemDof::Tz),
        "rx" => Some(FemDof::Rx),
        "ry" => Some(FemDof::Ry),
        "rz" => Some(FemDof::Rz),
        _ => None,
    }
}

/// 🛡️ Reads `addSupport`'s separator-delimited `fixed` list, defaulting to the fully clamped base
/// (`tx,ty,tz,rx,ry,rz`) the fem3d fixtures start from when nothing is staged.
pub(super) fn fem3d_dofs(value: Option<&str>) -> Vec<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof> {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
    let parsed: Vec<FemDof> = value.map(|text| text.split([',', ' ', ';']).filter_map(|token| fem3d_dof(Some(token))).collect()).unwrap_or_default();
    if parsed.is_empty() {
        vec![FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz]
    } else {
        parsed
    }
}
//#endregion 🔖️ActionArgHelpers
