/// 🪟️ Editor mode laws: the panel and the generate and edit mode window law wrappers.

#[path = ".."]
mod editor_generation3d_panels_artifact_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_panels::artifact::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_model::dag_host_snapshot_to_workflow;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_panels_inspection_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_panels::inspection::*;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_3d_model::widget_id;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_panels_catalogue_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_panels::catalogue::*;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_PLAY_APP_ID;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_modes_generate_windows_preview_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_mode_generate::windows::preview::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::show_mode_measure;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_camera_json;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_payload;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_selection_json;
use semio_s_artifact_procedural_3d_preview_eval::preview_status_json;
use semio_s_artifact_procedural_3d_preview_eval::preview_window_status_json;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewPayload;
use semio_s_artifact_procedural_3d_preview_eval::PreviewStatusDebug;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_DOMAIN;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_GRANULARITY;
use semio_s_artifact_procedural_3d_model::generation_by_id;
use semio_s_artifact_procedural_3d_model::generation_host_snapshot_for;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_modes_generate_windows_form_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_mode_generate::windows::form::*;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_PLAY_APP_ID;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_modes_generate_windows_generations_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_mode_generate::windows::generations::*;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_PLAY_APP_ID;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_modes_edit_windows_preview_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_camera_json;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_payload;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_selection_json;
use semio_s_artifact_procedural_3d_preview_eval::preview_status_json;
use semio_s_artifact_procedural_3d_preview_eval::preview_window_status_json;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks;
use semio_s_artifact_procedural_3d_preview_eval::PreviewStatusDebug;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_DOMAIN;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_GRANULARITY;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_modes_edit_windows_flow_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_mode_edit::windows::flow::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_CHANNEL;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_DOMAIN;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_GRANULARITY;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_3d_model::dag_host_snapshot_to_workflow;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}
