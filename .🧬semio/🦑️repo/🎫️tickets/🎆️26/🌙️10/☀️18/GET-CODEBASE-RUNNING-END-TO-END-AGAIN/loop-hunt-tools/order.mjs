import fs from "node:fs";import {createHash} from "node:crypto";import {dirname} from "node:path";
const np="/Users/ueli/Documents/semio/.cache/b/g/m/inference/np/";
const since=Date.parse(process.argv[2]);const until=Date.parse(process.argv[3]??"2999-01-01");
const lines=fs.readFileSync("/private/tmp/loop-hunt/projects.txt","utf8").trim().split("\n");
const h=r=>createHash("sha256").update(r).digest("hex").slice(0,24);
const written=new Set();for(const f of fs.readdirSync(np))if(f.endsWith(".json")){const m=fs.statSync(np+f).mtimeMs;if(m>=since&&m<until)written.add(f.slice(0,-5));}
let idx=[];lines.forEach((p,i)=>{const root=dirname(p);if(written.has(h(root)))idx.push(i);});
console.log("written total",written.size,"matched project.json roots",idx.length);
console.log("positions",idx.slice(0,40).join(","),"...",idx.slice(-10).join(","));
console.log("max pos",idx.at(-1),lines[idx.at(-1)]);
