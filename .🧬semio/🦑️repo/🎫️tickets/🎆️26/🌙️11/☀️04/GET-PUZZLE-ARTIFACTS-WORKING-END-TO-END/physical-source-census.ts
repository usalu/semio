/** 🔎️ Observe actual native physical-format acceptance Source owners without compiling. */
import {readFile,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
const root=process.cwd();
const {readRustSourceClosureV1}=await import(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const base="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/";
const owners=[{name:"physical2d",directory:base+"◻️2d/📦️packages/🦀️rust",entry:base+"◻️2d/🦀️.rs"},{name:"physical3d",directory:base+"🧊️3d/📦️packages/🦀️rust",entry:base+"🧊️3d/🧪️tests/🗃️snapshot-sqlite/🦀️.rs"},{name:"physical5d",directory:base+"🖐️5d/📦️packages/🦀️rust",entry:base+"🖐️5d/🦀️.rs"}];
const rows=await Promise.all(owners.map(async owner=>{
 const manifestDirectory=resolve(root,owner.directory),probes=new Map<string,number>();
 try{
  const inputs=await readRustSourceClosureV1([resolve(root,owner.entry)],root,["test"],async(path:string)=>{try{const bytes=await readFile(path);probes.set(path,bytes.byteLength);return bytes}catch(error){if(error.code==="ENOENT"||error.code==="ENOTDIR")return null;throw error}},{manifestDirectory,generated:()=>undefined});
  return{name:owner.name,bytes:inputs.reduce((n,input)=>n+input.bytes,0),files:inputs.length,activeLines:inputs.reduce((n,input)=>n+input.activeLines,0),probeBytes:[...probes.values()].reduce((n,size)=>n+size,0),probeFiles:probes.size,inputs,probes:[...probes]};
 }catch(error){return{name:owner.name,error:String(error),probes:[...probes]}}
}));
await writeFile(resolve(import.meta.dir,"🗑️generated/physical-source-extents.json"),JSON.stringify(rows,null,2)+"\n");
for(const{inputs,probes,...summary}of rows)console.log(JSON.stringify(summary));
if(rows.some(row=>"error"in row))process.exitCode=1;
