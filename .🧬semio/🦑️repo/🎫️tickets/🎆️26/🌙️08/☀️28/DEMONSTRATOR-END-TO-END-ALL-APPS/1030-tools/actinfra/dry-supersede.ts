import {supersedeBuildGraphObservationBlocksV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts";
const r=await supersedeBuildGraphObservationBlocksV1(process.argv[2]!,{lease:async()=>({release(){}})},{cancelled:()=>false,progress:()=>{}},{dryRun:true});
console.log(JSON.stringify(r));
