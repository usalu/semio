import { readFileSync, writeFileSync, mkdirSync, renameSync, rmdirSync, readdirSync, existsSync, statSync } from "node:fs";
import { join, resolve, relative, dirname, basename, extname } from "node:path";
import { createHash } from "node:crypto";
import { pathToFileURL } from "node:url";
import assert from "node:assert/strict";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=resolve(import.meta.dir,"../.."),input=join(import.meta.dir,"📥️input.md"),report=join(ticket,"📓️procedural-test-scope-taxonomy-2026-09-12.md"),command=process.argv[2];
const hash=(p:string)=>createHash("sha256").update(readFileSync(join(root,p))).digest("hex");
type Move={from:string;to:string;sha256:string};
const load=()=>JSON.parse(readFileSync(input,"utf8").match(/```json\n([\s\S]*?)\n```/)![1]) as {moves:Move[]};
if(command==="capture"){
 const census=JSON.parse(readFileSync(join(ticket,"🗑️generated/testing-taxonomy/census/findings.json"),"utf8")) as {path:string;code:string}[];
 const moves:Move[]=[];
 for(const f of census){
  if(!f.path.startsWith("✏️s/🔌️plugins/")||!existsSync(join(root,f.path))||!statSync(join(root,f.path)).isFile())continue;
  let to:string|undefined;
  if(f.code==="test-implementation-depth" && f.path.includes("/🌀️procedural/")){
   to=f.path.replace(/\/🧪️tests\/([^/]+)\/([^/]+)\/([^/]+)$/,"/🪟️windows/$1/🧪️tests/$2/$3");
   assert(existsSync(join(root,to.split("/🧪️tests/")[0],"🦀️.rs")),to);
  }else if(f.code==="test-implementation-filename"&&f.path.endsWith("/contract.ts"))to=dirname(f.path)+"/🟦️.ts";
  else if(f.code==="test-case-name"){
   if(f.path.includes("/mount-contract/"))to=f.path.replace("/mount-contract/","/🧩️mount-contract/");
   else if(f.path.includes("/📄️document-behavior/"))to=f.path.replace("/📄️document-behavior/","/🧩️document-behavior/");
  }else if(f.code==="test-data-in-case"){
   const index=f.path.indexOf("/🧪️tests/"),owner=f.path.slice(0,index),tail=f.path.slice(index+10).split("/"),caseName=tail[0]==="mount-contract"?"🧩️mount-contract":tail[0];
   to=owner+"/🧫️fixtures/"+caseName+"/🔣️.json";
  }
  if(to&&to!==f.path&&!moves.some(m=>m.from===f.path)){assert(!existsSync(join(root,to)),"Existing destination "+to);moves.push({from:f.path,to,sha256:hash(f.path)});}
 }
 writeFileSync(input,"# Procedural and Residual Test Scope Input\n\nWindow assertions belong to the window semantic owner. Direct TypeScript adapters use the canonical language leaf. Neutral JSON input belongs to owner fixtures.\n\n```json\n"+JSON.stringify({moves},null,2)+"\n```\n");
 writeFileSync(report,"# Procedural and Residual Test Scope Taxonomy — 2026-09-12\n\n"+moves.map(m=>"- `"+m.from+"` → `"+m.to+"` — SHA-256 `"+m.sha256+"`").join("\n")+"\n");console.log(JSON.stringify({captured:moves.length}));
}else if(command==="baseline"||command==="verify"){
 const data=load(),results=[];
 for(const m of data.moves.filter(m=>m.from.endsWith("/contract.ts"))){
  const path=command==="baseline"?m.from:m.to;
  try{const module=await import(pathToFileURL(join(root,path)).href);for(const [name,value] of Object.entries(module))if(name.startsWith("testGeneration")&&typeof value==="function"){try{await value();results.push({path,name,passed:true});}catch(e){results.push({path,name,passed:false,error:String(e)});}}}catch(e){results.push({path,passed:false,error:String(e)});}
 }
 writeFileSync(report,readFileSync(report,"utf8")+"\n## "+command+" Actual Contract Functions\n\n```json\n"+JSON.stringify(results,null,2)+"\n```\n");console.log(JSON.stringify({command,passed:results.filter(r=>r.passed).length,failed:results.filter(r=>!r.passed).length}));
 if(command==="verify"){
  const guard=await import(pathToFileURL(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts")).href);
  const findings=guard.inspectTestLayoutSources(guard.testTaxonomy(root),data.moves.map(m=>({path:m.to,source:readFileSync(join(root,m.to),"utf8")}))).filter((f:any)=>f.code!=="invalid-test-module-wiring");
  writeFileSync(report,readFileSync(report,"utf8")+"\n## Relocated File Guard\n\n```json\n"+JSON.stringify(findings,null,2)+"\n```\n");assert.equal(findings.length,0);
 }
 process.exitCode=results.some(r=>!r.passed)?1:0;
}else if(command==="readers"){
 const data=load(),results=[];
 for(const m of data.moves.filter(m=>m.from.includes("document-contract")&&m.to.endsWith(".json"))){
  const owner=m.to.split("/🧫️fixtures/")[0],path=owner+"/🧪️tests/🪪️document-contract/🟦️.ts";
  try{const module=await import(pathToFileURL(join(root,path)).href);for(const [name,fn] of Object.entries(module))if(name.startsWith("test")&&typeof fn==="function"){await fn();results.push({path,name,passed:true});}}catch(e){results.push({path,passed:false,error:String(e)});}
 }
 const mount=data.moves.find(m=>m.from.endsWith("/mount-contract/🟦️.ts"))!,json=data.moves.find(m=>m.from.endsWith("/mount-contract/expected.json"))!;
 const module=await import(pathToFileURL(join(root,mount.to)).href);module.assertMountContract(JSON.parse(readFileSync(join(root,json.to),"utf8")));
 const python=Bun.which("python3")??Bun.which("python");assert(python);
 const files=data.moves.filter(m=>m.to.endsWith(".json")).map(m=>m.to),parse=Bun.spawnSync([python,"-c","import json,sys; print(json.dumps([json.load(open(p, encoding='utf-8')) for p in sys.argv[1:]], ensure_ascii=False))",...files.map(p=>join(root,p))],{cwd:root,env:{...process.env,PYTHONDONTWRITEBYTECODE:"1"}});
 assert.equal(parse.exitCode,0,parse.stderr.toString());assert.deepEqual(JSON.parse(parse.stdout.toString()),files.map(p=>JSON.parse(readFileSync(join(root,p),"utf8"))));
 const oracle=Bun.spawnSync([python,join(root,mount.to.replace("🟦️.ts","🐍️.py"))],{cwd:root,env:{...process.env,PYTHONDONTWRITEBYTECODE:"1"}});assert.equal(oracle.exitCode,0,oracle.stderr.toString());
 writeFileSync(report,readFileSync(report,"utf8")+"\n## Independent Reader and Document Verification\n\nPython and Node JSON parsers agreed on all "+files.length+" moved data files. The actual Python and TypeScript mount-contract implementations both accepted the moved neutral fixture. Actual Ajv/fast-json-patch document contract results:\n\n```json\n"+JSON.stringify(results,null,2)+"\n```\n");console.log(JSON.stringify({jsonPairs:files.length,mountImplementations:2,documentResults:results}));process.exitCode=results.some(r=>!r.passed)?1:0;
}else if(command==="repair-readers"){
 const data=load(),reverse=new Map(data.moves.map(m=>[m.to,m.from])),files=Bun.spawnSync(["rg","--files","-0","✏️s/🔌️plugins/🌀️procedural","✏️s/🔌️plugins/🗄️stdio","✏️s/🔌️plugins/📐️cad"],{cwd:root}).stdout.toString().split("\0").filter(Boolean),changed=[];
 for(const p of files){
  if(![".rs",".ts",".tsx",".py"].includes(extname(p)))continue;
  const origin=reverse.get(p)??p,before=readFileSync(join(root,p),"utf8");
  let after=before.replace(/Path\(__file__\)\.with_name\("([^"]+)"\)/g,(full,value)=>{
   const move=data.moves.find(m=>m.from===relative(root,resolve(root,dirname(origin),value)));if(!move)return full;
   return "(Path(__file__).parent / "+JSON.stringify(relative(dirname(join(root,p)),join(root,move.to)).split("\\").join("/"))+")";
  });
  after=after.replace(/(["'])([^"'\n]+)\1/g,(full,quote,value)=>{
   if(value.includes("\0")||value.length>2000)return full;
   const move=data.moves.find(m=>m.from===relative(root,resolve(root,dirname(origin),value)));if(!move)return full;
   const path=relative(dirname(join(root,p)),join(root,move.to)).split("\\").join("/");return quote+(path.startsWith(".")?path:"./"+path)+quote;
  });
  if(after!==before){writeFileSync(join(root,p),after);changed.push(p);}
 }
 writeFileSync(report,readFileSync(report,"utf8")+"\n## Language-Specific Local Readers\n\nLocal Rust include paths without a dot prefix and Python with_name were repaired using the captured source coordinates.\n\n"+changed.map(p=>"- `"+p+"`").join("\n")+"\n");console.log(JSON.stringify({readers:changed.length}));
}else if(command==="move"){
 const data=load(),map=(p:string)=>data.moves.find(m=>m.from===p)?.to??p,reverse=new Map(data.moves.map(m=>[m.to,m.from]));
 for(const m of data.moves){assert.equal(hash(m.from),m.sha256);mkdirSync(dirname(join(root,m.to)),{recursive:true});renameSync(join(root,m.from),join(root,m.to));assert.equal(hash(m.to),m.sha256);}
 const scan=Bun.spawnSync(["rg","--files","--hidden","-0","-g","!AGENTS.md","-g","!**/target/**","-g","!**/node_modules/**","-g","!**/🗑️generated/**","✏️s","🧰️framework","🌎️hub",".vscode","📜️script.ts"],{cwd:root});const changed=[];
 for(const p of scan.stdout.toString().split("\0").filter(Boolean)){
  if(![".ts",".tsx",".js",".mjs",".json",".jsonc",".rs",".py",".toml",".md"].includes(extname(p))||!existsSync(join(root,p))||statSync(join(root,p)).size>5*1024*1024)continue;
  const before=readFileSync(join(root,p),"utf8"),origin=reverse.get(p)??p;
  let after=before.replace(/(["'])(\.\.?\/[^"'\n]+)\1/g,(full,quote,value)=>{
   const old=relative(root,resolve(root,dirname(origin),value)),target=map(old);
   if(target===old&&!reverse.has(p)||!existsSync(join(root,target)))return full;
   const rel=relative(dirname(join(root,p)),join(root,target)).split("\\").join("/");return quote+(rel.startsWith(".")?rel:"./"+rel)+quote;
  });
  for(const m of data.moves)after=after.replaceAll(m.from,m.to);
  if(after!==before){writeFileSync(join(root,p),after);changed.push(p);}
 }
 const removed=[];for(const m of data.moves){let p=dirname(join(root,m.from));while(existsSync(p)&&!readdirSync(p).length){rmdirSync(p);removed.push(relative(root,p));p=dirname(p);}}
 writeFileSync(report,readFileSync(report,"utf8")+"\n## Movement Verification and Consumers\n\nAll "+data.moves.length+" moves preserved captured bytes before relative consumer rewrites.\n\n"+changed.map(p=>"- `"+p+"`").join("\n")+"\n\n## Empty Old Directories Removed\n\n"+removed.map(p=>"- `"+p+"`").join("\n")+"\n");console.log(JSON.stringify({moved:data.moves.length,consumers:changed.length}));
}else throw Error("Expected capture, baseline, move, or verify");
