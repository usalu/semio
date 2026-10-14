//! 📥️ Original replies execute through the retained command receiver and its initiating caller.
use crate::editor::generation2d::config::{Generation2dConfig,Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
pub use crate::preview_eval::FlowEvalResolve;
/// 🪪️ A direct synchronous dispatch has no original retained source or receiver authority.
pub fn handle(_payload:&FlowEvalResolve,_doc:&ArtifactView<'_,Generation2dSnapshot>,_cfg:&ConfigView<'_,Generation2dConfig>,_session:&mut FlowEvalSession)->Result<Emit<Generation2dMutation,Generation2dConfigMutation>,Fault>{Err(Fault::from("original reply requires its retained command work"))}
