//! 🚦️ Literal eight-field Generation native mirrors with cumulative construction controls.
use super::*;
use semio_framework_artifact_flow_flow::{OrderedMap,WidgetLayout,neural::{ColdDictionaryBuilder,ColdOwner,ColdRetire}};
use semio_framework_value::{FromValue,ToValue,ValueError,ValueRefusalKind};
use semio_framework_value::NativeDecodeControl;
use semio_framework_value::NativeEncodeControl;
#[path="🌱️value/🦀️.rs"]mod values;
#[path="📏️rows/🦀️.rs"]mod rows;
fn error(e:ValueError)->semio_framework_diagnostic::TextError{semio_framework_diagnostic::TextError::from_value_error(e,semio_framework_diagnostic::TextSpan::at(1,1))}
fn project_value<T:semio_framework_dsl_record::DslField>(value:&T,c:&mut NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{c.scoped_stage(|c|{c.begin_stage(0)?;value.to_value_controlled(c)})}
fn project_optional_text(value:&Option<String>,c:&mut NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{match value{Some(value)=>project_value(value,c),None=>Ok(semio_framework_dsl_record::FieldValue::Absent)}}
fn dictionary_encode(v:&Dictionary,c:&mut NativeEncodeControl<'_>)->Result<Vec<DictEntryDsl>,ValueError>{c.scoped_depth(64,|c|c.scoped_stage(|c|{c.begin_stage(v.len())?;let mut out=c.allocate_vec(v.len())?;for(k,v)in v.iter(){let key=c.copy_text(k)?;let value=c.scoped_stage(|c|neural_encode(v,c))?;out.push(DictEntryDsl{key,value});c.step()?;}Ok(out)}))}
fn neural_encode(v:&NeuralValue,c:&mut NativeEncodeControl<'_>)->Result<ValueDsl,ValueError>{let mut out=ValueDsl{null:None,boolean:None,integer:None,decimal:None,text:None,dictionary:None};match v{NeuralValue::Atom(Atom::Null)=>out.null=Some(true),NeuralValue::Atom(Atom::Boolean(v))=>out.boolean=Some(*v),NeuralValue::Atom(Atom::Integer(v))=>out.integer=Some(*v),NeuralValue::Atom(Atom::Decimal(v))=>out.decimal=Some(*v),NeuralValue::Atom(Atom::String(v))=>out.text=Some(c.copy_text(v)?),NeuralValue::Dictionary(v)=>out.dictionary=Some(dictionary_encode(v,c)?)}Ok(out)}
fn dictionary_decode(v:Vec<DictEntryDsl>,c:&mut NativeDecodeControl<'_>)->Result<Dictionary,ValueError>{c.scoped_depth(64,|c|c.scoped_stage(|c|{c.begin_stage(v.len())?;let mut out=ColdDictionaryBuilder::new();for entry in v{let value=c.scoped_stage(|c|neural_decode(entry.value,c))?;out.insert_controlled(entry.key,value,c)?;c.step()?;}Ok(out.finish())}))}
fn neural_decode(v:ValueDsl,c:&mut NativeDecodeControl<'_>)->Result<NeuralValue,ValueError>{let present=usize::from(v.null.is_some())+usize::from(v.boolean.is_some())+usize::from(v.integer.is_some())+usize::from(v.decimal.is_some())+usize::from(v.text.is_some())+usize::from(v.dictionary.is_some());if present!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Generation neural value must own exactly one variant"))}if v.null.is_some(){return Ok(NeuralValue::Atom(Atom::Null))}if let Some(v)=v.boolean{return Ok(NeuralValue::Atom(Atom::Boolean(v)))}if let Some(v)=v.integer{return Ok(NeuralValue::Atom(Atom::Integer(v)))}if let Some(v)=v.decimal{return Ok(NeuralValue::Atom(Atom::Decimal(v)))}if let Some(v)=v.text{return Ok(NeuralValue::Atom(Atom::String(v)))}Ok(NeuralValue::Dictionary(dictionary_decode(v.dictionary.unwrap(),c)?))}
fn strings<'a>(items:impl ExactSizeIterator<Item=&'a String>,c:&mut NativeEncodeControl<'_>)->Result<Vec<String>,ValueError>{c.scoped_stage(|c|{c.begin_stage(items.len())?;let mut out=c.allocate_vec(items.len())?;for value in items{out.push(c.copy_text(value)?);c.step()?;}Ok(out)})}
fn widget_encode(v:&Widget,c:&mut NativeEncodeControl<'_>)->Result<WidgetDsl,ValueError>{Ok(match v{
 Widget::Neuron{id,neuron_kind,params,input_ports,output_ports,preview}=>WidgetDsl::Neuron{id:c.copy_text(id)?,neuron_kind:c.copy_text(neuron_kind)?,preview:*preview,input_ports:strings(input_ports.iter(),c)?,output_ports:strings(output_ports.iter(),c)?,params:dictionary_encode(params,c)?},
 Widget::InputSlider{id,label,value,min,max,step}=>WidgetDsl::InputSlider{id:c.copy_text(id)?,label:c.copy_text(label)?,value:*value,min:*min,max:*max,step:*step},
 Widget::InputNote{id,text}=>WidgetDsl::InputNote{id:c.copy_text(id)?,text:c.copy_text(text)?},
 Widget::InputImage{id,src}=>WidgetDsl::InputImage{id:c.copy_text(id)?,src:c.copy_text(src)?},
 Widget::Variable{id,name,schema}=>WidgetDsl::Variable{id:c.copy_text(id)?,name:c.copy_text(name)?,schema:c.copy_text(schema)?},
 Widget::OutputPreview{id,preview,expanded}=>WidgetDsl::OutputPreview{id:c.copy_text(id)?,preview:dictionary_encode(preview,c)?,expanded:strings(expanded.iter(),c)?},
 Widget::OutputAction{id,action}=>WidgetDsl::OutputAction{id:c.copy_text(id)?,action:c.copy_text(action)?},
 Widget::OutputExport{id,format}=>WidgetDsl::OutputExport{id:c.copy_text(id)?,format:c.copy_text(format)?},
 Widget::Cluster{id,name,tree,flow}=>{let id=c.copy_text(id)?;let name=c.copy_text(name)?;let tree=<semio_framework_value::DslValue as FromValue>::guard_decoded(values::tree_encode(tree,c)?);let flow=values::gui_encode(flow,c)?;WidgetDsl::Cluster{id,name,tree:tree.take(),flow}}
})}
fn widget_decode(v:WidgetDsl,c:&mut NativeDecodeControl<'_>)->Result<Widget,ValueError>{Ok(match v{
 WidgetDsl::Neuron{id,neuron_kind,preview,input_ports,output_ports,params}=>Widget::Neuron{id,neuron_kind,preview,input_ports,output_ports,params:dictionary_decode(params,c)?},
 WidgetDsl::InputSlider{id,label,value,min,max,step}=>Widget::InputSlider{id,label,value,min,max,step},
 WidgetDsl::InputNote{id,text}=>Widget::InputNote{id,text},WidgetDsl::InputImage{id,src}=>Widget::InputImage{id,src},WidgetDsl::Variable{id,name,schema}=>Widget::Variable{id,name,schema},
 WidgetDsl::OutputPreview{id,preview,expanded}=>{let preview=ColdOwner::new(dictionary_decode(preview,c)?);let expanded=values::set_decode(&expanded,c)?;Widget::OutputPreview{id,preview:preview.into_inner(),expanded}},
 WidgetDsl::OutputAction{id,action}=>Widget::OutputAction{id,action},WidgetDsl::OutputExport{id,format}=>Widget::OutputExport{id,format},
 WidgetDsl::Cluster{id,name,tree,flow}=>{let tree=<semio_framework_value::DslValue as FromValue>::guard_decoded(tree);let flow=<semio_framework_value::DslValue as FromValue>::guard_decoded(flow);let tree=ColdOwner::new(values::tree_decode(tree.get(),c)?);let flow=values::gui_decode(flow.get(),c)?;Widget::Cluster{id,name,tree:tree.into_inner(),flow}}
})}
fn pack_error(error:store::PackRefusal)->ValueError{error.into_value_error()}
fn encode_record_native(encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,construct:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{let native_control=native_owner.native();
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let Some(prefix)=CONTROLLED_PACK_PREFIX.filter(|_|matches!(encoding,SnapshotEncoding::Binary))else{return store::encode_sqlite_snapshot_record_native(encoding,CONTROLLED_ENVELOPE_ID,ControlledSnapshotDsl::__dsl_spec_producer(),construct,control,native_owner)};
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;
 let body_limit=limits.max_file_bytes.checked_sub(prefix.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation file cannot contain its declared discriminator"))?;
 control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{
  
  let native_before=native_control.owned_bytes();
    let result=native_control.scoped_maximum(match native_before.checked_add(remaining){Some(value)=>value,None=>return(Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow")),0)}, |native| {native.scoped_observer(&mut |event:semio_framework_value::native_encoding::NativeEncodeProgress|checkpoint(event.completed,event.total),|native|{

  let result=(||{
   let spec=ControlledSnapshotDsl::__dsl_spec_producer().encode(native)?;
   let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(construct(native)?);
   let mut options=store::PackEncodeOptions::default();options.limits.max_file_len=body_limit as u64;
   let body=pack::record::encode_document_controlled(&spec,record.as_record(),&options,native).map_err(pack_error)?;
   let length=prefix.len().checked_add(body.len()).filter(|n|*n<=limits.max_file_bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation snapshot output exceeds file byte limit"))?;
   let mut output=native.allocate_vec(length)?;native.begin_stage(length)?;output.extend_from_slice(prefix);native.advance(prefix.len())?;for chunk in body.chunks(65536){output.extend_from_slice(chunk);native.advance(chunk.len())?;}
   Ok(store::io_schema::IoPayload::Binary(output))
  })();result
    })});
    (result,native_control.owned_bytes().saturating_sub(native_before))
 })?
}
fn decode_record_native(construct:impl FnOnce(&semio_framework_dsl_record::RecordValue,&mut NativeDecodeControl<'_>)->Result<ControlledSnapshot,ValueError>,payload:&store::io_schema::IoPayload,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<ControlledSnapshot,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;
 let(store::io_schema::IoPayload::Binary(bytes),Some(prefix))=(payload,CONTROLLED_PACK_PREFIX)else{return store::decode_sqlite_snapshot_record_native(payload,CONTROLLED_ENVELOPE_ID,ControlledSnapshotDsl::__dsl_spec_producer(),|record,output,native,_body|{*output=Some(construct(record,native)?);Ok(())},control,native_owner)};
 let native_control=native_owner.native();
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,bytes.len())?;
 if bytes.len()>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation snapshot input exceeds file byte limit"))}
 let body=bytes.strip_prefix(prefix).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Generation pack discriminator mismatch"))?;
 control.allocation_stage_native(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  
  let native_before=native_control.owned_bytes();
    let result=native_control.scoped_maximum(match native_before.checked_add(remaining){Some(value)=>value,None=>return(Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow")),0)}, |native| {native.scoped_observer(&mut |event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total),|native|{

  let result=(||{let spec=ControlledSnapshotDsl::__dsl_spec_producer().decode(native)?;let(record,_)=pack::record::decode_document_controlled(body,&spec,&store::PackDecodeOptions::default(),native).map_err(pack_error)?;construct(&record,native)})();
  result
    })});
    (result,native_control.owned_bytes().saturating_sub(native_before))
 })?
}
pub(crate) fn encode(document:&ControlledSnapshot,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{
 let maximum=control.limits().max_rows;let maximum_bytes=control.limits().max_value_bytes;
 encode_record_native(encoding,|c|{
  rows::owned(document,maximum,maximum_bytes,c)?;
  let result=(||->Result<semio_framework_dsl_record::RecordValue,ValueError>{
   use semio_framework_dsl_record::DslField;
use semio_framework_dsl_record::DslVariants;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::native_encoding::EncodedRecord;
   let host=&document.host_snapshot;let generation=&document.generation;
   c.begin_stage(8)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(8,c)?;
   record.insert(0,project_value(&host.schema,c)?)?;c.step()?;
   let camera=project_value(&camera_to_dsl(&host.camera),c)?;
   let camera=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(camera,semio_framework_dsl_record::native_encoding::retire_field);c.charge(std::mem::size_of::<FieldValue>())?;
   record.insert(1,semio_framework_dsl_record::FieldValue::Block(Box::new(camera.take())))?;c.step()?;
   let widgets=c.scoped_stage(|c|{c.begin_stage(host.widgets.len())?;
   let mut widgets=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(c.allocate_vec(host.widgets.len())?,|items:Vec<(String,semio_framework_dsl_record::RecordValue)>|for(_,r)in items{semio_framework_dsl_record::native_encoding::retire_field(semio_framework_dsl_record::FieldValue::Record(r))});
   for w in &host.widgets{let mirror=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(c.scoped_stage(|c|widget_encode(w,c))?,|v|semio_framework_dsl_record::DslVariants::retire_decoded_variant(v));let mut mirror=mirror;widgets.as_mut().push(c.scoped_stage(|c|mirror.as_mut().to_named_record_controlled(c))?);c.step()?;}
   c.charge(std::mem::size_of::<FieldValue>())?;Ok::<_,ValueError>(semio_framework_dsl_record::FieldValue::Block(Box::new(semio_framework_dsl_record::FieldValue::Statements(widgets.take()))))})?;record.insert(2,widgets)?;c.step()?;
   let synapses=c.scoped_stage(|c|{c.begin_stage(host.synapses.len())?;
   let mut synapses=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(c.allocate_vec(host.synapses.len())?,|items:Vec<FieldValue>|for v in items{semio_framework_dsl_record::native_encoding::retire_field(v)});
   for s in &host.synapses{let id=c.copy_text(&s.id)?;let from=c.copy_text(&s.from)?;let to=c.copy_text(&s.to)?;let from_port=if s.from_port.is_empty(){None}else{Some(c.copy_text(&s.from_port)?)};let to_port=if s.to_port.is_empty(){None}else{Some(c.copy_text(&s.to_port)?)};let mirror=SynapseSpecDsl{id,wire:semio_framework_dsl_record::Wire(semio_framework_dsl_record::WireValue{from:semio_framework_dsl_record::WireNode{id:from,kind:None,port:from_port},edge:Some((true,semio_framework_dsl_record::WireNode{id:to,kind:None,port:to_port})),edge_label:semio_framework_dsl_record::WireEdgeLabel::default(),properties:semio_framework_value::DslValue::Object(Vec::new())})};synapses.as_mut().push(project_value(&mirror,c)?);c.step()?;}
   Ok::<_,ValueError>(semio_framework_dsl_record::FieldValue::List(synapses.take()))})?;record.insert(3,synapses)?;c.step()?;
   let layout=c.scoped_stage(|c|{c.begin_stage(host.layout.len())?;
   let mut layout=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(c.allocate_vec(host.layout.len())?,|items:Vec<(String,FieldValue)>|for(_,v)in items{semio_framework_dsl_record::native_encoding::retire_field(v)});
   for(k,v)in host.layout.iter(){let key=c.copy_text(k)?;layout.as_mut().push((key,project_value(&layout_to_dsl(v),c)?));c.step()?;}
   Ok::<_,ValueError>(semio_framework_dsl_record::FieldValue::Map(layout.take()))})?;record.insert(4,layout)?;c.step()?;
   record.insert(5,project_optional_text(&generation.selected_generation_id,c)?)?;c.step()?;record.insert(6,project_optional_text(&generation.preview_text,c)?)?;c.step()?;
   let generations=c.scoped_stage(|c|{c.begin_stage(generation.generations.len())?;
   let mut generations=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(c.allocate_vec(generation.generations.len())?,|items:Vec<FieldValue>|for v in items{semio_framework_dsl_record::native_encoding::retire_field(v)});
   for g in &generation.generations{
    let child=c.scoped_stage(|c|{c.begin_stage(3)?;let mut child=semio_framework_dsl_record::native_encoding::EncodedRecord::new(3,c)?;child.insert(0,project_value(&g.id,c)?)?;c.step()?;child.insert(1,project_value(&g.name,c)?)?;c.step()?;child.insert(2,values::playbook_encode(&g.values,c)?)?;c.step()?;Ok::<_,ValueError>(semio_framework_dsl_record::FieldValue::Record(child.take()))})?;
    generations.as_mut().push(child);c.step()?;
   }Ok::<_,ValueError>(semio_framework_dsl_record::FieldValue::List(generations.take()))})?;record.insert(7,generations)?;c.step()?;Ok(record.take())
  })();result
 },control,native_owner)
}
pub(crate) fn decode(payload:&store::io_schema::IoPayload,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<ControlledSnapshot,ValueError>{let maximum=control.limits().max_rows;let maximum_bytes=control.limits().max_value_bytes;decode_record_native(|record,c|{rows::borrowed(record,maximum,maximum_bytes,c)?;let parsed=ControlledSnapshotDsl::__dsl_from_record_controlled(record,c)?;let result=(||->Result<ControlledSnapshot,ValueError>{let mut host=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(FlowHostSnapshot{schema:parsed.schema,camera:camera_from_dsl(&parsed.camera),widgets:c.allocate_vec(parsed.widgets.len())?,synapses:c.allocate_vec(parsed.synapses.len())?,layout:OrderedMap::new()},|v|v.retire_cold());c.begin_stage(parsed.widgets.len())?;for w in parsed.widgets{host.as_mut().widgets.push(c.scoped_stage(|c|widget_decode(w,c))?);c.step()?;}c.begin_stage(parsed.synapses.len())?;for s in parsed.synapses{host.as_mut().synapses.push(synapse_from_dsl(s));c.step()?;}c.begin_stage(parsed.layout.len())?;for(k,v)in parsed.layout{values::insert(&mut host.as_mut().layout,k,WidgetLayout{x:v.x,y:v.y},c)?;c.step()?;}let mut generation=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(GenerationPlayState{selected_generation_id:parsed.selected_generation_id,preview_text:parsed.preview_text,generations:c.allocate_vec(parsed.generations.len())?},|v|semio_framework_artifact_playbook_playbook::GenerationPlayRoot::from(v).retire_cold());c.begin_stage(parsed.generations.len())?;for g in parsed.generations{let values=c.scoped_stage(|c|values::playbook_decode(g.values,c))?;generation.as_mut().generations.push(FormGeneration{id:g.id,name:g.name,values});c.step()?;}Ok(ControlledSnapshot{host_snapshot:host.take(),generation:generation.take().into()})})();result},payload,control,native_owner)}
