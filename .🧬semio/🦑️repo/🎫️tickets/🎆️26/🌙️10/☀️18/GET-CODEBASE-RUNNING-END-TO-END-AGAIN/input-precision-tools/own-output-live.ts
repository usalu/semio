// 🔎 Evaluates, with the production script-input analysis, the generated-task dependencies of every producer target of an owned generator contract and lists those that name themselves (the assembly now drops them instead of refusing the graph).
import {readFileSync} from "node:fs";
import {join,dirname} from "node:path";
import {spawnSync} from "node:child_process";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const owner:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"));
const contracts=JSON.parse(readFileSync(join(L,"🔣️taxonomy.json"),"utf8")).generatorContracts,nx=JSON.parse(readFileSync(join(W,"nx.json"),"utf8")).namedInputs??{};
const manifests=new Map<string,{root:string;json:any}>();
for(const f of spawnSync("git",["-C",W,"-c","core.quotepath=false","ls-files","-z","--","*📋️project.json"],{maxBuffer:1<<28}).stdout.toString().split("\0")){if(!f||f.startsWith(".🧬semio/"))continue;try{const json=JSON.parse(readFileSync(join(W,f),"utf8"));manifests.set(json.name,{root:dirname(f),json});}catch{}}
const scripts=inference.createScriptInputCache({workspaceRoot:W});scripts.generatedOutput=(path:string)=>owner.readBuildGeneratedCompileOutputV1(path,W,contracts,undefined);
const out:any[]=[];
for(const c of Object.values<any>(contracts)){
 if(c.ownership!=="owned")continue;
 const [project,target]=[c.target.slice(0,c.target.lastIndexOf(":")),c.target.slice(c.target.lastIndexOf(":")+1)],m=manifests.get(project);
 if(!m||!m.json.targets?.[target]){out.push({id:c.target,state:"no manifest target"});continue;}
 const expand=(inputs:any[],seen:string[]=[]):any[]=>inputs.flatMap(i=>typeof i==="string"&&!/^[{!^]/u.test(i)&&(m.json.namedInputs?.[i]||nx[i])&&!seen.includes(i)?expand(m.json.namedInputs?.[i]??nx[i],[...seen,i]):[i]);
 try{const r=await inference.cacheInternals.targetScriptSources({...m.json.targets[target],inputs:expand(m.json.targets[target].inputs??["default"])},W,scripts,m.root,undefined);out.push({id:c.target,generated:r.generatedDependencies??[],self:(r.generatedDependencies??[]).includes(c.target)});}
 catch(error:any){out.push({id:c.target,error:String(error.message).slice(0,160)});}
}
for(const row of out)console.log(JSON.stringify(row));
console.log("self-dependent:",out.filter(r=>r.self).map(r=>r.id).join(", ")||"none");
process.exit(0);
