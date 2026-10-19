/// 🧩️ Frozen laws inspect the actual mounted production owners after dissolution.
pub(crate) fn puzzle3d_source() -> String {
    [include_str!("../../🦀️.rs"), include_str!("../../🧵️retained/🎬️action/🦀️.rs"),
        include_str!("../../🧵️retained/🗂️document/🦀️.rs"), include_str!("../../🎮️commands/🦀️.rs")].concat()
}

#[path = "../🧰️context/🦀️.rs"]
pub(crate) mod context;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_editor::*;
use semio_s_artifact_puzzle_3d_retained_action::PUZZLE3D_RETAINED_PAYLOAD_SCHEMA;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dAcceptSuggestionWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dAddBrushObjectWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dFocusSelectionWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dInstanceBinding;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dPrecomputeCommandWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dTransformWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dWindowCommandWork;
use semio_s_artifact_puzzle_3d_retained_action::puzzle3d_retained_extent;
use semio_s_artifact_puzzle_3d_retained_action::puzzle3d_retained_reduce;
use {semio_s_artifact_puzzle_3d_retained_document::Puzzle3dAddObjectKindWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dCreateAttractionWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dKindWeightWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dPatchInspectorWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dSetActiveExampleWork};
use semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action;
use {semio_s_artifact_puzzle_3d_editor_session::PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::with_puzzle3d_session_for, semio_s_artifact_puzzle_3d_editor_session::puzzle3d_view_session_key};
use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_DEFAULT_UTILITY, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_KIND, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_HOVER_CHANNEL, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass, semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_addressed_window_id, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_entity_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_fault_notices, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_action, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_argument, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_inspector_origin_nudge, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_kind_ids, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_paste_operations, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_active_utility, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_play_snapshot, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_selected_objects_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_upsert_catalog_rows, semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot};
use semio_s_artifact_puzzle_3d_commands_document::world_relocate;
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime};
use semio_s_artifact_puzzle_3d_editor_modes as edit;
use semio_s_artifact_puzzle_3d_editor_modes::tools::fill as fill_tool;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities;
use semio_s_artifact_puzzle_3d_editor_panels::catalogue;
use semio_s_artifact_puzzle_3d_editor_panels::artifact;
use semio_s_artifact_puzzle_3d_editor_panels::inspection;
use semio_s_artifact_puzzle_3d_editor_panels::settings as settings_panel;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsOwner;
use semio_s_artifact_puzzle_3d_editor_presence as presence;
use semio_s_artifact_puzzle_3d_editor_presence::Puzzle3dPresence;
use semio_s_artifact_puzzle_3d_editor_presence::Puzzle3dPresenceMutation;
use semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_labels;
use semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_localized;
use semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_localized_phrase;
use semio_s_artifact_puzzle_3d_editor_terminology::Puzzle3dLabels;
use semio_s_artifact_puzzle_3d_editor_window as window_ownership;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::ClipboardError;
use semio_framework_plugin::kernel::ClipboardFragment;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::kernel::PastePlacement;
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::StepContext;
use semio_framework_plugin::panel_tab_element_id;
use semio_framework_plugin::panel_tab_first_draggable_element_id;
use semio_framework_plugin::window_element_id;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;

use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::InteractionWrite;
use semio_framework_onboarding::IntroductionDefinition;
use semio_framework_onboarding::IntroductionInteraction;
use semio_framework_onboarding::IntroductionPlacement;
use semio_framework_onboarding::IntroductionStepDefinition;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_2d_compute::EngineHandles;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::parse, semio_framework_pack_json::to_json_string, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use semio_framework_plugin::app::EphemeralEmit;
use semio_framework_plugin::app::InteractionView;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use semio_s_artifact_puzzle_3d_commands_document::delete_selection;


use context::*;
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::PluginApp;

include!("🕹️roster-and-read-laws/🦀️.rs");
include!("🧵️cursorized-route-laws/🦀️.rs");

include!("🧮️extent-and-step-laws/🦀️.rs");

include!("🎛️dispatch-and-operation-laws/🦀️.rs");

include!("🧾️inspector-and-manifest-laws/🦀️.rs");

include!("💡️suggestion-and-scene-laws/🦀️.rs");

//#region 🔖️WindowOptions
#[semio_framework_async_macros::async_test]
async fn grid_window_options_control_one_visible_grid_spacing() {
    let mut app = app().await;
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), None).await.expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 7.5 })), None).await.expect("setGridSpacing");
    let lod = lod_of(&render_composite(&mut app).await);
    assert_eq!(lod.get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(lod.get("gridFactor").and_then(Value::as_f64), Some(7.5));
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid")), Some(None));
    assert_eq!(find_measure_slider(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(7.5));
}

/// 🪟️ Two window instances of the same kind (a split top/perspective pane pair) must never share
/// window options — toggling grid visibility in one instance must leave every other instance's
/// grid untouched, both in its measures chrome and in its own rendered scene.
#[semio_framework_async_macros::async_test]
async fn window_options_are_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut reopened = app().await;
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");
    let base_view = app.window_view(main::WINDOW_KIND_ID);
    let second_view = app.window_view(second_window);
    let document_before = projection_of(&app);
    let app_config_before = app.config_pack().await.expect("app config before exact window publications");

    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 2.5 })), Some(main::WINDOW_KIND_ID)).await.expect("setGridSpacing on base window");
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(second_window)).await.expect("setGridVisible on second window");

    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_after = app.window_measures(&view).await;
    assert_eq!(find_measure_toggle(measures_after.get(main::WINDOW_KIND_ID).expect("base measures"), &toggle_id), Some(true), "the base window instance's grid must stay visible");
    assert_eq!(find_measure_toggle(measures_after.get(second_window).expect("second measures"), &toggle_id), Some(false), "only the targeted window instance's grid toggles off");
    let base_render = render_window(&mut app, main::WINDOW_KIND_ID).await;
    let second_render = render_window(&mut app, second_window).await;
    assert_eq!(lod_of(&base_render).get("showLodGrid").and_then(Value::as_bool), Some(true));
    assert_eq!(lod_of(&base_render).get("gridFactor").and_then(Value::as_f64), Some(2.5));
    assert_eq!(lod_of(&second_render).get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(projection_of(&app), document_before);
    let app_config_after = app.config_pack().await.expect("app config after exact window publications");
    assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));
    assert_eq!(app.window_config_generation(&base_view).await.expect("base generation"), Some(1));
    assert_eq!(app.window_config_generation(&second_view).await.expect("second generation"), Some(1));
    let packs = app.window_config_packs().await.expect("exact window packs");
    assert_eq!(packs.len(), 2);
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload exact window pack");
    }
    assert_eq!(render_window(&mut reopened, main::WINDOW_KIND_ID).await, base_render);
    assert_eq!(render_window(&mut reopened, second_window).await, second_render);
    assert!(close_witness(&mut reopened).expect("reopened app close"));
    assert!(close_witness(&mut app).expect("source app close"));
}

/// 📏 Wave B12: grid / spacing / LOD / vortex-show must publish into Puzzle3dWindowConfig and the
/// measures rail must read the new values. A silent `.ok().into_iter()` drop would leave generation
/// unchanged and the rail bound to the previous snapshot.
#[semio_framework_async_macros::async_test]
async fn window_option_rail_round_trips_grid_lod_and_vortex_into_published_config() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let before = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 25.0 })), Some(main::WINDOW_KIND_ID)).await.expect("setGridSpacing");
    dispatch(&mut app, "setLodAutomatic", Some(&json!({ "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("setLodAutomatic");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow");
    assert!(!app.has_pending_typed_operations(), "window-option gestures must quiesce");
    let after = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    assert_eq!(after, before + 4, "each window-option change must publish a WindowConfig generation, never drop");
    let measures = app.window_measures(&view).await;
    let rail = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_toggle(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible")), Some(false));
    assert_eq!(find_measure_slider(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(25.0));
    assert_eq!(find_measure_toggle(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-auto")), Some(false));
    assert_eq!(find_measure_select(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
}

/// 🪟️ The whole per-window option round trip, on the two instances the default layout actually opens
/// (`puzzle3d-main-top` / `puzzle3d-main-perspective`), driven exactly as the measures rail drives it:
/// the rail's own `WindowMeasure::Toggle` args (`{"pressed": …}`) tagged with the instance the rail
/// belongs to. One toggle must (a) advance THAT instance's window-config generation by exactly one,
/// (b) come back through the same instance's measures rail as the new value, (c) reach the world body
/// the instance renders, and (d) leave the sibling instance's rail, body and generation untouched.
///
/// 🕹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12: the browser symptom was a checkbox that never
/// moved. Every hop below is green, which is what pinned the defect to the two host-side halves — the
/// control rendering the published value with no state of its own while the round trip is in flight,
/// and `Puzzle3dScopeClass::Chrome` making that round trip a full-shell repaint.
#[semio_framework_async_macros::async_test]
async fn window_option_toggle_round_trips_on_its_own_instance_and_leaves_the_sibling_alone() {
    let mut app = app().await;
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");
    let top = app.window_view(main::WINDOW_INSTANCE_TOP);
    let perspective = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": true })), Some(main::WINDOW_INSTANCE_TOP)).await.expect("register the top pane's own config lane");
    let top_generation_before = app.window_config_generation(&top).await.expect("top generation");

    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setGridVisible on the perspective pane");

    let perspective_generation = app.window_config_generation(&perspective).await.expect("perspective generation");
    assert_eq!(perspective_generation, Some(1), "one rail toggle publishes exactly one generation on the instance it was dispatched from");
    assert_eq!(app.window_config_generation(&top).await.expect("top generation"), top_generation_before, "the sibling instance's own config lane must not advance");

    let view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let measures = app.window_measures(&view).await;
    assert_eq!(find_measure_toggle(measures.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective measures"), &toggle_id), Some(false), "the rail re-reads the value its own toggle just published");
    assert_eq!(find_measure_toggle(measures.get(main::WINDOW_INSTANCE_TOP).expect("top measures"), &toggle_id), Some(true), "the sibling rail keeps the value IT published");
    assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await).get("showLodGrid").and_then(Value::as_bool), Some(false), "the world body the instance renders consumes the published option");
    assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_INSTANCE_TOP).await).get("showLodGrid").and_then(Value::as_bool), Some(true), "the sibling's world body keeps its own grid");
}

/// ⚙️ The Settings panel's four steppers are addressable, carry the value they configure, and declare
/// the trigger the control they are built from actually fires.
///
/// 🔢️ `uniform` is the load-bearing half: a `NumberStepper` declared `uniform: false` renders MIXED —
/// a blank box with a placeholder instead of the value, and a +/− bump computed from the widget's own
/// `defaultValue` (0) rather than from the setting. These are one window's own scalars, never a
/// multi-selection aggregate, so the honest declaration is `true` (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B12, browser-measured: all four boxes rendered empty).
#[semio_framework_async_macros::async_test]
async fn settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on() {
    fn stepper(node: &Value, id: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(id) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| stepper(child, id)),
            Value::Array(items) => items.iter().find_map(|item| stepper(item, id)),
            _ => None,
        }
    }
    let mut app = app().await;
    let panel: Value = from_json_str(&to_json_string(&render_body(&mut app, settings_panel::BODY_KEY).await), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the settings panel renders parseable ui json");
    for (field, action) in [
        ("contact-tolerance", "setBrushPlacementContactTolerance"),
        ("proximity-radius", "setProximityRadius"),
        ("chunk-size", "setChunkSize"),
        ("grid-spacing", "setGridSpacing"),
    ] {
        let row = stepper(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}")).unwrap_or_else(|| panic!("the settings panel must declare a {field} field row: {panel}"));
        let control = stepper(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}.control")).unwrap_or_else(|| panic!("the {field} row must own the stepper control it labels: {row}"));
        let props = control.get("component").cloned().unwrap_or_else(|| panic!("{field} control is a number stepper: {control}"));
        assert_eq!(props.get("type").and_then(Value::as_str), Some("numberStepper"), "{field} control is a number stepper: {control}");
        assert_eq!(props.get("uniform").and_then(Value::as_bool), Some(true), "{field} configures ONE window's own scalar, so it can never render as a mixed selection: {props}");
        assert!(props.get("value").and_then(Value::as_f64).is_some(), "{field} must carry the value it configures: {props}");
        let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
        assert_eq!(bindings.len(), 1, "{field} declares exactly the one trigger it handles: {bindings:?}");
        assert_eq!(bindings[0].get("trigger").and_then(Value::as_str), Some("change"), "{field} handles absolute values, so it binds Change and NOT Delta: {bindings:?}");
        assert_eq!(bindings[0].pointer("/action/name").and_then(Value::as_str), Some(action), "{field} dispatches {action}: {bindings:?}");
        assert_eq!(bindings[0].pointer("/args/windowId").and_then(Value::as_str), Some(main::WINDOW_KIND_ID), "{field} is scoped at the window the panel was rendered for, so the value it publishes reaches THAT window's rail: {bindings:?}");
    }
}

/// 🐢️ Every control the measures rail and the Settings panel bind to a per-window option resolves to
/// [`Puzzle3dScopeClass::WindowOption`], and that class paints exactly the three surfaces those options
/// can move: the world body that re-derives from them, the measures rail that renders them, and the
/// Settings panel that mirrors four of the same fields. Never the outliner, catalogue, inspector,
/// history, labels, utilities or tool measures — and never `Full`, which is what these verbs used to
/// fall through to (one grid toggle re-rendered the whole shell before its own checkbox could move).
#[semio_framework_async_macros::async_test]
async fn window_option_verbs_paint_only_their_window_their_rail_and_the_settings_panel() {
    use crate::editor::puzzle3d::puzzle3d_command_scope_class;
    use crate::editor::puzzle3d::puzzle3d_scope;
    use crate::editor::puzzle3d::Puzzle3dScopeClass;
    for action in [
        "setGridVisible",
        "setGridSnapEnabled",
        "setGridSpacing",
        "setLodAutomatic",
        "setLodDepthVariable",
        "setLodManual",
        "setVortexShow",
        "setVortexDirection",
        "toggleSun",
        "setSunAzimuth",
        "setSunElevation",
        "setSunIntensity",
        "setSelectableKind",
        "setTransformGumballFlag",
        "setProximityRadius",
        "setChunkSize",
        "setVoxelDims",
        "setProjection",
        "setProjectionParam",
    ] {
        assert_eq!(puzzle3d_command_scope_class(action), Puzzle3dScopeClass::WindowOption, "{action} is a per-window option and must be classified as one");
    }
    let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = puzzle3d_scope(Puzzle3dScopeClass::WindowOption) else {
        panic!("the window-option class is a narrowed scope, never Full");
    };
    assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
    assert_eq!(panel_bodies, vec![settings_panel::BODY_KEY.to_string()], "the Settings panel mirrors four window options and must never disagree with the rail");
    assert!(measures, "the rail's own controls are bound to the value this publishes");
    assert!(!utilities && !tools && !engagements && !labels, "a window option moves no other chrome");
    for body in [inspection::BODY_KEY, artifact::BODY_KEY, catalogue::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
        assert!(!panel_bodies.iter().any(|named| named == body), "a window option cannot move the {body} panel");
    }
}

#[semio_framework_async_macros::async_test]
async fn window_transient_is_exact_instance_local_and_resets_on_reload() {
    let mut reopened = app().await;
    let mut app = app().await;
    let window_a = "puzzle3d-transient-a";
    let window_b = "puzzle3d-transient-b";
    let view_a = app.window_view(window_a);
    let view_b = app.window_view(window_b);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 10.0, "y": 20.0 })), Some(window_a)).await.expect("open suggestions in window a");
    let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
    let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
    assert_eq!(transient_a.get::<window_ownership::Puzzle3dWindowTransientOwner>().and_then(|value| value.suggestion_menu.as_ref()).map(|menu| menu.window_id.as_str()), Some(window_a));
    assert!(transient_b.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    dispatch(&mut app, "closeVortexSuggestions", None, Some(window_a)).await.expect("close suggestions in window a");
    let closed = app.window_transient_snapshot(&view_a).expect("closed transient").expect("closed owner");
    assert!(closed.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    let vortex_again = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex_again })), Some(window_a)).await.expect("open suggestions in window a again");
    let reset = reopened.window_transient_snapshot(&view_a).expect("reopened transient").expect("reopened owner");
    assert!(reset.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    assert_eq!(app.window_transient_generation(&view_a).expect("window a transient generation"), Some(3));
    assert_eq!(app.window_transient_generation(&view_b).expect("window b transient generation"), Some(0));
    // 🧷️ Every `window_transient_snapshot` above is a live READ of the partition store, and close is
    // blocked — by design — while one is outstanding. The reads have to be surrendered before the
    // close witness, exactly as a host surrenders a rendered body before retiring its app.
    drop((transient_a, transient_b, closed));
    drop(reset);
    assert!(close_witness(&mut reopened).expect("reopened app close"));
    assert!(close_witness(&mut app).expect("source app close"));
}

/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave S. A mounted worker job session is admitted out of a
/// FIXED process-wide array of `semio_framework_job::WORKER_JOB_SESSION_SLOTS` retirement slots, and an
/// app dropped while an operation is still mounted parks its node there instead of freeing it. ONLY a
/// live app's cooperative-maintenance round robin hands that slot back. Without that pump the array
/// filled up for the life of the process: from then on EVERY typed operation had its worker session
/// refused, and a refused session never runs the job, so the operation reached publication with no
/// completion value and the host's continuation loop spun on it forever — which is exactly how one
/// abandoned command turned a whole test binary into `pending typed operations outlived the settle
/// budget` for every later command, including plain view actions.
#[semio_framework_async_macros::async_test]
async fn an_abandoned_app_hands_its_worker_session_admission_back_to_the_next_app() {
    let mut abandoned = app().await;
    dispatch_unsettled(&mut abandoned, "setCamera", Some(&json!({ "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("setCamera mounts one typed operation");
    drop(abandoned);
    let mut live = app().await;
    for _ in 0..RETIREMENT_RECLAIM_TURNS {
        if !semio_framework_job::worker_job_retirements_are_parked() {
            break;
        }
        live.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("one cooperative maintenance unit");
    }
    assert!(
        !semio_framework_job::worker_job_retirements_are_parked(),
        "a live app's maintenance round robin must reclaim every parked worker-job retirement slot, or the process runs out of session admissions"
    );
    dispatch(&mut live, "setCamera", Some(&json!({ "position": [4.0, 5.0, 6.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("the next app still quiesces on a plain view action");
}

/// 🔁️ Maintenance units the reclaim law grants: every stage of the round robin, many times over, so the
/// law fails on a missing pump rather than on a tight budget.
const RETIREMENT_RECLAIM_TURNS: usize = semio_framework_plugin::MAINTENANCE_STAGES as usize * 64;

/// 🎥️ `setCamera`/`setProjection`/`setProjectionParam`/`focusSelection` moved off the document —
/// they are View-kind and must never emit VCS operations, no matter what they mutate.
#[semio_framework_async_macros::async_test]
async fn camera_actions_are_view_actions_that_emit_no_artifact_mutations() {
    let app_definition = create_puzzle3d_app();
    for action_id in ["setCamera", "setProjection", "setProjectionParam", "focusSelection"] {
        let def = dispatchable_actions(&app_definition).into_iter().find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("{action_id} declared"));
        assert_eq!(def.kind, ActionKind::View, "{action_id} must be a View action — camera is session-only, never a VCS edit");
    }
    let mut live = app().await;
    let before = projection_of(&live);
    let result = dispatch(&mut live, "setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.5 } })), None).await.expect("setCamera");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&live), before, "setCamera must not mutate the document");
}

/// 🎥️ Wave B11 (checklist §22, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 14): `f` with
/// nothing selected completed with a bare `Emit::default()` — the camera never moved, no notice was
/// raised, and the key read as dead on every boot before the user had picked anything. A camera verb
/// with no selection has an obvious subject (the whole document), so focus must FRAME it — while still
/// writing only the per-window camera, never a document operation.
#[semio_framework_async_macros::async_test]
async fn focus_selection_frames_the_whole_document_when_nothing_is_selected() {
    let mut app = app().await;
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [900.0, 900.0, 900.0], "target": [900.0, 900.0, 900.0], "zoom": 1.0 } })), Some(main::WINDOW_KIND_ID)).await.expect("park the camera far away");
    let parked = camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    let document_before = projection_of(&app);
    let result = dispatch(&mut app, "focusSelection", None, Some(main::WINDOW_KIND_ID)).await.expect("focusSelection with an empty selection must complete");
    let framed = camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_ne!(framed, parked, "focus with nothing selected must frame the document instead of leaving the camera parked");
    assert!(result.mutations.is_empty(), "focus is camera-only — it must emit no artifact history entry: {:?}", result.mutations);
    assert_eq!(projection_of(&app), document_before, "focus must not touch the document");
    let objects = document_before.get("objects").and_then(Value::as_array).expect("the example carries objects");
    let first = objects.first().and_then(|object| object.get("origin")).cloned().expect("an object origin");
    assert_eq!(framed.pointer("/target/0").and_then(Value::as_f64), first.pointer("/0").and_then(Value::as_f64), "a one-object document frames that object");
}

/// 🪟️ ONE per-window setting is ONE window-config publication that QUIESCES. The lane's own bounded
/// preparation used to demand the owner's whole declared publication ceiling
/// (`Puzzle3dWindowConfigOwner::MAXIMUM_PUBLICATION_BYTES`, 65 536) out of the publisher's per-turn
/// byte grant (`TYPED_OPERATION_RESULT_PAGE_BYTES`, 4 096) and answered `Blocked` on every single
/// turn, so `setCamera` — and every projection/grid/vortex/window option behind the same lane — sat
/// in stage `Publishing` forever and nothing queued behind it could ever run. The law is exactly:
/// one mutation, one generation, no pending operation left over.
#[semio_framework_async_macros::async_test]
async fn one_window_config_mutation_publishes_exactly_one_generation_and_quiesces() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let before = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [7.0, 8.0, 9.0], "target": [0.0, 0.0, 0.0], "zoom": 1.5 } })), None).await.expect("setCamera");
    assert!(!app.has_pending_typed_operations(), "one window-config gesture must leave no typed operation pending");
    let after = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    assert_eq!(after, before + 1, "one window-config mutation publishes exactly one store generation, never zero and never a spin");
    assert_eq!(camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).pointer("/position/0").and_then(Value::as_f64), Some(7.0), "the published window config is what the window renders");
}

/// 🛰️📷️ Wave B50: `setCamera` must move the pose the REFRESH route publishes, not only the one the
/// `<body>:<windowInstanceId>` surface route does. `plugin_refresh_ui` renders a window section as
/// `app.render(body_key, None, view.for_window_instance(key))` (`🔌️plugin/🦀️.rs:33790`) — the
/// instance reaches the guest through the VIEW alone, with no instance-suffixed body key — and the
/// section hash it answers is what `uiRefreshSectionUnchanged` compares. Every pre-existing camera
/// law measured [`render_window`] (the surface route), so a refresh route that publishes the opening
/// pose forever read green while the browser froze: wasm #59's battery reported
/// `camera settle puzzle3d-main-perspective moved=false waitedMs=30184` for orbit, pan AND zoom with
/// `refreshUi sections asked=["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"]
/// changed=[] hashes={…"puzzle3d-main-perspective":"aba169d7:1"}` — the same hash after every
/// `setCamera`, on a `full` scope included. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B50.
#[semio_framework_async_macros::async_test]
async fn set_camera_moves_the_pose_the_window_refresh_route_publishes() {
    let mut app = app().await;
    let window = main::WINDOW_INSTANCE_PERSPECTIVE;
    let before = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, window).await);
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window)).await.expect("setCamera on the perspective instance");
    let after = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, window).await);
    assert_ne!(after, before, "the refresh route must publish the gestured pose, not the opening one: before={before} after={after}");
    assert_eq!(after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "the refresh route's published camera is the one `setCamera` wrote: {after}");
    let sibling = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_TOP).await);
    assert_ne!(sibling.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "one instance's gesture must not move its sibling's refresh pose: {sibling}");
}

/// 🪟️📷️ Orbiting one window instance's camera must never move any sibling instance's camera, and
/// must never touch the shared document.
#[semio_framework_async_macros::async_test]
async fn set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched() {
    let mut app = app().await;
    let window_a = "puzzle3d-main-a";
    let window_b = "puzzle3d-main-b";
    dispatch(&mut app, "worldPointerDown", None, Some(window_a)).await.expect("register a");
    dispatch(&mut app, "worldPointerDown", None, Some(window_b)).await.expect("register b");

    let before_document = projection_of(&app);
    let camera_b_before = camera_of(&render_window(&mut app, window_b).await);

    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window_a)).await.expect("setCamera on window A");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&app), before_document, "setCamera must never mutate the shared document");

    let camera_a_after = camera_of(&render_window(&mut app, window_a).await);
    assert_eq!(camera_a_after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "window A's own rendered camera picks up the new pose");
    assert_eq!(camera_of(&render_window(&mut app, window_b).await), camera_b_before, "window B's rendered camera must be unaffected by window A's setCamera");
}

#[semio_framework_async_macros::async_test]
async fn vortex_show_window_option_defaults_to_selected_and_switches_to_always() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "scene_snapshot must expose vortices");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_SELECTED));

    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode must hide vortices while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_always = app.window_measures(&view).await;
    let window_measures_always = measures_always.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_always, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
    assert_eq!(vortices_of(&render_composite(&mut app).await).len(), all_vortex_ids.len(), "Always mode must emit every vortex while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_SELECTED })), None).await.expect("setVortexShow selected");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "switching back to Selected must hide idle vortices");
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS));

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let outwards_vortices = vortices_of(&render_composite(&mut app).await);
    assert!(!outwards_vortices.is_empty(), "scene_snapshot must expose vortices");
    assert!(outwards_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), None).await.expect("setVortexDirection inwards");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_inwards = app.window_measures(&view).await;
    let window_measures_inwards = measures_inwards.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_inwards, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS));
    assert!(vortices_of(&render_composite(&mut app).await).iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_option_is_local_to_the_window_instance() {
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    dispatch(&mut app, "worldPointerDown", None, Some(main::WINDOW_KIND_ID)).await.expect("register base window");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow always on base");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(second_window)).await.expect("setVortexShow always on second");
    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), Some(second_window)).await.expect("setVortexDirection inwards on second window");

    let base_vortices = vortices_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert!(!base_vortices.is_empty(), "the base window must still emit vortices");
    assert!(base_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    let second_vortices = vortices_of(&render_window(&mut app, second_window).await);
    assert!(second_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}
//#endregion 🔖️WindowOptions

//#region 🔖️Fill
/// 🛠️ Wave B14: Fill is a mode-level tool. A leftover/window `select` utility must not take the world lane —
/// the guest world lane publishes `fill` while `active_tool_id` is fill.
#[test]
fn fill_tool_wins_the_world_lane_over_a_select_window_utility() {
    let mut runtime = Puzzle3dRuntime::default();
    runtime.active_tool_id = Some(fill_tool::TOOL_ID.into());
    let mut view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    view.active_utility_id = Some("select".into());
    view.active_utility_by_window_id.insert("pane".into(), "select".into());
    assert!(puzzle3d_fill_tool_active(&runtime), "the Fill tab arms the mode tool, not a window utility");
    assert_eq!(puzzle3d_scene_active_utility(&runtime, Some(&view), Some("pane")), fill_tool::TOOL_ID, "guest world lane must stay fill while the tool is armed");
    runtime.active_tool_id = None;
    assert!(!puzzle3d_fill_tool_active(&runtime));
    assert_eq!(puzzle3d_scene_active_utility(&runtime, Some(&view), Some("pane")), "select");
}

/// ⏯️ Fill is a framework tool run declared through the manifest (`📋️tool-run-contract.md` §2.4): a mutating
/// `instance3d` run with revalidate rebase and resume reconfigure, run and revalidate jobs, and one reason per
/// schema reason spelling that reason's own code, id and verdict. The framework injects the seven run actions;
/// no plugin tick or cancel verb survives.
#[semio_framework_async_macros::async_test]
async fn the_fill_tool_declares_its_tool_run_through_the_manifest() {
    use semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::FillRunReason;
    use semio_framework_tool_run::ToolRunRebasePolicy;
    use semio_framework_tool_run::ToolRunReconfigurePolicy;
    use semio_framework_tool_run::ToolRunTraceKind;
    use semio_framework_tool_run::TOOL_RUN_ACTION_IDS;
    let definition = create_puzzle3d_app();
    let fill = definition.tools.iter().find(|tool| tool.id == fill_tool::TOOL_ID).expect("fill tool");
    let run = fill.run.as_ref().expect("the fill tool declares its ToolRunDefinition");
    run.validate().expect("the fill run definition validates");
    assert!(run.mutating, "fill writes the document, so its run is transactional");
    assert_eq!((run.rebase, run.reconfigure, run.trace), (ToolRunRebasePolicy::Revalidate, ToolRunReconfigurePolicy::Resume, ToolRunTraceKind::Instance3d));
    assert_eq!((run.run_job.as_str(), run.revalidate_job.as_ref().map(|job| job.as_str())), (fill_tool::RUN_JOB_KIND, Some(fill_tool::REVALIDATE_JOB_KIND)));
    assert_eq!(run.reasons.len(), FillRunReason::ALL.len());
    for (reason, declared) in FillRunReason::ALL.iter().zip(&run.reasons) {
        assert_eq!((declared.code, declared.id.as_str(), declared.verdict), (reason.code(), reason.id(), reason.verdict()), "{} spells the schema's own code, id and verdict", reason.id());
    }
    let actions: Vec<&str> = dispatchable_actions(&definition).into_iter().map(|action| action.id.as_str()).collect();
    for retired in ["fillBuildTick", "cancelFillBuild"] {
        assert!(!actions.contains(&retired), "{retired} is a per-plugin run verb the framework tool run replaces");
    }
    for id in TOOL_RUN_ACTION_IDS {
        assert!(actions.contains(&id), "declaring a run injects {id}");
    }
}

/// ⏯️ Red→green successor of the lock-is-commit tick law (`📋️tool-run-contract.md` §5 W1-F): a fill run holds
/// every placement provisionally in the instance-owned tool run ledger — the committed document never moves
/// while it runs and the viewport renders committed ⊕ provisional — and finalize publishes ONE edit carrying
/// every provisional placement, which one undo removes and one redo restores. No process-global run state
/// exists, so the law holds at any test thread count.
#[semio_framework_async_macros::async_test]
async fn fill_run_finalize_publishes_one_edit_with_every_provisional_placement() {
    use semio_framework_tool_run::TOOL_RUN_FINALIZE_ACTION_ID;
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 3 })), None).await.expect("setFillCount");
    let committed = committed_document(&app);
    let objects = object_count(&app);
    assert_eq!(start_fill_run(&mut app).await["toolRun"], json!("spawnJob"));
    pump_fill_run(&mut app, "the run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete).await;
    let placed = fill_run_presence(&app).completed as usize;
    assert_eq!(placed, 3, "the run places exactly what was requested");
    assert_eq!(committed_document(&app), committed, "a complete run has committed nothing yet");
    assert_eq!(rendered_object_count(&mut app).await, objects + placed, "the viewport renders committed ⊕ provisional");
    let provisional_instances = |node: &Value| instances_of(node).iter().filter(|instance| instance["provisional"] == json!(true)).count();
    assert_eq!(provisional_instances(&render_composite(&mut app).await), placed, "exactly the provisional placements carry the provisional instance style");
    let identity = fill_run_identity(&mut app).await.expect("a complete run is live until it is finalized");
    assert_eq!(tool_run_action(&mut app, TOOL_RUN_FINALIZE_ACTION_ID, identity).await["toolRun"], json!("beginFinalize"));
    pump_fill_run(&mut app, "finalize publishes", |presence| presence.state == protocol::PresenceToolRunState::Finalized).await;
    assert_eq!(object_count(&app), objects + placed, "finalize publishes every provisional placement");
    assert_eq!(rendered_object_count(&mut app).await, objects + placed, "the overlay is released onto the committed document");
    assert_eq!(provisional_instances(&render_composite(&mut app).await), 0, "finalized placements render as committed objects");
    assert!(fill_run_identity(&mut app).await.is_none(), "a finalized run is terminal, so Escape no longer aborts it");
    let finalized = committed_document(&app);
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "one undo removes every placement: start → complete → finalize is ONE undo entry");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(committed_document(&app), finalized, "one redo restores all of them");
}

/// 🛑 Red→green (`📋️tool-run-contract.md` §5 W1-F): the run's abort is offered from `starting` onwards with the
/// run's current identity, so it exists whenever provisional pieces are on screen. Aborting leaves zero
/// provisional placements and a committed document byte-identical to the one at start — no edit, no undo entry.
#[semio_framework_async_macros::async_test]
async fn cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run() {
    use semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID;
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 40 })), None).await.expect("setFillCount");
    let committed = committed_document(&app);
    let objects = object_count(&app);
    start_fill_run(&mut app).await;
    assert!(fill_run_identity(&mut app).await.is_some(), "abort is enabled while the run is still starting");
    pump_fill_run(&mut app, "a provisional placement", |presence| presence.completed > 0).await;
    assert!(rendered_object_count(&mut app).await > objects, "provisional pieces are on screen before the abort");
    let identity = fill_run_identity(&mut app).await.expect("abort carries the run's current identity in every non-terminal state");
    assert_eq!(tool_run_action(&mut app, TOOL_RUN_ABORT_ACTION_ID, identity).await["toolRun"], json!("closeJob"));
    pump_fill_run(&mut app, "the abort settles", |presence| presence.state == protocol::PresenceToolRunState::Aborted).await;
    assert_eq!(committed_document(&app), committed, "an aborted run leaves the committed document byte-identical");
    assert_eq!(rendered_object_count(&mut app).await, objects, "an aborted run holds zero provisional placements");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "and left no undo entry behind");
}

/// 🛑 Red→green (`📋️tool-run-contract.md` §5 W1-F): Escape (`engagementAbort`) maps to the framework-reserved
/// `toolRunAbort` of the run this window shows while it is non-terminal, and disarms the tool. The abort only
/// BEGINS inside that call — the run is `aborting` — and the teardown is spent in the driver's bounded close
/// turns, never inside one turn. Nothing was ever written, so no artifact lane is involved.
#[semio_framework_async_macros::async_test]
async fn engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one() {
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 40 })), None).await.expect("setFillCount");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("arm fill");
    let committed = committed_document(&app);
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app, "a provisional placement", |presence| presence.completed > 0).await;
    let identity = fill_run_identity(&mut app).await.expect("a live fill run");
    let run = identity[semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID].as_str().and_then(|run| run.parse().ok()).expect("run id");
    let generation = identity[semio_framework_tool_run::TOOL_RUN_ARG_GENERATION].as_u64().and_then(|generation| u32::try_from(generation).ok()).expect("generation");
    app.echo_tool_run_trace_cursor(main::WINDOW_KIND_ID, semio_framework_tool_run::ToolRunTraceCursor { run, generation, page: 1 });
    let escape = dispatch_unsettled(&mut app, "engagementAbort", None, None).await.expect("Escape");
    let settled = settle(&mut app).await;
    let effects: Vec<&Effect> = escape.requested_effects.iter().chain(&settled.effects).collect();
    let abort = effects
        .iter()
        .find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID => args.clone(),
            _ => None,
        })
        .expect("Escape dispatches toolRunAbort while a fill run is non-terminal");
    assert!(effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())), "Escape also disarms the Fill tool");
    let abort = semio_framework_pack_json::from_dsl_value(&abort);
    assert_eq!(abort[semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID], identity[semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID], "the abort names the run this window shows");
    tool_run_action(&mut app, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID, abort).await;
    assert_eq!(fill_run_presence(&app).state, protocol::PresenceToolRunState::Aborting, "the abort only begins inside the call");
    let turns = pump_fill_run(&mut app, "the teardown", |presence| presence.state == protocol::PresenceToolRunState::Aborted).await;
    assert!(turns >= 1, "the run tears down in the driver's close turns, never inside the abort's own turn");
    assert_eq!(committed_document(&app), committed, "nothing was ever written, so nothing is rolled back");
}

/// 🎚️ Red→green (`📋️tool-run-contract.md` §5 W1-F): a fresh document asks for the default of 100, and
/// `setFillCount` is configuration only — it never emits an artifact mutation. During a run it reconfigures
/// that run (`reconfigure: resume`): the same run id continues under a newer generation.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count() {
    async fn fill_count_entry(app: &mut Puzzle3dApp) -> Option<f64> {
        let view = app.window_view(main::WINDOW_KIND_ID);
        let measures = app.tool_measures(&view).await;
        find_measure_number(measures.get(fill_tool::TOOL_ID).expect("fill tool measures"), "puzzle3d-fill-count")
    }
    let mut app = app().await;
    assert_eq!(fill_count_entry(&mut app).await, Some(100.0), "a fresh document asks for the default of 100");
    let committed = committed_document(&app);
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 3 })), None).await.expect("setFillCount through the migrated tool-job path");
    assert_eq!(fill_count_entry(&mut app).await, Some(3.0), "setFillCount updates the requested count");
    assert_eq!(committed_document(&app), committed, "the count is configuration: no artifact mutation");
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app, "the first run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete).await;
    let before = fill_run_identity(&mut app).await.expect("a live run");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 5 })), None).await.expect("setFillCount during a run");
    pump_fill_run(&mut app, "the reconfigured run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == 5).await;
    let after = fill_run_identity(&mut app).await.expect("the same run is still live");
    assert_eq!(after["runId"], before["runId"], "a count change resumes the SAME run");
    assert!(after["generation"].as_u64() > before["generation"].as_u64(), "under a newer generation");
    assert_eq!(committed_document(&app), committed, "a reconfigured run still committed nothing");
}

/// 🔁️ A count raised during a run continues the same deterministic sequence, a lowered one retracts its tail:
/// the provisional placements at 2 ⊂ at 4 ⊂ at 6, all under one run id, and finalize commits exactly the
/// lowered count.
#[semio_framework_async_macros::async_test]
async fn raising_and_lowering_the_fill_count_during_a_run_continues_and_retracts_the_same_sequence() {
    use semio_framework_tool_run::TOOL_RUN_FINALIZE_ACTION_ID;
    async fn provisional_ids(app: &mut Puzzle3dApp, committed: &std::collections::BTreeSet<String>) -> std::collections::BTreeSet<String> {
        instances_of(&render_composite(app).await).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str)).filter(|id| !committed.contains(*id)).map(str::to_string).collect()
    }
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 4 })), None).await.expect("setFillCount 4");
    let committed: std::collections::BTreeSet<String> = instances_of(&render_composite(&mut app).await).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    let objects = object_count(&app);
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app, "four placements", |presence| presence.state == protocol::PresenceToolRunState::Complete).await;
    let run = fill_run_identity(&mut app).await.expect("live run")["runId"].clone();
    let four = provisional_ids(&mut app, &committed).await;
    assert_eq!(four.len(), 4);
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 6 })), None).await.expect("raise to 6");
    pump_fill_run(&mut app, "six placements", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == 6).await;
    let six = provisional_ids(&mut app, &committed).await;
    assert!(four.is_subset(&six) && six.len() == 6, "a raise continues the sequence: {four:?} ⊂ {six:?}");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 2 })), None).await.expect("lower to 2");
    pump_fill_run(&mut app, "two placements", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == 2).await;
    let two = provisional_ids(&mut app, &committed).await;
    assert!(two.is_subset(&four) && two.len() == 2, "a lower retracts the tail: {two:?} ⊂ {four:?}");
    let identity = fill_run_identity(&mut app).await.expect("live run");
    assert_eq!(identity["runId"], run, "raise and lower both resume the same run");
    tool_run_action(&mut app, TOOL_RUN_FINALIZE_ACTION_ID, identity).await;
    pump_fill_run(&mut app, "finalize", |presence| presence.state == protocol::PresenceToolRunState::Finalized).await;
    assert_eq!(object_count(&app), objects + 2, "finalize commits exactly the lowered count");
}

/// ♾️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS §1 decisions 1 and 4 — the count is any `u32`.
/// Nothing clamps a request against a planner ceiling; a document that cannot hold the ask reports a
/// stall step instead.
#[semio_framework_async_macros::async_test]
async fn the_fill_count_has_no_ceiling_and_the_entry_declares_none() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 5_000 })), None).await.expect("setFillCount");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_number(tool_measures, "puzzle3d-fill-count"), Some(5_000.0), "a request far above anything planned is kept verbatim — no clamp");
    assert_eq!(find_measure_number_max(tool_measures, "puzzle3d-fill-count"), Some(None), "the count entry declares no maximum");
    assert_eq!(crate::editor::puzzle3d::commands::set_fill_count::parse_count(Some(&json!({ "value": 4_000_000_000u32 }))), 4_000_000_000, "parse_count carries the whole u32 range");
}

/// 🪣️ The count is shared configuration: a count set in one split pane is the count every pane shows.
#[semio_framework_async_macros::async_test]
async fn fill_count_is_shared_across_split_panes() {
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 6 })), Some(top)).await.expect("setFillCount in the top pane");
    for window in [top, perspective] {
        let view = app.window_view(window);
        let measures = app.tool_measures(&view).await;
        assert_eq!(find_measure_number(measures.get(fill_tool::TOOL_ID).expect("fill tool measures"), "puzzle3d-fill-count"), Some(6.0), "{window} shows the shared count");
    }
}

/// 🔢️ The count control is an unbounded `Number` entry showing the REQUESTED count; progress and placements
/// are the framework ToolRun panel's, so the entry carries no `ready` extent and loads only while a fill run
/// is live.
#[test]
fn fill_count_measure_is_an_unbounded_number_entry_reporting_the_requested_count() {
    let scene = Puzzle3dScene { scene_snapshot: nakagin_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: fill_tool::TOOL_ID.into() };
    match fill_tool::count_measure(&scene, &Puzzle3dLabels::NATIVE_EN, None) {
        WindowMeasure::Number { label: Some(label), value, min, max, ready, loading, .. } => {
            assert_eq!(label, Puzzle3dLabels::NATIVE_EN.count.as_str());
            assert_eq!(value, scene.runtime.fill_count as f64, "the entry shows the REQUESTED count, verbatim");
            assert_eq!((min, max, ready, loading), (Some(0.0), None, None, None), "no ceiling, no locked extent, and no loading ring without a run");
        }
        other => panic!("expected a Number measure, got {other:?}"),
    }
}

/// 🔁️ A committed `setFillCount` survives the renders that follow it: the config mutation IS the count.
#[semio_framework_async_macros::async_test]
async fn a_committed_fill_count_survives_the_renders_that_follow_it() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 250 })), None).await.expect("setFillCount");
    for _ in 0..3 {
        render_composite(&mut app).await;
    }
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_number(tool_measures, "puzzle3d-fill-count"), Some(250.0), "three renders later the requested count is still what the user asked for");
}
//#endregion 🔖️Fill

//#region 🔖️Distribution
#[semio_framework_async_macros::async_test]
async fn puzzle3d_normalize_kind_weight_group_redistributes_siblings_proportionally() {
    let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut weights = HashMap::from([("a".to_string(), 0.2), ("b".to_string(), 0.3), ("c".to_string(), 0.5)]);
    weights = puzzle3d_normalize_kind_weight_group(&weights, &ids, "a", 0.5);
    let sum: f64 = ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    assert!((sum - 1.0).abs() < 1e-9, "simplex must stay at 1, got {sum}");
    assert!((weights.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 1e-9);
    // b:c were 0.3:0.5 — remainder 0.5 splits 0.3/0.8 and 0.5/0.8
    assert!((weights.get("b").copied().unwrap_or(0.0) - 0.5 * 0.3 / 0.8).abs() < 1e-9);
    assert!((weights.get("c").copied().unwrap_or(0.0) - 0.5 * 0.5 / 0.8).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_vortex_measure_exposes_joint_weight_scaled_by_object() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let mut joint_weights = HashMap::new();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids);
    let object_weight = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    let measures = puzzle3d_joint_vortex_measures("Object", object_weight, &vortex_ids, &object_weights, &joint_weights);
    let expected_joint = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
    match &measures[0] {
        WindowMeasure::Slider { value, max, step, disabled, .. } => {
            assert!((*value - expected_joint).abs() < 1e-9, "slider must show the joint probability, got {value}");
            assert!((*max - 1.0).abs() < 1e-9, "joint range spans the global simplex");
            assert_eq!(*step, Some(0.01));
            assert_eq!(*disabled, None);
        }
        other => panic!("expected vortex slider, got {other:?}"),
    }
    puzzle3d_set_object_kind_distribution_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, "Object", 0.8);
    let raised_weight = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    let raised_measures = puzzle3d_joint_vortex_measures("Object", raised_weight, &vortex_ids, &object_weights, &joint_weights);
    match (&measures[0], &raised_measures[0]) {
        (WindowMeasure::Slider { value: before, .. }, WindowMeasure::Slider { value: after, .. }) => {
            assert!(*after > *before, "raising P(object) must raise joint vortex percentages");
            let expected_after = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
            assert!((*after - expected_after).abs() < 1e-9);
        }
        _ => panic!("expected vortex sliders"),
    }
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_distribution_lists_global_vortices_and_joints_sum_to_one() {
    let scene_snapshot = nakagin_scene_snapshot();
    let object_ids = puzzle3d_kind_ids(&scene_snapshot, "objects");
    let vortex_ids = puzzle3d_kind_ids(&scene_snapshot, "vortices");
    assert!(object_ids.len() >= 2, "default scene_snapshot needs multiple object kinds");
    assert!(vortex_ids.len() >= 2, "default scene_snapshot needs multiple vortex kinds");
    let object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let vortex_kind_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
    let scene = Puzzle3dScene { scene_snapshot, runtime: Puzzle3dRuntime { object_kind_weights, vortex_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: fill_tool::TOOL_ID.into() };
    let distribution_children = puzzle3d_distribution_children(&scene, Some(true));
    assert_eq!(distribution_children.len(), object_ids.len());
    let mut joint_sum = 0.0;
    for measure in &distribution_children {
        let WindowMeasure::Group { children, value: Some(object_weight), .. } = measure else {
            panic!("expected object-kind group");
        };
        assert_eq!(children.len(), vortex_ids.len(), "each object must list the full global vortex catalog");
        let local_sum: f64 = children
            .iter()
            .map(|child| match child {
                WindowMeasure::Slider { value, .. } => *value,
                _ => panic!("expected vortex slider"),
            })
            .sum();
        assert!((local_sum - object_weight).abs() < 1e-6, "under one object joints sum to P(object), not 1");
        joint_sum += local_sum;
    }
    assert!((joint_sum - 1.0).abs() < 1e-6, "all nested joint percentages across objects must sum to 1, got {joint_sum}");
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_object_weight_change_scales_joint_sampling_product() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let mut joint_weights = HashMap::new();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids);
    let joint_before = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
    puzzle3d_set_object_kind_distribution_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, "Object", 0.6);
    let joint_after = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
    assert!(joint_after > joint_before);
    let global = puzzle3d_global_joint_total(&joint_weights, &object_weights, &object_ids, &vortex_ids);
    assert!((global - 1.0).abs() < 1e-6, "global joints must stay normalized, got {global}");
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_vortex_joint_edit_redistributes_siblings_and_preserves_object_total() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let mut joint_weights = HashMap::new();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids);
    let object_total_before = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    let sibling_before = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "b-s");
    let new_joint = object_total_before * 0.7;
    puzzle3d_set_vortex_joint_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, "Object", "c-b", new_joint);
    let object_total_after = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    assert!((object_total_after - object_total_before).abs() < 1e-6);
    let sibling_after = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "b-s");
    assert!(sibling_after < sibling_before, "raising one vortex must lower its siblings");
    let global = puzzle3d_global_joint_total(&joint_weights, &object_weights, &object_ids, &vortex_ids);
    assert!((global - 1.0).abs() < 1e-6);
}

/// 🚫️ Zero object-kind weight disables every vortex slider under that kind — anything × 0 is 0.
#[semio_framework_async_macros::async_test]
async fn zero_object_kind_weight_disables_joint_vortex_sliders() {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let scene_snapshot = nakagin_scene_snapshot();
    let object_ids = puzzle3d_kind_ids(&scene_snapshot, "objects");
    assert!(!object_ids.is_empty(), "default scene_snapshot must expose object kinds");
    let zeroed_id = object_ids[0].clone();
    let mut object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    object_kind_weights = puzzle3d_normalize_kind_weight_group(&object_kind_weights, &object_ids, &zeroed_id, 0.0);
    assert!(object_kind_weights.get(&zeroed_id).copied().unwrap_or(1.0) <= f64::EPSILON);
    let scene = Puzzle3dScene { scene_snapshot, runtime: Puzzle3dRuntime { object_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: fill_tool::TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&scene, labels, None);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    let zeroed_group_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution-object-{zeroed_id}");
    let zeroed_group = distribution_children.iter().find(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == &zeroed_group_id)).expect("zeroed object kind must appear in distribution");
    match zeroed_group {
        WindowMeasure::Group { value: Some(value), children, .. } => {
            assert!(*value <= f64::EPSILON, "object-kind header must read 0%");
            assert!(!children.is_empty(), "object kind must still list vortex sliders");
            assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: Some(true), value, .. } if *value <= f64::EPSILON)), "every joint vortex slider under a 0% object kind must be disabled at 0%");
        }
        other => panic!("expected object-kind group, got {other:?}"),
    }
    let live_group = distribution_children.iter().find(|measure| match measure {
        WindowMeasure::Group { id, value: Some(value), .. } if id != &zeroed_group_id => *value > f64::EPSILON,
        _ => false,
    });
    if let Some(WindowMeasure::Group { children, .. }) = live_group {
        assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: None | Some(false), .. })), "joint vortex sliders under a non-zero object kind must stay enabled");
    }
}

/// 🎯️ Fill tool measures expose count + nested distribution tree under the Fill toggle; the Volume
/// Brush voxel dims live in a utility-options group in the window's own measures.
#[semio_framework_async_macros::async_test]
async fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
    {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let fill_scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: fill_tool::TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&fill_scene, labels, None);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    assert!(!fill_measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle3d-play-tool-options-fill")), "fill must not wrap its options in a nested Fill group — the tool toggle already owns that row");
    assert_eq!(measure_group_tag(&fill_measures, &distribution_id), Some(None));
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    assert!(!distribution_children.is_empty(), "distribution must list object-kind groups");
    assert!(distribution_children.iter().all(|measure| matches!(measure, WindowMeasure::Group { value: Some(_), on_change: Some(_), .. })), "each object-kind group must carry a header weight slider");
    assert!(
        distribution_children.iter().all(|measure| match measure {
            WindowMeasure::Group { label, value: Some(_), on_change: Some(_), .. } => !label.contains('%'),
            _ => false,
        }),
        "object-kind group labels must not embed percentages — the header slider owns the value readout"
    );
    assert!(
        distribution_children.iter().any(|measure| match measure {
            WindowMeasure::Group { children, .. } => children.iter().any(|child| matches!(child, WindowMeasure::Slider { label: Some(label), .. } if !label.contains('%'))),
            _ => false,
        }),
        "vortex joint sliders must label kinds without embedding percentages"
    );
    assert!(find_measure_toggle(&fill_measures, "puzzle3d-edit-volumes").is_none(), "fill must not carry edit-volumes toggle");
    assert!(find_measure_slider(&fill_measures, "puzzle3d-voxel-w").is_none(), "fill must not carry voxel-dimension sliders");
    assert!(find_measure_number(&fill_measures, "puzzle3d-fill-count").is_some(), "the fill-count entry always lives in the fill tool measures");
    assert!(
        !main::window_measures(&fill_scene, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id.contains("fill"))),
        "fill must no longer surface in window_measures — it is a mode-level tool, not a window utility"
    );
    let volume_brush_scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::volume_brush::UTILITY_ID.into() };
    let volume_brush_measures = main::window_measures(&volume_brush_scene, labels);
    assert_eq!(measure_group_tag(&volume_brush_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-volume-brush")), Some(Some(utilities::volume_brush::UTILITY_ID.into())));
    assert!(find_measure_slider(&volume_brush_measures, "puzzle3d-voxel-w").is_some(), "volume brush utility exposes voxel width slider");
    let fill_engagement = main::engagement(&fill_scene, &Puzzle3dLabels::NATIVE_EN, None);
    assert!(fill_engagement.control.is_none() && fill_engagement.controls.is_none(), "fill engagement HUD must no longer carry the relocated controls");
    let brush_scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
    assert_eq!(measure_group_tag(&main::window_measures(&brush_scene, labels), &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())));
    let brush_engagement = main::engagement(&brush_scene, &Puzzle3dLabels::NATIVE_EN, None);
    assert!(brush_engagement.control.is_none() && brush_engagement.controls.is_none(), "brush engagement HUD must no longer carry the relocated control");
    }
    assert_brush_options_after_open_vortex_suggestions().await;
}

#[inline(never)]
async fn assert_brush_options_after_open_vortex_suggestions() {
    let mut app = app().await;
    activate_window_utility(&mut app, utilities::brush::UTILITY_ID);
    let view = app.window_view(main::WINDOW_KIND_ID);
    let brush_app_measures = app.window_measures(&view).await;
    let window_measures = brush_app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())), "the brush Utility Options group surfaces on the live window-measures path when brush is the active utility");
}
//#endregion 🔖️Distribution

//#region 🔖️UiScope
/// 🐢️ THE scope law: every declared MUTATING command's `UiDirtyScope` names the panels its
/// mutation changes. A document edit moves the artifact outliner (the object roster), the field
/// inspector (the selected entity's own fields) and the framework history panel (one command row per
/// dispatch) — so a mutation whose scope is `Partial` and omits any of those three leaves that panel
/// showing pre-edit content until some unrelated later action happens to repaint it.
///
/// 🧯️ Why this is a law and not a review note: before wave N every hand-written `Partial` in this app
/// carried `panel_bodies: Vec::new()` (`📓️2026-09-09-wave-L-…md` §5.3) and the shared dispatcher's
/// default was `Full`, so the only two states a command could be in were "repaint the whole shell" and
/// "silently stale panels". `puzzle3d_command_scope_class` is now the one place that decides, and this
/// law is what stops a narrower class from being handed to a verb that edits the document.
///
/// 🈳️ `Chrome` (i.e. `Full`) always satisfies this law — it is the widest, always-correct answer, and a
/// newly declared command that nobody has classified yet is slow rather than wrong.
#[semio_framework_async_macros::async_test]
async fn command_scope_classes_name_the_panels_they_change() {
    use crate::editor::puzzle3d::puzzle3d_command_scope_class;
    use crate::editor::puzzle3d::puzzle3d_scope;
    let definition = create_puzzle3d_app();
    let mut mutating = 0_usize;
    let mut narrowed = 0_usize;
    for action in dispatchable_actions(&definition) {
        let class = puzzle3d_command_scope_class(&action.id);
        let scope = puzzle3d_scope(class);
        // 🧾️ The declared `ActionKind` is a HISTORY/undo classification, not a paint one. Both
        // `relocateTargetVolume` and `worldRelocate` are Mutations (they emit artifact edits), so this
        // law requires them to name inspector, outliner and history. A `View` verb that happens to carry
        // the document class is over-painting at worst, never stale.
        if action.kind != ActionKind::Mutation {
            continue;
        }
        mutating += 1;
        match scope {
            UiDirtyScope::Full => {}
            UiDirtyScope::None => panic!("{} is a declared Mutation, so it cannot paint nothing", action.id),
            UiDirtyScope::Partial { ref panel_bodies, ref window_bodies, .. } => {
                narrowed += 1;
                assert_eq!(window_bodies, &vec![main::BODY_KEY.to_string()], "{} edits the document, so the world body is dirty", action.id);
                for body in [inspection::BODY_KEY, artifact::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
                    assert!(panel_bodies.iter().any(|named| named == body), "{} is a declared Mutation with a narrowed scope, so it must name the {body} panel body: {panel_bodies:?}", action.id);
                }
            }
        }
    }
    assert!(mutating > 0, "the app declares mutating commands");
    assert!(narrowed >= 10, "the scope table must actually narrow the mutating verbs, not fall back to Full for all of them: {narrowed}/{mutating}");
}

/// 🕹️ A pure SELECTION change repaints the field inspector — the panel that renders the selected
/// entity's own fields — and the outliner that marks the selected rows, but never the catalogue, which
/// lists object KINDS and cannot move when only the selection moves.
#[semio_framework_async_macros::async_test]
async fn selection_scope_names_the_inspector_and_not_the_catalogue() {
    use crate::editor::puzzle3d::puzzle3d_scope;
    use crate::editor::puzzle3d::Puzzle3dScopeClass;
    let UiDirtyScope::Partial { panel_bodies, measures, utilities, engagements, labels, .. } = puzzle3d_scope(Puzzle3dScopeClass::Selection) else {
        panic!("the selection class is a narrowed scope");
    };
    assert!(panel_bodies.iter().any(|body| body == inspection::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == artifact::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == FRAMEWORK_HISTORY_BODY_KEY), "{panel_bodies:?}");
    assert!(!panel_bodies.iter().any(|body| body == catalogue::BODY_KEY), "a selection cannot change the kind roster: {panel_bodies:?}");
    assert!(measures, "selection-dependent window measures are re-read");
    assert!(!utilities);
    assert!(!engagements);
    assert!(!labels);
}

/// 🎮️ Wave B34: the six framework interaction verbs are declared out of this app's own scope table
/// through `ArtifactEditor::interaction_scope`, and each one names EXACTLY the lanes it moves. Before
/// this wave the framework answered `UiDirtyScope::Full` for all six — including `interactionHover`,
/// which fires on pointer motion — so every mouse move over the viewport repainted every window body,
/// every panel body, the utilities/tools/engagements rails, the labels and the measures
/// (`📓️2026-09-12-wave-B32-world-lane-after-completion.md` §3.2: 24 interaction ingresses and 3
/// whole-shell completions per 80 s lane, which is what a document mutation's own refresh pass then
/// queued behind for 10–14 s).
#[semio_framework_async_macros::async_test]
async fn interaction_verbs_declare_exactly_the_lanes_they_move() {
    use crate::editor::puzzle3d::puzzle3d_interaction_chrome_scope;
    use crate::editor::puzzle3d::puzzle3d_selection_scope;
    use crate::editor::puzzle3d::puzzle3d_viewport_scope;
    use crate::editor::puzzle3d::PUZZLE3D_INTERACTION_DOMAIN;
    use semio_framework_plugin::InteractionVerb;
    let domain = [PUZZLE3D_INTERACTION_DOMAIN];
    assert_eq!(Puzzle3dPlayApp::interaction_scope(InteractionVerb::Hover, &domain), Some(puzzle3d_viewport_scope()), "a hover moves the world body's hover lane and nothing else");
    for verb in [InteractionVerb::Select, InteractionVerb::ClearSelection, InteractionVerb::SelectAll] {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &domain), Some(puzzle3d_selection_scope()), "{verb:?} moves the declared selection lanes");
    }
    for verb in [InteractionVerb::SetSelectionMode, InteractionVerb::SetGranularity] {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &domain), Some(puzzle3d_interaction_chrome_scope()), "{verb:?} moves the window's Select chrome");
    }
    for verb in InteractionVerb::ALL {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &["mesh"]), None, "{verb:?} on a domain this app never declared must fall back to the framework scope");
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &[]), None, "{verb:?} touching no domain must fall back to the framework scope");
    }
}

/// 🐁️ The hover half, pinned field by field: a pointer move paints the world body ALONE — no panel, no
/// rail, no label, not even the window measures. The pick half names exactly the three selection panels
/// (`selection_scope_names_the_inspector_and_not_the_catalogue` pins their identity) and no fourth one,
/// so widening either answer has to widen this law with it.
#[semio_framework_async_macros::async_test]
async fn a_hover_paints_only_the_world_body_and_a_pick_adds_exactly_the_selection_panels() {
    use crate::editor::puzzle3d::puzzle3d_interaction_chrome_scope;
    use crate::editor::puzzle3d::puzzle3d_selection_scope;
    use crate::editor::puzzle3d::puzzle3d_viewport_scope;
    let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = puzzle3d_viewport_scope() else {
        panic!("the hover scope is a narrowed scope");
    };
    assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
    assert!(panel_bodies.is_empty(), "a hover cannot move a panel: {panel_bodies:?}");
    assert!(!utilities && !tools && !engagements && !measures && !labels, "a hover cannot move the shell chrome");
    let UiDirtyScope::Partial { panel_bodies, .. } = puzzle3d_selection_scope() else {
        panic!("the selection scope is a narrowed scope");
    };
    assert_eq!(panel_bodies.len(), 3, "a pick moves exactly the inspector, the outliner and the history: {panel_bodies:?}");
    for body in [inspection::BODY_KEY, artifact::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
        assert!(panel_bodies.iter().any(|named| named == body), "a pick moves {body}: {panel_bodies:?}");
    }
    let UiDirtyScope::Partial { panel_bodies, measures, .. } = puzzle3d_interaction_chrome_scope() else {
        panic!("the interaction chrome scope is a narrowed scope");
    };
    assert!(panel_bodies.is_empty(), "no panel renders the active selection mode or granularity: {panel_bodies:?}");
    assert!(measures, "the measures rail binds both Select controls");
}

/// 🈳️ The two narrow non-document classes deliberately name NO panel body — a camera move and a fill
/// count or distribution slider each touch the world body (and at most the tool/window measures) and
/// nothing else; pointing the brush suggestions run at a vortex paints nothing at all. Pins the claim their doc comments make, so widening one
/// of them has to widen this law too.
#[semio_framework_async_macros::async_test]
async fn viewport_and_fill_option_scope_classes_name_no_panel_body() {
    use crate::editor::puzzle3d::puzzle3d_scope;
    use crate::editor::puzzle3d::Puzzle3dScopeClass;
    for class in [Puzzle3dScopeClass::Viewport, Puzzle3dScopeClass::FillOptions] {
        let UiDirtyScope::Partial { panel_bodies, window_bodies, .. } = puzzle3d_scope(class) else {
            panic!("{class:?} is a narrowed scope");
        };
        assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()], "{class:?}");
        assert!(panel_bodies.is_empty(), "{class:?} claims to paint no panel: {panel_bodies:?}");
    }
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Quiet), UiDirtyScope::None));
    assert_eq!(crate::editor::puzzle3d::puzzle3d_command_scope_class("targetBrushSuggestions"), Puzzle3dScopeClass::Quiet);
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Chrome), UiDirtyScope::Full));
}

/// 🪣️ `setFillCount` is configuration only — a live fill run holds its placements in the framework ledger —
/// so it declares the fill-options scope: the world body plus the tool and window measures, and no panel.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_declares_the_fill_options_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let result = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 1 })), None).await.expect("setFillCount");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty() || panel_bodies == [FRAMEWORK_HISTORY_BODY_KEY], "the count moves no document panel: {panel_bodies:?}");
            assert!(tools, "the fill-count entry lives in the fill tool's measures");
            assert!(measures);
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setFillCount, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn set_object_kind_weight_declares_fill_options_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let object_ids = puzzle3d_kind_ids(&nakagin_scene_snapshot(), "objects");
    let kind_id = object_ids.first().expect("object kind");
    let result = dispatch(&mut app, "setObjectKindWeight", Some(&json!({ "kindId": kind_id.as_str(), "value": 0.75 })), None).await.expect("setObjectKindWeight");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty());
            assert!(tools);
            assert!(measures, "distribution sliders live in tool + window measures");
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setObjectKindWeight, got {other:?}"),
    }
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `set_hover_is_a_view_action_with_no_ops_after_document_mutation`
// and `world_pick_declares_selection_ui_scope` deleted — both dispatched the now-deleted
// `setHover`/`worldPick` actions and asserted on the deleted `Effect::PatchWorld3dChrome`
// push-setter effect (selection/hover are framework-owned actions now, dispatched exclusively
// through the six reserved `interactionSelect`-family verbs; see `select_id`/`hover_id`).
//#endregion 🔖️UiScope

//#region 🔖️Utilities
#[semio_framework_async_macros::async_test]
async fn add_object_kind_honors_drop_origin() {
    let mut app = app().await;
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.5, 3.5, 0.0] })), None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1);
    let projection = projection_of(&app);
    let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("added object");
    let origin = object.get("origin").and_then(Value::as_array).expect("origin array");
    assert_eq!(origin.first().and_then(Value::as_f64), Some(2.5));
    assert_eq!(origin.get(1).and_then(Value::as_f64), Some(3.5));
    assert_eq!(origin.get(2).and_then(Value::as_f64), Some(0.0));
}

#[semio_framework_async_macros::async_test]
async fn add_object_kind_materializes_the_declared_kind_default() {
    // 📝️ P1 arg form: firing addObjectKind with no args on a document whose catalogs were cleared must
    // materialize the declared `objectKind` default — the catalog row AND the object referencing it —
    // in ONE gesture. A retained tool job never publishes inline (`settle`'s own docstring), so the
    // settled document, not the dispatch's `mutations` vector, is where the gesture is observed.
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", None, None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1, "the materialized default kind adds exactly one object");
    let projection = projection_of(&app);
    let kind = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("objectKind")).and_then(Value::as_str);
    assert_eq!(kind, Some("Object"), "the declared objectKind default was materialized host-side");
    let catalog = projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("the declared default kind catalog was materialized");
    assert!(catalog.iter().any(|row| row.get("id").and_then(Value::as_str) == Some("Object")), "the created object references a catalogued kind, never a dangling one: {catalog:?}");
}

/// 🌲️ The catalogue's own gesture on the DEFAULT document: pressing (or dropping) a `concrete-forest`
/// kind row must place an instance that carries that kind's `representations[].url` as its own
/// `meshUrl`. Every other `addObjectKind` law fires `"Object"`, a kind no shipped catalog declares, so
/// none of them ever read a representation — which is how the `meshUrl`-only read in
/// `🎮️commands/🌱️add-object-kind` survived: a placed object with no mesh identity renders nothing and
/// its fill/brush candidates read `mesh-unavailable`.
#[semio_framework_async_macros::async_test]
async fn adding_a_catalogued_concrete_forest_kind_places_an_object_carrying_its_mesh_url() {
    let scene_snapshot = crate::editor::puzzle3d::default_scene_snapshot();
    let entry = crate::editor::puzzle3d::puzzle3d_catalog_entries(&scene_snapshot, "objects").first().cloned().expect("the default document catalogues object kinds");
    let kind_id = entry.get("id").and_then(semio_framework_value::DslValue::as_str).expect("catalogued kind id").to_string();
    let expected = entry
        .get("representations")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|representation| representation.get("url").and_then(semio_framework_value::DslValue::as_str))
        .find(|url| !url.is_empty())
        .expect("the catalogued kind names a representation url")
        .to_string();
    let mut app = app().await;
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": kind_id.clone(), "origin": [4.0, 5.0, 0.0] })), None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1, "a catalogued kind places exactly one instance");
    let projection = projection_of(&app);
    let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("the placed object").clone();
    assert_eq!(object.get("objectKind").and_then(Value::as_str), Some(kind_id.as_str()), "the placed instance references the catalogued kind");
    assert_eq!(object.get("meshUrl").and_then(Value::as_str), Some(expected.as_str()), "the placed instance must carry its kind's representation url as its mesh identity");
}

/// 🌱️ A catalog TEMPLATE seats its vortex at `point`; a PLACED vortex carries `position`. The
/// non-migrated `addObjectKind` route seeded from `position` alone, so every seat of every catalogued
/// kind collapsed onto the object origin and the object it placed had a degenerate rim no brush could
/// use. Pins the real seats of a real shipped kind.
#[test]
fn catalogued_kind_templates_seed_vortices_at_their_catalog_points() {
    let scene_snapshot = crate::editor::puzzle3d::default_scene_snapshot();
    let entry = crate::editor::puzzle3d::puzzle3d_catalog_entries(&scene_snapshot, "objects").first().cloned().expect("the default document catalogues object kinds");
    let seats = crate::editor::puzzle3d::puzzle3d_vortices_from_kind_template(&entry);
    let templates = entry.get("vortices").and_then(|value| value.as_array()).expect("the catalogued kind declares vortex templates");
    assert_eq!(seats.len(), templates.len(), "one seeded vortex per declared template");
    assert!(seats.iter().any(|seat| seat.position != [0.0, 0.0, 0.0]), "a catalogued kind's seats must not all collapse onto the object origin: {seats:?}");
    for (seat, template) in seats.iter().zip(templates) {
        let point: [f64; 3] = template.get("point").and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()).expect("every template declares its point");
        assert_eq!(seat.position, point, "seeded vortex {} must sit on its template point", seat.id);
    }
}

/// 🛍️ Both doors onto the default document carry the same catalogue: the INITIAL snapshot the app
/// boots with, and `setActiveExample("concrete-forest")` — which replaces the catalogs through their
/// own `replace_kind_catalogs` mutation and could therefore drop them independently of the scene_snapshot.
#[semio_framework_async_macros::async_test]
async fn the_initial_snapshot_and_set_active_example_both_carry_the_concrete_forest_catalogue() {
    let declared: Vec<String> = crate::editor::puzzle3d::puzzle3d_catalog_entries(&crate::editor::puzzle3d::default_scene_snapshot(), "objects")
        .iter()
        .filter_map(|entry| entry.get("id").and_then(semio_framework_value::DslValue::as_str).map(str::to_string))
        .collect();
    assert!(!declared.is_empty(), "the concrete-forest scene_snapshot catalogues object kinds");
    let catalogued = |app: &Puzzle3dApp| -> Vec<String> {
        projection_of(app).pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).map(|rows| rows.iter().filter_map(|row| row.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default()
    };
    let mut app = app().await;
    assert_eq!(catalogued(&app), declared, "the app boots on the concrete-forest catalogue");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_CONCRETE_FOREST })), None).await.expect("concrete-forest");
    assert_eq!(catalogued(&app), declared, "switching back to concrete-forest restores its catalogue");
}

/// 🧊️ Wave B11 (checklist §10, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 12): the Volume
/// Brush armed and its W/D/H sliders moved, but Alt+click added nothing. This is the guest half of that
/// gesture — an `addTargetVolume` carrying the host's grid-snapped ground origin must land ONE volume
/// there, sized by the utility's own voxel dimensions times the grid spacing.
#[semio_framework_async_macros::async_test]
async fn alt_click_adds_one_target_volume_sized_by_the_utility_voxel_dims() {
    let volumes = |app: &Puzzle3dApp| projection_of(app).get("targetVolumes").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::volume_brush::UTILITY_ID })), None).await.expect("arm the volume brush");
    for (axis, value) in [("w", 2.0), ("d", 3.0), ("h", 4.0)] {
        dispatch(&mut app, "setVoxelDims", Some(&json!({ "axis": axis, "value": value })), Some(main::WINDOW_KIND_ID)).await.expect("voxel dimension");
    }
    let before = volumes(&app);
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [3.0, -2.0, 0.0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume");
    assert_eq!(volumes(&app), before + 1, "one Alt+click places exactly one target volume");
    let projection = projection_of(&app);
    let volume = projection.get("targetVolumes").and_then(Value::as_array).and_then(|entries| entries.last()).expect("the placed volume");
    let scale: Vec<f64> = volume.get("scale").and_then(Value::as_array).expect("the volume carries a scale").iter().filter_map(Value::as_f64).collect();
    assert_eq!(scale.len(), 3, "a voxel volume is a three-axis box: {scale:?}");
    assert!(scale[1] > scale[0] && scale[2] > scale[1], "the W/D/H steppers must size the placed box in their own order: {scale:?}");
    let origin: Vec<f64> = volume.get("origin").and_then(Value::as_array).expect("the volume carries an origin").iter().filter_map(Value::as_f64).collect();
    assert_eq!(origin.len(), 3, "the volume lands at the pointed ground point: {origin:?}");
    // 🔢️ A grid-snapped ground point is USUALLY whole (`[0,10,0]` was the live browser payload), and JSON
    // serializes a whole f64 as an integer — so the decode must admit integer carriers, not only floats.
    let whole = volumes(&app);
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [0, 10, 0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume with integer coordinates");
    assert_eq!(volumes(&app), whole + 1, "an integer-valued ground point must place a volume exactly like a fractional one");
}

/// 🧯️ …and a dispatch that carries no usable ground point answers with ONE localized notice instead of
/// returning silently, so a miss is distinguishable from a dead gesture.
#[semio_framework_async_macros::async_test]
async fn add_target_volume_without_a_ground_point_raises_one_notice_and_places_nothing() {
    let volumes = |app: &Puzzle3dApp| projection_of(app).get("targetVolumes").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    let mut app = app().await;
    let before = volumes(&app);
    let result = dispatch(&mut app, "addTargetVolume", None, Some(main::WINDOW_KIND_ID)).await.expect("a missing origin is a refusal, not a fault");
    let notices: Vec<&String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message),
            _ => None,
        })
        .collect();
    assert_eq!(notices.len(), 1, "an origin-less placement must say why: {:?}", result.requested_effects);
    assert_ne!(notices[0].as_str(), PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so the refusal must be real prose");
    assert_eq!(volumes(&app), before, "a refused placement adds nothing");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_ops_and_no_history_entry() {
    // 🧰️ Switching utilities is the framework-injected View action: no document operations, no undo
    // entry, no re-emitted utility-switch effect (the command IS the direct switch).
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), None).await.expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
    assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
}

#[test]
fn set_active_utility_dirties_the_world_body() {
    use crate::editor::puzzle3d::puzzle3d_command_scope_class;
    use crate::editor::puzzle3d::puzzle3d_scope;
    use crate::editor::puzzle3d::Puzzle3dScopeClass;
    assert_eq!(puzzle3d_command_scope_class(SET_ACTIVE_UTILITY_ACTION_ID), Puzzle3dScopeClass::Viewport, "utility switch must republish the world body (interaction + vortices), not Full chrome and not None");
    match puzzle3d_scope(Puzzle3dScopeClass::Viewport) {
        UiDirtyScope::Partial { window_bodies, .. } => assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]),
        other => panic!("utility switch scope must be the world body, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_exposes_no_utility_switch_options() {
    // 🧰️ select/brush/fill switching lives only on the framework utility bar; the engagement HUD
    // must not duplicate it as options. Object placement is catalogue drag-and-drop plus the shell
    // menu row `openAddObjectDialog`, not a per-viewport quick-action chip.
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let engagement = main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN, None);
    let options = engagement.options.as_deref().unwrap_or(&[]);
    assert!(options.is_empty(), "the puzzle3d engagement must not publish window quick-action options");
    assert!(options.iter().all(|option| !matches!(option.id.as_str(), "select" | "brush" | "fill" | "volumeBrush" | "worldRelocate" | "shell-menu.action.openAddObjectDialog")), "the puzzle3d engagement must not re-expose utilities or add-object chrome as options");
}

#[semio_framework_async_macros::async_test]
async fn transform_engagement_does_not_block_background_deselect() {
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
    assert_eq!(main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN, None).session_active, Some(false));
}

//#endregion 🔖️Utilities

//#region 🔖️WorldSelection
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the app-owned `worldSelect`
/// command is deleted — selection now goes exclusively through the framework's `interactionSelect`
/// verb (`select_id`), which is view-only by construction (`dispatch_interaction_action` never
/// touches `self.store`). Proves the `vortex` domain wiring reaches that same guarantee.
#[semio_framework_async_macros::async_test]
async fn world_select_emits_no_artifact_mutations() {
    let mut app = app().await;
    let before = projection_of(&app);
    let object_id = first_object_id(&app);
    let result = select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    assert!(result.mutations.is_empty(), "interactionSelect is framework-owned and view-only, must not diff the document");
    assert_eq!(projection_of(&app), before);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection paint rides on
/// `selectionJson` only — instance GEOMETRY stays byte-stable across a pick, so the host never has to
/// re-upload meshes just because something was selected.
#[semio_framework_async_macros::async_test]
async fn world_pick_keeps_instances_geometry_json_stable() {
    let mut app = app().await;
    let instances_before = instances_of(&render_composite(&mut app).await);
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let after = render_composite(&mut app).await;
    assert_eq!(instances_of(&after), instances_before, "picking must never perturb instance geometry");
    assert_eq!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()), Some(vec![object_id.clone()]));
    assert_eq!(
        selection_of(&after).get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![object_id]),
        "`selectionJson.ids` is what World3dHost paints instance selection from"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selectionJson` carries the exact
/// field names `World3dHost`'s `parseSelection`/`WorldSelectionRecord` reads — `ids` for object
/// instances, `targetVolumeIds`, `referenceSelectedId`, `hoveredId`. Vortex marks deliberately do NOT
/// appear here (that record has no vortex field; see `world_selection_json`'s doc comment).
#[semio_framework_async_macros::async_test]
async fn world_selection_json_carries_the_host_field_names_per_granularity() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(&object_id)).await.expect("hover object");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(selection.get("hoveredId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(selection.get("targetVolumeIds").and_then(Value::as_array).map(Vec::len), Some(0));
    assert!(selection.get("vortexIds").is_none(), "the host's WorldSelectionRecord has no vortexIds field");

    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [0.0, 0.0, 0.0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.last().cloned()).and_then(|volume| volume.get("id").and_then(Value::as_str).map(str::to_string)).expect("target volume id");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select target volume");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(
        selection.get("targetVolumeIds").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![volume_id]),
        "a target-volume granularity selection paints through `targetVolumeIds`"
    );
    assert_eq!(selection.get("ids").and_then(Value::as_array).map(Vec::len), Some(0), "object `ids` stay empty while a target volume is the live granularity");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the world scene binds `domainId` (and
/// its own `domainGranularityId`) so `World3dHost` dispatches the generic `interactionSelect`/
/// `interactionHover` verbs — this crate has no handler for the legacy `worldPick`/`worldSelect`/
/// `setHover` fallbacks the host would otherwise use.
#[semio_framework_async_macros::async_test]
async fn world_scene_binds_the_vortex_interaction_domain_and_its_granularity() {
    let mut app = app().await;
    let scene = render_composite(&mut app).await.get("world3d").cloned().unwrap_or(Value::Null);
    assert_eq!(scene.get("domainId").and_then(Value::as_str), Some(PUZZLE3D_INTERACTION_DOMAIN));
    assert_eq!(scene.get("domainGranularityId").and_then(Value::as_str), Some(PUZZLE3D_GRANULARITY_OBJECT));
}

#[semio_framework_async_macros::async_test]
async fn world_pick_null_clears_without_reselecting_first_object() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| !selection.ids.is_empty()));
    dispatch(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": "[]", "merge": "replace", "method": "pick" })),
        None,
    )
    .await
    .expect("empty-target interactionSelect");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "clicking empty background must clear, never fall back to reselecting the first object");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove that
/// world-picking a LOCKED object clears the selection instead of selecting it — the app-owned
/// `worldPick` command, which could read `object.locked` before deciding, is deleted.
/// `interactionSelect` is a framework-generic id/merge verb blind to app data, and this app's
/// `interaction_topology` does not filter locked ids out of the `vortex` domain either (see that
/// function's doc) — "never select a locked object" is now entirely a host click→pick
/// translation concern, not reachable from this crate. Proves the Rust-side floor instead: the
/// `vortex` domain has no lock awareness, so selecting a locked object's id still succeeds.
#[semio_framework_async_macros::async_test]
async fn world_pick_locked_object_clears_like_background() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "flag": "locked", "value": true })), None).await.expect("lock");
    let instances = instances_of(&render_composite(&mut app).await);
    assert_eq!(instances.first().and_then(|entry| entry.get("disabled")).and_then(Value::as_bool), Some(true));
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select locked object");
    assert_eq!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()),
        Some(vec![object_id]),
        "the vortex domain has no lock awareness — this now succeeds, the host must gate locked picks itself"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `PUZZLE3D_VORTEX_SHOW_SELECTED`
/// reveals a marker only while its own object (or one of its markers) is selected/hovered — the
/// live read `render_with_request_context` threads in as a `Puzzle3dInteractionSnapshot`.
#[semio_framework_async_macros::async_test]
async fn world_vortices_reveal_in_selected_mode_only_for_the_selected_object() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "scene_snapshot must expose vortices");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode with nothing selected reveals no vortex marker");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let revealed = vortices_of(&render_composite(&mut app).await);
    assert!(!revealed.is_empty(), "Selected mode must reveal the selected object's own vortex markers");
    assert!(revealed.iter().all(|vortex| vortex.get("objectId").and_then(Value::as_str) == Some(object_id.as_str())), "Selected mode must reveal ONLY the selected object's markers");
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "clearing the selection hides the markers again");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    assert!(!vortices_of(&render_composite(&mut app).await).is_empty(), "Always mode must still reveal every vortex marker");
}

/// 🕹️ A selected vortex marker carries its own `selected` flag on `vorticesJson` — the host's
/// `WorldVortexMarkers` reads it off each record (`WorldSelectionRecord` has no vortex field), and a
/// hovered marker carries `hovered` the same way.
#[semio_framework_async_macros::async_test]
async fn world_vortices_carry_their_own_selected_and_hovered_flags() {
    let mut app = app().await;
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selected = vortices_of(&render_composite(&mut app).await);
    let record = selected.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("selected vortex record");
    assert_eq!(record.get("selected").and_then(Value::as_bool), Some(true));
    assert!(selected.iter().filter(|entry| entry.get("fullId").and_then(Value::as_str) != Some(vortex.as_str())).all(|entry| entry.get("selected").and_then(Value::as_bool) == Some(false)), "only the picked marker is selected");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover vortex");
    let hovered = vortices_of(&render_composite(&mut app).await);
    let record = hovered.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("hovered vortex record");
    assert_eq!(record.get("hovered").and_then(Value::as_bool), Some(true));
    assert_eq!(interaction_of(&render_composite(&mut app).await).get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `worldVortexSelect`/`worldPick`
/// are deleted; a `vortex`-domain `DomainSelection` only ever carries one granularity at a time
/// (see `Puzzle3dActionCtx::selected_ids`'s doc), so a `merge: "replace"` pick at a different
/// granularity inherently replaces the whole prior selection — verified against
/// `interaction_state()` (the render-time `selectionJson`/`vorticesJson` fields carry no live ids
/// any more, per `world_selection_json`'s known-gap doc comment).
#[semio_framework_async_macros::async_test]
async fn world_pick_object_replaces_vortex_selection() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids, vec![object_id]);
}

#[semio_framework_async_macros::async_test]
async fn world_vortex_select_clears_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_VORTEX);
    assert_eq!(selection.ids, vec![vortex]);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove a
/// PERSISTED "default merge mode" (`selection_mode_default`, a config field — on this crate's
/// DELETE list) flips `worldVortexSelect`'s implicit merge between replace/invertive. The
/// framework has no equivalent persisted concept: `interactionSelect`'s `merge` arg is supplied
/// explicitly on every dispatch (the host decides per click — e.g. a held modifier key — never
/// defaulted from stored state). Proves the underlying `merge: "invertive"` primitive still
/// toggles a second target back into the selection instead.
#[semio_framework_async_macros::async_test]
async fn world_vortex_click_replaces_until_invertive_mode_is_selected() {
    let mut app = app().await;
    let vortices = vortex_full_ids(&app);
    assert!(vortices.len() >= 2, "scene_snapshot must expose two vortices");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[0]).await.expect("select first vortex");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[1]).await.expect("replace with second vortex");
    let replaced = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(replaced.ids, vec![vortices[1].clone()]);

    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), id: vortices[0].clone() }]);
    dispatch(&mut app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "invertive", "method": "pick" })), None).await.expect("invertive toggle");
    let invertive = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(invertive.ids.len(), 2, "invertive merge toggles the first vortex back into the selection alongside the second");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: "Select Same Kind" emits a real
/// `Emit.interaction_writes` entry, which `VcsArtifactApp` applies through the same `next_selection`
/// machine the reserved `interactionSelect` verb uses — so the widened selection is observable on
/// `interaction_state()` and painted into `selectionJson` on the next render.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_widens_the_selection_to_every_object_of_that_kind() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    for _ in 0..3 {
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    }
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert_eq!(ids.len(), 3, "three same-kind objects must exist");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &ids[0]).await.expect("select one");
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    let mut widened = selection.ids.clone();
    widened.sort();
    let mut expected = ids.clone();
    expected.sort();
    assert_eq!(widened, expected, "every object of the clicked object's kind is selected");
    let painted = selection_of(&render_composite(&mut app).await).get("ids").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
    let mut painted_sorted = painted.clone();
    painted_sorted.sort();
    assert_eq!(painted_sorted, expected, "the widened selection reaches the host through selectionJson");
}

/// 🕹️ "Select Same Kind" with nothing selected still aborts (nothing to widen from) and leaves the
/// selection untouched.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_with_no_selection_leaves_the_selection_untouched() {
    let mut app = app().await;
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "widening from nothing must not invent a selection");
}

/// 🎯️ A selection-scoped command dispatched with NOTHING selected must refuse VISIBLY: exactly one
/// `Effect::Notify` and no document edit. This is the browser defect measured
/// 2026-09-09 21:05 — "Duplicate Selection" from the context menu completed with no clone, no command
/// row and no notice of any kind, which is indistinguishable from a dead menu entry. Every arm that
/// reads the framework-owned selection is covered here, so the refusal cannot be reintroduced one
/// command at a time.
///
/// 🧾️ A command-log ROW is deliberately not asserted absent: `dispatch_emit` records one row per
/// dispatch by design, mutation-kind actions that produced zero operations included (see its own doc
/// in `🔌️plugin/🦀️.rs`), so a refused `Mutation` still shows up in history with no edit id. What must
/// never happen is a silent nothing — that is what the notice below pins.
#[semio_framework_async_macros::async_test]
async fn selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    // 🌱️ The seed gesture SELECTS what it adds (`add_object_kind_selects_the_object_it_adds`), so the
    // empty precondition this law is about has to be established explicitly — otherwise every arm below
    // measures a live selection and refuses nothing (wave B36).
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear the seed gesture's own selection");
    let objects_before = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    for action in ["duplicateSelection", "deleteSelection", "selectSameKindSelection"] {
        let result = dispatch(&mut app, action, None, None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(!matches!(result.ui_scope, UiDirtyScope::Full), "{action} painted nothing, so it must not force a full refresh");
    }
    // 🧲️ The three gumball verbs run the transform tool (`Puzzle3dTransformWork`), whose whole outcome
    // (notice, scope) lives on the completion lane that `context::settle` drains into `InvocationResult`: a
    // gesture addressing nothing refuses with one notice and paints nothing.
    for (action, args) in [
        ("translateSelection", json!({ "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
        ("rotateSelection", json!({ "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 1.0 })),
        ("scaleSelection", json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })),
    ] {
        let result = dispatch(&mut app, action, Some(&args), None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice on its completion lane: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(matches!(result.ui_scope, UiDirtyScope::None), "{action} painted nothing, so its completion must carry UiDirtyScope::None, got {:?}", result.ui_scope);
    }
    let objects_after = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    assert_eq!(objects_after, objects_before, "a refused selection command must leave the document untouched");
    assert!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
        "a refused selection command must not invent a selection either"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `duplicateSelection` re-selects the
/// clones it created — the interaction write is applied AFTER the document mutations land, so the new
/// ids are already in `interaction_topology` and survive `validate_state`'s pruning.
#[semio_framework_async_macros::async_test]
async fn duplicate_selection_reselects_the_created_clones() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "duplicateSelection", None, None).await.expect("duplicateSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids.len(), 1, "exactly the one clone is selected");
    assert_ne!(selection.ids.first().map(String::as_str), Some(object_id.as_str()), "the CLONE is selected, not the original");
    let live_ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert!(live_ids.contains(selection.ids.first().expect("clone id")), "the re-selected id must exist in the document");
}

/// 🌱️ `addObjectKind` SELECTS the object it adds — the same `Emit.interaction_writes` contract
/// `duplicateSelection` and `addBrushObject` already honour, on both of its implementations: the
/// cursorized `Puzzle3dAddObjectKindWork` the interactive job runs and the direct reducer arm. Adding an
/// object the user must then hunt for is half a gesture: the catalogue row, the drag-drop and the Add
/// Object dialog all end with the new object nowhere in the inspector, the gumball or the outliner
/// (battery #53 `catalogue-add-selects-new-object added=1`, ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B36). Every id is asserted against the live document, so a write naming a phantom fails here.
#[semio_framework_async_macros::async_test]
async fn add_object_kind_selects_the_object_it_adds() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("first object");
    let first = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(first.granularity, PUZZLE3D_GRANULARITY_OBJECT, "the add selects at OBJECT granularity: {first:?}");
    assert_eq!(first.ids.len(), 1, "exactly the added object is selected: {first:?}");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [4.0, 0.0, 0.0] })), None).await.expect("second object");
    let second = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(second.ids.len(), 1, "the second add REPLACES the selection with its own object, it does not widen it: {second:?}");
    assert_ne!(second.ids, first.ids, "the second add selects the object IT created: {second:?} vs {first:?}");
    let live_ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert_eq!(live_ids.len(), 2, "both adds landed in the document: {live_ids:?}");
    assert!(live_ids.contains(second.ids.first().expect("added id")), "the selected id is the one the document carries: {live_ids:?}");
}

/// 🗑️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31: one `deleteSelection` on a live object selection
/// removes that object from the RENDERED world census within the same settle, records exactly one
/// command-log row, and leaves the deleted id out of the framework-owned selection — so pressing
/// `Delete` twice cannot land a second, empty edit on an id the document no longer carries. The
/// browser red this states was `before=2 after=2 waitedMs=30299` with
/// `history patch applied labels=["delete-object id=object-1"]` in the same run: the document moved
/// and the selection did not, so every later delete was a no-op against a phantom.
#[semio_framework_async_macros::async_test]
async fn delete_selection_shrinks_the_world_census_and_drops_the_deleted_id() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("first object");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("second object");
    let census_before = instance_count(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(census_before, 2, "two added objects must both be in the rendered world census");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let (deleted, settled) = dispatch_reporting(&mut app, "deleteSelection", None, None).await;
    deleted.expect("deleteSelection");
    assert_eq!(history_rows(&settled), 1, "a delete that removed an object must publish exactly one command-log row");
    assert_eq!(object_count(&app), 1, "the document must carry one object fewer");
    assert_eq!(instance_count(&render_window(&mut app, main::WINDOW_KIND_ID).await), census_before - 1, "the rendered world census must shrink by one within the same settle");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert!(!selection.ids.contains(&object_id), "the deleted id must be gone from the selection, else a second Delete is a silent no-op: {:?}", selection.ids);
    let (again, settled_again) = dispatch_reporting(&mut app, "deleteSelection", None, None).await;
    again.expect("second deleteSelection");
    // 🧾️ A Mutation that emits nothing still logs one row (`edit_id: None`), on THIS settle — the same
    // empty-emit row `dispatch_emit` files under "Without Operations". What must not happen is an edit.
    let again_rows = history_row_labels(&settled_again);
    assert_eq!(history_rows(&settled_again), 1, "a delete with nothing selected logs the command and no edit: {again_rows:?}");
    assert!(again_rows.iter().all(|row| row.contains("deleteSelection") && row.contains("applied=false") && row.contains("ops=0")), "{again_rows:?}");
    assert_eq!(object_count(&app), 1, "and must not touch the document");
}
//#endregion 🔖️WorldSelection

//#region 🔖️Gumball
/// 🕹️ Framework leftover `interactionView` must not arm the gumball on a bare pick — only the guest
/// `selectionJson` lane may set `gumballActive`, and only when the transform utility is active.
#[semio_framework_async_macros::async_test]
async fn interaction_select_leftover_does_not_arm_gumball_without_transform_utility() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), id: object_id.clone() }]);
    let admitted = dispatch_reserved_unsettled(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })),
        Some(main::WINDOW_KIND_ID),
    )
    .await
    .expect("interactionSelect admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("interactionSelect settle");
    let gumball = settled
        .output
        .get("interactionView")
        .and_then(|view| view.get("gumball"))
        .expect("leftover gumball");
    assert_eq!(gumball.get("active").and_then(semio_framework_value::DslValue::as_bool), Some(false));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `gumballActive` requires BOTH the
/// transform utility active AND a live object (or target-volume) selection — the live read
/// `render_with_request_context` threads in. `transformMode`/`gumballConfig` depend only on the
/// active utility, per window.
#[semio_framework_async_macros::async_test]
async fn gumball_active_only_for_transform_utilities_with_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let idle_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(idle_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "selection alone must not show the gumball");
    assert!(idle_selection.get("transformMode").is_none(), "non-transform utility must not emit transformMode");

    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    let transform_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(transform_selection.get("gumballActive").and_then(Value::as_bool), Some(true), "transform utility plus a live object selection shows the gumball");
    assert_eq!(transform_selection.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()), Some(vec![object_id.clone()]));
    assert_eq!(transform_selection.get("activeObjectId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(transform_selection.get("transformMode").and_then(Value::as_str), Some("transform"));
    assert_eq!(transform_selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(transform_selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(true));

    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert_eq!(selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).get("gumballActive").and_then(Value::as_bool), Some(false), "an unattached gumball must never render");

    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("reselect");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let brush_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(brush_selection.get("gumballActive").and_then(Value::as_bool), Some(false));
    assert!(brush_selection.get("transformMode").is_none());
}

/// 🕹️ Both handle flags off leaves nothing to grab, so the gumball must not render even with a live
/// selection and the transform utility active (`setTransformGumballFlag` is what the user toggles).
#[semio_framework_async_macros::async_test]
async fn gumball_inactive_when_every_handle_flag_is_off() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "move", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("no move");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("no rotate");
    let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(selection.get("gumballActive").and_then(Value::as_bool), Some(false), "a gumball with no handles must not render");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `transformMode` depends only on
/// each window instance's host-owned `ViewModel` utility map, so it is the per-window-isolation proof.
#[semio_framework_async_macros::async_test]
async fn transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(top)).await.expect("transform on top");
    let top_selection = selection_of(&render_window(&mut app, top).await);
    assert_eq!(top_selection.get("transformMode").and_then(Value::as_str), Some("transform"), "transform on top pane must switch that pane's own scene mode");
    let perspective_selection = selection_of(&render_window(&mut app, perspective).await);
    assert!(perspective_selection.get("transformMode").is_none(), "perspective pane must not inherit top pane's transform utility");
}

#[semio_framework_async_macros::async_test]
async fn transform_utility_options_expose_move_and_rotate_flags() {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
    let measures = main::window_measures(&scene, labels);
    assert_eq!(measure_group_tag(&measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform")), Some(Some(utilities::transform::UTILITY_ID.into())));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-move"), Some(true));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-rotate"), Some(true));
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("disable rotate");
    let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(false));
    let view = app.window_view(main::WINDOW_KIND_ID);
    let app_measures = app.window_measures(&view).await;
    let window_measures = app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_toggle(window_measures, "puzzle3d-transform-rotate"), Some(false));
}

fn object_origin(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("origin").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

fn object_orientation(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("orientation").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

fn object_scale(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("scale").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

/// 🧾️ The applied history rows one settle upserted that carry document ops.
fn edit_rows(settled: &Puzzle3dSettled) -> Vec<semio_framework::kernel::HistoryEntry> {
    settled.completions.iter().filter_map(|completion| completion.history_patch.as_ref()).flat_map(|patch| patch.upserts.iter()).filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()
}

fn english(entry: &semio_framework::kernel::HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string()
}

fn german(entry: &semio_framework::kernel::HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).to_string()
}

/// 🌱️ An empty scene with one fresh object, the object selected and the Transform utility active.
async fn gumball_app() -> (Puzzle3dApp, String) {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    (app, object_id)
}

/// 🛠️ One gumball drag — the ONE pose delta both hosts dispatch on release — is ONE tool transaction: one
/// edit, one history row stamped with the transform tool's `TransactionRef`, whose op is the parametric
/// `drag-selection` over the gesture's literal ids and offset, labelled from the leaf in English and German.
/// One undo restores the pose exactly.
#[semio_framework_async_macros::async_test]
async fn one_gumball_translate_is_one_edit_one_row_and_one_transaction() {
    let (mut app, object_id) = gumball_app().await;
    let start = object_origin(&app, &object_id);
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 6.0, "dy": 0.0, "dz": 0.0 })), None).await;
    result.expect("the release delta commits");
    let rows = edit_rows(&settled);
    assert_eq!(rows.len(), 1, "one gesture, one history row: {}", history_row_labels(&settled).join(" | "));
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.puzzle.puzzle3d@1/*#editor#translateSelection", "the transform tool authored it: {transaction:?}");
    assert!(rows[0].op_lines.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection"), "the one op is the parametric leaf: {:?}", rows[0].op_lines);
    assert_eq!((english(&rows[0]), german(&rows[0])), ("Drag 1 item by (6, 0, 0)".to_string(), "1 Element um (6; 0; 0) ziehen".to_string()), "the row is labelled from the leaf");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 6.0).abs() < 1e-9, "the offset lands verbatim on the document");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole gesture");
}

/// 🧮️ Two gestures are two transactions — two rows with distinct refs — and one undo takes back only the
/// second: gestures never coalesce into one another.
#[semio_framework_async_macros::async_test]
async fn two_gumball_gestures_are_two_transactions() {
    let (mut app, object_id) = gumball_app().await;
    let start = object_origin(&app, &object_id);
    let (_, first) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), None).await;
    let (_, second) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 2.0, "dy": 0.0, "dz": 0.0 })), None).await;
    let (first, second) = (edit_rows(&first), edit_rows(&second));
    assert_eq!((first.len(), second.len()), (1, 1), "each gesture is its own row");
    assert_ne!(first[0].transaction.as_ref().map(|transaction| transaction.id.clone()), second[0].transaction.as_ref().map(|transaction| transaction.id.clone()), "two gestures mint two transactions");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 1.0).abs() < 1e-9, "one undo takes back only the second gesture");
}

/// 🔄️ A gumball turn and a gumball scaling are ONE transaction each, yielding `rotate-selection` and
/// `scale-selection`, and one undo restores the exact base pose (absolute setters, never a negated angle).
#[semio_framework_async_macros::async_test]
async fn gumball_rotate_and_scale_are_one_transaction_each() {
    let (mut app, object_id) = gumball_app().await;
    let (orientation, scale) = (object_orientation(&app, &object_id), object_scale(&app, &object_id));
    let (_, turned) = dispatch_reporting(&mut app, "rotateSelection", Some(&json!({ "ids": [object_id.as_str()], "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 0.75 })), None).await;
    let rows = edit_rows(&turned);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("rotate-selection") && rows[0].transaction.is_some(), "one turn, one transaction: {rows:?}");
    assert_ne!(object_orientation(&app, &object_id), orientation, "the turn moves the pose");
    dispatch(&mut app, "undo", None, None).await.expect("undo turn");
    assert_eq!(object_orientation(&app, &object_id), orientation, "one undo restores the orientation");
    let (_, grown) = dispatch_reporting(&mut app, "scaleSelection", Some(&json!({ "ids": [object_id.as_str()], "sx": 1.5, "sy": 1.0, "sz": 1.0 })), None).await;
    let rows = edit_rows(&grown);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("scale-selection") && rows[0].transaction.is_some(), "one scaling, one transaction: {rows:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo scaling");
    assert_eq!(object_scale(&app, &object_id), scale, "one undo restores the scale");
}

/// 🧲️ A gumball drag of an attracting object re-solves the attraction graph inside the SAME transaction: on
/// Nakagin (which ships no attraction, so the law first attracts the compatible door pair `25b0dba0-…:link` →
/// `5f0266bc-…:sl0_d0`) the object the attraction hangs off the dragged one is re-placed from it — one row, one
/// `drag-selection` op whose leaf carries the follower — and one undo restores both poses exactly.
#[semio_framework_async_macros::async_test]
async fn a_gumball_drag_carries_its_attracted_objects_in_the_same_transaction() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    dispatch(&mut app, "createAttraction", Some(&json!({ "attracting": "25b0dba0-8f81-423a-94a1-b911a6031010:link", "attracted": "5f0266bc-856b-4ef2-9eb0-16ef5e1fb952:sl0_d0" })), None).await.expect("attract the compatible door pair");
    let scene_snapshot = puzzle3d_scene_snapshot_from_document(app.snapshot().expect("typed app snapshot").typed());
    let owner = |full_id: &str| full_id.split(':').next().unwrap_or_default().to_string();
    let (parent, child) = scene_snapshot
        .attractions
        .iter()
        .map(|attraction| (owner(&attraction.attracting), owner(&attraction.attracted)))
        .find(|(parent, child)| parent != child && scene_snapshot.objects.iter().any(|object| &object.id == child && !object.locked) && scene_snapshot.objects.iter().any(|object| &object.id == parent && !object.locked))
        .expect("Nakagin carries an attraction between two unlocked objects");
    let (parent_start, child_start) = (object_origin(&app, &parent), object_origin(&app, &child));
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [parent.as_str()], "dx": 0.0, "dy": 0.0, "dz": 2.0 })), None).await;
    result.expect("the release delta commits");
    let rows = edit_rows(&settled);
    assert!(rows.len() == 1 && rows[0].transaction.is_some() && rows[0].op_lines.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection"), "one gesture, one row, one parametric op: {rows:?}");
    assert!((object_origin(&app, &parent)[2] - parent_start[2] - 2.0).abs() < 1e-9, "the dragged object lifts by the offset");
    assert_ne!(object_origin(&app, &child), child_start, "the attracted object follows its moved parent");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!((object_origin(&app, &parent), object_origin(&app, &child)), (parent_start, child_start), "one undo restores the dragged object AND its follower");
}

/// 🧯️ A motionless release — the drag that went nowhere — leaves zero trace: no edit, no row, no notice.
#[semio_framework_async_macros::async_test]
async fn a_motionless_gumball_release_leaves_zero_trace() {
    let (mut app, object_id) = gumball_app().await;
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 0.0, "dy": 0.0, "dz": 0.0 })), None).await;
    let result = result.expect("a motionless release completes");
    let noticed = result.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. }));
    assert!(edit_rows(&settled).is_empty() && result.mutations.is_empty() && !noticed, "nothing moved, nothing recorded: {}", history_row_labels(&settled).join(" | "));
}

/// 🛠️ LAW (audit Z4) — the World3d host corpus' LOCAL one-shot cases (`🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json`,
/// `live: false`, the protocol puzzle 3d speaks: one net delta on release, no stream phase): every verb dispatch the host
/// owes for a scripted gesture, sent with its exact wire args (the targets swapped for the scene_snapshot's own object), publishes
/// exactly the case's guest edits and moves the object by its offset; a gesture that dispatches nothing publishes nothing.
#[semio_framework_async_macros::async_test]
async fn the_world3d_local_gumball_cases_land_as_their_guest_edits() {
    let corpus = parse(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json")), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the protocol corpus parses");
    let cases: Vec<&Value> = corpus.get("cases").and_then(Value::as_array).expect("cases").iter().filter(|case| case.get("live").and_then(Value::as_bool) == Some(false)).collect();
    assert_eq!(cases.len(), 4, "the corpus carries the four local one-shot cases");
    for case in cases {
        let name = case.get("name").and_then(Value::as_str).expect("name");
        let (mut app, object_id) = gumball_app().await;
        let before = object_origin(&app, &object_id);
        let mut edits = 0;
        for dispatch in case.get("steps").and_then(Value::as_array).expect("steps").iter().filter_map(|step| step.get("dispatch")).filter(|dispatch| !dispatch.is_null()) {
            let action = dispatch.get("action").and_then(Value::as_str).expect("action");
            let mut args = dispatch.get("args").cloned().expect("args");
            args.as_object_mut().expect("object args").insert("ids", json!([object_id.as_str()]));
            let (result, settled) = dispatch_reporting(&mut app, action, Some(&args), None).await;
            result.unwrap_or_else(|fault| panic!("{name}: {action} dispatches from the host's wire args: {fault:?}"));
            edits += edit_rows(&settled).len();
        }
        let guest = case.get("guest");
        let expected_edits = guest.and_then(|guest| guest.get("edits")).and_then(Value::as_u64).unwrap_or(0);
        let offset: Vec<f64> = guest.and_then(|guest| guest.get("offset")).and_then(Value::as_array).map_or_else(|| vec![0.0; 3], |values| values.iter().filter_map(Value::as_f64).collect());
        let after = object_origin(&app, &object_id);
        assert_eq!(edits as u64, expected_edits, "{name}: the published edits");
        assert!((0..3).all(|axis| (after[axis] - before[axis] - offset[axis]).abs() < 1e-9), "{name}: the object's offset {:?} -> {:?}, expected {offset:?}", before, after);
    }
}

//#endregion 🔖️Gumball

//#region 🔖️HistoryEditReferences
/// 🪧️ N3 (design §16.4): a history-edit reference chip names a puzzle 3d entity as the outliner does — an unlabelled
/// object by its kind's catalog label, a vortex as `<object> · <vortex>`, an attraction as `<object> → <object>` — and
/// leaves target volumes, undeclared kinds and unknown ids to the framework's generic `<Kind> <short id>`.
#[test]
fn history_edit_reference_chips_name_entities_as_the_outliner_does() {
    use semio_s_artifact_puzzle_3d::standards::v1::subsets::any::io::text::snapshot::parse_dsl;
    use semio_s_artifact_puzzle_3d::standards::v1::subsets::any::io::text::snapshot::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT;
    let mut snapshot = parse_dsl(PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("the example parses");
    let object = snapshot.objects[0].id.clone();
    let vortex = puzzle3d_vortex_full_id(&object, &snapshot.objects[0].vortices[0].id);
    let vortex_label = snapshot.objects[0].vortices[0].label.clone().expect("the example labels its vortices");
    let kind = snapshot.objects[0].object_kind.clone().expect("the seed names its kind");
    snapshot.objects[0].label = None;
    snapshot.meta.kind_catalogs.as_mut().expect("the example ships its kind catalogs").objects.iter_mut().find(|entry| entry.id == kind).expect("the seed's kind is catalogued").label = "Forest Left".into();
    snapshot.attractions.push(crate::Puzzle3dAttraction { id: "attraction-chip".into(), attracting: vortex.clone(), attracted: vortex.clone(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    let chip = |kinds: &[&str], id: &str| puzzle3d_entity_label(&snapshot, &kinds.iter().map(|kind| kind.to_string()).collect::<Vec<_>>(), id).map(|label| label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string());
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_TARGET_VOLUME], &object).as_deref(), Some("Forest Left"), "an unlabelled object reads its kind's catalog label");
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_VORTEX], &vortex), Some(format!("Forest Left \u{b7} {vortex_label}")), "a vortex reads its object and its own label");
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_ATTRACTION], "attraction-chip").as_deref(), Some("Forest Left \u{2192} Forest Left"), "an attraction reads its two objects");
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_TARGET_VOLUME], &object), None, "an id outside the reference's declared kinds keeps the generic label");
    assert_eq!(chip(&[], "ghost"), None, "an unknown id keeps the generic label");
}

/// 🔗️ N3 (design §16.4): while a history edit of a gesture is open, the world paints exactly the objects its draft
/// references highlighted (one changed instance record, so the delta lane carries it) — and none once the edit exits.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_draft_highlights_exactly_the_objects_it_references() {
    let (mut app, object_id) = gumball_app().await;
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), None).await;
    result.expect("the drag commits");
    let mutation_id = edit_rows(&settled).first().and_then(|row| row.mutations.first()).map(|mutation| mutation.mutation_id.clone()).expect("the drag is one editable mutation");
    let highlighted = |node: &Value| instances_of(node).into_iter().filter(|instance| instance.get("highlighted").and_then(Value::as_bool) == Some(true)).filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect::<Vec<_>>();
    assert!(highlighted(&render_composite(&mut app).await).is_empty(), "nothing is highlighted before a draft opens");
    let admitted = dispatch_reserved_unsettled(&mut app, semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, Some(&json!({ "mutationId": mutation_id })), None).await.expect("historyEditBegin admits");
    settle_reserved(&mut app, admitted).await.expect("historyEditBegin settles");
    assert_eq!(highlighted(&render_composite(&mut app).await), vec![object_id.clone()], "the draft's target is painted highlighted");
    let admitted = dispatch_reserved_unsettled(&mut app, semio_framework::HISTORY_EDIT_EXIT_ACTION_ID, None, None).await.expect("historyEditExit admits");
    settle_reserved(&mut app, admitted).await.expect("historyEditExit settles");
    assert!(highlighted(&render_composite(&mut app).await).is_empty(), "closing the edit clears the highlight");
}
//#endregion 🔖️HistoryEditReferences

//#region 🔖️KitInPort
/// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like block3d's
/// `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
/// `Puzzle3dPlayApp::import_media` must normalize `objectKinds` → `objects` / `vortexKinds` →
/// `vortices` and, after applying the returned operations, land that object kind inside
/// `meta.kind_catalogs.objects` (and the vortex kind inside `.vortices`).
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_upserts_object_and_vortex_kinds_into_meta_kind_catalogs() {
    let projection = Puzzle3dPlayApp::initial_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);

    let fragment = json!({
        "schema": "manifest",
        "objectKinds": [{
            "id": "capsule",
            "name": "capsule",
            "label": "Capsule",
            "meshUrl": "/mesh/capsule.glb",
            "vortices": [{ "id": "v0", "vortexKind": "door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
        }],
        "vortexKinds": [{ "id": "door", "name": "door", "label": "Door", "color": "#ff0000", "defaultCableKind": "" }],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [{ "source": "door", "target": "door", "bidirectional": true }],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
    assert!(!emit.artifact_mutations.is_empty(), "importing a non-empty fragment must emit real operations");

    let mut next_projection = projection.typed().clone();
    for operation in &emit.artifact_mutations {
        next_projection = protocol::apply_diff_cold(protocol::Mutation::<crate::Puzzle3dSnapshot>::diff(operation, &next_projection).diff(), &next_projection).expect("valid mutation diff");
    }

    let next_projection = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&next_projection));
    let objects = next_projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert!(objects.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")), "the imported object kind must appear in meta.kind_catalogs.objects");
    let capsule = objects.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).unwrap();
    assert_eq!(capsule.pointer("/representations/0/url").and_then(Value::as_str), Some("/mesh/capsule.glb"));
    assert_eq!(capsule.pointer("/vortices/0/vortexKind").and_then(Value::as_str), Some("door"), "the per-object vortex template keeps its vortexKind after normalization");

    let vortices = next_projection.pointer("/meta/kindCatalogs/vortices").and_then(Value::as_array).expect("vortices catalog present");
    assert!(vortices.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("door")), "the imported vortex kind must appear in meta.kind_catalogs.vortices");

    let compatibility = next_projection.pointer("/meta/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
    assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
}

/// 🔁️ Re-importing the SAME fragment (a second producer edge, or a redelivered message on a
/// `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_is_idempotent_on_repeated_delivery() {
    let projection = Puzzle3dPlayApp::initial_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let mut current = projection.typed().clone();

    let fragment = json!({
        "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
        "vortexKinds": [],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    for _ in 0..2 {
        let doc_projection = Puzzle3dPlaySnapshot::new(current.clone());
        let doc = ArtifactView::new(&doc_projection, &history);
        let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
        for operation in &emit.artifact_mutations {
            current = protocol::apply_diff_cold(protocol::Mutation::<crate::Puzzle3dSnapshot>::diff(operation, &current).diff(), &current).expect("valid mutation diff");
        }
    }

    let current = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&current));
    let objects = current.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert_eq!(objects.iter().filter(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
}

#[semio_framework_async_macros::async_test]
async fn kit_in_port_is_declared_on_the_app_io() {
    let io = Puzzle3dPlayApp::io().expect("puzzle3d declares an AppIo");
    let port = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
    assert_eq!(port.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(port.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    assert!(matches!(port.multiplicity, PortMultiplicity::Many));
}
//#endregion 🔖️KitInPort

//#region 🔖️Convergence
/// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT object edits and,
/// after exchanging operations, both converge to contain BOTH objects — impossible under
/// whole-document `setSnapshot` snapshots, which would clobber one side.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_object_edits_via_backbone() {
    use store::os_store::backbone::MemoryBackbone;
    let mut instance_a = app().await;
    let mut instance_b = app().await;
    let seeded = object_count(&instance_a);
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle3d-convergence", "mem://puzzle3d-convergence").await;
    instance_a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("a adds object");
    dispatch(&mut instance_b, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("b adds object");

    // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
    dispatch(&mut instance_a, "commitCheckpoint", None, None).await.expect("pump a");
    dispatch(&mut instance_b, "commitCheckpoint", None, None).await.expect("pump b");

    assert_eq!(object_count(&instance_a), seeded + 2, "instance A must contain both objects");
    assert_eq!(object_count(&instance_b), seeded + 2, "instance B must contain both objects");
    let _ = Puzzle3dCamera::default();
}

//#endregion 🔖️Convergence

//#region 🔖️Close
/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END: the app-close contract, stated exactly once and asserted
/// by a test rather than by a destructor. `VcsArtifactApp::close_step` walks seven owned lanes in a
/// fixed order (`document-store`, `config-store`, `draft-store`, `presence-store`, `transient-store`,
/// window-transient, `interaction-store`) and every lane is mandatory: `drive_artifact_owned_disposer`
/// faults `interactive-job.close-owned-disposer-missing` the moment `A::build_<lane>_disposer()`
/// returned `None`. Until that holds for every lane, no puzzle3d instance can ever reach its
/// terminal-empty witness — in a test process OR in a host, where the same `close_step` runs on
/// shutdown and the framework-installed `ArtifactStoreCursorDisposer` members then panic in `Drop`.
#[semio_framework_async_macros::async_test]
async fn fixture_app_reaches_its_terminal_empty_close_witness() {
    match close_witness(&mut app().await) {
        Ok(true) => {}
        Ok(false) => panic!("close_step reported Complete without reaching terminal-empty ownership"),
        Err(fault) => panic!("every owned close lane must supply its bounded disposer, got {fault:?}"),
    }
}
//#endregion 🔖️Close

/// 🎟️ Wave W-P: `Puzzle3dPlayApp` used to be rebuilt from `default()` on every dispatch and every render,
/// so `geometry_cache` was structurally unable to observe two calls in a row and the whole scene_snapshot was
/// re-serialized every time (`📓️2026-09-08-performance-architecture-audit.md` §1, fix #1). With a session
/// slot keyed by `app_instance_id`, the second call for the same document must serialize NOTHING.
#[test]
fn a_second_call_on_one_instance_reuses_the_geometry_cache_instead_of_reserializing() {
    let config = Puzzle3dRuntime::default();
    let scene_snapshot = default_scene_snapshot();
    let fingerprint = main::scene_geometry_fingerprint(&scene_snapshot);
    let session = Some((4_001_u32, Some("document-geometry".to_string())));
    let cold = PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get);
    let first = with_puzzle3d_app_for(session.clone(), &config, |app| app.geometry_jsons(&scene_snapshot, &std::collections::BTreeSet::new(), &[]));
    let after_first = PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get);
    assert_eq!(after_first - cold, 1, "the first call for a cold instance serializes exactly once");
    let second = with_puzzle3d_app_for(session, &config, |app| {
        let cached = app.mesh_cache.lock().expect("mesh cache");
        assert_eq!(cached.as_ref().map(|(cached, _)| *cached), Some(fingerprint), "the session slot handed the warm cache to a brand-new app object");
        drop(cached);
        assert_eq!(app.instance_residency.lock().expect("instance residency").as_ref().map(|residency| residency.revision()), Some(1), "the per-object instance residency came back with the slot too");
        app.geometry_jsons(&scene_snapshot, &std::collections::BTreeSet::new(), &[])
    });
    assert_eq!(PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get), after_first, "the second call on the same instance must not re-serialize anything");
    assert_eq!(first, second, "a cache hit returns byte-identical instance and mesh json");
}

/// 🥽️ Wave W-P: a mesh registered by one dispatch used to be gone by the next, because the collision
/// engine died with its app object (audit bottleneck (h)). A worker hop is exactly "a new app object with
/// the same instance id", so the resumed call must still hold the registered geometry.
#[test]
fn a_worker_hop_resume_still_holds_the_registered_brush_mesh() {
    let config = Puzzle3dRuntime::default();
    let session = Some((4_002_u32, Some("document-mesh".to_string())));
    let url = "/test/session-hop.glb";
    let positions: Vec<f32> = vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
    with_puzzle3d_app_for(session.clone(), &config, |app| {
        app.precompute.borrow_mut().register_mesh(url, &positions, &indices);
        assert!(app.precompute.borrow().has_mesh(url), "the registering dispatch sees its own mesh");
    });
    with_puzzle3d_app_for(session, &config, |app| {
        assert!(app.precompute.borrow().has_mesh(url), "the resumed dispatch adopted the same instance's registered mesh");
    });
    with_puzzle3d_app_for(Some((4_003_u32, Some("document-other".to_string()))), &config, |app| {
        assert!(app.precompute.borrow_mut().adopt_shared_mesh(url, None), "a different document reaches the same geometry by id alone");
    });
    with_puzzle3d_app_for(None, &config, |app| {
        assert!(!app.precompute.borrow().has_mesh(url), "a call with no instance identity stays session-less, exactly as before");
    });
}

/// 🎫 Wave W-P: instance ids are reused by the framework, so a slot re-keyed to a different parent
/// document must retire — and every lease taken against the previous document must fail to check in
/// rather than leak one document's cached geometry into another's render.
#[test]
fn a_stale_session_lease_is_rejected_and_a_rekeyed_instance_starts_cold() {
    let instance = 4_004_u32;
    let stale = {
        let mut registry = puzzle3d_session_registry().lock().expect("session registry");
        let (stale, held) = registry.check_out(instance, Some("document-a")).expect("first lease");
        assert!(held.instances.is_none() && held.meshes.is_none(), "a cold slot hands out no cached geometry");
        let (fresh, rekeyed) = registry.check_out(instance, Some("document-b")).expect("re-keyed lease");
        assert!(rekeyed.instances.is_none() && rekeyed.meshes.is_none(), "a re-keyed instance starts cold instead of adopting the previous document");
        assert_ne!(stale.generation, fresh.generation, "re-keying bumps the slot generation");
        registry.check_in(stale, Puzzle3dSessionState { meshes: Some((7, "stale-meshes".into())), ..Default::default() });
        stale
    };
    let mut registry = puzzle3d_session_registry().lock().expect("session registry");
    let (_, adopted) = registry.check_out(instance, Some("document-b")).expect("post-stale lease");
    assert!(adopted.instances.is_none() && adopted.meshes.is_none(), "the stale lease's state was refused, so document-b is still cold");
    assert_ne!(stale.generation, registry.generations[usize::try_from(instance).expect("slot base") % PUZZLE3D_SESSION_SLOTS], "the retired generation is never handed out again");
}

/// ⚖️ Wave W-P: the session census is a real bound, not a slot count — a check-in whose bytes would cross
/// `PUZZLE3D_SESSION_PROCESS_BYTES` is dropped, which costs one cold rebuild and never corrupts anything.
#[test]
fn a_session_check_in_over_the_process_byte_ceiling_is_dropped() {
    let mut registry = Puzzle3dSessionRegistry::default();
    let (lease, _) = registry.check_out(11, Some("document-census")).expect("lease");
    registry.check_in(lease, Puzzle3dSessionState { meshes: Some((1, "x".repeat(PUZZLE3D_SESSION_PROCESS_BYTES))), ..Default::default() });
    assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "a census exactly at the ceiling is still admissible");
    let (lease, held) = registry.check_out(12, Some("document-second")).expect("second lease");
    assert!(held.instances.is_none() && held.meshes.is_none(), "a different instance owns a different slot and starts cold");
    registry.check_in(lease, Puzzle3dSessionState { meshes: Some((2, "y".into())), ..Default::default() });
    assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "one byte past the ceiling is refused rather than admitted");
    let (_, refused) = registry.check_out(12, Some("document-second")).expect("third lease");
    assert!(refused.instances.is_none() && refused.meshes.is_none(), "the refused state is simply absent on the next call, so that instance rebuilds cold");
}

/// 📐️ Wave W-P: the session row is a fixed 64-slot array, so one slot's inline size is multiplied by 64
/// every time the registry is constructed. Anything multi-kilobyte by value (a `BuiltNode`, a scene_snapshot)
/// belongs behind a pointer, not inline — a fat slot is how a fixed row turns into a stack overflow.
#[test]
fn one_session_slot_stays_small_enough_for_a_fixed_row() {
    let slot = size_of::<Puzzle3dSessionSlot>();
    let state = size_of::<Puzzle3dSessionState>();
    let collision = size_of::<Puzzle3dCollisionSession>();
    let app = size_of::<Puzzle3dPlayApp>();
    println!("[wave-P sizes] slot={slot} state={state} collision={collision} app={app} row={}", slot * PUZZLE3D_SESSION_SLOTS);
    assert!(slot <= 64, "one session slot grew to {slot} bytes; keep the cached state behind a pointer");
    assert!(slot * PUZZLE3D_SESSION_SLOTS <= 8 * 1024, "the whole session row grew to {} bytes and is built by value", slot * PUZZLE3D_SESSION_SLOTS);
    assert!(state <= 2048, "one session state grew to {state} bytes");
    assert!(collision <= 2048, "the carried collision session grew to {collision} bytes");
    assert!(app <= 32 * 1024, "Puzzle3dPlayApp grew to {app} bytes; it is built on the stack on every dispatch and every render, and async dispatch futures hold several copies inline");
}

//#region ⏱️InteractiveStepBudget
/// 🚨️ Mirror of `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 000 µs) — that crate is not a
/// direct dependency of this artifact. `semio_framework_job` faults any interactive step at or over it
/// with `interactive_step_contract_violated`, and `MountedTypedCommandFullOperation`'s worker pump
/// cancels the lease before the fault body is surfaced, so the user reads a real budget overrun as a
/// bare "typed-operation cancelled". Every retained step of every puzzle3d command must stay under it.
const PUZZLE3D_INTERACTIVE_STEP_CEILING: std::time::Duration = std::time::Duration::from_micros(8_000);

/// 🎯️ What this artifact holds itself to in the UNOPTIMIZED test profile — a quarter of the framework
/// ceiling, so an optimized guest keeps an order of magnitude of headroom over the same document.
const PUZZLE3D_MEASURED_STEP_BUDGET: std::time::Duration = std::time::Duration::from_micros(2_000);

/// 🔁️ Cold runs each measured law drives, keeping each TURN's best across them — the standard robust
/// estimator for "how much work does one turn do", which is the only thing this artifact controls. This
/// repository is worked on by several sessions at once and this box runs their cargo builds alongside
/// the suite: at the millisecond scale a single wall-clock sample measures the scheduler, not the
/// artifact. Measured on the SAME binary, same turn: the retired fill tick gave 1.55 / 3.68 / 2.69 / 3.72 /
/// 2.19 ms across five consecutive runs at load average 64, and 12.67 ms at load average 79, against a
/// best of 1.25 ms (ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3). Both bounds below are therefore read
/// off those per-turn minima — including the framework ceiling, whose per-turn contract the framework
/// itself enforces at runtime; what a test can prove is that the WORK inside a turn fits it.
const PUZZLE3D_MEASURED_STEP_RUNS: u32 = 5;

fn measured_step_loop(original: &mut semio_s_plugin_puzzle_retained::testing::OriginalFixture, work: &mut dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle3dPlayApp>>, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dConfig, context: &semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle3dPlayApp>>, guard: usize, label: &str) -> Vec<std::time::Duration> {

    use crate::retained_command::testing::PuzzleCommandWorkStep;
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let mut turns = Vec::with_capacity(guard);
    loop {
        assert!(turns.len() <= guard, "{label} step() did not reach Complete within {guard} bounded turns");
        let started = std::time::Instant::now();
        let outcome = crate::retained_command::testing::step(original, work, command, snapshot, config, &interaction, &hover, Some(context)).expect("bounded step");
        turns.push(started.elapsed());
        if matches!(outcome, PuzzleCommandWorkStep::Complete(_)) {
            break;
        }
    }
    semio_s_plugin_puzzle_retained::testing::close(original, work);
    turns
}

fn measured_tool_work(original: &mut semio_s_plugin_puzzle_retained::testing::OriginalFixture, tool_id: &'static str, run: u32) -> (Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle3dPlayApp>>>, semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle3dPlayApp>>) {
    let instance = Puzzle3dInstanceBinding { app_instance_id: 9_000 + run, parent_document_id: format!("measured-{tool_id}-{run}"), owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(Box::new(Puzzle3dInstanceOperationOwner::default())) };
    let app_instance_id = instance.app_instance_id;
    let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle3dPlayApp>>> = match tool_id {
        "acceptSuggestion" => original.construct_boxed(Puzzle3dAcceptSuggestionWork::original_birth_demands().unwrap(), |scope| Puzzle3dAcceptSuggestionWork::from_original_construction(scope).map(|(work, progress)| (work.bound(instance), progress))),
        "setActiveExample" => original.construct_boxed(Puzzle3dSetActiveExampleWork::original_birth_demands().unwrap(), |scope| Puzzle3dSetActiveExampleWork::from_original_construction(scope)),
        "openVortexSuggestions" => original.construct_boxed(Default::default(), |_scope| Ok((Puzzle3dWindowCommandWork::new(tool_id).bound(instance), Default::default()))),
        _ => original.construct_boxed(Default::default(), |_scope| Ok((crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent), Default::default()))),
    };
    let context = semio_framework_plugin::app::ArtifactOwnedToolJobContext::new(
        app_instance_id,
        Some(measured_view_state()),
        [0; 32],
        0,
        0,
        semio_framework_plugin::app::ArtifactOwnedToolJobSnapshots { children: Default::default(), draft: Default::default(), transient: Default::default(), window_config: None, window_transient: None },
    );
    (work, context)
}

/// ⏱️ Drives one tool id's REAL work to `Complete` `PUZZLE3D_MEASURED_STEP_RUNS` times, each from its
/// own cold session slot, and answers `(turns, the worst PER-TURN BEST)`: every run of the same cold
/// document takes the same bounded turns in the same order (asserted), so turn `t`'s cost is the
/// minimum of that turn across the runs, and the law is on the worst of those minima. A scheduler
/// preemption lands on one turn of one run and is cancelled by the others — see
/// `PUZZLE3D_MEASURED_STEP_RUNS` for why that matters on this box.
fn measured_cold_runs(tool_id: &'static str, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dConfig) -> (usize, std::time::Duration) {
    let mut best: Vec<std::time::Duration> = Vec::new();
    for run in 0..PUZZLE3D_MEASURED_STEP_RUNS {
            let mut original = semio_s_plugin_puzzle_retained::testing::OriginalFixture::new(
        semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_s_plugin_puzzle_retained::testing::fixture_stock(), || Some(0), u64::MAX, 1 << 24);
        let (mut work, context) = measured_tool_work(&mut original, tool_id, run);
        let turns = measured_step_loop(&mut original, work.as_mut(), command, snapshot, config, &context, crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, tool_id);
        original.finish();
        if best.is_empty() {
            best = turns;
            continue;
        }
        assert_eq!(turns.len(), best.len(), "{tool_id} run {run} took a different number of bounded turns than its cold siblings");
        for (slot, measured) in best.iter_mut().zip(turns) {
            *slot = (*slot).min(measured);
        }
    }
    let (index, worst) = best.iter().enumerate().max_by_key(|(_, turn)| **turn).map_or((0, std::time::Duration::ZERO), |(index, turn)| (index + 1, *turn));
    (best.len(), worst)
}

fn measured_view_state() -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel {
        window_id: Some(main::WINDOW_KIND_ID.to_string()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: main::WINDOW_KIND_ID.to_string(), window_kind_id: main::WINDOW_KIND_ID.to_string() }],
        ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

fn measured_nakagin_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"))
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END: `openVortexSuggestions` syncs the whole precompute session
/// and refreshes one vortex's brush candidates. Measured on Nakagin at opt-level 0: 82.5 ms (W-P2's
/// baseline), 17.6 ms once W-P2 removed the doubled session sync and the doubled projection decode, and
/// under this artifact's own 2 000 µs budget once W-P3 split `handle_action_impl`'s prologue into the
/// scene / session-sync / dispatch turns [`Puzzle3dActionPrologue`] declares and made each of them typed.
/// This drives the REAL `Puzzle3dWindowCommandWork` `build_tool_job` routes this tool id to.
#[test]
fn open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("openVortexSuggestions", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(main::WINDOW_KIND_ID.to_string())).expect("openVortexSuggestions command decodes");
    let (steps, worst) = measured_cold_runs("openVortexSuggestions", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "openVortexSuggestions worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "openVortexSuggestions worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END W-P2: `acceptSuggestion` walks the document for its target
/// vortex and publishes the placement. Measured at 11 796 µs in ONE step before this wave.
#[test]
fn accept_suggestion_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(main::WINDOW_KIND_ID.to_string())).expect("acceptSuggestion command decodes");
    let (steps, worst) = measured_cold_runs("acceptSuggestion", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "acceptSuggestion worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "acceptSuggestion worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

fn measured_concrete_forest_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"))
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3: swapping the Concrete Forest document for the
/// 180-object Nakagin one is the single largest document gesture this artifact has — it deletes every
/// existing attraction and object and creates every Nakagin one. Its work is fully typed and cursorized
/// (`Puzzle3dSetActiveExampleWork`), so no turn of it may cross the interactive step ceiling either.
#[test]
fn set_active_example_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_concrete_forest_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), Some(main::WINDOW_KIND_ID.to_string())).expect("setActiveExample command decodes");
    let (steps, worst) = measured_cold_runs("setActiveExample", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "setActiveExample worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "setActiveExample worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// 🌉️ Differential law for W-P3's typed `scene_from_snapshot`: the derived `ToValue`/`FromValue`
/// machinery is an independent implementation of the same structural-twin translation, so the typed
/// scene_snapshot must equal what the persisted-projection bridge produced for every shipped document. A
/// disagreement here is a real behaviour change, not a performance one — the semantic document delta
/// every editing action publishes is taken against exactly this scene_snapshot.
///
/// 🪪️ Each snapshot is canonicalized first (rebuilt from its OWN typed authority) because that is the
/// only projection production ever hands the app: a store-driven `Puzzle3dPlaySnapshot` carries the
/// typed document and materializes `value()` from it, so `value()` is by construction
/// `ToValue(typed())`. Feeding a raw editor-side scene_snapshot straight into `new()` can hand the two halves
/// different content — `empty_scene_snapshot()`'s `meta` serializes both members as `Null`, which the typed
/// decode refuses, and `new()` silently falls back to `Puzzle3dSnapshot::default()`; see this wave's
/// report §6.
#[test]
fn puzzle3d_next_object_label_increments_from_the_authored_seed_name() {
    let scene_snapshot = CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone();
    let kind = "Hexagonal Cut Concrete Forest Left";
    let next = puzzle3d_next_object_label(&scene_snapshot.objects, &scene_snapshot, kind);
    assert_eq!(next, "Hexagonal Cut Concrete Forest Left 2");
    let mut peers = scene_snapshot.objects.clone();
    peers.push(Puzzle3dObject {
        id: "fill-1".into(),
        label: Some("Hexagonal Cut Concrete Forest Left 2".into()),
        object_kind: Some(kind.into()),
        origin: [0.0; 3],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
    });
    assert_eq!(puzzle3d_next_object_label(&peers, &scene_snapshot, kind), "Hexagonal Cut Concrete Forest Left 3");
}

/// 🗝️ The two untyped `meta` members are compared as the TYPED catalogs they stand for
/// (`Puzzle3dKindCatalogs` / `Vec<Puzzle3dKindCompatibility>` — lossless, and what every reader of
/// those members ultimately decodes them into), because raw `DslValue` equality would compare key
/// ORDER: the bridge inherits the persisted projection's own `serde_json` map order and the typed
/// construction emits declaration order, for byte-identical content. Every typed member of the scene_snapshot
/// is compared directly, unnormalized.
#[test]
fn puzzle3d_typed_fixture_matches_the_projection_bridge_for_every_example() {
    for (label, scene_snapshot) in [("empty", empty_scene_snapshot()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()), ("nakagin", NAKAGIN_EXAMPLE_SNAPSHOT.clone())] {
        let seed = Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&scene_snapshot).expect("typed fixture admits"));
        let snapshot = Puzzle3dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(seed.typed())).expect("typed fixture admits"));
        let bridged = Puzzle3dScene { scene_snapshot: semio_framework_value::FromValue::from_value(snapshot.value().clone()).expect("native fixture view admits"), runtime: Puzzle3dRuntime::default(), active_utility: "utility".into() };
        let typed = scene_from_snapshot(snapshot.typed(), Puzzle3dRuntime::default(), "utility");
        assert_eq!(typed.scene_snapshot.schema, bridged.scene_snapshot.schema, "{label}: schema disagrees");
        assert_eq!(typed.scene_snapshot.domain, bridged.scene_snapshot.domain, "{label}: domain disagrees");
        assert_eq!(typed.scene_snapshot.objects, bridged.scene_snapshot.objects, "{label}: objects disagree");
        assert_eq!(typed.scene_snapshot.attractions, bridged.scene_snapshot.attractions, "{label}: attractions disagree");
        assert_eq!(typed.scene_snapshot.target_volumes, bridged.scene_snapshot.target_volumes, "{label}: target volumes disagree");
        assert_eq!(typed.scene_snapshot.references, bridged.scene_snapshot.references, "{label}: references disagree");
        let catalogs = |meta: &Puzzle3dSceneMetadata| -> crate::Puzzle3dKindCatalogs { meta.kind_catalogs.clone().map_or_else(crate::Puzzle3dKindCatalogs::default, |rows| semio_framework_value::FromValue::from_value(rows).expect("kind catalogs decode")) };
        let compatibility = |meta: &Puzzle3dSceneMetadata| -> Vec<crate::Puzzle3dKindCompatibility> { meta.kind_compatibility.clone().map_or_else(Vec::new, |rows| semio_framework_value::FromValue::from_value(rows).expect("kind compatibility decodes")) };
        assert_eq!(catalogs(&typed.scene_snapshot.meta), catalogs(&bridged.scene_snapshot.meta), "{label}: kind catalogs disagree");
        assert_eq!(compatibility(&typed.scene_snapshot.meta), compatibility(&bridged.scene_snapshot.meta), "{label}: kind compatibility disagrees");
        assert_eq!(typed.active_utility, bridged.active_utility, "{label}: active utility disagrees");
    }
}

/// 🧲️ Differential law for the world attraction segments: resolving both ends through the one per-call vortex
/// index lands every segment exactly where `resolve_vortex_world_position` (the scan it replaced) puts it, and
/// drops exactly the attractions that scan cannot resolve — for every example, and for a fill-run-sized
/// document whose attractions multiply with its placements.
#[test]
fn world_attraction_segments_match_the_vortex_position_resolver_for_every_example() {
    let mut grown = NAKAGIN_EXAMPLE_SNAPSHOT.clone();
    let ends: Vec<String> = grown.objects.iter().flat_map(|object| object.vortices.iter().map(|vortex| crate::editor::puzzle3d::puzzle3d_vortex_full_id(&object.id, &vortex.id))).collect();
    assert!(ends.len() > 2, "nakagin declares vortices to attract");
    grown.attractions = ends.windows(2).enumerate().map(|(index, pair)| Puzzle3dAttraction { id: format!("a{index}"), attracting: pair[0].clone(), attracted: pair[1].clone(), ..Default::default() }).collect();
    grown.attractions.push(Puzzle3dAttraction { id: "dangling".into(), attracting: ends[0].clone(), attracted: "no-such-object:no-such-vortex".into(), ..Default::default() });
    for (label, scene_snapshot) in [("empty", empty_scene_snapshot()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()), ("nakagin", NAKAGIN_EXAMPLE_SNAPSHOT.clone()), ("nakagin-dangling", grown)] {
        let expected: Vec<Value> = scene_snapshot
            .attractions
            .iter()
            .filter_map(|attraction| {
                let from = crate::editor::puzzle3d::resolve_vortex_world_position(&scene_snapshot, &attraction.attracting)?;
                let to = crate::editor::puzzle3d::resolve_vortex_world_position(&scene_snapshot, &attraction.attracted)?;
                Some(json!({ "id": attraction.id, "from": from, "to": to, "color": "#60a5fa" }))
            })
            .collect();
        let rendered = parse(&main::world_attractions_json(&scene_snapshot), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("attractions json");
        let rendered = rendered.as_array().cloned().unwrap_or_default();
        assert_eq!(rendered.len(), expected.len(), "{label}: exactly the resolvable attractions render");
        for (segment, resolved) in rendered.iter().zip(&expected) {
            for field in ["id", "from", "to", "color"] {
                assert_eq!(segment[field], resolved[field], "{label}: indexed attraction segment {} diverges from the resolver at {field}", resolved["id"]);
            }
        }
    }
}

/// 🌉️ Differential law for W-P3's typed `scene_config`: the same engine scene the all-`DslValue` bridge
/// (`scene_config_value` + the derived `FromValue`) produced, field for field, on every shipped document
/// and with real kind weights on the runtime. `SceneConfig: PartialEq` is the engine's OWN resync
/// verdict, so equality here is exactly the property the precompute session reads.
#[test]
fn puzzle3d_typed_scene_config_matches_the_value_bridge_for_every_example() {
    for (label, scene_snapshot) in [("empty", empty_scene_snapshot()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()), ("nakagin", NAKAGIN_EXAMPLE_SNAPSHOT.clone())] {
        let mut runtime = Puzzle3dRuntime::default();
        runtime.contact_tolerance = 0.375;
        runtime.object_kind_weights.insert("capsule".into(), 0.25);
        runtime.vortex_kind_weights.insert("rim".into(), 0.75);
        let envelope = Puzzle3dScene { scene_snapshot, runtime, active_utility: "utility".into() };
        let bridged: semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::SceneConfig = semio_framework_value::FromValue::from_value(scene_config_value(&envelope)).expect("value bridge decodes");
        let typed = scene_config(&envelope).expect("typed scene config builds");
        assert_eq!(typed, bridged, "{label}: typed engine scene disagrees with the value bridge");
    }
}

/// 🥽️ W-P3: the mesh-url index answers exactly what the per-object catalog scan answered, for every
/// object of every shipped document, and `collect_mesh_urls` still returns the same SET of identities.
#[test]
fn puzzle3d_kind_mesh_index_matches_a_per_object_catalog_scan() {
    for (label, scene_snapshot) in [("empty", empty_scene_snapshot()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()), ("nakagin", NAKAGIN_EXAMPLE_SNAPSHOT.clone())] {
        let index = Puzzle3dKindMeshIndex::of(&scene_snapshot.meta);
        for object in &scene_snapshot.objects {
            let scanned = scene_snapshot
                .meta
                .kind_catalogs
                .as_ref()
                .and_then(|catalogs| catalogs.get("objects"))
                .and_then(|value| value.as_array())
                .and_then(|rows| rows.iter().find(|row| row.get("id").and_then(semio_framework_value::DslValue::as_str) == object.object_kind.as_deref()))
                .and_then(|row| row.get("meshUrl"))
                .and_then(semio_framework_value::DslValue::as_str);
            let expected = object.mesh_url.as_deref().filter(|url| !url.is_empty()).or(scanned);
            assert_eq!(index.resolve(object), expected, "{label}: {} resolved to a different mesh identity", object.id);
        }
        let indexed: std::collections::BTreeSet<String> = collect_mesh_urls(&scene_snapshot).into_iter().collect();
        assert!(indexed.iter().all(|url| !url.is_empty()), "{label}: an empty mesh identity was collected");
    }
}

//#endregion ⏱️InteractiveStepBudget

//#region 🩹️CheckedDefects
/// 🎯️ Every action id one built context menu dispatches, groups and submenus included.
fn context_menu_action_ids(items: &[semio_framework_plugin::ContextMenuItemSpec], into: &mut Vec<(String, String)>) {
    for item in items {
        if let Some(action) = item.action.as_deref() {
            into.push((item.id.clone(), action.to_string()));
        }
        if let Some(children) = item.children.as_ref() {
            context_menu_action_ids(children, into);
        }
    }
}

/// 🎯️ Every action id `create_puzzle3d_app()` declares — the set `dispatch_action` can actually route.
fn declared_action_ids() -> std::collections::BTreeSet<String> {
    dispatchable_actions(&create_puzzle3d_app()).into_iter().map(|action| action.id.clone()).collect()
}

/// 🖱️ Context-menu rows bypass the registry-validated `Menu::action` builder (they carry
/// `Puzzle3dLabels` text the English-only `ActionDefinition` cannot resolve), so nothing catches a
/// typo'd action id at build time. `📓️2026-09-09-user-feature-checklist.md` §15/§22 and summary #7:
/// "Zoom to Selection" dispatched `zoomToSelection`, which this crate never declares — the registered
/// verb is `focusSelection`. This law walks EVERY selection kind's menu and holds every emitted id
/// inside the declared set, so the whole class cannot come back.
#[semio_framework_async_macros::async_test]
async fn every_context_menu_row_dispatches_a_declared_action() {
    let declared = declared_action_ids();
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let object_id = first_object_id(&app);
    let vortex_id = first_vortex_full_id(&app);
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let mut zoom_rows = 0usize;
    for (granularity, id) in [
        (PUZZLE3D_GRANULARITY_OBJECT, object_id.as_str()),
        (PUZZLE3D_GRANULARITY_VORTEX, vortex_id.as_str()),
        (PUZZLE3D_GRANULARITY_TARGET_VOLUME, volume_id.as_str()),
        (PUZZLE3D_GRANULARITY_REFERENCE, "reference-1"),
        (PUZZLE3D_GRANULARITY_ATTRACTION, "attraction-1"),
    ] {
        // 🧹️ `Puzzle3dContextSelection::fill_from_interaction` fills every granularity the CLIENT
        // surface did not send from the framework-owned selection, so the object `addObjectKind`
        // left selected would win every branch and each granularity would build the OBJECT menu.
        // A real client's document selection is the one it right-clicks in; clear it so the
        // surface group under test is the only subject and each branch is genuinely exercised.
        dispatch(&mut app, "clearSelection", None, None).await.expect("clearSelection");
        let menu = context_menu_for_selection(&mut app, granularity, id).await;
        let mut rows = Vec::new();
        context_menu_action_ids(&menu, &mut rows);
        assert!(!rows.is_empty(), "{granularity} selection built a menu with no actionable row");
        for (row, action) in &rows {
            assert!(declared.contains(action), "{granularity} row {row} dispatches undeclared action {action}; declared: {declared:?}");
            if row == "zoom" {
                zoom_rows += 1;
                assert_eq!(action, "focusSelection", "the zoom row must dispatch the registered camera verb");
            }
        }
    }
    assert_eq!(zoom_rows, 3, "object, vortex and reference selections each carry a Zoom to Selection row");
    // 🎯️ Reachability, not just declaration: the id the row carries must be one the typed command
    // channel actually admits (an unknown id panics inside `dispatch`).
    dispatch(&mut app, "focusSelection", None, None).await.expect("the context menu's zoom row must dispatch");
}

/// 🗂️ `📓️2026-09-09-user-feature-checklist.md` §23/summary #9: both the "Add Object" dialog and the
/// standalone `addObjectKind` arg form hardcoded ONE static option (`"Object"`), a kind neither
/// example's catalog declares — so the dialog could not add a single real object kind. Both forms must
/// now offer exactly the declared examples' own catalog rows, bounded by
/// [`PUZZLE3D_OBJECT_KIND_OPTIONS_MAX`], with a default that IS one of them.
#[semio_framework_async_macros::async_test]
async fn the_add_object_dialog_offers_every_object_kind_of_both_examples() {
    use semio_framework_plugin::ArgSchema;
    let definition = create_puzzle3d_app();
    let expected: Vec<String> = [&*CONCRETE_FOREST_EXAMPLE_SNAPSHOT, &*NAKAGIN_EXAMPLE_SNAPSHOT]
        .into_iter()
        .flat_map(|scene_snapshot| puzzle3d_kind_ids(scene_snapshot, "objects"))
        .fold(Vec::new(), |mut ids, id| {
            if !ids.contains(&id) {
                ids.push(id);
            }
            ids
        });
    assert!(expected.len() >= 2, "both shipped examples must declare object kinds, got {expected:?}");
    let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
    let standalone = dispatchable_actions(&definition).into_iter().find(|action| action.id == "addObjectKind").expect("addObjectKind declared").args.clone();
    for (surface, args) in [("dialog", dialog.args.clone()), ("action", standalone)] {
        let arg = args.iter().find(|arg| arg.id == "objectKind").unwrap_or_else(|| panic!("{surface} declares an objectKind arg"));
        let ArgSchema::String { options, .. } = &arg.schema else { panic!("{surface}'s objectKind must stay a select") };
        let values: Vec<String> = options.iter().map(|option| option.value.clone()).collect();
        assert_eq!(values, expected, "{surface} must offer the live catalog's object kinds");
        assert!(values.len() <= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX, "{surface} exceeded its fixed option ceiling");
        let default = arg.default.as_ref().and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
        assert!(values.contains(&default), "{surface}'s default {default} is not one of its own options");
    }
    assert!(dialog.args.iter().all(|arg| arg.required), "the dialog's kind select stays required");
}

/// 🗨️ Wave B11 (checklist §23, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 7): the shell
/// fallback menu / command palette published TWO rows carrying the identical "Add Object…" label —
/// `addObjectKind` (arg-carrying, so the shell redirects it to the bare action pane) and
/// `openAddObjectDialog` (the declared dialog). The arg-carrying one came FIRST, so every user and
/// every probe clicking "Add Object…" got the action pane and the dialog was unreachable. Exactly one
/// add-object row may be palette/menu vocabulary, and it must be the dialog opener; it must also be
/// declared ahead of `addObjectKind`, because the shell menu keeps its first leaves at top level and
/// folds the rest into "More ›".
#[semio_framework_async_macros::async_test]
async fn exactly_one_add_object_row_is_menu_vocabulary_and_it_opens_the_dialog() {
    let definition = create_puzzle3d_app();
    let actions: Vec<&ActionDefinition> = dispatchable_actions(&definition);
    let find = |id: &str| *actions.iter().find(|action| action.id == id).unwrap_or_else(|| panic!("{id} declared"));
    let opener = find("openAddObjectDialog");
    let parametrized = find("addObjectKind");
    assert_eq!(opener.kind, ActionKind::Shell, "the dialog opener emits an effect, never a document edit");
    assert!(opener.in_palette, "the dialog opener IS the user-facing add-object row");
    assert!(!parametrized.in_palette, "the dialog's parametrized verb must not publish a second identical row");
    fn label(action: &ActionDefinition) -> &str {
        action.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En)
    }
    assert!(label(opener).starts_with(label(parametrized)), "both rows describe the same verb, which is exactly why only one may be offered: {:?} vs {:?}", label(opener), label(parametrized));
    let visible: Vec<&str> = actions.iter().filter(|action| action.in_palette && label(action).starts_with("Add Object")).map(|action| action.id.as_str()).collect();
    assert_eq!(visible, vec!["openAddObjectDialog"], "exactly one add-object row may reach a menu");
    let order = |id: &str| actions.iter().position(|action| action.id == id).unwrap_or_else(|| panic!("{id} declared"));
    assert!(order("openAddObjectDialog") < order("addObjectKind"), "the dialog opener must be declared ahead of the verb it drives so the shell menu keeps it at top level");
    // 🎯️ Reachability, not just declaration: the row's id must be one the typed command channel admits.
    let mut app = app().await;
    let result = dispatch(&mut app, "openAddObjectDialog", None, None).await.expect("the add-object row must dispatch");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::OpenDialog { dialog_id, .. } if dialog_id == "addObject")),
        "clicking the add-object row must open the declared dialog: {:?}",
        result.requested_effects,
    );
}

/// 🗣️ `📓️2026-09-09-user-feature-checklist.md` §14/summary #10: the engagement input advertised
/// `clear`/`rectangle`/`lasso` while the parser silently dropped all three. The placeholder is now
/// DERIVED from the verb list, and every verb in it must do something observable: the three marquee
/// verbs move the viewport's `selection.method`, and `clear` empties the framework-owned domain.
#[semio_framework_async_macros::async_test]
async fn every_advertised_engagement_verb_is_implemented() {
    use crate::editor::puzzle3d::commands::engagement_submit::PUZZLE3D_ENGAGEMENT_VERBS;
    let mut app = app().await;
    let envelope = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let placeholder = main::engagement(&envelope, &Puzzle3dLabels::NATIVE_EN, None).input.and_then(|input| input.placeholder).unwrap_or_default();
    for verb in PUZZLE3D_ENGAGEMENT_VERBS {
        assert!(placeholder.contains(verb), "the engagement placeholder must advertise {verb}: {placeholder}");
    }
    let method_of = |node: &Value| selection_of(node).get("method").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(method_of(&render_composite(&mut app).await), PUZZLE3D_SELECTION_METHOD_PICK, "a fresh window sweeps with the pick default");
    for method in [PUZZLE3D_SELECTION_METHOD_RECTANGLE, PUZZLE3D_SELECTION_METHOD_LASSO, PUZZLE3D_SELECTION_METHOD_PICK] {
        dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": method })), None).await.unwrap_or_else(|error| panic!("engagementSubmit {method}: {error:?}"));
        let rendered = render_composite(&mut app).await;
        assert_eq!(method_of(&rendered), method, "typing {method} must move the viewport marquee method");
    }
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let selected = |node: &Value| selection_of(node).get("ids").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
    assert_eq!(selected(&render_composite(&mut app).await), 1, "the object must be selected before clear");
    dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "clear" })), None).await.expect("engagementSubmit clear");
    assert_eq!(selected(&render_composite(&mut app).await), 0, "typing clear must empty the framework-owned selection");
}

/// 🪣️ Wave B13, `📓️2026-09-11-wave-B1-battery-extension.md` §14 `engagement-fill-verb`: typing `fill <n>`
/// has to ARM the Fill tool, not merely park a utility name in the scene. Fill is a mode-level TOOL, so the
/// only thing that can press its `#tool.fill` tab is an `Effect::SetActiveTool` leaving the turn; `brush` is
/// a window UTILITY and must take the `Effect::SetActiveUtility` lane instead. This law pins both lanes plus
/// the count hand-off, because the two are one `match` apart in `engagement_submit` and the browser
/// symptom of getting it wrong (`#tool.fill aria-pressed=null`) is indistinguishable from a dead input.
#[semio_framework_async_macros::async_test]
async fn the_engagement_fill_verb_arms_the_fill_tool_and_hands_over_its_count() {
    let mut app = app().await;
    let result = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "fill 5" })), None).await.expect("engagementSubmit fill 5");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id == fill_tool::TOOL_ID)),
        "typing `fill 5` must arm the fill tool: {:?}",
        result.requested_effects,
    );
    assert!(
        !result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { .. })),
        "fill is a tool, so it must not also claim a window utility: {:?}",
        result.requested_effects,
    );
    let count = result
        .requested_effects
        .iter()
        .find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "setFillCount" => args.as_ref().and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_f64),
            _ => None,
        })
        .expect("typing `fill 5` must hand the count to the retained setFillCount command");
    assert_eq!(count, 5.0, "the typed count reaches setFillCount verbatim");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, args: Some(args), .. } if action == semio_framework_tool_run::TOOL_RUN_START_ACTION_ID && args.get(semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID).and_then(semio_framework_value::DslValue::as_str) == Some(fill_tool::TOOL_ID))),
        "typing `fill 5` starts a fill tool run through the framework: {:?}",
        result.requested_effects,
    );
    let brush = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), None).await.expect("engagementSubmit brush");
    assert!(
        brush.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { utility_id, .. } if utility_id == "brush")),
        "typing `brush` must claim the brush window utility: {:?}",
        brush.requested_effects,
    );
    assert!(
        !brush.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { .. })),
        "leaving fill is the host's own `setActiveTool \"\"`, never a guest tool effect: {:?}",
        brush.requested_effects,
    );
}

/// 🧰️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B35, checklist §14 `engagement-brush-verb`: `fill`
/// arms and `brush` does not, because `fill` is a MODE-level tool whose effect carries no window and
/// `brush` is a window utility whose effect carries one. `active_utility_by_window_id` is keyed by
/// window INSTANCE (wave B9), and a pane reads only its own key — so an effect addressed at the bare
/// window KIND (`puzzle3d-main`) arms a key nothing ever reads and the world lane keeps publishing
/// `activeUtility=select` forever, which is exactly the browser verdict. The sibling law above pins
/// WHICH effect the verb emits; this one pins WHERE it lands, on the multi-pane roster the shell
/// actually runs (the engagement line is window chrome, and an app-level panel resolves through
/// `focused_window_id`, wave B15 — both must resolve to an instance, never to the kind).
#[semio_framework_async_macros::async_test]
async fn the_engagement_brush_verb_arms_the_utility_of_a_pane_instance_never_the_window_kind() {
    let mut app = app().await;
    let perspective = "puzzle3d-main-perspective";
    let top = "puzzle3d-main-top";
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": true })), Some(top)).await.expect("grow the roster to two panes");
    let armed = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), Some(perspective)).await.expect("engagementSubmit brush at the pane");
    let addressed: Vec<&String> = armed
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == utilities::brush::UTILITY_ID => Some(window_id),
            _ => None,
        })
        .collect();
    assert_eq!(addressed, vec![&perspective.to_string()], "the pane-addressed brush verb must arm that pane's own key: {:?}", armed.requested_effects);
    let unaddressed = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "brush" })), None).await.expect("engagementSubmit brush with no window of its own");
    let fallback: Vec<&String> = unaddressed
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == utilities::brush::UTILITY_ID => Some(window_id),
            _ => None,
        })
        .collect();
    assert!(
        fallback.iter().all(|window_id| !crate::editor::puzzle3d::puzzle3d_window_id_is_kind(window_id)),
        "a verb that carries no window of its own must still resolve a pane INSTANCE, never the bare kind: {fallback:?}",
    );
}

/// 🛑 Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31, checklist §12/§14 `Abort`: Escape while the Fill
/// TOOL is armed has to LEAVE it — abort the run it shows and disarm the tool — instead of
/// returning untouched, which is what `engagement_abort` did while `puzzle3d_fill_tool_active` and why
/// the browser read `engagement-abort ... activeUtility=fill` after every `fill <n>` (wave B29 §2.5).
/// The disarm is the app's own explicit effect, and the abort leaves NO tool and NO utility armed —
/// the same place the shell's own Escape leaves them (`ShellHost` dispatches `setActiveTool ""`), not
/// a remembered previous tool this app keeps no history of. The `brush`-claims-a-utility law above
/// still holds: a plain verb switch never emits a tool effect, only this abort does.
#[semio_framework_async_macros::async_test]
async fn escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool() {
    let mut armed = app().await;
    dispatch(&mut armed, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("arm fill");
    let aborted = dispatch(&mut armed, "engagementAbort", None, None).await.expect("engagementAbort");
    assert!(
        aborted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())),
        "Escape with fill armed must disarm the tool: {:?}",
        aborted.requested_effects,
    );
    assert!(
        !aborted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id == fill_tool::TOOL_ID)),
        "and must never re-arm the tool it is leaving: {:?}",
        aborted.requested_effects,
    );
    let active_utility = |node: &Value| selection_of(node).get("activeUtility").and_then(Value::as_str).unwrap_or_default().to_string();
    let rendered = render_window(&mut armed, main::WINDOW_KIND_ID).await;
    assert_eq!(active_utility(&rendered), PUZZLE3D_DEFAULT_UTILITY, "the abort leaves no utility armed either: {}", selection_of(&rendered));

    // 🏛️ `active_tool_id` is host-owned (`ViewModel`, `🪟️window/🦀️.rs` `host_activation`), so the guest
    // keeps reading `fill` until the host answers the disarm. A second Escape in that window therefore
    // repeats the same idempotent disarm — and still aborts nothing, because no run was ever shown.
    let repeated = dispatch(&mut armed, "engagementAbort", None, None).await.expect("second engagementAbort");
    assert!(
        repeated.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())),
        "while the host still reports fill armed the disarm repeats rather than giving up: {:?}",
        repeated.requested_effects,
    );
    assert!(
        !repeated.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID)),
        "and aborts no run, because none is shown: {:?}",
        repeated.requested_effects,
    );

    let mut unarmed = app().await;
    let untouched = dispatch(&mut unarmed, "engagementAbort", None, None).await.expect("engagementAbort with nothing armed");
    assert!(
        !untouched.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { .. }) || matches!(effect, Effect::DispatchAction { action, .. } if action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID)),
        "an abort with no tool armed owes no tool or abort effect: {:?}",
        untouched.requested_effects,
    );
}

/// 🧯️ `📓️2026-09-09-user-feature-checklist.md` §9/§13/summary #13: a placement that produces nothing
/// used to produce NOTHING — no fault, no notice, no visible change. Every refusal path of the two
/// placement verbs must now carry exactly one `Effect::Notify`, and a SUCCESSFUL placement must carry
/// none (a notice on the happy path would be noise, and would hide the real ones).
#[semio_framework_async_macros::async_test]
async fn a_refused_placement_surfaces_exactly_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    // 🎯️ Nothing selected, no popup open, no `fullId` — the accept has no target at all.
    let orphan = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion without a target still completes");
    assert_eq!(notices(&orphan).len(), 1, "an accept with no target must say so: {:?}", orphan.requested_effects);
    // 🎯️ A brush placement naming a vortex no object owns, on a kind the catalog does not declare.
    let unknown = dispatch(
        &mut app,
        "addBrushObject",
        Some(&json!({ "targetVortexFullId": "no-such-object:v0", "objectKindId": "no-such-kind", "sourceVortexIndex": 0, "origin": [0.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0] })),
        None,
    )
    .await
    .expect("addBrushObject on an unknown kind still completes");
    assert_eq!(notices(&unknown).len(), 1, "a brush placement onto an undeclared kind must say so: {:?}", unknown.requested_effects);
    for message in notices(&orphan).into_iter().chain(notices(&unknown)) {
        assert_ne!(message, PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored locale/terminology axis, so the notice must be real prose");
        assert!(!message.is_empty(), "a notice must carry text");
    }
    // 🎯️ The happy path stays quiet.
    let placed = dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    assert!(notices(&placed).is_empty(), "a successful action must not raise a notice: {:?}", placed.requested_effects);
}

/// 🙈️ `📓️2026-09-09-user-feature-checklist.md` §16 (re-checked for W-D4's §17 law): the inspection
/// panel's `patchInspector` flag rows must carry the INVERSE of the row's current state, so an object
/// hidden from here can be un-hidden from here. Driven end to end — hide through the panel's own
/// dispatch shape, then re-read the panel and show again.
#[semio_framework_async_macros::async_test]
async fn inspection_flag_rows_toggle_back_off() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let hidden_of = |app: &Puzzle3dApp, id: &str| {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(id)).cloned())
            .and_then(|object| object.get("hidden").and_then(Value::as_bool))
            .unwrap_or(false)
    };
    for expected in [true, false] {
        let panel = to_json_string(&render_body(&mut app, inspection::BODY_KEY).await);
        assert!(panel.contains("puzzle3d-play-inspector.object.hidden"), "the inspector must render the object's hidden row: {panel}");
        dispatch(&mut app, "patchInspector", Some(&json!({ "entity": PUZZLE3D_GRANULARITY_OBJECT, "field": "hidden", "ids": [object_id.as_str()], "value": expected })), None).await.expect("patchInspector hidden");
        assert_eq!(hidden_of(&app, &object_id), expected, "the inspector's hidden row must reach {expected}");
    }
}

/// 🙈️ `📓️2026-09-09-user-feature-checklist.md` §17: the outliner row's own Hide/Show control, driven
/// through the target the panel itself declares — never a hand-written literal, since the reported defect
/// ("Hide does nothing, Show can never un-hide") was exactly a wrong value baked into the row. Hide,
/// re-read the panel, and dispatch the row's `setSelectionHidden` with its NEW target args to show again;
/// the flag must round-trip both ways, exactly as the host binds a row verb to the row's one target.
#[semio_framework_async_macros::async_test]
async fn outliner_row_hide_and_show_round_trip_through_their_own_declared_args() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    let hidden_of = |app: &Puzzle3dApp, id: &str| {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(id)).cloned())
            .and_then(|object| object.get("hidden").and_then(Value::as_bool))
            .unwrap_or(false)
    };
    assert!(!hidden_of(&app, &object_id), "a freshly added object starts visible");
    for expected in [true, false] {
        let panel: Value = from_json_str(&to_json_string(&render_body(&mut app, artifact::BODY_KEY).await), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the outliner renders parseable ui json");
        let args = outliner_verb_row(&panel, &object_id, "setSelectionHidden").and_then(|row| row.pointer("/target/args").cloned()).unwrap_or_else(|| panic!("the outliner row must declare a setSelectionHidden target for {object_id}: {panel}"));
        assert_eq!(args.get("hidden").and_then(Value::as_bool), Some(expected), "the row's own target must ask for the INVERSE of the state it renders: {args}");
        dispatch(&mut app, "setSelectionHidden", Some(&args), None).await.expect("setSelectionHidden from the outliner row's own target");
        assert_eq!(hidden_of(&app, &object_id), expected, "the outliner row's hidden flag must reach {expected}");
    }
}

/// 🙈️ The hop AFTER the one above: hiding an object from the outliner row must also reach the WORLD
/// lane the viewport paints from, and the row itself must re-render with the inverse control. Wave B20
/// measured the browser publishing an unchanged row AND an unchanged `data-instances-json` while the
/// host's history had already recorded `change-object-hidden id=seed-left-001 new-hidden=true`, so
/// "the flag reached the projection" (the law above) is NOT the same claim as "the two surfaces the
/// user looks at moved". `world_instances_geometry_json` encodes hidden as a zero scale — the id stays
/// in the array so no other object's index shifts — and the row's own control flips Hide↔Show.
#[semio_framework_async_macros::async_test]
async fn outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control() {
    fn instance_scale(world: &Value, object_id: &str) -> Vec<f64> {
        let instances: Value = parse(world.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("the world body publishes an instances lane"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("instances lane is json");
        instances
            .as_array()
            .expect("instances lane is an array")
            .iter()
            .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id))
            .and_then(|instance| instance.get("scale").and_then(Value::as_array))
            .map(|scale| scale.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default()
    }
    /// 🙈️ The visibility row action's rendered face: the icon of the `setSelectionHidden` action on the row whose
    /// target names this object. Keyed on the target rather than on a row id so the law survives any reshaping
    /// of the tree above the control.
    fn visibility_control_icon(panel: &Value, object_id: &str) -> Option<String> {
        let row = outliner_verb_row(panel, object_id, "setSelectionHidden")?;
        let actions = row.get("rowActions").and_then(Value::as_array)?;
        let action = actions.iter().find(|action| action.get("verb").and_then(Value::as_str) == Some("setSelectionHidden"))?;
        action.get("icon").and_then(Value::as_str).filter(|icon| matches!(*icon, "eye" | "eye-off")).map(str::to_string)
    }
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let visible = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(instance_scale(&visible, &object_id), vec![1.0, 1.0, 1.0], "a visible object publishes its real scale");
    let before_icon = visibility_control_icon(&render_body(&mut app, artifact::BODY_KEY).await, &object_id);
    assert_eq!(before_icon.as_deref(), Some("eye"), "a visible object's row renders the Hide control");

    dispatch(&mut app, "setSelectionHidden", Some(&json!({ "entity": "object", "hidden": true, "ids": [object_id.clone()] })), None).await.expect("setSelectionHidden");

    let hidden = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(instance_scale(&hidden, &object_id), vec![0.0, 0.0, 0.0], "a hidden object must publish a zero scale into the world instance lane");
    let after_icon = visibility_control_icon(&render_body(&mut app, artifact::BODY_KEY).await, &object_id);
    assert_eq!(after_icon.as_deref(), Some("eye-off"), "a hidden object's row must re-render as the Show control");
}

/// 👁️ The RESTORE half of the law above, in the SAME settle: the row's Show control must put the object's
/// real scale back into the world instance lane, not merely flip the row's label back. The lane is served
/// from [`Puzzle3dInstanceResidency`], which re-serializes only the records whose own
/// `instance_record_fingerprint` moved and caches every other record verbatim — so a key that failed to
/// read `hidden` would leave an un-hidden object at `[0,0,0]` for the rest of the session while the row,
/// the projection and the history all read `hidden=false`, and nothing but the world would disagree.
/// Browser battery #64 reported `outliner-show-restores restored=false` while the `seed-left-001` row had
/// already come back to `Hide` (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B57), and no row-text verdict
/// can tell those two apart.
#[semio_framework_async_macros::async_test]
async fn outliner_show_restores_the_world_instance_scale_in_the_same_settle() {
    fn instance_scale(world: &Value, object_id: &str) -> Vec<f64> {
        let instances: Value = parse(world.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("the world body publishes an instances lane"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("instances lane is json");
        instances
            .as_array()
            .expect("instances lane is an array")
            .iter()
            .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id))
            .and_then(|instance| instance.get("scale").and_then(Value::as_array))
            .map(|scale| scale.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default()
    }
    let flag_args = |object_id: &str, value: bool| json!({ "entity": "object", "hidden": value, "ids": [object_id] });
    let mut app = app().await;
    let object_id = first_object_id(&app);
    assert_eq!(instance_scale(&render_body(&mut app, main::BODY_KEY).await, &object_id), vec![1.0, 1.0, 1.0], "a visible object publishes its real scale");
    dispatch(&mut app, "setSelectionHidden", Some(&flag_args(&object_id, true)), None).await.expect("setSelectionHidden hidden=true");
    assert_eq!(instance_scale(&render_body(&mut app, main::BODY_KEY).await, &object_id), vec![0.0, 0.0, 0.0], "the hide half must still reach the world lane");
    dispatch(&mut app, "setSelectionHidden", Some(&flag_args(&object_id, false)), None).await.expect("setSelectionHidden hidden=false");
    let shown = render_body(&mut app, main::BODY_KEY).await;
    assert_eq!(object_flag(&app, &object_id, "hidden"), Some(false), "the show write must reach the document");
    assert_eq!(instance_scale(&shown, &object_id), vec![1.0, 1.0, 1.0], "un-hiding must republish the object's real scale — a residency that cached the zero-scale record would leave it invisible for good");
}

/// 🎯️ An outliner row's flag write names its OWN entity, so the live selection must not decide what it
/// hits. `set_selection_flag` chooses between the explicit `{entity, ids}` the row declares and the whole
/// live selection, and the browser only ever exercised the branch with NOTHING selected: with a sibling
/// entity selected, `outliner-hide-applies` failed on the same row with the same args while the command
/// still settled `command-complete` with `historyUpserts: 0` (ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B47 §6 — PASS with an empty selection at 2 345 ms, FAIL with a sibling selected at 30 237 ms). Locked,
/// too: a lock guards an entity's GEOMETRY, and a row that says "Hide" must still hide.
#[semio_framework_async_macros::async_test]
async fn an_explicit_outliner_flag_write_ignores_whatever_is_selected() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select a vortex, i.e. NOT the object the row names");
    dispatch(&mut app, "setSelectionLocked", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "locked": true })), None).await.expect("lock the object by its own row");
    assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "an explicit lock write lands while a vortex is the live selection");

    dispatch(&mut app, "setSelectionHidden", Some(&json!({ "entity": "object", "hidden": true, "ids": [object_id.clone()] })), None).await.expect("hide the object by its own row");

    assert_eq!(object_flag(&app, &object_id, "hidden"), Some(true), "the row's explicit id decides what is hidden — never the live selection, and never its lock");
}

/// 🎯️ The rendered outliner row that offers `verb` and whose ONE target names `object_id` — the row a host binds that verb to.
fn outliner_verb_row<'a>(node: &'a Value, object_id: &str, verb: &str) -> Option<&'a Value> {
    let offers = node.get("rowActions").and_then(Value::as_array).is_some_and(|actions| actions.iter().any(|action| action.get("verb").and_then(Value::as_str) == Some(verb)));
    let names = node.pointer("/target/args/ids").and_then(Value::as_array).is_some_and(|ids| ids.iter().any(|id| id.as_str() == Some(object_id)));
    if offers && names {
        return Some(node);
    }
    match node {
        Value::Object(fields) => fields.iter().find_map(|(_, child)| outliner_verb_row(child, object_id, verb)),
        Value::Array(items) => items.iter().find_map(|item| outliner_verb_row(item, object_id, verb)),
        _ => None,
    }
}

/// 🙈️ LAW: the outliner's set-verbs are idempotent by value — replaying `setSelectionHidden`/`setSelectionLocked` with the
/// same `{entity, ids, flag: true}` (a stale view's second click) leaves exactly the first dispatch's one applied history
/// row and a byte-identical document; and a set-verb without its value is refused at the command boundary, never defaulted.
#[semio_framework_async_macros::async_test]
async fn outliner_set_verbs_are_idempotent_by_value_and_refuse_a_missing_value() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let bare = json!({ "entity": "object", "ids": [object_id.clone()] });
    for (verb, flag, args) in [
        ("setSelectionHidden", "hidden", json!({ "entity": "object", "hidden": true, "ids": [object_id.clone()] })),
        ("setSelectionLocked", "locked", json!({ "entity": "object", "ids": [object_id.clone()], "locked": true })),
    ] {
        assert!(Puzzle3dPlayApp::command_from_action(verb, Some(&semio_framework_pack_json::to_dsl_value(&bare))).is_err(), "{verb} without its {flag} value must be refused");
        let (first, settled) = dispatch_reporting(&mut app, verb, Some(&args), None).await;
        first.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        let rows = history_row_labels(&settled);
        assert_eq!(rows.iter().filter(|row| row.contains("applied=true") && !row.contains("ops=0")).count(), 1, "{verb} commits exactly one document edit: {rows:?}");
        assert_eq!(object_flag(&app, &object_id, flag), Some(true), "{verb} sets {flag}");
        let set = committed_document(&app);
        let (replay, settled) = dispatch_reporting(&mut app, verb, Some(&args), None).await;
        replay.unwrap_or_else(|fault| panic!("{verb} replay: {fault:?}"));
        let rows = history_row_labels(&settled);
        assert!(rows.iter().all(|row| !row.contains("applied=true") || row.contains("ops=0")), "replaying {verb} with the same value commits no second edit: {rows:?}");
        assert_eq!(committed_document(&app), set, "replaying {verb} with the same value leaves the document byte-identical");
    }
}

fn first_target_volume_id(app: &Puzzle3dApp) -> String {
    let projection = projection_of(app);
    projection
        .get("targetVolumes")
        .or_else(|| projection.get("target_volumes"))
        .and_then(Value::as_array)
        .and_then(|volumes| volumes.first())
        .and_then(|volume| volume.get("id").and_then(Value::as_str))
        .expect("a target volume is present")
        .to_string()
}

fn volume_origin(app: &Puzzle3dApp, volume_id: &str) -> Vec<f64> {
    let projection = projection_of(app);
    let volumes = projection.get("targetVolumes").or_else(|| projection.get("target_volumes")).and_then(Value::as_array).cloned().unwrap_or_default();
    volumes
        .iter()
        .find(|volume| volume.get("id").and_then(Value::as_str) == Some(volume_id))
        .and_then(|volume| volume.get("origin").or_else(|| volume.get("position")).and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn relocate_target_volume_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let volume_id = first_target_volume_id(&app);
    let start = volume_origin(&app, &volume_id);
    let pose = |position: [f64; 3]| json!({ "position": position, "quaternion": [0.0, 0.0, 0.0, 1.0], "scale": [1.0, 1.0, 1.0] });
    let (before, after) = ([start[0], start[1], start[2]], [start[0] + 3.0, start[1] + 3.0, start[2] + 3.0]);
    let (_, settled) = dispatch_reporting(&mut app, "relocateTargetVolume", Some(&json!({ "volumeId": volume_id, "mode": "translate", "before": pose(before), "after": pose(after) })), None).await;
    let rows = edit_rows(&settled);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection") && rows[0].transaction.is_some(), "the volume gumball is one relative drag transaction: {rows:?}");
    let moved = volume_origin(&app, &volume_id);
    assert!(moved.iter().zip(after).all(|(landed, wanted)| (landed - wanted).abs() < 1e-9), "relocateTargetVolume must land the gumball's after pose, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(volume_origin(&app, &volume_id), start, "undo restores the volume pose");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(volume_origin(&app, &volume_id), moved, "redo reapplies the volume pose");
}

#[semio_framework_async_macros::async_test]
async fn world_relocate_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [9.0, 8.0, 7.0] })), None).await.expect("worldRelocate");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 9.0).abs() < 1e-9 && (moved[1] - 8.0).abs() < 1e-9 && (moved[2] - 7.0).abs() < 1e-9, "worldRelocate must write the world position, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "undo restores the object origin");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_origin(&app, &object_id), moved, "redo reapplies the object origin");
}

/// 🚚️ Wave W-Y: Nakagin-scale `worldRelocate` of one object must admit and complete inside the
/// command work budget — the extent is the true per-item vortex walk, not `objects + attractions`.
#[semio_framework_async_macros::async_test]
async fn world_relocate_on_nakagin_admits_and_completes() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [4.0, 5.0, 6.0] })), None).await.expect("worldRelocate nakagin");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 4.0).abs() < 1e-6 && (moved[1] - 5.0).abs() < 1e-6 && (moved[2] - 6.0).abs() < 1e-6, "nakagin relocate must land, got {moved:?} from {start:?}");
}

/// 🚚️ Wave B5: the Relocate utility's single commit, both answers. An UNLOCKED object takes the absolute
/// world origin the host's pointer-up sends and publishes a real document edit; the SAME verb on a LOCKED
/// object refuses with exactly one `selection_locked` notice, no edit and no movement — the identical
/// refusal `translateSelection` already owes a locked selection, which this verb used to skip silently
/// (the host's relocate ghost just snapped back with nothing said). The declared `ActionKind` is pinned
/// here too: `worldRelocate` writes the document, so undo/redo and the history panel must see a
/// `Mutation`, never the `View` the 2026-09-10 checklist reverification claimed.
#[semio_framework_async_macros::async_test]
async fn world_relocate_moves_an_unlocked_object_and_refuses_a_locked_one_with_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let moved = dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id.as_str(), "position": [3.0, -2.0, 1.5] })), None).await.expect("worldRelocate unlocked");
    assert!(notices(&moved).is_empty(), "an unlocked relocate must not refuse: {:?}", moved.requested_effects);
    let landed = object_origin(&app, &object_id);
    assert!((landed[0] - 3.0).abs() < 1e-9 && (landed[1] + 2.0).abs() < 1e-9 && (landed[2] - 1.5).abs() < 1e-9, "worldRelocate must write the absolute origin, got {landed:?}");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.as_str()], "flag": "locked", "value": true })), None).await.expect("lock");
    let refused = dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id.as_str(), "position": [9.0, 9.0, 9.0] })), None).await.expect("worldRelocate locked");
    let raised = notices(&refused);
    assert_eq!(raised.len(), 1, "a locked relocate must raise exactly one notice: {:?}", refused.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so the refusal must be real prose");
    assert!(refused.mutations.is_empty(), "a locked relocate must emit no edit: {:?}", refused.mutations);
    assert_eq!(object_origin(&app, &object_id), landed, "a locked object must not move");
    let definition = create_puzzle3d_app();
    let declared = dispatchable_actions(&definition).into_iter().find(|action| action.id == "worldRelocate").map(|action| action.kind).expect("worldRelocate is declared");
    assert_eq!(declared, ActionKind::Mutation, "worldRelocate edits the document, so history and undo must see a Mutation");
}

/// 📋️ Wave W-Y: copy then paste clones the selection with new ids as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn copy_then_paste_clones_selection_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let copied = dispatch(&mut app, "copy", None, None).await.expect("copy");
    let fragment = copied.requested_effects.iter().find_map(|effect| match effect {
        Effect::ClipboardWrite { fragment } => Some(fragment.clone()),
        _ => None,
    }).expect("copy must emit ClipboardWrite");
    let before = object_count(&app);
    let paste_args = json!({ "fragment": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&fragment)) });
    let pasted = dispatch(&mut app, "paste", Some(&paste_args), None).await.expect("paste");
    assert!(!pasted.mutations.is_empty(), "paste must emit one mutation edit: {:?}", pasted.mutations);
    assert_eq!(object_count(&app), before + 1, "paste clones the selection");
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).unwrap().iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    assert!(ids.iter().any(|id| id != &object_id), "the clone must carry a new id: {ids:?}");
}

/// ✂️ Wave W-Y: cut is copy + delete as one undo step.
#[semio_framework_async_macros::async_test]
async fn cut_undoes_as_one_step() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let before = object_count(&app);
    dispatch(&mut app, "cut", None, None).await.expect("cut");
    assert_eq!(object_count(&app), before - 1, "cut removes the selection");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), before, "one undo restores the cut");
}

/// ⬇️ Wave W-Y: export carries the full round-trippable scene_snapshot JSON.
#[semio_framework_async_macros::async_test]
async fn export_snapshot_downloads_round_trippable_json() {
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, "exportSnapshot", None, None).await.expect("export");
    let data = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, mime_type, data, .. } => {
            assert_eq!(filename, "concrete-forest.json", "a fresh session exports under the example its document was seeded from");
            assert_eq!(mime_type, "application/json");
            Some(data.clone())
        }
        _ => None,
    }).expect("export must emit DownloadMediaExport");
    let exported: Value = parse(&data, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("export JSON parses");
    assert_eq!(exported.get("schema"), before.get("schema"), "export schema must match the live scene_snapshot");
    assert_eq!(exported.get("objects").and_then(Value::as_array).map(Vec::len), before.get("objects").and_then(Value::as_array).map(Vec::len));
}

/// 🧬️ Identity of imported objects without scene_snapshot/snapshot projection twins (anchor / null scale).
fn object_cores(value: &Value) -> Vec<(String, String, String, Value, Vec<String>)> {
    value
        .get("objects")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|object| {
            let id = object.get("id").and_then(Value::as_str)?.to_string();
            let kind = object.get("objectKind").and_then(Value::as_str).unwrap_or_default().to_string();
            let mesh = object.get("meshUrl").and_then(Value::as_str).unwrap_or_default().to_string();
            let origin = object.get("origin").cloned().unwrap_or(Value::Null);
            let vortices = object
                .get("vortices")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|vortex| vortex.get("id").and_then(Value::as_str).map(str::to_string))
                .collect();
            Some((id, kind, mesh, origin, vortices))
        })
        .collect()
}

/// 📥️ Wave W-Y: importing that JSON reproduces the document as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_reproduces_the_exported_document() {
    let mut app = app().await;
    let source = projection_of(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("import");
    assert_eq!(loads_in(&settled), 1, "import is one whole-document load");
    assert!(history_row_labels(&settled).iter().all(|row| row.contains("applied=false") && row.contains("ops=0")), "a whole-document load journals no applied history row: {:?}", history_row_labels(&settled));
    assert_eq!(object_cores(&imported_projection(&settled)), object_cores(&source), "import reproduces the exported objects");
}

/// 📥️ Wave W-AB: workspace Import is `openImportSnapshot` (file picker). `importSnapshot` stays
/// dispatchable for the host re-dispatch after the pick, but is not a file-menu row.
#[test]
fn file_menu_import_row_opens_the_file_picker() {
    let definition = create_puzzle3d_app();
    let file: Vec<(&str, bool)> = dispatchable_actions(&definition)
        .into_iter()
        .filter(|action| action.category.as_deref() == Some("file"))
        .map(|action| (action.id.as_str(), action.in_palette))
        .collect();
    assert!(file.iter().any(|(id, _)| *id == "exportSnapshot"), "file menu keeps Export: {file:?}");
    assert!(file.iter().any(|(id, in_palette)| *id == "openImportSnapshot" && *in_palette), "file menu Import is openImportSnapshot: {file:?}");
    assert!(!file.iter().any(|(id, _)| *id == "importSnapshot"), "importSnapshot is the picker completion, not a menu row: {file:?}");
    let import = dispatchable_actions(&definition).into_iter().find(|action| action.id == "importSnapshot").expect("importSnapshot stays dispatchable");
    assert!(!import.in_palette);
    assert_eq!(import.category.as_deref(), None);
}

/// 📥️ Wave W-AB: activating Import requests a file open; completing it with scene_snapshot JSON imports.
#[semio_framework_async_macros::async_test]
async fn open_import_snapshot_requests_file_open_then_import_applies_payload() {
    let mut app = app().await;
    let source = projection_of(&app);
    let opened = dispatch(&mut app, "openImportSnapshot", None, None).await.expect("openImportSnapshot");
    let req = opened.requested_effects.iter().find_map(|effect| match effect {
        Effect::RequestFileOpen { accept, read_as, import_action, multiple, .. } => {
            assert!(accept.contains("json"), "picker accepts JSON: {accept}");
            assert_eq!(read_as.as_deref(), Some("text"));
            assert_eq!(import_action, "importSnapshot");
            assert!(!*multiple);
            Some(())
        }
        _ => None,
    });
    assert!(req.is_some(), "Import must emit RequestFileOpen: {:?}", opened.requested_effects);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("import");
    assert_eq!(loads_in(&settled), 1, "import is one whole-document load");
    assert_eq!(object_cores(&imported_projection(&settled)), object_cores(&source), "picked payload reproduces the exported objects");
}

/// 📥️ Wave W-AB: re-importing the live scene_snapshot is a store identity, not a guest payload dedupe.
/// `import_snapshot` always assigns. The store folds equal content into no edit, so the admitting
/// settle carries one unapplied `importSnapshot` row (`ops=0`) — the empty-emit command log — and
/// the object cores do not move.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_of_the_live_document_records_whether_identical_content_is_an_edit() {
    let mut app = app().await;
    let source = projection_of(&app);
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&source) })), None).await;
    imported.expect("reimport");
    let objects_after = object_cores(&projection_of(&app));
    assert_eq!(objects_after, object_cores(&source), "identical payload must not rewrite object cores");
    let rows = history_row_labels(&settled);
    assert_eq!(history_rows(&settled), 1, "identical live scene_snapshot logs the command and no edit: {rows:?}");
    assert!(rows.iter().all(|row| row.contains("importSnapshot") && row.contains("applied=false") && row.contains("ops=0")), "{rows:?}");
}

/// 📥️ Wave W-AB #44: leftover `importSnapshot` against a one-object live scene_snapshot applies a distinct two-object JSON.
#[semio_framework_async_macros::async_test]
async fn import_snapshot_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations() {
    let source = include_str!("../../🦀️.rs");
    assert!(
        source.contains(r#""importSnapshot" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance))"#),
        "importSnapshot must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("seed");
    assert_eq!(object_count(&app), 1, "live scene_snapshot must start as one object");
    let live_id = first_object_id(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("distinct");
    let two = projection_of(&app);
    let cores = object_cores(&two);
    assert_eq!(cores.len(), 2, "distinct payload must carry two objects");
    let distinct_id = cores.iter().map(|row| row.0.as_str()).find(|id| *id != live_id).expect("distinct object id").to_string();
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&two) })), None).await;
    imported.expect("import distinct");
    assert_eq!(loads_in(&settled), 1, "distinct two-object import is one whole-document load");
    let after = object_cores(&imported_projection(&settled));
    assert_eq!(after.len(), 2, "after-snapshot must contain both objects");
    assert!(after.iter().any(|row| row.0 == distinct_id), "after-snapshot must contain the distinct object id {distinct_id}");
}

/// 📥️ Wave B9 lane 3: the picker's REAL round trip — the bytes `exportSnapshot` hands the host's
/// download are the bytes the file chooser hands `importSnapshot` back. A distinct export must upsert
/// its objects and record one history row; re-importing the SAME file against the document it
/// describes must stay an identity no-op. The browser measured `effects:0 historyUpserts:0` on the
/// distinct half, which is also exactly what a silently-dropped refusal notice looks like — so this
/// law asserts the absence of a refusal as well as the presence of the edit.
#[semio_framework_async_macros::async_test]
async fn exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity() {
    let exported = |result: &semio_framework_plugin::InvocationResult| -> String {
        result
            .requested_effects
            .iter()
            .find_map(|effect| match effect {
                Effect::DownloadMediaExport { data, .. } => Some(data.clone()),
                _ => None,
            })
            .expect("exportSnapshot emits one DownloadMediaExport")
    };
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    let one_object = exported(&dispatch(&mut app, "exportSnapshot", None, None).await.expect("export the boot document"));
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("seed a second object");
    let two_objects = exported(&dispatch(&mut app, "exportSnapshot", None, None).await.expect("export the distinct document"));
    assert_ne!(one_object, two_objects, "the two exports must be distinct files");
    let (restored, restored_settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": one_object.as_str(), "name": "puzzle-3d.json" })), None).await;
    let restored = restored.expect("restore the one-object document");
    assert!(notices(&restored).is_empty(), "restoring an exported file must not refuse: {:?}", notices(&restored));
    assert_eq!(object_cores(&imported_projection(&restored_settled)).len(), 1, "the restored file is the one-object document");
    let (distinct, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": two_objects.as_str(), "name": "puzzle-3d-distinct.json" })), None).await;
    let distinct = distinct.expect("import the distinct file");
    assert!(notices(&distinct).is_empty(), "a distinct exported file must not refuse: {:?}", notices(&distinct));
    assert_eq!(loads_in(&settled), 1, "a distinct exported file is one whole-document load");
    let after_distinct = object_cores(&imported_projection(&settled));
    assert_eq!(after_distinct.len(), 2, "a distinct exported file must carry its objects");
    let (again, again_settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": two_objects.as_str(), "name": "puzzle-3d-distinct.json" })), None).await;
    let again = again.expect("re-import the same file");
    assert!(notices(&again).is_empty(), "re-importing the same file must not refuse: {:?}", notices(&again));
    assert_eq!(object_cores(&imported_projection(&again_settled)), after_distinct, "re-importing the same file loads the same document");
}

/// 🔢️ The payload an import actually receives was written by a BROWSER, not by this crate's own writer:
/// the file chooser hands back whatever `JSON.stringify` produced, which spells a whole float as `1` (no
/// `.0`), a tiny one in exponent form (`-5.551115123125783e-17`), an absent optional as `null`, and a
/// non-ASCII label as a `\u` escape. The law above only ever fed `import_snapshot` this crate's own export
/// text, so none of those spellings was under test — and the browser's `import-distinct` read
/// `effects:0 historyUpserts:0` with NO refusal notice, which is exactly what `import_snapshot`'s
/// `ctx.abort` path looks like when `parse` rejects the text it was handed (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B57 §2). One unreadable spelling is a silently dropped document.
#[semio_framework_async_macros::async_test]
async fn a_browser_serialized_fixture_payload_imports_every_json_number_spelling() {
    let mut app = app().await;
    let payload = r#"{"schema":"puzzle.3d.scene.v1","domain":"architecture","objects":[{"id":"browser-clone","label":"Distinct Capsule J · cs_sl1","objectKind":"Object","origin":[-16.75,-3.6499999999999986,0],"orientation":[-5.551115123125783e-17,5.551115123125783e-17,0.7071067811865475,0.7071067811865475],"scale":null,"vortices":[{"id":"browser-clone:v0","position":[1,0,2]}],"hidden":false,"locked":false}],"attractions":[],"targetVolumes":[],"references":[]}"#;
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": payload, "name": "browser-stringify.json" })), None).await;
    let imported = imported.expect("import a browser-serialized payload");
    let notices: Vec<String> = imported
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert!(notices.is_empty(), "a browser-serialized payload must not be refused: {notices:?}");
    assert_eq!(loads_in(&settled), 1, "a browser-serialized payload is one whole-document load");
    let objects = object_cores(&imported_projection(&settled));
    assert_eq!(objects.len(), 1, "the imported document is exactly the payload's objects: {objects:?}");
    assert_eq!(objects.first().map(|(id, ..)| id.as_str()), Some("browser-clone"), "the payload's own object id survives the import: {objects:?}");
}

/// 🧱️ Wave B59: the payload the browser hands `importSnapshot` for a REAL example is 145 714 B — the
/// Nakagin Capsule Tower export, 180 objects — and the live `import-distinct` verdict read
/// `paneObjects=180→180` with no notice at all (wave B57 §2.3). Every import law before this one fed a
/// payload of a few hundred bytes, so the size class the product actually imports was never under test.
///
/// 🧩️ The payload reaches the action the way the framework hands it once it has reassembled the host's
/// chunks (`semio_framework::kernel::ImportStaging`): ONE whole `{payload, name}`. The witnesses are the
/// browser's own three: the object census moves, exactly ONE applied history row lands, nothing is refused.
#[semio_framework_async_macros::async_test]
async fn a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [220.0, 0.0, 0.0] })), None).await.expect("seed one more object");
    let distinct = crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("typed app snapshot").typed()));
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("return to the example document");
    assert_eq!(object_count(&app), seeded, "the document is back at the example census before the import");
    assert!(distinct.len() > 140_000, "the payload under test must be the product's own size class; observed {} B", distinct.len());
    let (result, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": distinct.as_str(), "name": "nakagin-capsule-tower-distinct.json" })), None).await;
    let result = result.expect("a product-sized import inside the budget is admitted");
    let notices: Vec<String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert!(notices.is_empty(), "a payload inside the declared import budget must not be refused: {notices:?}");
    assert_eq!(loads_in(&settled), 1, "the whole import is exactly one whole-document load");
    assert_eq!(object_cores(&imported_projection(&settled)).len(), seeded + 1, "a product-sized distinct payload must carry the document it replaces");
}

/// 🧩️ The framework half of an import, end to end for this app: the host's chunk envelope
/// (`semio_framework::kernel::import_chunk_arguments`) goes through the SDK's own action dispatch, whose
/// instance-scoped `ImportStaging` stages every chunk but the last — no command, no edit — and hands
/// `importSnapshot` the whole file once the run closes: ONE applied history row, the census moved.
#[semio_framework_async_macros::async_test]
async fn a_chunked_pick_reaches_import_snapshot_as_one_whole_file_through_the_framework_staging() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [220.0, 0.0, 0.0] })), None).await.expect("seed one more object");
    let distinct = crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("typed app snapshot").typed()));
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("return to the example document");
    let chunks = semio_framework::kernel::import_payload_chunks(&distinct);
    assert!(chunks.len() > 1, "a product-sized payload spans several host chunks; observed {}", chunks.len());
    let mut loaded = None;
    for chunk in &chunks {
        let args = semio_framework::kernel::import_chunk_arguments("nakagin-capsule-tower-distinct.json", chunk, None);
        let (result, settled) = dispatch_action_reporting(&mut app, "importSnapshot", &args).await;
        result.expect("every chunk of a product-sized pick is admitted");
        let rows = history_row_labels(&settled);
        if chunk.chunk + 1 < chunk.chunk_count {
            assert!(rows.is_empty(), "a staged chunk is no command and no edit; chunk {}: {rows:?}", chunk.chunk);
            assert_eq!(object_count(&app), seeded, "a staged chunk must not move the document; chunk {}", chunk.chunk);
        }
        if loads_in(&settled) == 1 {
            assert!(loaded.is_none(), "the whole chunked pick is exactly one whole-document load");
            loaded = Some(imported_projection(&settled));
        }
    }
    let loaded = loaded.expect("the closing chunk publishes the whole-document load");
    assert_eq!(object_cores(&loaded).len(), seeded + 1, "the reassembled file carries the document it replaces");
}

/// 🧯️ An import above the budget one export may stream is REFUSED with a notice, never silently dropped
/// and never a fault — the framework hands the whole file, so this budget is the app's one size rule.
#[semio_framework_async_macros::async_test]
async fn an_import_above_the_export_budget_refuses_with_a_notice() {
    use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("load the nakagin example");
    let seeded = object_count(&app);
    let oversized = "x".repeat(PUZZLE_IMPORT_TOTAL_BYTES + 1);
    let (result, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": oversized.as_str(), "name": "oversized.json" })), None).await;
    let result = result.expect("an over-budget import is an ANSWER, never a fault");
    let notices: Vec<String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(notices.len(), 1, "an over-budget import publishes exactly one notice: {notices:?}");
    let rows = history_row_labels(&settled);
    assert_eq!(history_rows(&settled), 1, "a refused import logs the command and no edit: {rows:?}");
    assert!(rows.iter().all(|row| row.contains("importSnapshot") && row.contains("applied=false") && row.contains("ops=0")), "{rows:?}");
    assert_eq!(object_count(&app), seeded, "a refused import leaves the document alone");
}

/// 📤️📥️ LAW: the Nakagin export re-imports as the very document it came from — the whole exported file,
/// read by `puzzle3d_import_value`, rebuilds a scene_snapshot whose own export is byte-identical.
#[test]
fn the_nakagin_export_reimports_byte_for_byte() {
    use crate::editor::puzzle3d::commands::import_snapshot::puzzle3d_import_value;
    let payload = crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&NAKAGIN_EXAMPLE_SNAPSHOT.clone());
    assert!(payload.len() > semio_framework::kernel::IMPORT_CHUNK_BYTES, "the payload under test spans several host chunks: {} B", payload.len());
    let root = puzzle3d_import_value(&json!({ "payload": payload.as_str(), "name": "nakagin.json" })).expect("the whole export is a puzzle 3D document");
    assert_eq!(
        crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&Puzzle3dSceneSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&root)).expect("the imported root IS a puzzle 3D document")),
        payload,
        "the import is byte-identical to the document that was exported"
    );
}

/// 📥️ Wave B16: leftover `exportSnapshot` must emit `DownloadMediaExport`.
/// 🏷️ Wave B30: a fresh session's document IS the Concrete Forest example
/// (`ArtifactApp::initial_snapshot`), and since the config lane says so from the first render the
/// boot download is named after it. The app-generic `puzzle-3d.json` belongs to a document that came
/// from no example at all — see `export_snapshot_names_the_download_after_the_active_example`, which
/// reaches that state by clearing the picker.
#[semio_framework_async_macros::async_test]
async fn leftover_export_snapshot_downloads_the_boot_example_json() {
    let source = include_str!("../../🦀️.rs");
    assert!(
        source.contains(r#""exportSnapshot" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance))"#),
        "exportSnapshot must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    let result = dispatch(&mut app, "exportSnapshot", None, None).await.expect("leftover export");
    let filename = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, .. } => Some(filename.as_str()),
        _ => None,
    });
    assert_eq!(filename, Some("concrete-forest.json"), "leftover export must name the boot example: {:?}", result.requested_effects);
}

/// 🏷️ Wave B26: exporting Concrete Forest and exporting Nakagin must not both land as one constant
/// filename. The download is named after the ACTIVE EXAMPLE — the id `set_active_example` persists on
/// the shared config, restored into `Puzzle3dRuntime` by `window::runtime` — and a document that came
/// from no example at all keeps the app-generic `puzzle-3d.json`. Both picker aliases (`concrete`,
/// `nakagin`) resolve to the canonical id, so the filename never depends on how the row was spelled.
#[semio_framework_async_macros::async_test]
async fn export_snapshot_names_the_download_after_the_active_example() {
    /// 🏷️ The name is asserted across BOTH publication lanes, because the lane is chosen by payload size
    /// and the naming contract is not: an over-budget example publishes a segmented handle rather than an
    /// inline `DownloadMediaExport`, and it must carry the same filename (wave B38).
    async fn exported_filename(app: &mut Puzzle3dApp) -> String {
        let (result, settled) = dispatch_reporting(app, "exportSnapshot", None, None).await;
        result.expect("export");
        let inline = settled.effects.iter().find_map(|effect| match effect {
            Effect::DownloadMediaExport { filename, .. } => Some(filename.clone()),
            _ => None,
        });
        let segmented = settled.downloads.first().map(|download| download.filename.clone());
        assert!(inline.is_none() || segmented.is_none(), "one export publishes exactly one lane, never both: inline={inline:?} segmented={segmented:?}");
        inline.or(segmented).expect("export must publish a download on one of the two lanes")
    }
    let mut app = app().await;
    assert_eq!(exported_filename(&mut app).await, "concrete-forest.json", "a fresh session already names the example its document was seeded from");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("clear to a blank document");
    assert_eq!(exported_filename(&mut app).await, "puzzle-3d.json", "a document that came from no example keeps the app-generic name");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "concrete-forest" })), None).await.expect("load concrete forest");
    assert_eq!(exported_filename(&mut app).await, "concrete-forest.json", "Concrete Forest must export under its own id");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "nakagin" })), None).await.expect("load nakagin by alias");
    assert_eq!(exported_filename(&mut app).await, "nakagin-capsule-tower.json", "the picker alias must still export the canonical example id");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("clear to a blank document");
    assert_eq!(exported_filename(&mut app).await, "puzzle-3d.json", "clearing the example clears the name it exported under");
}

/// ⬇️ Wave B43: an export above what ONE segmented download may carry is refused with a NOTICE, and the
/// budget it is measured against is the framework's own end-to-end cap — never a literal of this app's.
///
/// 🧯 A fault here would land as a dead job; before B43 the whole lane's over-cap answer was a producer
/// `Fault`. The refusal names the file, its size and the budget, so the answer is readable.
#[test]
fn export_refuses_a_payload_above_the_declared_segmented_budget_with_a_notice() {
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_refusal;
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_segmented;
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_segmented_budget_bytes;
    use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
    use semio_framework_plugin::app::ArtifactOutputChunks;
    let budget = puzzle3d_export_segmented_budget_bytes().expect("the declared output budget is within the framework cap");
    assert_eq!(budget, PUZZLE_COMMAND_OUTPUT_BYTES, "the segmented budget IS this command's declared contract output budget");
    assert!(budget <= ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, "no app may declare a download larger than the wire admits");
    let over = "x".repeat(budget + 1);
    assert!(puzzle3d_export_segmented("nakagin-capsule-tower.json".into(), &over).is_err(), "a payload above the budget must never open a download handle");
    let notice = puzzle3d_export_refusal("nakagin-capsule-tower.json", budget + 1, budget);
    assert!(notice.contains("nakagin-capsule-tower.json") && notice.contains(&(budget + 1).to_string()) && notice.contains(&budget.to_string()), "the refusal must name the file, its size and the budget: {notice}");
}

/// ⬇️ Wave B38: an export larger than one wire page must reach the user as ONE download carrying the WHOLE
/// scene_snapshot, through the framework's segmented lane — never as an inline effect field.
///
/// 🧊️ The lane is picked by size against the guest's own contiguous-request ceiling
/// (`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`, one wasm page), derived from the wire constant rather than
/// spelled as a literal: Concrete Forest (7 542 B) rides inline, Nakagin Capsule Tower (145 714 B) does not.
/// The browser measured exactly that split — the inline Nakagin effect left the guest (`"effects":1`) and
/// reached no file (`📓️2026-09-12-wave-B36-full-run-bisect-2.md` §5).
///
/// 🧾️ The assertion is the REASSEMBLED payload, drained one bounded chunk at a time exactly as
/// `drainSegmentedMediaExport` drains it, parsed back to JSON and compared object-for-object with the live
/// document — so a lane that publishes a handle but loses, reorders or truncates chunks fails here.
#[semio_framework_async_macros::async_test]
async fn export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture() {
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_inline_budget_bytes;
    use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json;
    assert_eq!(
        puzzle3d_export_inline_budget_bytes(),
        semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "the inline budget is the guest's own contiguous-request ceiling, never an independent literal"
    );
    let mut app = app().await;

    // 🌲️ Below the budget: the inline lane, unchanged, and NO segmented handle at all.
    let (small, small_settled) = dispatch_reporting(&mut app, "exportSnapshot", None, None).await;
    small.expect("concrete forest export");
    let small_bytes = puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("live snapshot").typed())).len();
    assert!(small_bytes <= puzzle3d_export_inline_budget_bytes(), "Concrete Forest must sit under the inline budget, got {small_bytes} B");
    assert!(small_settled.downloads.is_empty(), "a payload that fits one page must not open a segmented handle: {:?}", small_settled.downloads);
    let inline = small_settled.effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, data, encoding, .. } => Some((filename.clone(), data.len(), encoding.clone())),
        _ => None,
    });
    assert_eq!(inline, Some(("concrete-forest.json".to_string(), small_bytes, Some("utf-8".to_string()))), "the inline lane carries the whole payload and the utf-8 encoding");

    // 🏢️ Above the budget: one segmented handle, no inline payload, and the full JSON on drain.
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "nakagin" })), None).await.expect("load nakagin");
    let expected = puzzle3d_export_json(&puzzle3d_scene_snapshot_from_document(app.snapshot().expect("live snapshot").typed()));
    assert!(expected.len() > puzzle3d_export_inline_budget_bytes(), "Nakagin must exceed the inline budget, got {} B", expected.len());
    let (large, large_settled) = dispatch_reporting(&mut app, "exportSnapshot", None, None).await;
    large.expect("nakagin export");
    assert!(
        !large_settled.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. })),
        "an over-budget export must publish no inline download effect: {:?}",
        large_settled.effects
    );
    assert_eq!(large_settled.downloads.len(), 1, "exactly one segmented download handle: {:?}", large_settled.downloads);
    let handle = large_settled.downloads[0].clone();
    assert_eq!(handle.filename, "nakagin-capsule-tower.json", "the segmented lane keeps the per-example filename");
    assert_eq!(handle.mime_type, "application/json");
    assert_eq!(handle.encoding.as_deref(), Some("identity"), "UTF-8 JSON streams as identity, never base64");
    assert_eq!(handle.bytes, expected.len(), "the handle declares the whole payload");
    let drained = drain_segmented_download(&mut app, &handle).await;
    assert_eq!(drained.len(), expected.len(), "the drained bytes are the whole payload");
    assert_eq!(String::from_utf8(drained).expect("drained payload is UTF-8"), expected, "the drained payload IS the export, byte for byte");
    let reassembled: Value = parse(&expected, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the reassembled export parses");
    let live = projection_of(&app);
    assert_eq!(object_cores(&reassembled), object_cores(&live), "the streamed export reproduces every object of the live document");
    assert_eq!(reassembled.get("schema"), live.get("schema"));
}

/// 📥️ Wave B16: leftover `openImportSnapshot` must emit the file picker that completes as `importSnapshot`.
#[semio_framework_async_macros::async_test]
async fn leftover_open_import_snapshot_requests_file_open() {
    let source = include_str!("../../🦀️.rs");
    assert!(
        source.contains(r#""openImportSnapshot" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance))"#),
        "openImportSnapshot must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep"
    );
    let mut app = app().await;
    let opened = dispatch(&mut app, "openImportSnapshot", None, None).await.expect("leftover openImportSnapshot");
    let req = opened.requested_effects.iter().find_map(|effect| match effect {
        Effect::RequestFileOpen { accept, import_action, .. } => {
            assert!(accept.contains("json"), "picker accepts JSON: {accept}");
            assert_eq!(import_action, "importSnapshot");
            Some(())
        }
        _ => None,
    });
    assert!(req.is_some(), "leftover Import must emit RequestFileOpen: {:?}", opened.requested_effects);
}

/// 📥️ Wave B16: leftover `importSnapshot` of a distinct two-object JSON replaces the live one-object document.
#[semio_framework_async_macros::async_test]
async fn leftover_import_snapshot_replaces_live_document_with_distinct_two_object_json() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("seed");
    assert_eq!(object_count(&app), 1, "live scene_snapshot must start as one object");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("distinct");
    let two = projection_of(&app);
    assert_eq!(object_cores(&two).len(), 2, "distinct payload must carry two objects");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty again");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [0.0, 0.0, 0.0] })), None).await.expect("one-object live");
    assert_eq!(object_count(&app), 1, "live scene_snapshot is one object before import");
    let (imported, settled) = dispatch_reporting(&mut app, "importSnapshot", Some(&json!({ "payload": to_json_string(&two), "name": "puzzle-3d-distinct.json" })), None).await;
    imported.expect("leftover distinct import");
    assert_eq!(loads_in(&settled), 1, "distinct leftover import is one whole-document load");
    assert_eq!(object_cores(&imported_projection(&settled)).len(), 2, "distinct leftover import must carry both objects");
}

/// 🖱️ Wave W-Y: a selected-object context menu is puzzle-owned — never the shell fallback vocabulary.
#[semio_framework_async_macros::async_test]
async fn object_context_menu_owns_puzzle_rows_not_shell_fallback() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("duplicateSelection"), "object menu must own Duplicate: {menu_json}");
    assert!(menu_json.contains("selectSameKindSelection"), "object menu must own Select same kind: {menu_json}");
    assert!(!menu_json.contains("setActiveExample"), "object menu must not dress itself as the shell fallback: {menu_json}");
}

/// 📏️ Wave W-Y: gumball scale on a locked volume refuses with a notice and emits no edit.
#[semio_framework_async_macros::async_test]
async fn gumball_scale_on_locked_volume_refuses_without_edit() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("volume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id, "flag": "locked", "value": true })), None).await.expect("lock");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select volume");
    let before = projection_of(&app);
    let result = dispatch(&mut app, "scaleSelection", Some(&json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })), None).await.expect("scale locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked volume must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked volume must emit no edit: {:?}", result.mutations);
    assert_eq!(projection_of(&app).get("targetVolumes"), before.get("targetVolumes"));
}

/// 📋️ leftover vortex-granularity selection still captures the object, and paste clones it with a new id.
#[test]
fn leftover_copy_paste_clones_selected_object() {
    let mut scene_snapshot = empty_scene_snapshot();
    scene_snapshot.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["seed-left-001".into()],
        hovered: Vec::new(),
        referenced: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
    assert_eq!(objects.len(), 1, "leftover selected object id must copy even when granularity is vortex: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    assert_eq!(objects[0].origin, [1.0, 2.0, 3.0]);
    let fragment = puzzle3d_copy_fragment_from(&scene_snapshot, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations(&fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "paste must commit one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001", "clone must mint a fresh id");
    assert!((created[0].object.origin[0] - 1.5).abs() < 1e-9, "clone origin must keep source fields plus paste offset: {:?}", created[0].object.origin);
    assert_eq!(created[0].object.object_kind.as_deref(), Some("Object"));
    assert_eq!(created[0].object.label.as_deref(), Some("seed"));
}

/// 📋️ leftover selected vortex uuid (not object.id) still captures the parent object for copy.
#[test]
fn leftover_copy_paste_clones_object_from_selected_vortex_uuid() {
    let mut scene_snapshot = empty_scene_snapshot();
    scene_snapshot.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: vec![Puzzle3dVortex {
            id: "5de35caa-0f02-43d7-ae74-aa730efd3386".into(),
            vortex_kind: None,
            position: [0.0, 0.0, 0.0],
            direction: None,
            radius: None,
            hidden: false,
            locked: false,
        }],
        hidden: false,
        locked: false,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["5de35caa-0f02-43d7-ae74-aa730efd3386".into()],
        hovered: Vec::new(),
        referenced: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
    assert_eq!(objects.len(), 1, "leftover selected vortex uuid must resolve to the parent object: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    let fragment = puzzle3d_copy_fragment_from(&scene_snapshot, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations(&fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "vortex-uuid leftover copy must paste one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001");
}

/// 🕹️ leftover overlay → translateSelection on a locked object must refuse with a visible notice.
#[semio_framework_async_macros::async_test]
async fn leftover_translate_selection_on_locked_object_refuses_with_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.as_str()], "flag": "locked", "value": true })), None).await.expect("lock");
    let before = object_origin_x(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("translate locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked object must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked object must emit no edit: {:?}", result.mutations);
    assert!((object_origin_x(&app, &object_id) - before).abs() < 1e-9, "locked object origin must not move");
}

/// 🕹️ leftover overlay → translateSelection carries explicit ids; guest snapshot granularity may be empty.
#[semio_framework_async_macros::async_test]
async fn leftover_overlay_translate_selection_moves_unlocked_object() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let before = object_origin(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("leftover overlay translate");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::Notify { .. })), "unlocked leftover overlay must not refuse: {:?}", result.requested_effects);
    assert!((object_origin(&app, &object_id)[0] - before[0] - 4.0).abs() < 1e-9, "leftover overlay ids must move the unlocked object by dx");
}

/// 🕹️ leftover `translateSelection` with explicit ids + mesh mode commits a pose delta.
#[semio_framework_async_macros::async_test]
async fn leftover_translate_selection_mesh_mode_commits_pose_delta() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let before = object_origin(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "mode": "mesh", "dx": 3.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("leftover mesh translate");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::Notify { .. })), "mesh leftover translate must pre-admit: {:?}", result.requested_effects);
    assert!((object_origin(&app, &object_id)[0] - before[0] - 3.0).abs() < 1e-9, "leftover mesh-mode translateSelection must commit a pose delta");
}

//#endregion 🩹️CheckedDefects

/// 📏️ Wave W-M2: one whole `registerBrushMesh` page validates inside one command's own work budget.
/// The page scan is cursorized at [`PUZZLE3D_MESH_PAGE_SCAN_CHARS`] characters per step over both
/// payload streams, and the largest page the plugin admits is
/// `PUZZLE3D_MESH_PAGE_BASE64_CHARS` characters per stream — so the extent a page claims can never
/// approach `PUZZLE_COMMAND_WORK_ITEMS`, which is what keeps every per-page unit under the 8 ms law.
#[test]
fn one_brush_mesh_page_validates_inside_one_command_work_budget() {
    let page = crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS;
    let steps = page.div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).saturating_mul(2).saturating_add(1);
    assert!(steps <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "a whole page validates in {steps} bounded steps, inside {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
    assert!(page + 8 < crate::retained_command::PUZZLE_COMMAND_RAW_BYTES, "a page's two payload streams — one page's values, split at most once and padded per stream — leave the JSON envelope room inside the raw wire");
}

/// 🚚️ Wave W-H: a client whose "already uploaded" bookkeeping outlived the guest that justified it
/// announces identities this instantiation holds nothing for. The arm may not answer that with a notice
/// — nothing on the host reads a notification's text, so the brush utility would silently keep no
/// collision body until a full page reload. It publishes the identity on the world body instead, and
/// widens its own otherwise-`Quiet` scope to the viewport so that body is actually republished. The
/// client answers with the page run, and the request retires.
#[semio_framework_async_macros::async_test]
async fn an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes() {
    let mut app = app().await;
    // 🎲️ Geometry no other law in this binary derives — the brush-mesh store is content-addressed since
    // wave B22, so the shared cube's digest would be adoptable under a new id and the refusal this law is
    // about could never happen.
    let (positions, indices) = semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::precompute_model_tests::context::seeded_cube_mesh_buffers(23.0);
    let digest = crate::editor::puzzle3d::precompute::brush_mesh_digest(&positions, &indices);
    let url = "/test/restarted-guest-reannounce.glb";
    let announcement = json!({ "surfaceId": "world-3d", "url": url, "digest": digest });
    let refused = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("id-only announcement");
    assert!(!refused.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "a refusal the client cannot act on is not an answer");
    assert!(
        matches!(&refused.ui_scope, UiDirtyScope::Partial { window_bodies, .. } if window_bodies.iter().any(|body| body == main::BODY_KEY)),
        "the request is worthless until the world body carrying it repaints; got {:?}",
        refused.ui_scope
    );
    let requested = interaction_of(&render_composite(&mut app).await);
    assert_eq!(requested.pointer("/meshReuploadUrls").and_then(Value::as_array).cloned().unwrap_or_default(), vec![json!(url)], "the refused identity is published as a request for its bytes");
    let residency = requested.pointer("/meshResidency").and_then(Value::as_u64).expect("the world body publishes the guest's mesh residency");
    // 🐢️ Wave B32: the SECOND refusal of the same id is where the storm lived. The standing request set
    // is already published, so this announcement has nothing new for any surface — and republishing the
    // world body for it re-drives the client's registrar, which re-announces, which refuses again. The
    // live `:6013` shell measured 23 of 33 typed-operation completions carrying this viewport scope in a
    // 75 s window with four user actions, 23 of 31 refresh passes answering every world body
    // `unchanged`, and `translateSelection` needing 8–15 s to reach `data-instances-json` behind them.
    let repeated = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("the client re-announces the same dead identity");
    assert!(matches!(repeated.ui_scope, UiDirtyScope::None), "a re-announcement of an ALREADY standing request republishes nothing and must stay Quiet; got {:?}", repeated.ui_scope);
    let unchanged = interaction_of(&render_composite(&mut app).await);
    assert_eq!(unchanged.pointer("/meshReuploadUrls"), requested.pointer("/meshReuploadUrls"), "the second refusal leaves the published request set byte-identical, which is why it owes no repaint");

    let position_bytes: Vec<u8> = positions.iter().flat_map(|value| value.to_le_bytes()).collect();
    let index_bytes: Vec<u8> = indices.iter().flat_map(|value| value.to_le_bytes()).collect();
    let page = json!({
        "surfaceId": "world-3d",
        "url": url,
        "digest": digest,
        "page": 0,
        "pageCount": 1,
        "positionsB64": semio_framework_io_base64::base64_standard_encode(&position_bytes),
        "indicesB64": semio_framework_io_base64::base64_standard_encode(&index_bytes),
    });
    let accepted = dispatch(&mut app, "registerBrushMesh", Some(&page), None).await.expect("the client answers with the page run");
    assert!(!accepted.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "an accepted page run is silent");
    let settled = interaction_of(&render_composite(&mut app).await);
    assert_eq!(settled.pointer("/meshReuploadUrls").and_then(Value::as_array).map(Vec::len), Some(0), "an answered request retires, so a steady state carries no standing request");
    assert!(settled.pointer("/meshResidency").and_then(Value::as_u64).expect("residency") > residency, "the client's proof that this guest holds the geometry is the counter climbing");

    let readopted = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("the identity is now resident");
    assert!(readopted.requested_effects.is_empty(), "an identity this guest holds is adopted by id alone, at no cost on the wire");
    assert!(matches!(readopted.ui_scope, UiDirtyScope::None), "the adopting fast path stays the quiet command it is declared to be");
    assert!(
        !puzzle3d_action_uses_precompute("registerBrushMesh"),
        "registerBrushMesh only WRITES geometry into the precompute session, so it may not owe the sync prologue: that prologue seeds a fallback body for every id the session has no geometry for — precisely the ids this command supplies — clearing the brush cache and rebuilding the queue each time, which the arm's own install then discards, and which is where the 0.65 s → 2.85 s per announcement measured on 2026-09-09 came from"
    );
}

//#region 📷️OpeningCamera
/// 📷️ Reads one pane's published pose straight off the world lane it renders — the exact string
/// `World3dHost` parses into `data-camera-json`.
async fn published_camera(app: &mut Puzzle3dApp, window_id: &str) -> Value {
    camera_of(&render_window(app, window_id).await)
}

fn camera_position(camera: &Value) -> Vec<f64> {
    camera.get("position").and_then(Value::as_array).map(|axes| axes.iter().filter_map(Value::as_f64).collect()).unwrap_or_default()
}

/// 📷️ Every pane opens on a real pose of its own before the user has touched anything.
///
/// `Puzzle3dCamera::default()` is all zeros, and `World3dHost` frames each pane LOCALLY and
/// deliberately never dispatches that framing back (`🌐️World3dHost/🟦️.tsx`'s `WorldAutoFit`), so
/// until this wave the world lane of both panes carried position == target == `[0,0,0]` until the
/// first `setCamera` landed: no view direction at boot, and two panes of one document publishing the
/// SAME empty pose (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §4.1 measured it live, wave B15
/// fixed it).
#[semio_framework_async_macros::async_test]
async fn every_pane_opens_on_its_own_framed_camera_before_any_gesture() {
    let mut app = app().await;
    let top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    let perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    for (window_id, camera) in [(main::WINDOW_INSTANCE_TOP, &top), (main::WINDOW_INSTANCE_PERSPECTIVE, &perspective)] {
        let position = camera_position(camera);
        assert_eq!(position.len(), 3, "{window_id} publishes a three-axis camera position: {camera}");
        assert!(position.iter().any(|axis| axis.abs() > 1e-6), "{window_id} opens on a real pose, never the all-zero default: {camera}");
        assert_ne!(camera.get("position"), camera.get("target"), "{window_id}'s opening camera must look at something other than where it stands: {camera}");
    }
    assert_ne!(top, perspective, "the Top and Perspective panes of ONE document open on DIFFERENT poses: top={top} perspective={perspective}");
    assert_eq!(top.pointer("/projection/mode/kind").and_then(Value::as_str), Some("orthographic"), "the Top pane opens under the orthographic plan its layout template declares: {top}");
    assert_eq!(top.pointer("/projection/orientation/view").and_then(Value::as_str), Some("top"), "the Top pane looks straight down: {top}");
    assert_eq!(perspective.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "the Perspective pane keeps the three-point default: {perspective}");
}

/// 📷️ Swapping the example re-frames the pane on what the NEW document holds, instead of leaving the
/// camera framed on bounds nothing occupies any more.
#[semio_framework_async_macros::async_test]
async fn switching_the_example_reframes_every_unposed_pane_on_the_new_document() {
    let mut app = app().await;
    let before_perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let before_top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;

    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("switch to the Nakagin example");

    let after_perspective = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let after_top = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    assert_ne!(before_perspective, after_perspective, "the pane the switch was dispatched from re-frames: before={before_perspective} after={after_perspective}");
    assert_ne!(before_top, after_top, "a sibling pane the user never posed re-frames on the new document too: before={before_top} after={after_top}");
    assert_ne!(after_top, after_perspective, "re-framing keeps the two panes distinct: top={after_top} perspective={after_perspective}");
}

/// 📷️ A framed pane is framed ONCE: an opening pose is not a gesture, so re-reading the same pane
/// twice, and dispatching an unrelated window option in between, must publish the same pose.
#[semio_framework_async_macros::async_test]
async fn an_opening_camera_is_stable_and_never_overrules_a_pose_the_user_set() {
    let mut app = app().await;
    let opening = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("an unrelated window option");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await, opening, "an unrelated option must not move the opening pose");

    let posed = json!({ "camera": { "position": [11.0, -7.0, 5.0], "target": [1.0, 2.0, 3.0], "zoom": 1.0 } });
    dispatch(&mut app, "setCamera", Some(&posed), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("the user poses the camera");
    let user = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(camera_position(&user), vec![11.0, -7.0, 5.0], "a real gesture owns the pane's pose from then on: {user}");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await, user, "and the framing never takes it back: {user}");
}

/// 📽️ Flipping ONE pane's projection repaints THAT pane's published camera and no sibling's.
///
/// A projection is not a label on the pose — it IS the pose: `setProjection` re-derives position and
/// up from the new orientation around the unchanged target (`🎮️commands/📽️set-projection`), and the
/// `WindowOption` scope it publishes under repaints the world body, so the world lane's `cameraJson`
/// — what `World3dHost` parses into `data-camera-json` — must differ in the same settle. The browser
/// probe read `projection-repaints-camera` as bit-identical (ticket 26/09/02 wave B38 §1.3); this law
/// is what says whether the guest owes that, and it does not — the probe sampled the camera ~100 ms
/// after the click, while a measures-rail control's own round trip is documented at 0.7 s and up
/// (`🛠️ShellHelpers/🎚️measure-controls`'s `useWindowMeasureDraft`), so the value it saw flip was that
/// control's optimistic DRAFT and the camera it read was the pre-dispatch one.
#[semio_framework_async_macros::async_test]
async fn flipping_one_panes_projection_repaints_that_panes_camera_and_leaves_its_sibling_alone() {
    let mut app = app().await;
    let before = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    let sibling_before = published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await;
    assert_eq!(before.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "the law starts from the perspective pane's own three-point default: {before}");

    dispatch(&mut app, "setProjection", Some(&json!({ "field": "orthographicView", "value": "plan" })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setProjection orthographicView=plan");

    let after = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(after.pointer("/projection/mode/kind").and_then(Value::as_str), Some("orthographic"), "the flipped pane publishes the projection it was given: {after}");
    assert_eq!(after.pointer("/projection/orientation/view").and_then(Value::as_str), Some("plan"), "and the orientation it was given: {after}");
    assert_ne!(camera_position(&after), camera_position(&before), "an orthographic plan looks straight down — the POSE must move, not only its label: before={before} after={after}");
    assert_ne!(after.get("up"), before.get("up"), "and the up vector is re-derived with it: before={before} after={after}");
    let eye = camera_position(&after);
    let focus = camera_position(&after.get("target").cloned().map(|target| json!({ "position": target })).expect("the camera lane carries its target"));
    assert!((eye[0] - focus[0]).abs() < 1e-9 && (eye[1] - focus[1]).abs() < 1e-9 && eye[2] > focus[2], "a plan view stands directly above what it looks at: eye={eye:?} target={focus:?}");
    assert_eq!(published_camera(&mut app, main::WINDOW_INSTANCE_TOP).await, sibling_before, "the sibling pane's camera lane is untouched by another pane's projection: {sibling_before}");

    dispatch(&mut app, "setProjection", Some(&json!({ "field": "perspectiveKind", "value": "threePoint" })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setProjection perspectiveKind=threePoint");
    let restored = published_camera(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await;
    assert_eq!(restored.pointer("/projection/mode/kind").and_then(Value::as_str), Some("threePoint"), "flipping back is symmetric: {restored}");
    assert_ne!(camera_position(&restored), camera_position(&after), "and moves the pose back off the plan axis: after={after} restored={restored}");
}
//#endregion 📷️OpeningCamera

//#region ⚙️SettingsPanelScope
/// ⚙️ The Settings panel addresses the pane the user is LOOKING at.
///
/// 🎯️ An app-level panel body is rendered through `ViewModel::for_panel()`, which clears `window_id`
/// by design — so before this wave `puzzle3d_addressed_window_id` fell through to the roster's first
/// entry, the base window KIND, and every stepper baked THAT into its args: a Settings edit landed on
/// a pane nobody has open (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §5.1 measured
/// `setGridSpacing window=Some("puzzle3d-main")` live). `focused_window_id` survives `for_panel` and
/// is what the panel resolves through now.
#[semio_framework_async_macros::async_test]
async fn the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind() {
    fn node_by_key(node: &Value, key: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(key) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| node_by_key(child, key)),
            Value::Array(items) => items.iter().find_map(|item| node_by_key(item, key)),
            _ => None,
        }
    }
    let mut app = app().await;
    for focused in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        let panel = render_panel_body(&mut app, settings_panel::BODY_KEY, Some(focused)).await;
        let rendered = panel.to_string();
        assert!(rendered.contains(focused), "the Settings section names the pane it is addressed at, so the user can see which one a bump will retune: {rendered}");
        for field in ["contact-tolerance", "proximity-radius", "chunk-size", "grid-spacing"] {
            let control = node_by_key(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}.control")).unwrap_or_else(|| panic!("the {field} stepper: {panel}"));
            let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
            assert_eq!(bindings[0].pointer("/args/windowId").and_then(Value::as_str), Some(focused), "{field} tags the focused pane, never the base window kind: {bindings:?}");
        }
    }
    // 🪟️ With nothing focused the panel still falls back to the ROSTER — and the roster arm of
    // `puzzle3d_addressed_window_id` deliberately skips ids that are a window KIND
    // (`!puzzle3d_window_id_is_kind`), so it lands on the first LIVE pane instance. That is this wave's
    // whole point: the base kind `puzzle3d-main` is a pane nobody has open, and baking it into a
    // stepper's args is the defect the wave measured. Pinning the base kind here was pinning the
    // pre-wave behaviour the same law's own docstring calls the bug.
    let unfocused = render_panel_body(&mut app, settings_panel::BODY_KEY, None).await;
    let control = node_by_key(&unfocused, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.grid-spacing.control")).expect("the grid spacing stepper");
    let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
    let addressed = bindings[0].pointer("/args/windowId").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_ne!(addressed, main::WINDOW_KIND_ID, "with nothing focused the panel addresses a LIVE pane, never the base window kind nobody has open: {bindings:?}");
    assert!([main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE].contains(&addressed.as_str()), "and that pane is one this app really has open: {addressed:?} {bindings:?}");
}

/// ⚙️ A Settings bump reaches the focused pane's own rail and leaves the sibling pane alone — the
/// split-window law A1/A2 §19 asked for, played through the id the panel itself baked.
#[semio_framework_async_macros::async_test]
async fn a_settings_bump_retunes_only_the_focused_panes_rail() {
    let spacing_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing");
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    let top_view = app.window_view(main::WINDOW_INSTANCE_TOP);
    let perspective_view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let top_before = find_measure_slider(app.window_measures(&top_view).await.get(main::WINDOW_INSTANCE_TOP).expect("top rail"), &spacing_id);
    let perspective_before = find_measure_slider(app.window_measures(&perspective_view).await.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id);
    assert_eq!(top_before, perspective_before, "both panes start on the same spacing, so a divergence below can only come from the addressing");

    let bumped = top_before.expect("the rail publishes a spacing") + 0.5;
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "windowId": main::WINDOW_INSTANCE_TOP, "value": bumped })), Some(main::WINDOW_INSTANCE_TOP)).await.expect("bump the focused pane's spacing");

    let top_view = app.window_view(main::WINDOW_INSTANCE_TOP);
    let measures = app.window_measures(&top_view).await;
    assert_eq!(find_measure_slider(measures.get(main::WINDOW_INSTANCE_TOP).expect("top rail"), &spacing_id), Some(bumped), "the focused pane's own rail carries what the Settings stepper published");
    assert_eq!(find_measure_slider(measures.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id), perspective_before, "the pane the user is NOT looking at keeps its own spacing");
}

/// ⚙️ The Settings panel must RENDER the pane it is ADDRESSED at. Every stepper tags the focused pane
/// as `windowId` (law above), but the value it shows came from `ConfigView::window`, which is `None` for
/// a panel projection (`ViewModel::for_panel` drops `window_id`) and therefore fell through to
/// `Puzzle3dWindowConfig::default()`. The panel then displayed the DEFAULT spacing while writing to the
/// focused pane: battery #53 read `settings-value-reaches-window-rail settings=10.5 windowRail=12.5`
/// after the rail had been nudged to 12.5 — the panel bumped a 10.0 it had invented
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B36).
#[semio_framework_async_macros::async_test]
async fn the_settings_panel_renders_the_focused_panes_own_value_not_a_default() {
    fn control(node: &Value, key: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(key) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| control(child, key)),
            Value::Array(items) => items.iter().find_map(|item| control(item, key)),
            _ => None,
        }
    }
    let spacing_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing");
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "windowId": main::WINDOW_INSTANCE_PERSPECTIVE, "value": 12.5 })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("nudge the perspective pane's spacing");
    let perspective_view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let rail = app.window_measures(&perspective_view).await;
    assert_eq!(find_measure_slider(rail.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective rail"), &spacing_id), Some(12.5), "the rail carries the nudge, so the panel below has something to disagree with");
    let panel = render_panel_body(&mut app, settings_panel::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await;
    let stepper = control(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.grid-spacing.control")).unwrap_or_else(|| panic!("the grid spacing stepper: {panel}"));
    let rendered = stepper.get("component").and_then(|props| props.get("value")).and_then(Value::as_f64);
    assert_eq!(rendered, Some(12.5), "the Settings panel renders the focused pane's own spacing, never a default it invented: {stepper}");
}
//#endregion ⚙️SettingsPanelScope

//#region 🔬️B23RenderedVsMutatedDocument
/// 🔬️ Wave B23 — the browser's own sequence, on the two-pane session the browser boots: pick the
/// object, read Inspection through BOTH host render routes (the panel route, which carries no
/// `window_id` at all, and the window-instance route), lock the picked selection through the
/// context-menu path (no explicit ids — the arm reads the selection the render just painted), then
/// translate it and read the world lane the pane publishes.
///
/// 🕳️ Every failing verdict of the `#48` selection-scoped family is one row of this law
/// (`inspection-object-fields id=null`, `locked-flag-row lockChrome=false`,
/// `gumball-scene-delta sceneDelta=false`, `relocate-pose-delta`), and the law is GREEN — the guest
/// keeps the pick across both routes and both mutating dispatches, including the
/// `revalidate_interaction_on_document_change` pass every artifact-intent dispatch runs.
/// What the browser has and this law cannot reach is the surface-context route
/// (`plugin_mount_surface` → `SurfaceContexts` → `plugin_render_surface`): every test context render
/// helper calls `PluginApp::render` directly with a hand-built `ViewModel`, so a body that is never
/// re-rendered — or re-rendered against another surface's retained view — is invisible here.
#[semio_framework_async_macros::async_test]
async fn a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches() {
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("browser-shaped pick");
    let panel_route = render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string();
    assert!(!panel_route.contains("puzzle3d-play-inspector.empty"), "the panel route — the only one a real Inspection refresh uses — must see the pick: {panel_route}");
    assert!(panel_route.contains(object_id.as_str()), "the panel route must name the picked object: {panel_route}");
    assert!(panel_route.contains("puzzle3d-play-inspector.object.locked"), "the panel route must assemble the lock flag_row the context menu drives: {panel_route}");
    let window_route = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!window_route.contains("puzzle3d-play-inspector.empty"), "the window-instance route must see the same pick: {window_route}");

    let before = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("lock the picked object");
    assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "the context-menu lock path must land on the object the pick named");
    dispatch(&mut app, "translateSelection", Some(&json!({ "dx": 3.0, "dy": 0.0, "dz": 0.0 })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("translate the picked object");
    let after = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    assert_ne!(before, after, "a translate taken on the SECOND mutating dispatch after the pick must still change the world lane the pane renders");
    assert!(
        !render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string().contains("puzzle3d-play-inspector.empty"),
        "two artifact-intent dispatches run two `revalidate_interaction_on_document_change` passes; neither may eat the pick"
    );
}

fn object_flag(app: &Puzzle3dApp, object_id: &str, flag: &str) -> Option<bool> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get(flag).and_then(Value::as_bool))
}
//#endregion 🔬️B23RenderedVsMutatedDocument

//#region 🔖️Pz2ShippedKindCatalog
/// 🗂️ The authored [`PUZZLE3D_SHIPPED_OBJECT_KINDS`] IS the two shipped examples' own
/// `meta.kindCatalogs` rows, in their own order — the law that lets the `objectKind` select be a
/// `const` instead of a parse of 136 395 B of DSL on the `describe()` path (slice PZ2, 2026-09-22).
/// It pays that parse HERE, once, and fails the moment an example's catalog changes without the
/// authored list following.
#[semio_framework_async_macros::async_test]
async fn shipped_object_kinds_are_the_two_examples_own_catalog_rows() {
    let mut derived: Vec<(String, String)> = Vec::new();
    for scene_snapshot in [&*CONCRETE_FOREST_EXAMPLE_SNAPSHOT, &*NAKAGIN_EXAMPLE_SNAPSHOT] {
        for entry in puzzle3d_catalog_entries(scene_snapshot, "objects") {
            if derived.len() >= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX {
                break;
            }
            let Some(id) = entry.get("id").and_then(semio_framework_value::DslValue::as_str) else {
                continue;
            };
            if derived.iter().any(|(existing, _)| existing == id) {
                continue;
            }
            derived.push((id.to_string(), catalogue::catalog_entry_label(entry)));
        }
    }
    let authored: Vec<(String, String)> = PUZZLE3D_SHIPPED_OBJECT_KINDS.iter().map(|(id, label)| ((*id).to_string(), (*label).to_string())).collect();
    assert_eq!(authored, derived, "PUZZLE3D_SHIPPED_OBJECT_KINDS drifted from the shipped documents — re-author it from `concrete-forest` then `nakagin-capsule-tower`");
}
//#endregion 🔖️Pz2ShippedKindCatalog

//#region 🔖️InitialSnapshotCost
/// 🚀️ LAW (ticket 26/09/23, H12): the initial snapshot costs its document and nothing else. Every construction of
/// this app pays for it — each mount, and every `codec` call a hub makes to create or validate a document, where
/// the interpreter multiplies it — so it must never build a scene, sync a precompute session or parse an example
/// the document does not read. Weighed by this thread's heap peak (`HeapWitness`), deterministic and independent
/// of other laws: the snapshot may peak no higher than converting the default scene_snapshot itself, within 4 KiB.
#[test]
fn the_initial_snapshot_costs_only_its_document() {
    let _ = default_scene_snapshot();
    let peak_of = |work: &dyn Fn()| {
        let before = semio_framework_trace::retained_heap_bytes_on_this_thread();
        semio_framework_trace::reset_heap_peak_on_this_thread();
        work();
        semio_framework_trace::peak_heap_bytes_on_this_thread() - before
    };
    let document = peak_of(&|| drop(Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("typed fixture admits"))));
    let snapshot = peak_of(&|| drop(<Puzzle3dPlayApp as ArtifactEditor>::initial_snapshot()));
    assert!(document > 0, "the witness weighs the document conversion: {document}");
    assert!(snapshot <= document + 4096, "the initial snapshot peaked at {snapshot} B, its document at {document} B: it does work the document never reads");
}
//#endregion 🔖️InitialSnapshotCost

/// 🛡️ The host twin carries every object of the typed document and refuses a scale the typed record cannot admit.
#[test]
fn puzzle3d_host_twin_round_trips_its_vocabulary_and_refuses_bad_scale() {
    let document=crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("authored fixture admits");
    let mut host=puzzle3d_scene_snapshot_from_document(&document);
    let admitted=crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&host).expect("the twin readmits");
    assert_eq!(admitted.objects.iter().map(|object|object.id.as_str()).collect::<Vec<_>>(),document.objects.iter().map(|object|object.id.as_str()).collect::<Vec<_>>());
    host.objects[0].scale=Some(semio_framework_value::DslValue::String("invalid".into()));
    assert!(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&host).is_err());
}
