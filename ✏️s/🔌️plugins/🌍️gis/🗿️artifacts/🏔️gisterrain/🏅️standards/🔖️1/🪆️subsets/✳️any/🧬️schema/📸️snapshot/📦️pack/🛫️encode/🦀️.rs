//! 🏔️ Controlled terrain emission through its actual three-field native record authority.
use super::*;
use semio_framework_dsl_record::DslField;

pub(super) fn producer()->semio_framework_dsl_record::RecordSpecProducer{Terrain::__dsl_spec_producer()}
pub(super) fn project(snapshot:&GisTerrainSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{
 control.begin_stage(3)?;
 let mut fields=semio_framework_dsl_record::native_encoding::EncodedRecord::new(3,control)?;
 fields.insert(0,control.scoped_stage(|control|{control.begin_stage(0)?;snapshot.exaggeration.to_value_controlled(control)})?)?;control.step()?;
 fields.insert(1,control.scoped_stage(|control|{control.begin_stage(0)?;match &snapshot.imported_map{Some(map)=>semio_framework_dsl_record::DslField::to_value_controlled(map,control),None=>Ok(semio_framework_dsl_record::FieldValue::Absent)}})?)?;control.step()?;
 let mesh=control.scoped_stage(|control|->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{if let Some(child)=&snapshot.mesh{
  control.begin_stage(5)?;
  let mut owned=semio_framework_dsl_record::native_encoding::EncodedRecord::new(5,control)?;
  for(id,text)in[(0,&child.child_id),(1,&child.target.artifact_id),(2,&child.target.dialect.artifact_kind),(3,&child.target.dialect.standard),(4,&child.target.dialect.subset)]{
   owned.insert(id,control.scoped_stage(|control|{control.begin_stage(0)?;text.to_value_controlled(control)})?)?;control.step()?;
  }
  Ok(semio_framework_dsl_record::FieldValue::Record(owned.take()))
 }else{Ok(semio_framework_dsl_record::FieldValue::Absent)}})?;
 fields.insert(2,mesh)?;control.step()?;Ok(fields.take())
}
