// 🔎️ Static checks of the ui-runtime carve: manifests, workspace, owners, launch routes (both files), corpus, path resolution, single ownership, path length.
import {readFileSync,existsSync,readdirSync,statSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
const ROOT="/Users/ueli/Documents/semio", UI=ROOT+"/🧰️framework/🔨️modules/🖱️ui/🧠️runtime";
const NEW:[string,string][]=[["exchange","📡️exchange"],["presentation","🖼️presentation"],["reconcile-primitives","🧱️primitives"],["reconcile-cursor","⏭️cursor"],["reconcile","♻️reconcile"]];
let fails=0; const ok=(c:boolean,m:string)=>{ if(!c){fails++;console.log("FAIL",m);} };
const manifests:[string,string][]=[["ui-runtime",UI+"/📦️packages/🦀️rust"],...NEW.map(([id,node])=>["ui-runtime-"+id,UI+"/"+node+"/📦️packages/🦀️rust"] as [string,string])];
const names=new Map<string,string>();
for(const [id,dir] of manifests){
  const m=Bun.TOML.parse(readFileSync(dir+"/Cargo.toml","utf8")) as any;
  names.set(dir,m.package.name);
  ok(m.package.name==="semio-framework-"+id,id+" name");
  ok(m.package.metadata.semio.id===id,id+" semio id");
  if(id!=="ui-runtime"){
    const b=m.package.metadata.semio["rust-source-boundary"];
    ok(JSON.stringify(Object.keys(b).sort())===JSON.stringify(["maxActiveLines","maxSourceBytes","maxSourceFiles"])&&b.maxActiveLines===4000&&b.maxSourceFiles===32&&b.maxSourceBytes===262144,id+" boundary keys");
    ok(m.lib.path==="../../🦀️.rs",id+" lib path");
    ok(existsSync(resolve(dir,m.lib.path)),id+" lib root exists");
    const proj=JSON.parse(readFileSync(dir+"/📋️project.json","utf8")); ok(proj.name==="@semio-tech/"+id+"-rs",id+" project name");
  } else ok(existsSync(resolve(dir,m.lib.path)),"facade lib root");
  ok(existsSync(resolve(dir,m.package.workspace,"Cargo.toml")),id+" workspace resolves");
  for(const tbl of [m.dependencies,m["dev-dependencies"]]) for(const [alias,spec] of Object.entries(tbl??{}) as [string,any][]) if(spec.path){ const t=resolve(dir,spec.path,"Cargo.toml"); ok(existsSync(t),id+" dep path "+alias); if(existsSync(t)){ const tn=(Bun.TOML.parse(readFileSync(t,"utf8")) as any).package.name; ok((spec.package??alias)===tn,id+" dep name "+alias+" vs "+tn); } }
  for(const t of m.test??[]) ok(existsSync(resolve(dir,t.path)),id+" [[test]] path");
  for(const [f,list] of Object.entries(m.features??{}) as [string,string[]][]) for(const e of list){ const dep=e.split("/")[0].replace("dep:",""); ok(dep in (m.dependencies??{})||dep in (m["dev-dependencies"]??{}),id+" feature "+f+" -> "+e); }
}
// workspace members
const members=(Bun.TOML.parse(readFileSync(ROOT+"/🧰️framework/Cargo.toml","utf8")) as any).workspace.members as string[];
for(const [id,dir] of manifests) ok(members.includes(dir.replace(ROOT+"/🧰️framework/","")),"workspace member "+id);
const rt=members.filter(m=>m.includes("🧠️runtime"));
ok(JSON.stringify(rt.map(s=>Buffer.from(s)).sort(Buffer.compare).map(b=>b.toString()))===JSON.stringify(rt),"runtime members byte-sorted");
ok(new Set(members).size===members.length,"members unique");
// owners
const owners=JSON.parse(readFileSync(ROOT+"/📋️build-owners.json","utf8")).owners as any[];
ok(new Set(owners.map(o=>o.name)).size===owners.length,"owner names unique");
for(const [id,dir] of manifests){ const o=owners.find(o=>o.name==="@semio-tech/"+id+"-rs"); ok(!!o&&o.root===dir.replace(ROOT+"/","")&&o.manifest===o.root+"/📋️project.json"&&existsSync(ROOT+"/"+o.manifest)&&o.packages.length===0,"owner row "+id); }
// launch routes
for(const file of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
  const text=readFileSync(ROOT+"/"+file,"utf8"); const cfg=JSON.parse(file.endsWith(".json")?text:text.replace(/^\s*\/\/.*$/gm,"")).configurations as any[];
  const names2=new Map<string,number>(); for(const c of cfg) names2.set(c.name,(names2.get(c.name)??0)+1);
  const orders=new Map<string,number>(); for(const c of cfg) if(c.presentation&&typeof c.presentation==="object") { const k=c.presentation.group+"|"+c.presentation.order; orders.set(k,(orders.get(k)??0)+1); }
  for(const [id,] of NEW.map(([i,n])=>["ui-runtime-"+i,n])) for(const [t,glyph,grp] of [["build","🏗️build","4_build"],["check","🔎️check","4_gate"],["test","🧪️test","4_test"]]) for(const [mode,mg] of [["prototype","🛠️prototype"],["release","🚀️release"]]){
    const name=`🦀️ semio-framework-${id} ${glyph} ${mg}`, cmd=`bun repo run @semio-tech/${id}-rs:${t} --configuration=${mode} --parallel=1 -- --lib --offline`;
    const rows=cfg.filter(c=>c.name===name); ok(rows.length===1&&rows[0].command===cmd&&rows[0].presentation.group===grp,file+" route "+name);
    if(rows.length===1) ok(orders.get(grp+"|"+rows[0].presentation.order)===1,file+" unique order "+name+" "+rows[0].presentation.order);
  }
}
// corpus
const corpus=JSON.parse(readFileSync(ROOT+"/🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json","utf8")).crates as any[];
for(const [id,node] of NEW) ok(corpus.filter(r=>r.package==="semio-framework-ui-runtime-"+id&&r.manifest==="🧠️runtime/"+node+"/📦️packages/🦀️rust/Cargo.toml"&&r.entry==="🧠️runtime/"+node+"/🦀️.rs").length===1,"corpus row "+id);
// path/include resolution + single ownership + path length
const rs:string[]=[]; const walk=(d:string)=>{ for(const e of readdirSync(d)){ const p=join(d,e); const s=statSync(p); if(s.isDirectory()) { if(e!=="target"&&e!=="node_modules") walk(p);} else if(p.endsWith(".rs")) rs.push(p); } }; walk(UI);
const mounts=new Map<string,string[]>();
let longest=0;
for(const f of rs){
  longest=Math.max(longest,Buffer.byteLength(f.replace(ROOT+"/","")));
  const t=readFileSync(f,"utf8");
  for(const m of t.matchAll(/#\[path\s*=\s*"([^"]+)"\s*\]/g)){ const target=resolve(dirname(f),m[1]); ok(existsSync(target),"path mount "+f.replace(UI,"")+" -> "+m[1]); (mounts.get(target)??mounts.set(target,[]).get(target)!).push(f); }
  for(const m of t.matchAll(/include_(?:str|bytes)!\("([^"]+)"\)/g)) ok(existsSync(resolve(dirname(f),m[1])),"include "+f.replace(UI,"")+" -> "+m[1]);
}
for(const [target,by] of mounts) ok(by.length===1,"single mount "+target.replace(UI,"")+" by "+by.length);
console.log("rs files",rs.length,"mounted",mounts.size,"longest path bytes",longest);
ok(longest<=240,"path length");
console.log(fails===0?"ALL STATIC CHECKS PASSED":"FAILURES "+fails);
process.exit(fails?1:0);
