import {readFileSync,statSync,promises as fs} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname,resolve} from "node:path";
const [,,metaPath,rootName,...flags]=process.argv;
const root=resolve(process.cwd());
const {readRustSourceClosureV1}=await import(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const meta=JSON.parse(readFileSync(metaPath,"utf8"));
const pk=new Map(meta.packages.map(p=>[p.id,p]));
const nodes=new Map(meta.resolve.nodes.map(n=>[n.id,n]));
const start=meta.packages.find(p=>p.name===rootName).id;
const seen=new Set([start]),queue=[start];
while(queue.length){const n=nodes.get(queue.pop());for(const d of n.deps)if(d.dep_kinds.some(k=>k.kind!=="dev")&&!seen.has(d.pkg)){seen.add(d.pkg);queue.push(d.pkg);}}
const cfgOf=t=>execFileSync("rustc",["--print","cfg",...(t?["--target",t]:[])]).toString().trim().split("\n");
const target=cfgOf("wasm32-wasip2"),host=cfgOf(null);
const policy={maxSourceBytes:262144,maxSourceFiles:32,maxActiveLines:4000};
const rows=[];
for(const id of seen){
 const p=pk.get(id);if(p.source!==null)continue;
 const b=p.metadata?.semio?.["rust-source-boundary"];
 const lim={bytes:Math.min(policy.maxSourceBytes,b?.maxSourceBytes??1e12),files:Math.min(policy.maxSourceFiles,b?.maxSourceFiles??1e9),lines:Math.min(policy.maxActiveLines,b?.maxActiveLines??1e9)};
 const feats=nodes.get(id).features.map(f=>"feature="+JSON.stringify(f));
 for(const t of p.targets.filter(t=>["lib","rlib","cdylib","proc-macro","custom-build"].some(k=>t.kind.includes(k)))){
  const cfg=[...(t.kind.includes("proc-macro")||t.kind.includes("custom-build")?host:target),...feats];
  let r={files:0,bytes:0,lines:0};
  try{
   const inputs=await readRustSourceClosureV1([t.src_path],root,cfg,async path=>{try{return new Uint8Array(await fs.readFile(path));}catch{return null;}},{manifestDirectory:dirname(p.manifest_path),generated:()=>undefined});
   if(process.env.CRATE_FILES===p.name)for(const i of inputs)console.error(i.bytes,i.activeBytes,i.activeLines,i.path);r={newest:Math.max(...inputs.map(i=>statSync(resolve(root,i.path)).mtimeMs)),files:inputs.length,bytes:inputs.reduce((s,i)=>s+i.bytes,0),lines:inputs.reduce((s,i)=>s+i.activeLines,0)};
  }catch(e){r.error=String(e.message).slice(0,120);}
  rows.push({crate:p.name,kind:t.kind[0],manifest:p.manifest_path.replace(root+"/",""),...r,lim});
 }
}
rows.sort((a,b)=>b.bytes/b.lim.bytes-a.bytes/a.lim.bytes);
console.log(JSON.stringify(rows));
