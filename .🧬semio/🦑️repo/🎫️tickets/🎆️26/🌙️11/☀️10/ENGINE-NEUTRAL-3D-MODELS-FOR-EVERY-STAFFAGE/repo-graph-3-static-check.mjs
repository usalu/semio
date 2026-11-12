// Static pre-flight of the hard errors `repo show projects` can raise, WITHOUT building the project graph.
// Run from the workspace root with the repo's bun:  bun <this file> [outfile.json]
// Classes: (1) workspace imports (the very resolver the graph walk uses), (2) project files (parse, schema,
// cache decision, duplicate names, referenced files, artifact producers), (3) build-owner registry vs project files.
import {existsSync,readFileSync,writeFileSync,statSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {createRequire} from "node:module";
import {dirname,join,relative,resolve,isAbsolute,sep} from "node:path";

const ROOT=process.cwd(),L=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const out=process.argv[2],require=createRequire(import.meta.url),Ajv=require(ROOT+"/node_modules/ajv"),addFormats=require(ROOT+"/node_modules/ajv-formats");
const t0=Date.now(),log=m=>console.error("[DEBUG] +"+Math.round((Date.now()-t0)/1000)+"s "+m);
const files=execFileSync("git",["ls-files","-co","--exclude-standard","-z"],{cwd:ROOT,maxBuffer:1<<29}).toString().split("\0").filter(f=>f&&!f.startsWith(".🧬semio/")&&existsSync(join(ROOT,f)));
log("files "+files.length);
const findings={workspaceImports:[],producer:[],projectParse:[],projectSchema:[],cacheDecision:[],duplicateNames:[],missingFiles:[],registry:[],dependsOn:[],other:[]};
const read=f=>readFileSync(join(ROOT,f),"utf8");

const projectFiles=files.filter(f=>f===("📋️project.json")||f.endsWith("/📋️project.json")),projects=new Map();
const projectSchema=JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"));
const cacheSchema=JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/💾️cache.json"));
const producerSchema=JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🧬️schema/🔣️.json"));
const mk=()=>{const a=new Ajv({strict:false,allErrors:false,validateFormats:false});try{addFormats(a)}catch{}a.addSchema(producerSchema,"producer.json");return a;};
let validateProject,validateCache;
try{validateProject=mk().compile(JSON.parse(JSON.stringify(projectSchema).split("../../../⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🧬️schema/🔣️.json#").join("producer.json#")));}catch(e){findings.other.push("project schema did not compile: "+e.message);}
try{validateCache=mk().compile(cacheSchema);}catch(e){findings.other.push("cache schema did not compile: "+e.message);}
const names=new Map();
for(const f of projectFiles){
 let j;try{j=JSON.parse(read(f));}catch(e){findings.projectParse.push({file:f,error:e.message});continue;}
 projects.set(f,j);const root=dirname(f);
 if(validateProject&&!validateProject(j))findings.projectSchema.push({file:f,error:validateProject.errors.slice(0,2).map(x=>x.instancePath+" "+x.message).join("; ")});
 if(typeof j.name==="string"){(names.get(j.name)??names.set(j.name,[]).get(j.name)).push(f);}else findings.projectParse.push({file:f,error:"missing name"});
 for(const [tn,t] of Object.entries(j.targets??{})){
  const c=t?.metadata?.semio?.cache;
  if(c!==undefined&&validateCache&&!validateCache(c))findings.cacheDecision.push({file:f,target:tn,cache:c});
  const cwd=t?.options?.cwd;if(typeof cwd==="string"&&!cwd.includes("{")&&!cwd.includes("$")){const p=isAbsolute(cwd)?cwd:resolve(ROOT,cwd);if(!existsSync(p))findings.missingFiles.push({file:f,target:tn,kind:"options.cwd",path:cwd});}
  for(const d of t?.dependsOn??[]){const s=typeof d==="string"?d:d?.target,proj=typeof d==="object"?d?.projects:undefined;if(typeof s!=="string"||s.startsWith("^")||proj)continue;if(s.includes(":")){const [p,tg]=[s.slice(0,s.lastIndexOf(":")),s.slice(s.lastIndexOf(":")+1)];(findings.__deps??=[]).push({file:f,target:tn,dep:s,p,tg});}else if(!(j.targets??{})[s])findings.dependsOn.push({file:f,target:tn,dep:s,why:"local target absent"});}
 }
 if(typeof j.$schema==="string"&&!j.$schema.startsWith("http")&&!existsSync(resolve(ROOT,root,j.$schema)))findings.missingFiles.push({file:f,kind:"$schema",path:j.$schema});
 if(typeof j.sourceRoot==="string"&&!existsSync(resolve(ROOT,j.sourceRoot)))findings.missingFiles.push({file:f,kind:"sourceRoot",path:j.sourceRoot});
}
for(const [n,fs] of names)if(fs.length>1)findings.duplicateNames.push({name:n,files:fs});
for(const d of findings.__deps??[]){const hit=names.get(d.p);if(hit){const j=projects.get(hit[0]);if(!(j.targets??{})[d.tg])findings.dependsOn.push({file:d.file,target:d.target,dep:d.dep,why:"target absent in "+hit[0]});}}
delete findings.__deps;
log("projects "+projectFiles.length);

const {parserTypescriptV1}=await import(L+"🏗️build/🚀️bootstrap/📦️parsers/🟨️.mjs");
const si=await import(L+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const {readTypeScriptPackageProducerV1}=await import(L+"⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🏭️producer/🟦️.ts");
const compiler=parserTypescriptV1();
const host={...compiler.sys,realpath:compiler.sys.realpath};
for(const f of projectFiles){const j=projects.get(f);if(!j||dirname(f)===".")continue;try{readTypeScriptPackageProducerV1(join(ROOT,dirname(f)),ROOT,p=>existsSync(p)?readFileSync(p,"utf8"):undefined);}catch(e){findings.producer.push({file:f,error:e.message});}}

const workspaceNames=new Set();for(const f of files.filter(f=>f.endsWith("package.json")))try{const n=JSON.parse(read(f)).name;if(n)workspaceNames.add(n);}catch{}
const pkgOf=s=>s.startsWith("@")?s.split("/").slice(0,2).join("/"):s.split("/")[0];
const od=await import(L+"🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"),pn=await import(L+"🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"),admission=await import(L+"🏗️build/🕸️graph/🚪️admission/🟦️.ts");
await (await import(L+"🏗️build/🕸️graph/📥️inference/🟨️.mjs")).libraryBootstrap;
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(read("📋️build-owners.json"))),contracts=pn.projectGeneratorContractsV1(ROOT);
const produced=request=>{try{return od.readBuildProducedCompileOutputV1(request,ROOT,contracts,registry);}catch(e){return undefined;}};
const memo=new Map(),sources=files.filter(f=>/\.(ts|tsx|mts|cts|js|jsx|mjs|cjs)$/u.test(f)&&!/\.d\.[cm]?ts$/u.test(f)&&!f.split("/").some(s=>["node_modules","🤖️generated","🗑️generated",".🧬semio"].includes(s)));
log("sources "+sources.length+" workspace packages "+workspaceNames.size);
let n=0;
for(const f of sources){
 if(++n%5000===0)log("scanned "+n);
 let text;try{text=read(f);}catch{continue;}
 let specs;try{specs=compiler.preProcessFile(text,true,true).importedFiles.map(x=>x.fileName);}catch{continue;}
 const abs=join(ROOT,f);
 for(const s of new Set(specs)){
  if(s.startsWith(".")||isAbsolute(s)||!workspaceNames.has(pkgOf(s)))continue;
  const key=dirname(f)+"\0"+s;let r=memo.get(key);
  if(r===undefined){try{const got=si.workspacePackageModuleV1(abs,s,ROOT,compiler,undefined,compiler.ModuleKind.ESNext,false,produced);r={ok:true,got:got&&relative(ROOT,got)};}catch(e){r={ok:false,error:e.message.slice(0,300)};}memo.set(key,r);}
  if(!r.ok)findings.workspaceImports.push({file:f,specifier:s,error:r.error});
 }
}
log("imports checked");
const {checkBuildOwnerRegistry}=await import(L+"🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts");
{const r=checkBuildOwnerRegistry(ROOT);for(const f of r.findings)findings.registry.push(f);log("registry owners "+r.owners);}
const summary=Object.fromEntries(Object.entries(findings).map(([k,v])=>[k,v.length]));
console.log(JSON.stringify(summary));
if(out)writeFileSync(out,JSON.stringify({summary,findings},null,1));
log("done");
