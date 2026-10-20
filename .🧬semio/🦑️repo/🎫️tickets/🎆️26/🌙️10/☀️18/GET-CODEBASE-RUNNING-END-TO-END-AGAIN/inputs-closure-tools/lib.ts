// 🧰 Shared harness setup: the shipped inference module and the staged script-inputs patch loaded side by side in one process.
import {readFileSync,existsSync,writeFileSync,mkdirSync} from "node:fs";import {join,dirname,resolve} from "node:path";
export const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),G=join(import.meta.dir,"..","🗑️generated","inputs-closure");
export const strip=(s:string)=>s.replace(/️/g,"");
export async function loadBoth(){
 const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
 const boot:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🔁️bootstrap/🟨️.mjs")),observation:any=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
 const {cacheInternals,createScriptInputCache,prepareSourceAnalysesV1}=inference;
 const original=join(L,"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
 const all=JSON.parse(Bun.spawnSync(["python3",join(import.meta.dir,"apply.py"),"dump-ops"]).stdout.toString()),ops=(all.plugin as [string,string][]).filter(([anchor])=>!process.env.INCLOS_ONLY_OPT||anchor.includes("requiredSources)readPhysicalSource"));
 let text=readFileSync(original,"utf8");for(const [anchor,replacement] of ops){if(text.includes(replacement))continue;if(text.split(anchor).length!==2)throw Error("anchor count "+anchor.slice(0,60));text=text.replace(anchor,replacement);}
 if(process.env.INCLOS_INSTRUMENT){
  text=text.replace("const references = observedSourceAnalysisV1(prepareSourceAnalysesV1().references,path);","const __a=performance.now();const references = observedSourceAnalysisV1(prepareSourceAnalysesV1().references,path);globalThis.__t={...(globalThis.__t??{}),analysis:((globalThis.__t??{}).analysis??0)+performance.now()-__a,files:((globalThis.__t??{}).files??0)+1};");
  text=text.replace("const member = workspacePackageModuleV1(path, specifier, workspaceRoot);","const __b=performance.now();const member = workspacePackageModuleV1(path, specifier, workspaceRoot);globalThis.__t.resolve=(globalThis.__t.resolve??0)+performance.now()-__b;globalThis.__t.resolves=(globalThis.__t.resolves??0)+1;");
  text=text.replace("const target = workspaceUrlTargetV1(path, literal, workspaceRoot);","const __c=performance.now();const target = workspaceUrlTargetV1(path, literal, workspaceRoot);globalThis.__t.url=(globalThis.__t.url??0)+performance.now()-__c;");
 }
 text=text.replace(/from\s+"(\.\.?\/[^"]+)"/gu,(_,spec)=>`from ${JSON.stringify(resolve(dirname(original),spec))}`);
 mkdirSync(G,{recursive:true});const scratch=join(G,"script-inputs-patched.mjs");writeFileSync(scratch,text);
 const parserPath=join(L,"⚡️caching/📥️inference/🟨️.mjs");let parserText=readFileSync(parserPath,"utf8");for(const [anchor,replacement] of all.parser as [string,string][]){if(parserText.includes(replacement))continue;if(parserText.split(anchor).length!==2)throw Error("parser anchor");parserText=parserText.replace(anchor,replacement);}
 const parserScratch=join(G,"parser-patched.mjs");writeFileSync(parserScratch,parserText);const parser:any=await import(parserScratch);
 const coordinates:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/📍️coordinates/🟨️.mjs"));
 const patched:any=await import(scratch);
 const base=prepareSourceAnalysesV1(),references=observation.registerSourceAnalysisV1({id:base.command.id.replace("command-imports","command-references"),revision:base.command.revision,analyze:(path:string,bytes:Buffer)=>{const text=bytes.toString("utf8");return {runtime:boot.loaded.runtimeModuleSourceImports(path,text),urls:parser.commandUrlReferences(path,text)};}});
 patched.wire({coordinates,sourceAnalyses:{prepareSourceAnalysesV1:()=>({...prepareSourceAnalysesV1(),references})},nativeManifests:{readToml:(p:string)=>Bun.TOML.parse(readFileSync(p,"utf8"))},bunLock:{readBunLockGraph:cacheInternals.readBunLockGraph}});
 return {inference,cacheInternals,createScriptInputCache,patched,shipped:{...cacheInternals,createScriptInputCache}};
}
export function scriptEntries(stride=1,offset=0,limit=100000){
 const tracked=new TextDecoder().decode(Bun.spawnSync(["git","-c","core.quotepath=false","ls-files","-z"],{cwd:W}).stdout).split("\0").filter(f=>f&&strip(f.split("/").at(-1)!)===strip("📜️script.ts")&&existsSync(join(W,f)));
 const ig=new Set(["🧫️fixtures","🧫️examples","🧪️fixtures","🧪️examples"].map(strip));
 return tracked.filter(f=>{const s=f.split("/").map(strip);return s[0]!==strip(".🧬semio")&&!s.slice(0,-1).some(x=>ig.has(x));}).sort().filter((_,i)=>i%stride===0).slice(offset,offset+limit);
}
