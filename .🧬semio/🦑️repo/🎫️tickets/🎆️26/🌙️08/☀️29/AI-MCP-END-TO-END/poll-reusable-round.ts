import{readFileSync,writeFileSync,renameSync,appendFileSync}from'node:fs';import{dirname,join}from'node:path';
const ticket=dirname(import.meta.path),root='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/';
function rep(s:string,a:string,b:string){if(s.split(a).length!==2)throw Error('anchor '+a.slice(0,90));return s.replace(a,b)}
function edit(file:string,fn:(s:string)=>string){const old=readFileSync(root+file,'utf8'),next=fn(old),tmp=join(ticket,'🗑️generated/poll-round-'+Date.now()+'.rs');writeFileSync(tmp,next);if(readFileSync(root+file,'utf8')!==old)throw Error('concurrent '+file);renameSync(tmp,root+file)}
edit('🦀️.rs',s=>{
 s=rep(s,'    core_quiescent:bool,','    core_quiescent:bool,\n    origin_binding:Option<OriginalSourceBinding>,origin_address:Option<usize>,decode_policy:OriginalNativeDecodePolicy,decode_limits:JsonReadLimits,');
 s=rep(s,'Self { receiver,binding:','Self { origin_binding:None,origin_address:None,decode_policy:policy,decode_limits:limits,receiver,binding:');
 s=rep(s,'        if self.binding.is_some_and','        if self.origin_binding.is_some_and(|binding|!binding.matches(cx))||self.origin_address.is_some_and(|address|address!=self as*const Self as usize){return Err(invalid("Poll changed its original native producer across commands"))}\n        if self.binding.is_some_and');
 s=rep(s,'        self.binding = Some(OriginalSourceBinding::borrow(cx));','        self.origin_binding.get_or_insert(OriginalSourceBinding::borrow(cx));self.origin_address.get_or_insert(self as*const Self as usize);\n        self.binding = Some(OriginalSourceBinding::borrow(cx));');
 const anchor='    pub fn terminal_is_empty(&self) -> bool {';
 const method=`    /// 🔁️ Begins another fully settled command on the same native producer and resident guest context.
    pub fn begin_next_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(cx)?;if !self.terminal_is_empty()||self.origin_binding.is_none()||self.origin_address.is_none()||self.failed||self.core_quiescent{return Err(invalid("next Poll command requires its original successful settled round"))}if cx.original_retirement_active()||cx.is_cancelled()||cx.should_yield(){return Ok(false)}
        let bytes=std::mem::size_of_val(&self.unit)+std::mem::size_of_val(&self.decoder)+std::mem::size_of_val(&self.prepared)+std::mem::size_of_val(&self.phase)+3*std::mem::size_of::<bool>()+2*std::mem::size_of::<u32>();if !charge_frame(cx,bytes)?{return Ok(false)}
        self.unit=GuestRelayUnitFlow::empty();self.decoder=OriginalJsonOutputSource::new(self.decode_policy,self.decode_limits);self.prepared=None;self.phase=Phase::Encode;self.resident=false;self.output_captured=false;self.awaiting_input_budget=false;self.pointer=0;self.length=0;Ok(true)
    }
`;
 return rep(s,anchor,method+anchor);
});
edit('🧪️tests/🦀️.rs',s=>{
 s=rep(s,'struct Receiver{closed:bool}','struct Receiver{closed:bool,failure:ActorFailureRecipient}\n impl GuestPollFailureReceiving for Receiver{fn actor_failure_recipient(&mut self)->&mut ActorFailureRecipient{&mut self.failure}}');
 s=rep(s,'if self.closed{return Ok(true)}let grant=','if self.closed{return self.failure.close_on_source(cx)}let grant=');
 s=rep(s,'fn terminal_is_empty(&self)->bool{self.closed}','fn terminal_is_empty(&self)->bool{self.closed&&self.failure.terminal_is_empty()}');
 return rep(s,'Receiver{closed:false});','Receiver{closed:false,failure:ActorFailureRecipient::empty()});');
});
appendFileSync(join(ticket,'poll-native-receiver-2026-11-06.md'),'\n## Reusable Commands\n\nA completed Poll already clears its per-command binding on the next actual advance; the earlier research statement that Complete was unreachable was incorrect. A permanent opaque origin binding/frame address now survives that command cleanup, separate from the active command binding. begin_next_step validates that actual origin, requires a fully settled successful round and live nonretirement turn, and pays the exact empty-unit/decoder/control reset before beginning Encode on the same guest context. No guest or decoder Source birth occurs during the reset. Native component laws and actual MCP adoption remain to be qualified.\n');
console.log('Source Poll retains its permanent native origin and has a paid next-command reset');
