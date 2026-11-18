const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");
const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const ex=await import(B+"/🪪️graph-state/🛰️exchange/🟦️.ts");
const pool=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env),f=remoteCacheFetchV1(env,{root});
const key=ex.graphEntryKeyV1(pool.ownerPoolStateV1("pointer"));
const N=Number(process.argv[2]??300),C=Number(process.argv[3]??3);const lat=[];let bad=0;
async function one(){const t=performance.now();try{const r=await f(`${s.endpoint}/${s.instance}/ac/${key}`,{method:"GET",signal:AbortSignal.timeout(10000)});await r.arrayBuffer();if(r.status!==200)bad++;}catch{bad++;}lat.push(performance.now()-t);}
let i=0;await Promise.all(Array.from({length:C},async()=>{while(i++<N)await one();}));
lat.sort((a,b)=>a-b);const q=p=>Math.round(lat[Math.min(lat.length-1,Math.floor(lat.length*p))]);
console.log(JSON.stringify({n:lat.length,bad,p50:q(.5),p90:q(.9),p99:q(.99),p999:q(.999),max:Math.round(lat.at(-1)),over1500:lat.filter(x=>x>1500).length,over500:lat.filter(x=>x>500).length}));
