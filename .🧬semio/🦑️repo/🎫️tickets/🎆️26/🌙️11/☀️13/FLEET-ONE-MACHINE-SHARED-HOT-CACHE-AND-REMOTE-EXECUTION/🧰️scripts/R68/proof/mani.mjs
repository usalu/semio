const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const X=await import(B+"/🪪️graph-state/🛰️exchange/🟦️.ts");const O=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env);const client=X.graphFleetClientV1(s,{fetch:remoteCacheFetchV1(env,{root})});
const ptr=JSON.parse(Buffer.from((await X.fetchGraphEntryV1(client,O.ownerPoolStateV1("pointer"))).documents.find(d=>d.name===O.OWNER_POOL_POLICY_V1.pointerName).bytes).toString());
for(const w of ptr.waves){const st=O.ownerPoolStateV1("manifest",w.id);const r=await client.getResult(X.graphEntryKeyV1(st));console.log(w.id.slice(0,12),new Date(w.createdMs).toISOString(),w.units,"ac:",typeof r==="string"?r:"present "+r.length+" bytes","slot",X.graphEntrySlotV1(st).slice(0,12));}
