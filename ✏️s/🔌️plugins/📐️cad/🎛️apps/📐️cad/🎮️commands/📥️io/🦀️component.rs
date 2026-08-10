//! 📥️ CAD play app commands — the shell file round-trip: native/spatial import and the three export flavours.

use crate::apps::cad::config::{CadConfig, CadConfigMutation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{cad_solid_export_effect, cad_spatial_export_effect, export_solid_for_pane, export_solid_modelspace, export_spatial_json, runtime_of, snapshot_of, CadPlayView};
use crate::artifacts::cad::engine::{import_cad_object_by_extension, scene_from_spatial_payload, unwrap_spatial_load_payload};
use crate::artifacts::cad::{cad_pane_from_model_definition_id, CadPaneId};
use semio_framework::kernel::HostEffect;
use semio_framework_plugin::{MediaFormat, SelectionSet};
use serde_json::Value;


//#region 🔖️ImportCadFile
pub mod import_cad_file {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-cad-file")]
    pub struct ImportCadFile {
        pub name: String,
        pub payload: String,
    }

    pub fn handle(payload: &ImportCadFile, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        let name_lower = payload.name.to_ascii_lowercase();
        let payload_value: Value = serde_json::from_str(&payload.payload).unwrap_or_else(|_| Value::String(payload.payload.clone()));
        if let Some(object) = import_cad_object_by_extension(&name_lower, &payload_value) {
            runtime.selected_object_ids = SelectionSet::from(vec![object.id.clone()]);
            let mut emit = Emit::mutations(vec![CadMutation::AddObject { pane: CadPaneId::Shape, object }]);
            emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)];
            return Ok(emit);
        }
        let unwrapped = unwrap_spatial_load_payload(&payload_value).unwrap_or(payload_value);
        let scene = scene_from_spatial_payload(&unwrapped).or_else(|| serde_json::from_value::<CadSnapshot>(unwrapped).ok());
        if let Some(scene) = scene {
            runtime.selected_object_ids.clear();
            runtime.engagement_session = None;
            let mut emit = Emit::mutations(vec![CadMutation::SetSnapshot { snapshot: Box::new(scene) }]);
            emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)];
            return Ok(emit);
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️ImportCadFile

//#region 🔖️SaveSelected
pub mod save_selected {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "save-selected")]
    pub struct SaveSelected {}

    pub fn handle(_payload: &SaveSelected, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let view = CadPlayView { document: doc.snapshot.clone(), runtime: runtime_of(cfg) };
        Ok(Emit::effect(cad_spatial_export_effect(&export_spatial_json(&view, "selected"), "cad.selected.spatial.dsl")))
    }
}
//#endregion 🔖️SaveSelected

//#region 🔖️SaveInPlay
pub mod save_in_play {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "save-in-play")]
    pub struct SaveInPlay {}

    pub fn handle(_payload: &SaveInPlay, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let view = CadPlayView { document: doc.snapshot.clone(), runtime: runtime_of(cfg) };
        let effect = match export_solid_modelspace(&view, MediaFormat::Step) {
            Some(export) => cad_solid_export_effect(export),
            None => cad_spatial_export_effect(&export_spatial_json(&view, "modelspace"), "cad.modelspace.spatial.dsl"),
        };
        Ok(Emit::effect(effect))
    }
}
//#endregion 🔖️SaveInPlay

//#region 🔖️SaveCurrent
pub mod save_current {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "save-current")]
    pub struct SaveCurrent {
        pub format: Option<String>,
    }

    pub fn handle(payload: &SaveCurrent, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let format = match payload.format.as_deref() {
            Some("obj") => MediaFormat::Obj,
            Some("stl") => MediaFormat::Stl,
            _ => MediaFormat::Step,
        };
        let pane = cad_pane_from_model_definition_id(&document.active_model_definition_id).unwrap_or(CadPaneId::Shape);
        let view = CadPlayView { document: document.clone(), runtime: runtime_of(cfg) };
        let effect = match export_solid_for_pane(&view, pane, format) {
            Some(export) => cad_solid_export_effect(export),
            None => cad_spatial_export_effect(&export_spatial_json(&view, "current"), "cad.current.spatial.dsl"),
        };
        Ok(Emit::effect(effect))
    }
}
//#endregion 🔖️SaveCurrent

//#region 🔖️LoadRawRequest
pub mod load_raw_request {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "load-raw-request")]
    pub struct LoadRawRequest {}

    pub fn handle(_payload: &LoadRawRequest, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(Emit::effect(HostEffect::RequestFileOpen {
            accept: ".dsl,.spatial.dsl,.spk,.ops,.stp,.step,.obj,.stl,.glb,application/octet-stream,text/plain".into(),
            read_as: Some("dataUrl".into()),
            import_action: "importCadFile".into(),
            multiple: false,
        }))
    }
}
//#endregion 🔖️LoadRawRequest
