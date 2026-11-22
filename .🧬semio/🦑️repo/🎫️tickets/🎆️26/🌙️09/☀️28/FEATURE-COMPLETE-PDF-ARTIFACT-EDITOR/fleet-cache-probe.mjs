const root="/Users/ueli/Documents/semio";
const base="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const {remoteCacheFetchV1,resolveRemoteCacheV1}=await import(`${root}/${base}🌐️remote-cache/🛰️fleet/🟦️.ts`);
const env={...process.env,SEMIO_WORKSPACE_ROOT:root};
const r=resolveRemoteCacheV1(env,{root});
console.log(JSON.stringify({endpoint:r?.contract.endpoint,from:r?.endpointFrom,tokenFrom:r?.tokenFrom}));
const f=remoteCacheFetchV1(env,{root});
for(const p of["ac","cas"]){const t=Date.now();try{const x=await f(`${r.contract.endpoint}/${r.contract.instance}/${p}/${"0".repeat(64)}`,{signal:AbortSignal.timeout(10000)});console.log(p,x.status,Date.now()-t,"ms",(await x.text()).slice(0,200));}catch(e){console.log(p,"ERR",e?.name,e?.message,e?.cause?.message);}}
const ms=[];let bad=0;for(let i=0;i<40;i++){const t=Date.now();try{const x=await f(`${r.contract.endpoint}/${r.contract.instance}/ac/${i.toString(16).padStart(64,"0")}`,{signal:AbortSignal.timeout(1500)});await x.text();ms.push(Date.now()-t);}catch(e){bad++;ms.push(-(Date.now()-t));}}
console.log("latencies",ms.join(","),"failures",bad);
