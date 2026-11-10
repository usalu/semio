import{mkdtempSync}from"node:fs";import{join}from"node:path";import{tmpdir}from"node:os";
import{acquireResourceLease,acquireFreshResourceLeaseV1}from"@semio-tech/process/leases";
const directory=mkdtempSync(join(tmpdir(),"fr-"));
const ctl=()=>({directory,resource:"r1",mode:"exclusive",owner:Math.random().toString(36).slice(2),signal:new AbortController().signal,timeoutMs:0});
const a=await acquireFreshResourceLeaseV1(ctl() as never);console.log("[DEBUG] first",!!a);
try{const b=await acquireFreshResourceLeaseV1(ctl() as never);console.log("[DEBUG] second got lease (BUG)",!!b);}catch(e){console.log("[DEBUG] second refused:",String(e.message).slice(0,100));}
try{const c=await acquireResourceLease(ctl() as never);console.log("[DEBUG] try third got lease (BUG)",!!c);}catch(e){console.log("[DEBUG] third refused:",String(e.message).slice(0,100));}
process.exit(0)
