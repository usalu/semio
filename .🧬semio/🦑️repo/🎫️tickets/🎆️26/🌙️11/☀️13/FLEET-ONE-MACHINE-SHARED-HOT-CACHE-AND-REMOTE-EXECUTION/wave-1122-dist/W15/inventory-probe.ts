/** 🔬️ W15: the file-inventory phase of the selected read (memo read, memo seed, memo check) on the real rows of the Ready workspace scope: an admission memo is packed from the coordinator's owner checkpoint packs of that scope exactly as the read packs it (`packAdmissionMemoV1`), written with `writeAdmissionMemoV1` to OUT, then read back, seeded and verified as `inventory()` does; one JSON line per step with rss, heap, external after a full collection and the peak. */
import {readFileSync,readdirSync,statSync,rmSync} from "node:fs";import {heapStats} from "bun:jsc";
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const rt=await import(L+"👁️observation/🔢️row-table/🟨️.mjs"),obs=await import(L+"👁️observation/🟨️.mjs"),kinds=(await import(L+"🧬️schema/📥️authority.json",{with:{type:"json"}})).default.definitions.ObservationKind.enum;
const memoM=await import(L+"🕸️graph/📥️load/🧠️admission-memo/🟦️.ts"),ws=await import(L+"🕸️graph/📁️workspace/🟦️.ts");
const D=root+"/.cache/b/g/m/inference/selections/2f245bafbb6a1b8305c18d9386d61ec1853ca12938201ee6677109a1a44acc64/oc/",MB=1048576,OUT=process.env.OUT!,scope="probe";
const mark=(label:string,extra:Record<string,unknown>={})=>{Bun.gc(true);const m=process.memoryUsage(),h=heapStats();console.log(JSON.stringify({label,rssMB:Math.round(m.rss/MB),peakMB:Math.round(process.resourceUsage().maxRSS/1024),heapMB:Math.round(m.heapUsed/MB),externalMB:Math.round(m.external/MB),strings:h.objectTypeCounts.string,objects:h.objectCount,...extra}));};
mark("baseline");
{
 const owners:{suffix:string;facts:unknown;rows:unknown[]}[]=[],union:unknown[]=[],seen=new Set();
 for(const name of readdirSync(D).filter(n=>n.endsWith(".pack")).sort()){const bytes=readFileSync(D+name),nl=bytes.indexOf(10),header=JSON.parse(bytes.subarray(0,nl).toString());let off=nl+1;for(const r of header.records){const rec=JSON.parse(bytes.subarray(off,off+r.bytes).toString());off+=r.bytes;const rows=obs.shareObservationRowsV1(rt.decodeAuthorityObservationsV1(rec.observations,kinds,()=>{}));for(const row of rows)if(!seen.has(row)){seen.add(row);union.push(row);}owners.push({suffix:rec.facts.project.name+"\0"+rec.key,facts:rec.facts,rows});}}
 const admission={owners:owners.map(o=>(o.facts as any).project.name).sort(),files:[...new Set(owners.flatMap(o=>(o.facts as any).files??[]))].sort(),targets:{},inputs:{},catalogProviders:[]};
 const packed=memoM.packAdmissionMemoV1({format:"probe",scope,admission:admission as never,observations:union,owners});
 await memoM.writeAdmissionMemoV1(OUT,packed,1<<30);
 mark("memo-written",{owners:owners.length,observations:union.length,fileMB:+(statSync(OUT).size/MB).toFixed(1)});
}
mark("writer-released");
const {readFile}=await import("node:fs/promises");const lent:any=JSON.parse(await readFile(OUT,"utf8"));
mark("memo-read",{observations:lent?.observations.length,owners:lent?.owners.length});
const seeded=memoM.ownersOfAdmissionMemoV1(lent!);const memo=new Map<string,unknown>();for(const o of seeded)memo.set(o.suffix,{facts:o.facts,rows:obs.shareObservationRowsV1([...o.rows])});
mark("memo-seeded",{entries:memo.size});
let verified=0;const t=performance.now();let stale=0;const holds=await ws.buildGraphSourceFactsHoldV1(lent!.observations,undefined,()=>{stale++;},(rows:number)=>{verified=rows;},false,true);
mark("memo-checked",{holds,verified,stale,seconds:Math.round((performance.now()-t)/1000)});
let s=0;for(const c of obs.observationCollectionsV1())s++;console.log(JSON.stringify(obs.observationCollectionsV1().filter((c:any)=>c.entries>0).map((c:any)=>[c.name,c.entries,Math.round((c.estimatedBytes??c.bytes??0)/1048576)])));
lent.observations=undefined;lent.owners=undefined;memo.clear();seeded.length=0;mark("memo-dropped");
rmSync(OUT,{force:true});
