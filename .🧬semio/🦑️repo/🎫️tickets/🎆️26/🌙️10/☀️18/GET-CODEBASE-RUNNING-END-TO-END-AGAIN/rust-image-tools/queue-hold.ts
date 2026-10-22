import {join} from "node:path";
import {randomBytes} from "node:crypto";
import {pathToFileURL} from "node:url";

/** 🎟️ Queues for the given resource lease exclusively through the repository's own queued leases API (FIFO: every later shared opener queues behind this ticket, nobody holding the resource is ever interrupted), prints `QUEUED` once the ticket stands, `WAITING <seconds>` while it waits and `LOCKED` once it is granted, holds until stdin closes or it is terminated, then releases. SIGTERM/SIGINT while waiting withdraws the ticket. Prints `FAILED <reason>` and exits 3 when the lease directory is unavailable. Usage: bun queue-hold.ts <root> <resource> [--limit=<seconds>] */
const root=process.argv[2],resource=process.argv[3],limitArgument=process.argv.find(value=>value.startsWith("--limit=")),limit=limitArgument?Number(limitArgument.slice(8))*1000:0;
if(!root||!resource){console.error("usage: queue-hold.ts <root> <resource> [--limit=<seconds>]");process.exit(2);}
const library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),leases=await import(pathToFileURL(join(root,"🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts")).href),opening=await import(pathToFileURL(join(library,"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts")).href);
const controller=new AbortController();let held:{release():void}|undefined;
const done=(code:number)=>{try{held?.release();}catch{}process.exit(code);};
process.on("SIGTERM",()=>{controller.abort(new Error("withdrawn"));if(held)done(0);});process.on("SIGINT",()=>{controller.abort(new Error("withdrawn"));if(held)done(0);});
const started=Date.now();let announced=false;
try{
 const directory=opening.repositoryLeaseDirectoryV1(root);
 const timer=limit>0?setTimeout(()=>controller.abort(new Error("limit reached")),limit):undefined;
 console.log("QUEUED");
 held=await leases.acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:controller.signal,onWait:()=>{if(!announced){announced=true;}console.log("WAITING "+Math.round((Date.now()-started)/1000));}});
 if(timer)clearTimeout(timer);
}catch(error){console.log("FAILED "+String((error as Error).message).split("\n")[0]);process.exit(3);}
console.log("LOCKED "+Math.round((Date.now()-started)/1000));
process.stdin.on("end",()=>done(0));process.stdin.on("close",()=>done(0));process.stdin.resume();
