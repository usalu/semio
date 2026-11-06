import fs from"node:fs";const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs";const old=fs.readFileSync(file,"utf8");let next=old;
const start="    let original_progress=cx.retained_progress();let original_work=cx.original_work_units_spent();";
if(!next.includes(start))throw Error("entry seam");next=next.replace(start,start+"\n    let reserved_progress=cx.reserved_return_progress();let reserved_work=cx.reserved_return_work();");
const receipt='    let retained_receipt=budget.retained.return_original(original_turn_progress(original_progress,cx.retained_progress())?,u64::try_from(cx.original_work_units_spent()-original_work).map_err(|_|reactor_close_fault("original turn work exceeds fixed receipt extent"))?)?;';
if(!next.includes(receipt))throw Error("receipt seam");next=next.replace(receipt,`    let fuel_used=u64::try_from(cx.original_work_units_spent().checked_sub(original_work).ok_or_else(||reactor_close_fault("original kernel work observer moved backward"))?).map_err(|_|reactor_close_fault("original turn work exceeds fixed receipt extent"))?;
    let reserved_work=cx.reserved_return_work().checked_sub(reserved_work).ok_or_else(||reactor_close_fault("reserved kernel work observer moved backward"))?;
    let retained_receipt=kernel_work_receipt(budget.retained,original_turn_progress(original_progress,cx.retained_progress())?,fuel_used,original_turn_progress(reserved_progress,cx.reserved_return_progress())?,reserved_work)?;`);
if(!next.includes("status, fuel_used: 0, command_ingress"))throw Error("fuel field");next=next.replace("status, fuel_used: 0, command_ingress","status, fuel_used, command_ingress");
const helper=`
/// 🧮️ Keeps the nested input receipt fresh-conserved; fixed Core receipts carry the authenticated aggregate.
fn kernel_work_receipt(input:semio_framework::kernel::RetainedTurnInput,aggregate:semio_framework_value::RetainedCloneProgress,work:u64,reserved:semio_framework_value::RetainedCloneProgress,reserved_work:u64)->Result<semio_framework::kernel::RetainedTurnReceipt,semio_framework_job::OriginalTurnFailure>{
    let fresh=original_turn_progress(reserved,aggregate)?;
    let work=work.checked_sub(reserved_work).ok_or_else(||reactor_close_fault("reserved kernel work exceeds its observed aggregate"))?;
    Ok(input.return_original(fresh,work)?)
}
`;next=next.replace("\n#[cfg(test)]\nmod receipt_tests",helper+"\n#[cfg(test)]\nmod receipt_tests");
if(fs.readFileSync(file,"utf8")!==old)throw Error("stale");fs.writeFileSync(import.meta.dir+"/🗑️generated/kernel-receipt.rs",next);fs.renameSync(import.meta.dir+"/🗑️generated/kernel-receipt.rs",file);
fs.appendFileSync(import.meta.dir+"/source-native-custody-and-encoding-2026-11-05.md","\n### Kernel Work Receipt\nThe kernel now snapshots the real Source reserved observers around its reducer. Its nested RetainedTurnReceipt subtracts only the observed in-kernel reserved redemption and remains valid under its original fresh-input conservation validator. TurnResult.fuel_used records the actual aggregate reducer work before output capture/encoding/header processing. The independently authenticated fixed Core receipt continues to carry the full per-loan aggregate. Schema/AJV/SQLite model: 1 test / 8 assertions passed; authored native conservation law is uncompiled.\n");
console.log("Applied Source-observed fresh kernel receipt and actual reducer fuel.");
