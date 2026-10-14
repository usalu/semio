import {resolve,join} from "node:path";
import {writeFile} from "node:fs/promises";
const root=process.cwd(),ticket=resolve(import.meta.dir,".."),started=performance.now(),limit=512*1024*1024;
const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const graph=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts"));
const rows:unknown[]=[],policy={version:1 as const,owner:"build:inventory-proof",maximumElapsedMilliseconds:60000};let completed=false,peak=0,files:string[]=[];
await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope(policy,{},Date.now()),async original=>{
 const invocation={...original,control:{...original.control,publish:async(event:any)=>{
  await original.control.publish(event);routing.checkScriptInvocation(original);const memory=process.memoryUsage(),elapsedMilliseconds=Math.round(performance.now()-started);peak=Math.max(peak,memory.rss);const row={command:event.command,stage:event.stage,elapsedMilliseconds,rss:memory.rss};rows.push(row);console.log("[DEBUG] Inventory-only progress "+JSON.stringify(row));if(memory.rss>limit)throw Error("Inventory-only proof exceeded 512MiB RSS");if(elapsedMilliseconds>60000)throw Error("Inventory-only proof exceeded 60s");if(event.command==="build:discover"&&event.stage==="complete")completed=true;
 }}};files=await graph.workspaceBuildFilesV1(root,invocation);
});
const result={completedInventory:completed,files:files.length,completedGraph:false,stoppedBeforeInference:true,elapsedMilliseconds:Math.round(performance.now()-started),peakRss:peak,progressEvents:rows.length,rows};await writeFile(join(ticket,"🗑️generated/inventory-proof.json"),JSON.stringify(result,null,2)+"\n");console.log("[DEBUG] Inventory-only result "+JSON.stringify({...result,rows:undefined}));
