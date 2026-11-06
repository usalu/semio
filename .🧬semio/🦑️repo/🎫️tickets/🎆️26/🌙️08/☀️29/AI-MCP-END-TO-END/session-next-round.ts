import{readFileSync,writeFileSync,renameSync,appendFileSync}from'node:fs';import{dirname,join}from'node:path';
const ticket=dirname(import.meta.path),file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs',old=readFileSync(file,'utf8');let s=old;
function rep(a:string,b:string){if(s.split(a).length!==2)throw Error('anchor '+a.slice(0,110));s=s.replace(a,b)}
rep(' guest:Option<GuestRelayGuestSubloan>,',' next_place:OriginalCallerReturnPlace,next_result:ManuallyDrop<Option<Result<bool,OriginalTurnFailure>>>,\n guest:Option<GuestRelayGuestSubloan>,');
rep('cleanup_failure:ManuallyDrop::new(None),guest:','cleanup_failure:ManuallyDrop::new(None),next_place:OriginalCallerReturnPlace::empty(),next_result:ManuallyDrop::new(None),guest:');
const method=` /// 🔁️ Captures and settles the actual next-command reset before releasing another request to the resident guest.
 pub fn next_input_step(&mut self)->GuestPollSessionStatus{
  if self.failure.is_some()||self.cleanup_failure.is_some()||!self.opened||self.closing||self.closed{return GuestPollSessionStatus::Refused}if !self.delivered&&self.next_result.is_none()&&!self.next_place.is_admitted(){return GuestPollSessionStatus::Ready}if self.stage!=0||self.guest.is_some()||self.request.is_some()||self.frozen.is_some(){return GuestPollSessionStatus::Pending}
  let producer=self.producer.as_mut().unwrap();let place=&mut self.next_place;let result=&mut*self.next_result;let delivered=&mut self.delivered;let mut ready=false;
  let turn=self.driver.dispatch_operation_step(false,false,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|_,_,_,poll,cx|{
   if place.is_admitted()||result.is_some(){settle_next_round(place,result,delivered,cx)?;ready=!*delivered;return Ok(())}
   let address=(result as*mut Option<Result<bool,OriginalTurnFailure>>)as usize;let bytes=std::mem::size_of_val(result);if !place.admit_sized(cx,address,bytes)?{return Ok(())}*result=Some(poll.begin_next_step(cx));place.record_sized_return(address,bytes)?;Ok(())
  })?;Ok(())})?;Ok(())});self.retain(turn,false);if self.failure.is_some(){GuestPollSessionStatus::Refused}else if ready{GuestPollSessionStatus::Ready}else{GuestPollSessionStatus::Pending}
 }
`;
rep(' pub fn input<\'a,S:JsonWriteSource+?Sized>',method+' pub fn input<\'a,S:JsonWriteSource+?Sized>');
rep('  let input=input.get_ref();match self.driver.policy()', '  if self.next_place.is_admitted()||self.next_result.is_some(){return GuestPollSessionStatus::Pending}let input=input.get_ref();match self.driver.policy()');
rep('   0=>{\n    if let Some(admission)',`   0=>{
    if self.next_place.is_admitted()||self.next_result.is_some(){let place=&mut self.next_place;let result=&mut*self.next_result;let delivered=&mut self.delivered;self.driver.dispatch_operation_step(false,true,|cx|{settle_next_round(place,result,delivered,cx)?;Ok(())})?;return Ok(false)}
    if let Some(admission)`);
rep('pub fn terminal_is_empty(&self)->bool{self.closed&&','pub fn terminal_is_empty(&self)->bool{self.next_result.is_none()&&self.next_place.terminal_is_empty()&&self.closed&&');
s+=`\nfn settle_next_round(place:&mut OriginalCallerReturnPlace,result:&mut Option<Result<bool,OriginalTurnFailure>>,delivered:&mut bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if place.is_admitted(){place.settle_return(cx)?;return Ok(false)}if result.is_none(){return Ok(true)}let bytes=std::mem::size_of_val(result)+std::mem::size_of::<bool>();if !session_metadata(cx,bytes)?{return Ok(false)}match result.take().unwrap(){Ok(true)=>*delivered=false,Ok(false)=>{},Err(error)=>return Err(error)}Ok(true)
}\n`;
const tmp=join(ticket,'🗑️generated/session-next-'+Date.now()+'.rs');writeFileSync(tmp,s);if(readFileSync(file,'utf8')!==old)throw Error('concurrent');renameSync(tmp,file);
appendFileSync(join(ticket,'poll-native-receiver-2026-11-06.md'),'\nThe Session next_input_step uses a preborn original Result<bool,OriginalTurnFailure> slot and exact sized Source return place. It always captures the actual reset result, including Pendingfalse or the complete cause; a later funded turn settles the witness and promotes the result before another request may drive. Cleanup drains that same frontier. A reset error returns through the existing complete Driver carrier instead of being formatted or replacing another error. This is authored and awaiting native qualification.\n');console.log('Session next command has actual preissued result custody, separate settlement, funded promotion and cleanup');
