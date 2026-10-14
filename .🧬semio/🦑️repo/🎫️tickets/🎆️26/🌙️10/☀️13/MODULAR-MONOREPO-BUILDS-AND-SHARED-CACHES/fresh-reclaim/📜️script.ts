import {lstat,readdir,rename,unlink,rmdir} from "node:fs/promises";
import {readFileSync,existsSync} from "node:fs";
import {join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {receiveScriptProcessInvocation} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1,compilerAdmissionBudgetV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {acquireQueuedResourceLease} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {acquireCargoBuildLeaseV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
const workspace=resolve(import.meta.dir,"../../../../../../../.."),generation=join(workspace,".🧬semio/🦑️repo/⚡️cache/fChPzBn"),store=join(generation,"t");
await receiveScriptProcessInvocation(process.env,async invocation=>{
 const control=scriptCompilerAdmissionControlV1(invocation,"cache:private-generation",300000),leases=join(workspace,".cache/framework/leases"),held=[] as {release():void}[];
 try{
  const status=readFileSync(join(workspace,".🧬semio/🦑️repo/⚡️cache/play-fleet/coordinator/fresh-10.status"),"utf8"),log=readFileSync(join(workspace,".🧬semio/🦑️repo/⚡️cache/play-fleet/coordinator/fresh-10.log"),"utf8");
  if(!status.includes("exit 1 2026-10-13 02:01:08")||!log.includes(`Fresh play release cache: ${generation}`))throw Error("Private generation ownership/exit evidence refused");
  if((await lstat(generation)).isSymbolicLink()||(await lstat(store)).isSymbolicLink()||Date.now()-(await lstat(store)).mtimeMs<3600000)throw Error("Private generation storage age/identity refused");
  held.push(await acquireQueuedResourceLease({directory:leases,resource:"cache-prune",mode:"exclusive",signal:control.signal,control}));
  for(const profile of ["debug","release","prototype","wasm-prototype","wasm-release"])held.push(await acquireCargoBuildLeaseV1({directory:leases,buildDirectory:store,args:["--profile",profile],signal:control.signal,control}));
  compilerAdmissionBudgetV1(control);
  const processes=spawnSync("ps",["eww","-axo","pid,command"],{encoding:"utf8"});if(processes.status!==0||processes.stdout.split("\n").some(line=>line.includes(generation)))throw Error("Private generation still belongs to a live process");
  const opened=spawnSync("lsof",["-nP","+D",store],{encoding:"utf8"});if(opened.status!==1||opened.stdout.trim())throw Error("Private generation has open descriptors or inspection failed");compilerAdmissionBudgetV1(control);
  const provenance=join(store,"semio-cargo-provenance");if(existsSync(provenance))await rename(provenance,join(generation,"provenance"));
  let files=0,bytes=0,last=0;
  const remove=async(path:string):Promise<void>=>{compilerAdmissionBudgetV1(control);const entry=await lstat(path);if(entry.isDirectory()&&!entry.isSymbolicLink()){for(const name of await readdir(path))await remove(join(path,name));await rmdir(path);}else{bytes+=entry.size;await unlink(path);files++;}if(Date.now()-last>1000){last=Date.now();console.log(`[DEBUG] Reclaimed private fresh generation files=${files} bytes=${bytes}`);await control.publish({resource:"retired-private-generation",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}};
  await remove(store);console.log(`[DEBUG] Private fresh generation completed=${JSON.stringify({generation,files,bytes,retained:["p.json","o.log","d","n","provenance"],removed:"t",liveReferences:0,openDescriptors:0})}`);
 }finally{for(const lease of held.reverse())lease.release();}
});
