import{acquireResourceLease,acquireFreshResourceLeaseV1}from"@semio-tech/process/leases";
const directory=process.argv[3];const req=()=>({directory,resource:"r1",mode:"exclusive",owner:Math.random().toString(36).slice(2),signal:new AbortController().signal,timeoutMs:0});
if(process.argv[2]==="hold"){await acquireFreshResourceLeaseV1(req() as never);console.log("held");await new Promise(r=>setTimeout(r,8000));process.exit(0)}
for(const[n,f]of[["try",acquireResourceLease],["fresh",acquireFreshResourceLeaseV1]] as const){try{await (f as any)(req());console.log("[DEBUG]",n,"GOT (BUG)")}catch(e:any){console.log("[DEBUG]",n,"refused",String(e.message).slice(0,80))}}
process.exit(0)
