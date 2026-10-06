//! 📨️ Owned child emission preparation retains typed operation owners across every refusal.
use crate::{store,protocol,app::{ChildEmit,PluginCloseStep}};
use semio_framework_diagnostic::{Fault,FaultFrom};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementStep};

pub enum ChildEmitPreparationStep {
    Pending,
    Ready,
    Refused(Fault),
}
trait ChildEmitPreparationOwner: Send {
    fn as_any_mut(&mut self)->&mut dyn std::any::Any;
    fn step(&mut self, maximum_items:usize, maximum_bytes:usize)->Result<ChildEmitPreparationStep,Fault>;
    fn take_ready(&mut self)->Option<ChildEmit>;
    fn begin_close(&mut self);
    fn close_step(&mut self, maximum_items:usize, maximum_bytes:usize)->Result<PluginCloseStep,Fault>;
    fn terminal_is_empty(&self)->bool;
    fn next_close_byte_demand(&mut self)->usize;
    fn refusal(&self)->Option<&::protocol::ProtocolError>;
    fn retirement_refusal(&self)->Option<&semio_framework_value::ValueError>;
    fn accepted_prefix(&self)->Option<&ChildEmit>;
    fn retained_operation_count(&self)->usize;
}
pub struct ChildEmitPreparation {
    owner:Box<dyn ChildEmitPreparationOwner>,
}
impl ChildEmitPreparation {
    pub fn of<S,M>(slot:impl Into<String>,child_id:impl Into<String>,operations:Vec<M>)->Self
    where M:protocol::SemanticMutation<S>+::protocol::OpBinary+semio_framework_value::retirement::RetireOwned {
        Self::with_factory_kind::<S,M>(slot.into(),child_id.into(),operations,None,Some(semio_framework_value::retirement::owned_retirement::<M>))
    }
    pub fn with_factory<S,M>(slot:String,child_id:String,operations:Vec<M>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>)->Self
    where M:protocol::SemanticMutation<S>+::protocol::OpBinary+Send+'static {
        Self::with_factory_kind::<S,M>(slot,child_id,operations,Some(factory),None)
    }
    fn with_factory_kind<S,M>(slot:String,child_id:String,operations:Vec<M>,factory:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,retire_builtin:Option<fn(M)->Box<dyn ErasedSnapshotRetirement>>)->Self
    where M:protocol::SemanticMutation<S>+::protocol::OpBinary+Send+'static {
        let backing_bytes=operations.capacity().checked_mul(std::mem::size_of::<M>()).expect("owned mutation vector allocation has a representable layout");
        Self{owner:Box::new(TypedChildEmitPreparation::<M>{
            remaining:ManuallyDrop::new(Some(operations.into_iter())),backing_bytes,
            current:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),retired_schema:ManuallyDrop::new(None),
            prefix:Some(ChildEmit::open(slot,child_id,0)),cause:ManuallyDrop::new(None),close_refusal:ManuallyDrop::new(None),factory:ManuallyDrop::new(factory),retire_builtin,append:|prefix,operation|prefix.push::<S,M>(operation),
            closing:false,ready:false,
        })}
    }
    pub fn take_retirement_provider<M:Send+'static>(&mut self)->Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>{
        let owner=self.owner.as_any_mut().downcast_mut::<TypedChildEmitPreparation<M>>()?;
        if owner.retire_builtin.is_some()||owner.remaining.is_some()||owner.current.is_some()||owner.retirement.is_some(){return None;}
        owner.factory.take()
    }
    pub fn step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<ChildEmitPreparationStep,Fault>{self.owner.step(maximum_items,maximum_bytes)}
    pub fn take_ready(&mut self)->Option<ChildEmit>{self.owner.take_ready()}
    pub fn begin_close(&mut self){self.owner.begin_close()}
    pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<PluginCloseStep,Fault>{self.owner.close_step(maximum_items,maximum_bytes)}
    pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
    pub fn owner_cell_bytes(&self)->usize{std::mem::size_of_val(self.owner.as_ref())}
    pub fn next_close_byte_demand(&mut self)->usize{self.owner.next_close_byte_demand()}
    pub fn refusal(&self)->Option<&::protocol::ProtocolError>{self.owner.refusal()}
    pub fn retirement_refusal(&self)->Option<&semio_framework_value::ValueError>{self.owner.retirement_refusal()}
    pub fn accepted_prefix(&self)->Option<&ChildEmit>{self.owner.accepted_prefix()}
    pub fn retained_operation_count(&self)->usize{self.owner.retained_operation_count()}
}
impl Drop for ChildEmitPreparation {
    fn drop(&mut self){assert!(std::thread::panicking()||self.owner.terminal_is_empty(),"child emission preparation dropped before its exact typed owners and accepted prefix were returned or retired");}
}
struct TypedChildEmitPreparation<M> {
    remaining:ManuallyDrop<Option<std::vec::IntoIter<M>>>,
    backing_bytes:usize,
    current:ManuallyDrop<Option<M>>,
    retirement:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    retired_schema:ManuallyDrop<Option<String>>,
    prefix:Option<ChildEmit>,
    cause:ManuallyDrop<Option<::protocol::ProtocolError>>,
    close_refusal:ManuallyDrop<Option<semio_framework_value::ValueError>>,
    factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,
    retire_builtin:Option<fn(M)->Box<dyn ErasedSnapshotRetirement>>,
    append:fn(&mut ChildEmit,&M)->Result<Option<semio_framework::kernel::SchemaId>,::protocol::ProtocolError>,
    closing:bool,
    ready:bool,
}
impl<M> TypedChildEmitPreparation<M>
where M:Send+'static {
    fn retire_one(&mut self,maximum_bytes:usize)->Result<PluginCloseStep,Fault>{
        if let Some(error)=self.close_refusal.as_mut(){
            if !self.closing{return Err(retirement_refusal_fault(error));}
            let bytes=error.message.capacity();
            if bytes>maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
            self.close_refusal.take();return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
        }
        if let Some(schema)=self.retired_schema.as_ref(){
            let bytes=schema.capacity();
            if bytes>maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
            self.retired_schema.take();return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
        }
        if let Some(retirement)=self.retirement.as_mut(){
            if retirement.terminal_is_empty(){
                let bytes=std::mem::size_of_val(retirement.as_ref());
                if bytes>maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
                self.retirement.take();
                return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
            }
            let step=match retirement.close_step(1,maximum_bytes){
                Ok(step)=>step,
                Err(error)=>{let fault=retirement_refusal_fault(&error);*self.close_refusal=Some(error);return Err(fault);}
            };
            return Ok(match step{SnapshotRetirementStep::Pending{released_items,released_bytes}=>PluginCloseStep::Pending{released_items,released_bytes},SnapshotRetirementStep::Blocked=>PluginCloseStep::Pending{released_items:0,released_bytes:0},SnapshotRetirementStep::Complete=>PluginCloseStep::Pending{released_items:1,released_bytes:0}});
        }
        if self.current.is_some()&&std::mem::size_of::<M>().max(1)>maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
        if let Some(operation)=self.current.take(){
            *self.retirement=Some(match self.retire_builtin{Some(retire)=>retire(operation),None=>self.factory.as_ref().expect("nonterminal owned mutation retains its exact retirement factory").retire_owned(operation)});
            return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});
        }
        if self.remaining.as_ref().is_some_and(|owner|owner.len()!=0){
            if std::mem::size_of::<M>().max(1)>maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
            *self.current=self.remaining.as_mut().expect("remaining owner exists").next();
            return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});
        }
        if self.backing_bytes!=0{
            if self.backing_bytes>maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
            let released_bytes=self.backing_bytes;self.backing_bytes=0;self.remaining.take();
            return Ok(PluginCloseStep::Pending{released_items:1,released_bytes});
        }
        self.remaining.take();
        Ok(PluginCloseStep::Complete)
    }
}
impl<M> ChildEmitPreparationOwner for TypedChildEmitPreparation<M>
where M:Send+'static {
    fn as_any_mut(&mut self)->&mut dyn std::any::Any{self}
    fn step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<ChildEmitPreparationStep,Fault>{
        if maximum_items==0||maximum_bytes==0{return Ok(ChildEmitPreparationStep::Pending);}
        if self.closing{return Err(Fault::from("owned-child-emission-step-after-close"));}
        if let Some(cause)=self.cause.as_ref(){return Ok(ChildEmitPreparationStep::Refused(cause.to_fault()));}
        if self.ready{return Ok(ChildEmitPreparationStep::Ready);}
        if self.retired_schema.is_some()||self.retirement.is_some()||self.current.is_some(){
            self.retire_one(maximum_bytes)?;
            return Ok(ChildEmitPreparationStep::Pending);
        }
        if maximum_bytes<std::mem::size_of::<M>().max(1){return Ok(ChildEmitPreparationStep::Pending);}
        if let Some(operation)=self.remaining.as_mut().and_then(Iterator::next){
            *self.current=Some(operation);
            let appended=(self.append)(self.prefix.as_mut().expect("unpublished preparation owns its prefix"),self.current.as_ref().expect("exact current operation"));
            match appended{
                Ok(retired_schema)=>*self.retired_schema=retired_schema.map(|schema|schema.0),
                Err(cause)=>{let fault=cause.to_fault();*self.cause=Some(cause);return Ok(ChildEmitPreparationStep::Refused(fault));}
            }
            return Ok(ChildEmitPreparationStep::Pending);
        }
        if self.retire_one(maximum_bytes)?!=PluginCloseStep::Complete{return Ok(ChildEmitPreparationStep::Pending);}
        if self.factory.is_some(){return Ok(ChildEmitPreparationStep::Pending);}
        self.ready=true;
        Ok(ChildEmitPreparationStep::Ready)
    }
    fn take_ready(&mut self)->Option<ChildEmit>{
        if !self.ready||self.closing||self.retired_schema.is_some()||self.current.is_some()||self.retirement.is_some()||self.remaining.is_some(){return None;}
        if self.factory.is_some(){return None;}
        self.prefix.take()
    }
    fn begin_close(&mut self){self.closing=true;}
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<PluginCloseStep,Fault>{
        if !self.closing{return Err(Fault::from("owned-child-emission-close-before-admission"));}
        if maximum_items==0||maximum_bytes==0{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
        let step=self.retire_one(maximum_bytes)?;
        if step!=PluginCloseStep::Complete{return Ok(step);}
        if let Some(prefix)=self.prefix.as_mut(){
            let step=prefix.close_one(1,maximum_bytes);
            if step==PluginCloseStep::Complete{self.prefix.take();}
            return Ok(step);
        }
        let cause_step=close_protocol_owned_cause_one(&mut self.cause,maximum_bytes);
        if cause_step!=PluginCloseStep::Complete{return Ok(cause_step);}
        if self.factory.is_some(){return Ok(PluginCloseStep::AwaitingInput{reason:"custom child emission retirement provider awaits its owning caller handback"});}
        Ok(PluginCloseStep::Complete)
    }
    fn terminal_is_empty(&self)->bool{
        self.remaining.is_none()&&self.current.is_none()&&self.retirement.is_none()&&self.retired_schema.is_none()&&self.prefix.is_none()&&self.cause.is_none()&&self.close_refusal.is_none()&&self.factory.is_none()&&self.backing_bytes==0
    }
    fn next_close_byte_demand(&mut self)->usize{
        if let Some(error)=self.close_refusal.as_ref(){return error.message.capacity().max(1);}
        if let Some(schema)=self.retired_schema.as_ref(){return schema.capacity().max(1);}
        if let Some(retirement)=self.retirement.as_ref(){return retirement.next_close_byte_demand().max(std::mem::size_of_val(retirement.as_ref())).max(1);}
        if self.current.is_some()||self.remaining.as_ref().is_some_and(|owner|owner.len()!=0){return std::mem::size_of::<M>().max(1);}
        if self.backing_bytes!=0{return self.backing_bytes;}
        if let Some(prefix)=self.prefix.as_ref(){return prefix.next_close_byte_demand().max(1);}
        if self.cause.is_none()&&self.factory.is_some(){return 1;}
        self.cause.as_mut().and_then(|cause|protocol_owned_cause_text(cause).ok().flatten()).map(|text|text.capacity()).unwrap_or(1)
    }
    fn refusal(&self)->Option<&::protocol::ProtocolError>{self.cause.as_ref()}
    fn retirement_refusal(&self)->Option<&semio_framework_value::ValueError>{self.close_refusal.as_ref()}
    fn accepted_prefix(&self)->Option<&ChildEmit>{self.prefix.as_ref()}
    fn retained_operation_count(&self)->usize{self.remaining.as_ref().map_or(0,ExactSizeIterator::len)+usize::from(self.current.is_some())+usize::from(self.retirement.is_some())}
}
impl<M> Drop for TypedChildEmitPreparation<M>{
    fn drop(&mut self){
        assert!(std::thread::panicking()||(self.remaining.is_none()&&self.current.is_none()&&self.retirement.is_none()&&self.retired_schema.is_none()&&self.prefix.is_none()&&self.cause.is_none()&&self.close_refusal.is_none()&&self.factory.is_none()),"typed child emission lost actual operation, prefix or encoder refusal owners");
        unsafe{ManuallyDrop::drop(&mut self.remaining);ManuallyDrop::drop(&mut self.current);ManuallyDrop::drop(&mut self.retirement);ManuallyDrop::drop(&mut self.cause);ManuallyDrop::drop(&mut self.retired_schema);ManuallyDrop::drop(&mut self.close_refusal);ManuallyDrop::drop(&mut self.factory);}
    }
}


fn protocol_owned_cause_text(cause:&mut ::protocol::ProtocolError)->Result<Option<&mut String>,()> {
    use ::protocol::ProtocolError as Error;
    match cause{
        Error::Malformed{detail,..}|Error::Io(detail)=>Ok((detail.capacity()!=0).then_some(detail)),
        Error::Pack(error)=>match error{
            store::PackError::TransportFailure(_)=>Err(()),
            store::PackError::Refusal(refusal)=>match refusal{
                store::PackRefusal::Malformed{detail,..}=>Ok((detail.capacity()!=0).then_some(detail)),
                store::PackRefusal::ValueRefusal(error)|store::PackRefusal::Io{error,..}=>Ok((error.message.capacity()!=0).then_some(&mut error.message)),
                store::PackRefusal::TextRefusal(error)=>{
                    if error.message.capacity()!=0{return Ok(Some(&mut error.message));}
                    Ok(error.expected.as_mut().filter(|value|value.capacity()!=0))
                },
                store::PackRefusal::BadMagic|store::PackRefusal::UnsupportedVersion{..}|store::PackRefusal::UnknownRequiredFlags(_)|store::PackRefusal::Truncated(_)|store::PackRefusal::ChecksumMismatch{..}|store::PackRefusal::ContentHashMismatch|store::PackRefusal::LimitExceeded{..}|store::PackRefusal::RetainedMalformed{..}|store::PackRefusal::RetainedAllocation{..}|store::PackRefusal::NonCanonical(_)|store::PackRefusal::UnsupportedCodec(_)|store::PackRefusal::TransportAdmission{..}=>Ok(None),
            }
        },
        Error::ChainMismatch{..}|Error::TornTail(_)|Error::UnknownCriticalRecord(_)|Error::DictMiss(_)|Error::DictOutOfOrder{..}|Error::VerifierRequired|Error::SignatureInvalid{..}|Error::FrameFraming(_)|Error::LimitExceeded(_)=>Ok(None),
    }
}
pub(crate) fn close_protocol_owned_cause_one(cause:&mut Option<::protocol::ProtocolError>,maximum_bytes:usize)->PluginCloseStep{
    let Some(error)=cause.as_mut()else{return PluginCloseStep::Complete};
    match protocol_owned_cause_text(error){
        Err(())=>PluginCloseStep::AwaitingInput{reason:"owned encoder transport cause requires its genuine provider retirement handoff"},
        Ok(Some(text))=>{
            let bytes=text.capacity();
            if bytes>maximum_bytes{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
            *text=String::new();
            PluginCloseStep::Pending{released_items:1,released_bytes:bytes}
        },
        Ok(None)=>{cause.take();PluginCloseStep::Pending{released_items:1,released_bytes:0}},
    }
}

pub(crate) fn retirement_refusal_fault(error:&semio_framework_value::ValueError)->Fault{
    Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,"framework.child-emission.retirement-refusal",&error.message).with_param("refusalKind",error.kind.as_str())
}
