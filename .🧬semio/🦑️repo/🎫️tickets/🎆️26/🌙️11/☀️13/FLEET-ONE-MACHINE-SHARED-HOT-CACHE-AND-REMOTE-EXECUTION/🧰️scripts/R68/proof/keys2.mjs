import { mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { deserialize } from "node:v8";
const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const PN=B+"/🕸️graph/📥️inference/📦️project-nodes";
const {libraryBootstrap}=await import(B+"/🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const X=await import(B+"/🪪️graph-state/🛰️exchange/🟦️.ts");
const O=await import(PN+"/🌐️owner-pool/🟦️.ts");
const R=await import(PN+"/🟨️.mjs");
const PR=await import(B+"/🪪️graph-state/📦️project-records/🟦️.ts");const P=await import(B+"/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const OB=await import(B+"/👁️observation/🟨️.mjs");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env);const client=X.graphFleetClientV1(s,{fetch:remoteCacheFetchV1(env,{root})});
const waveId=process.argv[2];const ptr=JSON.parse(Buffer.from((await X.fetchGraphEntryV1(client,O.ownerPoolStateV1("pointer"))).documents.find(d=>d.name===O.OWNER_POOL_POLICY_V1.pointerName).bytes).toString());
const full=ptr.waves.map(w=>w.id).find(i=>i.startsWith(waveId));const m=await O.readOwnerPoolV1(client,full);
console.log("wave",full.slice(0,12),"units",m.units.length);
const service=await import(PN+"/🏊️project-farm/👷️service/🟨️.mjs");const base=mkdtempSync(join(realpathSync(tmpdir()),"semio-r68-"));
const session={...deserialize(m.session),workspaceRoot:root,closureStore:undefined,stagingBase:base,authority:undefined};
await service.open(session,{slot:0,generation:1});const envr=service.sessionEnvironmentV1();
let poolHit,readerHit,both;const decodeP=r=>{return decodeProtobufV1(REAPI_V1.actionResult,r)};const {REAPI_V1,decodeProtobufV1}=await import(B+"/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");const names=process.argv.slice(3);const pick=names.length?m.units.filter(u=>names.includes(u.name)):m.units;
const out=[];const stampSchema=(await import(B+"/👁️observation/🧬️schema/🔣️.json",{with:{type:"json"}})).default;const STAMP=stampSchema.$defs.record.const.stamp;console.log("record stamp",STAMP);
for(const unit of pick){
 const job=R.ownerJobV1(envr,m.head.globals,unit);
 const keyText=OB.projectRecordKeyTextV1(R.projectRecordKeyV1(job.head,job.root,job.json,undefined,job.selectedInputs));
 const state=PR.projectRecordStateV1({root,fleet:job.root,keyText,platform:"darwin-arm64"});
 const key=X.graphEntryKeyV1(state);
 const res=await client.getResult(key);
 const keyText2=JSON.stringify([STAMP,null,R.projectRecordKeyV1(job.head,job.root,job.json,undefined,job.selectedInputs)]);
 const state2=PR.projectRecordStateV1({root,fleet:job.root,keyText:keyText2,platform:"darwin-arm64"});const res2=await client.getResult(X.graphEntryKeyV1(state2));
 let note2;if(typeof res2!=="string"){const {REAPI_V1,decodeProtobufV1}=await import(B+"/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");const r=decodeProtobufV1(REAPI_V1.actionResult,res2);note2=JSON.parse(Buffer.from(r.stdout_raw).toString());}
 poolHit=(poolHit??0)+(typeof res!=="string"?1:0);readerHit=(readerHit??0)+(typeof res2!=="string"?1:0);both=(both??0)+(typeof res!=="string"&&typeof res2!=="string"?1:0);
 if(process.env.SHOW)console.log("SHOW",unit.name.padEnd(48),"pool-key:",typeof res!=="string"?"HIT "+new Date(JSON.parse(Buffer.from(decodeP(res).stdout_raw)).createdMs).toISOString().slice(11,19):"miss","| reader-key:",typeof res2!=="string"?"HIT "+new Date(note2.createdMs).toISOString().slice(11,19)+" by "+String(note2.producer).split(":")[0].slice(0,26):"miss");
 let note;if(typeof res!=="string"){const {REAPI_V1,decodeProtobufV1}=await import(B+"/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");const r=decodeProtobufV1(REAPI_V1.actionResult,res);note=JSON.parse(Buffer.from(r.stdout_raw).toString());}
 if(process.env.ANALYZE&&typeof res!=="string"){
  const {createHash}=await import("node:crypto");const fs=await import("node:fs");
  const entry=await X.fetchGraphEntryV1(client,state);const doc=entry.documents.find(d=>d.name==="record");
  const neutral=P.unpackRecordV1(JSON.parse(Buffer.from(doc.bytes).toString()));
  const taken=P.localRecordV1(neutral,P.hostOfRootV1(root),{providers:[],generation:undefined}).record;
  const kinds={},bad={};let badfiles=[];
  for(const o of taken.observations){kinds[o.kind]=(kinds[o.kind]??0)+1;
   const f=o.kind==="file"?o.fact:o.kind==="analysis"?o.fact?.source:undefined;
   if(f&&typeof f.sha256==="string"&&typeof o.path==="string"){let h;try{h=createHash("sha256").update(fs.readFileSync(o.path)).digest("hex")}catch{h="GONE"}
     if(h!==f.sha256){bad[o.kind]=(bad[o.kind]??0)+1;if(badfiles.length<6)badfiles.push(o.path.replace(root+"/","").slice(-110));}}}
  console.log("ANALYZE",unit.name,"rows",taken.observations.length,JSON.stringify(kinds),"STALE",JSON.stringify(bad),JSON.stringify(badfiles));
 }
 out.push({name:unit.name,kind:unit.kind,key:process.env.FULLKEY?key:key.slice(0,16),state:state.digest.slice(0,12),stored:res==="miss"?"MISS":typeof res==="string"?res:"HIT",producer:note?.producer,createdMs:note?.createdMs,head0:job.head.map(x=>x===null?null:String(x).slice(0,10))});
}
rmSync(base,{recursive:true,force:true});
console.log("RESULT units",pick.length,"pool-key HIT",poolHit,"reader-key HIT",readerHit,"both",both);
console.log(JSON.stringify(out.map(o=>({...o,created:o.createdMs?new Date(o.createdMs).toISOString():undefined,createdMs:undefined})),null,0).replace(/\},\{/g,"},\n{"));
process.exit(0);
