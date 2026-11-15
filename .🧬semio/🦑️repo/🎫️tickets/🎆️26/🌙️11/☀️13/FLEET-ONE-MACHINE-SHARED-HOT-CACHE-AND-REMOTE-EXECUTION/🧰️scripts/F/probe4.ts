import {readReplicaFilesV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📥️replica/🟦️.ts";
import {readTreeFilesV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🌳️tree/🟦️.ts";
import {classifyGraphInputV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🟦️.ts";
const root="/Users/Shared/semio",memo="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/F/memo.json";
let t=performance.now();
const tr=await readTreeFilesV1(root,{memoFile:memo});console.log("tree2",Math.round(performance.now()-t),"ms read",tr.read,"memo",tr.memoised);
const r=await readReplicaFilesV1(root);
const tm=new Map(tr.files);let diff=0,miss=0;const ex:string[]=[];
for(const [p,s] of r!.files()){const k=classifyGraphInputV1(p);if(k!=="content")continue;const o=tm.get(p);if(o===undefined){miss++;continue;}if(o!==s){diff++;if(ex.length<10)ex.push(p);}}
console.log({diff,miss,ex});
