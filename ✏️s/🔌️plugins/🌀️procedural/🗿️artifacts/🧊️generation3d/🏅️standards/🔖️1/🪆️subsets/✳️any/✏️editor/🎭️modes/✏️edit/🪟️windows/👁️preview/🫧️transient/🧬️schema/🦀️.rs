//! 🧬️ Generation3d edit-preview window-transient schema leaf.

use semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.generation.3d.window.procedural-preview.transient")]
pub struct Generation3dPreviewWindowTransient {
    #[state(transient)]
    pub preview_eval_text: Option<String>,
}
