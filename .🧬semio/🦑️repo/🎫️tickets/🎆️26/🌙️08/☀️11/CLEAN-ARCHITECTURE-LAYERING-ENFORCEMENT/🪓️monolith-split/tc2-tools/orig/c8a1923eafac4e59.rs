use semio_s_artifact_draw_drawing_style::set_layer_fill_rule::mutation::SetLayerFillRule;
use super::*;
use semio_s_artifact_draw_drawing_structure::create_layer::mutation::CreateLayer;
use semio_s_artifact_draw_drawing_structure::delete_layer::mutation::DeleteLayer;
use semio_s_artifact_draw_drawing_structure::duplicate_layer::mutation::DuplicateLayer;
use semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::RenameLayer;
use semio_s_artifact_draw_drawing_structure::reorder_layer::mutation::ReorderLayer;
use semio_s_artifact_draw_drawing_style::replace_layer_fill::mutation::ReplaceLayerFill;
use semio_s_artifact_draw_drawing_style::replace_layer_stroke::mutation::ReplaceLayerStroke;
use semio_s_artifact_draw_drawing_style::set_layer_blend_mode::mutation::SetLayerBlendMode;
use semio_s_artifact_draw_drawing_transform::set_layer_boolean_operation::mutation::SetLayerBooleanOperation;
use semio_s_artifact_draw_drawing_metadata::set_layer_locked::mutation::SetLayerLocked;
use semio_s_artifact_draw_drawing_style::set_layer_opacity::mutation::SetLayerOpacity;
use semio_s_artifact_draw_drawing_metadata::set_layer_visible::mutation::SetLayerVisible;
use semio_s_artifact_draw_drawing_transform::update_layer_trace_params::mutation::UpdateLayerTraceParams;
use semio_s_artifact_draw_drawing_transform::update_layer_transform::mutation::UpdateLayerTransform;

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

#[path = "🧬️snapshot-laws/🦀️.rs"]
mod snapshot_laws;
use snapshot_laws::*;

#[path = "🔎️lookup-laws/🦀️.rs"]
mod lookup_laws;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
use kit::*;

#[path = "🏊️arena-laws/🦀️.rs"]
mod arena_laws;

#[path = "🧩️candidate-laws/🦀️.rs"]
mod candidate_laws;

#[path = "🔏️digest-laws/🦀️.rs"]
mod digest_laws;

#[path = "🪞️duplicate-laws/🦀️.rs"]
mod duplicate_laws;

#[path = "✏️edit-laws/🦀️.rs"]
mod edit_laws;
