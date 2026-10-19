import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFileSync,existsSync,readdirSync} from "node:fs";
import {resolve,dirname,join} from "node:path";
const roots=process.argv.slice(2);const files=[];
const walk=d=>{for(const e of readdirSync(d,{withFileTypes:true})){if(["node_modules",".git","target",".cache","🗑️generated"].includes(e.name)||e.name.startsWith(".🧬"))continue;const p=join(d,e.name);if(e.isDirectory())walk(p);else if(e.name==="Cargo.toml")files.push(p);}};
for(const r of roots)walk(r);
const res=[];const base=['debug_assertions','unix','target_os=macos','target_arch=aarch64','target_family=unix','target_pointer_width=64','target_endian=little','target_vendor=apple','target_env=','panic=unwind','target_has_atomic=64','target_has_atomic=ptr','target_has_atomic=8','target_has_atomic=16','target_has_atomic=32','overflow_checks','ub_checks','target_abi=','target_feature=neon'];
for(const f of files){let m;try{m=Bun.TOML.parse(readFileSync(f,"utf8"));}catch{continue;}if(!m.package)continue;const dir=dirname(f);
 const b=m.package.metadata?.semio?.["rust-source-boundary"];const lim={f:Math.min(32,b?.maxSourceFiles??32),b:Math.min(262144,b?.maxSourceBytes??262144),l:Math.min(4000,b?.maxActiveLines??4000)};
 const feats=Object.keys(m.features??{});
 const entries=[];
 const lib=m.lib?.path??"src/lib.rs";if(existsSync(resolve(dir,lib)))entries.push(["lib",resolve(dir,lib),["all"]]);
 for(const t of m.test??[])if(t.path&&existsSync(resolve(dir,t.path)))entries.push(["test:"+t.name,resolve(dir,t.path),["all","test"]]);
 for(const t of m.bin??[])if(t.path&&existsSync(resolve(dir,t.path)))entries.push(["bin:"+t.name,resolve(dir,t.path),["all"]]);
 for(const [kind,entry,mode] of entries){
  for(const mm of (kind==="lib"?["default","all","test"]:mode.includes("test")?["all"]:["default","all"])){
   const cfg=new Set(base.concat(mm==="all"?feats.map(x=>'feature='+x):[]));if(mm==="test"||kind.startsWith("test"))cfg.add("test");
   try{const inputs=await readRustSourceClosureV1([entry],process.cwd(),cfg,async p=>{try{return new Uint8Array(readFileSync(p));}catch{return null;}},{manifestDirectory:dir,generated:()=>undefined});
    const fc=inputs.length,by=inputs.reduce((s,i)=>s+i.bytes,0),ln=inputs.reduce((s,i)=>s+i.activeLines,0);
    if(fc>lim.f||by>lim.b||ln>lim.l)res.push(m.package.name+"\t"+kind+"\t"+mm+"\tfiles "+fc+"/"+lim.f+" bytes "+by+"/"+lim.b+" lines "+ln+"/"+lim.l+"\t"+f);
   }catch(e){res.push(m.package.name+"\t"+kind+"\t"+mm+"\tERR "+String(e.message).slice(0,120)+"\t"+f);}
  }}
}
console.log(res.join("\n"));console.error("manifests",files.length,"findings",res.length);
