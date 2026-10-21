// 🧪 Evaluates the generated-task dependencies of one manifest target before and after a metadata change, with the production script-input analysis.
import {readFileSync} from "node:fs";
import {join} from "node:path";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const [root,name]=process.argv.slice(2) as [string,string];
const json=JSON.parse(readFileSync(join(W,root,"📋️project.json"),"utf8")),target=json.targets[name];
const nx=JSON.parse(readFileSync(join(W,"nx.json"),"utf8")).namedInputs??{};
const expand=(inputs:any[],seen:string[]=[]):any[]=>inputs.flatMap(i=>typeof i==="string"&&!i.startsWith("{")&&!i.startsWith("!")&&!i.startsWith("^")&&(json.namedInputs?.[i]||nx[i])&&!seen.includes(i)?expand(json.namedInputs?.[i]??nx[i],[...seen,i]):[i]);
const entries={...target,inputs:expand(target.inputs??[])};
if(process.env.IP_NO_INPUTS)entries.inputs=[];
if(process.env.IP_FRESHNESS){entries.metadata=JSON.parse(JSON.stringify(entries.metadata??{}));entries.metadata.semio={...entries.metadata.semio,sourceInputObservation:'freshness'};}
if(process.env.IP_NO_FRESHNESS){entries.metadata=JSON.parse(JSON.stringify(entries.metadata??{}));delete entries.metadata.semio?.sourceInputObservation;}
console.log(Object.keys(inference.cacheInternals).filter(k=>/ScriptSources|generated/i.test(k)).join(","));
const owner:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs")),contracts=JSON.parse(readFileSync(join(L,"🔣️taxonomy.json"),"utf8")).generatorContracts;
const scripts=inference.createScriptInputCache({workspaceRoot:W});scripts.generatedOutput=(path:string)=>{const produced=owner.readBuildGeneratedCompileOutputV1(path,W,contracts,undefined);if(produced&&process.env.IP_TRACE)console.log('GENERATED',path.replace(W+'/',''),produced.target,new Error().stack!.split('\n').slice(2,5).map(l=>l.trim().replace(W+'/','')).join(' < '));return produced;};
const fn=inference.cacheInternals.targetScriptSources;
if(!fn){console.log("targetScriptSources not exported");process.exit(1);}
try{const result=await fn(entries,W,scripts,root,undefined);console.log(JSON.stringify({generatedDependencies:result.generatedDependencies,closure:result.closure?.length,importers:[...scripts.generatedImports.keys()].map((p:string)=>p.replace(W+'/',''))}));}catch(error:any){console.log("ERR",String(error.message).slice(0,300));}
process.exit(0);
