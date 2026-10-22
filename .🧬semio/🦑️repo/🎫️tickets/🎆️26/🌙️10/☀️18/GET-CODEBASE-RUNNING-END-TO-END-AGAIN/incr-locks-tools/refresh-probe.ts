import {incrementalTrackedFilesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/⌨️command/🟦️.ts";
import {incrementalRefreshV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏃️runner/🟦️.ts";
const [live,copy]=process.argv.slice(2) as [string,string],files=await incrementalTrackedFilesV1(live,new AbortController().signal);
console.log(JSON.stringify(await incrementalRefreshV1(live,copy,files,{signal:new AbortController().signal,progress:async(d,t)=>{console.log("progress",d,t);}})));
