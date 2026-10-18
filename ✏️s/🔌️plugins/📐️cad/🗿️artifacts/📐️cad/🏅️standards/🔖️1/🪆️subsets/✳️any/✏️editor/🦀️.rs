//! 📐️ CAD play app — the plugin's play app: its `ArtifactApp` impl (dispatch-only), the
//! `CadPlayRuntime` scratch mirror of `CadConfig`, the shared view/export helpers its command,
//! panel and window nodes build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

//#region 🧩️Modules
#[path = "🌉️wasm/🦀️.rs"]
pub mod wasm;
//#endregion 🧩️Modules

use semio_s_artifact_cad_cad_commands::{camera::set_camera, camera::set_projection, camera::set_projection_param};
use semio_s_artifact_cad_cad_commands::contribution::set_contributions;
use semio_s_artifact_cad_cad_commands::{engagement::engagement_abort, engagement::engagement_input, engagement::engagement_possible_select, engagement::engagement_repeat_last, engagement::engagement_submit, engagement::world_pointer_down, engagement::world_pointer_move};
use semio_s_artifact_cad_cad_commands::{io::import_cad_file, io::load_raw_request, io::save_current, io::save_in_play, io::save_selected};
use semio_s_artifact_cad_cad_commands::model_definition::set_active_example;
use semio_s_artifact_cad_cad_commands::{node::add_node, node::rename_node, node::set_node_selection};
use semio_s_artifact_cad_cad_commands::{object::add_object, object::delete_object, object::duplicate_object, object::patch_object, object::patch_selection};
use semio_s_artifact_cad_cad_commands::{reference::patch_cad_play_reference, reference::reference_hover, reference::set_reference_hidden, reference::set_reference_locked, reference::set_reference_selection};
use semio_s_artifact_cad_cad_commands::{sun::set_sun_azimuth, sun::set_sun_elevation, sun::set_sun_intensity, sun::toggle_sun};
use semio_s_artifact_cad_cad_commands::{transform::apply_transformation, transform::rotate_selection, transform::scale_selection, transform::translate_selection};
use semio_s_artifact_cad_cad_commands::utility::set_dislocate_option;
use semio_s_artifact_cad_cad_config::{cad_sun_config_to_world, CadConfig, CadConfigMutation, CadDislocateOptions, SetEdit, SetContributionsEdit};
use semio_s_artifact_cad_cad_interaction::{self, apply_event, can_commit, keyed_transitions, resolve_interaction_key, start_session, CadEngagementScratch};
use semio_s_artifact_cad_cad_modes as edit;
use semio_s_artifact_cad_cad_transform::{cad_import_object_emit, CadPaneModels, CadToolEntry, CadToolLeaf, CadTransformRecord};
use semio_s_artifact_cad_cad_modes::window_transient;
use semio_s_artifact_cad_cad_transient::CadWorldWindowTransient;
use semio_s_artifact_cad_cad_modes::{windows::building, windows::config::runtime_of, windows::energy, windows::shape, windows::structure_classic};
use semio_s_artifact_cad_cad_panels::{catalogue, document, inspection};
use semio_s_artifact_cad_cad_kit::{editor::cad::terminology::cad_is_de_locale, editor::cad::terminology::cad_labels, make_object_for_typology, CadInteractionSnapshot};
use semio_s_artifact_cad_cad_mutations::CadMutation;
use semio_s_artifact_cad_cad_model::{geometry::cad_object_from_model_element, geometry::model_element_from_cad_object, geometry::objects_from_model_snapshot, geometry::CadObject, geometry::CAD_OBJECT_PSET};
use semio_s_artifact_cad_cad_io::{export_solids_as, CadSolidExport, CAD_SOLID_EXPORT_DIALECT_STEP};
use semio_s_artifact_cad_cad_inferences::{cad_brep_kernel, cad_camera_projection_config, ensure_object_solid_handle, forest_play_scene, next_cad_id, CAD_EXAMPLE_FOREST_LEFT, CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC};
use semio_s_artifact_cad_cad_model::{snapshot::artifact_kind, snapshot::CadCamera, snapshot::CadPaneId, snapshot::CadSnapshot, snapshot::CadWorkingScene, snapshot::CAD_DOCUMENT_SCHEMA};
use semio_s_artifact_cad_cad_play::CadComposedPanes;
use {semio_s_artifact_stdio_semio_model::schema::mutations::remove_element::RemoveElement, semio_s_artifact_stdio_semio_model::schema::mutations::set_element::SetElement, semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation};
#[cfg(test)] use semio_s_artifact_stdio_semio_model::schema::mutations::insert_element::InsertElement;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelElement;
use semio_framework::kernel::Effect;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::tree_item as framework_tree_item;
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::world3d_camera_projection_json;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ChildContentView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::Menu;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::WorldSunConfig;
use semio_framework_3d_brep_engine::{Brep, GeometryHandle};
// 🚧️ SDK GAP: `ArtifactEditor`/`Editor`/`Dialect` (ticket 26/08/16 contract §2.1/§2.4)? are not yet
// in `semio_framework_plugin`'s curated crate-root re-export list (`🔌️plugin/🦀️.rs:17858`)
// — only reachable through the `app` submodule they're actually declared in. Not fixable here
// (`🧰️framework/**` is outside this packet's lease); flagged for W1-A in the migration report.
use semio_framework_plugin::app::{ArtifactEditor, Dialect, Editor};
use semio_framework_value_derive::{FromValue, ToValue};
// 🌉️ Test harness only — `protocol::json` (the `pack::json` re-export) carries its own first-party
// `json!` macro (`Value`-producing, same object/array literal syntax as `serde_json::json!`) and
// `Value` type (`Index`/`PartialEq<&str>`/`as_*` parity with `serde_json::Value`), so no
// `serde_json` dependency survives even here (ticket 26/09/01/
// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS).
#[cfg(test)]
use semio_framework_pack_json::Value;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Commands

semio_framework_plugin::app_commands! {
    /// 🎯️ `CadPlayApp::Command` — the SOLE dispatch surface for cad's own behavior, decomposed into
    /// one `🎮️commands/<group>/<command>` payload module per row. Row order IS the binary variant
    /// ordinal and the two literals are two different vocabularies (camelCase manifest action id,
    /// kebab wire keyword) — both are copied verbatim from the pre-consolidation `CadCommand` enum.
    pub enum CadCommand for CadSnapshot, CadMutation, CadConfig, CadConfigMutation, ctx = CadDispatchCtx {
        // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
        "addObject" as "add-object" => add_object::AddObject,
        "patchObject" as "patch-object" => patch_object::PatchObject,
        "patchSelection" as "patch-selection" => patch_selection::PatchSelection,
        "deleteObject" as "delete-object" => delete_object::DeleteObject,
        "duplicateObject" as "duplicate-object" => duplicate_object::DuplicateObject,
        "addNode" as "add-node" => add_node::AddNode,
        "renameNode" as "rename-node" => rename_node::RenameNode,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "applyTransformation" as "apply-transformation" => apply_transformation::ApplyTransformation,
        "importCadFile" as "import-cad-file" => import_cad_file::ImportCadFile,
        "patchCadPlayReference" as "patch-cad-play-reference" => patch_cad_play_reference::PatchCadPlayReference,
        "setReferenceHidden" as "set-reference-hidden" => set_reference_hidden::SetReferenceHidden,
        "setReferenceLocked" as "set-reference-locked" => set_reference_locked::SetReferenceLocked,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown,

        // 👁️ Local configuration only — emit app or exact-window configuration, never document operations.
        "setCamera" as "camera" => set_camera::SetCamera,
        "setProjection" as "projection" => set_projection::SetProjection,
        "setProjectionParam" as "projection-param" => set_projection_param::SetProjectionParam,
        "setDislocateOption" as "dislocate-option" => set_dislocate_option::SetDislocateOption,
        "setNodeSelection" as "set-node-selection" => set_node_selection::SetNodeSelection,
        "setReferenceSelection" as "reference-selection" => set_reference_selection::SetReferenceSelection,
        "referenceHover" as "reference-hover" => reference_hover::ReferenceHover,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "engagementPossibleSelect" as "engagement-possible-select" => engagement_possible_select::EngagementPossibleSelect,
        "engagementRepeatLast" as "engagement-repeat-last" => engagement_repeat_last::EngagementRepeatLast,
        "engagementAbort" as "engagement-abort" => engagement_abort::EngagementAbort,
        "worldPointerMove" as "world-pointer-move" => world_pointer_move::WorldPointerMove,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setContributions" as "contributions" => set_contributions::SetContributions,

        // 🐚️ Shell effects — export/import round-trips through the host, no operations either way.
        "saveSelected" as "save-selected" => save_selected::SaveSelected,
        "saveInPlay" as "save-in-play" => save_in_play::SaveInPlay,
        "saveCurrent" as "save-current" => save_current::SaveCurrent,
        "loadRawRequest" as "load-raw-request" => load_raw_request::LoadRawRequest,
    }
}

#[path = "⌨️command-from-action/🦀️.rs"]
mod command_from_action;
use command_from_action::*;

#[path = "🧵️retained-command-tools/🦀️.rs"]
mod retained_command_tools;
use retained_command_tools::*;

#[path = "📤️media-export-job/🦀️.rs"]
mod media_export_job;
use media_export_job::*;

#[path = "🔧️config-store-preparation/🦀️.rs"]
mod config_store_preparation;
use config_store_preparation::*;

#[path = "🗄️artifact-store-preparation/🦀️.rs"]
mod artifact_store_preparation;
use artifact_store_preparation::*;

#[path = "🪟️editor-and-interaction-scope/🦀️.rs"]
mod editor_and_interaction_scope;

#[path = "🧭️app-manifest/🦀️.rs"]
mod app_manifest;
pub use app_manifest::*;

//#endregion 🔖️Commands

//#region 🔖️PlayApp
// 📐️ B1/WORKFLOWS-END-TO-END-TYPED-PORTS: unit-struct-shaped pure `ArtifactApp` — every former
// `CadPlayRuntime`/`self.runtime` field lives in `CadConfig` (written through `CadConfigMutation`s with real
// `backwards`) or, for the per-frame engagement state, in the addressed world window's transient (design §17.4).
#[derive(Default, Clone, Copy)]
pub struct CadPlayApp;

impl CadPlayApp {
    /// 🪟️ The one window-chrome implementation both trait entry points share — `interaction` is the
    /// live `"cad"` domain for the request-context twin and empty for the interaction-less one.
    fn window_engagements_body(doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel, transient: &CadWorldWindowTransient, interaction: CadInteractionSnapshot) -> HashMap<String, WindowEngagement> {
        let view = CadPlayView::of(doc.snapshot, &doc.children, runtime_of(cfg, transient), interaction);
        let labels = cad_labels(view_state);
        HashMap::from([
            (shape::WINDOW_KIND_ID.to_string(), shape::engagement(&view, labels)),
            (building::WINDOW_KIND_ID.to_string(), building::engagement(&view, labels)),
            (energy::WINDOW_KIND_ID.to_string(), energy::engagement(&view, labels)),
            (structure_classic::WINDOW_KIND_ID.to_string(), structure_classic::engagement(&view, labels)),
        ])
    }

    /// 🖼️ The one render implementation both trait entry points share.
    fn render_body(body_key: &str, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel, transient: &CadWorldWindowTransient, interaction: CadInteractionSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        semio_s_artifact_cad_cad_inferences::validate_cad_computer_contributions(&cfg.snapshot.contributions_json);
        let view = CadPlayView::of(doc.snapshot, &doc.children, runtime_of(cfg, transient), interaction);
        let labels = cad_labels(view_state);
        let window_kind_id = match body_key {
            shape::BODY_KEY => shape::WINDOW_KIND_ID,
            building::BODY_KEY => building::WINDOW_KIND_ID,
            energy::BODY_KEY => energy::WINDOW_KIND_ID,
            structure_classic::BODY_KEY => structure_classic::WINDOW_KIND_ID,
            _ => shape::WINDOW_KIND_ID,
        };
        let active_utility = view_state.active_utility_id.as_deref();
        let options = view.runtime.dislocate_options(window_kind_id);
        match body_key {
            shape::BODY_KEY => shape::render(&view, active_utility, options).map(semio_framework_plugin::built_to_component_tree),
            building::BODY_KEY => building::render(&view, active_utility, options).map(semio_framework_plugin::built_to_component_tree),
            energy::BODY_KEY => energy::render(&view, active_utility, options).map(semio_framework_plugin::built_to_component_tree),
            structure_classic::BODY_KEY => structure_classic::render(&view, active_utility, options).map(semio_framework_plugin::built_to_component_tree),
            // 🪟️ One `TreeWindows` per rendered body, read from the host's `ViewModel.tree_windows`:
            // it decides which containers are open and which row slice each one materialises.
            document::CAD_PLAY_BODY_ARTIFACT => document::build_document_tree(&view, labels, &TreeWindows::for_body(view_state, document::CAD_PLAY_BODY_ARTIFACT)).map(semio_framework_plugin::built_to_component_tree),
            catalogue::CAD_PLAY_BODY_CATALOGUE => catalogue::build_catalogue_tree(labels, &TreeWindows::for_body(view_state, catalogue::CAD_PLAY_BODY_CATALOGUE)).map(semio_framework_plugin::built_to_component_tree),
            inspection::CAD_PLAY_BODY_PROPERTIES => {
                inspection::build_properties_panel(&view, labels, active_utility, &TreeWindows::for_body(view_state, inspection::CAD_PLAY_BODY_PROPERTIES)).map(semio_framework_plugin::built_to_component_tree)
            }
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

//#endregion 🔖️PlayApp

//#region 🔖️WorkingSceneFixtures
/// 🌲️ The Concrete Forest Left example's REAL per-pane object content, built straight from the
/// same fixture JSON `forest_play_scene()`'s (persisted, handle-only) `CadSnapshot` is built from —
/// see `crate::standards::v1::subsets::any::io::geometry_import::forest_pane_bundle`.
/// This is the app-layer `CadWorkingScene` counterpart to `forest_play_scene()`: use `forest_play_scene()`
/// for `drive`/render dispatch (a `CadSnapshot`, composed-child HANDLES only) and this for reading
/// actual object data in tests/render-path exemplars.
pub fn forest_working_scene() -> CadWorkingScene {
    use semio_s_artifact_cad_cad_play::forest_pane_bundle;
    let (objects, geometry) = forest_pane_bundle(CadPaneId::Shape);
    let (building_objects, building_geometry) = forest_pane_bundle(CadPaneId::Building);
    let (energy_objects, energy_geometry) = forest_pane_bundle(CadPaneId::Energy);
    let (structure_classic_objects, structure_classic_geometry) = forest_pane_bundle(CadPaneId::StructureClassic);
    CadWorkingScene {
        objects,
        geometry: Some(geometry),
        building_objects,
        building_geometry: Some(building_geometry),
        energy_objects,
        energy_geometry: Some(energy_geometry),
        structure_classic_objects,
        structure_classic_geometry: Some(structure_classic_geometry),
    }
}

/// 🟦️ The single-box placeholder scene `default_document()`'s `CadSnapshot` used to inline directly
/// (pre-wave-3) — realized now as the app-layer `CadWorkingScene` counterpart: use `default_document()`
/// for `drive`/render dispatch, this for reading its (one, real) object.
pub fn default_working_scene() -> CadWorkingScene {
    CadWorkingScene { objects: vec![make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape)], ..CadWorkingScene::default() }
}
//#endregion 🔖️WorkingSceneFixtures

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
