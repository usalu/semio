import fs from 'node:fs';
const b='🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/';
{
 const p=b+'🧩️artifact/🫴️receiving/🦀️.rs';let s=fs.readFileSync(p,'utf8');
 const old='pub fn refused(&self,owned:usize,error:ValueError)->ValueError{match self.settle(owned){Err(cause)=>cause,Ok(())=>self.cause.borrow_mut().take().unwrap_or(error)}}';
 const added='pub fn take_refusal(&self,owned:usize)->Option<ValueError>{if let Some(cause)=self.cause.borrow_mut().take(){return Some(cause)}self.settle(owned).err()}';
 if(!s.includes(old))throw Error('Original SQL refusal source changed');s=s.replace(old,added);
 s=s.replace('pub fn observe(&self,owned:usize,completed:usize,total:usize)->bool{if !self.post_body.get()','pub fn observe(&self,owned:usize,completed:usize,total:usize)->bool{if self.cause.borrow().is_some(){return false}if !self.post_body.get()');
 fs.writeFileSync(p,s);
}
{
 const p=b+'🫴️receiving/🦀️.rs';let s=fs.readFileSync(p,'utf8');
 s=s.replace('NativeSnapshotBodyWallet,NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner','NativeSnapshotBodyWallet,NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner,NativeSnapshotFailure');
 s=s.replace('use semio_framework_value::{NativeEncodeControl,','use semio_framework_value::{RetainedWorkOutcome,NativeEncodeControl,');
 s=s.replaceAll('publish:impl FnOnce(&mut F)->Result<O,ValueError>)->Result<O,ValueError>','publish:impl FnOnce(&mut F)->Result<O,ValueError>)->RetainedWorkOutcome<O,NativeSnapshotFailure>');
 const start=s.indexOf('   let result=(||{self.native().checkpoint()?;');const end=s.indexOf('\n  }\n }\n };',start);if(start<0||end<0)throw Error('SQL macro implementation source changed');
 const body=`   let mut work=0;
   let result=(||->Result<O,NativeSnapshotFailure>{self.native().checkpoint()?;let before=self.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original SQLite receiving extent overflow"))?;let settlement=crate::artifact::receiving::ReceivingSettlement::new(control,before,crate::SqliteSnapshotPhase::$phase);let mut observer=|event:$progress|settlement.observe(event.owned_bytes,event.completed,event.total);
    let turn=self.scoped_native(maximum,&mut observer,|owner|owner.receive::<F,O,ValueError>(|slot,native,body|{
     let before=settlement.with_control(|control|control.performed_work());
     let result=(||{
      let owned=settlement.with_control(|control|{let mut port=crate::artifact::receiving::Port{control,native:crate::artifact::receiving::Direction::$direction(native),body};port.work(1,std::mem::size_of::<F>())?;*slot=Some(construct());let frame=slot.as_mut().unwrap();port.work(1,std::mem::size_of::<O>())?;let status=operation(frame,&mut port);*frame.receiving_status()=Some(status);Ok::<_,ValueError>(port.owned())})?;
      settlement.finish_body(owned)?;let frame=slot.as_mut().unwrap();if frame.receiving_status().as_ref().is_some_and(Result::is_ok){native.checkpoint()?;}frame.receiving_status().take().unwrap()?;publish(frame)
     })();
     RetainedWorkOutcome{work:settlement.with_control(|control|control.performed_work()).checked_sub(before).expect("original SQL operation counter is monotonic"),result}
    }));work=turn.work;
    turn.result.map_err(|failure|match settlement.take_refusal(self.native().owned_bytes()){None=>failure,Some(refusal)=>failure.with_settlement_refusal(refusal).expect("value-only SQLite body produces a fresh settlement channel")})
   })();RetainedWorkOutcome{result:result.map_err(|error|error.with_retained_progress(self.progress())),work}`;
 s=s.slice(0,start)+body+s.slice(end);fs.writeFileSync(p,s);
}
