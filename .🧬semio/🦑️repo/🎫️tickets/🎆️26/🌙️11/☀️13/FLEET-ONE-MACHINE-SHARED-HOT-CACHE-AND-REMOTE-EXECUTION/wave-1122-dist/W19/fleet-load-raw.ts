const env={...process.env,SEMIO_WORKSPACE_ROOT:"/Users/ueli/Documents/semio"};
const {remoteCacheFetchV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts");
const call=remoteCacheFetchV1(env,{root:"/Users/ueli/Documents/semio",base:fetch,audience:"execution",roleOf:()=>"submit"});
const r=await call("http://192.168.178.70:8981/v1/fleet/load",{signal:AbortSignal.timeout(8000)});const t=await r.json() as any;
for(const m of t.machines)if(["g1-m04","g1-m05","g1-m01"].includes(m.origin))console.log("[DEBUG]",JSON.stringify(m.counters).slice(0,1500));
