import {join} from "node:path";
import {randomBytes} from "node:crypto";
import {pathToFileURL} from "node:url";
/** 🎟️ Queues exclusively for `native-fixed-image:<image directory>` in the CONTROL lease directory `.cache/b/c` (where the lease image's own image lease lives, not in the lease image it protects), prints QUEUED, WAITING <s>, LOCKED, holds until stdin closes; SIGTERM while waiting withdraws the ticket. Usage: bun hold-control.ts <root> <image directory> [--limit=<seconds>] [--control=<control lease directory>] */
const root=process.argv[2],image=process.argv[3],controlArgument=process.argv.find(value=>value.startsWith("--control=")),control=controlArgument?controlArgument.slice(10):join(root,".cache/b/c"),limitArgument=process.argv.find(value=>value.startsWith("--limit=")),limit=limitArgument?Number(limitArgument.slice(8))*1000:0;
if(!root||!image){console.error("usage: hold-control.ts <root> <image directory> [--limit=<seconds>]");process.exit(2);}
const leases=await import(pathToFileURL(join(root,"🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts")).href),controller=new AbortController();let held:{release():void}|undefined;
const done=(code:number)=>{try{held?.release();}catch{}process.exit(code);};
process.on("SIGTERM",()=>{controller.abort(new Error("withdrawn"));if(held)done(0);});process.on("SIGINT",()=>{controller.abort(new Error("withdrawn"));if(held)done(0);});
const started=Date.now();
try{
 const timer=limit>0?setTimeout(()=>controller.abort(new Error("limit reached")),limit):undefined;console.log("QUEUED");
 held=await leases.acquireQueuedResourceLease({directory:control,resource:"native-fixed-image:"+image,mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:controller.signal,onWait:()=>console.log("WAITING "+Math.round((Date.now()-started)/1000))});
 if(timer)clearTimeout(timer);
}catch(error){console.log("FAILED "+String((error as Error).message).split("\n")[0]);process.exit(3);}
console.log("LOCKED "+Math.round((Date.now()-started)/1000));
process.stdin.on("end",()=>done(0));process.stdin.on("close",()=>done(0));process.stdin.resume();
