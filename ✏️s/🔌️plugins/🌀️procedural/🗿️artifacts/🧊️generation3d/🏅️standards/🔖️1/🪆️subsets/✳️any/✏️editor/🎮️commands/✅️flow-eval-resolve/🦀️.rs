//! 📥️ Original replies execute through the retained command receiver and its initiating caller.
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_s_artifact_procedural_3d_preview_eval::FlowEvalResolve;

/// 🪪️ A direct synchronous dispatch has no original retained source or receiver authority.
pub fn handle(_payload:&FlowEvalResolve,_doc:&ArtifactView<'_,Generation3dSnapshot>,_cfg:&ConfigView<'_,Generation3dConfig>,_session:&mut FlowEvalSession)->Result<Emit<Generation3dMutation,Generation3dConfigMutation>,Fault>{Err(Fault::from("original reply requires its retained command work"))}
