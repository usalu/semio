import {readFileSync,existsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
const R="/Users/ueli/Documents/semio";
const T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/trace-testkit-tools";
const units=JSON.parse(readFileSync(T+"/units.json","utf8")) as any[];
const NEW="🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust";
const mans=[...new Set(units.map(u=>u.pkg))].concat([NEW]);
let fail=0;
const ok=(c:boolean,m:string)=>{ if(!c){fail++;console.log("FAIL",m);} };
for(const pkg of mans){
  const p=join(R,pkg,"Cargo.toml"); const t=Bun.TOML.parse(readFileSync(p,"utf8")) as any;
  if(pkg!==NEW){
    const d=t["dev-dependencies"]?.["semio-framework-trace-memory-testing"];
    ok(!!d&&typeof d.path==="string",pkg+" dev-dep missing");
    if(d) ok(existsSync(join(resolve(R,pkg,d.path),"Cargo.toml")) && resolve(R,pkg,d.path)===resolve(R,NEW),pkg+" path does not resolve to the owner");
    ok(!(t.dependencies?.["semio-framework-trace-memory-testing"]),pkg+" normal dep leaked");
  } else {
    ok(t.package.name==="semio-framework-trace-memory-testing","name");
    const lib=resolve(R,NEW,t.lib.path); ok(existsSync(lib),"lib path "+lib);
    ok(resolve(R,NEW,t.package.workspace)===join(R,"🧰️framework"),"workspace resolves to 🧰️framework");
    ok(existsSync(join(resolve(R,NEW,t.dependencies["semio-framework-trace"].path),"Cargo.toml")),"trace path");
    const b=t.package.metadata.semio["rust-source-boundary"]; ok(JSON.stringify(Object.keys(b).sort())===JSON.stringify(["maxActiveLines","maxSourceBytes","maxSourceFiles"]),"boundary keys");
  }
}
const fw=Bun.TOML.parse(readFileSync(R+"/🧰️framework/Cargo.toml","utf8")) as any;
const mem=fw.workspace.members as string[];
const rel="🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust";
ok(mem.includes(rel),"member present");
ok(mem.filter(m=>m===rel).length===1,"member unique");
const i=mem.indexOf(rel); ok(Buffer.compare(Buffer.from(mem[i-1]!),Buffer.from(rel))<0 && Buffer.compare(Buffer.from(rel),Buffer.from(mem[i+1]!))<0,"member byte-sorted between neighbours: "+mem[i-1]+" | "+mem[i+1]);
const owners=JSON.parse(readFileSync(R+"/📋️build-owners.json","utf8")).owners as any[];
const row=owners.filter(o=>o.name==="@semio-tech/trace-memory-testing-rs"); ok(row.length===1,"owner row");
ok(row[0].root===NEW && row[0].manifest===NEW+"/📋️project.json" && existsSync(join(R,row[0].manifest)) && Array.isArray(row[0].packages)&&row[0].packages.length===0,"owner row fields");
const pj=JSON.parse(readFileSync(join(R,NEW,"📋️project.json"),"utf8")); ok(pj.name===row[0].name,"project name equals owner name");
ok(owners.map(o=>o.name).filter(n=>n===pj.name).length===1,"owner unique");
const routes=["🏗️build 🛠️prototype","🏗️build 🚀️release","🔎️check 🛠️prototype","🔎️check 🚀️release","🧪️test 🛠️prototype","🧪️test 🚀️release"];
for(const f of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
  const txt=readFileSync(R+"/"+f,"utf8"); const j=(Bun as any).JSONC.parse(txt); const cs=j.configurations as any[];
  const orders=new Map<number,number>(); for(const c of cs){const o=c.presentation?.order; if(typeof o==="number") orders.set(o,(orders.get(o)??0)+1);}
  for(const r of routes){
    const name="🦀️ semio-framework-trace-memory-testing "+r; const m=cs.filter(c=>c.name===name); ok(m.length===1,f+" route "+name);
    if(m[0]){ const tgt=r.includes("build")?"build":r.includes("check")?"check":"test"; const cfg=r.includes("prototype")?"prototype":"release";
      ok(m[0].command===`bun repo run @semio-tech/trace-memory-testing-rs:${tgt} --configuration=${cfg} --parallel=1 -- --lib --offline`,f+" command "+name);
      ok(orders.get(m[0].presentation.order)===1,f+" order unique "+m[0].presentation.order);
      ok(m[0].presentation.group===({build:"4_build",check:"4_gate",test:"4_test"} as any)[tgt],f+" group"); }
  }
}
console.log(fail===0?"ALL STATIC CHECKS PASSED":"FAILURES "+fail, "manifests checked:",mans.length);
