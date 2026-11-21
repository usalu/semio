import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const src=join(process.cwd(),"🧰️framework","🛍️products","🦑️repo","🔨️modules","🤝️shared-worktree","🔁️replica","🟦️.ts");
const { writeHintV1 }=await import(pathToFileURL(join(src,"..","..","🎣️tiers","🟦️.ts")).href);
const { closeReplicaV1, followOnceV1, joinV1, openReplicaV1, publishOnceV1, runV1, seedV1 }=await import(pathToFileURL(src).href);
const artifacts=join(import.meta.dir,"🗑️generated");
mkdirSync(artifacts,{recursive:true});
const scratch=mkdtempSync(join(artifacts,"ot-"));
const [store,ar,br,code]=["s","a","b","c"].map(name=>join(scratch,name));
for(const dir of [store,ar,br,code])mkdirSync(dir);
const options=(worktree,originId)=>({worktree,originId,storeRoot:store,groupId:"g1",pollMilliseconds:20,debounceMilliseconds:0,scanSeconds:3600,walkSeconds:3600,sweepMilliseconds:3600000,watch:false,codeRoot:code});
const a=options(ar,"a"),b=options(br,"b");
const controller=new AbortController();
let source,receiver,run,turns=0;
const seen=[];
const until=async(predicate,description)=>{
 const end=Date.now()+6000;
 while(!predicate()){
  if(Date.now()>=end)throw Error(description+" was not observed: "+JSON.stringify({turns,events:seen.slice(-12)}));
  await Bun.sleep(30);
 }
};
const read=path=>existsSync(path)?readFileSync(path,"utf8"):null;
try{
 writeFileSync(join(ar,".gitignore"),".cache/\n");
 await seedV1(a);
 await joinV1(b);
 source=await openReplicaV1(a);
 run=runV1({...b,opened:p=>{receiver=p;},storeGate:async()=>{turns++;}},controller.signal,event=>seen.push(event));
 await until(()=>turns>=2,"initial daemon turns");
 writeFileSync(join(ar,"before.txt"),"healthy inbound before the outbound fault");
 await publishOnceV1(source,undefined,["before.txt"]);
 await until(()=>read(join(br,"before.txt"))==="healthy inbound before the outbound fault","healthy inbound baseline");
 const segments=join(store,"v1","origins","b","segments");
 rmSync(segments,{recursive:true,force:true});
 writeFileSync(segments,"blocks writes from b only");
 writeFileSync(join(br,"local.txt"),"outbound work");
 await writeHintV1(br,["local.txt"]);
 const failures=()=>receiver.failures.summary().find(row=>row.kind==="round")?.count??0;
 await until(()=>failures()>=1,"first source publish exception");
 const inbound=join(br,"after.txt");
 writeFileSync(join(ar,"after.txt"),"inbound work after the outbound fault");
 const published=await publishOnceV1(source,undefined,["after.txt"]);
 assert.equal(published.published,1);
 await until(()=>failures()>=3,"repeated source publish exceptions");
 console.log("[DEBUG] daemon",JSON.stringify({turns,failures:failures(),inbound:read(inbound),fault:seen.filter(event=>event.event==="error").map(event=>event.message)}));
 assert.equal(read(inbound),process.argv.includes("--expect-starved")?null:"inbound work after the outbound fault");
 controller.abort();
 await run;
 run=undefined;
 const follow=await openReplicaV1(b);
 try{await followOnceV1(follow);}finally{await closeReplicaV1(follow);}
 assert.equal(read(inbound),"inbound work after the outbound fault");
 assert.equal(read(segments),"blocks writes from b only");
 console.log("[DEBUG] direct tail used the same store and origin; inbound materialized with outbound path still blocked");
}finally{
 controller.abort();
 try{await run;}finally{if(source)await closeReplicaV1(source);rmSync(scratch,{recursive:true,force:true});}
}
