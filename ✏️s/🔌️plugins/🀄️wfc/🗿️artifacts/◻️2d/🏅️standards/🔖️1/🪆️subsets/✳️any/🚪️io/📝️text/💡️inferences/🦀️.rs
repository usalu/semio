//! 📝️ Inference request and result codecs.

#[allow(unused_imports)]
mod inference_runtime_codec {
use crate::schema::snapshot::Wfc2dSnapshot;
use std::collections::{BTreeMap, BTreeSet};
use crate::host::inferences::*;
use crate::standards::v1::subsets::any::schema::inferences::*;
}
pub(crate) fn decode_inference_value<T:semio_framework_value::FromValue>(text:&str)->Result<T,String> {semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|error.to_string())}
pub(crate) fn encode_inference_value<T:semio_framework_value::ToValue>(value:&T)->String {semio_framework_pack_json::to_json_string(value)}
