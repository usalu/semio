const LIB="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/";
const fleet=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts");
const {resolveRemoteExecutionEndpointV1,resolveRemoteExecutionSchedulerV1}=await import(LIB+"🧭️endpoint/🟦️.ts");
const {remoteExecutionOriginV1}=await import(LIB+"🌡️load/🟦️.ts");
const root="/Users/ueli/Documents/semio",environment=process.env;
const published=resolveRemoteExecutionEndpointV1({environment,root})?.endpoint;
const scheduler=await resolveRemoteExecutionSchedulerV1({environment,root},published!);
const call=fleet.remoteCacheFetchV1({...environment,SEMIO_WORKSPACE_ROOT:root},{root,audience:"execution",roleOf:()=>"submit"});
const base=scheduler!.url.replace(/\/+$/u,""),own=remoteExecutionOriginV1(root,environment);
const out:Record<string,unknown>={own};
for(const peak of [0,1024,2048,3072,4096,5120,6144,7168,8192]){const r=await call(base+"/v1/fleet/idle?origin="+encodeURIComponent(own)+(peak>0?"&peak="+peak:""),{signal:AbortSignal.timeout(8000)});out["idle@"+peak]=await r.json();}
const r=await call(base+"/v1/fleet/load",{signal:AbortSignal.timeout(8000)});out.load=r.ok?await r.json():r.status;
console.log(JSON.stringify(out));
