import * as fsp from "node:fs/promises";
import {spyOn} from "bun:test";
import {readdirSync,readFileSync} from "node:fs";
import {join,sep} from "node:path";
import {phaseCompilerAdmissionControlV1} from "C:/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🎫️lease/🟦️.ts";
import {createPhysicalCargoSourceReaderV1} from "C:/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
import {readPhysicalCargoSourceV1 as readOld} from "./source-admission-baseline-read.ts";
const store="C:/semio/.cache/b/r/m/u/a",units:string[][]=[];
for(const d of readdirSync(store))for(const f of readdirSync(join(store,d)))if(f.endsWith(".json")&&units.length<40)units.push(JSON.parse(readFileSync(join(store,d,f),"utf8")).stamps.filter((s:any)=>s.present).map((s:any)=>s.path));
const control=phaseCompilerAdmissionControlV1(new AbortController().signal,0,()=>undefined,()=>new Promise(d=>setImmediate(d))),root="C:/semio".replaceAll("/",sep),request={version:1 as const,root,expectedBytes:null,maximumBytes:1<<28,remainingFiles:1<<20};
for(const mode of ["old","new"]){const l=spyOn(fsp,"lstat"),o=spyOn(fsp,"open"),reader=createPhysicalCargoSourceReaderV1();let reads=0;for(const u of units){for(const p of u){await (mode==="old"?readOld(p,request,control):reader.read(p,request,control));reads++;}if(mode==="new")await reader.sweep(control);}console.log(mode,{reads,lstats:l.mock.calls.length,opens:o.mock.calls.length,lstatPerRead:+(l.mock.calls.length/reads).toFixed(1)});l.mockRestore();o.mockRestore();}
