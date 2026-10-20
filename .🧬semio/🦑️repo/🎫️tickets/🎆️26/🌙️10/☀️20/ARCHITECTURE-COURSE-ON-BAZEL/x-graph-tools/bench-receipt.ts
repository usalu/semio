/** ⏱️ Measures the warm-path cost of an admission receipt: serialize, parse and validate, then re-check every consumed fact against the disk.
 * usage: bun bench-receipt.ts ROOT_DIR ROWS     (run from a tree that contains the applied staged set; reads, never writes)
 */
import {readdirSync,statSync} from "node:fs";
import {join,relative} from "node:path";
import {createBuildAdmissionReceiptV1,readBuildAdmissionReceiptV1} from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import * as observation from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";

const [root,rowsArgument]=process.argv.slice(2),limit=Number(rowsArgument??"50000");
const files:string[]=[],walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){if(files.length>=limit)return;if(entry.name==="node_modules"||entry.name===".git"||entry.name===".cache")continue;const path=join(directory,entry.name);if(entry.isDirectory())walk(path);else if(entry.isFile()&&statSync(path).size<2000000)files.push(path);}};
walk(root!);
const time=async<T>(label:string,run:()=>Promise<T>|T):Promise<T>=>{const started=performance.now(),value=await run();console.log(label.padEnd(44),String(Math.round(performance.now()-started)).padStart(7),"ms");return value;};
const captured=await time("capture "+files.length+" file reads (cold, sha256)",()=>observation.captureObservedSourceInputsV1(()=>{for(const file of files)observation.observedFs.readFileSync(file);return files.length;},undefined,false));
const admission={owners:["bench"],files:files.slice(0,1000).map(file=>relative(root!,file)),targets:{bench:["build"]},catalogProviders:[],inputs:{}},module={implementation:"1".repeat(64),publication:"2".repeat(64)};
const text=await time("serialize + validate receipt",()=>createBuildAdmissionReceiptV1({version:1,scope:"a".repeat(64),module,startedAt:Date.now(),admission,observations:captured.observations}));
console.log("receipt bytes".padEnd(44),String(Buffer.byteLength(text)).padStart(7),"B,",captured.observations.length,"rows");
const receipt=await time("read + parse + validate receipt",()=>readBuildAdmissionReceiptV1(text,{scope:"a".repeat(64),module}));
const holds=await time("observationsHold (warm page cache)",()=>observation.observationsHold(receipt!.observations,64,new Set(),new Set()));
console.log("holds",holds);
