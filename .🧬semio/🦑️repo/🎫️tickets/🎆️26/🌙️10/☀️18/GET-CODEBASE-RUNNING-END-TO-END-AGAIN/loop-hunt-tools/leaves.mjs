import fs from "node:fs";import {brotliDecompressSync} from "node:zlib";import {createHash} from "node:crypto";import {dirname} from "node:path";
const M="/Users/ueli/Documents/semio/.cache/b/g/m/",np=M+"inference/np/";
const lines=fs.readFileSync("/private/tmp/loop-hunt/projects.txt","utf8").trim().split("\n");
const h=r=>createHash("sha256").update(r).digest("hex").slice(0,24);
const dirs={SOURCE_CLOSURE:M+"build-input-closures/",CARGO_LOCK:M+"build-lock-closures/"};
let tot=0,missing=0;const rows=[];
for(const pos of process.argv.slice(2).map(Number)){
 const f=np+h(dirname(lines[pos]))+".json";let text;try{text=brotliDecompressSync(fs.readFileSync(f)).toString("utf8");}catch(e){rows.push([pos,"noRecord"]);continue;}
 const nl=text.indexOf("\n"),body=text.slice(nl+1),names=new Set(body.match(/SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_[a-f0-9]{64}/g)??[]);
 let miss=[];for(const n of names){const k=n.startsWith("SEMIO_SOURCE")?"SOURCE_CLOSURE":"CARGO_LOCK";const key=n.slice(("SEMIO_"+k+"_").length);if(!fs.existsSync(dirs[k]+key+".json"))miss.push(n.slice(-12));}
 rows.push([pos,new Date(fs.statSync(f).mtimeMs).toLocaleTimeString("de-DE"),"refs",names.size,"missing",miss.length,miss.slice(0,2).join(",")]);
}
for(const r of rows)console.log(r.join(" "));
