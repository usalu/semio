// 🧩️ Scratch driver for the staged unit engine: plan / check / oracle over a workspace directory (overlay or real tree).
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,readdirSync,statSync,writeFileSync,realpathSync} from "node:fs";
import {dirname,join,relative} from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {checkTypeScriptUnitsV1,deriveTypeScriptImportsV1,planTypeScriptUnitsV1,readTypeScriptUnitsDocumentV1,runTypeScriptCompilerV1,type TypeScriptUnitPlanV1} from "/private/tmp/conc-ts/engine/🧩️units/🟦️.ts";
const [command,workspace,family,...rest]=process.argv.slice(2) as string[];
const flag=(name:string,fallback?:string):string|undefined=>{const i=rest.indexOf("--"+name);return i<0?fallback:rest[i+1];};
const document=readTypeScriptUnitsDocumentV1(JSON.parse(readFileSync("/private/tmp/conc-ts/engine/🧩️units/🔣️.json","utf8")));
for(const [k,v] of Object.entries(JSON.parse(flag("override","{}")!))) (document as any)[k]={...(document as any)[k],...(v as object)};
const out=flag("out","/private/tmp/conc-ts/run")!;mkdirSync(out,{recursive:true});
const parsed=ts.getParsedCommandLineOfConfigFile(join(workspace,family),{},{...ts.sys,onUnRecoverableConfigFileDiagnostic:d=>{throw Error(String(d.messageText))}})!;
const read=(path:string)=>{try{return readFileSync(path,"utf8")}catch{return undefined}};
const planFile=join(out,"plan.json");
function manifests():string[]{if(rest.includes("--git")){return Bun.spawnSync(["git","ls-files","-co","--exclude-standard","-z","--","package.json","*/package.json"],{cwd:workspace}).stdout.toString().split("\0").filter(f=>f&&!f.startsWith(".🧬semio")&&(f==="package.json"||f.endsWith("/package.json"))).map(f=>f==="package.json"?"":f.slice(0,-13));}const rows:string[]=[],walk=(d:string,depth:number)=>{for(const e of readdirSync(join(workspace,d),{withFileTypes:true})){if(e.name==="node_modules"||e.name===".cache"||e.name===".git")continue;const p=d?d+"/"+e.name:e.name;if(e.isDirectory())walk(p,depth+1);else if(e.name==="package.json")rows.push(d);}};walk("",0);return rows;}
if(command==="plan"){
  const t0=performance.now(),roots=parsed.fileNames.filter(f=>f.startsWith(workspace+"/")&&!f.includes("/node_modules/")).map(f=>f.slice(workspace.length+1));
  const imports=deriveTypeScriptImportsV1(ts,{workspace,compilerOptions:parsed.options,roots,exists:p=>existsSync(p),read});
  const t1=performance.now(),plan=planTypeScriptUnitsV1(imports,document,family),t2=performance.now();
  writeFileSync(planFile,JSON.stringify(plan));writeFileSync(join(out,"imports.json"),JSON.stringify(imports));
  const sizes=plan.units.map(u=>u.files.length).sort((a,b)=>b-a);
  console.log(JSON.stringify({roots:roots.length,files:imports.files.length,edges:imports.edges.length,json:imports.json.length,deriveMs:Math.round(t1-t0),planMs:Math.round(t2-t1),units:plan.units.length,ambient:plan.ambient.length,largest:sizes.slice(0,8),components:plan.components.length,unitEdges:plan.units.reduce((s,u)=>s+u.dependencies.length,0),digest:plan.digest.slice(0,12)}));
}
if(command==="check"){
  const plan=JSON.parse(readFileSync(planFile,"utf8")) as TypeScriptUnitPlanV1,session=flag("session","s1")!,store=flag("store",join(out,"store"))!,state=flag("state",join(out,"state"))!;
  const memo=new Map<string,{key:string,digest:string}>(),digest=(p:string)=>{const abs=join(workspace,p),s=statSync(abs),key=s.mtimeMs+":"+s.size,m=memo.get(abs);if(m&&m.key===key)return m.digest;const d=createHash("sha256").update(readFileSync(abs)).digest("hex");memo.set(abs,{key,digest:d});return d;};
  const options=createHash("sha256").update(JSON.stringify(parsed.options)).update(JSON.stringify(document.compiler)).digest("hex");
  const tsc=runTypeScriptCompilerV1(process.execPath,"/Users/ueli/Documents/semio/node_modules/typescript/bin/tsc",workspace,{...process.env,NODE_OPTIONS:"--max-old-space-size="+(flag("heap","1536"))});
  const typeRoots:string[]=[];for(let d=dirname(join(workspace,family));d.startsWith(workspace);d=dirname(d)){const t=join(d,"node_modules","@types");if(existsSync(t))typeRoots.push(t);}
  const typeFiles=document.compiler.extraTypes.map(n=>ts.resolveModuleName(n,join(workspace,family),parsed.options,ts.sys).resolvedModule!.resolvedFileName);
  const host={typeFiles,typeRoots,workspace,compiler:"typescript@"+ts.version,options,environment:"nm",family,store,session,state,digest,tsc};
  const only=flag("only")?.split(",");const ctl=new AbortController();
  const t0=performance.now();
  const result=await checkTypeScriptUnitsV1(plan,document,host,{concurrency:Number(flag("concurrency","1")),only,signal:ctl.signal,onProgress:r=>{if(!rest.includes("--quiet"))console.error(`[${r.done}/${r.total}] ${r.status} ${r.unit} ${r.milliseconds}ms`);}});
  const wall=Math.round(performance.now()-t0),ran=result.units.filter(u=>u.status!=="hit");
  writeFileSync(join(out,"diagnostics-"+session+".txt"),result.diagnostics);
  console.log(JSON.stringify({wallMs:wall,units:result.units.length,ran:ran.length,hits:result.units.length-ran.length,errors:result.errors,sumRanMs:ran.reduce((s,u)=>s+u.milliseconds,0),slowest:ran.sort((a,b)=>b.milliseconds-a.milliseconds).slice(0,5).map(u=>[u.id.slice(0,70),u.milliseconds])}));
}
