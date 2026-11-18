const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const {graphFleetClientV1}=await import(B+"/🪪️graph-state/🛰️exchange/🟦️.ts");
const O=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env);const client=graphFleetClientV1(s,{fetch:remoteCacheFetchV1(env,{root})});
const raw=await (async()=>{const {fetchGraphEntryV1}=await import(B+"/🪪️graph-state/🛰️exchange/🟦️.ts");const e=await fetchGraphEntryV1(client,O.ownerPoolStateV1("pointer"));return e;})();
const doc=raw.documents.find(d=>d.name===O.OWNER_POOL_POLICY_V1.pointerName);const ptr=JSON.parse(Buffer.from(doc.bytes).toString());
console.log(JSON.stringify(ptr,null,1));
for(const w of ptr.waves){const m=await O.readOwnerPoolV1(client,w.id);if(!m){console.log(w.id.slice(0,12),"manifest unreadable");continue;}
 console.log(w.id.slice(0,12),new Date(w.createdMs).toISOString(),"units",m.units.length,"rev",m.head.revision?.slice(0,16),"ident",m.head.identity.slice(0,12),"pub",m.head.publisher,JSON.stringify(m.head.globals));}
