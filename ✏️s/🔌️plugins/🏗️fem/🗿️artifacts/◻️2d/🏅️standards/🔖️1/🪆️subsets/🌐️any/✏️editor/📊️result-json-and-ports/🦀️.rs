//! 📊️ Result json and ports: the result export helpers, the io ports and the dof action argument helpers. See [`super`].

use super::*;

//#region 🔖️ExportImportHelpers
/// 👁️ Project persisted configuration from the addressed results window for rendering.
pub(super) fn config_result_display(cfg: &results_window::config::Fem2dResultsWindowConfig) -> ResultDisplay {
    let mode = cfg.result_mode.display(cfg.result_mode_index);
    ResultDisplay { source_id: cfg.result_source_id.clone(), mode }
}

/// 🎨️ Manual `semio_s_fem_model::StaticResult` -> JSON bridge for `"results:out"` (see `export_media` below)
/// — `semio_s_fem_model::StaticResult`/`ElementResult`/`Dof` don't derive `Serialize` (out of this ticket's
/// scope: `🫀️core` is a shared crate), so this hand-rolls the same shape `semio_framework_pack_json::to_json_string` would
/// have produced, using `Dof`'s existing `{:?}` formatting (already used for the reaction-label layers
/// in the results window's render).
fn dof_json(dof: Dof) -> Value {
    semio_framework_pack_json::json!(format!("{dof:?}"))
}

fn element_result_json(result: &ElementResult) -> Value {
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

fn static_result_json(result: &semio_s_fem_model::StaticResult) -> Value {
    semio_framework_pack_json::json!({
        "displacements": result.displacements.iter().map(|d| semio_framework_pack_json::json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| semio_framework_pack_json::json!({ "nodeId": r.node_id, "dof": dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| semio_framework_pack_json::json!({ "id": id, "result": element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

pub(super) fn results_map_json(results: &HashMap<String, semio_s_fem_model::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), static_result_json(result))).collect())
}
//#endregion 🔖️ExportImportHelpers

//#region 🔌️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document port pair
/// (`fem.2d` × 2D-Vector) plus `geometry:in` (importing an externally authored 2D outline as a new
/// `FemRegion` — see `import_media` above) and `results:out` (every load case/combination's solved
/// `semio_s_fem_model::StaticResult`, pinned to the `computation.fem2d` artifact kind declared in
/// `semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::computation_artifact_kind` — see `export_media` above). Moved out of the
/// (now deleted) artifact `⚙️engine`: it returns `AppIo`, an app type, so it belongs here.
pub fn fem2d_io() -> AppIo {
    AppIo {
        artifact_schema: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        ports: vec![fem2d_geometry_in_port(), fem2d_results_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.fem".into(), name: "FEM 2D".into(), dimension: "2d".into(), component_kind: "fem2d".into() },
    }
}

/// 🔌️ `geometry:in` — an externally authored 2D polygon-with-holes outline, imported as a new
/// `FemRegion`.
pub fn fem2d_geometry_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "geometry:in".into(),
        label: "Geometry".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: None,
        required: true,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `results:out` — every load case/combination's solved `semio_s_fem_model::StaticResult`, pinned to the
/// `computation.fem2d` artifact kind.
pub fn fem2d_results_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "results:out".into(),
        label: "Results".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        kind_id: Some("computation.fem2d".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}
//#endregion 🔌️Io

//#region 🔖️ActionArgHelpers
/// 🛡️ Reads one `FemDof` out of an action arg's free text (`"tx"`, `"Ty"`, `"rz"`, …) — the shells
/// stage every declared arg as a JSON scalar, so the typed enum is resolved here rather than by the
/// stringly action wire.
pub(super) fn fem2d_dof(value: Option<&str>) -> Option<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof> {
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

/// 🛡️ Reads `addSupport`'s separator-delimited `fixed` list, defaulting to the pinned support
/// (`tx,ty`) every fem2d fixture starts from when nothing is staged.
pub(super) fn fem2d_dofs(value: Option<&str>) -> Vec<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof> {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
    let parsed: Vec<FemDof> = value.map(|text| text.split([',', ' ', ';']).filter_map(|token| fem2d_dof(Some(token))).collect()).unwrap_or_default();
    if parsed.is_empty() {
        vec![FemDof::Tx, FemDof::Ty]
    } else {
        parsed
    }
}
//#endregion 🔖️ActionArgHelpers
