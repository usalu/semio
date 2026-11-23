import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const root="C:\\semio\\🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\🏗️build\\";
const observation=await import(pathToFileURL(root+"👁️observation\\🟨️.mjs").href);
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const kinds=new Set((process.argv[3]??"lstat").split(","));
let rows=(memo.observations as {kind:string;path:string}[]).filter(r=>kinds.has(r.kind)).slice(0,Number(process.argv[4]??30000));
const plain=async()=>{const t=performance.now();const ok=await observation.observationsHoldWithinRoundV1(rows,64);return {ok,ms:Math.round(performance.now()-t)};};
console.log("plain",JSON.stringify(await plain()),"rows",rows.length);
console.log("plain again",JSON.stringify(await plain()));
const t=performance.now();
const captured=await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{
 const s=performance.now();const ok=await observation.observationsHoldWithinRoundV1(rows,64);
 return {ok,ms:Math.round(performance.now()-s)};
}));
console.log("inside capture+settle",JSON.stringify(captured.value),"recorded observations",captured.observations.length,"total ms",Math.round(performance.now()-t));
