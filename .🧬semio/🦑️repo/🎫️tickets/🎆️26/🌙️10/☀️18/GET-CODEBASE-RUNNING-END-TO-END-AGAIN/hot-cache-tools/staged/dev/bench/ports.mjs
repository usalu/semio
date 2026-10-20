/** 🔌 Prototype ports: graph from graph.json, listing via the native scan or a JS walk, execution by spawning a fake tool, and mock custody. */
import {spawn,spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {lstat,readdir} from "node:fs/promises";
import {environmentDigest} from "./workspace.mjs";

/** 📋 Graph port over graph.json. */
export function createGraphPort({root,scanBinary,threads=8}){
 return{
  async load(){return JSON.parse(readFileSync(root+"/graph.json","utf8"));},
  async environmentDigest(){return environmentDigest(root,readFileSync);},
  async listing({exclude,filter}){
   if(scanBinary){
    const args=["scan","--root",root,"--threads",String(threads)];for(const item of exclude.rooted)args.push("--exclude",item);for(const item of exclude.names)args.push("--exclude-name",item);
    const out=await new Promise((resolve,reject)=>{const child=spawn(scanBinary,args,{stdio:["ignore","pipe","inherit"]});const chunks=[];child.stdout.on("data",c=>chunks.push(c));child.on("exit",code=>code===0?resolve(Buffer.concat(chunks)):reject(new Error("scan exit "+code)));});
    const frame=JSON.parse(out.toString("utf8"));
    return{directories:frame.directories,members:frame.members};
   }
   const directories=[],members=[];
   const walk=async rel=>{
    const abs=rel===""?root:root+"/"+rel;
    const status=await lstat(abs);directories.push([rel,status.mtimeMs,status.ino]);
    const entries=await readdir(abs,{withFileTypes:true}),subs=[];
    await Promise.all(entries.map(async entry=>{const child=rel===""?entry.name:rel+"/"+entry.name;if(filter.excluded(child))return;if(entry.isDirectory())subs.push(child);else if(entry.isFile()){const s=await lstat(root+"/"+child);members.push([child,s.mode]);}}));
    await Promise.all(subs.map(walk));
   };
   await walk("");
   return{directories,members};
  },
 };
}

/** ▶ Execution port that spawns the fake tool and reports the spawn instant. */
export function createExecutionPort({tool,stampDirectory,workMs=30}){
 return{
  run({id,command,owner,cwd,env,signal,onSpawn,onOutput}){
   return new Promise((resolve,reject)=>{
    if(signal?.aborted){reject(signal.reason);return;}
    const out=id.endsWith(":test")?".out/"+owner+"-test":".out/"+owner;
    const child=spawn(tool,[id,cwd+"/"+out],{cwd,env:{...process.env,FAKE_STAMP_DIR:stampDirectory,FAKE_WORK_MS:String(workMs)},stdio:["ignore","pipe","pipe"]});
    onSpawn?.();
    const kill=()=>child.kill("SIGTERM");
    signal?.addEventListener("abort",kill,{once:true});
    child.stdout.on("data",chunk=>{void onOutput("stdout",chunk.toString("utf8"));});
    child.stderr.on("data",chunk=>{void onOutput("stderr",chunk.toString("utf8"));});
    child.on("error",reject);
    child.on("exit",(code,sig)=>{signal?.removeEventListener("abort",kill);resolve({exit:code??1,signal:sig});});
   });
  },
 };
}

/** 📌 Custody mock: acquire costs `acquireMs` (the cited image/lease open), release is free. */
export function createCustodyPort({acquireMs=0}={}){
 let held=false;const stats={acquired:0,released:0};
 return{async acquire(){if(held)return;await new Promise(resolve=>setTimeout(resolve,acquireMs));held=true;stats.acquired++;},async release(){held=false;stats.released++;},held:()=>held,stats};
}
