import { createHash } from "node:crypto";import fs from "node:fs";
import { mkdtempSync, realpathSync, rmSync } from "node:fs";import { tmpdir } from "node:os";import { join } from "node:path";import { deserialize } from "node:v8";
const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const PN=B+"/🕸️graph/📥️inference/📦️project-nodes";
const {libraryBootstrap}=await import(B+"/🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const X=await import(B+"/🪪️graph-state/🛰️exchange/🟦️.ts");const O=await import(PN+"/🌐️owner-pool/🟦️.ts");const R=await import(PN+"/🟨️.mjs");
const PR=await import(B+"/🪪️graph-state/📦️project-records/🟦️.ts");const P=await import(B+"/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");const OB=await import(B+"/👁️observation/🟨️.mjs");
const {REAPI_V1,decodeProtobufV1}=await import(B+"/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env);const client=X.graphFleetClientV1(s,{fetch:remoteCacheFetchV1(env,{root})});
const ptr=JSON.parse(Buffer.from((await X.fetchGraphEntryV1(client,O.ownerPoolStateV1("pointer"))).documents.find(d=>d.name===O.OWNER_POOL_POLICY_V1.pointerName).bytes).toString());
const full=ptr.waves.map(w=>w.id).find(i=>i.startsWith(process.argv[2]));const m=await O.readOwnerPoolV1(client,full);
const N=Number(process.argv[3]||300);
const service=await import(PN+"/🏊️project-farm/👷️service/🟨️.mjs");const base=mkdtempSync(join(realpathSync(tmpdir()),"semio-r68-"));
await service.open({...deserialize(m.session),workspaceRoot:root,closureStore:undefined,stagingBase:base,authority:undefined},{slot:0,generation:1});const envr=service.sessionEnvironmentV1();
let units=m.units.slice();let seed=7;const rnd=()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296};
for(let i=units.length-1;i>0;i--){const j=Math.floor(rnd()*(i+1));[units[i],units[j]]=[units[j],units[i]];}
units=units.slice(0,N);
const host=P.hostOfRootV1(root);const hashCache=new Map();
const hashOf=p=>{if(hashCache.has(p))return hashCache.get(p);let h;try{h=createHash("sha256").update(fs.readFileSync(p)).digest("hex")}catch{h="GONE"}hashCache.set(p,h);return h;};
const rows=[];let next=0;
const one=async unit=>{
 const job=R.ownerJobV1(envr,m.head.globals,unit);
 const keyText=OB.projectRecordKeyTextV1(R.projectRecordKeyV1(job.head,job.root,job.json,undefined,job.selectedInputs));
 const state=PR.projectRecordStateV1({root,fleet:job.root,keyText,platform:"darwin-arm64"});
 const e=await X.fetchGraphEntryV1(client,state);
 if(e===undefined){rows.push({kind:unit.kind,state:"miss"});return;}if(e==="fault"){rows.push({kind:unit.kind,state:"fault"});return;}
 let verdict="fresh",bad=0,total=0;
 try{const doc=e.documents.find(d=>d.name==="record");const neutral=P.unpackRecordV1(JSON.parse(Buffer.from(doc.bytes).toString()));
  const rec=P.localRecordV1(neutral,host,{providers:[],generation:undefined}).record;total=rec.observations.length;
  for(const o of rec.observations){const f=o.kind==="file"?o.fact:o.kind==="analysis"?o.fact?.source:undefined;if(f&&typeof f.sha256==="string"&&typeof o.path==="string"&&hashOf(o.path)!==f.sha256)bad++;}
  if(bad>0)verdict="stale";}catch(err){verdict="undecodable";}
 rows.push({kind:unit.kind,state:verdict,bad,total,created:e.note.createdMs,producer:e.note.producer.split(":")[0]});
};
await Promise.all(Array.from({length:6},async()=>{while(next<units.length){const u=units[next++];try{await one(u)}catch(err){rows.push({kind:u.kind,state:"error"})}}}));
rmSync(base,{recursive:true,force:true});
const by={};for(const r of rows){const k=r.kind+"|"+r.state;by[k]=(by[k]??0)+1;}
console.log("sample",rows.length,JSON.stringify(by));
const t=(x)=>Math.floor(x/3600000);const hour={};for(const r of rows)if(r.created){const k=new Date(r.created).toISOString().slice(5,13)+"|"+r.state;hour[k]=(hour[k]??0)+1;}
console.log(JSON.stringify(Object.fromEntries(Object.entries(hour).sort())));
const sm=rows.filter(r=>r.state==="stale");console.log("stale bad rows median",sm.map(r=>r.bad).sort((a,b)=>a-b)[Math.floor(sm.length/2)]);
process.exit(0);
