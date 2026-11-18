import{graphFleetClientV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import{readOwnerPoolPointerV1,readOwnerPoolV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts";
import{projectRecordKeyTextV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
const seconds=Number(process.argv[2]??120),R="/Users/ueli/Documents/semio",env={...process.env};for(const n of["SEMIO_REMOTE_CACHE_ENDPOINT","SEMIO_REMOTE_CACHE_ACCESS","SEMIO_REMOTE_CACHE_INSTANCE","SEMIO_REMOTE_CACHE_TOKEN"])delete env[n];
const c=graphFleetClientV1(graphFleetSettingsV1(R,env)!,{fetch:remoteCacheFetchV1(env,{root:R})}),rev=projectRecordKeyTextV1(null),t0=Date.now(),seen=new Set<string>();
console.log("revision",rev.slice(0,14),"since",new Date().toISOString());
while(Date.now()-t0<seconds*1000){
 const p=await readOwnerPoolPointerV1(c);
 if(p&&p!=="fault")for(const w of p.waves){if(seen.has(w.id))continue;seen.add(w.id);const m=await readOwnerPoolV1(c,w.id);console.log(new Date().toISOString(),"wave",w.id.slice(0,12),"units",w.units,"created",new Date(w.createdMs).toISOString(),"current-revision",m?.head.revision===rev,m?.head.publisher.split(":")[0].slice(-40));}
 await new Promise(r=>setTimeout(r,5000));
}
process.exit(0);
