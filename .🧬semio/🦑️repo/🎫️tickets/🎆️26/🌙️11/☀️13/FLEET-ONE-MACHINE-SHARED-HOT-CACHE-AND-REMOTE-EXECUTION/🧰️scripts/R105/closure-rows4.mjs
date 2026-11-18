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
const SAm=await import(B+"/🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs"),SIm=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const PROV=[...Object.values(SAm.prepareSourceAnalysesV1()).map(p=>p.id),...SIm.scriptProviderIdsV1()];console.log("providers",PROV.length);
const rows=[];let next=0;
const memo=new Map();
const one=async unit=>{
 const job=R.ownerJobV1(envr,m.head.globals,unit);
 const keyText=R.ownerRecordKeyTextV1(R.projectRecordKeyV1(job.head,job.root,job.json,undefined,job.selectedInputs));
 const state=PR.projectRecordStateV1({root,fleet:job.root,keyText,platform:"darwin-arm64"});
 const e=await X.fetchGraphEntryV1(client,state,undefined,name=>name.startsWith("closure-"));
 if(e===undefined||e==="fault"){rows.push({name:unit.name,state:String(e)});return;}
 const out={name:unit.name,keyDigest:state.digest.slice(0,12),created:new Date(e.note.createdMs).toISOString(),producer:e.note.producer.split(":")[0],holds:true,bad:{},examples:[]};
 const docs=[];for(const [name,ent] of e.deferred){docs.push({name,hash:ent.hash,ent});}
 const bundles=[];for(const d of docs){const doc=await d.ent.read({keep:true});bundles.push({recipeKey:d.name.slice(8),bundle:P.unpackBundleV1(JSON.parse(Buffer.from(doc.bytes).toString())),hash:d.hash});}
 const all=new Map([...P.bundleKeysV1(bundles.map(d=>({recipeKey:d.recipeKey,bundle:d.bundle})),host,"local")]);
 for(const d of bundles){
  if(memo.has(d.hash)){const v=memo.get(d.hash);if(!v.holds){out.holds=false;for(const k in v.bad)out.bad[k]=v.bad[k];out.examples=v.examples;}continue;}
  const moved=P.localBundleV1({recipeKey:d.recipeKey,bundle:d.bundle},host,all,{providers:PROV,generation:P.dependencyGenerationV1(root)});
  const v={holds:true,bad:{},examples:[]};
  if(!moved)v.holds=false,v.bad.unlocalizable=1;else{const rowsAll=[...moved.bundle.rows];for(const layer of moved.bundle.layers)rowsAll.push(...layer);
   for(const r of rowsAll){let ok;try{ok=OB.observationRowFreshV1(r);}catch{ok=false}if(ok!==true){v.holds=false;v.bad[r.kind]=(v.bad[r.kind]??0)+1;if(v.examples.length<3)v.examples.push({kind:r.kind,path:String(r.path).replace(root,"~").slice(0,110)});}}}
  memo.set(d.hash,v);if(!v.holds){out.holds=false;for(const k in v.bad)out.bad[k]=v.bad[k];out.examples=v.examples;}
 }
 rows.push(out);
};
await Promise.all(Array.from({length:6},async()=>{while(next<units.length){const u=units[next++];try{await one(u)}catch(err){rows.push({kind:u.kind,state:"error"})}}}));
rmSync(base,{recursive:true,force:true});
for(const r of rows)console.log(JSON.stringify(r));process.exit(0);
