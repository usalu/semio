import {mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
const role=process.argv[2];
const api=await import(process.env.LEASES!);
if(role==="holder"){
 const [,,,directory,resource,ms]=process.argv;
 const l=await api.acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"holder",signal:new AbortController().signal});
 console.log("held");await Bun.sleep(Number(ms));l.release();process.exit(0);
}
if(role==="waiter"){
 const [,,,directory,resource,ms,i]=process.argv;
 try{await api.acquireQueuedResourceLease({directory,resource,mode:"shared",sharedCohort:true,owner:"w"+i,signal:new AbortController().signal,timeoutMs:Number(ms),onWait:()=>{}});console.log(JSON.stringify({acquired:true}));}
 catch(e){const u=process.cpuUsage();console.log(JSON.stringify({cpuMs:Math.round((u.user+u.system)/1000)}));}
 process.exit(0);
}
const [,,,waiters,ms]=process.argv;
const dir=mkdtempSync(join(process.env.TMPDIR!,"lf2-"));
const holder=Bun.spawn([process.execPath,import.meta.path,"holder",dir,"res",String(Number(ms)+3000)],{env:process.env,stdout:"pipe"});
const r=holder.stdout.getReader();await r.read();
const ws=[];for(let i=0;i<Number(waiters);i++){ws.push(Bun.spawn([process.execPath,import.meta.path,"waiter",dir,"res",ms,String(i)],{env:process.env,stdout:"pipe"}));await Bun.sleep(30);}
let total=0;for(const w of ws){const t=(await new Response(w.stdout).text()).trim();try{total+=JSON.parse(t).cpuMs??0}catch{console.log("raw:",t)}}
console.log("waiters",waiters,"waitMs",ms,"totalWaiterCpuMs",total,"perWaiterCpuMs",Math.round(total/Number(waiters)));
holder.kill();await holder.exited;rmSync(dir,{recursive:true,force:true});
