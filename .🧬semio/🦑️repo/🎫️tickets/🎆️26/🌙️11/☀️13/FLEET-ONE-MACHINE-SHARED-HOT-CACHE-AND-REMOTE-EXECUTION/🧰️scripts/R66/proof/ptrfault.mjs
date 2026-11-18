const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const{graphFleetClientV1,fetchGraphEntryV1}=await import(B+"/🪪️graph-state/🛰️exchange/🟦️.ts");
const{graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const{remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");
const pool=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts");
const env={...process.env};const base=remoteCacheFetchV1(env,{root}),codes={};const wrapped=async(u,i)=>{try{const r=await base(u,i);codes[r.status]=(codes[r.status]||0)+1;return r}catch(e){codes['throw:'+String(e?.message).slice(0,60)]=(codes['throw:'+String(e?.message).slice(0,60)]||0)+1;throw e}};const client=graphFleetClientV1(graphFleetSettingsV1(root,env),{fetch:wrapped});
const lat=[];let miss=0,fault=0,ok=0;
for(let i=0;i<60;i++){const t=Date.now();const e=await fetchGraphEntryV1(client,pool.ownerPoolStateV1("pointer"));lat.push(Date.now()-t);if(e==="fault")fault++;else if(e===undefined)miss++;else ok++;await new Promise(r=>setTimeout(r,500));}
lat.sort((a,b)=>a-b);console.log(JSON.stringify({ok,miss,fault,p50:lat[30],p90:lat[54],max:lat[59]}));console.log(JSON.stringify(codes));process.exit(0)
