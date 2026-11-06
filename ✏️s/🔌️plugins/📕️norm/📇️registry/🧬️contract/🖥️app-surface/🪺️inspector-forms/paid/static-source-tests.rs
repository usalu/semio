//! 🧪️ Original caller allocation receipts, denied frontiers and cancellation cover the production AOT source wrapper.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_replication_mutation::MutationSourceProvenance;
use semio_framework_value::{DslValue,RetainedWorkOutcome};
use semio_framework_value::retained_clone::RetainedCloneSourceAdmission;
static COMPILED:CompiledMutationInputDefinitions=CompiledMutationInputDefinitions{
 provenance:MutationSourceProvenance{workspace_token:[1;32],mutation_root:"fixture",owner:"fixture",source_path:"fixture.rs",descriptor_path:"fixture.json",taxonomy_path:"fixture"},
 semantic_kind:"fixture.insert",schema_sha256:[2;32],document_sha256:&[],framework_document_sha256:&[],
 definitions_json:Ok(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-pages/definitions.json")),
 seed_definitions_json:Ok(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-pages/seeds.json")),
};
static ARRAY:CompiledMutationInputDefinitions=CompiledMutationInputDefinitions{
 definitions_json:Ok(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-pages/definitions-array.json")),
 seed_definitions_json:Ok(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-pages/seeds-array.json")),
 ..COMPILED
};
static ABSENT_ARRAY:CompiledMutationInputDefinitions=ARRAY;
static CHOICE:CompiledMutationInputDefinitions=CompiledMutationInputDefinitions{
 definitions_json:Ok(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-pages/definitions-choice.json")),
 seed_definitions_json:Ok(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-pages/seeds-choice.json")),
 ..COMPILED
};
static VARIANT:CompiledMutationInputDefinitions=CompiledMutationInputDefinitions{
 definitions_json:Ok(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-pages/definitions-variant.json")),
 seed_definitions_json:Ok(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-pages/seeds-variant.json")),
 ..COMPILED
};
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:2_000_000,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:4096}}
enum Decoder{Fixture(OriginalJobDecodeOwner),Original(semio_framework_job_native_decode::OriginalNativeDecodeSource)}
struct Owner{caller:OriginalJobFixtureCaller,decoder:Decoder}
impl Owner{
 fn new()->Self{Self::for_operation(913)}
 fn for_operation(operation:u64)->Self{Self{caller:OriginalJobFixtureCaller::admit(OperationId(operation),Generation(1),stock(),||Some(1),u64::MAX,2_000_000),decoder:Decoder::Fixture(OriginalJobDecodeOwner::empty())}}
 fn production(operation:u64)->Self{Self{caller:OriginalJobFixtureCaller::admit(OperationId(operation),Generation(1),stock(),||Some(1),u64::MAX,2_000_000),decoder:Decoder::Original(semio_framework_job_native_decode::OriginalNativeDecodeSource::empty(semio_framework_job_native_decode::OriginalNativeDecodePolicy{maximum_owned_bytes:1<<28}))}}
 fn receive(&mut self,run:impl FnOnce(&mut NativeDecodeControl<'_>)->(Result<(),ValueError>,RetainedCloneProgress,u64)){
  match &mut self.decoder{
   Decoder::Fixture(decoder)=>{self.caller.turn_with_decoder(grant(),decoder,|cx|cx.with_original_work_loan(4096,cx.deadline_us(),|cx|{
    let ((result,receipt,work),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_native_receiving::<_,ValueError>(|control|Ok(run(control))).unwrap());
    assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
    cx.consume_receiving_retained(grant(),receipt)?;cx.consume_fuel(work);result?;Ok(())
   })).unwrap();},
   Decoder::Original(decoder)=>{
    let mut run=Some(run);
    for _ in 0..100_000{
     let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn(grant(),|cx|{
      let result=decoder.receive_step(cx,|control,grant|{let(value,progress,work)=run.take().unwrap()(control);semio_framework_job_native_decode::OriginalNativeDecodeIssued{value,grant,progress,work}})?;
      let Some(result)=result else{return Ok(false)};if let Some(error)=result.receiving_refusal{return Err(error)}result.value?;Ok(true)
     }));
     let receipt=self.caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if result.unwrap(){return}
    }panic!("production original receiving stalled")
   }
  }
 }
 fn close(&mut self,run:impl FnOnce(&mut dyn OriginalRetirementControl)->RetainedWorkOutcome){
  match &mut self.decoder{
   Decoder::Fixture(decoder)=>{self.caller.turn_with_decoder(grant(),decoder,|cx|cx.with_original_retirement(|cx|{
    let ((result,work,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
     let mut work=0;let result=cx.with_native_retirement(|control|{let turn=run(control);work=turn.work;turn.result});
     let receipt=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};(result,work,receipt)
    });
    assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));cx.consume_retained(receipt)?;cx.consume_fuel(work);result?;Ok(())
   })).unwrap();},
   Decoder::Original(decoder)=>{
    let mut run=Some(run);
    for _ in 0..100_000{
     let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{
      let result=decoder.close_producer_step(cx,|control,grant|{let turn=control.with_retirement(grant,|control|Ok(run.take().unwrap()(control))).unwrap();let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};semio_framework_job_native_decode::OriginalNativeDecodeIssued{value:turn.result,grant,progress,work:turn.work}})?;
      let Some(result)=result else{return Ok(false)};if let Some(error)=result.receiving_refusal{return Err(error)}result.value?;Ok(true)
     })));
     let receipt=self.caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if result.unwrap(){return}
    }panic!("production original retirement stalled")
   }
  }
 }
 fn finish(&mut self){
  match &mut self.decoder{
   Decoder::Fixture(decoder)=>{for _ in 0..2_000_000{if decoder.terminal_is_empty(){return}self.caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{
    let demand=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
    semio_framework_job::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())
   })).unwrap()}},
   Decoder::Original(decoder)=>{for _ in 0..100_000{
    if decoder.terminal_is_empty(){return}
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{decoder.close_step(cx)?;Ok(())})));
    let receipt=self.caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));result.unwrap()
   }}
  }panic!("original decoder retirement stalled")
 }
}
fn receiving_error(error:OriginalReceivingPrimary)->ValueError{error.primary.or(error.quote_refusal).or(error.allocation_refusal).unwrap_or_else(||ValueError::invariant("native receiving failure omitted every cause"))}
fn denied(axis:usize)->RetainedCloneGrant{let mut value=grant();match axis{0=>value.maximum_items=0,1=>value.maximum_copy_bytes=0,2=>value.maximum_capacity_bytes=0,3=>value.maximum_release_bytes=0,_=>value.maximum_depth=0}value}
fn prepare<T:NativeReceive<dyn NativeValueSource>+Sync+'static>(owner:&mut Owner,make:impl FnOnce(&NativeDecodeControl<'_>)->Result<Option<(NormInspectorStaticSourcePreparation<T>,RetainedCloneProgress)>,ValueError>)->NormInspectorStaticSourcePreparation<T>{
 let mut output=None;owner.receive(|control|{
  let result=make(control);let receipt=result.as_ref().ok().and_then(|value|value.as_ref()).map_or(Default::default(),|(_,receipt)|*receipt);
  match result{Ok(value)=>{output=value.map(|(value,_)|value);(Ok(()),receipt,1)},Err(error)=>(Err(error),receipt,0)}
 });output.unwrap()
}
fn close<T:NativeReceive<dyn NativeValueSource>+Sync+'static>(owner:&mut Owner,source:&mut NormInspectorStaticSourcePreparation<T>){
 source.begin_close();
 for _ in 0..2_000_000{
  if source.terminal_is_empty(){return}
  let demand=source.close_demands(4096).unwrap();
  for axis in (0..5).filter(|axis|match axis{1=>demand.copy_bytes>0,2=>demand.capacity_bytes>0,3=>demand.release_bytes>0,4=>demand.depth>0,_=>true}){
   owner.close(|control|{let turn=source.close_step(control,denied(axis));assert_eq!(turn.work,0);assert_eq!(turn.result.as_ref().unwrap().progress(),RetainedCloneProgress::default());turn})
  }
  owner.close(|control|source.close_step(control,grant()))
 }panic!("static source retirement stalled")
}
fn run<T:NativeReceive<dyn NativeValueSource>+Sync+serde::Serialize+'static>(seeds:bool,cancel:Option<usize>)->usize{
 let mut owner=Owner::new();
 let mut preparation=prepare(&mut owner,|control|NormInspectorStaticSourcePreparation::<T>::admit(&COMPILED,if seeds{COMPILED.seed_definitions_json}else{COMPILED.definitions_json},JsonReadLimits{maximum_bytes:1<<20,maximum_allocation_bytes:1<<20,maximum_depth:4096,maximum_items:2_000_000},control,grant()));
 let mut count=0;
 while !preparation.ready()&&cancel!=Some(count){
  let demand=preparation.demands(4096).unwrap();
  for axis in (0..5).filter(|axis|match axis{1=>demand.copy_bytes>0,2=>demand.capacity_bytes>0,3=>demand.release_bytes>0,4=>demand.depth>0,_=>true}){
   owner.receive(|control|{let turn=preparation.advance(control,4096,denied(axis));let receipt=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.progress};assert_eq!(receipt,RetainedCloneProgress::default());(turn.result.map(|_|()).map_err(receiving_error),receipt,turn.work)});
  }
  owner.receive(|control|{let turn=preparation.advance(control,4096,grant());let receipt=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.progress};(turn.result.map(|_|()).map_err(receiving_error),receipt,turn.work)});
  count+=1;assert!(count<10_000)
 }
 let mut output=None;
 if cancel.is_none(){
  assert!(preparation.ready());owner.receive(|control|{let turn=preparation.take(control,grant());match turn.result{Ok(Some((value,receipt)))=>{output=Some(value);(Ok(()),receipt,turn.work)},Ok(None)=>panic!("ready output omitted"),Err(error)=>(Err(error),Default::default(),turn.work)}});
  let expected:serde_json::Value=serde_json::from_str(if seeds{COMPILED.seed_definitions_json.unwrap()}else{COMPILED.definitions_json.unwrap()}).unwrap();
  let actual=serde_json::to_value(output.as_ref().unwrap().borrow().get()).unwrap();
  assert_eq!(actual,expected);
 }
 close(&mut owner,&mut preparation);
 if let Some(mut output)=output{
  for _ in 0..2_000_000{if output.terminal_is_empty(){break}owner.close(|_|output.close_step(grant()))}
  assert!(output.terminal_is_empty())
 }
 owner.finish();owner.caller.retire(stock());count
}
#[test]
fn original_static_definitions_have_exact_receipts_and_cancel_at_every_prefix(){
 let count=run::<ActionArgDef>(false,None);for prefix in 0..=count{run::<ActionArgDef>(false,Some(prefix));}
}
#[test]
fn original_static_seeds_have_exact_receipts_and_cancel_at_every_prefix(){
 let count=run::<ActionArgShapeSeed>(true,None);for prefix in 0..=count{run::<ActionArgShapeSeed>(true,Some(prefix));}
}
#[test]
fn source_heap_witness_detects_a_positive_seventy_three_byte_allocation(){
 let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut value=Vec::<u8>::new();value.try_reserve_exact(73).unwrap();value});
 assert_eq!(heap.requested_bytes,73);assert_eq!(heap.released_bytes,0);
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(value));assert_eq!(heap.released_bytes,73);
}

fn read<T:NativeReceive<dyn NativeValueSource>+Sync+'static>(owner:&mut Owner,compiled:&'static CompiledMutationInputDefinitions,seeds:bool)->NormInspectorPreparedStaticSource<T>{
 let mut input=prepare(owner,|control|NormInspectorStaticSourcePreparation::<T>::admit(compiled,if seeds{compiled.seed_definitions_json}else{compiled.definitions_json},JsonReadLimits{maximum_bytes:1<<20,maximum_allocation_bytes:1<<20,maximum_depth:4096,maximum_items:2_000_000},control,grant()));
 for _ in 0..100_000{if input.ready(){break}owner.receive(|control|{let turn=input.advance(control,4096,grant());let receipt=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.progress};(turn.result.map(|_|()).map_err(receiving_error),receipt,turn.work)})}
 let mut output=None;
 owner.receive(|control|{let turn=input.take(control,grant());let Some((value,receipt))=turn.result.unwrap()else{panic!("static source not ready")};output=Some(value);(Ok(()),receipt,turn.work)});
 close(owner,&mut input);output.unwrap()
}
fn page(cancel:Option<usize>,locale:semio_ui_locale::Locale,compiled:&'static CompiledMutationInputDefinitions,offset:usize,handoff:bool)->usize{
 use crate::{NormInspectorPageOriginal,NormInspectorPreparedSemantics,NormInspectorDraftKey,NormInspectorSchemaPath,NormInspectorPagePreparation};
 use semio_framework::action_staging::{StagedFormRequest,StagedFormStatus,StagedFormToken};
 let mut owner=if handoff{Owner::production(913)}else{Owner::new()};
 let descriptors=read::<ActionArgDef>(&mut owner,compiled,false);let seeds=read::<ActionArgShapeSeed>(&mut owner,compiled,true);
 let token=StagedFormToken{action:"insertItem".into(),generation:"1".into(),revision:"r1".into(),window:"real-inputs".into()};
 let args=if std::ptr::eq(compiled,&VARIANT){DslValue::Object(vec![("value".into(),DslValue::Object(vec![("kind".into(),DslValue::String("a".into())),("x".into(),DslValue::String(String::new()))]))])}else if std::ptr::eq(compiled,&ARRAY){DslValue::Object(vec![("value".into(),DslValue::Array((0..35).map(|_|DslValue::String(String::new())).collect()))])}else if std::ptr::eq(compiled,&CHOICE){DslValue::Object(vec![("value".into(),DslValue::String("64".into()))])}else{DslValue::Object(vec![])};
 let request=StagedFormRequest{token:token.clone(),args,locale,page:(offset>0).then(||semio_framework::action_staging::StagedFormPage{field_offset:offset.to_string(),diagnostic_offset:"0".into(),choice:None})};
 let status=StagedFormStatus{token,ready:false,commit_args:None,diagnostics:vec![],diagnostic_offset:"0".into(),diagnostic_total:"0".into()};
 let variants=if std::ptr::eq(compiled,&VARIANT){vec![crate::NormInspectorPreparedVariantGroup{schema_row:1,value_path:vec![0],options:[("a","x"),("b","y")].into_iter().enumerate().map(|(option_index,(kind,key))|crate::NormInspectorPreparedVariant{option_index,value:DslValue::Object(if option_index==0{vec![("kind".into(),DslValue::String(kind.into())),(key.into(),DslValue::String(String::new()))]}else{vec![("kind".into(),DslValue::String(kind.into()))]})}).collect()}]}else{vec![]};
 let original=NormInspectorPageOriginal{descriptors,seeds,request,status,schema_path:NormInspectorSchemaPath::new(&[0]).unwrap(),draft_key:NormInspectorDraftKey::Value,controller:"fixture.inspector".into(),semantics:NormInspectorPreparedSemantics{initial:None,variants}};
 let mut admission=RetainedCloneSourceAdmission::new(original,(),|value|value);
 for _ in 0..64{if admission.phase()=="ready"{break}owner.receive(|control|{let turn=control.receive_retained_work(grant(),admission.advance(grant()));let receipt=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.progress};(turn.result.map(|_|()).map_err(receiving_error),receipt,turn.work)})}
 let mut source=None;
 owner.receive(|control|{let turn=control.receive_retained_work(grant(),admission.take_output(&mut source,grant()));let receipt=turn.result.as_ref().unwrap().progress();(turn.result.map(|_|()).map_err(receiving_error),receipt,turn.work)});
 assert!(admission.terminal_is_empty());
 for axis in [0,1,4]{owner.receive(|control|{
  let result=NormInspectorPagePreparation::admit(&mut source,control,denied(axis));assert!(result.unwrap().is_none());assert!(source.is_some());(Ok(()),Default::default(),0)
 });}
 let mut preparation=None;
 owner.receive(|control|{let result=NormInspectorPagePreparation::admit(&mut source,control,grant());let Some((value,receipt))=result.unwrap()else{panic!("page constructor not admitted")};preparation=Some(value);(Ok(()),receipt,1)});
 let mut preparation=preparation.unwrap();let mut count=0;
 while !preparation.ready()&&cancel!=Some(count){
  for axis in [0,1,4]{owner.receive(|control|{let turn=preparation.advance(control,denied(axis));assert_eq!(turn.work,0);let receipt=turn.result.as_ref().unwrap().progress();assert_eq!(receipt,RetainedCloneProgress::default());(turn.result.map(|_|()),receipt,turn.work)})}
  owner.receive(|control|{let turn=preparation.advance(control,grant());let receipt=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};(turn.result.map(|_|()),receipt,turn.work)});
  count+=1;assert!(count<1000)
 }
 let mut output=None;
 if cancel.is_none(){
  owner.receive(|control|{let turn=preparation.take(control,grant());let Some((value,receipt))=turn.result.unwrap()else{panic!("page not ready")};output=Some(value);(Ok(()),receipt,turn.work)});
  let page=output.as_ref().unwrap();
  if std::ptr::eq(compiled,&ABSENT_ARRAY){let value=page.fixed_value_source(0).unwrap().unwrap();assert!(matches!(value.get(),DslValue::Array(values)if values.len()==1));assert_eq!(value.get(),crate::norm_inspector_singleton_source(page.borrow_original().unwrap(),0).unwrap().get());}
  if std::ptr::eq(compiled,&ARRAY){
   let singleton=crate::norm_inspector_singleton_source(page.borrow_original().unwrap(),0).unwrap();assert!(matches!(singleton.get(),DslValue::Array(values)if values.len()==1&&values[0]==DslValue::String(String::new())));
   assert!(matches!(crate::norm_inspector_seed_source(page.borrow_original().unwrap(),1).unwrap().get(),DslValue::String(_)));
   assert_eq!(page.content_controls(),(71-offset).min(16));assert_eq!(page.previous,(offset>0).then(||offset.saturating_sub(16)));assert_eq!(page.next,(offset+16<71).then_some(offset+16));if offset==64{assert!(matches!(page.fixed_value_source(6).unwrap().unwrap().get(),DslValue::String(value)if value.is_empty()))}}
  else{assert_eq!(page.content_controls(),if std::ptr::eq(compiled,&VARIANT){2}else{1});assert_eq!((page.previous,page.next),(None,None))}
  if std::ptr::eq(compiled,&VARIANT){
   assert_eq!(page.controls[0].unwrap().candidate.variant_group,Some(0));assert_eq!(page.variant_choice_source(0,1).unwrap().unwrap().get(),crate::norm_inspector_variant_source(page.borrow_original().unwrap(),0,1).unwrap().get());
   let original=page.borrow_original().unwrap();let original=original.get();let(_,schema)=original.schema(original.schema_path).unwrap();
   let semio_framework::ArgSchema::Object{fields}=schema else{panic!("variant fixture object")};let selector=semio_framework::action_arg_variant_selector(fields).unwrap();
   for (index,variant) in ["a","b"].into_iter().enumerate(){assert_eq!(original.semantics.variants[0].options[index].value,crate::time_travel_switch_variant(fields,selector,original.request.args.get("value").unwrap(),variant))}
   assert_eq!(crate::norm_inspector_variant_source(page.borrow_original().unwrap(),0,1).unwrap().get().get("kind").and_then(DslValue::as_str),Some("b"));
  }
  if std::ptr::eq(compiled,&CHOICE){
   let choice=page.controls[0].unwrap().choice.unwrap();assert_eq!(choice.len(),32);assert_eq!(choice.resident,Some(64));assert_eq!(choice.index(0),Some(64));
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
    use semio_framework_plugin_decl_staged_form::StagedScalarChoiceSource;
    let choices=page.scalar_choices(0).unwrap().unwrap();let current=choices.choice(0).unwrap();
    (choices.len(),current.key=="64",current.label=="Wert 64",page.scalar_selection(0).unwrap()==Some("64"),choices.choice(32).is_none())
   });
   assert_eq!(result,(32,true,true,true,true));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
  assert_eq!(page.borrow_original().unwrap().get().request.locale,locale);
  assert!(crate::norm_inspector_commit_source(page.borrow_original().unwrap()).is_err());
  if std::ptr::eq(compiled,&COMPILED){assert_eq!(crate::norm_inspector_seed_source(page.borrow_original().unwrap(),0).unwrap().get(),&DslValue::String(String::new()));}
  if std::ptr::eq(compiled,&VARIANT){
   let mut loan=None;
   owner.receive(|_|{
    let turn=crate::norm_inspector_seed_source(page.borrow_original().unwrap(),0).unwrap().retain_owned(grant());
    let(value,receipt)=turn.result.unwrap();loan=Some(value);(Ok(()),receipt,turn.work)
   });
   let mut loan=loan.unwrap();
   assert!(loan.matches_source(crate::norm_inspector_seed_source(page.borrow_original().unwrap(),0).unwrap()));
   assert!(!loan.matches_source(crate::norm_inspector_variant_source(page.borrow_original().unwrap(),0,0).unwrap()));
   for _ in 0..64{if loan.terminal_is_empty(){break}owner.close(|_|loan.close_step(grant()))}
   assert!(loan.terminal_is_empty())
  }
 }
 preparation.begin_close();
 for _ in 0..10_000{if preparation.terminal_is_empty(){break}owner.close(|control|preparation.close_step(control,grant()))}
 assert!(preparation.terminal_is_empty());
 if handoff{
  let singleton=std::ptr::eq(compiled,&ARRAY)||std::ptr::eq(compiled,&ABSENT_ARRAY);let pointer=(if singleton{crate::norm_inspector_singleton_source(output.as_ref().unwrap().borrow_original().unwrap(),0)}else{crate::norm_inspector_variant_source(output.as_ref().unwrap().borrow_original().unwrap(),0,1)}).unwrap().get() as *const DslValue;
  owner.finish();owner.caller.retire(stock());
  next_caller(output.take().unwrap(),pointer,singleton);return count
 }
 if let Some(mut page)=output{for _ in 0..10_000{if page.terminal_is_empty(){break}owner.close(|_|page.close_step(grant()))}assert!(page.terminal_is_empty())}
 owner.finish();owner.caller.retire(stock());count
}
#[test]
fn original_page_retains_exact_seed_authority_and_cancels_at_every_preparation_prefix(){
 for locale in [semio_ui_locale::Locale::En,semio_ui_locale::Locale::De]{let count=page(None,locale,&COMPILED,0,false);for prefix in 0..=count{page(Some(prefix),locale,&COMPILED,0,false);}}
}

#[test]
fn original_array_pages_use_sixteen_content_and_two_navigation_slots(){
 for offset in [0,16,64]{page(None,semio_ui_locale::Locale::En,&ARRAY,offset,false);}
}
#[test]
fn original_large_choice_page_keeps_thirty_one_choices_and_the_entered_resident(){
 page(None,semio_ui_locale::Locale::De,&CHOICE,0,false);
}

#[test]
fn original_variant_page_borrows_aligned_complete_parent_replacements(){
 page(None,semio_ui_locale::Locale::En,&VARIANT,0,false);
}

#[test]
fn original_static_source_constructor_denials_allocate_nothing(){
 let mut owner=Owner::new();
 for axis in [0,1,4]{owner.receive(|control|{
  let result=NormInspectorStaticSourcePreparation::<ActionArgDef>::definitions(&COMPILED,JsonReadLimits{maximum_bytes:1<<20,maximum_allocation_bytes:1<<20,maximum_depth:4096,maximum_items:2_000_000},control,denied(axis));assert!(result.unwrap().is_none());(Ok(()),Default::default(),0)
 });}
 owner.finish();owner.caller.retire(stock());
}

fn retire<T:semio_framework_value::retirement::RetireOwned>(owner:&mut Owner,value:T){
 let mut value=Some(value);let mut retirement=semio_framework_value::retained_clone::RetainedCloneClose::default();
 for _ in 0..100_000{
  if value.is_none()&&retirement.is_empty(){return}
  owner.close(|_|if retirement.is_empty(){let turn=retirement.begin_granted(&mut value,grant());RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}else{retirement.step_granted(grant())})
 }panic!("native donated value retirement stalled")
}
fn next_caller(page:crate::NormInspectorPreparedPage,pointer:*const DslValue,singleton:bool){
 use semio_framework_value_native_owned::native_decoding::owned::tree::NativeReceiveSlot;
 let mut next=Owner::production(914);let mut loan=None;
 next.receive(|_|{
  let turn=(if singleton{crate::norm_inspector_singleton_source(page.borrow_original().unwrap(),0)}else{crate::norm_inspector_variant_source(page.borrow_original().unwrap(),0,1)}).unwrap().retain_owned(grant());
  let(value,receipt)=turn.result.unwrap();loan=Some(value);(Ok(()),receipt,turn.work)
 });
 let mut loan=loan.unwrap();assert_eq!(loan.borrow().unwrap().get() as *const DslValue,pointer);
 let mut receiver=NativeReceiveSlot::<DslValue,DslValue>::default();
 for _ in 0..100_000{
  if receiver.ready(){break}
  next.receive(|control|{
   let turn=receiver.advance(loan.borrow().unwrap(),control,4096,grant());
   let receipt=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.progress};
   (turn.result.map(|_|()).map_err(receiving_error),receipt,turn.work)
  })
 }
 let mut output=None;
 next.receive(|_|{let turn=receiver.take(grant());let Some((value,receipt))=turn.result.unwrap()else{panic!("next caller native output absent")};output=Some(value);(Ok(()),receipt,turn.work)});
 assert_eq!(output.as_ref().unwrap(),loan.borrow().unwrap().get());
 if singleton{assert!(matches!(output.as_ref().unwrap(),DslValue::Array(values)if values.len()==1&&matches!(&values[0],DslValue::String(value)if value.is_empty())));assert!(!loan.matches_source(crate::norm_inspector_seed_source(page.borrow_original().unwrap(),1).unwrap()))}else{assert_eq!(output.as_ref().unwrap().get("kind").and_then(DslValue::as_str),Some("b"))}
 receiver.begin_close();
 for _ in 0..100_000{if receiver.terminal_is_empty(){break}next.close(|_|receiver.close_step(grant()))}
 assert!(receiver.terminal_is_empty());
 for _ in 0..64{if loan.terminal_is_empty(){break}next.close(|_|loan.close_step(grant()))}
 assert!(loan.terminal_is_empty());
 retire(&mut next,output.take().unwrap());
 retire(&mut next,page);next.finish();next.caller.retire(stock());
}
#[test]
fn finalized_original_page_survives_decoder_and_caller_return_and_lends_native_source_to_next_caller(){
 page(None,semio_ui_locale::Locale::En,&VARIANT,0,true);
}

#[test]
fn compiled_singleton_seed_lends_a_real_array_across_complete_original_caller_return(){
 page(None,semio_ui_locale::Locale::De,&ABSENT_ARRAY,0,true);
}
