import {open} from "node:fs/promises";
import {streamJsonObjectV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧠️admission-memo/🌊️json-stream/🟦️.ts";
const sets:Set<string>[]=[];
for(const path of process.argv.slice(2)){const f=await open(path,"r"),s=new Set<string>();
 for await(const e of streamJsonObjectV1(f,new Set(["owners"])))if("element" in e&&e.key==="owners")s.add((e.element as any).suffix);
 await f.close();sets.push(s);}
const [a,...rest]=sets;for(const [i,b] of rest.entries())console.log("scope 0 vs",i+1,"sizes",a!.size,b.size,"shared suffixes",[...b].filter(x=>a!.has(x)).length);
