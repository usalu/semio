import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFile} from "node:fs/promises";
import {existsSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {execFileSync} from "node:child_process";
const R="/Users/ueli/Documents/semio";
const base=execFileSync("rustc",["--print","cfg"]).toString().split("\n").filter(Boolean).map(l=>l.replace(/="(.*)"$/,"=$1"));
const input:string[]=JSON.parse(await Bun.file(process.argv[2]).text());
const cache=new Map<string,Set<string>|null>();
async function closureOf(manifest:string):Promise<Set<string>>{
  if(cache.has(manifest))return cache.get(manifest)!;
  const text=await Bun.file(manifest).text();
  const feats=[...(text.match(/^\[features\]\n([\s\S]*?)(?=^\[|$)/m)?.[1]??"").matchAll(/^([A-Za-z0-9_-]+)\s*=/gm)].map(m=>`feature=${m[1]}`);
  const cfg=[...base,"test",...feats];
  const entries:string[]=[];
  const sections=text.split(/^(?=\[)/m);
  for(const s of sections){
    const h=s.split("\n")[0].trim();
    if(["[lib]","[[bin]]","[[test]]","[[bench]]","[[example]]"].includes(h)){
      const m=s.match(/^path\s*=\s*"([^"]+)"/m);
      if(m)entries.push(resolve(dirname(manifest),m[1]));
    }
  }
  const set=new Set<string>();
  for(const e of entries){
    try{
      const rows=await readRustSourceClosureV1([e],R,cfg,async p=>{try{return new Uint8Array(await readFile(p));}catch{return null;}});
      for(const r of rows)set.add(resolve(R,r.path));
    }catch(err){console.error("ERR",manifest.replace(R+"/",""),e.replace(R+"/",""),String(err).slice(0,160));}
  }
  cache.set(manifest,set);return set;
}
const out:Record<string,string[]>={};
for(const f of input){
  const abs=resolve(R,f);
  const owners:string[]=[];
  let d=dirname(abs);
  while(d.startsWith(R)&&d!==R){
    for(const m of [d+"/📦️packages/🦀️rust/Cargo.toml",d+"/Cargo.toml"]){
      if(existsSync(m)){
        const t=await Bun.file(m).text();
        if(!t.includes("[package]"))continue;
        if((await closureOf(m)).has(abs))owners.push(m.replace(R+"/",""));
      }
    }
    d=dirname(d);
  }
  out[f]=owners;
}
console.log(JSON.stringify(out,null,1));
