/** 🔬️ W15: how much resident memory a phase boundary gives back after a read-like phase (real observation rows decoded, shared, serialised and parsed again as an admission memo text would be, 5 rounds), with a plain collection vs `settleGraphMemoryV1` (collection + `Bun.shrink` + collection). */
import {readFileSync,readdirSync} from "node:fs";
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const rt=await import(L+"👁️observation/🔢️row-table/🟨️.mjs"),kinds=(await import(L+"🧬️schema/📥️authority.json",{with:{type:"json"}})).default.definitions.ObservationKind.enum,settle=await import(L+"🧠️memory/🧽️settle/🟨️.mjs");
const D=root+"/.cache/b/g/m/inference/selections/2f245bafbb6a1b8305c18d9386d61ec1853ca12938201ee6677109a1a44acc64/oc/",MB=1048576,rss=()=>Math.round(process.memoryUsage().rss/MB);
const kept:unknown[]=[];
for(let round=0;round<5;round++)for(const name of readdirSync(D).filter(n=>n.endsWith(".pack"))){const bytes=readFileSync(D+name),nl=bytes.indexOf(10),header=JSON.parse(bytes.subarray(0,nl).toString());let off=nl+1;for(const r of header.records){const rec=JSON.parse(bytes.subarray(off,off+r.bytes).toString());off+=r.bytes;const rows=rt.decodeAuthorityObservationsV1(rec.observations,kinds,()=>{});const text=JSON.stringify(rows.map((row:any)=>({...row,path:"🧰️/"+round+row.path})));const again=JSON.parse(text);if(round===4)kept.push(again);}}
console.log(JSON.stringify({phase:"after",rssMB:rss(),heapMB:Math.round(process.memoryUsage().heapUsed/MB),peakMB:Math.round(process.resourceUsage().maxRSS/1024)}));
Bun.gc(true);console.log(JSON.stringify({phase:"gc",rssMB:rss(),heapMB:Math.round(process.memoryUsage().heapUsed/MB)}));
const r=await settle.settleGraphMemoryV1({force:true,report:()=>{}});console.log(JSON.stringify({phase:"settle",rssMB:rss(),...r,kept:kept.length}));
