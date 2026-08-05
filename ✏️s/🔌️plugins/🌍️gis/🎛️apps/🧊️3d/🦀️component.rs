//! ⛰️ GIS 3D play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the World3d
//! viewport in `🎭️modes/👁️view/🪟️windows/🏔️terrain`, view state in `🦀️config.rs`, and document-side
//! compute (fixture scenery, `map:in` overlay, ports, scene media) in
//! `crate::artifacts::gisterrain::engine`.

use crate::apps::gis3d::commands::{exaggeration, locale, selection, view};
use crate::apps::gis3d::config::{Gis3dConfig, Gis3dConfigOperation};
use crate::apps::gis3d::modes::view as view_mode;
use crate::apps::gis3d::modes::view::windows::terrain;
use crate::artifacts::gisterrain::engine::{default_terrain_document, gis3d_io, gis3d_map_in_port, gis3d_scene_media, gis3d_scene_out_port};
use crate::artifacts::gisterrain::op::Gis3dTerrainOperation;
use crate::artifacts::gisterrain::{mesh_artifact_kind, Gis3dTerrainDocument, GIS_3D_TERRAIN_SCHEMA};
use semio_framework_plugin::{
    ui_text, App, AppIo, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, UiNode,
};
use serde_json::Value;
use store::DocumentPack;

//#region 🔖️Constants
pub const GIS3D_PLAY_APP_ID: &str = "gis3d-play";
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Gis3dPlayApp::Command` — the SOLE dispatch surface for gis3d's own behavior, covering every
    /// action `create_gis3d_app` declares. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Gis3dCommand for Gis3dTerrainDocument, Gis3dTerrainOperation, Gis3dConfig, Gis3dConfigOperation {
        "setExaggeration" as "exaggeration" => set_exaggeration::SetExaggeration,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setSelection" as "selection" => set_selection::SetSelection,
        "worldSelect" as "world-select" => world_select::WorldSelect,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier.
use exaggeration::set_exaggeration;
use locale::set_locale;
use selection::{set_selection, world_select};
use view::set_camera;
//#endregion 🔖️Commands

//#region 🔖️Gis3dPlayApp
/// ⛰️ GIS 3D terrain play app. The document holds exaggeration plus the `map:in` overlay layer;
/// the camera and pin selection are [`Gis3dConfig`] — session-only but real, undoable config state.
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

    /// 🎞️ `scene:out` (see `crate::artifacts::gisterrain::engine::gis3d_scene_media`) plus the inherited
    /// `document:out` default (the pack of `doc.projection`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Gis3dTerrainDocument>) -> Result<Media, MediaError> {
        match port {
            "scene:out" => Ok(gis3d_scene_media(doc.projection)),
            "document:out" => {
                let media_type = self.io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `map:in` writes the incoming `2d.map` descriptor JSON verbatim into
    /// `Gis3dTerrainDocument::imported_features_json` (rendered as an extra pin layer, see the
    /// 🏔️terrain window) plus the inherited `document:in` default (replicated inline for the same
    /// reason as `export_media`).
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

    fn command_id(&self, command: &Gis3dCommand) -> &str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Gis3dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly. Mirrors `crate::apps::gis2d`'s arg-key tolerance (camelCase + snake_case + the
    /// nested `camera` object form).
    fn command_from_action(&self, action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(Value::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let ids = || -> Vec<String> {
            let ids = string_list("ids");
            if ids.is_empty() {
                string_list("selectedIds")
            } else {
                ids
            }
        };
        match action {
            "setExaggeration" => Ok(Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration {
                exaggeration: ["exaggeration", "value"].iter().find_map(|key| args.get(key).and_then(Value::as_f64)).unwrap_or(1.0),
            })),
            "setCamera" => {
                let camera_json = str_arg(&["cameraJson", "camera_json"])
                    .or_else(|| args.get("camera").map(|value| if value.is_string() { value.as_str().unwrap_or("{}").to_string() } else { value.to_string() }))
                    .unwrap_or_else(|| "{}".into());
                Ok(Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json }))
            }
            "setSelection" => Ok(Gis3dCommand::SetSelection(set_selection::SetSelection { ids: ids() })),
            "worldSelect" => Ok(Gis3dCommand::WorldSelect(world_select::WorldSelect { ids: ids() })),
            "setLocale" => Ok(Gis3dCommand::SetLocale(set_locale::SetLocale { value: str_arg(&["value", "locale"]).unwrap_or_default() })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(&self, command: &Gis3dCommand, doc: &DocumentView<'_, Gis3dTerrainDocument>, cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<Gis3dTerrainOperation, Gis3dConfigOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🧮️ Empty — gis3d's `Config` is session view state (camera/selection), not a user-facing
    /// settings record; `ConfigSpec::empty()` (the trait default) is correct as-is.
    fn config_spec(&self) -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::empty()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Gis3dTerrainDocument>, cfg: &ConfigView<'_, Gis3dConfig>) -> UiNode {
        match body_key {
            terrain::GIS3D_PLAY_BODY_COMPOSITE => terrain::render(doc.projection, cfg.projection),
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
            // Wave 2 port recipe) — the canonical declaration is the gismap artifact's;
            // identical-shape duplicates are harmless (registry dedupes by id).
            .artifact_kind(crate::artifacts::gismap::artifact_kind())
            .artifact_kind(mesh_artifact_kind())
            .media_input(gis3d_map_in_port())
            .media_output(gis3d_scene_out_port())
            .icon_id("gis3d")
            .mode_def(view_mode::definition())
            .default_mode_id(view_mode::GIS3D_PLAY_MODE_VIEW)
            .window_kind_def(terrain::definition())
            .default_layout(view_mode::layout())
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("worldSelect", LocalizedLabel::native("Select", "Auswählen"))
            .operation("setExaggeration", LocalizedLabel::native("Set Exaggeration", "Überhöhung festlegen"))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Gis3dPlayApp::default().config_spec())
            .io(gis3d_io()),
    )
    .example("reuse-terrain", LocalizedLabel::native("Reuse Terrain", "Gelände wiederverwenden"), serde_json::to_string(&default_terrain_document()).unwrap_or_default(), "file-text")
    .workflow("gis3d", "GIS 3D", "terrain")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewState};

    pub type Gis3dApp = VcsDocumentApp<Gis3dPlayApp>;

    pub fn app() -> Gis3dApp {
        new_app::<Gis3dPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View/Shell-emits-operations rejection) runs.
    pub fn app_with_registry() -> Gis3dApp {
        new_app_with_registry::<Gis3dPlayApp>(create_gis3d_app)
    }

    pub fn dispatch(app: &mut Gis3dApp, command: Gis3dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Gis3dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis3d::testkit::{app, app_with_registry, dispatch, render};
    use semio_framework_plugin::{ActionKind, PluginApp};
    use serde_json::json;

    //#region 🔖️CommandSurface
    /// 🎯️ One value per `app_commands!` row, in row order.
    fn every_command() -> Vec<Gis3dCommand> {
        vec![
            Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.5 }),
            Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() }),
            Gis3dCommand::SetSelection(set_selection::SetSelection { ids: vec!["p1".into()] }),
            Gis3dCommand::WorldSelect(world_select::WorldSelect { ids: vec!["p1".into()] }),
            Gis3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// 🏷️ The wire keyword each row prints under — the kebab `as` literal, independent of the camelCase
    /// manifest action id.
    const WIRE_KEYWORDS: &[&str] = &["exaggeration", "camera", "selection", "world-select", "locale"];

    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Gis3dCommand::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 5, "every Gis3dCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
        assert_eq!(every_command().len(), WIRE_KEYWORDS.len());
        for (command, keyword) in every_command().iter().zip(WIRE_KEYWORDS) {
            store::test_support::assert_op_text_binary_equivalence(command);
            let printed = protocol::OpText::print_op(command);
            assert!(printed == *keyword || printed.starts_with(&format!("{keyword} ")), "row {} printed {printed:?}, expected the {keyword:?} wire keyword", command.command_id());
        }
    }

    /// 🧷️ Pins the exact pre-migration bytes for every row. Hex copied verbatim from the pre-migration
    /// baseline dump (ticket `26/08/05/GIS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
    /// `🧪️wire-baseline-3d-before.txt`).
    #[test]
    fn every_row_keeps_its_pre_migration_bytes() {
        let hex = |command: &Gis3dCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(hex(&Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.5 })), "0100000100050000000000000440");
        assert_eq!(hex(&Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() })), "0101011a7b22706f736974696f6e223a5b312e302c322e302c332e305d7d01000600");
        assert_eq!(hex(&Gis3dCommand::SetSelection(set_selection::SetSelection { ids: vec!["p1".into()] })), "01020102703101000c010600");
        assert_eq!(hex(&Gis3dCommand::SetSelection(set_selection::SetSelection { ids: Vec::new() })), "01020001000c00");
        assert_eq!(hex(&Gis3dCommand::WorldSelect(world_select::WorldSelect { ids: vec!["p1".into()] })), "01030102703101000c010600");
        assert_eq!(hex(&Gis3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })), "0104010564652d444501000600");
    }

    /// 🎯️ Every declared action maps to a typed command. The pre-migration `gis3d_ui` crate had NO
    /// `command_from_action` override at all — it inherited the trait default, which errors for every
    /// action, so the whole `{action,args}` host wire was dead. That crate never compiled (see the
    /// migration ticket), which is why the gap was invisible; this test locks the fix in.
    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        let app = Gis3dPlayApp;
        for action in create_gis3d_app().definition.actions.iter().filter(|action| !matches!(action.kind, ActionKind::Framework)) {
            let command = app.command_from_action(&action.id, None).unwrap_or_else(|error| panic!("action {} must map to a command: {error:?}", action.id));
            assert_eq!(command.command_id(), action.id);
        }
        assert!(app.command_from_action("noSuchAction", None).is_err());
    }

    #[test]
    fn command_from_action_reads_the_nested_camera_object_and_both_id_key_spellings() {
        let app = Gis3dPlayApp;
        let camera = app.command_from_action("setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0] } }))).expect("setCamera");
        assert!(matches!(camera, Gis3dCommand::SetCamera(ref payload) if payload.camera_json.contains("position")));
        let selection = app.command_from_action("worldSelect", Some(&json!({ "selectedIds": ["p1"] }))).expect("worldSelect");
        assert!(matches!(selection, Gis3dCommand::WorldSelect(ref payload) if payload.ids == vec!["p1".to_string()]));
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_gis3d_app().definition;
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 1);
        assert!(definition.panel_tabs.is_empty(), "gis3d declares no app panels");
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "2d.map"));
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "3d.mesh"));
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = app();
        assert!(render(&mut app, "gis3d.play.nope").contains("Unknown body"));
    }

    #[test]
    fn view_actions_emit_no_ops_under_registry_kind_discipline() {
        let mut app = app_with_registry();
        assert!(dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: "{}".into() })).operations.is_empty());
        assert!(dispatch(&mut app, Gis3dCommand::WorldSelect(world_select::WorldSelect { ids: vec!["p1".into()] })).operations.is_empty());
        assert_eq!(dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.0 })).operations.len(), 1);
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Media
    #[test]
    fn export_media_scene_out_produces_a_3d_mesh_structured_payload() {
        let app = app();
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
        let app = app();
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
    //#endregion 🔖️Media
}
//#endregion 🧪️Tests
