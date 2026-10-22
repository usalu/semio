import {readFileSync,writeFileSync} from "node:fs";
import {Universe} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🔍️resolver/🟦️.ts";
import {Unmirror} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🟦️.ts";
const cases=JSON.parse(readFileSync(new URL("u49_cases.json",import.meta.url),"utf8")).cases;
const out:any={};
for(const row of cases){
 const u=new Universe("/virtual",new Map(Object.entries<string>(row.files)));for(const c of row.crates)u.addCrate(c.name,c.dir,c.root);for(const c of row.crates)u.root(c.name.replaceAll("-","_"));
 const editable=row.editable===null?()=>true:(p:string)=>row.editable.includes(p),m=new Unmirror(u,{apiRoots:new Set<string>(row.apiRoots),targets:new Set<string>(row.targets),editable});
 const paths=[...u.files.keys()].sort(),audit:any={};for(const p of paths){const f=m.audit(p);if(f.length)audit[p]=f;}
 for(const p of paths)m.processFile(p);
 const edits:any={};for(const[p,t]of[...m.report.edits].sort())edits[p]=t;const r=m.report;
 out[row.name]={edits,report:{removedLeaves:r.removedLeaves,repointedLeaves:r.repointedLeaves,repointedChains:r.repointedChains,addedImports:r.addedImports,removedModules:r.removedModules,unresolvedMirrors:[...r.unresolvedMirrors],multiMounted:[...r.multiMounted],invisible:[...r.invisible]},audit};
}
writeFileSync(new URL("../gen/u49_goldens.json",import.meta.url),JSON.stringify(out,null,1));
