import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),f="🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs",s=readFileSync(f,"utf8");let n=s;function rep(a:string,b:string){if(n.split(a).length!==2)throw Error("anchor "+a.slice(0,100));n=n.replace(a,b)}
rep("enum OriginalDemand{Native,Shared}","#[derive(Clone,Copy)]enum OriginalDemand{Native,Shared}");
rep("admit:fn(OriginalSlot<'_,T>,RetainedCloneGrant)->RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,RetainedCloneProgress)>>","work:fn(&T,OriginalDemand)->Result<Option<u64>,ValueError>,admit:fn(OriginalSlot<'_,T>,RetainedCloneGrant,&mut Option<usize>)->RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,RetainedCloneProgress)>>");
const a=n.indexOf("impl<T:RetireOwned+Sync> SharedIssuer<T>{"),b=n.indexOf("\n\n/// ❄️",a);if(a<0||b<0)throw Error("issuer");
n=n.slice(0,a)+`impl<T:RetireOwned+Sync> SharedIssuer<T>{
 pub fn owned()->Self{Self{
  transfer_copy_bytes:|_|size_of::<T>(),
  demand:|value,mode|match mode{OriginalDemand::Native=>native_payload_demand(value).map(Some),OriginalDemand::Shared=>{if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"sealed original has no controlled shared issuer"))}value.shared_original_demands()}},
  work:|value,mode|match mode{OriginalDemand::Native=>{native_payload_demand(value)?;Ok(Some(retirement::retirement_cursor_birth_work()))},OriginalDemand::Shared=>value.shared_original_work_demand()},
  admit:|slot,grant,actual|match slot{OriginalSlot::Shared(slot)=>T::shared_original_retirement(slot,grant,actual),OriginalSlot::Native(slot)=>{let mut target=None;let turn=retirement::admit_retirement_cursor(slot,&mut target,grant,actual);RetainedWorkOutcome{result:turn.result.map(|step|target.map(|target|(target,step.progress()))),work:turn.work}}},
 }}
}
fn native_payload_demand<T:RetireOwned>(value:&T)->Result<RetirementDemand,ValueError>{
 if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"sealed original has no native owned issuer"))}
 if !value.retirement_ready(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"sealed original requires pending recovery before retirement"))}
 retirement::retirement_cursor_birth_demand::<T>()
}
`+n.slice(b);
rep("struct Cursor<T:Send+Sync+'static>","pub struct Cursor<T:Send+Sync+'static>");
n=n.replaceAll("fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep","fn step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetirementStep").replaceAll("self.step_selected(grant,d)","self.step_selected(grant,d,actual)").replaceAll("fn step_selected(&mut self,grant:RetainedCloneGrant,demand:RetirementDemand)->","fn step_selected(&mut self,grant:RetainedCloneGrant,demand:RetirementDemand,actual:&mut Option<usize>)->").replaceAll("self.admit_cursor(grant,true)","self.admit_cursor(grant,true,actual)").replaceAll("self.admit_cursor(grant,false)","self.admit_cursor(grant,false,actual)").replaceAll("fn admit_cursor(&mut self,grant:RetainedCloneGrant,shared:bool)->","fn admit_cursor(&mut self,grant:RetainedCloneGrant,shared:bool,actual:&mut Option<usize>)->").replaceAll("(self.handle.issuer.admit)(slot,grant)","(self.handle.issuer.admit)(slot,grant,actual)").replaceAll("fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep","fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetirementStep").replaceAll("self.step(grant)","self.step(grant,actual)");
rep('Ok(None)=>RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"original issuer omitted its owned cursor").into(),turn.work)','Ok(None)=>RetirementStep::Progress(Default::default(),turn.work)');
rep("fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{handle:self,pending:ManuallyDrop::new(None),original:None})}","type Cursor=Cursor<T>;fn retirement(self)->Self::Cursor{Cursor{handle:self,pending:ManuallyDrop::new(None),original:None}}");
rep('fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.demands(body)?;if self.handle.source.is_some(){return Ok(1)}if self.original.is_none()&&self.pending.is_none(){return Ok(0)}Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"sealed original issuer lacks a work quote"))}',`fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{
  self.demands(body)?;if self.handle.source.is_some(){return Ok(1)}
  if let Some(original)=self.original.as_ref(){if(self.handle.issuer.demand)(original.get(),OriginalDemand::Shared)?.is_some(){return(self.handle.issuer.work)(original.get(),OriginalDemand::Shared)?.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original shared issuer omitted its work quote"))}return Ok(ORIGINAL_TRANSFER_WORK)}
  if let Some(value)=self.pending.as_ref(){return(self.handle.issuer.work)(value,OriginalDemand::Native)?.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original native issuer omitted its work quote"))}Ok(0)
 }`);
const q=join(t,"🗑️generated/sealed-original-issuer.rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent sealed");renameSync(q,f);console.log("Sealed issuer full physical/work quotes and actual allocator marker custody authored");
