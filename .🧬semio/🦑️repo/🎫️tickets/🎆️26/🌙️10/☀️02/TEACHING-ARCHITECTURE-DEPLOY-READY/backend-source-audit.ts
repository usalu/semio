import { readdirSync, writeFileSync } from "node:fs";
import { relative, resolve } from "node:path";
const root=process.cwd();
const inference=await import(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const manifests:string[]=[],failures:any[]=[],memo=new Map();
function walk(directory:string){for(const entry of readdirSync(directory,{withFileTypes:true})){if(entry.name.startsWith(".")||["target","dist","node_modules","🗑️generated","🧫️fixtures","🔮️oracles","👽️guest"].includes(entry.name))continue;const path=directory+"/"+entry.name;if(entry.isDirectory())walk(path);else if(entry.name==="Cargo.toml")manifests.push(directory);}}
walk(root);
for(const directory of manifests){const path=relative(root,directory);try{const set=inference.cacheInternals.cargoSourceSetV1(path,root,memo,true,false);if(["macro-mount","dynamic-include","conditional-mount"].includes(set.reason))failures.push({root:path,reason:set.reason,message:set.message});}catch(error){failures.push({root:path,error:String(error)});}}
const result={count:manifests.length,failures};
writeFileSync(resolve(import.meta.dir,"🗑️generated/backend-source-audit.json"),JSON.stringify(result,null,2));
console.log(JSON.stringify(result,null,2));
