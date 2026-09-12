import { existsSync, readFileSync, writeFileSync, readdirSync, mkdirSync, renameSync, rmdirSync, statSync } from "node:fs";
import { join, resolve, relative, dirname, extname } from "node:path";
import { createHash } from "node:crypto";
import assert from "node:assert/strict";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT??process.cwd(), ticket=resolve(import.meta.dir,"../..");
const output=join(ticket,"🗑️generated/testing-taxonomy/synthetic-inputs");
mkdirSync(output,{recursive:true});
const os="🧰️framework/🛍️products/💻️os", library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const mappings=[
 [os+"/🧪️tests/🧩️jcoprobe-callback/🦀️rust/👽️guest",os+"/🧫️fixtures/🧩️jcoprobe/👽️guest"],
 [os+"/🧪️tests/🧩️jcoprobe-callback/🟦️typescript",os+"/🧫️fixtures/🧩️jcoprobe/🌐️browser-host"],
 [os+"/🧪️tests/⚖️scale/🦀️rust",os+"/🧫️fixtures/⚖️scale"],
 [os+"/🧪️tests/⚖️scale/🟦️typescript/🟦️.ts",os+"/🧫️fixtures/⚖️scale/🟦️.ts"],
];
const hash=(p:string)=>createHash("sha256").update(readFileSync(join(root,p))).digest("hex");
const map=(p:string)=>{ for(const [a,b] of mappings) if(p===a||p.startsWith(a+"/")) return b+p.slice(a.length); return p; };
const files=(path:string):string[]=>statSync(join(root,path)).isDirectory()?readdirSync(join(root,path),{withFileTypes:true}).flatMap(e=>e.isSymbolicLink()?[]:files(path+"/"+e.name)):[path];
const report=join(ticket,"📓️synthetic-input-taxonomy-2026-09-12.md"), input=join(import.meta.dir,"📥️input.md");
async function run(name:string,args:string[],cwd=root) {
 const log=join(output,name+".log");writeFileSync(log,"");writeFileSync(join(output,name+".stderr.log"),"");
 const child=Bun.spawn(args,{cwd,env:{...process.env,CARGO_TARGET_DIR:join(output,"target"),TMPDIR:output,TMP:output,TEMP:output},stdout:Bun.file(log),stderr:Bun.file(join(output,name+".stderr.log"))});
 const code=await child.exited;
 const stdout=readFileSync(log,"utf8"), stderr=readFileSync(join(output,name+".stderr.log"),"utf8");
 writeFileSync(report,(existsSync(report)?readFileSync(report,"utf8"):"")+"\n## "+name+"\n\nExit "+code+".\n\n```text\n"+stdout.slice(-4500)+stderr.slice(-2200)+"\n```\n");
 console.log(JSON.stringify({name,code,log}));
 return code;
}
const command=process.argv[2];
if(command==="capture"){
 const moves=mappings.flatMap(([a])=>files(a).map(p=>({from:p,to:map(p),sha256:hash(p)})));
 writeFileSync(input,"# Synthetic Input Relocation Input\n\nJCO guest and browser-host stand-ins, and the scale actor are synthetic executable inputs; assertion programs remain direct canonical tests. Build output remains inside the source fixture package.\n\n```json\n"+JSON.stringify({mappings,moves},null,2)+"\n```\n");
 writeFileSync(report,"# Synthetic Input Testing Taxonomy — 2026-09-12\n\nThe JCO guest and host stand-ins and the scale actor are synthetic programs supplied to runtime tests. They belong to OS fixtures. Assertions remain direct leaves under OS tests. The scale component build still writes to dist/component inside its source package.\n\n"+moves.map(m=>"- `"+m.from+"` → `"+m.to+"` — SHA-256 `"+m.sha256+"`").join("\n")+"\n");
 console.log(JSON.stringify({captured:moves.length}));
}else if(command==="baseline"||command==="verify"){
 await run(command+"-jco",["bun",join(root,os,"🧪️tests/🧩️jcoprobe-callback/🟦️.ts")]);
 await run(command+"-scale",["cargo","test","-p","semio-framework-os-scale-fixture","--lib"]);
}else if(command==="server"){
 const entry=join(root,os,"🧫️fixtures/🧩️jcoprobe/🌐️browser-host/📜️script.ts");
 const child=Bun.spawn(["bun",entry],{cwd:root,env:{...process.env,SEMIO_JCO_PROBE_PORT:"0"},stdout:"pipe",stderr:"pipe"});
 const deadline=setTimeout(()=>child.kill(),20000), reader=child.stdout.getReader();
 try {
  const first=await reader.read(),line=new TextDecoder().decode(first.value),port=line.match(/localhost:(\d+)/)?.[1];
  assert(port,line);
  const paths=["🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🌐️.html","🧪️tests/🧩️jco-callback/🟨️.mjs","🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🖥️host-shim.js","🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/io.js","🧫️fixtures/🧩️jcoprobe/🌐️browser-bundles/📞️out-callback/jcoprobe.js","🧫️fixtures/🧩️jcoprobe/🌐️browser-bundles/📞️out-callback/jcoprobe.core.wasm"];
  for(const [i,path] of paths.entries()){
   const response=await fetch("http://localhost:"+port+(i===0?"/":"/"+encodeURI(path)));
   assert.equal(response.status,200,path);assert.deepEqual(Buffer.from(await response.arrayBuffer()),readFileSync(join(root,os,path)),path);
  }
  assert.equal((await fetch("http://localhost:"+port+"/Cargo.toml")).status,404);
  writeFileSync(report,readFileSync(report,"utf8")+"\n## Browser Host HTTP Verification\n\nThe live relocated Bun server returned six exact source/binary payloads for its page, canonical worker, host and WASI stand-ins, JCO bundle, and core WASM. Byte comparison used Node assert against the current files. An unrelated path returned404.\n");
  console.log(JSON.stringify({serverPaths:paths.length,unownedPathStatus:404}));
 }finally{clearTimeout(deadline);reader.releaseLock();child.kill();await child.exited;}
}else if(command==="wasm"){
 process.exitCode=await run("verify-scale-wasm",["cargo","check","-p","semio-framework-os-scale-fixture","--target","wasm32-wasip2","--features","component-guest"]);
}else if(command==="move"||command==="restore-scale"){
 const data=command==="restore-scale"?{moves:mappings.slice(2).flatMap(([a])=>files(a).map(p=>({from:p,to:map(p),sha256:hash(p)})))}:JSON.parse(readFileSync(input,"utf8").match(/```json\n([\s\S]*?)\n```/)![1]) as {moves:{from:string;to:string;sha256:string}[]};
 if(command==="restore-scale")writeFileSync(join(import.meta.dir,"📥️scale-coordinate-correction.md"),"# Scale Coordinate Correction\n\nThe interrupted executor restored the nested test package after the accepted move. This correction preserves its current bytes and returns the synthetic input program to fixtures.\n\n```json\n"+JSON.stringify(data,null,2)+"\n```\n");
 for(const m of data.moves){ if(hash(m.from)!==m.sha256) throw Error("Concurrent change: "+m.from); mkdirSync(dirname(join(root,m.to)),{recursive:true}); if(existsSync(join(root,m.to)))throw Error("Destination exists: "+m.to); renameSync(join(root,m.from),join(root,m.to)); if(hash(m.to)!==m.sha256)throw Error("Hash changed: "+m.to); }
 const scan=Bun.spawnSync(["rg","--files","--hidden","-0","-g","!AGENTS.md","-g","!.🧬semio/**","-g","!**/target/**","-g","!**/node_modules/**","-g","!**/🗑️generated/**","🧰️framework","✏️s","🌎️hub",".vscode","Cargo.toml","📜️script.ts"],{cwd:root});
 const changed:string[]=[];
 const movedReverse=new Map(data.moves.map(m=>[m.to,m.from]));
 for(const p of scan.stdout.toString().split("\0").filter(Boolean)){
  if(![".ts",".tsx",".js",".mjs",".rs",".json",".jsonc",".toml",".md",".html",".wit",".lock"].includes(extname(p)))continue;
  if(!existsSync(join(root,p))||statSync(join(root,p)).size>8*1024*1024)continue;
  const before=readFileSync(join(root,p),"utf8");let after=before;
  const origin=movedReverse.get(p)??p;
  after=after.replace(/(["'])(\.\.?\/[^"'\n]+)\1/g,(full,quote,value)=>{
    const oldValue=before.includes(value)?value:null;
    if(!oldValue)return full;
    const destination=map(relative(root,resolve(root,dirname(origin),value)));
    const wasMoved=destination!==relative(root,resolve(root,dirname(origin),value));
    if(!wasMoved && !movedReverse.has(p))return full;
    if(!existsSync(join(root,destination)))return full;
    const rel=relative(dirname(join(root,p)),join(root,destination)).split("\\").join("/");
    return quote+(rel.startsWith(".")?rel:"./"+rel)+quote;
  });
  for(const [a,b] of mappings){ after=after.replaceAll(a,b).replaceAll(a.slice(os.length+1),b.slice(os.length+1)); }
  if(before!==after){writeFileSync(join(root,p),after);changed.push(p);}
 }
 function prune(p:string){if(!existsSync(join(root,p)))return; for(const e of readdirSync(join(root,p),{withFileTypes:true}))if(e.isDirectory())prune(p+"/"+e.name);if(!readdirSync(join(root,p)).length)rmdirSync(join(root,p));}
 for(const [a] of mappings)if(existsSync(join(root,a))&&statSync(join(root,a)).isDirectory())prune(a);
 for(const p of [os+"/🧪️tests/⚖️scale",os+"/🧪️tests/🧩️jcoprobe-callback/🦀️rust"])prune(p);
 writeFileSync(report,readFileSync(report,"utf8")+"\n## Immediate Movement Verification\n\nAll "+data.moves.length+" files retained their captured SHA-256 on movement.\n\n## Updated Consumers\n\n"+changed.map(p=>"- `"+p+"`").join("\n")+"\n");
 console.log(JSON.stringify({moved:data.moves.length,consumers:changed.length}));
}else throw Error("Expected capture, baseline, move, or verify");
