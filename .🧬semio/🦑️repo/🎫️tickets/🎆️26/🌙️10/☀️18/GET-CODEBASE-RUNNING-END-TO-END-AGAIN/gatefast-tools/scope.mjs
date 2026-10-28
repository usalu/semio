/** 🎯 Read-only: for the slice of one memo plan file, how many derived questions the planner's cutoff can re-ask (pseudo owners in the selected classes) versus how many have a holder inside the slice. `bun scope.mjs <root> <plan-file>...` (each plan file names its slice by `projects` + `unavailable`). Uses the live generation modules (`selectFactClassesV1`, `scopeWithDerivedV1`) exactly as `plan()` does. */
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
const [root,...plans]=process.argv.slice(2);
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations");
const codec=await import(join(L,"🧮️fact-codec/🟨️.mjs")),generation=await import(join(L,"🟨️.mjs")),cutoff=await import(join(L,"✂️cutoff/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const index=codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const pseudo=new Set();index.owners.forEach((name,id)=>{if(cutoff.isDerivedOwnerV1(name))pseudo.add(id);});
for(const file of plans){
 const plan=JSON.parse(readFileSync(file,"utf8")),owners=new Set([...Object.keys(plan.projects),...(plan.unavailable??[]).map(row=>row.project)]);
 const scope=cutoff.scopeWithDerivedV1(index,new Set([...owners,generation.GENERATION_POLICY_V1.envOwner])),selection=generation.selectFactClassesV1(index,scope);
 const reachable=new Set();for(const at of selection.classes)for(const o of index.classes[at].o)if(pseudo.has(o))reachable.add(index.owners[o]);
 const holders=cutoff.holdersOfDerivedV1(index,reachable);let inside=0,outside=0;
 for(const set of holders.values()){if([...set].some(name=>owners.has(name)))inside++;else outside++;}
 const inScope=[...scope].filter(name=>cutoff.isDerivedOwnerV1(name)).length;
 console.log(JSON.stringify({plan:file.split("/").pop().slice(0,12),generation:cur.generation,sliceOwners:owners.size,selectedClasses:selection.classes.length,selectedRows:selection.ids.length,pseudoInScope:inScope,suspectsReachable:reachable.size,withHolderInSlice:inside,withoutHolderInSlice:outside,pseudoTotal:pseudo.size}));
}
console.log("rss MB",Math.round(process.memoryUsage().rss/2**20));
