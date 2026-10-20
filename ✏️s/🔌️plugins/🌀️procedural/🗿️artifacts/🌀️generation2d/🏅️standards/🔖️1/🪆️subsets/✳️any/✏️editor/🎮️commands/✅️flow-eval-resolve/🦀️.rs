//! 📥️ Original replies execute through the retained command receiver and its initiating caller.
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_s_artifact_procedural_2d_preview_eval::FlowEvalResolve;

/// 🪪️ A direct synchronous dispatch has no original retained source or receiver authority.
pub fn handle(_payload:&FlowEvalResolve,_doc:&ArtifactView<'_,Generation2dSnapshot>,_cfg:&ConfigView<'_,Generation2dConfig>,_session:&mut FlowEvalSession)->Result<Emit<Generation2dMutation,Generation2dConfigMutation>,Fault>{Err(Fault::from("original reply requires its retained command work"))}
