//! 🎬️ Graph scenes use an owned neutral projection and one declared intrinsic frame.
use super::{NodeGraphError, NodeGraphErrorKind, NodeGraphScenePayload};
use semio_framework_value::{FromValue, NativeDecodeControl, ValueError, ValueRefusalKind};
pub use pack::intrinsic::IntrinsicFormat;
pub use pack::record::DecodeOptions;

/// 🕸️ A graph update can only be constructed by the defining scene decoder.
#[derive(Clone, Debug)]
pub struct GraphScene { pub(super) payload: NodeGraphScenePayload }

fn validated(payload: NodeGraphScenePayload, control: &mut NativeDecodeControl<'_>) -> Result<GraphScene, NodeGraphError> {
 control.begin_stage(payload.nodes.len()).map_err(|error|NodeGraphError::from_cause(NodeGraphErrorKind::Json,error))?;
 for node in &payload.nodes {
  if [node.x,node.y,node.width,node.height].into_iter().flatten().any(|value|!value.is_finite()) {
   return Err(NodeGraphError::from_cause(NodeGraphErrorKind::Json,ValueError::new(ValueRefusalKind::InvalidValue,"graph node coordinates must be finite").under(&node.id)));
  }
  control.step().map_err(|error|NodeGraphError::from_cause(NodeGraphErrorKind::Json,error))?;
 }
 control.checkpoint().map_err(|error|NodeGraphError::from_cause(NodeGraphErrorKind::Json,error))?;
 Ok(GraphScene { payload })
}

/// 📦️ Decodes only the caller-declared physical frame under cumulative caller authority.
pub fn decode(bytes: &[u8], format: IntrinsicFormat, options: &DecodeOptions, control: &mut NativeDecodeControl<'_>) -> Result<GraphScene, NodeGraphError> {
 let value=pack::intrinsic::decode(bytes,format,options,control).map_err(|error|NodeGraphError::from_cause(NodeGraphErrorKind::Pack,error))?;
 let payload=NodeGraphScenePayload::from_value_controlled(&value,control).map_err(|error|NodeGraphError::from_cause(NodeGraphErrorKind::Json,error))?;
 validated(payload,control)
}

/// 🧾️ Parses a closed scene document with duplicate members refused.
pub fn from_json(json: &str, control: &mut NativeDecodeControl<'_>) -> Result<GraphScene, NodeGraphError> {
 let payload=semio_framework_pack_json::from_json_str_controlled(json,semio_framework_pack_json::JsonMemberPolicy::Reject,control).map_err(|error|NodeGraphError::from_cause(NodeGraphErrorKind::Json,error))?;
 validated(payload,control)
}
