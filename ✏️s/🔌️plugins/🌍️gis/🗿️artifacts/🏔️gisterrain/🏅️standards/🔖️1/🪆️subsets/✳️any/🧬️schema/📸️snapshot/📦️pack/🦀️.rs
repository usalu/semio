//! 🏔️ Three handwritten native fields retain exact IEEE state and all five child identity strings.
use super::GisTerrainSnapshot;
#[derive(dsl::DslRecord)]
struct Child{child_id:String,artifact_id:String,artifact_kind:String,standard:String,subset:String}
#[derive(dsl::DslRecord)]
#[dsl(extension="gisterrain")]
struct Terrain{exaggeration:f64,imported_features_json:String,mesh:Option<Child>}
impl Terrain{
 fn from_snapshot(value:&GisTerrainSnapshot)->Self{Self{exaggeration:value.exaggeration,imported_features_json:value.imported_features_json.clone(),mesh:value.mesh.as_ref().map(|c|Child{child_id:c.child_id.clone(),artifact_id:c.target.artifact_id.clone(),artifact_kind:c.target.dialect.artifact_kind.clone(),standard:c.target.dialect.standard.clone(),subset:c.target.dialect.subset.clone()})}}
 fn into_snapshot(self)->GisTerrainSnapshot{GisTerrainSnapshot{exaggeration:self.exaggeration,imported_features_json:self.imported_features_json,mesh:self.mesh.map(|c|store::ArtifactChild::new(c.child_id,store::os_io::ArtifactRef{artifact_id:c.artifact_id,dialect:store::os_io::ArtifactDialect{artifact_kind:c.artifact_kind,standard:c.standard,subset:c.subset}}))}}
}
impl store::ArtifactDsl for GisTerrainSnapshot{
 const EXTENSION:&'static str="gisterrain";
 fn envelope_id()->&'static str{"gis.gisterrain"}
 fn print_dsl(&self)->String{let body=dsl::print(&Terrain::from_snapshot(self).__dsl_to_record(),&Terrain::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("terrain envelope");store::semio_format::wrap_text(&envelope,&body)}
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|e|dsl::__rt::field_error(e.to_string()))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(dsl::__rt::field_error("terrain native text identity differs"))}let record=dsl::parse_exact(body,&Terrain::__dsl_spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Document})?;Ok(Terrain::__dsl_from_record(&record)?.into_snapshot())}
}
impl store::ArtifactPack for GisTerrainSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn record_spec()->Option<dsl::RecordSpec>{Some(Terrain::__dsl_spec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let inner=store::pack_rt::encode_document(&Terrain::__dsl_spec(),&Terrain::from_snapshot(self).__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|e|store::PackError::Schema(e.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|e|store::PackError::Schema(e.to_string()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("terrain native binary identity differs".into()))}let(record,_)=store::pack_rt::decode_document(&inner,&Terrain::__dsl_spec(),options)?;Ok(Terrain::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot())}
}

pub(super) fn reconstruct_record_controlled(source:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>,maximum_rows:usize)->Result<GisTerrainSnapshot,String>{control.checkpoint()?;let rows=match source.fields.get(&2){None|Some(dsl::FieldValue::Absent)=>2,Some(dsl::FieldValue::Record(_))=>3,_=>return Err("terrain optional child record differs".into())};if rows>maximum_rows{return Err("terrain native row limit exceeded".into())}Ok(Terrain::__dsl_from_record_controlled(source,control).map_err(|e|e.message)?.into_snapshot())}
