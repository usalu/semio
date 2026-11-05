/** 🔎️ Current Hub and 3d editor acceptance Source owners, selected through their actual Cargo targets. */
import {readFile,writeFile} from "node:fs/promises";
import {resolve,dirname} from "node:path";
const root=process.cwd();
const{readRustSourceClosureV1}=await import(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const cases=[{"name": "2d-IO-production", "manifest": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/📦️packages/🦀️rust/Cargo.toml", "cfg": []}, {"name": "2d-IO-test", "manifest": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/📦️packages/🦀️rust/Cargo.toml", "cfg": ["test"]}, {"name": "3d-IO-production", "manifest": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml", "cfg": []}, {"name": "3d-IO-test", "manifest": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml", "cfg": ["test"]}, {"name": "5d-IO-production", "manifest": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml", "cfg": []}, {"name": "5d-IO-test", "manifest": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml", "cfg": ["test"]}];
const rows=await Promise.all(cases.map(async row=>{
 const file=resolve(root,row.manifest),manifestDirectory=dirname(file),cargo=Bun.TOML.parse(await readFile(file,"utf8")),entry=resolve(manifestDirectory,row.target?cargo.test.find(target=>target.name===row.target).path:cargo.lib.path),probes=new Map<string,number>();
 try{
  const inputs=await readRustSourceClosureV1([entry],root,row.cfg,async path=>{try{const bytes=await readFile(path);probes.set(path,bytes.byteLength);return bytes}catch(error){if(error.code==="ENOENT"||error.code==="ENOTDIR")return null;throw error}},{manifestDirectory,generated:()=>undefined});
  return{...row,entry,bytes:inputs.reduce((n,input)=>n+input.bytes,0),files:inputs.length,activeLines:inputs.reduce((n,input)=>n+input.activeLines,0),probeBytes:[...probes.values()].reduce((n,size)=>n+size,0),probeFiles:probes.size,inputs,probes:[...probes].map(([path,bytes])=>({path,bytes}))};
 }catch(error){return{...row,entry,error:String(error),probes:[...probes].map(([path,bytes])=>({path,bytes}))}}
}));
await writeFile(resolve(import.meta.dir,"🗑️generated/registry-consolidation-extents.json"),JSON.stringify(rows,null,2)+"\n");
for(const{inputs,probes,...summary}of rows)console.log(JSON.stringify(summary));
if(rows.some(row=>"error"in row))process.exitCode=1;
