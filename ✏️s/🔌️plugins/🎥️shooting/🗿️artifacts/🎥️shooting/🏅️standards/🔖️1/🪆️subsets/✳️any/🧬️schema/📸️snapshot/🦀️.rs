//! 🎥️ Persisted Shooting fields and literal composed emblem ownership.
use crate::{ShootingAsset,ShootingEmblemChild,ShootingSavedCamera,ShootingSceneLighting,ShootingShot,SHOOTING_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
/// 📸️ Complete Shooting document snapshot, with ordered records and optional child reference.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all="camelCase")]
#[artifact_schema(id="s.shooting.shooting")]
#[dsl(extension="shooting")]
#[dsl(layout="lines")]
pub struct ShootingSnapshot{
 #[state(artifact)]
 pub schema:String,
 #[state(artifact)]
 #[value(default)]
 #[dsl(table)]
 pub assets:Vec<ShootingAsset>,
 #[state(artifact)]
 #[value(default)]
 #[dsl(table)]
 pub saved_cameras:Vec<ShootingSavedCamera>,
 #[state(artifact)]
 #[value(default)]
 #[dsl(block)]
 pub scene:ShootingSceneLighting,
 #[state(artifact)]
 #[value(default)]
 #[dsl(table)]
 pub shots:Vec<ShootingShot>,
 #[state(artifact)]
 #[value(default)]
 pub active_shot_id:String,
 #[state(artifact)]
 #[value(default)]
 pub active_asset_id:String,
 #[state(artifact)]
 #[child(kind="s.stdio.semio")]
 #[value(default,skip_serializing_if="Option::is_none")]
 pub emblem:Option<ShootingEmblemChild>,
}
impl Default for ShootingSnapshot{
 fn default()->Self{Self{schema:SHOOTING_DOCUMENT_SCHEMA.into(),assets:Vec::new(),saved_cameras:Vec::new(),scene:ShootingSceneLighting::default(),shots:Vec::new(),active_shot_id:String::new(),active_asset_id:String::new(),emblem:None}}
}
impl store::ArtifactDsl for ShootingSnapshot{
 const EXTENSION:&'static str="shooting";
 fn envelope_id()->&'static str{"shooting.shooting"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
  let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|e|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
  if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Shooting native text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}
  let record=semio_framework_dsl_record::parse(body,&Self::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;Self::__dsl_from_record(&record)
 }
 fn print_dsl(&self)->String{
  let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("valid Shooting envelope");store::semio_format::wrap_text(&envelope,&body)
 }
}
impl store::ArtifactPack for ShootingSnapshot{
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
  let inner=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|e|store::PackError::from(e.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))
 }
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{
  let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|e|store::PackError::from(e.into_value_error()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Shooting native pack envelope mismatch")))}
  let(record,_)=store::pack_rt::decode_document(&inner,&Self::__dsl_spec(),options)?;Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
 }
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
}
#[path="🪶️sqlite/🦀️.rs"]
mod sqlite;
#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;
