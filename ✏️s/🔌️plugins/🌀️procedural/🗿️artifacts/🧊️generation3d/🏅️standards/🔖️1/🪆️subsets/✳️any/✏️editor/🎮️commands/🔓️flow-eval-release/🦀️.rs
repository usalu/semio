//! 🔓️ Generation3d play app commands command — `flow-eval-release`: the hop a closed `previewEval` run
//! hands the host to release the kernel work it left in the geometry extension's own instance. The
//! payload shape and the release law live in `🧵️preview-eval`, shared verbatim with `👁️viewer`.

use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};



/// 🔓️ Emits the `evaluateCancel` and `tessellateCancel` invocations; the session was already quiesced
/// by the run's close, so this hop touches it no further.
pub fn handle(payload: &FlowEvalRelease, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit { extension_invocations: preview_eval::release_invocations(payload), ..Default::default() })
}
