import {readRustSourceClosureV1} from "C:/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFileSync,writeFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {execSync} from "node:child_process";
const root="C:/semio",ticket=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-FEM-3D-END-TO-END/🗑️generated/";
const base=execSync("rustc --print cfg --target wasm32-wasip2").toString().trim().split("\n").map(v=>v.replace(/=(".*")$/u,(_,q)=>"="+JSON.parse(q)));
const norm=(f:string)=>f.replaceAll("\\","/").replace(/^C:\/semio\//,"");
const changed=new Set(readFileSync(ticket+"changed.txt","utf8").split(/\r?\n/).filter(Boolean).map(norm));
const tomls=readFileSync(ticket+"users.txt","utf8").split(/\r?\n/).filter(f=>f.endsWith("Cargo.toml")).map(f=>resolve(root,f.replaceAll("\\","/")));
const reader=async(p:string)=>{try{return new Uint8Array(readFileSync(p));}catch{return null;}};
const out:Record<string,{lib:string[];dev:string[]}>={};
for(const m of tomls){const t=readFileSync(m,"utf8"),name=/^name\s*=\s*"([^"]+)"/m.exec(t)?.[1];if(!name)continue;
 const lib=/\[lib\][^\[]*?path\s*=\s*"([^"]+)"/s.exec(t)?.[1]??(()=>{try{readFileSync(dirname(m)+"/src/lib.rs");return"src/lib.rs";}catch{return undefined;}})();
 const tests=[...t.matchAll(/\[\[(?:test|bin|example|bench)\]\][^\[]*?path\s*=\s*"([^"]+)"/gs)].map(x=>x[1]);
 const run=async(entries:string[],cfg:string[])=>{if(!entries.length)return new Set<string>();const inputs=await readRustSourceClosureV1(entries.map(e=>resolve(dirname(m),e)),root,cfg,reader,{manifestDirectory:dirname(m),generated:()=>undefined}).catch(e=>{console.log("ERR",name,String(e).slice(0,120));return[];});return new Set(inputs.map(i=>i.path));};
 const l=await run(lib?[lib]:[],base),d=await run([...(lib?[lib]:[]),...tests],[...base,"test","debug_assertions"]);
 const hitLib=[...l].filter(f=>changed.has(f)),hitDev=[...d].filter(f=>changed.has(f)&&!l.has(f));
 if(hitLib.length||hitDev.length)out[name]={lib:hitLib,dev:hitDev,manifest:norm(m)} as never;}
writeFileSync(ticket+"owners.json",JSON.stringify(out,null,1));console.log(Object.entries(out).map(([k,v])=>k+" lib:"+v.lib.length+" dev:"+v.dev.length).join("\n"));
