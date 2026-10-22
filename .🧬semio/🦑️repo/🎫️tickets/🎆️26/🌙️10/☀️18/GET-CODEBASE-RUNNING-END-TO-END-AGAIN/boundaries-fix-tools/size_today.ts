import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFile} from "node:fs/promises";
import {resolve,dirname} from "node:path";
const root="/Users/ueli/Documents/semio";
const platforms:Record<string,string[]>={
 mac:["target_arch=aarch64","target_os=macos","target_env=","target_pointer_width=64","unix","debug_assertions"],
 linux:["target_arch=x86_64","target_os=linux","target_env=gnu","target_pointer_width=64","unix","debug_assertions"],
 win:["target_arch=x86_64","target_os=windows","target_env=msvc","target_pointer_width=64","windows","debug_assertions"],
 wasm:["target_arch=wasm32","target_os=wasi","target_env=p2","target_pointer_width=32","debug_assertions"],
};
let bad=0;
for(const m of process.argv.slice(2)){
  const t=await Bun.file(m).text();const toml=Bun.TOML.parse(t) as any;
  const entry=resolve(dirname(m),toml.lib?.path??"src/lib.rs");
  const feats=Object.keys(toml.features??{}).map(f=>`feature=${f}`);
  const rows:string[]=[];
  let max={b:0,l:0,f:0},worst="";
  for(const [pn,pc] of Object.entries(platforms)) for(const mode of ["lib","test"]){
    const cfg=[...pc,...feats,...(mode==="test"?["test"]:[])];
    const r=await readRustSourceClosureV1([entry],root,cfg,async p=>{try{return new Uint8Array(await readFile(p));}catch{return null;}});
    let b=0,l=0;for(const x of r){b+=x.bytes;l+=x.activeLines;}
    if(b>max.b||l>max.l||r.length>max.f){max={b:Math.max(b,max.b),l:Math.max(l,max.l),f:Math.max(r.length,max.f)};}
    if(b>262144||l>4000||r.length>32){bad++;worst+=` OVER(${pn}/${mode}:${b}B,${l}L,${r.length}f)`;}
  }
  console.log(toml.package.name.padEnd(52),`max ${max.b}B (${(100*max.b/262144).toFixed(0)}%) ${max.l}L ${max.f}f`,worst);
}
console.log(bad?"OVER-LIMIT":"ALL WITHIN GATE");
