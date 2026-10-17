pub mod context {
    use crate::{DRAWING_BOUNDED_TOOL_IDS, DRAWING_GESTURE_TOOL_IDS, DRAWING_IMAGE_TOOL_IDS, DRAWING_PATH_TOOL_IDS, DrawingBoundedCommandJobFactory, DrawingCommand, DrawingGestureOperationJobFactory, DrawingPlayApp, args_bridge, create_drawing_app, drawing_active_utility};
    use semio_s_artifact_draw_drawing_editor_interaction::{DRAWING_DEFAULT_UTILITY, DRAWING_INTERACTION_DOMAIN, DRAWING_INTERACTION_GRANULARITY, DRAWING_POINT_DOMAIN, DRAWING_POINT_GRANULARITY, geometry_session};
    use semio_s_artifact_draw_drawing_editor_interaction::panels::catalogue::DRAWING_PLAY_BODY_CATALOGUE;
    use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::{DRAWING_PLAY_BODY_COMPOSITE, DRAWING_PLAY_WINDOW_CANVAS};
    use semio_s_artifact_draw_drawing_editor_interaction::panels::layers::DRAWING_PLAY_BODY_LAYERS;
    use semio_s_artifact_draw_drawing_editor_tool::{DrawingGesturePreview, DrawingSession, canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_move, canvas_pointer_up};
    use crate::image_job::DrawingImageCommandJobFactory;
    use crate::path_job::DrawingPathCommandJobFactory;
    use semio_s_artifact_draw_drawing_editor_commands::{add_layer, combine_boolean, commit_document, delete_layer, delete_selection, drop_layer_kind, duplicate_layer, edit_fill, edit_path, edit_selection, engagement_input, engagement_submit, export_document, import_image, load_document_json, move_layer, patch_layer, patch_layers, set_active_example, set_camera, set_camera_zoom, set_selected_opacity, set_snapshot, toggle_layer_visible};
    use semio_s_artifact_draw_drawing_editor_tool as canvas_pointer_down;
    use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas as canvas_window;
    use semio_s_artifact_draw_drawing_editor_commands::nudge_selection::{nudge_selection_down, nudge_selection_down_fast, nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast};
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel};

    pub type DrawingApp = VcsArtifactApp<EditorApp<DrawingPlayApp>>;

    pub struct DrawingAppFixture(DrawingApp);

    impl std::ops::Deref for DrawingAppFixture {
        type Target = DrawingApp;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for DrawingAppFixture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for DrawingAppFixture {
        fn drop(&mut self) {
            if std::thread::panicking() || self.0.close_terminal_is_empty() {
                return;
            }
            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
        }
    }

    /// ✏️ `DrawingPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<DrawingPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<DrawingPlayApp>` builds it.

    /// 🧪️ Draw fixtures carry the production manifest and its exact registered factories.
    pub async fn drawing_app() -> DrawingAppFixture {
        let mut app = new_app_with_registry::<EditorApp<DrawingPlayApp>>(|| App { definition: create_drawing_app(), examples: Vec::new() }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
        app.bind_instance_id(meta("local").instance_id).await;
        DrawingAppFixture(app)
    }

    semio_framework_plugin::history_edit_acceptance_law!("draw", DrawingPlayApp, || App { definition: create_drawing_app(), examples: Vec::new() }, "../..");

    /// 🧰️ Captures the host-owned active utility in one operation's invocation context.
    pub fn meta_with_utility(utility: &str) -> ActionMeta {
        let mut action_meta = meta("local");
        action_meta.view_state = Some(ViewModel { active_utility_id: Some(utility.into()), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
        action_meta
    }
}

use crate::{DRAWING_BOUNDED_TOOL_IDS, DRAWING_GESTURE_TOOL_IDS, DRAWING_IMAGE_TOOL_IDS, DRAWING_PATH_TOOL_IDS, DrawingBoundedCommandJobFactory, DrawingCommand, DrawingGestureOperationJobFactory, DrawingPlayApp, args_bridge, create_drawing_app, drawing_active_utility};
use semio_s_artifact_draw_drawing_editor_interaction::{DRAWING_DEFAULT_UTILITY, DRAWING_INTERACTION_DOMAIN, DRAWING_INTERACTION_GRANULARITY, DRAWING_POINT_DOMAIN, DRAWING_POINT_GRANULARITY, geometry_session};
use semio_s_artifact_draw_drawing_editor_interaction::panels::catalogue::DRAWING_PLAY_BODY_CATALOGUE;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::{DRAWING_PLAY_BODY_COMPOSITE, DRAWING_PLAY_WINDOW_CANVAS};
use semio_s_artifact_draw_drawing_editor_interaction::panels::layers::DRAWING_PLAY_BODY_LAYERS;
use semio_s_artifact_draw_drawing_editor_tool::{DrawingGesturePreview, DrawingSession, canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_move, canvas_pointer_up};
use crate::image_job::DrawingImageCommandJobFactory;
use crate::path_job::DrawingPathCommandJobFactory;
use semio_s_artifact_draw_drawing_editor_commands::{add_layer, combine_boolean, commit_document, delete_layer, delete_selection, drop_layer_kind, duplicate_layer, edit_fill, edit_path, edit_selection, engagement_input, engagement_submit, export_document, import_image, load_document_json, move_layer, patch_layer, patch_layers, set_active_example, set_camera, set_camera_zoom, set_selected_opacity, set_snapshot, toggle_layer_visible};
use semio_s_artifact_draw_drawing_editor_tool as canvas_pointer_down;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas as canvas_window;
use semio_s_artifact_draw_drawing_editor_commands::nudge_selection::{nudge_selection_down, nudge_selection_down_fast, nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast};
#[path="../../📋️clipboard/🧪️tests/🪟️runtime/🦀️.rs"]
mod clipboard_runtime;
use semio_s_artifact_draw_drawing_model::layer_id;
use semio_s_artifact_draw_drawing_model::default_drawing_document;
use semio_s_artifact_draw_drawing_io::text::snapshot::snapshot_codec::semio_drawing_example_json;
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{artifact_app_laws as artifact_laws, PluginApp, ViewModel, SET_ACTIVE_UTILITY_ACTION_ID};
use context::{drawing_app, meta_with_utility, DrawingApp, DrawingAppFixture};

#[path = "✉️envelope-laws/🦀️.rs"]
mod envelope_laws;
use envelope_laws::*;

#[path = "🖼️canvas-laws/🦀️.rs"]
mod canvas_laws;
use canvas_laws::*;

#[path = "✋️gesture-laws/🦀️.rs"]
mod gesture_laws;
use gesture_laws::*;

#[path = "🧰️utility-laws/🦀️.rs"]
mod utility_laws;
use utility_laws::*;

#[path = "🪟️window-laws/🦀️.rs"]
mod window_laws;

#[path = "👆️pointer-laws/🦀️.rs"]
mod pointer_laws;

#[path = "⌨️command-laws/🦀️.rs"]
mod command_laws;

#[path = "🎛️selection-laws/🦀️.rs"]
mod selection_laws;

#[path = "🔷️node-laws/🦀️.rs"]
mod node_laws;

//#region 🔖️GesturePreview

//#region 🧵️BatchedSamplesAndCancel

//#endregion 🧵️BatchedSamplesAndCancel
//#endregion 🔖️GesturePreview

//#region 🔖️WireGuards

//#endregion 🔖️WireGuards

//#region 🔖️CommandSurface

//#endregion 🔖️CommandSurface
