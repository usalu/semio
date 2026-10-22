import fs from "node:fs";
import {pathToFileURL} from "node:url";
const [,, listFile, outFile]=process.argv,paths=JSON.parse(fs.readFileSync(listFile!,"utf8")) as string[];
const now=()=>Bun.nanoseconds()/1e6,calls:{api:string;path:string;t:number;ms:number}[]=[];
(globalThis as any).__bfCalls=Object.assign(calls,{push:(c:any)=>calls[calls.length]=c});
Bun.plugin({name:"bf-iso",setup(b){b.onLoad({filter:/\.(ts|mjs)$/},a=>{
 const text=fs.readFileSync(a.path,"utf8");const shim=a.path.includes("/node_modules/")||a.path.includes("/boot-fast/tools/")?text:text.replace(/from\s*(["'])(node:)?fs\1/g,'from "/private/tmp/boot-fast/tools/fs-shim.ts"');
 return {contents:shim,loader:a.path.endsWith(".ts")?"ts":"js"};});}});
const rows:{path:string;ms:number;fs:number;fsMs:number;error?:string}[]=[],t0=now();
for(const p of paths){const c0=calls.length,s=now();let error:string|undefined;try{await import(pathToFileURL(p).href);}catch(e){error=String(e).slice(0,120);}const ms=now()-s,mine=calls.slice(c0);rows.push({path:p,ms,fs:mine.length,fsMs:mine.reduce((x,c)=>x+c.ms,0),error});}
fs.writeFileSync(outFile!,JSON.stringify({totalMs:now()-t0,rows,calls:calls.map(c=>({...c}))}));
