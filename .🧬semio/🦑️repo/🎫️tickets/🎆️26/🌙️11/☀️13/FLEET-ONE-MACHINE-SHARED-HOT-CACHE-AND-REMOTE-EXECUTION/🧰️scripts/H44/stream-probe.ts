import {offloadCheckoutTestV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🔬️checkout-test/🔌️client/🟦️.ts";
import {openOffloadSessionV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts";
const root="/Users/ueli/Documents/semio",env={...process.env,SEMIO_WORKSPACE_ROOT:root};
const s:any=await openOffloadSessionV1({root:"/tmp/h44-probe",environment:env});
let reads=0,hits=0;const t0=Date.now();
const out=await offloadCheckoutTestV1({root,memoRoot:"/tmp/h44-probe",environment:env,file:process.argv[2]!,budgetSeconds:120,language:"en",onOutput:(t,o)=>console.log("LIVE",Date.now()-t0,o,JSON.stringify(t.slice(0,60))),ports:{readChunk:async(h,sig)=>{reads++;const r=await s.content.readActionResult(s.instance,h,sig);if(r)hits++;return r;}}});
console.log(out.kind,out.kind==="done"?{origin:out.origin,streamed:out.streamed,err:out.stderr.length}:out,"reads",reads,"hits",hits,Date.now()-t0);
