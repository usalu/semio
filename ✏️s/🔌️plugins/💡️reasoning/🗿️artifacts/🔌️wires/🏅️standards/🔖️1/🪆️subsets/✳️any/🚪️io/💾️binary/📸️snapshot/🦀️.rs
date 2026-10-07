//! 🔌️ Native Wires Pack preserves explicit intrinsic nodes and the independent child handle.
pub const COMPONENT_PROTOCOL_SEMIO:&str=include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH:&str=concat!(module_path!(),"::📡️.protocol.semio");
use crate::WiresSnapshot;
pub(crate)fn print_pack_record_text(snapshot:&WiresSnapshot)->String{semio_framework_dsl_record::print(&pack::record(snapshot),&pack::record_spec(),semio_framework_dsl_record::JoinMode::Document)}
pub(crate)fn parse_pack_record_text(body:&str)->Result<WiresSnapshot,semio_framework_diagnostic::TextError>{let record=semio_framework_dsl_record::parse(body,&pack::record_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;pack::reconstruct_record(&record)}
impl store::ArtifactPack for WiresSnapshot{
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let inner=store::pack_rt::encode_document(&pack::record_spec(),&pack::record(self),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error| store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Wires Pack envelope differs")))}let(record,_)=store::pack_rt::decode_document(&inner,&pack::record_spec(),options)?;pack::reconstruct_record(&record).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(pack::record_spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;




#[path="📦️pack/🦀️.rs"]
pub(crate) mod pack;
