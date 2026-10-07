//! 🏔️ Three handwritten native fields retain exact IEEE state and all five child identity strings.
use crate::standards::v1::subsets::any::schema::snapshot::GisTerrainSnapshot;
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Child{child_id:String,artifact_id:String,artifact_kind:String,standard:String,subset:String}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Terrain{exaggeration:f64,imported_map:Option<crate::schema::ImportedMap>,mesh:Option<Child>}
#[path="🛫️encode/🦀️.rs"]mod controlled_output;
pub(crate) fn producer()->semio_framework_dsl_record::RecordSpecProducer{Terrain::__dsl_spec_producer()}
pub(crate) fn encode_record_controlled(snapshot:&GisTerrainSnapshot,c:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{controlled_output::project(snapshot,c)}
impl Terrain{
 pub(crate) fn from_snapshot(value:&GisTerrainSnapshot)->Self{Self{exaggeration:value.exaggeration,imported_map:value.imported_map.clone(),mesh:value.mesh.as_ref().map(|c|Child{child_id:c.child_id.clone(),artifact_id:c.target.artifact_id.clone(),artifact_kind:c.target.dialect.artifact_kind.clone(),standard:c.target.dialect.standard.clone(),subset:c.target.dialect.subset.clone()})}}
 pub(crate) fn into_snapshot(self)->GisTerrainSnapshot{GisTerrainSnapshot{exaggeration:self.exaggeration,imported_map:self.imported_map,mesh:self.mesh.map(|c|store::ArtifactChild::new(c.child_id,semio_framework_artifact_reference::ArtifactRef{artifact_id:c.artifact_id,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:c.artifact_kind,standard:c.standard,subset:c.subset}}))}}
}

impl store::ArtifactPack for GisTerrainSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Terrain::__dsl_spec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let inner=store::pack_rt::encode_document(&Terrain::__dsl_spec(),&Terrain::from_snapshot(self).__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|e| store::PackError::from(e.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "terrain native binary identity differs")))}let(record,_)=store::pack_rt::decode_document(&inner,&Terrain::__dsl_spec(),options)?;let snapshot=Terrain::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot();if let Some(map)=&snapshot.imported_map{map.validate().map_err(|detail| store::PackError::Refusal(store::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "pack", offset: 0, detail }))?;}Ok(snapshot)}
}

pub(crate) fn reconstruct_record_controlled(source:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>,maximum_rows:usize,maximum_value_bytes:usize)->Result<GisTerrainSnapshot,semio_framework_value::ValueError>{control.checkpoint()?;crate::standards::v1::subsets::any::schema::snapshot::row_admission::native(source,maximum_rows,||control.checkpoint())?;crate::standards::v1::subsets::any::schema::snapshot::value_admission::native(source,maximum_value_bytes,control)?;let snapshot=Terrain::__dsl_from_record_controlled(source,control)?.into_snapshot();if let Some(map)=&snapshot.imported_map{map.validate().map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error))?;}Ok(snapshot)}
