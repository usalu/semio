#[derive(Clone)]
struct TrackedChildOperation {
    inner:RefusingChildOperation,
    payload:Vec<u8>,
    retired:bool,
    returned:std::sync::Arc<std::sync::atomic::AtomicUsize>,
}
impl semio_framework_value::ToValue for TrackedChildOperation{
    fn to_value(&self)->semio_framework_value::DslValue{semio_framework_value::DslValue::Object(vec![("inner".into(),semio_framework_value::ToValue::to_value(&self.inner)),("payload".into(),semio_framework_value::ToValue::to_value(&self.payload))])}
}
impl semio_framework_value::FromValue for TrackedChildOperation{
    fn from_value(_:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"tracked emission source requires its registered owning fixture issuer"))}
}
impl Drop for TrackedChildOperation{
    fn drop(&mut self){assert!(std::thread::panicking()||(self.retired&&self.payload.capacity()==0),"actual typed child operation payload lost before its genuine retirement");}
}
impl ::protocol::OpBinary for TrackedChildOperation{
    fn encode_op(&self)->Result<Vec<u8>,::protocol::ProtocolError>{self.inner.encode_op()}
    fn decode_op(_: &[u8])->Result<Self,::protocol::ProtocolError>{Err(::protocol::ProtocolError::LimitExceeded("test source is issued only by the owned fixture"))}
}
impl protocol::Mutation<TestSnapshot> for TrackedChildOperation{
    type Diff=<RefusingChildOperation as protocol::Mutation<TestSnapshot>>::Diff;
    const DESCRIPTORS:&'static[::protocol::MutationLeafDescriptor]=<RefusingChildOperation as protocol::Mutation<TestSnapshot>>::DESCRIPTORS;
    fn descriptor(&self)->&'static ::protocol::MutationLeafDescriptor{protocol::Mutation::descriptor(&self.inner)}
    fn diff(&self,base:&TestSnapshot)->::protocol::MutationOutcome<Self::Diff>{protocol::Mutation::diff(&self.inner,base)}
    fn inverse(&self,_:&TestSnapshot)->Result<Vec<Self>,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"tracked fixture mutation has no inverse source admission"))}
    fn retire_cold(self){panic!("actual owned emission must use the installed retirement continuation, not Mutation::retire_cold");}
}
impl protocol::SemanticMutation<TestSnapshot> for TrackedChildOperation{
    fn kinds()->&'static[protocol::SemanticDescriptor]{<RefusingChildOperation as protocol::SemanticMutation<TestSnapshot>>::kinds()}
    fn semantics(&self)->&'static protocol::SemanticDescriptor{protocol::SemanticMutation::semantics(&self.inner)}
    fn label(&self)->LocalizedLabel{protocol::SemanticMutation::label(&self.inner)}
    fn target(&self)->Vec<String>{protocol::SemanticMutation::target(&self.inner)}
}
struct TrackedChildRetirement{operation:Option<TrackedChildOperation>,refuse_once:bool}
impl store::ErasedSnapshotRetirement for TrackedChildRetirement{
    fn close_step(&mut self,items:usize,bytes:usize)->Result<store::SnapshotRetirementStep,semio_framework_value::ValueError>{
        if items==0||bytes==0{return Ok(store::SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        if self.refuse_once{self.refuse_once=false;return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"actual accepted mutation retirement refusal"));}
        let Some(operation)=self.operation.as_mut()else{return Ok(store::SnapshotRetirementStep::Complete)};
        let allocation=operation.payload.capacity();
        if allocation>bytes{return Ok(store::SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        if allocation!=0{operation.payload=Vec::new();return Ok(store::SnapshotRetirementStep::Pending{released_items:1,released_bytes:allocation});}
        operation.retired=true;operation.returned.fetch_add(1,std::sync::atomic::Ordering::SeqCst);self.operation.take();
        Ok(store::SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})
    }
    fn terminal_is_empty(&self)->bool{self.operation.is_none()}
    fn next_close_byte_demand(&self)->usize{self.operation.as_ref().map_or(0,|operation|operation.payload.capacity().max(1))}
}
struct TrackedChildRetirementFactory;
impl store::ArtifactOwnedValueRetirementFactory<TrackedChildOperation> for TrackedChildRetirementFactory{
    fn retire_owned(&self,operation:TrackedChildOperation)->Box<dyn store::ErasedSnapshotRetirement>{Box::new(TrackedChildRetirement{operation:Some(operation),refuse_once:false})}
}
struct RefusingTrackedChildRetirementFactory;
impl store::ArtifactOwnedValueRetirementFactory<TrackedChildOperation> for RefusingTrackedChildRetirementFactory{
    fn retire_owned(&self,operation:TrackedChildOperation)->Box<dyn store::ErasedSnapshotRetirement>{Box::new(TrackedChildRetirement{operation:Some(operation),refuse_once:true})}
}

#[test]
fn child_emission_owned_refusal_keeps_prefix_rejected_and_remaining_until_actual_typed_retirement(){
    use crate::app::{ChildEmitPreparation,ChildEmitPreparationStep,PluginCloseStep};
    let fixture:Value=serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🧫️fixtures/🔣️.json")).expect("closed owned emission law");
    let demand=&fixture["ownedPreparation"];
    let returned=std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let operations=demand["operations"].as_array().expect("complete authored source").iter().map(|row|TrackedChildOperation{
        inner:RefusingChildOperation{inner:TestMutation::SetCount(SetCount{value:i32::try_from(row["value"].as_i64().expect("signed source word")).expect("exact i32 word")}),refuse:row["refuse"].as_bool().expect("authored refusal")},
        payload:row["payload"].as_array().expect("actual owned source payload").iter().map(|word|u8::try_from(word.as_u64().expect("unsigned byte")).expect("exact byte")).collect(),
        retired:false,returned:std::sync::Arc::clone(&returned),
    }).collect::<Vec<_>>();
    let count=operations.len();
    let factory:std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<TrackedChildOperation>>=std::sync::Arc::new(TrackedChildRetirementFactory);
    let mut preparation=ChildEmitPreparation::with_factory::<TestSnapshot,_>(fixture["slot"].as_str().unwrap().to_owned(),fixture["childId"].as_str().unwrap().to_owned(),operations,std::sync::Arc::clone(&factory));
    assert!(matches!(preparation.step(1,0).expect("zero-byte retained step"),ChildEmitPreparationStep::Pending));
    assert_eq!(preparation.retained_operation_count(),count);
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),0);
    let maximum=demand["maximumAllocationGrant"].as_u64().unwrap() as usize;
    let mut refused=false;
    for _ in 0..demand["maximumPumpSteps"].as_u64().unwrap(){
        match preparation.step(1,maximum).expect("actual owned producer"){
            ChildEmitPreparationStep::Pending=>{},
            ChildEmitPreparationStep::Refused(fault)=>{assert_eq!(fault.code.0,"module.protocol");refused=true;break;},
            ChildEmitPreparationStep::Ready=>panic!("closed second encoder refusal cannot issue a complete group"),
        }
    }
    assert!(refused);
    let prefix=preparation.accepted_prefix().expect("actual retained prefix");
    assert_eq!(prefix.slot,fixture["slot"].as_str().unwrap());assert_eq!(prefix.child_id,fixture["childId"].as_str().unwrap());
    assert_eq!(prefix.ops.len(),demand["expected"]["acceptedOperations"].as_u64().unwrap() as usize);
    let independent=serde_json::to_value(prefix).expect("independent complete wire projection");
    assert_eq!(Value::from(semio_framework_value::ToValue::to_value(prefix)),independent);
    assert!(matches!(preparation.refusal(),Some(::protocol::ProtocolError::Malformed{what:"owned-child-operation",offset:9,detail})if detail==fixture["refusal"]["detail"].as_str().unwrap()));
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),demand["expected"]["retiredBeforeRefusal"].as_u64().unwrap() as usize);
    assert_eq!(preparation.retained_operation_count(),demand["expected"]["retainedAfterRefusal"].as_u64().unwrap() as usize);
    preparation.begin_close();
    assert_eq!(preparation.close_step(1,0).expect("zero close"),PluginCloseStep::Pending{released_items:0,released_bytes:0});
    assert_eq!(preparation.retained_operation_count(),demand["expected"]["retainedAfterRefusal"].as_u64().unwrap() as usize);
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),demand["expected"]["retiredAfterZeroGrant"].as_u64().unwrap() as usize);
    assert_eq!(preparation.close_step(1,1).expect("unfunded typed owner close"),PluginCloseStep::Pending{released_items:0,released_bytes:0});
    assert_eq!(preparation.retained_operation_count(),2);
    let mut returned_provider=None;
    for _ in 0..demand["maximumCloseSteps"].as_u64().unwrap(){
        if preparation.terminal_is_empty(){break;}
        let grant=preparation.next_close_byte_demand().max(1);assert!(grant<=maximum);
        let step=preparation.close_step(1,grant).expect("actual retained owned close");
        if let PluginCloseStep::Pending{released_items,released_bytes}=step{assert!(released_items<=1);assert!(released_bytes<=grant);}
        else if let PluginCloseStep::AwaitingInput{reason}=step{
            assert_eq!(reason,"custom child emission retirement provider awaits its owning caller handback");
            assert!(returned_provider.is_none());
            let provider=preparation.take_retirement_provider::<TrackedChildOperation>().expect("same typed provider owner returned");
            assert!(std::sync::Arc::ptr_eq(&provider,&factory));returned_provider=Some(provider);
        }else{assert_eq!(step,PluginCloseStep::Complete);}
    }
    assert!(preparation.terminal_is_empty());
    assert_eq!(returned_provider.is_some(),demand["expected"]["providerReturnedToCaller"].as_bool().unwrap());
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),demand["expected"]["retiredAfterClose"].as_u64().unwrap() as usize);
    println!("[DEBUG] Actual owned child refusal retains accepted prefix and rejected plus remaining typed operations across zero grants, then returns each exact payload through its registered continuation");
}

#[test]
fn child_emission_accepted_owner_retirement_refusal_retains_exact_prefix_and_provider(){
    use crate::app::{ChildEmitPreparation,ChildEmitPreparationStep,PluginCloseStep};
    let fixture:Value=serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🧫️fixtures/🔣️.json")).expect("closed accepted retirement refusal");
    let demand=&fixture["ownedPreparation"];
    let returned=std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let operation=TrackedChildOperation{
        inner:RefusingChildOperation{inner:TestMutation::SetCount(SetCount{value:fixture["validValue"].as_i64().unwrap() as i32}),refuse:false},
        payload:demand["operations"][0]["payload"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect(),
        retired:false,returned:std::sync::Arc::clone(&returned),
    };
    let factory:std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<TrackedChildOperation>>=std::sync::Arc::new(RefusingTrackedChildRetirementFactory);
    let mut preparation=ChildEmitPreparation::with_factory::<TestSnapshot,_>(fixture["slot"].as_str().unwrap().into(),fixture["childId"].as_str().unwrap().into(),vec![operation],std::sync::Arc::clone(&factory));
    let maximum=demand["maximumAllocationGrant"].as_u64().unwrap() as usize;
    let mut refused=false;
    for _ in 0..demand["maximumPumpSteps"].as_u64().unwrap(){
        match preparation.step(1,maximum){Ok(ChildEmitPreparationStep::Pending)=>{},Err(fault)=>{assert_eq!(fault.code.0,"framework.child-emission.retirement-refusal");refused=true;break},_=>panic!("accepted owner retirement refusal cannot produce a complete group")}
    }
    assert!(refused);
    let error=preparation.retirement_refusal().expect("original typed retirement error");
    assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::UnsupportedOwner);
    assert_eq!(error.message,demand["retirementRefusal"]["message"].as_str().unwrap());
    assert_eq!(preparation.retained_operation_count(),1);
    let prefix=preparation.accepted_prefix().unwrap();
    assert_eq!(prefix.ops.len(),1);assert_eq!(prefix.labels.len(),1);
    assert_eq!(Value::from(semio_framework_value::ToValue::to_value(prefix)),serde_json::to_value(prefix).unwrap());
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),0);
    preparation.begin_close();
    assert_eq!(preparation.close_step(1,0).unwrap(),PluginCloseStep::Pending{released_items:0,released_bytes:0});
    assert!(preparation.retirement_refusal().is_some());
    let mut provider_returned=false;
    for _ in 0..demand["maximumCloseSteps"].as_u64().unwrap(){
        if preparation.terminal_is_empty(){break;}
        let grant=preparation.next_close_byte_demand().max(1);assert!(grant<=maximum);
        match preparation.close_step(1,grant).expect("retained accepted owner close"){
            PluginCloseStep::AwaitingInput{..}=>{let provider=preparation.take_retirement_provider::<TrackedChildOperation>().expect("original retained provider handback");assert!(std::sync::Arc::ptr_eq(&provider,&factory));provider_returned=true;},
            PluginCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=grant);},
            PluginCloseStep::Complete=>{},
            _=>panic!("genuine fixture provider closes after its one refusal"),
        }
    }
    assert!(provider_returned&&preparation.terminal_is_empty());
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),1);
    println!("[DEBUG] Accepted child operation retirement refuses with its exact typed owner and prefix retained, then returns the real payload and same provider under explicit grants");
}
