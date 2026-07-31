//! 🖥️ GIS 3D app — DocumentApp impl, render, manifest (constitutional: ui).
//!
//! ⛰️ Reuses the existing `World3d` viewport/renderer rather than a bespoke one; deliberately
//! read-mostly for this first pass — the only editable/undoable property is vertical exaggeration.

use gis3d::{Gis3dTerrainDocument, GIS_3D_TERRAIN_SCHEMA};
use gis3d_dsl::REUSE_TERRAIN_EXAMPLE_TEXT;
use gis3d_engine::default_terrain_document;
use gis3d_op::Gis3dTerrainOperation;
use framework_surface_terrain::{build_terrain_scene_json, projection, TerrainDescriptorJson};
use semio_framework_plugin::{
    app_labels, build_world_3d_scene, create_default_layout, is_de_locale, localized_label_map, resolve_labels, ui_text,
    world3d_default_camera, world3d_scene_extended, world3d_selection_json,
    ActionEmit, App, AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView, SurfaceKind, UiNode, ViewState, WindowMeasure,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
const GIS3D_PLAY_APP_ID: &str = "gis3d-play";
const GIS3D_PLAY_SURFACE: &str = "gis3d.play.composite";
const GIS3D_PLAY_BODY_COMPOSITE: &str = "gis3d.play.composite";
const GIS3D_PLAY_WINDOW_MAIN: &str = "gis3d-main";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🎛️ Ephemeral view state — the read-only terrain fixture, the camera, and the current selection —
/// lives in the app struct; only the vertical exaggeration is document (undoable) state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Gis3dPlayRuntime {
    #[serde(default)]
    terrain_fixture_text: String,
    #[serde(default = "world3d_default_camera")]
    camera_json: String,
    #[serde(default)]
    selected_ids: Vec<String>,
}

impl Default for Gis3dPlayRuntime {
    fn default() -> Self {
        Self {
            terrain_fixture_text: REUSE_TERRAIN_EXAMPLE_TEXT.into(),
            camera_json: initial_camera_json(),
            selected_ids: Vec::new(),
        }
    }
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
/// 📜️ Hand-rolled reader for the `.gisterrain` fixture's `origin`/`position` scenery lines — the
/// read-only pins/project-origin data rendered alongside the document (see module docs); the
/// `gisterrain exaggeration=...` header line those same files start with is instead read by
/// `Gis3dTerrainDocument`'s own derive-generated `DocumentDsl` (see `gis3d::Gis3dTerrainDocument`),
/// since exaggeration is the one piece of this fixture that IS undoable document state.
mod terrain_fixture_text {
    use framework_surface_terrain::{TerrainDescriptorJson, TerrainPositionData, TerrainProjectOrigin};

    /// 🔤️ Splits one line into whitespace-separated tokens, treating a `"..."` quoted run (escapes
    /// `\\`, `\"`, `\n`) as part of the token it's glued to — so `label="Institut de Botanique"`
    /// lexes as one `label=Institut de Botanique` token even though the value contains spaces.
    fn line_tokens(line: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut chars = line.chars().peekable();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                continue;
            }
            let mut token = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() {
                    break;
                }
                if c == '"' {
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '"' {
                            break;
                        }
                        if c == '\\' {
                            match chars.next() {
                                Some('n') => token.push('\n'),
                                Some('"') => token.push('"'),
                                Some('\\') => token.push('\\'),
                                Some(other) => {
                                    token.push('\\');
                                    token.push(other);
                                }
                                None => {}
                            }
                        } else {
                            token.push(c);
                        }
                    }
                } else {
                    token.push(c);
                    chars.next();
                }
            }
            tokens.push(token);
        }
        tokens
    }

    fn kv_lookup<'a>(tokens: &'a [String], key: &str) -> Option<&'a str> {
        tokens.iter().find_map(|token| token.strip_prefix(&format!("{key}=")))
    }

    fn parse_project_origin(tokens: &[String]) -> Option<TerrainProjectOrigin> {
        Some(TerrainProjectOrigin { lon: kv_lookup(tokens, "lon")?.parse().ok()?, lat: kv_lookup(tokens, "lat")?.parse().ok()? })
    }

    fn parse_position(tokens: &[String]) -> Option<TerrainPositionData> {
        Some(TerrainPositionData {
            id: kv_lookup(tokens, "id")?.to_string(),
            lon: kv_lookup(tokens, "lon")?.parse().ok()?,
            lat: kv_lookup(tokens, "lat")?.parse().ok()?,
            label: kv_lookup(tokens, "label").map(str::to_string),
            icon: kv_lookup(tokens, "icon").map(str::to_string),
        })
    }

    /// 📥️ Parses every `origin`/`position` line of the fixture text (its `gisterrain exaggeration=...`
    /// header is parsed separately, see module docs); malformed or missing lines simply contribute
    /// nothing, so a truncated/empty fixture yields the world origin with no positions rather than an error.
    pub(super) fn parse_descriptor(text: &str, schema: &str, exaggeration: f64) -> TerrainDescriptorJson {
        let mut project_origin = TerrainProjectOrigin { lon: 0.0, lat: 0.0 };
        let mut positions = Vec::new();
        for line in text.lines() {
            let tokens = line_tokens(line);
            match tokens.first().map(String::as_str) {
                Some("origin") => {
                    if let Some(origin) = parse_project_origin(&tokens) {
                        project_origin = origin;
                    }
                }
                Some("position") => {
                    if let Some(position) = parse_position(&tokens) {
                        positions.push(position);
                    }
                }
                _ => {}
            }
        }
        TerrainDescriptorJson { schema: schema.to_string(), project_origin, positions, exaggeration }
    }
}

/// 🏔️ The full rendering descriptor (project origin + pins + exaggeration) for the current runtime's
/// fixture text; `exaggeration` here always mirrors the LIVE document (see `render_canvas`'s override),
/// not this fixture text's own header, so the fixture's exaggeration line only ever seeds the document
/// once via {@link gis3d_engine::default_terrain_document}.
fn parse_descriptor(runtime: &Gis3dPlayRuntime, exaggeration: f64) -> TerrainDescriptorJson {
    terrain_fixture_text::parse_descriptor(&runtime.terrain_fixture_text, GIS_3D_TERRAIN_SCHEMA, exaggeration)
}

/// 🎥️ A default overview camera scaled for a real-world DEM tile patch (hundreds of meters to a
/// few kilometers wide) — the generic `world3d_default_camera()` (position `[4,-4,3]`) assumes
/// an object-scale scene and would sit inside the ground here.
fn initial_camera_json() -> String {
    json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the GIS 3D app; one field per label makes every locale combination compile-checked.
app_labels! {
    struct Gis3dPlayLabels {
        window_terrain: &'static str = en: "Terrain", de: "Gelände";
        mode_view: &'static str = en: "View", de: "Ansicht";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every view-action/operation declared in `create_gis3d_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how
/// the command palette and Actions rail get a translated label without threading locale through the
/// whole builder chain.
fn gis3d_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("worldSelect", "Select", "Auswählen"),
        ("setExaggeration", "Set Exaggeration", "Überhöhung festlegen"),
    ])
}
//#endregion 🔖️CommandLabels

//#region 🔖️Render
/// 📍️ GIS pins are emitted as plain `World3d` instances with no matching `meshesJson` entry —
/// `WorldInstancesLayer`'s existing missing-mesh fallback renders a small colored box, so
/// selection/hover/context-menu all work for free without any new scene-schema surface.
fn instances_json(descriptor: &TerrainDescriptorJson) -> String {
    let instances: Vec<Value> = descriptor
        .positions
        .iter()
        .map(|position| {
            let (x, y) = projection::lonlat_to_local_meters(position.lon, position.lat, descriptor.project_origin.lon, descriptor.project_origin.lat);
            json!({
                "id": position.id,
                "meshId": "pin",
                "position": [x, y, 50.0],
                "color": "#ff3355",
                "label": position.label,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn render_canvas(document: &Gis3dTerrainDocument, runtime: &Gis3dPlayRuntime) -> UiNode {
    let descriptor = parse_descriptor(runtime, document.exaggeration);
    let mut scene = world3d_scene_extended(
        runtime.camera_json.clone(),
        "[]".into(),
        instances_json(&descriptor),
        world3d_selection_json("rectangle", &runtime.selected_ids, None),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    scene.terrain_json = Some(build_terrain_scene_json(&descriptor));
    build_world_3d_scene(GIS3D_PLAY_SURFACE, GIS3D_PLAY_APP_ID, scene)
}
//#endregion 🔖️Render

//#region 🔖️Gis3dPlayApp
#[derive(Default)]
pub struct Gis3dPlayApp {
    runtime: Gis3dPlayRuntime,
}

impl DocumentApp for Gis3dPlayApp {
    type Projection = Gis3dTerrainDocument;
    type Operation = Gis3dTerrainOperation;

    fn app_id(&self) -> &str {
        GIS3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        GIS_3D_TERRAIN_SCHEMA
    }

    fn initial_projection(&self) -> Gis3dTerrainDocument {
        default_terrain_document()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        _doc: &DocumentView<'_, Gis3dTerrainDocument>,
        _view_state: &ViewState,
    ) -> ActionEmit<Gis3dTerrainOperation> {
        match action {
            "setCamera" => {
                let camera = args.and_then(|value| value.get("camera")).or_else(|| args.and_then(|value| value.get("cameraJson")));
                if let Some(camera) = camera {
                    self.runtime.camera_json = camera.to_string();
                }
                ActionEmit::default()
            }
            "setSelection" | "worldSelect" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                    self.runtime.selected_ids = ids;
                }
                ActionEmit::default()
            }
            "setExaggeration" => {
                if let Some(exaggeration) = args.and_then(|value| value.get("exaggeration")).and_then(|value| value.as_f64()) {
                    return ActionEmit::amend(vec![Gis3dTerrainOperation::SetExaggeration { exaggeration }], "gis3d-exaggeration");
                }
                ActionEmit::default()
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Gis3dTerrainDocument>, _view_state: &ViewState) -> UiNode {
        match body_key {
            GIS3D_PLAY_BODY_COMPOSITE => render_canvas(doc.projection, &self.runtime),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<Gis3dPlayLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(GIS3D_PLAY_WINDOW_MAIN, labels.window_terrain)
            .mode_label("view", labels.mode_view)
            .action_labels(gis3d_action_labels(is_de))
    }
}
//#endregion 🔖️Gis3dPlayApp

//#region 🔖️Manifest
pub fn create_gis3d_app() -> App {
    App::from_builder(
        App::builder(GIS3D_PLAY_APP_ID, "GIS 3D")
            .document(["semio", "gis", "3d"])
            .icon_id("gis3d")
            .mode("view", "View")
            .default_mode_id("view")
            .window_kind(GIS3D_PLAY_WINDOW_MAIN, "Terrain", GIS3D_PLAY_BODY_COMPOSITE, SurfaceKind::World3d, "terrain-3d")
            .default_layout(create_default_layout(&[GIS3D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Terrain".into()])))
            .view_action("setCamera", "Set Camera")
            .view_action("setSelection", "Set Selection")
            .view_action("worldSelect", "Select")
            .operation("setExaggeration", "Set Exaggeration")
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("reuse-terrain", "Reuse Terrain", serde_json::to_string(&default_terrain_document()).unwrap())
    .workflow("gis3d", "GIS 3D", "terrain")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<Gis3dPlayApp> {
        testkit::new_app::<Gis3dPlayApp>()
    }

    #[test]
    fn seeds_exaggeration_from_the_terrain_fixture() {
        let app = new_app();
        assert_eq!(app.projection().expect("projection").exaggeration, 1.5);
    }

    #[test]
    fn camera_and_selection_are_view_state_and_emit_no_operations() {
        let mut app = new_app();
        let camera = app
            .handle_action("setCamera", Some(&json!({ "camera": { "position": [1.0, 1.0, 1.0] } })), &ViewState::default(), &testkit::meta("local"))
            .expect("setCamera");
        assert!(camera.operations.is_empty(), "camera is ephemeral view state");
        let selection = app
            .handle_action("worldSelect", Some(&json!({ "ids": ["p_institut_de_botanique_ulg_liege"] })), &ViewState::default(), &testkit::meta("local"))
            .expect("worldSelect");
        assert!(selection.operations.is_empty(), "selection is ephemeral view state");
    }

    /// 🧪️ A slider drag is many `setExaggeration` ticks sharing one coalesce key: they fold into ONE
    /// undoable edit, so a single undo restores the fixture's exaggeration rather than a mid-drag value.
    #[test]
    fn exaggeration_drag_coalesces_into_one_undo_step() {
        let mut app = new_app();
        for value in [2.0, 2.5, 3.0] {
            app.handle_action("setExaggeration", Some(&json!({ "exaggeration": value })), &ViewState::default(), &testkit::meta("local")).expect("drag tick");
        }
        assert_eq!(app.projection().expect("projection").exaggeration, 3.0);
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").exaggeration, 1.5, "one coalesced edit: undo restores the fixture exaggeration");
    }

    /// 📜️ The `.gisterrain` fixture's `gisterrain exaggeration=...` header is parsed twice for two
    /// different purposes (see `parse_descriptor`/`gis3d_engine::default_terrain_document`'s docs); this
    /// proves the scenery-data reader (`terrain_fixture_text`) still recovers the bundled fixture's
    /// pins/origin after the document-only conversion — i.e. converting the fixture to the DSL didn't
    /// lose data.
    #[test]
    fn terrain_fixture_text_recovers_bundled_scenery_data() {
        let descriptor = parse_descriptor(&Gis3dPlayRuntime::default(), 1.5);
        assert_eq!(descriptor.project_origin.lon, 5.5818);
        assert_eq!(descriptor.project_origin.lat, 50.603);
        assert_eq!(descriptor.positions.len(), 2);
        assert_eq!(descriptor.positions[0].id, "p_institut_de_botanique_ulg_liege");
    }
}
//#endregion 🧪️Tests
