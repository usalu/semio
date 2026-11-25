/** 🔬️ W15: real bytes of the owner admission memo of the Ready workspace scope, rebuilt from the coordinator's owner checkpoint packs (read-only): entries as the read keeps them (facts parsed per entry, rows shared through the row pool), the strings in the facts (held vs distinct), and the memory after a full collection with the facts as parsed, with their strings interned, and dropped. */
import {readFileSync,readdirSync} from "node:fs";import {heapStats} from "bun:jsc";
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const rt=await import(L+"👁️observation/🔢️row-table/🟨️.mjs"),obs=await import(L+"👁️observation/🟨️.mjs"),kinds=(await import(L+"🧬️schema/📥️authority.json",{with:{type:"json"}})).default.definitions.ObservationKind.enum;
const D=root+"/.cache/b/g/m/inference/selections/2f245bafbb6a1b8305c18d9386d61ec1853ca12938201ee6677109a1a44acc64/oc/",MB=1048576;
const mark=(label:string,extra:Record<string,unknown>={})=>{Bun.gc(true);Bun.gc(true);const m=process.memoryUsage(),h=heapStats();console.log(JSON.stringify({label,rssMB:Math.round(m.rss/MB),heapMB:Math.round(m.heapUsed/MB),externalMB:Math.round(m.external/MB),objects:h.objectCount,strings:h.objectTypeCounts.string,...extra}));};
mark("baseline");
const memo=new Map<string,{facts:any;rows:unknown[]}>();let packBytes=0,recordBytes=0;
for(const name of readdirSync(D).filter(n=>n.endsWith(".pack")).sort()){const bytes=readFileSync(D+name);packBytes+=bytes.length;const nl=bytes.indexOf(10),header=JSON.parse(bytes.subarray(0,nl).toString());let off=nl+1;for(const r of header.records){const rec=JSON.parse(bytes.subarray(off,off+r.bytes).toString());recordBytes+=r.bytes;off+=r.bytes;const rows=rt.decodeAuthorityObservationsV1(rec.observations,kinds,()=>{});memo.set(rec.key,{facts:rec.facts,rows:obs.shareObservationRowsV1(rows)});}}
let held=0;const distinctRows=new Set();for(const e of memo.values()){held+=e.rows.length;for(const r of e.rows)distinctRows.add(r);}
let strings=0,chars=0;const distinct=new Set<string>();const walk=(v:any)=>{if(typeof v==="string"){strings++;chars+=v.length;distinct.add(v);}else if(Array.isArray(v))v.forEach(walk);else if(v&&typeof v==="object")for(const k in v)walk(v[k]);};
for(const e of memo.values())walk(e.facts);
let dchars=0;for(const s of distinct)dchars+=s.length;
const sample=[...memo.values()][0]!.facts;
mark("memo-parsed",{entries:memo.size,packMB:+(packBytes/MB).toFixed(1),recordMB:+(recordBytes/MB).toFixed(1),rowRefs:held,distinctRows:distinctRows.size,factStrings:strings,factCharsM:+(chars/1e6).toFixed(1),distinctFactStrings:distinct.size,distinctCharsM:+(dchars/1e6).toFixed(2),factKeys:Object.keys(sample),factFieldSizes:Object.fromEntries(Object.entries(sample).map(([k,v])=>[k,JSON.stringify(v).length]))});
distinct.clear();distinctRows.clear();
const pool=new Map<string,string>(),intern=(v:any):any=>{if(typeof v==="string"){const h=pool.get(v);if(h!==undefined)return h;pool.set(v,v);return v;}if(Array.isArray(v)){for(let i=0;i<v.length;i++)v[i]=intern(v[i]);return v;}if(v&&typeof v==="object"){for(const k in v)v[k]=intern(v[k]);return v;}return v;};
for(const e of memo.values())intern(e.facts);
mark("facts-interned",{pool:pool.size});
pool.clear();mark("pool-dropped");
for(const e of memo.values())e.facts=undefined;mark("facts-dropped");
memo.clear();mark("memo-dropped");
