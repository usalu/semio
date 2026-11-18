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
console.log("code revision",OB.projectRecordKeyTextV1(null));const full=process.argv[2];const m=await O.readOwnerPoolV1(client,full);console.log("manifest",m.units.length,"wave revision",m.head.revision,"created",new Date(m.head.createdMs).toISOString());
const N=Number(process.argv[3]||300);
const service=await import(PN+"/🏊️project-farm/👷️service/🟨️.mjs");const base=mkdtempSync(join(realpathSync(tmpdir()),"semio-r68-"));
await service.open({...deserialize(m.session),workspaceRoot:root,closureStore:undefined,stagingBase:base,authority:undefined},{slot:0,generation:1});const envr=service.sessionEnvironmentV1();
let units=m.units.slice();let seed=7;const rnd=()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296};
for(let i=units.length-1;i>0;i--){const j=Math.floor(rnd()*(i+1));[units[i],units[j]]=[units[j],units[i]];}
units=units.slice(0,N);
const host=P.hostOfRootV1(root);const hashCache=new Map();
const hashOf=p=>{if(hashCache.has(p))return hashCache.get(p);let h;try{h=createHash("sha256").update(fs.readFileSync(p)).digest("hex")}catch{h="GONE"}hashCache.set(p,h);return h;};
const SI0=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
{const SI1=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");SI1.bindClosureStoreV1(root,join(base,"ce"),true);}
SI0.bindClosureScriptsV1(root,SI0.createScriptInputCache({lateClosures:true}));
const port=PR.createProjectFleetPortV1({root,environment:env,client});const rows=[];let next=0;
const one=async unit=>{
 const job=R.ownerJobV1(envr,m.head.globals,unit);
 const keyText=R.ownerRecordKeyTextV1(R.projectRecordKeyV1(job.head,job.root,job.json,undefined,job.selectedInputs));
 const state=PR.projectRecordStateV1({root,fleet:job.root,keyText,platform:"darwin-arm64"});
 let got;const info={cachePath:join(base,"x.json"),fleet:job.root,keyText,exportRecord:()=>undefined,closures:SI0.closureTravelV1,import:async portable=>{got=portable;return true;}};
 const gate=await PR.runTakeLookV1(()=>SI0.runClosureTakeV1(()=>port.gate(info)));
 if(!got){rows.push({name:unit.name,state:"nogate",seeded:gate?.seeded});return;}
 const rec=got;const e={note:{createdMs:0,producer:"?"}};
 const SI=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const value=JSON.parse(rec.image);const obsr=rec.observations;
 const before=SI0.closureEntryCountsV1();let verify,err;
 try{verify=await SI0.runClosureTakeV1(async()=>{await OB.settleClosureRowsV1(obsr,value);return OB.projectRowsHoldV1(obsr,value,rest=>OB.observationRowsHoldV1(rest));});}catch(e){verify="throw:"+(e.code||"")+":"+String(e.message).slice(0,80)}
 const after=SI0.closureEntryCountsV1();const delta=Object.fromEntries(Object.keys(after).map(k=>[k,after[k]-before[k]]));
 const others=obsr.filter(r=>r.kind!=="closure");const othersHold=OB.observationRowsHoldV1(others);
 rows.push({name:unit.name,state:"hit",total:obsr.length,othersHold,verify,delta,closureRows:obsr.filter(r=>r.kind==="closure").length});
};
await Promise.all(Array.from({length:6},async()=>{while(next<units.length){const u=units[next++];try{await one(u)}catch(err){rows.push({name:u.name,state:"error",err:String(err.stack).slice(0,300)})}}}));
rmSync(base,{recursive:true,force:true});
for(const r of rows)console.log(JSON.stringify(r));process.exit(0);
