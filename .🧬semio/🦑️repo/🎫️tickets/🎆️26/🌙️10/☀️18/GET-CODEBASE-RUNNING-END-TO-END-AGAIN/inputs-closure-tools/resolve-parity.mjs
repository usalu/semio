// 🧭 Resolution parity: every literal workspace package export subpath resolved by require.resolve under the running host; run under bun and node and diff the JSON.
import {createRequire} from "node:module";import {readFileSync} from "node:fs";import {spawnSync} from "node:child_process";
const W="/Users/ueli/Documents/semio";
const files=spawnSync("git",["-c","core.quotepath=false","ls-files","-z"],{cwd:W,encoding:"utf8",maxBuffer:1<<28}).stdout.split("\0").filter(f=>f.endsWith("package.json")&&!f.startsWith(".🧬semio")&&!f.includes("node_modules"));
const specs=new Set();
for(const f of files){let d;try{d=JSON.parse(readFileSync(W+"/"+f,"utf8"));}catch{continue;}if(!d.name||!d.exports)continue;
 const keys=typeof d.exports==="string"||Object.keys(d.exports).every(k=>!k.startsWith("."))?["."]:Object.keys(d.exports);
 for(const k of keys)if(!k.includes("*"))specs.add(k==="."?d.name:d.name+k.slice(1));}
const r=createRequire(W+"/package.json"),out={};
for(const s of [...specs].sort()){try{out[s]=r.resolve(s).replace(W+"/","");}catch(e){out[s]="ERR:"+(e.code??"?");}}
console.log(JSON.stringify(out));
