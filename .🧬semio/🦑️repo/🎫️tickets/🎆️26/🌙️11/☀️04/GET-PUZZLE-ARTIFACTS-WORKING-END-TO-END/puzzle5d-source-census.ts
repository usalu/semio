/** 🔎️ Observe complete declared 5d mutation and app-test Source closures without compiling. */
import {readFile,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
const root=process.cwd();
const {readRustSourceClosureV1}=await import(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const base="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/";
const owners=[{name:"mutations-tests",directory:base+"🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust"},{name:"editor-app-tests",directory:base+"✏️editor/🧪️tests/🔬️unit-2/📦️packages/🦀️rust"}];
const rows=await Promise.all(owners.flatMap(owner=>[[],["test"]].map(async cfg=>{
 const manifestDirectory=resolve(root,owner.directory),probes=new Map<string,number>();
 try{
  const inputs=await readRustSourceClosureV1([resolve(manifestDirectory,"🦀️.rs")],root,cfg,async(path:string)=>{
   try{const bytes=await readFile(path);probes.set(path,bytes.byteLength);return bytes}
   catch(error){if(error.code==="ENOENT"||error.code==="ENOTDIR")return null;throw error}
  },{manifestDirectory,generated:()=>undefined});
  return{name:owner.name,cfg,bytes:inputs.reduce((n,input)=>n+input.bytes,0),files:inputs.length,activeLines:inputs.reduce((n,input)=>n+input.activeLines,0),probeBytes:[...probes.values()].reduce((n,size)=>n+size,0),probeFiles:probes.size,inputs,probes:[...probes].map(([path,bytes])=>({path,bytes}))};
 }catch(error){return{name:owner.name,cfg,error:String(error),probes:[...probes].map(([path,bytes])=>({path,bytes}))}}
})));
await writeFile(resolve(import.meta.dir,"🗑️generated/puzzle5d-source-extents.json"),JSON.stringify(rows,null,2)+"\n");
for(const{inputs,probes,...summary}of rows)console.log(JSON.stringify(summary));
if(rows.some(row=>"error"in row))process.exitCode=1;
