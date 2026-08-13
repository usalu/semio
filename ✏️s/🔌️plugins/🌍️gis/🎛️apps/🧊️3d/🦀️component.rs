//! ⛰️ GIS 3D play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the World3d
//! viewport in `🎭️modes/👁️view/🪟️windows/🏔️terrain`, view state in `🦀️config.rs`, fixture-scenery
//! compute in `crate::artifacts::gisterrain::schema::inferences` (`parse_descriptor`), and this app's
//! typed media I/O surface (`map:in` overlay, ports, scene media) below in `🔖️Io` — relocated from
//! the artifact's `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).

use crate::apps::gis3d::commands::{exaggeration, locale, selection, view};
use crate::apps::gis3d::config::{Gis3dConfig, Gis3dConfigMutation};
use crate::apps::gis3d::modes::view as view_mode;
use crate::apps::gis3d::modes::view::windows::terrain;
use crate::artifacts::gisterrain::schema::default_terrain_document;
use crate::artifacts::gisterrain::op::GisTerrainMutation;
use crate::artifacts::gisterrain::{GisTerrainSnapshot, GIS_3D_TERRAIN_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, 
    ui_text, App, AppIo, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, UiNode,
};
use store::EngineHandles;
use serde_json::Value;
use store::ArtifactPack;

//#region 🔖️Constants
pub const GIS3D_PLAY_APP_ID: &str = "gis3d-play";
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this app's typed media I/O surface
/// (`AppDefinition.io`), plus the two app-specific workflow ports
/// (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe): `map:in`
/// (a `2d.map` producer — gis2d's `map:out` — feeds an overlay pin layer, see
/// `GisTerrainSnapshot::imported_features_json`) and `scene:out` (this terrain as `3d.mesh`).
/// `document_media_type` is Data×Value (the document is a scalar "exaggeration + imported overlay"
/// record, not itself mesh geometry — `scene:out` is the actual renderable mesh/terrain surface).
pub fn gis3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: GIS_3D_TERRAIN_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
        ports: vec![gis3d_map_in_port(), gis3d_scene_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "gis.terrain".into(), name: "GIS Terrain".into(), dimension: "3d".into(), component_kind: "gisterrain".into() },
    }
}

/// 🔌️ `map:in` — a `2d.map` producer (gis2d's `map:out`) feeding an overlay pin layer into this
/// terrain (see `GisTerrainSnapshot::imported_features_json`). `One`/optional: exactly one map may
/// be draped onto a terrain at a time, and a terrain with no upstream edge is valid.
pub fn gis3d_map_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "map:in".into(),
        label: "Map".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        kind_id: Some("2d.map".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `scene:out` — this terrain as `3d.mesh` (kind already registered by lowpoly; reused verbatim,
/// not redeclared — WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). `Many`/optional: several
/// downstream consumers may fan out from one terrain, and a terrain with no downstream edge is valid.
pub fn gis3d_scene_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "scene:out".into(),
        label: "Scene".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        kind_id: Some("3d.mesh".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🎞️ `scene:out`'s `Media` value. First pass (mirrors this app's own "deliberately minimal" module
/// doc): gis3d has no CPU-side heightmap tessellator yet (rendering is scene-descriptor driven, see the
/// 🏔️terrain window's `render`/`build_terrain_scene_json`), so this exports the same terrain descriptor
/// fields (exaggeration + imported overlay) as a structured `3d.mesh` payload rather than a real
/// triangulated mesh — an honest placeholder for the day a tessellator lands, not a silent fake.
pub fn gis3d_scene_media(document: &GisTerrainSnapshot) -> semio_framework_plugin::Media {
    semio_framework_plugin::Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        payload: semio_framework_plugin::MediaPayload::Structured {
            schema: "3d.mesh".into(),
            json: serde_json::json!({
                "exaggeration": document.exaggeration,
                "importedFeatures": serde_json::from_str::<Value>(&document.imported_features_json).unwrap_or(serde_json::json!(null)),
            })
            .to_string(),
        },
    }
}
//#endregion 🔖️Io

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Gis3dPlayApp::Command` — the SOLE dispatch surface for gis3d's own behavior, covering every
    /// action `create_gis3d_app` declares. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Gis3dCommand for GisTerrainSnapshot, GisTerrainMutation, Gis3dConfig, Gis3dConfigMutation {
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

impl ArtifactApp for Gis3dPlayApp {
    type Snapshot = GisTerrainSnapshot;
    type Mutation = GisTerrainMutation;
    type Config = Gis3dConfig;
    type ConfigMutation = Gis3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::apps::gis3d::presence::Gis3dPresence;
    type PresenceMutation = crate::apps::gis3d::presence::Gis3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Gis3dCommand;

    const APP_ID: &'static str = GIS3D_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = GIS_3D_TERRAIN_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::apps::gis3d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> GisTerrainSnapshot {
        default_terrain_document()
    }

    /// 🔌️ `map:in`/`scene:out` (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) plus the implicit
    /// document ports.
    fn io() -> Option<AppIo> {
        Some(gis3d_io())
    }

    /// 🎞️ `scene:out` (see `gis3d_scene_media` in `🔖️Io` above) plus the inherited
    /// `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, GisTerrainSnapshot>) -> Result<Media, MediaError> {
        match port {
            "scene:out" => Ok(gis3d_scene_media(doc.snapshot)),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `map:in` writes the incoming `2d.map` descriptor JSON verbatim into
    /// `GisTerrainSnapshot::imported_features_json` (rendered as an extra pin layer, see the
    /// 🏔️terrain window) via `change-imported-features`. `document:in` (whole-document replace) is
    /// deliberately unimplemented — per the semantic-mutations taxonomy, whole-document replace has
    /// no in-history mutation; it goes through `ArtifactStore::reset` (file-open/import/load-example),
    /// entirely outside this method.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, GisTerrainSnapshot>) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "map:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "map:in only accepts a Structured JSON payload".into()));
                };
                use crate::artifacts::gisterrain::mutations::change_imported_features::mutation::ChangeImportedFeatures;
                Ok(Emit::mutations(vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: json.clone() })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Gis3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Gis3dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly. Mirrors `crate::apps::gis2d`'s arg-key tolerance (camelCase + snake_case + the
    /// nested `camera` object form).
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
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

    fn handle(command: &Gis3dCommand, doc: &ArtifactView<'_, GisTerrainSnapshot>, cfg: &ConfigView<'_, Gis3dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🧮️ Empty — gis3d's `Config` is session view state (camera/selection), not a user-facing
    /// settings record; `ConfigSpec::empty()` (the trait default) is correct as-is.
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::empty()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, GisTerrainSnapshot>, cfg: &ConfigView<'_, Gis3dConfig>) -> UiNode {
        match body_key {
            terrain::GIS3D_PLAY_BODY_COMPOSITE => terrain::render(doc.snapshot, cfg.snapshot),
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
            // 🧱️ `.artifact_kind(mesh_artifact_kind())` REMOVED — `3d.mesh` duplicate kind deleted
            // repo-wide (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`); mesh is now
            // canonically `s.stdio.semio@v1/mesh`, composed via `GisTerrainSnapshot.mesh`.
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
            .mutation("setExaggeration", LocalizedLabel::native("Set Exaggeration", "Überhöhung festlegen"))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Gis3dPlayApp::config_spec())
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
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Gis3dApp = VcsArtifactApp<Gis3dPlayApp>;

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
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use semio_framework_plugin::ArtifactApp;
    use super::*;
    use crate::apps::gis3d::testkit::{app, app_with_registry, dispatch, render};
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
            store::os_store::test_support::assert_op_text_binary_equivalence(command);
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
    /// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
    /// `command_id`. Uses the framework's own harness, which stages each action's declared args and
    /// knows the framework-injected ids to skip (`undo`/`copy`/`recordTutorial`/…).
    ///
    /// 🩹️ This is the test that would have caught the pre-migration gap: `gis3d_ui` had NO
    /// `command_from_action` override, so every declared action fell through to the trait default's
    /// hard error and the whole `{action,args}` host wire was dead.
    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<Gis3dPlayApp>(create_gis3d_app);
        assert!(Gis3dPlayApp::command_from_action("noSuchAction", None).is_err());
    }

    #[test]
    fn command_from_action_reads_the_nested_camera_object_and_both_id_key_spellings() {
        let app = Gis3dPlayApp;
        let camera = Gis3dPlayApp::command_from_action("setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0] } }))).expect("setCamera");
        assert!(matches!(camera, Gis3dCommand::SetCamera(ref payload) if payload.camera_json.contains("position")));
        let selection = Gis3dPlayApp::command_from_action("worldSelect", Some(&json!({ "selectedIds": ["p1"] }))).expect("worldSelect");
        assert!(matches!(selection, Gis3dCommand::WorldSelect(ref payload) if payload.ids == vec!["p1".to_string()]));
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_gis3d_app().definition;
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 1);
        // 🧷️ gis3d declares no app panel tabs of its own; whatever is present comes from the framework.
        assert!(!definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref().is_some_and(|key| key.starts_with("gis3d.play."))), "gis3d declares no app panels");
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "2d.map"));
        // 🧱️ `3d.mesh` is NO LONGER independently registered here (ticket
        // `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — duplicate `ArtifactKindSpec` deleted, see
        // `crate::artifacts::gisterrain::🦀️component.rs`'s removal comment). `scene:out`'s
        // `kind_id: Some("3d.mesh".into())` media-port tag (asserted separately below) still
        // references the canonical kind by id; this manifest just no longer redundantly declares it.
        assert!(!definition.artifact_kinds.iter().any(|kind| kind.id == "3d.mesh"), "3d.mesh is composed via GisTerrainSnapshot.mesh now, never a standalone ArtifactKindSpec");
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = app();
        assert!(render(&mut app, "gis3d.play.nope").contains("Unknown body"));
    }

    #[test]
    fn view_actions_emit_no_ops_under_registry_kind_discipline() {
        let mut app = app_with_registry();
        assert!(dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: "{}".into() })).mutations.is_empty());
        assert!(dispatch(&mut app, Gis3dCommand::WorldSelect(world_select::WorldSelect { ids: vec!["p1".into()] })).mutations.is_empty());
        assert_eq!(dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.0 })).mutations.len(), 1);
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Media
    #[test]
    fn export_media_scene_out_produces_a_3d_mesh_structured_payload() {
        let app = app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let media = Gis3dPlayApp::export_media("scene:out", &doc).expect("scene:out export");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "3d.mesh");
        assert!(json.contains("exaggeration"));
    }

    #[test]
    fn import_media_map_in_writes_the_imported_features_operation() {
        let app = app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let incoming = json!({ "positions": [{ "id": "imported-1", "lon": 1.0, "lat": 2.0 }] }).to_string();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.map".into(), json: incoming.clone() } };
        let emit = Gis3dPlayApp::import_media("map:in", &media, &doc).expect("map:in import");
        use crate::artifacts::gisterrain::mutations::change_imported_features::mutation::ChangeImportedFeatures;
        assert_eq!(emit.artifact_mutations, vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: incoming })]);
    }

    #[test]
    fn media_ports_declare_map_in_and_scene_out() {
        let app = Gis3dPlayApp;
        let ports = Gis3dPlayApp::media_ports();
        assert!(ports.iter().any(|port| port.id == "map:in"));
        assert!(ports.iter().any(|port| port.id == "scene:out"));
    }

    /// 🧭️ Relocated from the artifact's `⚙️engine` tests (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) alongside `gis3d_io`/`gis3d_scene_media`.
    #[test]
    fn gis3d_io_declares_the_map_in_and_scene_out_ports() {
        let io = gis3d_io();
        assert_eq!(io.document_schema, GIS_3D_TERRAIN_SCHEMA);
        let ports = io.all_ports();
        let map_in = ports.iter().find(|port| port.id == "map:in").expect("map:in declared");
        assert_eq!(map_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert_eq!(map_in.kind_id.as_deref(), Some("2d.map"));
        let scene_out = ports.iter().find(|port| port.id == "scene:out").expect("scene:out declared");
        assert_eq!(scene_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(scene_out.kind_id.as_deref(), Some("3d.mesh"));
    }

    #[test]
    fn gis3d_scene_media_exports_the_terrain_descriptor() {
        let document = default_terrain_document();
        let media = gis3d_scene_media(&document);
        let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else {
            panic!("expected a structured scene:out payload");
        };
        assert_eq!(schema, "3d.mesh");
        assert!(json.contains("exaggeration"));
    }
    //#endregion 🔖️Media
}
//#endregion 🧪️Tests
