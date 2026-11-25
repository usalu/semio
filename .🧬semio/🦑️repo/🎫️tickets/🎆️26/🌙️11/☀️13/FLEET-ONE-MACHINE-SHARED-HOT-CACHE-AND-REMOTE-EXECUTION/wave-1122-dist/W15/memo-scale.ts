/** 🔬️ W15: the admission memo of the Ready workspace scope at worker size, and what reading it costs. MODE=write builds a memo from the coordinator's real owner checkpoint rows copied COPIES times (every path of a copy moved under its own directory, nested analysis rows kept shared per copy), so the observation and owner counts reach the workers' (283k distinct rows, 1.12M owner row references at COPIES=5), and writes it with `writeAdmissionMemoV1` to OUT. MODE=read reads OUT in a fresh process as `inventory()` does (`readAdmissionMemoV1` text + parse, then `ownersOfAdmissionMemoV1` and the row pool) and prints the peak resident size. */
import {readFileSync,readdirSync,statSync} from "node:fs";
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const rt=await import(L+"👁️observation/🔢️row-table/🟨️.mjs"),obs=await import(L+"👁️observation/🟨️.mjs"),kinds=(await import(L+"🧬️schema/📥️authority.json",{with:{type:"json"}})).default.definitions.ObservationKind.enum,memoM=await import(L+"🕸️graph/📥️load/🧠️admission-memo/🟦️.ts");
const D=root+"/.cache/b/g/m/inference/selections/2f245bafbb6a1b8305c18d9386d61ec1853ca12938201ee6677109a1a44acc64/oc/",MB=1048576,OUT=process.env.OUT!,COPIES=Number(process.env.COPIES??5);
const peak=()=>Math.round(process.resourceUsage().maxRSS/1024);
if(process.env.MODE==="write"){
 const entries:{suffix:string;facts:any;rows:any[]}[]=[];
 for(const name of readdirSync(D).filter(n=>n.endsWith(".pack")).sort()){const bytes=readFileSync(D+name),nl=bytes.indexOf(10),header=JSON.parse(bytes.subarray(0,nl).toString());let off=nl+1;for(const r of header.records){const rec=JSON.parse(bytes.subarray(off,off+r.bytes).toString());off+=r.bytes;entries.push({suffix:rec.facts.project.name+"\0"+rec.key,facts:rec.facts,rows:obs.shareObservationRowsV1(rt.decodeAuthorityObservationsV1(rec.observations,kinds,()=>{}))});}}
 const owners:{suffix:string;facts:unknown;rows:unknown[]}[]=[],union:unknown[]=[],seen=new Set();
 for(let copy=0;copy<COPIES;copy++){
  const moved=new Map<object,any>(),move=(row:any):any=>{let known=moved.get(row);if(known)return known;const fact=row.kind==="analysis"&&Array.isArray(row.fact?.nestedObservations)?{...row.fact,nestedObservations:row.fact.nestedObservations.map(move)}:row.fact;known={...row,path:typeof row.path==="string"?row.path.replace(/^(\{workspaceRoot\}\/|\/Users\/ueli\/Documents\/semio\/)?/u,"$1🗂️copy-"+copy+"/"):row.path,fact};moved.set(row,known);return known;};
  for(const entry of entries){const rows=obs.shareObservationRowsV1(entry.rows.map(move));for(const row of rows)if(!seen.has(row)){seen.add(row);union.push(row);}owners.push({suffix:entry.suffix+"\0"+copy,facts:entry.facts,rows});}
 }
 const admission={owners:[],files:[],targets:{},inputs:{},catalogProviders:[]};
 await memoM.writeAdmissionMemoV1(OUT,memoM.packAdmissionMemoV1({format:"probe",scope:"probe",admission:admission as never,observations:union,owners}),2**31);
 let refs=0;for(const o of owners)refs+=o.rows.length;
 console.log(JSON.stringify({mode:"write",owners:owners.length,observations:union.length,rowRefs:refs,fileMB:+(statSync(OUT).size/MB).toFixed(1),peakMB:peak()}));
}else{
 const {readFile}=await import("node:fs/promises");const before=peak();
 const text=await readFile(OUT,"utf8");const textPeak=peak();
 const memo=JSON.parse(text);const parsePeak=peak();
 const owners=memoM.ownersOfAdmissionMemoV1(memo);const held=owners.map(o=>({facts:o.facts,rows:obs.shareObservationRowsV1([...o.rows])}));
 Bun.gc(true);
 console.log(JSON.stringify({mode:"read",observations:memo.observations.length,owners:held.length,textChars:text.length,startMB:before,afterTextMB:textPeak,afterParseMB:parsePeak,peakMB:peak(),liveMB:Math.round((process.memoryUsage().heapUsed+process.memoryUsage().external)/MB)}));
}
