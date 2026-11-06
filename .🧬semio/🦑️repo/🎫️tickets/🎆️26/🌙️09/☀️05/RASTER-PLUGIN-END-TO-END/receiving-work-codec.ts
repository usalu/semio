import {readFileSync,writeFileSync} from 'node:fs';
const files:[string,[string,string][]][]=[
 ['🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧾️result/🛬️receiving/🦀️.rs',[
  ['.or(self.allocation_refusal.as_ref()).or(allocation)', '.or(self.allocation_refusal.as_ref()).or(self.work_refusal.as_ref()).or(allocation)'],
 ]],
 ['🧰️framework/🔨️modules/🧵️job/⚠️failure/🫴️projection/🦀️.rs',[
  ['usize::from(r.allocation_refusal.is_some())),Self::Value','usize::from(r.allocation_refusal.is_some())+usize::from(r.work_refusal.is_some())),Self::Value'],
 ]],
 ['🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/🔁️codec/🦀️.rs',[
  ['Some(NativeSnapshotCause::Receiving(value))=>object(control,8,','Some(NativeSnapshotCause::Receiving(value))=>object(control,9,'],
  ['field(entries,"allocationRefusal",field_value,control)?;let field_value=match &value.allocation_failure','field(entries,"allocationRefusal",field_value,control)?;let field_value=optional_error(value.work_refusal.as_ref(),control)?;field(entries,"workRefusal",field_value,control)?;let field_value=match &value.allocation_failure'],
  ['let[_,primary,quote,allocation,native_allocation,allocation_failure,receipt_refusal,progress]=fields(cause,["type","primary","quoteRefusal","allocationRefusal","nativeAllocation","allocationFailure","receiptRefusal","progress"])?','let[_,primary,quote,allocation,work_refusal,native_allocation,allocation_failure,receipt_refusal,progress]=fields(cause,["type","primary","quoteRefusal","allocationRefusal","workRefusal","nativeAllocation","allocationFailure","receiptRefusal","progress"])?'],
  ['receive_error(allocation,&mut cause.allocation_refusal,&mut frame.pending,native,body,work)?;','receive_error(allocation,&mut cause.allocation_refusal,&mut frame.pending,native,body,work)?;receive_error(work_refusal,&mut cause.work_refusal,&mut frame.pending,native,body,work)?;'],
 ]],
];for(const [file,changes] of files){let text=readFileSync(file,'utf8');for(const [before,after] of changes){if(!text.includes(before))throw new Error(`changed ${file}: ${before}`);text=text.replace(before,after)}writeFileSync(file,text)}
