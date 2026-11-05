/** 🔎️ Measure the actual 2d aggregate cfg-selected inputs and every physical source probe. */
import {readFile,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
const root=process.cwd();
const {readRustSourceClosureV1}=await import(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const manifestDirectory=resolve(root,"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone/📦️packages/🦀️rust");
const rows=[];
for(const {name,entry,cfg} of [{name:"production",entry:resolve(manifestDirectory,"🦀️.rs"),cfg:[]},{name:"lib-test",entry:resolve(manifestDirectory,"🦀️.rs"),cfg:["test"]},{name:"typed-json",entry:resolve(root,"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/📝️typed-json/🦀️.rs"),cfg:["test"]}]){
 const probes=new Map<string,number>();
 const inputs=await readRustSourceClosureV1([entry],root,cfg,async(path:string)=>{
  try{const bytes=await readFile(path);probes.set(path,bytes.byteLength);return bytes}
  catch(error){if(error.code==="ENOENT"||error.code==="ENOTDIR")return null;throw error}
 },{manifestDirectory,generated:()=>undefined});
 const summary={name,cfg,bytes:inputs.reduce((n,input)=>n+input.bytes,0),files:inputs.length,activeLines:inputs.reduce((n,input)=>n+input.activeLines,0),probeBytes:[...probes.values()].reduce((n,size)=>n+size,0),probeFiles:probes.size};
 rows.push({...summary,inputs,probes:[...probes].map(([path,bytes])=>({path,bytes}))});
 console.log(JSON.stringify(summary));
}
await writeFile(resolve(import.meta.dir,"🗑️generated/puzzle2d-aggregate-extents.json"),JSON.stringify(rows,null,2)+"\n");
