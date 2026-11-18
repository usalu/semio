import {readFileSync} from "node:fs";
const out=process.env.R140_OUT!;
const owner:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
const observation:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs");
const rows=JSON.parse(readFileSync(out+"/probe.admission.json","utf8")).observations,stale:any[]=[];
const s=performance.now();const ok=await observation.observationsHold(rows,64,new Set(),new Set(),undefined,(row:any)=>{if(stale.length<12)stale.push(row);});
console.log(JSON.stringify({ok,ms:Math.round(performance.now()-s),rows:rows.length,stale:stale.map(r=>({kind:r.kind,path:String(r.path).slice(-110),fact:JSON.stringify(r.fact).slice(0,100)}))},null,1));
