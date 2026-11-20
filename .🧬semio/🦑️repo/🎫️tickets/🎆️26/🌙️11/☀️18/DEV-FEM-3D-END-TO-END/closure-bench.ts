import {readdirSync,readFileSync} from "node:fs";
import {join,dirname,basename} from "node:path";
import {spawnSync} from "node:child_process";
import {readRustSourceClosureV1,createRustSourceAnalysisV1} from "C:/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
/** 🧮️ CPU-only closure analysis over real remembered closures, with files preloaded so no I/O is measured. */
const store="C:/semio/.cache/b/r/m/u/a",limit=Number(process.argv[2]??"150"),units:{entry:string;inputs:string[]}[]=[];
for(const d of readdirSync(store))for(const f of readdirSync(join(store,d)))if(f.endsWith(".json")&&units.length<limit){const m=JSON.parse(readFileSync(join(store,d,f),"utf8")),inputs=m.inputs.map((i:any)=>i.path);const entry=inputs.find((p:string)=>/[\/]src[\/](lib|main)\.rs$/.test(p))??inputs.find((p:string)=>/build\.rs$/.test(p))??inputs[0];if(entry)units.push({entry,inputs});}
const cfgOf=(t:string[])=>spawnSync("rustc",["--print","cfg",...t],{encoding:"utf8"}).stdout.trim().split("\n");
const cfgs=[cfgOf([]),cfgOf(["--target","wasm32-unknown-unknown"])];
const bytes=new Map<string,Uint8Array>();for(const u of units)for(const p of u.inputs)if(!bytes.has(p))try{bytes.set(p,readFileSync(p));}catch{}
(globalThis as any).shown=0;const root="C:/semio".replaceAll("/","\\");
const run=async(shared:boolean,alternate:boolean)=>{const analysis=shared?createRustSourceAnalysisV1():undefined;let ok=0,files=0;const t=performance.now();for(const [i,u] of units.entries()){try{const r=await readRustSourceClosureV1([u.entry],root,cfgs[alternate?i%2:0]!,async p=>bytes.get(p.replaceAll("\\","/"))??bytes.get(p)??null,{manifestDirectory:dirname(dirname(u.entry)),generated:()=>undefined},analysis);ok++;files+=r.length;}catch(e){if(!(globalThis as any).shown++)console.log(String(e).slice(0,300),u.entry);}}return{shared,alternate,units:units.length,ok,files,ms:Math.round(performance.now()-t)};};
console.log(await run(false,false),await run(true,false),await run(false,true),await run(true,true));
