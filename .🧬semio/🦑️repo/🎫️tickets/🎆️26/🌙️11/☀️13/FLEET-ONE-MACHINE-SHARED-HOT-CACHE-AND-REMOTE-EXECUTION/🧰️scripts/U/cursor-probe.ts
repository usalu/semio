import {graphJournalPortV1,manifestTrustV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📍️cursor/🟦️.ts";
const root="/Users/Shared/semio";
console.log("trust",await manifestTrustV1(root));
const say:string[]=[];
const port=graphJournalPortV1(root,()=>({watches:"all",cursor:async()=>undefined,since:async()=>({complete:false,reason:"unrecorded"}),excluded:()=>()=>false}) as never,r=>say.push(r));
let t=performance.now();
const c=await port.cursor();
console.log("cursor",JSON.stringify(c),"ms",Math.round(performance.now()-t),"say",say,"reason",port.manifest.lastReason());
