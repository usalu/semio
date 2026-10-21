// 📏 Runs the live project assembly with the source-contract step bypassed (sourceGroups given) and reports how many input references the normalized targets carry, the figure the closure walk counts against its bound.
import {readFileSync} from "node:fs";
import {join} from "node:path";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const assembly:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"));
const contracts=JSON.parse(readFileSync(join(L,"🔣️taxonomy.json"),"utf8")).generatedContracts??JSON.parse(readFileSync(join(L,"🔣️taxonomy.json"),"utf8")).generatorContracts;
const owner:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"));
const scripts=inference.createScriptInputCache({workspaceRoot:W});scripts.generatedOutput=(path:string)=>owner.readBuildGeneratedCompileOutputV1(path,W,contracts,undefined);
for(const root of process.argv.slice(2)){
 const json=JSON.parse(readFileSync(join(W,root,"📋️project.json"),"utf8")),t0=performance.now();
 try{const result=await assembly.projectWithDefaults(json,root,join(W,root),W,contracts,new Map(),undefined,scripts,{},"source");
  const rows=Object.entries<any>(result.targets).map(([n,t])=>({n,len:(t.inputs??[]).length,literal:(t.inputs??[]).filter((i:any)=>typeof i!=="string"||i.startsWith("{")||i.startsWith("!")).length})).sort((a,b)=>b.len-a.len);
  const sample=result.targets[process.env.IP_TARGET??rows[0].n]?.inputs??[];console.log("SAMPLE",JSON.stringify([sample.slice(0,6),sample.slice(-4)]).slice(0,900));console.log(JSON.stringify({project:json.name,targets:rows.length,totalRefs:rows.reduce((s,r)=>s+r.len,0),top:rows.slice(0,5),ms:Math.round(performance.now()-t0)}));}
 catch(error:any){console.log(JSON.stringify({project:json.name,ok:false,error:String(error.message).slice(0,200),stack:String(error.stack).split("\n").slice(1,7).map((l:string)=>l.trim().replace(W+"/","").slice(0,170))}));}
}
process.exit(0);
