import * as fs from "node:fs";
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🔌️extension-invocation-and-command-ingress/🦀️.rs";
const before=fs.readFileSync(file,"utf8");let text=before;
function region(start:string,end:string,body:string){const a=text.indexOf(start),b=text.indexOf(end,a);if(a<0||b<a)throw Error("owned command return region changed");text=text.slice(0,a)+body+text.slice(b)}
region("pub fn step_retiring_command_ingress(","/// 🔌️ Advances the retained extension invocation",`pub fn step_retiring_command_ingress(cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 retire_command_slot(COMMAND_INGRESS_RETIRING_SLOT,cx)
}
fn retire_command_slot(index:usize,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 if !COMMAND_INGRESS.with(|slots|slots.borrow_mut()[index].settle_step(cx))?{return Ok(())}
 if COMMAND_INGRESS.with(|slots|slots.borrow()[index].peek().is_none()){return Ok(())}
 let mut loan=std::pin::pin!(CommandIngressLoan::empty(index));if !loan.lend(cx)?{return Ok(())}
 let result=match loan.take_state(){Some(state)=>retire_command_ingress(state,cx),None=>Ok(None)};
 match result{Ok(state)=>loan.put_state(state),Err(failure)=>{loan.clear_empty_entry(cx)?;loan.restore()?;return Err(failure)}}
 loan.clear_empty_entry(cx)?;loan.restore()
}
pub(super) fn retiring_command_ingress_is_free()->bool{COMMAND_INGRESS.with(|slots|slots.borrow()[COMMAND_INGRESS_RETIRING_SLOT].peek().is_none())}
pub(super) fn park_retiring_command_ingress(key:instance_lifetime::NativeCloseKey,state:&mut Option<CommandIngressOwner>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 if !COMMAND_INGRESS.with(|slots|slots.borrow_mut()[COMMAND_INGRESS_RETIRING_SLOT].settle_step(cx))?{return Ok(false)}
 if !retiring_command_ingress_is_free(){return Ok(false)}let mut loan=std::pin::pin!(CommandIngressLoan::empty(COMMAND_INGRESS_RETIRING_SLOT));if !loan.lend(cx)?||!loan.admit_entry(key,cx)?{return Ok(false)}
 let bytes=2*std::mem::size_of::<Option<CommandIngressOwner>>();let grant=cx.retained_grant();if cx.should_yield()||cx.fuel_remaining()<2||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
 loan.put_state(state.take());cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:2,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(2);loan.restore()?;Ok(true)
}
#[doc(hidden)]
pub fn close_command_ingress_step(key:instance_lifetime::NativeCloseKey,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 for index in 0..2{let selected=COMMAND_INGRESS.with(|slots|slots.borrow()[index].peek().is_some_and(|entry|entry.key==key));if selected{retire_command_slot(index,cx)?;}else{COMMAND_INGRESS.with(|slots|slots.borrow_mut()[index].settle_step(cx))?;}}
 Ok(COMMAND_INGRESS.with(|slots|slots.borrow().iter().all(|slot|slot.peek().is_none_or(|entry|entry.key!=key))))
}
pub fn retained_command_ingress_occupancy()->usize{COMMAND_INGRESS.with(|slots|slots.borrow().iter().filter(|slot|slot.peek().is_some()).count())}

`);
if(fs.readFileSync(file,"utf8")!==before)throw Error("concurrent command return edit");const temp=import.meta.dir+"/🗑️generated/command-return-consumer.tmp";fs.writeFileSync(temp,text);fs.renameSync(temp,file);
