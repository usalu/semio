import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🎫️lease/🟦️.ts";
import {withCompilerWriterCustodyV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";

/** 🔬️ Two concurrent compiler writers on one image root: does the second wait for the first body to finish? */
const lease="/private/tmp/conc-cargo/wp/lease",image={root:"/private/tmp/conc-cargo/wp/img",maximumPhysicalBytes:1<<40,recheck:async()=>{}};
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"conc-cargo-writer-proof",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{
 const control=scriptCompilerAdmissionControlV1(invocation,"rust:writer-proof",120000),log:string[]=[],t0=Date.now();
 const writer=(name:string,hold:number)=>withCompilerWriterCustodyV1(image,lease,control,{version:1,wait:true},async()=>{log.push(`${name} start +${Date.now()-t0}ms`);await new Promise(done=>setTimeout(done,hold));log.push(`${name} end +${Date.now()-t0}ms`);});
 await Promise.all([writer("A",1500),writer("B",1500),writer("C",1500)]);
 console.log(log.join("\n"));
});
