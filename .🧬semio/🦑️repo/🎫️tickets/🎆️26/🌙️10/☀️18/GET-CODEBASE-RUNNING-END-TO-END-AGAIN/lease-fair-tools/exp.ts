import {mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
const [role]=process.argv.slice(2);
const LEASES=process.env.LEASES!;
const api=await import(LEASES);
if(role==="worker"){
 const [,kind,directory,resource,apiKind,holdMs,untilMs,offsetMs]=process.argv.slice(2);
 const mode=kind==="writer"?"exclusive":"shared";
 const stop=Date.now()+Number(untilMs);const controller=new AbortController();
 await Bun.sleep(Number(offsetMs));
 let n=0,worst=0;const t0=Date.now();
 try{
  while(Date.now()<stop){
   const s=Date.now();
   const lease=await (apiKind==="queued"?api.acquireQueuedResourceLease({directory,resource,mode,owner:kind+process.pid,signal:controller.signal,sharedCohort:mode==="shared",timeoutMs:Number(untilMs)}):api.acquireResourceLease({directory,resource,mode,signal:controller.signal,timeoutMs:Number(untilMs)}));
   const w=Date.now()-s;worst=Math.max(worst,w);n++;
   if(kind==="writer"){console.log(JSON.stringify({kind,acquiredAfterMs:w}));lease.release();break;}
   await Bun.sleep(Number(holdMs));lease.release();
  }
  console.log(JSON.stringify({kind,done:true,acquisitions:n,worstWaitMs:worst,cpuMs:Math.round((process.cpuUsage().user+process.cpuUsage().system)/1000)}));
 }catch(e){console.log(JSON.stringify({kind,error:String(e).slice(0,80),acquisitions:n,cpuMs:Math.round((process.cpuUsage().user+process.cpuUsage().system)/1000)}));}
 process.exit(0);
}
const [,apiKind,readers,holdMs,untilMs]=process.argv.slice(2);
const dir=mkdtempSync(join(process.env.TMPDIR??"/tmp","lf-"));
const R=Number(readers),H=Number(holdMs);
const kids:any[]=[];
const spawn=(kind:string,offset:number)=>{const p=Bun.spawn([process.execPath,import.meta.path,"worker",kind,dir,"res",apiKind,String(H),String(untilMs),String(offset)],{env:process.env,stdout:"pipe",stderr:"inherit"});kids.push(p);return p;};
for(let i=0;i<R;i++)spawn("reader",Math.floor(i*H/R));
const w=spawn("writer",500);
const outs=await Promise.all(kids.map(async p=>{const t=await new Response(p.stdout).text();await p.exited;return t.trim().split("\n");}));
console.log(apiKind,"readers",R,"hold",H);for(const o of outs)for(const l of o)console.log(l);
rmSync(dir,{recursive:true,force:true});
