/** 🔬️ W15: what one observation row costs as the read keeps it, on the real rows of the Ready workspace scope (coordinator owner checkpoint packs, read-only): the decoded rows alone, then the same rows shared through the process row pool (its identity keys, weak references and finalization registrations), counted after a full collection; and the row kinds with their share of the pool key characters. */
import {readFileSync,readdirSync} from "node:fs";import {heapStats} from "bun:jsc";
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const rt=await import(L+"👁️observation/🔢️row-table/🟨️.mjs"),kinds=(await import(L+"🧬️schema/📥️authority.json",{with:{type:"json"}})).default.definitions.ObservationKind.enum;
const D=root+"/.cache/b/g/m/inference/selections/2f245bafbb6a1b8305c18d9386d61ec1853ca12938201ee6677109a1a44acc64/oc/",MB=1048576;
const mark=(label:string,extra:Record<string,unknown>={})=>{Bun.gc(true);Bun.gc(true);const m=process.memoryUsage(),h=heapStats();const r={label,heapMB:+(m.heapUsed/MB).toFixed(1),externalMB:+(m.external/MB).toFixed(1),strings:h.objectTypeCounts.string,objects:h.objectCount,weakRefs:h.objectTypeCounts.WeakRef??0,...extra};console.log(JSON.stringify(r));return r;};
const base=mark("baseline");
const distinct=new Map<string,any>();
for(const name of readdirSync(D).filter(n=>n.endsWith(".pack")).sort()){const bytes=readFileSync(D+name),nl=bytes.indexOf(10),header=JSON.parse(bytes.subarray(0,nl).toString());let off=nl+1;for(const r of header.records){const rec=JSON.parse(bytes.subarray(off,off+r.bytes).toString());off+=r.bytes;for(const row of rt.decodeAuthorityObservationsV1(rec.observations,kinds,()=>{})){const k=JSON.stringify(row);if(!distinct.has(k))distinct.set(k,row);}}}
let rows:any[]=[...distinct.values()];distinct.clear();
const byKind:Record<string,number>={};for(const r of rows)byKind[r.kind]=(byKind[r.kind]??0)+1;
const a=mark("rows",{rows:rows.length,byKind});
const pool=rt.createRowPoolV1();pool.rows(rows);
const b=mark("rows+pool",{poolSize:pool.size()});
console.log(JSON.stringify({perRowBytes:Math.round(((a.heapMB+a.externalMB)-(base.heapMB+base.externalMB))*MB/rows.length),poolPerRowBytes:Math.round(((b.heapMB+b.externalMB)-(a.heapMB+a.externalMB))*MB/rows.length)}));
