/** 📏️ Inventories physical Rust test Source without issuing compiler, Original, or budget authority. */
import {readFileSync,statSync,writeFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const [platform,output,owner="editor"]=process.argv.slice(2);
if(!["editor","patch"].includes(owner))throw Error("Closed diagnostic owner required");
if(!["macos","linux","windows"].includes(platform??"")||!output)throw Error("source-inventory.ts macos|linux|windows <ticket/generated/output.json>");
const root=process.cwd(),ticket=resolve(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FORMS-PLUGIN-END-TO-END/🗑️generated"),destination=resolve(root,output);
if(!destination.startsWith(ticket+"/")&&!destination.startsWith(ticket+"\\"))throw Error("Ticket generated output required");
const arch=platform==="macos"?"aarch64":"x86_64",family=platform==="windows"?"windows":"unix",environment=platform==="macos"?"":platform==="windows"?"msvc":"gnu",vendor=platform==="macos"?"apple":platform==="windows"?"pc":"unknown";
const cfg=["test","debug_assertions",family,'target_arch="'+arch+'"','target_os="'+platform+'"','target_family="'+family+'"','target_env="'+environment+'"','target_vendor="'+vendor+'"','target_pointer_width="64"','target_endian="little"','panic="unwind"'];
const manifest=resolve(root,owner==="editor"?"✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml":"✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-questions/🧪️tests/📦️packages/🦀️rust/Cargo.toml");
const declaration=Bun.TOML.parse(readFileSync(manifest,"utf8")) as {lib:{path:string}},entry=resolve(dirname(manifest),declaration.lib.path);
const rows=await readRustSourceClosureV1([entry],root,cfg,async path=>{try{if(statSync(path).size>1048576)throw Error("Finite diagnostic input required");return new Uint8Array(readFileSync(path));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}},{manifestDirectory:dirname(manifest),generated:()=>{throw Error("Actual compiler ownership required for generated diagnostic input");}});
if(rows.length>128)throw Error("Finite diagnostic closure required");
const bytes=rows.reduce((sum,row)=>sum+row.bytes,0),report={platform,owner,manifestDirectory:dirname(manifest),cfg,files:rows.length,activeLines:rows.reduce((sum,row)=>sum+row.activeLines,0),bytes,comparisonLimit:262144,headroom:262144-bytes,embedded:rows.filter(row=>row.kind==="embedded"),all:rows};
writeFileSync(destination,JSON.stringify(report,null,2)+"\n");
console.log(JSON.stringify({platform,owner,files:rows.length,activeLines:report.activeLines,bytes,headroom:report.headroom}));
