import fs from "node:fs";import {createHash} from "node:crypto";import {dirname} from "node:path";
const np="/Users/ueli/Documents/semio/.cache/b/g/m/inference/np/";
const lines=fs.readFileSync("/private/tmp/loop-hunt/projects.txt","utf8").trim().split("\n");
const h=r=>createHash("sha256").update(r).digest("hex").slice(0,24);
const lo=Number(process.argv[2]),hi=Number(process.argv[3]);
for(let i=lo;i<=hi;i++){const f=np+h(dirname(lines[i]))+".json";let m="none";try{m=new Date(fs.statSync(f).mtimeMs).toLocaleTimeString("de-DE");}catch{}
 if(process.argv[4]==="all"||m==="none"||process.argv[4]==="t")console.log(i,m,lines[i].slice(0,150));}
