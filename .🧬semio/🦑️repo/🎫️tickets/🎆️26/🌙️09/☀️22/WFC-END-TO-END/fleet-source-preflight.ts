/** 📏️ Diagnostic source closure using the same cfg=test sample as bitmap; not native authority. */
import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { dirname, resolve } from "node:path";
const root="✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/";
const owners=[
["wfc2d-v1",root+"◻️2d/🏅️standards/🔖️1/📦️packages/🦀️rust/🦀️.rs"],
["grid2d-v1",root+"🔲️grid2d/🏅️standards/🔖️1/📦️packages/🦀️rust/🦀️.rs"],
["wfc3d-editor",root+"🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/🦀️.rs"],
["grid3d-editor",root+"🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/🦀️.rs"]
] as const;
const results=[];
for(const [owner,entry] of owners){
try{
const inputs=await readRustSourceClosureV1([entry],process.cwd(),["test"],async path=>{try{return new Uint8Array(await readFile(path));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}},{manifestDirectory:dirname(resolve(entry)),generated:()=>undefined});
results.push({owner,entry,cfg:["test"],authoritativeNativeGate:false,upperBound:false,files:inputs.length,bytes:inputs.reduce((n,x)=>n+x.bytes,0),activeLines:inputs.reduce((n,x)=>n+x.activeLines,0),inputs});
}catch(error){results.push({owner,entry,cfg:["test"],authoritativeNativeGate:false,upperBound:false,error:String(error)});}
}
const output=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/🗑️generated/fleet-source-preflight.json";
await mkdir(dirname(output),{recursive:true});
await writeFile(output,JSON.stringify(results,null,2)+"\n");
for(const row of results){const {inputs,...summary}=row as typeof row & {inputs?:unknown};console.log(JSON.stringify(summary));}
