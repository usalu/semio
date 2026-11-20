import {readdirSync,readFileSync} from "node:fs";
import {lstat,open} from "node:fs/promises";
import {constants} from "node:fs";
import {join,parse,relative,sep,dirname} from "node:path";
import {phaseCompilerAdmissionControlV1} from "C:/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🎫️lease/🟦️.ts";
import {readPhysicalCargoSourceV1,createPhysicalCargoSourceReaderV1} from "C:/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
import {readPhysicalCargoSourceV1 as readOld} from "./source-admission-baseline-read.ts";
import {recallAdmissionV1} from "C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/🟦️.ts";
/** 📈️ Real-file benchmark: replays the probed source paths of every remembered unit admission through the real physical reader. */
const store="C:/semio/.cache/b/r/m/u/a",limit=Number(process.argv[2]??"60"),units:{key:string;paths:string[]}[]=[];
for(const d of readdirSync(store))for(const f of readdirSync(join(store,d)))if(f.endsWith(".json"))try{const m=JSON.parse(readFileSync(join(store,d,f),"utf8"));units.push({key:m.key,paths:m.stamps.filter((s:any)=>s.present).map((s:any)=>s.path)});}catch{}
const sample=units.slice(0,limit),reads=sample.flatMap(u=>u.paths),distinct=new Set(reads);
const control=phaseCompilerAdmissionControlV1(new AbortController().signal,0,()=>undefined,()=>new Promise(done=>setImmediate(done)));
let syscalls=0;const wall=async<T>(f:()=>Promise<T>)=>{const t=performance.now();await f();return performance.now()-t;};
const phases:Record<string,number>={};
phases.ancestryLstats=await wall(async()=>{for(const p of reads){const parts=relative(parse(p).root,p).split(sep).filter(Boolean);let c=parse(p).root;for(const part of parts.slice(0,-1)){c=join(c,part);await lstat(c,{bigint:true});syscalls++;}}});
phases.openStatClose=await wall(async()=>{for(const p of reads){await lstat(p,{bigint:true});const h=await open(p,constants.O_RDONLY);await h.stat({bigint:true});await h.close();}});
phases.readOnly=await wall(async()=>{for(const p of reads){const h=await open(p,constants.O_RDONLY),s=await h.stat();const b=Buffer.alloc(Number(s.size));await h.read(b,0,b.length,0);await h.close();}});
phases.yield=await wall(async()=>{for(let i=0;i<reads.length*10;i++)await control.yieldContinuation();});
const mode=process.argv[3]??"new",root="C:/semio".replaceAll("/",sep),reader=createPhysicalCargoSourceReaderV1();let bytes=0;const t0=performance.now();
for(const u of sample){for(const p of u.paths){const request={version:1 as const,root,expectedBytes:null,maximumBytes:1<<28,remainingFiles:1<<20};const out=mode==="old"?await readOld(p,request,control):await reader.read(p,request,control);bytes+=out?.byteLength??0;}if(mode!=="old")await reader.sweep(control);}
const total=performance.now()-t0;
let memoHit=0;const tm=performance.now();for(const u of sample)if(recallAdmissionV1("C:/semio/.cache/b/r/m/u",u.key))memoHit++;const memoMs=performance.now()-tm;
console.log(JSON.stringify({mode,units:sample.length,reads:reads.length,distinctFiles:distinct.size,bytes,totalReadMs:Math.round(total),filesPerSecond:+(reads.length/(total/1000)).toFixed(1),KBps:Math.round(bytes/1024/(total/1000)),phasesMs:Object.fromEntries(Object.entries(phases).map(([k,v])=>[k,Math.round(v)])),memoRecallMs:Math.round(memoMs),memoHit},null,1));
