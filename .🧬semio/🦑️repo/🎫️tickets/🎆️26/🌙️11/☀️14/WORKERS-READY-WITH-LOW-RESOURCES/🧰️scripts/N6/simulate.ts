import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const L="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const {openFactIndexV1}=await import(L+"🧮️fact-codec/🟨️.mjs");const {derivedOwnerNameV1}=await import(L+"✂️cutoff/🟨️.mjs");
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maximumOutputLength:2**31} as never);
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
const {changed,analysisChanged}=JSON.parse(readFileSync(process.argv[2]!,"utf8")) as {changed:string[];analysisChanged:string[]};
const bodyOnly=changed.filter(p=>!analysisChanged.includes(p)),prefix="@derived:";
const byName=new Map<string,number>();index.owners.forEach((n:string,i:number)=>byName.set(n,i));
const isReal=(o:number)=>!index.owners[o].startsWith(prefix);
function run(paths:string[],analysisOnly:boolean){
 const P=new Set(paths),cand=new Set<number>();
 for(let id=0;id<index.rows.length;id++){const r=index.rows[id];if(P.has(r.path)&&(r.kind==="analysis"&&analysisOnly||r.kind==="file"&&!analysisOnly))cand.add(id);}
 const holders=new Map<number,Set<number>>(),pseudoOf=new Map<number,Set<number>>();
 for(const cls of index.classes){const hit=cls.r.filter((id:number)=>cand.has(id));if(!hit.length)continue;
  for(const o of cls.o){if(isReal(o)){let h=holders.get(o);if(!h)holders.set(o,h=new Set());for(const id of hit)h.add(id);}else for(const id of hit){let s=pseudoOf.get(id);if(!s)pseudoOf.set(id,s=new Set());s.add(o);}}}
 const heldDerived=new Map<number,Set<number>>();
 const need=new Set(holders.keys());
 for(const cls of index.classes){const d=cls.r.filter((id:number)=>index.rows[id].kind==="derived");if(!d.length)continue;
  for(const o of cls.o){if(!need.has(o))continue;let s=heldDerived.get(o);if(!s)heldDerived.set(o,s=new Set());for(const id of d){const pi=byName.get(derivedOwnerNameV1(index.rows[id]));if(pi!==undefined)s.add(pi);}}}
 let before=0,after=0,pseudoStale=new Set<number>();
 for(const [o,rowsHit] of holders){before++;let uncovered=false;for(const id of rowsHit){const ps=pseudoOf.get(id);const hd=heldDerived.get(o);if(!(ps&&hd&&[...ps].some(p=>hd.has(p)))){uncovered=true;break;}}if(uncovered)after++;}
 const pseudoHit=new Set<number>();for(const s of pseudoOf.values())for(const p of s)pseudoHit.add(p);
 return {candidateRows:cand.size,before,after,suspectPseudoOwners:pseudoHit.size};
}
console.log("generation",pointer.generation,"owners",index.owners.length,"changed",changed.length,"import-changing",analysisChanged.length,"body-only",bodyOnly.length);
console.log("body-only edits, content rows (kind file) of the files:",JSON.stringify(run(bodyOnly,false)));
console.log("body-only edits, analysis rows (meaning unchanged, so never stale by themselves):",JSON.stringify(run(bodyOnly,true)));
console.log("import-changing edits, analysis rows:",JSON.stringify(run(analysisChanged,true)));
