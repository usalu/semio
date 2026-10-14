//! 📥️ Original replies execute through the retained command receiver and its initiating caller.
use crate::editor::generation3d::config::{Generation3dConfig,Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
pub use crate::preview_eval::FlowEvalResolve;
/// 🪪️ A direct synchronous dispatch has no original retained source or receiver authority.
pub fn handle(_payload:&FlowEvalResolve,_doc:&ArtifactView<'_,Generation3dSnapshot>,_cfg:&ConfigView<'_,Generation3dConfig>,_session:&mut FlowEvalSession)->Result<Emit<Generation3dMutation,Generation3dConfigMutation>,Fault>{Err(Fault::from("original reply requires its retained command work"))}
