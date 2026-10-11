//! 🧩️ Literal Layout child metadata and guarded inline Drawing field ownership.
use crate::{dsl,store,LayoutDrawingChild,SemioDrawingSnapshot};
use semio_framework_value::{DecodedValue,FromValue,ToValue,ValueError,ValueRefusalKind};

pub fn spec<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(2)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(2)?;
        fields.push(semio_framework_dsl_record::producer::field(0,"handle",<store::ArtifactChild<SemioDrawingSnapshot> as semio_framework_dsl_record::DslField>::shape_controlled(control)?,control)?);control.step()?;
        fields.push(semio_framework_dsl_record::producer::field(1,"content",semio_framework_dsl_record::Shape::Value,control)?);control.step()?;
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}
pub fn producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:crate::layout_drawing_child_spec,decoding:|control|spec(control),encoding:|control|spec(control)}}
pub fn decode(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<LayoutDrawingChild,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(2)?;
        let semio_framework_dsl_record::FieldValue::Record(record)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected inline Drawing record"))};
        let handle=<store::ArtifactChild<SemioDrawingSnapshot> as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(0).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing Drawing handle"))?,control)?;control.step()?;
        let Some(semio_framework_dsl_record::FieldValue::Value(content))=record.get(1) else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected inline Drawing content"))};
        let content=DecodedValue::new(<SemioDrawingSnapshot as FromValue>::from_value_controlled(content,control)?,retire_drawing_content);
        control.step()?;Ok(LayoutDrawingChild{handle,content:content.take()})
    })
}
pub fn encode(value:&LayoutDrawingChild,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(2)?;let mut output=semio_framework_dsl_record::native_encoding::EncodedRecord::new(2,control)?;
        output.insert(0,<store::ArtifactChild<SemioDrawingSnapshot> as semio_framework_dsl_record::DslField>::to_value_controlled(&value.handle,control)?)?;control.step()?;
        output.insert(1,semio_framework_dsl_record::FieldValue::Value(<SemioDrawingSnapshot as ToValue>::to_value_controlled(&value.content,control)?))?;control.step()?;
        Ok(semio_framework_dsl_record::FieldValue::Record(output.take()))
    })
}
pub fn retire(value:LayoutDrawingChild){retire_drawing_content(value.content);}

fn retire_drawing_content(value: SemioDrawingSnapshot) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|(error, _)| panic!("Layout drawing cold retirement refused: {error}"));
    while !owner.terminal_is_empty() {
        let copy = owner.next_copy_byte_demand().expect("Layout drawing cold copy demand");
        let release = owner.next_release_byte_demand().expect("Layout drawing cold release demand");
        let capacity = owner.next_capacity_byte_demand(if copy == 0 { release } else { copy }).expect("Layout drawing cold capacity demand");
        let depth = owner.next_depth_demand().expect("Layout drawing cold depth demand");
        owner.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth }).expect("Layout drawing cold grant");
    }
}
