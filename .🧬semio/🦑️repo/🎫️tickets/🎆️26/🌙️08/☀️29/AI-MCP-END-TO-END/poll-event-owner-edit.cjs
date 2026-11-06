const fs=require('fs'),f='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📥️input/🦀️.rs',s=fs.readFileSync(f,'utf8');let t=s;
function once(a,b){if(t.split(a).length!==2)throw Error('anchor '+a);t=t.replace(a,b)}
once('job:ManuallyDrop<Option<semio_framework_actor::OriginalJobSource>>,','event:ManuallyDrop<Option<semio_framework::kernel::Event>>,\n    job:ManuallyDrop<Option<semio_framework_actor::OriginalJobSource>>,');
once('job:ManuallyDrop::new(None),job_retirement:', 'event:ManuallyDrop::new(None),job:ManuallyDrop::new(None),job_retirement:');
once('&&self.retirement.is_none()&&self.job.is_none()&&self.job_retirement.is_none()}', '&&self.retirement.is_none()&&self.event.is_none()&&self.job.is_none()&&self.job_retirement.is_none()}');
const anchor='    pub fn input_returned(&self)->bool';
const method=`    /// 📨️ Keeps the unconsumed event in the same Source owner across a pending turn.
    pub(super) fn prepare_poll_event(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(None,cx)?;if !self.input_returned()||self.closing{return Err(invalid())}
        let Some(ActorInput::Poll(poll))=self.input.as_ref()else{return Err(invalid())};
        if poll.events.len()>1||poll.command_page.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"poll input exceeds its retained single-event ingress").into())}
        if self.event.is_some()||poll.events.is_empty(){return Ok(true)}
        if !charge(cx,false,std::mem::size_of::<Option<semio_framework::kernel::Event>>()+std::mem::size_of::<usize>())?{return Ok(false)}
        let Some(ActorInput::Poll(poll))=self.input.as_mut()else{return Err(invalid())};*self.event=poll.events.pop();Ok(true)
    }
    pub(super) fn with_poll_step<T>(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut semio_plugin_abi::owned_abi::PollInput,&mut Option<semio_framework::kernel::Event>,&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<Option<T>,OriginalTurnFailure>{
        self.validate(None,cx)?;if !self.input_returned()||self.closing{return Err(invalid())}if !charge(cx,false,std::mem::size_of::<(&mut semio_plugin_abi::owned_abi::PollInput,&mut Option<semio_framework::kernel::Event>)>())?{return Ok(None)}
        let Some(ActorInput::Poll(poll))=self.input.as_mut()else{return Err(invalid())};Ok(Some(run(poll,&mut self.event,cx)?))
    }
`;
once(anchor,method+anchor);
const close='        if self.closing{\n            if self.job_retirement.is_none()';
const restore=`        if self.closing{
            if self.event.is_some(){if !charge(cx,true,std::mem::size_of::<Option<semio_framework::kernel::Event>>()+std::mem::size_of::<usize>())?{return Ok(InputAdmission::Pending)}let Some(ActorInput::Poll(poll))=self.input.as_mut()else{return Err(invalid())};poll.events.push(self.event.take().ok_or_else(invalid)?);return Ok(InputAdmission::Pending)}
            if self.job_retirement.is_none()`;
once(close,restore);let p=f+'.poll-event';fs.writeFileSync(p,t,{flag:'wx'});if(fs.readFileSync(f,'utf8')!==s){fs.unlinkSync(p);throw Error('concurrent change')}fs.renameSync(p,f);