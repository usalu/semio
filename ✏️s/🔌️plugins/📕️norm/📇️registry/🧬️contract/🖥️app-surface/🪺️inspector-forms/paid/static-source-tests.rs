//! 🧪️ Original caller allocation receipts, denied frontiers and cancellation cover the production AOT source wrapper.
use super::*;
use semio_framework_replication_mutation::MutationSourceProvenance;
use semio_framework_value::{DslValue,OriginalRetirementFailure};
use semio_framework_value::retained_clone::{RetainedCloneRef,RetainedOwnedProjection,RetainedOwnedProjectionAdmission};
use semio_framework_value_native_owned::native_decoding::{NativeReceiver,NativeReceivingFailure as Failure};
use semio_framework_value_testing_admission::{EffectBank,session_driven,session_nested,session_open};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[path="../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
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
const BODY:usize=4096;
const MAXIMUM:u64=1<<20;
type Prep<T>=NormInspectorStaticSourcePreparation<T>;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn exact(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn denied(axis:usize)->RetainedCloneGrant{let mut value=grant();match axis{0=>value.maximum_items=0,1=>value.maximum_copy_bytes=0,2=>value.maximum_capacity_bytes=0,3=>value.maximum_release_bytes=0,_=>value.maximum_depth=0}value}
fn denied_from(base:RetainedCloneGrant,axis:usize)->RetainedCloneGrant{let mut value=base;match axis{0=>value.maximum_items=0,1=>value.maximum_copy_bytes=value.maximum_copy_bytes.saturating_sub(1),2=>value.maximum_capacity_bytes=value.maximum_capacity_bytes.saturating_sub(1),3=>value.maximum_release_bytes=value.maximum_release_bytes.saturating_sub(1),_=>value.maximum_depth=value.maximum_depth.saturating_sub(1)}value}
fn limits()->JsonReadLimits{JsonReadLimits{maximum_bytes:1<<20,maximum_allocation_bytes:1<<20,maximum_depth:4096,maximum_items:2_000_000}}
fn failure(error:Failure)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,format!("{error:?}"))}
fn axes(demand:RetirementDemand)->impl Iterator<Item=usize>{(0..5).filter(move|axis|match axis{1=>demand.copy_bytes>0,2=>demand.capacity_bytes>0,3=>demand.release_bytes>0,4=>demand.depth>0,_=>true})}
fn quiet(turn:&RetainedWorkOutcome<RetainedCloneStep,Failure>){assert_eq!(turn.work,0);assert_eq!(turn.result.as_ref().unwrap_or_else(|error|panic!("{error:?}")).progress(),RetainedCloneProgress::default());}
struct Owner{bank:EffectBank}
impl Owner{
 fn new()->Self{let bank=driver::bank();session_open(&bank);Self{bank}}
 fn settle<E:std::fmt::Debug>(&mut self,granted:RetainedCloneGrant,heap:(usize,usize),poll:RetainedWorkPoll<RetainedCloneStep,E>){
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("synchronous turn stayed pending")};
  self.bank.post(turn.result.unwrap_or_else(|error|panic!("{error:?}")).progress(),granted,heap);
 }
 fn receive(&mut self,run:impl FnOnce(&mut NativeDecodeControl<'_>)->(Result<(),ValueError>,RetainedCloneProgress,u128)){self.receive_under(grant(),run)}
 fn receive_under(&mut self,granted:RetainedCloneGrant,run:impl FnOnce(&mut NativeDecodeControl<'_>)->(Result<(),ValueError>,RetainedCloneProgress,u128)){
  let mut done=None;
  let(poll,heap)=observe(||session_nested(&self.bank,granted,|control,_|{let(result,receipt,work)=run(control);done=Some((result,receipt));RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<(),ValueError>(()),work})}));
  let(result,receipt)=done.expect("receiving turn ran");assert!(matches!(poll,RetainedWorkPoll::Complete(_)));
  result.unwrap();self.bank.post(receipt,granted,heap);
 }
 fn close(&mut self,quote:RetirementDemand,work:u64,denial:Option<RetainedCloneGrant>,run:impl FnOnce(&mut dyn OriginalRetirementControl,&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>{
  let granted=denial.unwrap_or_else(||exact(quote));let mut done=None;
  let(_,heap)=observe(||session_nested(&self.bank,granted,|control,_|{
   let polled=match denial{
    Some(denied)=>control.with_effect(denied,work,|control,effect|control.with_retirement::<_,ValueError>(grant(),|retirement|Ok(run(retirement,effect)))),
    None=>control.with_allocated_effect(quote,work,|control,effect|control.with_retirement::<_,ValueError>(granted,|retirement|Ok(run(retirement,effect)))),
   };
   done=Some(polled);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<(),ValueError>(()),work:0})
  }));
  match done.expect("close turn ran").unwrap(){
   None=>{self.bank.post(Default::default(),granted,heap);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}},
   Some(polled)=>match polled.unwrap(){
    RetainedWorkPoll::Pending{..}=>panic!("synchronous close stayed pending"),
    RetainedWorkPoll::Complete(turn)=>{self.bank.post(match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},granted,heap);turn}
   },
  }
 }
}
fn prepare<T:NativeReceive<dyn NativeValueSource>+Sync+'static>(owner:&mut Owner,make:impl FnOnce(&NativeDecodeControl<'_>)->Result<Option<(Prep<T>,RetainedCloneProgress)>,ValueError>)->Prep<T>{
 let mut output=None;owner.receive(|control|{
  let result=make(control);let receipt=result.as_ref().ok().and_then(|value|value.as_ref()).map_or(Default::default(),|(_,receipt)|*receipt);
  match result{Ok(value)=>{output=value.map(|(value,_)|value);(Ok(()),receipt,1)},Err(error)=>(Err(error),receipt,0)}
 });output.unwrap()
}
fn advance<T:NativeReceive<dyn NativeValueSource>+Sync+'static>(owner:&mut Owner,preparation:&mut Prep<T>){
 let demand=preparation.demands(BODY,MAXIMUM).unwrap();
 let funded=exact(demand);
 for axis in axes(demand){let denial=denied_from(funded,axis);owner.receive_under(denial,|control|{let turn=preparation.advance(control,BODY,MAXIMUM,denial);quiet(&turn);(Ok(()),RetainedCloneProgress::default(),turn.work)})}
 owner.receive_under(funded,|control|{let turn=preparation.advance(control,BODY,MAXIMUM,funded);let receipt=turn.result.as_ref().map(|step|step.progress()).unwrap_or_default();(turn.result.map(|_|()).map_err(failure),receipt,turn.work)});
}
fn close<T:NativeReceive<dyn NativeValueSource>+Sync+'static>(owner:&mut Owner,source:&mut Prep<T>){
 source.begin_close();
 for _ in 0..2_000_000{
  if source.terminal_is_empty(){return}
  let body=BODY.max(source.close_demands(BODY).unwrap().copy_bytes);let demand=source.close_demands(body).unwrap();let work=source.close_work_demand(body).unwrap().max(1);
  for axis in axes(demand){let turn=owner.close(demand,work,Some(denied(axis)),|control,effect|source.close_step(control,effect));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default())}
  owner.close(RetirementDemand{copy_bytes:body,..demand},work,None,|control,effect|source.close_step(control,effect)).result.unwrap_or_else(|error|panic!("{error:?}"));
 }panic!("static source retirement stalled")
}
fn run<T:NativeReceive<dyn NativeValueSource>+Sync+serde::Serialize+'static>(seeds:bool,cancel:Option<usize>)->usize{
 let mut owner=Owner::new();
 let mut preparation=prepare(&mut owner,|control|Prep::<T>::admit(&COMPILED,if seeds{COMPILED.seed_definitions_json}else{COMPILED.definitions_json},limits(),control,grant()));
 let mut count=0;
 while !preparation.ready()&&cancel!=Some(count){advance(&mut owner,&mut preparation);count+=1;assert!(count<10_000)}
 let mut output=None;
 if cancel.is_none(){
  assert!(preparation.ready());owner.receive(|control|{let turn=preparation.take(control,grant());match turn.result{Ok(Some((value,receipt)))=>{output=Some(value);(Ok(()),receipt,turn.work)},Ok(None)=>panic!("ready output omitted"),Err(error)=>(Err(error),Default::default(),turn.work)}});
  let expected:serde_json::Value=serde_json::from_str(if seeds{COMPILED.seed_definitions_json.unwrap()}else{COMPILED.definitions_json.unwrap()}).unwrap();
  let actual=serde_json::to_value(output.as_ref().unwrap().borrow().get()).unwrap();
  assert_eq!(actual,expected);
 }
 close(&mut owner,&mut preparation);
 if let Some(output)=output{driver::retire(output,&mut owner.bank);}
 count
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
 let mut input=prepare(owner,|control|Prep::<T>::admit(compiled,if seeds{compiled.seed_definitions_json}else{compiled.definitions_json},limits(),control,grant()));
 for _ in 0..100_000{if input.ready(){break}advance(owner,&mut input)}
 let mut output=None;
 owner.receive(|control|{let turn=input.take(control,grant());let Some((value,receipt))=turn.result.unwrap()else{panic!("static source not ready")};output=Some(value);(Ok(()),receipt,turn.work)});
 close(owner,&mut input);output.unwrap()
}
fn retain<T:?Sized+Sync+'static>(owner:&mut Owner,source:RetainedCloneRef<'_,T>)->RetainedOwnedProjection<T>{
 let mut admission=RetainedOwnedProjectionAdmission::<T>::default();
 let demand=admission.demand(source).unwrap();let work=admission.work_demand(source).unwrap();let granted=exact(demand);
 let((poll,_),heap)=observe(||session_driven::<ValueError>(&mut owner.bank,granted,work.max(1),|effect,reply,recover|if recover{admission.recover_admit(reply)}else{admission.admit(source,effect,reply)}));owner.settle(granted,heap,poll);
 let demand=admission.take_demand().unwrap();let work=admission.take_work_demand().unwrap();let granted=exact(demand);let mut output=None;
 let((poll,_),heap)=observe(||session_driven::<ValueError>(&mut owner.bank,granted,work.max(1),|effect,reply,recover|if recover{admission.recover_take(reply)}else{admission.take(&mut output,effect,reply)}));owner.settle(granted,heap,poll);
 assert!(admission.terminal_is_empty());output.expect("retained owned projection")
}
fn release<T:?Sized+Sync+'static>(owner:&mut Owner,mut loan:RetainedOwnedProjection<T>){
 for _ in 0..64{
  if loan.terminal_is_empty(){return}
  let demand=loan.close_demand(BODY).unwrap();let work=loan.close_work_demand(BODY).unwrap();let granted=RetainedCloneGrant{maximum_copy_bytes:BODY.max(demand.copy_bytes),..exact(demand)};
  let((poll,_),heap)=observe(||session_driven::<OriginalRetirementFailure>(&mut owner.bank,granted,work.max(1),|effect,reply,recover|if recover{loan.recover_close_into(BODY,effect,reply)}else{loan.close_into(BODY,effect,reply)}));owner.settle(granted,heap,poll);
 }panic!("owned projection retirement stalled")
}

fn close_page(owner:&mut Owner,preparation:&mut crate::NormInspectorPagePreparation){
 preparation.begin_close();
 for _ in 0..10_000{
  if preparation.terminal_is_empty(){return}
  let body=BODY.max(preparation.close_demands(BODY).unwrap().copy_bytes);let demand=preparation.close_demands(body).unwrap();let work=preparation.close_work_demand(body).unwrap().max(1);
  for axis in axes(demand){let turn=owner.close(demand,work,Some(denied(axis)),|control,effect|preparation.close_step(control,effect));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default())}
  owner.close(RetirementDemand{copy_bytes:body,..demand},work,None,|control,effect|preparation.close_step(control,effect)).result.unwrap_or_else(|error|panic!("{error:?}"));
 }panic!("page preparation retirement stalled")
}
fn page(cancel:Option<usize>,locale:semio_ui_locale::Locale,compiled:&'static CompiledMutationInputDefinitions,offset:usize,handoff:bool)->usize{
 use crate::{NormInspectorPageOriginal,NormInspectorPreparedSemantics,NormInspectorDraftKey,NormInspectorSchemaPath,NormInspectorPagePreparation};
 use semio_framework::action_staging::{StagedFormRequest,StagedFormStatus,StagedFormToken};
 let mut owner=Owner::new();
 let descriptors=read::<ActionArgDef>(&mut owner,compiled,false);let seeds=read::<ActionArgShapeSeed>(&mut owner,compiled,true);
 let token=StagedFormToken{action:"insertItem".into(),generation:"1".into(),revision:"r1".into(),window:"real-inputs".into()};
 let args=if std::ptr::eq(compiled,&VARIANT){DslValue::Object(vec![("value".into(),DslValue::Object(vec![("kind".into(),DslValue::String("a".into())),("x".into(),DslValue::String(String::new()))]))])}else if std::ptr::eq(compiled,&ARRAY){DslValue::Object(vec![("value".into(),DslValue::Array((0..35).map(|_|DslValue::String(String::new())).collect()))])}else if std::ptr::eq(compiled,&CHOICE){DslValue::Object(vec![("value".into(),DslValue::String("64".into()))])}else{DslValue::Object(vec![])};
 let request=StagedFormRequest{token:token.clone(),args,locale,page:(offset>0).then(||semio_framework::action_staging::StagedFormPage{field_offset:offset.to_string(),diagnostic_offset:"0".into(),choice:None})};
 let status=StagedFormStatus{token,ready:false,commit_args:None,diagnostics:vec![],diagnostic_offset:"0".into(),diagnostic_total:"0".into()};
 let variants=if std::ptr::eq(compiled,&VARIANT){vec![crate::NormInspectorPreparedVariantGroup{schema_row:1,value_path:vec![0],options:[("a","x"),("b","y")].into_iter().enumerate().map(|(option_index,(kind,key))|crate::NormInspectorPreparedVariant{option_index,value:DslValue::Object(if option_index==0{vec![("kind".into(),DslValue::String(kind.into())),(key.into(),DslValue::String(String::new()))]}else{vec![("kind".into(),DslValue::String(kind.into()))]})}).collect()}]}else{vec![]};
 let original=NormInspectorPageOriginal{descriptors,seeds,request,status,schema_path:NormInspectorSchemaPath::new(&[0]).unwrap(),draft_key:NormInspectorDraftKey::Value,controller:"fixture.inspector".into(),semantics:NormInspectorPreparedSemantics{initial:None,variants}};
 let mut source=Some(driver::admit(&mut owner.bank,original));
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
    use semio_plugin_decl_staged_form::StagedScalarChoiceSource;
    let choices=page.scalar_choices(0).unwrap().unwrap();let current=choices.choice(0).unwrap();
    (choices.len(),current.key=="64",current.label=="Wert 64",page.scalar_selection(0).unwrap()==Some("64"),choices.choice(32).is_none())
   });
   assert_eq!(result,(32,true,true,true,true));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
  assert_eq!(page.borrow_original().unwrap().get().request.locale,locale);
  assert!(crate::norm_inspector_commit_source(page.borrow_original().unwrap()).is_err());
  if std::ptr::eq(compiled,&COMPILED){assert_eq!(crate::norm_inspector_seed_source(page.borrow_original().unwrap(),0).unwrap().get(),&DslValue::String(String::new()));}
  if std::ptr::eq(compiled,&VARIANT){
   let loan=retain(&mut owner,crate::norm_inspector_seed_source(page.borrow_original().unwrap(),0).unwrap());
   assert!(loan.matches_source(crate::norm_inspector_seed_source(page.borrow_original().unwrap(),0).unwrap()));
   assert!(!loan.matches_source(crate::norm_inspector_variant_source(page.borrow_original().unwrap(),0,0).unwrap()));
   release(&mut owner,loan)
  }
 }
 close_page(&mut owner,&mut preparation);
 if handoff{
  let singleton=std::ptr::eq(compiled,&ARRAY)||std::ptr::eq(compiled,&ABSENT_ARRAY);let pointer=(if singleton{crate::norm_inspector_singleton_source(output.as_ref().unwrap().borrow_original().unwrap(),0)}else{crate::norm_inspector_variant_source(output.as_ref().unwrap().borrow_original().unwrap(),0,1)}).unwrap().get() as *const DslValue;
  drop(owner);
  next_caller(output.take().unwrap(),pointer,singleton);return count
 }
 if let Some(page)=output{driver::retire(page,&mut owner.bank);}
 count
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
  let result=NormInspectorStaticSourcePreparation::<ActionArgDef>::definitions(&COMPILED,limits(),control,denied(axis));assert!(result.unwrap().is_none());(Ok(()),Default::default(),0)
 });}
}

fn next_caller(page:crate::NormInspectorPreparedPage,pointer:*const DslValue,singleton:bool){
 let mut next=Owner::new();
 let loan=retain(&mut next,if singleton{crate::norm_inspector_singleton_source(page.borrow_original().unwrap(),0)}else{crate::norm_inspector_variant_source(page.borrow_original().unwrap(),0,1)}.unwrap());
 assert_eq!(loan.borrow().unwrap().get() as *const DslValue,pointer);
 let mut receiver=<DslValue as NativeReceive<DslValue>>::receiver();
 let output=driver::receive_cut(&mut receiver,loan.borrow().unwrap(),&mut next.bank,BODY,None).unwrap().expect("next caller native output");
 assert_eq!(&output,loan.borrow().unwrap().get());
 if singleton{assert!(matches!(&output,DslValue::Array(values)if values.len()==1&&matches!(&values[0],DslValue::String(value)if value.is_empty())));assert!(!loan.matches_source(crate::norm_inspector_seed_source(page.borrow_original().unwrap(),1).unwrap()))}else{assert_eq!(output.get("kind").and_then(DslValue::as_str),Some("b"))}
 driver::close(&mut receiver,&mut next.bank,BODY);assert!(receiver.terminal_is_empty());
 release(&mut next,loan);
 driver::retire(output,&mut next.bank);
 driver::retire(page,&mut next.bank);
}
#[test]
fn finalized_original_page_survives_decoder_and_caller_return_and_lends_native_source_to_next_caller(){
 page(None,semio_ui_locale::Locale::En,&VARIANT,0,true);
}

#[test]
fn compiled_singleton_seed_lends_a_real_array_across_complete_original_caller_return(){
 page(None,semio_ui_locale::Locale::De,&ABSENT_ARRAY,0,true);
}
