import {readFileSync,writeFileSync} from "node:fs";
import {posix,relative,resolve} from "node:path";
import {aggregateScope,convertMutationLeaves,decoupleFromAggregate} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
const cases=JSON.parse(readFileSync(new URL("u45_cases.json",import.meta.url),"utf8"));
const ROOT=resolve("/w");const rel=(abs:string)=>relative(ROOT,abs).split("\\").join("/");
const out:any={convert:{},scope:[],decouple:[]};
for(const c of cases.convert){
 const reader=(abs:string)=>c.files[rel(abs)]??null;
 const r=convertMutationLeaves(resolve(ROOT,c.root),c.files[c.root],reader);
 const e:any={};for(const[k,v]of [...r.edits].sort())e[rel(k)]=v;const cr:any={};for(const[k,v]of [...r.creates].sort())cr[rel(k)]=v;
 out.convert[c.name]={converted:[...r.converted],hand:r.hand.map(h=>({payload:h.payload,reason:h.reason})),notes:[...r.notes],aggregates:r.aggregates.map(a=>({name:a.name,snapshot:a.snapshot,diff:a.diff,variants:a.variants.map(v=>v.variant)})),edits:e,creates:cr};
}
for(const s of cases.scope){const r=aggregateScope(s.text,s.eff,s.isComponent);const names:any={};for(const[k,v]of [...r.names].sort())names[k]=v;out.scope.push({names,glob:r.glob});}
for(const d of cases.decouple){const scope=aggregateScope(d.aggregate,d.eff,d.isComponent);const r=decoupleFromAggregate(d.leaf,d.depth,scope);out.decouple.push(r===null?null:{text:r.text,coupled:[...r.coupled]});}
writeFileSync(new URL("../gen/u45_goldens.json",import.meta.url),JSON.stringify(out,null,1));
