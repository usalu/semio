use crate::{editor::shooting::commands::document, editor::shooting::commands::scene};
use crate::editor::shooting::modes::edit::windows::icon as icon_window;
use crate::editor::shooting::modes::edit::windows::scene as scene_window;
use crate::editor::shooting::terminology::shooting_play_labels;
use semio_framework_test_fixtures::fixture_text;
#[allow(unused_imports)]
use crate::{editor::shooting::commands::asset, editor::shooting::commands::camera, editor::shooting::commands::export, editor::shooting::commands::gumball, editor::shooting::commands::selection, editor::shooting::commands::shot};
#[allow(unused_imports)]
use crate::{editor::shooting::config::ShootingConfig, editor::shooting::config::ShootingConfigMutation};
#[allow(unused_imports)]
use crate::editor::shooting::modes::edit;
#[allow(unused_imports)]
use crate::{editor::shooting::panels::catalogue as catalogue_panel, editor::shooting::panels::document as document_panel, editor::shooting::panels::inspection as inspection_panel};
#[allow(unused_imports)]
use crate::{editor::shooting::presence::ShootingPresence, editor::shooting::presence::ShootingPresenceMutation};
#[allow(unused_imports)]
use crate::editor::shooting::ui_helpers_and_io::*;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
#[allow(unused_imports)]
use crate::{ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_plugin::{retained_command::ArtifactRetainedCommandJob, retained_command::ArtifactRetainedCommandPayload, retained_command::BoundedArtifactCommandWork};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::AppIo;
#[allow(unused_imports)]
use semio_framework_plugin::AppOperationContext;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactEditor;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::DslValue;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::FaultCode;
#[allow(unused_imports)]
use semio_framework_plugin::FaultOrigin;
#[allow(unused_imports)]
use semio_framework_plugin::GranularityDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_framework_plugin::HoverSpec;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::Media;
#[allow(unused_imports)]
use semio_framework_plugin::MediaClass;
#[allow(unused_imports)]
use semio_framework_plugin::MediaError;
#[allow(unused_imports)]
use semio_framework_plugin::MediaForm;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPayload;
#[allow(unused_imports)]
use semio_framework_plugin::MediaType;
#[allow(unused_imports)]
use semio_framework_plugin::MergeMode;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::OsMediaCapability;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMethod;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMode;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionSpec;
#[allow(unused_imports)]
use semio_framework_plugin::UtilityDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagement;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
#[allow(unused_imports)]
use crate::{editor::shooting::commands::asset::add_asset, editor::shooting::commands::asset::import_asset, editor::shooting::commands::asset::import_asset_request, editor::shooting::commands::asset::patch_assets, editor::shooting::commands::asset::set_active_asset};
#[allow(unused_imports)]
use crate::{editor::shooting::commands::camera::load_saved_camera, editor::shooting::commands::camera::save_camera, editor::shooting::commands::camera::set_camera, editor::shooting::commands::camera::set_shot_camera};
#[allow(unused_imports)]
use crate::{editor::shooting::commands::export::export_active_shot, editor::shooting::commands::export::export_all_shots};
#[allow(unused_imports)]
use crate::{editor::shooting::commands::document::import_snapshot_json, editor::shooting::commands::document::reset_snapshot, editor::shooting::commands::document::save_download, editor::shooting::commands::document::set_active_example};
#[allow(unused_imports)]
use crate::{editor::shooting::commands::gumball::rotate_selection, editor::shooting::commands::gumball::scale_selection, editor::shooting::commands::gumball::translate_selection};
#[allow(unused_imports)]
use crate::{editor::shooting::commands::scene::set_ambient_intensity, editor::shooting::commands::scene::set_material_roughness, editor::shooting::commands::scene::set_shadow_enabled, editor::shooting::commands::scene::set_sun_azimuth, editor::shooting::commands::scene::set_sun_elevation, editor::shooting::commands::scene::set_sun_intensity, editor::shooting::commands::scene::toggle_sun};
#[allow(unused_imports)]
use crate::{editor::shooting::commands::selection::set_center_model, editor::shooting::commands::selection::set_shot_selection, editor::shooting::commands::selection::world_pointer_down, editor::shooting::commands::selection::world_pointer_move};
#[allow(unused_imports)]
use crate::{editor::shooting::commands::shot::add_shot, editor::shooting::commands::shot::patch_shots, editor::shooting::commands::shot::set_active_shot, editor::shooting::commands::shot::set_active_shot_format, editor::shooting::commands::shot::set_active_shot_label, editor::shooting::commands::shot::set_active_shot_shape};
use crate::editor::shooting::*;
use crate::editor::shooting::ui_helpers_and_io::*;

#[test]
fn shooting_window_actions_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️window-actions.json")).expect("neutral window vectors");
    let document = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
        let view_state = semio_framework_plugin::ViewModel { locale, ..semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native) };
        let labels = shooting_play_labels(&view_state);
        for (kind, measures) in [("scene", scene_window::window_measures(&document, labels)), ("icon", icon_window::window_measures(&document, labels))] {
            let json = serde_json::to_value(measures).expect("independent window oracle");
            let actions = json
                .as_array()
                .expect("measures")
                .iter()
                .map(|measure| {
                    assert_eq!(measure["onChange"]["controllerId"], vectors["controller"]);
                    measure["onChange"]["action"].clone()
                })
                .collect::<Vec<_>>();
            assert_eq!(serde_json::Value::Array(actions), vectors[kind]);
        }
        let scene = scene_window::engagement(&document, labels);
        let scene_input = scene.input.expect("camera input");
        assert!(scene_input.on_change.is_none() && scene_input.value.is_none(), "typing the camera label publishes nothing: the host holds the line until submit");
        assert_eq!(scene_input.on_submit.expect("camera submit").action, vectors["sceneSubmit"].as_str().expect("scene submit"));
        let icon = icon_window::engagement(&document, labels);
        assert_eq!(icon.input.expect("shot input").on_change.expect("shot change").action, vectors["iconChange"].as_str().expect("icon change"));
    }
}
