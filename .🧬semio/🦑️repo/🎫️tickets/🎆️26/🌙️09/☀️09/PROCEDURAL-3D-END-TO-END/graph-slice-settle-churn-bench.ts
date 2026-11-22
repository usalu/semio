/** 🌀️ Measures one read of the owner phase under a path that flips on every discovery: old settle (whole-round conflict) against the unit settle. Run: bun graph-slice-settle-churn-bench.ts [owners] */
import {mkdtemp,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
const build=new URL("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",import.meta.url);
const owners=Number(process.argv[2]??600),limit=64;
const load=async(module:string)=>({observation:await import(new URL("👁️observation/"+module,build).href),loader:await import(new URL("🕸️graph/📥️load/🟦️.ts",build).href),authority:await import(new URL("🕸️graph/🔐️authority/🟦️.ts",build).href)});
async function run(label:string,module:string){
 const{observation,loader,authority}=await load(module),root=await mkdtemp(join(tmpdir(),"churn-bench-"));let flips=0,discoveries=0;
 try{
  await writeFile(join(root,"noise.txt"),"n");for(let at=0;at<owners;at++)await writeFile(join(root,"own-"+at+".txt"),"o"+at);
  const scope="bench-"+label+Math.random(),trust={current:new WeakSet<object>()};let rounds=0,outcome="sealed";const started=performance.now();
  try{
   await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{
    for(let at=0;at<owners;at++)await loader.observeOwnerFactsMemoizedV1({key:scope+"\0"+authority.loadedGraphAdmissionFormatV1+"\0o"+at,hold:async(rows:unknown[])=>observation.observationRowsHoldV1(rows),observation,trust:{has:(entry:object)=>trust.current.has(entry),add:(entry:object)=>trust.current.add(entry)},observe:async()=>{
     discoveries++;observation.observedFs.readFileSync(join(root,"own-"+at+".txt"),"utf8");
     if(at===0||at===owners-1){observation.observedFs.readFileSync(join(root,"noise.txt"),"utf8");if(at===0)await writeFile(join(root,"noise.txt"),"n".repeat(++flips+1));}
     return at;
    }});
    return 1;
   },{limit,onRound:(event:{round:number;settled?:true;paths?:string[]})=>{rounds=event.round+1;if(event.settled!==true&&event.paths?.length)loader.evictOwnerFactsReferencingV1(scope,event.paths);else if(event.settled!==true)trust.current=new WeakSet();}}),undefined,false);
  }catch(error){outcome="refused: "+(error as Error).message.slice(0,60);}
  console.log(JSON.stringify({label,owners,outcome,rounds,discoveries,flips,seconds:Math.round((performance.now()-started))/1000}));
 }finally{await rm(root,{recursive:true,force:true});}
}
await run("old","🟨️old-settle.mjs");
await run("new","🟨️.mjs");
