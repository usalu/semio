import {join} from "node:path";
import {pathToFileURL} from "node:url";

/** 🔒️ Takes the given resource leases exclusively without waiting, through the repository's own leases API, prints `LOCKED` once it holds all of them, holds until its stdin closes or it is terminated, then releases them in reverse. Prints `BUSY <resource>: <reason>` and exits 3 when any lease is taken or the lease directory is not available; nothing is ever waited for. Usage: bun lease-hold.ts <root> [--directory=<lease directory>] <resource>... */
const root=process.argv[2],rest=process.argv.slice(3),given=rest.find(value=>value.startsWith("--directory=")),resources=rest.filter(value=>!value.startsWith("--"));
if(!root||!resources.length){console.error("usage: lease-hold.ts <root> [--directory=<dir>] <resource>...");process.exit(2);}
const library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),leases=await import(pathToFileURL(join(root,"🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts")).href),opening=await import(pathToFileURL(join(library,"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts")).href);
const held:Array<{release():void}>=[],controller=new AbortController();
const release=()=>{for(const lease of held.splice(0).reverse())try{lease.release();}catch{}};
try{
 const directory=given?given.slice("--directory=".length):opening.repositoryLeaseDirectoryV1(root);
 for(const resource of resources){
  try{held.push(await leases.acquireResourceLease({directory,resource,mode:"exclusive",signal:controller.signal,timeoutMs:0}));}
  catch(error){console.log("BUSY "+resource+": "+String((error as Error).message).split("\n")[0]);release();process.exit(3);}
 }
}catch(error){console.log("BUSY lease directory: "+String((error as Error).message).split("\n")[0]);release();process.exit(3);}
console.log("LOCKED");
const done=()=>{release();process.exit(0);};
process.on("SIGTERM",done);process.on("SIGINT",done);process.stdin.on("end",done);process.stdin.on("close",done);process.stdin.resume();
