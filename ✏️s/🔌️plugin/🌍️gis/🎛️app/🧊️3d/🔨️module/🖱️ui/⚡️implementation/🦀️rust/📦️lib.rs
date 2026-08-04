//! 🖥️ GIS 3D app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! migration — `Gis3dPlayApp` is a unit struct; every former `Gis3dPlayRuntime` field (camera,
//! selection) now lives in `gis3d_engine::Gis3dConfig`, written via `gis3d_op::Gis3dConfigOperation`s
//! (real `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `gis3d_protocol::Gis3dCommand` channel via `DocumentApp::handle`.
//!
//! ⛰️ Reuses the existing `World3d` viewport/renderer rather than a bespoke one; deliberately
//! read-mostly for this first pass — exaggeration and the `map:in` overlay layer are the only
//! editable/undoable document state (see `gis3d::Gis3dTerrainDocument`).

use framework_surface_terrain::{build_terrain_scene_json, projection, TerrainDescriptorJson};
use gis3d::{Gis3dTerrainDocument, GIS_3D_TERRAIN_SCHEMA};
use gis3d_dsl::REUSE_TERRAIN_EXAMPLE_TEXT;
use gis3d_engine::{default_terrain_document, gis3d_io, gis3d_map_in_port, gis3d_scene_media, gis3d_scene_out_port, Gis3dConfig};
use gis3d_op::{Gis3dConfigOperation, Gis3dTerrainOperation};
use gis3d_protocol::Gis3dCommand;
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, ui_text, world3d_scene_extended, world3d_selection_json, App, AppIo, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm,
    MediaPayload, MediaType, OsMediaCapability, SurfaceKind, UiNode,
};
use serde_json::{json, Value};
use store::DocumentPack;

//#region 🔖️Constants
const GIS3D_PLAY_APP_ID: &str = "gis3d-play";
const GIS3D_PLAY_SURFACE: &str = "gis3d.play.composite";
const GIS3D_PLAY_BODY_COMPOSITE: &str = "gis3d.play.composite";
const GIS3D_PLAY_WINDOW_MAIN: &str = "gis3d-main";
//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
/// 📜️ Hand-rolled reader for the `.gisterrain` fixture's `origin`/`position` scenery lines — the
/// read-only pins/project-origin data rendered alongside the document (see module docs); the
/// `gisterrain exaggeration=...` header line those same files start with is instead read by
/// `Gis3dTerrainDocument`'s own derive-generated `DocumentDsl` (see `gis3d::Gis3dTerrainDocument`),
/// since exaggeration is undoable document state.
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

/// 🔌️ `map:in`'s overlay pin layer (see `Gis3dTerrainDocument::imported_features_json`), decoded from
/// its `{positions:[{id,lon,lat,label?,icon?}]}` descriptor JSON — malformed/empty JSON (including the
/// default empty string) simply contributes no extra pins.
fn imported_positions(document: &Gis3dTerrainDocument) -> Vec<framework_surface_terrain::TerrainPositionData> {
    let Ok(value) = serde_json::from_str::<Value>(&document.imported_features_json) else {
        return Vec::new();
    };
    let Some(positions) = value.get("positions").and_then(|value| value.as_array()) else {
        return Vec::new();
    };
    positions
        .iter()
        .filter_map(|entry| {
            Some(framework_surface_terrain::TerrainPositionData {
                id: entry.get("id").and_then(|value| value.as_str())?.to_string(),
                lon: entry.get("lon").and_then(|value| value.as_f64())?,
                lat: entry.get("lat").and_then(|value| value.as_f64())?,
                label: entry.get("label").and_then(|value| value.as_str()).map(str::to_string),
                icon: entry.get("icon").and_then(|value| value.as_str()).map(str::to_string),
            })
        })
        .collect()
}

/// 🏔️ The full rendering descriptor (project origin + fixture pins + `map:in` overlay pins +
/// exaggeration) for the given document — `exaggeration` always mirrors the LIVE document, and the
/// bundled fixture's own `gisterrain exaggeration=...` header only ever seeds it once via
/// {@link gis3d_engine::default_terrain_document}.
fn parse_descriptor(document: &Gis3dTerrainDocument) -> TerrainDescriptorJson {
    let mut descriptor = terrain_fixture_text::parse_descriptor(REUSE_TERRAIN_EXAMPLE_TEXT, GIS_3D_TERRAIN_SCHEMA, document.exaggeration);
    descriptor.positions.extend(imported_positions(document));
    descriptor
}
//#endregion 🔖️DocumentHelpers

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

fn render_canvas(document: &Gis3dTerrainDocument, cfg: &Gis3dConfig) -> UiNode {
    let descriptor = parse_descriptor(document);
    let mut scene =
        world3d_scene_extended(cfg.camera_json.clone(), "[]".into(), instances_json(&descriptor), world3d_selection_json("rectangle", &cfg.selected_ids, None), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None);
    scene.terrain_json = Some(build_terrain_scene_json(&descriptor));
    build_world_3d_scene(GIS3D_PLAY_SURFACE, GIS3D_PLAY_APP_ID, scene)
}
//#endregion 🔖️Render

//#region 🔖️Gis3dPlayApp
/// 🧪️ B1: unit struct — every former `Gis3dPlayRuntime` field now lives in `gis3d_engine::Gis3dConfig`
/// (see `DocumentApp::Config`), written through `gis3d_op::Gis3dConfigOperation`s.
#[derive(Default)]
pub struct Gis3dPlayApp;

impl DocumentApp for Gis3dPlayApp {
    type Projection = Gis3dTerrainDocument;
    type Operation = Gis3dTerrainOperation;
    type Config = Gis3dConfig;
    type ConfigOperation = Gis3dConfigOperation;
    type Command = Gis3dCommand;

    fn app_id(&self) -> &str {
        GIS3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        GIS_3D_TERRAIN_SCHEMA
    }

    fn initial_projection(&self) -> Gis3dTerrainDocument {
        default_terrain_document()
    }

    /// 🔌️ `map:in`/`scene:out` (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) plus the implicit
    /// document ports.
    fn io(&self) -> Option<AppIo> {
        Some(gis3d_io())
    }

    fn whole_document_operation(&self, projection: Gis3dTerrainDocument) -> Option<Gis3dTerrainOperation> {
        Some(Gis3dTerrainOperation::SetDocument { document: projection })
    }

    /// 🎞️ `scene:out` (see `gis3d_engine::gis3d_scene_media`) plus the inherited `document:out`
    /// default (the pack of `doc.projection`, replicated inline — overriding `export_media` shadows the
    /// trait's provided body for every port on this app, not just the new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Gis3dTerrainDocument>) -> Result<Media, MediaError> {
        match port {
            "scene:out" => Ok(gis3d_scene_media(doc.projection)),
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `map:in` writes the incoming `2d.map` descriptor JSON verbatim into
    /// `Gis3dTerrainDocument::imported_features_json` (see `parse_descriptor`/`imported_positions` —
    /// rendered as an extra pin layer) plus the inherited `document:in` default (replicated inline for
    /// the same reason as `export_media`).
    fn import_media(&self, port: &str, media: &Media, _doc: &DocumentView<'_, Gis3dTerrainDocument>) -> Result<Emit<Gis3dTerrainOperation, Gis3dConfigOperation>, MediaError> {
        match port {
            "map:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "map:in only accepts a Structured JSON payload".into()));
                };
                Ok(Emit::operations(vec![Gis3dTerrainOperation::SetImportedFeatures { features_json: json.clone() }]))
            }
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let projection = <Gis3dTerrainDocument as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match self.whole_document_operation(projection) {
                    Some(operation) => Ok(Emit::operations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `Gis3dCommand` variant back to the action id it was declared under in
    /// `create_gis3d_app`. `SetLocale` is not palette-declared (host/test infra dispatches it
    /// directly, mirroring `gis2d_ui::Gis2dPlayApp::command_id`).
    fn command_id(&self, command: &Gis3dCommand) -> &str {
        match command {
            Gis3dCommand::SetExaggeration { .. } => "setExaggeration",
            Gis3dCommand::SetCamera { .. } => "setCamera",
            Gis3dCommand::SetSelection { .. } => "setSelection",
            Gis3dCommand::WorldSelect { .. } => "worldSelect",
            Gis3dCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &Gis3dCommand, _doc: &DocumentView<'_, Gis3dTerrainDocument>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<Gis3dTerrainOperation, Gis3dConfigOperation>, Fault> {
        match command {
            Gis3dCommand::SetCamera { camera_json } => Ok(Emit::config(vec![Gis3dConfigOperation::SetCamera { camera_json: camera_json.clone() }]),
            Gis3dCommand::SetSelection { ids } | Gis3dCommand::WorldSelect { ids } => Ok(Emit::config(vec![Gis3dConfigOperation::SetSelection { ids: ids.clone() }]),
            Gis3dCommand::SetExaggeration { exaggeration } => Ok(Emit::amend(vec![Gis3dTerrainOperation::SetExaggeration { exaggeration: *exaggeration }], "gis3d-exaggeration"),
            Gis3dCommand::SetLocale { value } => Ok(Emit::config(vec![Gis3dConfigOperation::SetLocale { value: value.clone() }]),
        }
    }

    /// 🧮️ Empty — gis3d's `Config` is session view state (camera/selection), not a user-facing
    /// settings record; `ConfigSpec::empty()` (the trait default) is correct as-is.
    fn config_spec(&self) -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::empty()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Gis3dTerrainDocument>, cfg: &ConfigView<'_, Gis3dConfig>) -> UiNode {
        match body_key {
            GIS3D_PLAY_BODY_COMPOSITE => render_canvas(doc.projection, cfg.projection),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Gis3dPlayApp

//#region 🔖️Manifest
pub fn create_gis3d_app() -> App {
    App::from_builder(
        App::builder(GIS3D_PLAY_APP_ID, LocalizedLabel::native("GIS 3D", "GIS 3D"))
            .document(["semio", "gis", "3d"])
            // 🔌️ Declared for clarity on both sides of the `map:in` edge (WORKFLOWS-END-TO-END-TYPED-PORTS
            // Wave 2 port recipe) — the canonical declaration is `gis2d_ui::create_gis2d_app`'s;
            // identical-shape duplicates are harmless (registry dedupes by id).
            .artifact_kind(ArtifactKindSpec {
                id: "2d.map".into(),
                name: "2D Map".into(),
                source_format: "gis.map".into(),
                component_kind: "gismap".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                schema: "gis.map".into(),
                export_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
                import_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
            })
            // 🔌️ `3d.mesh` — the interchange kind `scene:out` produces; canonically declared by
            // `lowpoly` (`mesh_from_mesh_document`'s registration) — identical-shape duplicate.
            .artifact_kind(ArtifactKindSpec {
                id: "3d.mesh".into(),
                name: "3D Mesh".into(),
                source_format: "mesh.reference".into(),
                component_kind: "mesh".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
                schema: "mesh.reference".into(),
                export_formats: vec![semio_framework_plugin::OsMediaFormat::Glb, semio_framework_plugin::OsMediaFormat::Obj, semio_framework_plugin::OsMediaFormat::Stl],
                import_formats: vec![semio_framework_plugin::OsMediaFormat::Glb, semio_framework_plugin::OsMediaFormat::Obj],
            })
            .media_input(gis3d_map_in_port())
            .media_output(gis3d_scene_out_port())
            .icon_id("gis3d")
            .mode("view", LocalizedLabel::native("View", "Ansicht"), "eye")
            .default_mode_id("view")
            .window_kind(GIS3D_PLAY_WINDOW_MAIN, LocalizedLabel::native("Terrain", "Gelände"), GIS3D_PLAY_BODY_COMPOSITE, SurfaceKind::World3d, "terrain-3d")
            .default_layout(create_default_layout(&[GIS3D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Terrain".into()])))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("worldSelect", LocalizedLabel::native("Select", "Auswählen"))
            .operation("setExaggeration", LocalizedLabel::native("Set Exaggeration", "Überhöhung festlegen"))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Gis3dPlayApp::default().config_spec())
            .io(gis3d_io()),
    )
    .example("reuse-terrain", LocalizedLabel::native("Reuse Terrain", "Gelände wiederverwenden"), serde_json::to_string(&default_terrain_document()).unwrap(), "file-text")
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
    fn camera_and_selection_are_config_state_and_emit_no_operations() {
        let mut app = new_app();
        let camera = app.dispatch_typed(Gis3dCommand::SetCamera { camera_json: json!({ "position": [1.0, 1.0, 1.0] }).to_string() }, &testkit::meta("local")).expect("setCamera");
        assert!(camera.operations.is_empty(), "camera is ephemeral config state");
        let selection = app.dispatch_typed(Gis3dCommand::WorldSelect { ids: vec!["p_institut_de_botanique_ulg_liege".into()] }, &testkit::meta("local")).expect("worldSelect");
        assert!(selection.operations.is_empty(), "selection is ephemeral config state");
    }

    /// 🧪️ A slider drag is many `setExaggeration` ticks sharing one coalesce key: they fold into ONE
    /// undoable edit, so a single undo restores the fixture's exaggeration rather than a mid-drag value.
    #[test]
    fn exaggeration_drag_coalesces_into_one_undo_step() {
        let mut app = new_app();
        for value in [2.0, 2.5, 3.0] {
            app.dispatch_typed(Gis3dCommand::SetExaggeration { exaggeration: value }, &testkit::meta("local")).expect("drag tick");
        }
        assert_eq!(app.projection().expect("projection").exaggeration, 3.0);
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").exaggeration, 1.5, "one coalesced edit: undo restores the fixture exaggeration");
    }

    /// 📜️ The `.gisterrain` fixture's `gisterrain exaggeration=...` header is parsed twice for two
    /// different purposes (see `parse_descriptor`/`gis3d_engine::default_terrain_document`'s docs); this
    /// proves the scenery-data reader (`terrain_fixture_text`) still recovers the bundled fixture's
    /// pins/origin after the document-only conversion — i.e. converting the fixture to the DSL didn't
    /// lose data.
    #[test]
    fn terrain_fixture_text_recovers_bundled_scenery_data() {
        let descriptor = parse_descriptor(&Gis3dTerrainDocument { exaggeration: 1.5, imported_features_json: String::new() });
        assert_eq!(descriptor.project_origin.lon, 5.5818);
        assert_eq!(descriptor.project_origin.lat, 50.603);
        assert_eq!(descriptor.positions.len(), 2);
        assert_eq!(descriptor.positions[0].id, "p_institut_de_botanique_ulg_liege");
    }

    /// 🔌️ `map:in`'s overlay layer renders as extra pins alongside the fixture's own two.
    #[test]
    fn imported_map_features_render_as_extra_pins() {
        let document = Gis3dTerrainDocument { exaggeration: 1.5, imported_features_json: json!({ "positions": [{ "id": "imported-1", "lon": 5.58, "lat": 50.60 }] }).to_string() };
        let descriptor = parse_descriptor(&document);
        assert_eq!(descriptor.positions.len(), 3, "2 fixture pins + 1 imported pin");
        assert!(descriptor.positions.iter().any(|position| position.id == "imported-1"));
    }

    #[test]
    fn export_media_scene_out_produces_a_3d_mesh_structured_payload() {
        let app = new_app();
        let document = app.projection().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = Gis3dPlayApp.export_media("scene:out", &doc).expect("scene:out export");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "3d.mesh");
        assert!(json.contains("exaggeration"));
    }

    #[test]
    fn import_media_map_in_writes_the_imported_features_operation() {
        let app = new_app();
        let document = app.projection().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let incoming = json!({ "positions": [{ "id": "imported-1", "lon": 1.0, "lat": 2.0 }] }).to_string();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.map".into(), json: incoming.clone() } };
        let emit = Gis3dPlayApp.import_media("map:in", &media, &doc).expect("map:in import");
        assert_eq!(emit.document_operations, vec![Gis3dTerrainOperation::SetImportedFeatures { features_json: incoming }]);
    }

    #[test]
    fn media_ports_declare_map_in_and_scene_out() {
        let app = Gis3dPlayApp;
        let ports = app.media_ports();
        assert!(ports.iter().any(|port| port.id == "map:in"));
        assert!(ports.iter().any(|port| port.id == "scene:out"));
    }

    /// 🗣️ `SetLocale` is not palette-declared but still dispatches cleanly end-to-end (command_id
    /// mapping → `handle` → config store) — the same typed channel the shell uses to push locale.
    #[test]
    fn locale_command_dispatches_through_the_config_store() {
        let mut app = new_app();
        let result = app.dispatch_typed(Gis3dCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        assert!(result.operations.is_empty(), "locale is config state, not a document edit");
    }
}
//#endregion 🧪️Tests
