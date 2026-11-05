/** 📏️ Samples owned engine source with cfg=test; native admission remains authoritative. */
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFile,writeFile} from "node:fs/promises";
import {dirname,resolve} from "node:path";
const owners=[
["framework-headless","🧰️framework/🔨️modules/🧵️job/🏃️headless/📦️packages/🦀️rust/🦀️.rs"],
["engine-job","✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/📦️packages/🦀️rust/🦀️.rs"],
["grid3d-inferences","✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/🦀️.rs"]
] as const;
const results=[];
for(const [owner,entry]of owners){
try{
const inputs=await readRustSourceClosureV1([entry],process.cwd(),["test"],async path=>{try{return new Uint8Array(await readFile(path));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}},{manifestDirectory:dirname(resolve(entry)),generated:()=>undefined});
results.push({owner,cfg:["test"],authoritativeNativeGate:false,upperBound:false,files:inputs.length,bytes:inputs.reduce((n,row)=>n+row.bytes,0),activeLines:inputs.reduce((n,row)=>n+row.activeLines,0),inputs});
}catch(error){results.push({owner,authoritativeNativeGate:false,upperBound:false,error:String(error)});}
}
await writeFile(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/🗑️generated/engine-source-preflight.json",JSON.stringify(results,null,2)+"\n");
for(const row of results){const {inputs,...summary}=row as typeof row&{inputs?:unknown};console.log(JSON.stringify(summary));}
