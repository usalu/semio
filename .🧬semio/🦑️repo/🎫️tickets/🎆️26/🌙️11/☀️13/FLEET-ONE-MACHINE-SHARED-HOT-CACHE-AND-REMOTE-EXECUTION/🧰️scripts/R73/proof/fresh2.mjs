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
let poolHit,readerHit,both;const decodeP=r=>{return decodeProtobufV1(REAPI_V1.actionResult,r)};const {REAPI_V1,decodeProtobufV1}=await import(B+"/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");const names=process.argv.slice(3);const N=Number(process.env.N??60);const pick=(names.length?m.units.filter(u=>names.includes(u.name)):m.units.filter((u,i)=>i%Math.max(1,Math.floor(m.units.length/N))===0)).slice(0,N);
const out=[];const stampSchema=(await import(B+"/👁️observation/🧬️schema/🔣️.json",{with:{type:"json"}})).default;const STAMP=stampSchema.$defs.record.const.stamp;console.log("record stamp",STAMP);

const FR=await import(B+"/🪪️graph-state/📦️project-records/🌱️fresh/🟦️.ts");await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const CUT=Date.parse(new Date().toISOString().slice(0,10)+"T04:05:00Z");const z=()=>({"post-fresh":0,"post-stale":0,"pre-fresh":0,"pre-stale":0,miss:0,fault:0});const tally={pool:z(),reader:z()};
async function verdict(state){
 const res=await client.getResult(X.graphEntryKeyV1(state));if(typeof res==="string")return "miss";if(JSON.parse(Buffer.from(decodeP(res).stdout_raw)).createdMs<=CUT)return "pre-stale";

 const found=await X.fetchGraphEntryV1(client,state,e=>e.path!=="record.br");
 if(found===undefined)return "miss";if(found==="fault")return "fault";
 const doc=found.documents.find(d=>d.name==="record");if(!doc)return "miss";
 const text=Buffer.from(doc.bytes).toString("utf8");
 const taken=P.localRecordV1(P.unpackRecordV1(JSON.parse(text)),P.hostOfRootV1(root),{providers:[],generation:undefined}).record;
 const rows=taken.observations,closures=rows.filter(r=>r.kind==="closure");let holds;
 if(closures.length){try{await OB.settleClosureRowsV1(closures,JSON.parse(taken.image));holds=row=>OB.observationRowsHoldV1([row]);}catch{}}
 const v=FR.freshVerdictV1({rows,hashOf:FR.fileDigestV1,closureHolds:holds,createdMs:found.note.createdMs,now:Date.now()});
 if(!v.fresh&&found.note.createdMs>CUT){{const res=closures.map(r=>[OB.observationRowsHoldV1([r]),JSON.parse(r.path)[1].slice(0,6),JSON.stringify(JSON.parse(r.path)[2]).slice(0,50)]);console.log("ROWS",JSON.stringify(res));}console.log("POSTSTALE age s",Math.round((Date.now()-found.note.createdMs)/1000));const f=FR.freshVerdictV1({rows,hashOf:FR.fileDigestV1,createdMs:found.note.createdMs,now:Date.now()});const bad=FR.observedDigestsV1(rows).filter(d=>FR.fileDigestV1(d.path)!==d.sha256).map(d=>d.path.replace(root+"/","").slice(-70));if(!globalThis.__p){globalThis.__p=1;const val=JSON.parse(taken.image);console.log("VALUEKEYS",Object.keys(val).slice(0,30).join(","),"recipes",Object.keys(val.closureRecipes??{}).length);for(const r of closures.slice(0,3)){console.log("ROW",r.path.slice(0,200),"FACT",String(r.fact).slice(0,80));}}console.log("STALE file-rows",f.stale,"of",f.checked,"closure rows",closures.length,"total stale",v.stale,JSON.stringify(bad.slice(0,2)));}
 const post=found.note.createdMs>CUT;return (post?"post-":"pre-")+(v.fresh?"fresh":"stale");
}
for(const unit of pick){
 const job=R.ownerJobV1(envr,m.head.globals,unit);const rk=R.projectRecordKeyV1(job.head,job.root,job.json,undefined,job.selectedInputs);
 const sp=PR.projectRecordStateV1({root,fleet:job.root,keyText:OB.projectRecordKeyTextV1(rk),platform:"darwin-arm64"});
 const sr=PR.projectRecordStateV1({root,fleet:job.root,keyText:JSON.stringify([STAMP,null,rk]),platform:"darwin-arm64"});
 tally.pool[await verdict(sp)]++;tally.reader[await verdict(sr)]++;
}
rmSync(base,{recursive:true,force:true});
console.log("FRESH sample",pick.length,JSON.stringify(tally));
process.exit(0);
