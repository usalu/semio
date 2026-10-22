import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFile} from "node:fs/promises";
import {execFileSync} from "node:child_process";
const root=process.cwd();
const base=execFileSync("rustc",["--print","cfg"]).toString().split("\n").filter(Boolean).map(l=>l.replace(/="(.*)"$/,"=$1"));
const feats=(process.env.FEATURES??"").split(",").filter(Boolean).map(f=>`feature=${f}`);
const modes:Record<string,string[]>={lib:[],test:["test"]};
for(const lib of process.argv.slice(2)){
  for(const [mode,extra] of Object.entries(modes)){
    const cfg=[...base,...extra,...feats];
    try{
      const rows=await readRustSourceClosureV1([lib],root,cfg,async p=>{try{return new Uint8Array(await readFile(p));}catch{return null;}});
      let b=0,l=0;for(const r of rows){b+=r.bytes;l+=r.activeLines;}
      const flag=(b>262144||rows.length>32||l>4000)?" OVER":"";
      console.log(mode.padEnd(5),String(rows.length).padStart(3)+"f",String(b).padStart(7)+"B",String(l).padStart(5)+"L",lib.split("/").slice(-5,-2).join("/"),flag);
      if(process.env.V)for(const r of rows)console.log("    ",r.bytes,r.activeLines,r.path.split("/").slice(-4).join("/"));
    }catch(e){console.log(mode,"ERR",String(e).slice(0,200));}
  }
}
