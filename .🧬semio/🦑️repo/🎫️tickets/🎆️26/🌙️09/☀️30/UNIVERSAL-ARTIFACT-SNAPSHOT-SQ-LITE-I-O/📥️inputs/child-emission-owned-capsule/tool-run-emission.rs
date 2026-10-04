trait ToolRunMemberEmissionOwner:Send{
    fn as_any_mut(&mut self)->&mut dyn std::any::Any;
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<PluginCloseStep,Fault>;
    fn is_empty(&self)->bool;
}
struct ToolRunMemberEmissionState<M>{
    current:std::mem::ManuallyDrop<Option<M>>,
    retirement:std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    schema:std::mem::ManuallyDrop<Option<String>>,
    refusal:std::mem::ManuallyDrop<Option<::protocol::ProtocolError>>,
    retirement_refusal:std::mem::ManuallyDrop<Option<semio_framework_value::ValueError>>,
}
impl<M> Default for ToolRunMemberEmissionState<M>{
    fn default()->Self{Self{current:std::mem::ManuallyDrop::new(None),retirement:std::mem::ManuallyDrop::new(None),schema:std::mem::ManuallyDrop::new(None),refusal:std::mem::ManuallyDrop::new(None),retirement_refusal:std::mem::ManuallyDrop::new(None)}}
}
impl<M:Send+'static> ToolRunMemberEmissionOwner for ToolRunMemberEmissionState<M>{
    fn as_any_mut(&mut self)->&mut dyn std::any::Any{self}
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<PluginCloseStep,Fault>{
        if maximum_items==0||maximum_bytes==0{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
        if self.current.is_some(){return Ok(PluginCloseStep::AwaitingInput{reason:"decoded member mutation retains its exact Store retirement issuer"});}
        if let Some(error)=self.retirement_refusal.as_ref(){
            let bytes=error.message.capacity();
            if bytes>maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
            self.retirement_refusal.take();return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
        }
        if let Some(schema)=self.schema.as_ref(){
            let bytes=schema.capacity();
            if bytes>maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
            self.schema.take();return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
        }
        if let Some(retirement)=self.retirement.as_mut(){
            if retirement.terminal_is_empty(){
                let bytes=std::mem::size_of_val(retirement.as_ref());
                if bytes>maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
                self.retirement.take();return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
            }
            return match retirement.close_step(1,maximum_bytes){
                Ok(store::SnapshotRetirementStep::Pending{released_items,released_bytes})=>Ok(PluginCloseStep::Pending{released_items,released_bytes}),
                Ok(store::SnapshotRetirementStep::Complete)=>Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0}),
                Ok(store::SnapshotRetirementStep::Blocked)=>Ok(PluginCloseStep::Blocked{reason:"decoded member mutation retirement retains its provider continuation"}),
                Err(error)=>{let fault=crate::child_emit_preparation::retirement_refusal_fault(&error);*self.retirement_refusal=Some(error);Err(fault)},
            };
        }
        Ok(crate::child_emit_preparation::close_protocol_owned_cause_one(&mut self.refusal,maximum_bytes))
    }
    fn is_empty(&self)->bool{self.current.is_none()&&self.retirement.is_none()&&self.schema.is_none()&&self.refusal.is_none()&&self.retirement_refusal.is_none()}
}
impl<M> Drop for ToolRunMemberEmissionState<M>{
    fn drop(&mut self){
        assert!(std::thread::panicking()||(self.current.is_none()&&self.retirement.is_none()&&self.schema.is_none()&&self.refusal.is_none()&&self.retirement_refusal.is_none()),"member emission dropped its decoded mutation or exact refusal before retirement");
        unsafe{std::mem::ManuallyDrop::drop(&mut self.current);std::mem::ManuallyDrop::drop(&mut self.retirement);std::mem::ManuallyDrop::drop(&mut self.schema);std::mem::ManuallyDrop::drop(&mut self.refusal);std::mem::ManuallyDrop::drop(&mut self.retirement_refusal);}
    }
}

struct ToolRunMemberEmit<'a>{
    slot:&'a str,
    child_id:&'a str,
    ops:&'a [Vec<u8>],
    emit:&'a mut Option<ChildEmit>,
    owner:&'a mut Option<Box<dyn ToolRunMemberEmissionOwner>>,
    maximum_bytes:usize,
}
impl store::MemberStoreVisitor for ToolRunMemberEmit<'_>{
    type Output=Result<bool,Fault>;
    fn visit<P,Mu>(self,store:&ArtifactStore<P,Mu>)->Self::Output
    where
        P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,
        Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+::protocol::Mutation<P>+protocol::SemanticMutation<P>+::protocol::OpBinary+::protocol::OpText+Send+'static,
    {
        if self.maximum_bytes==0{return Ok(false);}
        let owner=self.owner.get_or_insert_with(||Box::new(ToolRunMemberEmissionState::<Mu>::default()));
        let typed=owner.as_any_mut().downcast_mut::<ToolRunMemberEmissionState<Mu>>().ok_or_else(||plugin_sdk_fault("member emission owns another exact mutation type"))?;
        if let Some(refusal)=typed.refusal.as_ref(){return Err(refusal.to_fault());}
        if typed.current.is_some(){
            if std::mem::size_of::<Mu>().max(1)>self.maximum_bytes{return Ok(false);}
            *typed.retirement=store.retire_owned_mutation(&mut typed.current).map_err(FaultFrom::into_fault)?;
            return Ok(false);
        }
        if !typed.is_empty(){typed.close_step(1,self.maximum_bytes)?;return Ok(false);}
        let emit=self.emit.get_or_insert_with(||ChildEmit::open(self.slot,self.child_id,0));
        if emit.ops.len()==self.ops.len(){return Ok(true);}
        if std::mem::size_of::<Mu>().max(1)>self.maximum_bytes{return Ok(false);}
        match <Mu as ::protocol::OpBinary>::decode_op(&self.ops[emit.ops.len()]){
            Ok(operation)=>*typed.current=Some(operation),
            Err(refusal)=>{let fault=refusal.to_fault();*typed.refusal=Some(refusal);return Err(fault);},
        }
        match emit.push::<P,Mu>(typed.current.as_ref().expect("exact decoded mutation owner")){
            Ok(schema)=>*typed.schema=schema.map(|schema|schema.0),
            Err(refusal)=>{let fault=refusal.to_fault();*typed.refusal=Some(refusal);return Err(fault);},
        }
        Ok(false)
    }
}

struct ToolRunMemberEmissionRetire<'a>{owner:&'a mut(dyn ToolRunMemberEmissionOwner+'static),maximum_bytes:usize}
impl store::MemberStoreVisitor for ToolRunMemberEmissionRetire<'_>{
    type Output=Result<PluginCloseStep,Fault>;
    fn visit<P,Mu>(self,store:&ArtifactStore<P,Mu>)->Self::Output
    where
        P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,
        Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+::protocol::Mutation<P>+protocol::SemanticMutation<P>+::protocol::OpBinary+::protocol::OpText+Send+'static,
    {
        if self.maximum_bytes==0{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
        let typed=self.owner.as_any_mut().downcast_mut::<ToolRunMemberEmissionState<Mu>>().ok_or_else(||plugin_sdk_fault("member retirement owns another exact mutation type"))?;
        if typed.current.is_some(){
            if std::mem::size_of::<Mu>().max(1)>self.maximum_bytes{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
            *typed.retirement=store.retire_owned_mutation(&mut typed.current).map_err(FaultFrom::into_fault)?;
            return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0});
        }
        typed.close_step(1,self.maximum_bytes)
    }
}
