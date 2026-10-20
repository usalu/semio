#!/usr/bin/env bun
/** 🐢 Stateless in-process path of the same decision: membership proof by stat, closure digests from scratch, memo compare. Phases are what a command without a resident service must redo every time. */
import {createHash} from "node:crypto";
import {lstat,readFile} from "node:fs/promises";
import {readFileSync,existsSync,writeFileSync} from "node:fs";
import {closureOf} from "./lib.mjs";

const[,, root,target="p1499:build",mode="stat"]=process.argv;
const t0=performance.now(),phases={};let mark=t0;
const lap=name=>{const at=performance.now();phases[name]=Math.round((at-mark)*10)/10;mark=at;};
const graph=JSON.parse(readFileSync(root+"/graph.json","utf8"));lap("graph");
const frontierPath=root+"/.frontier.json";
if(!existsSync(frontierPath)){
 const{spawnSync}=await import("node:child_process");
 const out=spawnSync("/private/tmp/hot-cache/bin/semio-repo-watch",["scan","--root",root,"--exclude",".git","--exclude",".cache","--exclude-name","node_modules","--exclude-name","target"],{maxBuffer:1<<28});
 writeFileSync(frontierPath,out.stdout);
}
const frontier=JSON.parse(readFileSync(frontierPath,"utf8"));lap("load-frontier");
let ok=true,next=0;
const rows=[...frontier.directories.map(r=>[r[0],true,r[1],r[2]]),...frontier.members.map(r=>[r[0],false,r[1]])];
await Promise.all(Array.from({length:64},async()=>{while(next<rows.length){const row=rows[next++];try{const s=await lstat(row[0]===""?root:root+"/"+row[0]);if(row[1]?false:s.mode!==row[2])ok=false;}catch{ok=false;}}}));
lap("membership-stat-pass("+rows.length+")");
const closure=closureOf(graph,target.split(":")[0]);
const hash=createHash("sha256");let files=0;
for(const owner of closure){
 const names=Array.from({length:20},(_,f)=>root+"/pkgs/"+owner+"/src/f"+f+".ts");
 const bufs=await Promise.all(names.map(n=>readFile(n)));files+=bufs.length;
 for(const b of bufs)hash.update(b);
}
const digest=hash.digest("hex");lap("closure-digest("+files+" files)");
console.log(JSON.stringify({what:"stateless decision, no module load, no custody",ok,totalMs:Math.round((performance.now()-t0)*10)/10,phases}));
