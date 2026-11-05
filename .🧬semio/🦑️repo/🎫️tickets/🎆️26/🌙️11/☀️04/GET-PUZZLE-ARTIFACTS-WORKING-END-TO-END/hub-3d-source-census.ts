/** 🔎️ Current Hub and 3d editor acceptance Source owners, selected through their actual Cargo targets. */
import {readFile,writeFile} from "node:fs/promises";
import {resolve,dirname} from "node:path";
const root=process.cwd();
const{readRustSourceClosureV1}=await import(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const hub="🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml";
const editor="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml";
const cases=[{name:"Hub-production",manifest:hub,cfg:[]},{name:"Hub9-lib-test",manifest:hub,cfg:["test"]},{name:"3d-editor-gestures",manifest:editor,target:"editor-gestures",cfg:["test"]},{name:"3d-editor-surface",manifest:editor,target:"editor-surface",cfg:["test"]}];
const rows=await Promise.all(cases.map(async row=>{
 const file=resolve(root,row.manifest),manifestDirectory=dirname(file),cargo=Bun.TOML.parse(await readFile(file,"utf8")),entry=resolve(manifestDirectory,row.target?cargo.test.find(target=>target.name===row.target).path:cargo.lib.path),probes=new Map<string,number>();
 try{
  const inputs=await readRustSourceClosureV1([entry],root,row.cfg,async path=>{try{const bytes=await readFile(path);probes.set(path,bytes.byteLength);return bytes}catch(error){if(error.code==="ENOENT"||error.code==="ENOTDIR")return null;throw error}},{manifestDirectory,generated:()=>undefined});
  return{...row,entry,bytes:inputs.reduce((n,input)=>n+input.bytes,0),files:inputs.length,activeLines:inputs.reduce((n,input)=>n+input.activeLines,0),probeBytes:[...probes.values()].reduce((n,size)=>n+size,0),probeFiles:probes.size,inputs,probes:[...probes].map(([path,bytes])=>({path,bytes}))};
 }catch(error){return{...row,entry,error:String(error),probes:[...probes].map(([path,bytes])=>({path,bytes}))}}
}));
await writeFile(resolve(import.meta.dir,"🗑️generated/hub-3d-preflight-extents.json"),JSON.stringify(rows,null,2)+"\n");
for(const{inputs,probes,...summary}of rows)console.log(JSON.stringify(summary));
if(rows.some(row=>"error"in row))process.exitCode=1;
