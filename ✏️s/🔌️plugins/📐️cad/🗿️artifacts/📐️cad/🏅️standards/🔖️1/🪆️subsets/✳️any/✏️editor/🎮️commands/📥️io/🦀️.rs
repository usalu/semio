//! 📥️ CAD play app commands — the shell file round-trip: native/spatial import and the three export flavours.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::modes::edit::tools::transform::{cad_transform_tool_emit, CadToolEntry};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{cad_solid_export_effect, cad_spatial_export_effect, cad_pane_from_view, export_solid_for_pane, export_solid_modelspace, export_spatial_json, publish_engagement, reset_document_effect, runtime_of, CadInteractionSnapshot, CadPlayView};
use crate::op::CadMutation;
use crate::standards::v1::subsets::any::io::{import_cad_object_by_extension, scene_from_spatial_payload, unwrap_spatial_load_payload, CAD_SOLID_EXPORT_DIALECT_OBJ, CAD_SOLID_EXPORT_DIALECT_STEP, CAD_SOLID_EXPORT_DIALECT_STL};
use crate::{CadPaneId, CadSnapshot};
use semio_framework_value::DslValue;
use semio_framework::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ImportCadFile
pub mod import_cad_file {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "import-cad-file")]
    pub struct ImportCadFile {
        pub name: String,
        pub payload: String,
    }

    /// 📥️ A single object file (STEP, OBJ, STL, GLB) lands as ONE `insert-element` transaction on the composed model child
    /// of the addressed pane (the shape pane without an addressed window, design §20.15); a whole spatial scene replaces
    /// the document through the host's `LoadDocument`. Selecting the imported object is the host's follow-up
    /// `interactionSelect` — selection is framework-owned.
    pub fn handle(payload: &ImportCadFile, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg, &ctx.window_transient);
        let name_lower = payload.name.to_ascii_lowercase();
        let payload_value: DslValue = semio_framework_pack_json::from_json_str(&payload.payload, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_value::DslValue::String(payload.payload.clone()));
        if let Some(element) = import_cad_object_by_extension(&name_lower, &payload_value) {
            let pane = ctx.view_state.as_ref().and_then(|view| cad_pane_from_view(view).ok()).unwrap_or(CadPaneId::Shape);
            let emit = cad_transform_tool_emit(doc, "importCadFile", vec![CadToolEntry::Create { pane, element }]);
            if emit.child_preparations.is_empty() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("cad.import-object-refused"), format!("importCadFile cannot place the object \"{}\": the addressed pane composes no model child or already holds its id", payload.name)));
            }
            return Ok(emit);
        }
        let unwrapped = unwrap_spatial_load_payload(&payload_value).unwrap_or(payload_value);
        let scene = scene_from_spatial_payload(&unwrapped).or_else(|| <CadSnapshot as semio_framework_value::FromValue>::from_value(unwrapped).ok());
        if let Some(scene) = scene {
            runtime.engagement_session = None;
            publish_engagement(&runtime, ctx);
            return Ok(Emit { effects: vec![reset_document_effect(&scene)], ..Default::default() });
        }
        Err(Fault::new(FaultOrigin::App, FaultCode::new("cad.import-unreadable"), format!("importCadFile cannot read \"{}\" as a spatial scene or a CAD object", payload.name)))
    }
}
//#endregion 🔖️ImportCadFile

//#region 🔖️SaveSelected
pub mod save_selected {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "save-selected")]
    pub struct SaveSelected {}

    pub fn handle(_payload: &SaveSelected, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let pane = cad_pane_from_view(ctx.view_state.as_ref().ok_or_else(|| Fault::from("cad.window.invalid: selected export has no host view context"))?)?;
        let view = CadPlayView::of(doc.snapshot, &doc.children, runtime_of(cfg, &ctx.window_transient), CadInteractionSnapshot::default());
        let export = export_spatial_json(&view, "selected", Some(pane))?;
        Ok(Emit::effect(cad_spatial_export_effect(&export, "cad.selected.spatial.dsl")))
    }
}
//#endregion 🔖️SaveSelected

//#region 🔖️SaveInPlay
pub mod save_in_play {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "save-in-play")]
    pub struct SaveInPlay {}

    pub fn handle(_payload: &SaveInPlay, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let view = CadPlayView::of(doc.snapshot, &doc.children, runtime_of(cfg, &ctx.window_transient), CadInteractionSnapshot::default());
        let effect = match export_solid_modelspace(&view, CAD_SOLID_EXPORT_DIALECT_STEP) {
            Some(export) => cad_solid_export_effect(export),
            None => cad_spatial_export_effect(&export_spatial_json(&view, "modelspace", None)?, "cad.modelspace.spatial.dsl"),
        };
        Ok(Emit::effect(effect))
    }
}
//#endregion 🔖️SaveInPlay

//#region 🔖️SaveCurrent
pub mod save_current {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "save-current")]
    pub struct SaveCurrent {
        pub format: Option<String>,
    }

    pub fn handle(payload: &SaveCurrent, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let format = match payload.format.as_deref() {
            Some("obj") => CAD_SOLID_EXPORT_DIALECT_OBJ,
            Some("stl") => CAD_SOLID_EXPORT_DIALECT_STL,
            _ => CAD_SOLID_EXPORT_DIALECT_STEP,
        };
        let pane = cad_pane_from_view(ctx.view_state.as_ref().ok_or_else(|| Fault::from("cad.window.invalid: current export has no host view context"))?)?;
        let view = CadPlayView::of(doc.snapshot, &doc.children, runtime_of(cfg, &ctx.window_transient), CadInteractionSnapshot::default());
        let export = export_solid_for_pane(&view, pane, format).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("cad.export.empty-pane"), "The current pane has no solid to export."))?;
        Ok(Emit::effect(cad_solid_export_effect(export)))
    }
}
//#endregion 🔖️SaveCurrent

//#region 🔖️LoadRawRequest
pub mod load_raw_request {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "load-raw-request")]
    pub struct LoadRawRequest {}

    pub fn handle(_payload: &LoadRawRequest, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen {
            req: semio_framework_plugin::RequestId(116),
            accept: ".dsl,.spatial.dsl,.spk,.ops,.stp,.step,.obj,.stl,.glb,application/octet-stream,text/plain".into(),
            read_as: Some("dataUrl".into()),
            import_action: "importCadFile".into(),
            multiple: false,
        args: None, }))
    }
}
//#endregion 🔖️LoadRawRequest
