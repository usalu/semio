//! 👁️ Preview of the instance-local progressive or completed fill at its exact source revision.

use crate::editor::grid3d::modes::edit::tools::fill::{self, Grid3dFillPayload};
use crate::editor::grid3d::window::Grid3dWindowConfig;
use crate::schema::inferences::Grid3dAssignment;
use crate::editor::grid3d::transient::Grid3dTransient;




use crate::Grid3dSnapshot;
use semio_plugin::world3d_camera_json;
use semio_plugin::world3d_scene;
use semio_plugin::world3d_selection_json;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::ToolRunView;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowOptions;
use semio_plugin::WorldSunConfig;
use semio_plugin::plugin_app_close_prelude::SurfaceKind;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "wfc-grid3d-preview";
pub const BODY_KEY: &str = "wfc.grid3d.preview";
pub const SURFACE_ID: &str = "wfc.grid3d.preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 👁️ Stitched into the editor manifest by `crate::editor::grid3d::create_grid3d_editor`. Read-only:
/// it declares no action and no utility, because the solve is an inference and this window only
/// shows it.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: BODY_KEY.into(),
        surface_kind: semio_plugin::SurfaceKind::World3d,
        icon_id: "preview".into(),
        options: WindowOptions::default(),
        actions: vec![semio_framework_plugin::ActionDefinition{in_palette:false,..semio_framework_plugin::ActionDefinition::bounded_catalog("commit-fill",LocalizedLabel::native("Commit Fill","Füllen übernehmen"),semio_framework_plugin::ActionKind::Mutation)}],
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

/// 🪪️ A result is visible only at its exact canonical render revision.
pub fn source_matches(source:&[u8],revision:Option<&[u8;32]>)->bool {revision.is_some_and(|revision|source==revision.as_slice())}

/// 🪣️ Only a running fill at the current document revision supplies a live preview.
pub fn live_fill_payload(tool_run:Option<&ToolRunView>,revision:Option<&[u8;32]>)->Option<Grid3dFillPayload>{
    let run=fill::live_fill_run(tool_run)?;
    if !source_matches(&run.identity.base_revision,revision){return None;}
    let payload=Grid3dFillPayload::decode(run.payload.as_ref()?)?;
    source_matches(&payload.source_revision,revision).then_some(payload)
}

/// 🏁️ Completed fills live in the app transient; an unsolved or stale document remains empty.
pub fn paint_assignments(transient:&Grid3dTransient,tool_run:Option<&ToolRunView>,revision:Option<&[u8;32]>)->(Vec<Grid3dAssignment>,bool){
    let payload=live_fill_payload(tool_run,revision).or_else(||Grid3dFillPayload::decode(transient.payload_json.as_bytes()).filter(|payload|payload.done&&source_matches(&payload.source_revision,revision)));
    let Some(payload)=payload else{return (Vec::new(),true)};
    if payload.contradiction{return (Vec::new(),false)}
    let mut assignments=payload.decided_assignments();
    if !payload.done {for event in &payload.trace {
        assignments.retain(|cell|cell.x!=event.x||cell.y!=event.y||cell.z!=event.z);
        assignments.push(Grid3dAssignment{x:event.x,y:event.y,z:event.z,tile_id:event.tile_id.clone()});
    }
    }
    (assignments,true)
}

//#region 🔖️Render
/// 👁️ The solved scene. A contradiction paints the empty instance set and says so in the status line,
/// rather than failing the surface.
pub fn render(document: &Grid3dSnapshot, config: &Grid3dWindowConfig, transient:&Grid3dTransient, tool_run: Option<&ToolRunView>, revision:Option<&[u8;32]>) -> UiAssemblyResult<BuiltNode> {
    let (position, target) = super::grid::framed_camera(document, config);
    let (assignments,satisfiable)=paint_assignments(transient,tool_run,revision);
    let instances=crate::standards::v1::subsets::any::io::text::inferences::scene_projection::preview_instances_json(document,&assignments);
    let status = status_json(document, satisfiable, &instances);
    let scene = semio_ui_declaration::ui::World3dScene {
        instances_delta_json: None,
        status_json: Some(status),
        ..world3d_scene(
            world3d_camera_json(position, target, 45.0),
            crate::standards::v1::subsets::any::io::text::inferences::scene_projection::preview_meshes_json(document),
            instances,
            world3d_selection_json("interactionSelect", &[], None),
            &WorldSunConfig::default(),
        )
    };
    semio_plugin::scene_surface(SURFACE_ID, SurfaceKind::World3d, &scene)
}

/// 🩺️ The one-line verdict the window chrome shows: solved cell count against the cells the grid
/// actually has to fill, plus the last solve's verdict. Kept short on purpose — an oversized status
/// string fails the WHOLE surface's admission, not just this field.
pub fn status_json(document: &Grid3dSnapshot, satisfiable: bool, instances: &str) -> String {
    let total = (document.width as usize) * (document.height as usize) * (document.depth as usize);
    let cells = total.saturating_sub(document.masked.len().min(total));
    let solved = instances.matches("\"meshId\"").count();
    let state = if !satisfiable {
        "contradiction"
    } else if solved == 0 {
        "unsolved"
    } else {
        "solved"
    };
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("state".to_string(), semio_framework_pack_json::Value::String(state.to_string())),
        ("solved".to_string(), semio_framework_pack_json::Value::from(solved as u64)),
        ("cells".to_string(), semio_framework_pack_json::Value::from(cells as u64)),
        ("seed".to_string(), semio_framework_pack_json::Value::from(document.seed)),
    ]))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
