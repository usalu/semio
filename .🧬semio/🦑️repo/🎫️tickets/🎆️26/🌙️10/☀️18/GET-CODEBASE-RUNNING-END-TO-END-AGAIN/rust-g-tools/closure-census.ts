/** bun closure-census.ts : file counts of the cargo script's command closure per registered command word (native, relay, unit) against the union. */
import {resolve} from "node:path";
const root=process.cwd(),lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const inference=await import(resolve(root,lib+"🏗️build/🕸️graph/📥️inference/🟨️.mjs")) as any;await inference.libraryBootstrap;
const scripts=await import(resolve(root,lib+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs")) as any;
const script=resolve(root,lib+"⚡️caching/🦀️cargo/📜️script.ts");
const closure=async(words?:string[])=>{
 const cache=inference.createScriptInputCache({workspaceRoot:root});
 const rows=await scripts.relativeScriptInputs([script],root,cache,true,[],words?new Map([[script,words]]):new Map());
 return new Set<string>(rows.filter((row:any)=>typeof row==="string"));
};
const union=await closure(),parts:Record<string,Set<string>>={};
for(const word of ["native","relay","unit"])parts[word]=await closure([word]);
console.log("union",union.size);
for(const [word,set] of Object.entries(parts))console.log(word.padEnd(6),set.size,"only-here",[...set].filter(file=>!Object.entries(parts).some(([other,rest])=>other!==word&&rest.has(file))).length);
const all=new Set(Object.values(parts).flatMap(set=>[...set]));
console.log("union of per-command",all.size,"missing from union",[...all].filter(file=>!union.has(file)).length);
const only=(word:string)=>[...parts[word]!].filter(file=>!Object.entries(parts).some(([other,rest])=>other!==word&&rest.has(file)));
for(const word of ["relay","unit"])console.log(word,"exclusive sample",only(word).slice(0,8).map(file=>file.slice(-90)));
const stripped=(set:Set<string>)=>new Set<string>(scripts.withoutSchedulingV1([...set]));
const cleanUnion=stripped(union);
console.log("without scheduling: union",cleanUnion.size,"native",stripped(parts.native!).size,"unit",stripped(parts.unit!).size);
const buckets=new Map<string,number>();
for(const file of cleanUnion){const key=file.replace("{workspaceRoot}/","").split("/").slice(0,9).join("/").replace(/^🧰️framework\/(🛍️products\/🦑️repo\/)?🔨️modules\//,"");buckets.set(key,(buckets.get(key)??0)+1);}
console.log([...buckets].sort((a,b)=>b[1]-a[1]).slice(0,25).map(([k,v])=>v+"  "+k).join("\n"));
