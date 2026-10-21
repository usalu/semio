// 🧪 Runs the live project assembly for chosen projects (by manifest directory) and reports each target's resolution or the refusal; the cheap stand-in for a selected-scope graph build.
import {readFileSync} from "node:fs";
import {join} from "node:path";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const assembly:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"));
const contracts=JSON.parse(readFileSync(join(L,"🔣️taxonomy.json"),"utf8")).generatorContracts;
const scripts=inference.createScriptInputCache({workspaceRoot:W}),facts=new Map();
const owner:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"));scripts.generatedOutput=(path:string)=>owner.readBuildGeneratedCompileOutputV1(path,W,contracts,undefined);
for(const root of process.argv.slice(2)){
 const json=JSON.parse(readFileSync(join(W,root,"📋️project.json"),"utf8")),t0=performance.now();
 try{const result=await assembly.projectWithDefaults(json,root,join(W,root),W,contracts,facts,undefined,scripts,undefined,"source");
  const own=Object.entries<any>(result.targets).filter(([n,t])=>(t.dependsOn??[]).some((d:any)=>d===json.name+":"+n)).map(([n])=>n);
  console.log(JSON.stringify({project:json.name,ok:true,targets:Object.keys(result.targets).length,selfDependent:own,ms:Math.round(performance.now()-t0)}));}
 catch(error:any){console.log(JSON.stringify({project:json.name,ok:false,error:String(error.message).slice(0,300),stack:String(error.stack).split("\n").slice(1,6).map(l=>l.trim().replace(W+"/","").slice(0,200))}));}
}
process.exit(0);
