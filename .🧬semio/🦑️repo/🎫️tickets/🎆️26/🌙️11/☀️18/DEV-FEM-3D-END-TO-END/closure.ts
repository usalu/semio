import {readRustSourceClosureV1} from "C:/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFileSync,readdirSync,statSync} from "node:fs";
import {join,dirname,resolve} from "node:path";
import {execSync} from "node:child_process";
const root="C:/semio",cfg=execSync("rustc --print cfg --target wasm32-wasip2").toString().trim().split("\n").map(v=>v.replace(/=(".*")$/u,(_,q)=>"="+JSON.parse(q)));
const walk=(d:string,o:string[]=[]):string[]=>{for(const e of readdirSync(d)){const p=join(d,e);if(e==="node_modules"||e==="target")continue;const s=statSync(p);if(s.isDirectory())walk(p,o);else if(e==="Cargo.toml")o.push(p);}return o;};
const only=process.argv.slice(2),rows:string[]=[];
for(const base of ["✏️s/🔌️plugins/🏗️fem","🌎️hub/🧩️compositions/🏗️fem"])for(const m of walk(join(root,base))){const t=readFileSync(m,"utf8"),name=/^name\s*=\s*"([^"]+)"/m.exec(t)?.[1],lib=/\[lib\][^\[]*?path\s*=\s*"([^"]+)"/s.exec(t)?.[1];if(!name||!lib)continue;if(only.length&&!only.includes(name))continue;
 const b=/\[package\.metadata\.semio\.rust-source-boundary\]([^\[]*)/.exec(t)?.[1]??"";
 try{const inputs=await readRustSourceClosureV1([resolve(dirname(m),lib)],root,cfg,async p=>{try{return new Uint8Array(readFileSync(p));}catch{return null;}},{manifestDirectory:dirname(m),generated:()=>undefined});
 const bytes=inputs.reduce((s,i)=>s+i.bytes,0),lines=inputs.reduce((s,i)=>s+i.activeLines,0);rows.push(`${String(bytes).padStart(7)} ${String(inputs.length).padStart(3)} ${String(lines).padStart(5)} ${name} ${b.trim().replace(/\n/g," ")}`);
 if(only.length)for(const i of [...inputs].sort((a,c)=>c.bytes-a.bytes))console.log(String(i.bytes).padStart(7),String(i.activeLines).padStart(5),i.kind,i.path);}
 catch(e){rows.push("ERR "+name+" "+String(e).slice(0,150));}}
rows.sort().reverse();console.log("bytes files lines crate (limits 262144/32/4000)");console.log(rows.slice(0,25).join("\n"));
