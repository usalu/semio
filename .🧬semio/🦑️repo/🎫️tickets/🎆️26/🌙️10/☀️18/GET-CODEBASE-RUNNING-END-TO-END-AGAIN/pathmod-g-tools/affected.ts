// affected: bun affected.ts <census.json> <out.json> : which LIVE precision rules (cargo reads) and which crates of the repository have a crate with an unresolved Rust source set (fallback) in their path-dependency closure, i.e. whose derived key would miss the mounted files of that crate. Uses the live nativeDependencyRoots; streams; RSS guard 900 MB.
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const W="/Users/ueli/Documents/semio",LIB=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(LIB,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const {cacheInternals}=inference,cache=new Map(),closures=new Map();
const census=JSON.parse(readFileSync(process.argv[2]!,"utf8")) as any[];
const fallbackNamed=new Set(census.filter(r=>r.named?.status==="fallback").map(r=>r.root)),fallbackDep=new Set(census.filter(r=>r.dep?.status==="fallback"&&r.dep.reason!=="no-entries").map(r=>r.root));
const byName=new Map(census.filter(r=>r.name).map(r=>[r.name,r.root]));
const rules=JSON.parse(readFileSync(join(LIB,"🏗️build/📥️inputs/🎯️precision/🔣️.json"),"utf8")).rules as any[];
const out:any={rules:[],crates:{total:census.length,withFallbackInClosure:0,rows:[]}};
for(const rule of rules)for(const read of rule.reads??[]){
 if(!read.cargo)continue;
 const directories:string[]=[...(read.cargo.directories??[]),...(read.cargo.packages??[]).map((n:string)=>byName.get(n)).filter(Boolean)];
 for(const directory of directories){
  const roots:string[]=cacheInternals.nativeDependencyRoots(directory,W,true,cache,closures);
  const hit=roots.filter(r=>fallbackNamed.has(r)||fallbackDep.has(r));
  out.rules.push({rule:rule.id,directory,closure:roots.length,fallbackCrates:hit});
 }
}
for(const row of census){
 if(!row.name)continue;
 const roots:string[]=cacheInternals.nativeDependencyRoots(row.root,W,true,cache,closures),hit=roots.filter(r=>r!==row.root&&(fallbackNamed.has(r)||fallbackDep.has(r)));
 if(hit.length||fallbackNamed.has(row.root)){out.crates.withFallbackInClosure++;out.crates.rows.push({name:row.name,self:fallbackNamed.has(row.root),via:hit.length});}
}
writeFileSync(process.argv[3]!,JSON.stringify(out));
console.log(JSON.stringify({rules:out.rules.length,crates:out.crates.withFallbackInClosure,rssMB:Math.round(process.memoryUsage().rss/1048576)}));
if(process.memoryUsage().rss>900<<20)throw new Error("RSS above 900 MB");
