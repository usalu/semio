// [scratch drill] Real cargo on a scratch workspace: baseline local build against the same build with every miss offloaded to worker processes behind a scheduler.
// Usage (from any directory): bun live-drill.ts [crates=6] [functions=1500]; needs the pinned toolchain; touches only a temporary directory.
import {spawn,spawnSync,type ChildProcess} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startClusterV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🧰️cluster/🟦️.ts";
import {ensureUnitCacheShimV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";

const LIB="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",crates=Number(process.argv[2]??6),functions=Number(process.argv[3]??1500);
const entry=`${LIB}/🏗️build/🌐️remote-execution/🏃️entry/🟦️.ts`;
const scratch=realpathSync(mkdtempSync(join(tmpdir(),"offload-drill-"))),children:ChildProcess[]=[];
const toolchain=process.env.RUSTUP_TOOLCHAIN??"nightly-2026-07-20";
const baseEnvironment={...process.env,RUSTUP_TOOLCHAIN:toolchain} as Record<string,string>;
for(const name of Object.keys(baseEnvironment))if(name.startsWith("SEMIO_")||name==="RUSTC_WRAPPER")delete baseEnvironment[name];

function workspace(directory:string):string{
 mkdirSync(directory,{recursive:true});
 const members:string[]=[],write=(name:string,source:string,dependencies:string[])=>{
  mkdirSync(join(directory,name,"src"),{recursive:true});
  writeFileSync(join(directory,name,"src","lib.rs"),source);
  writeFileSync(join(directory,name,"Cargo.toml"),`[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\n${dependencies.map(dependency=>`${dependency} = { path = "../${dependency}" }\n`).join("")}`);
  members.push(`"${name}"`);
 };
 const leaves=Array.from({length:crates},(_,index)=>`unit${index}`);
 leaves.forEach((name,index)=>write(name,Array.from({length:functions},(_,i)=>`pub fn f${i}(x:u64)->u64{let mut s=x;for k in 0..${i%7+2}{s=s.wrapping_mul(31).wrapping_add(k as u64+${i+index});}s}`).join("\n")+"\npub fn root(x:u64)->u64{f0(x)}\n",[]));
 write("top",`pub fn all(x:u64)->u64{${leaves.map(name=>`${name}::root(x)`).join("+")}}\n`,leaves);
 writeFileSync(join(directory,"Cargo.toml"),`[workspace]\nmembers = [${members.join(",")}]\nresolver = "2"\n[profile.dev]\nincremental = false\n`);
 return directory;
}

function cargo(label:string,directory:string,environment:Record<string,string>):Promise<{seconds:number;status:number|null}>{
 const started=Date.now();
 return new Promise(done=>{
  const child=spawn("cargo",["build","-j","3","--offline"],{cwd:directory,env:environment,stdio:["ignore","ignore","pipe"]}),errors:Buffer[]=[];
  child.stderr.on("data",chunk=>errors.push(chunk));
  child.on("close",status=>{console.log(`${label}: ${((Date.now()-started)/1000).toFixed(1)} s, exit ${status}${status===0?"":"\n"+Buffer.concat(errors).toString().slice(-800)}`);done({seconds:(Date.now()-started)/1000,status});});
 });
}

async function main(){
 const cluster=await startClusterV1({workers:0,root:scratch,scheduler:{leaseSeconds:5,lingerSeconds:1}});
 const idle=join(scratch,"idle.json");
 writeFileSync(idle,JSON.stringify({host:{version:1,atMs:Date.now()+86400000,platform:"darwin",cores:16,totalMegabytes:65536,availableMegabytes:48000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:null,openFilesMaximum:null,attachedImages:null},grants:[],waiters:[]}));
 for(const name of ["drill-a","drill-b"]){
  const directory=join(scratch,name);mkdirSync(directory,{recursive:true});
  children.push(spawn(process.execPath,[entry,"--join",cluster.server.workerUrl,"--name",name,"--origin",name,"--work-directory",join(directory,"w"),"--concurrency","4"],{cwd:directory,env:{...baseEnvironment,SEMIO_REMOTE_EXECUTION_HOST_VIEW:idle},stdio:"ignore"}));
 }
 const deadline=Date.now()+60000;
 while(Date.now()<deadline){try{const table=await (await fetch(cluster.server.workerUrl+"/v1/fleet/load")).json() as {machines?:{origin:string;headroom?:boolean}[]};if(["drill-a","drill-b"].every(name=>table.machines?.some(row=>row.origin===name&&row.headroom)))break;}catch{}await Bun.sleep(300);}
 const rustc=spawnSync("rustc",["-vV"],{env:baseEnvironment,encoding:"utf8"}).stdout,counting=(root:string)=>{const path=join(root,"count.sh"),log=join(root,"calls.log");writeFileSync(path,`#!/bin/sh\ncase " $* " in *" -vV "*) ;; *) echo call >> '${log}';; esac\nexec rustc "$@"\n`);chmodSync(path,0o755);return{path,log};};
 const environmentFor=(root:string,mode:string,jobserver:string)=>{
  mkdirSync(root,{recursive:true});const counter=counting(root),shim=ensureUnitCacheShimV1(root);
  return{...baseEnvironment,RUSTC_WRAPPER:shim,SEMIO_UNIT_CACHE_ROOT:root,SEMIO_UNIT_CACHE_LANE:"drill",SEMIO_UNIT_CACHE_INNER:JSON.stringify([counter.path]),SEMIO_UNIT_CACHE_SLOTS:"0",SEMIO_OFFLOAD:mode,SEMIO_CAS_ROOT:join(root,"cas"),
   SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_CACHE_ENDPOINT:cluster.cache.url.replace("//0.0.0.0","//127.0.0.1"),SEMIO_REMOTE_CACHE_INSTANCE:"main",MAKEFLAGS:` -j4 --jobserver-auth=fifo:${jobserver}`,CARGO_TARGET_DIR:join(root,"target")};
 };
 const fifo=(path:string,tokens:number)=>{spawnSync("mkfifo",[path]);const fd=require("node:fs").openSync(path,"r+");require("node:fs").writeSync(fd,"+".repeat(tokens));return fd;};
 const localRoot=join(scratch,"local"),remoteRoot=join(scratch,"remote");
 const localWorkspace=workspace(join(scratch,"ws-local")),remoteWorkspace=workspace(join(scratch,"ws-remote"));
 fifo(join(scratch,"pool-local"),2);fifo(join(scratch,"pool-remote"),2);
 console.log(`drill: ${crates} independent crates and one top crate of ${functions} functions, two worker processes, scheduler and cache on this machine (${scratch})`);
 const local=await cargo("local build (SEMIO_OFFLOAD=never)",localWorkspace,environmentFor(localRoot,"never",join(scratch,"pool-local")));
 const remote=await cargo("offloaded build (SEMIO_OFFLOAD=always)",remoteWorkspace,environmentFor(remoteRoot,"always",join(scratch,"pool-remote")));
 const calls=(root:string)=>existsSync(join(root,"calls.log"))?readFileSync(join(root,"calls.log"),"utf8").split("\n").filter(Boolean).length:0;
 const events=existsSync(join(remoteRoot,"x","offload.jsonl"))?readFileSync(join(remoteRoot,"x","offload.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line)):[];
 console.log(`local rustc calls: baseline ${calls(localRoot)}, offloaded build ${calls(remoteRoot)}`);
 const byOrigin:Record<string,string[]>={};for(const row of events)(byOrigin[row.origin??`${row.reason}: ${row.detail}`]??=[]).push(row.crate);
 console.log("units by origin:",JSON.stringify(byOrigin));
 console.log("progress lines:\n"+(existsSync(join(remoteRoot,"x","offload.progress"))?readFileSync(join(remoteRoot,"x","offload.progress"),"utf8").split("\n").filter((_,i)=>i%2===0).join("\n"):""));
 console.log(`scheduler events: ${cluster.events.filter(e=>e.startsWith("assigned")).length} assignments`);
 console.log(JSON.stringify({baselineSeconds:local.seconds,offloadedSeconds:remote.seconds}));
 await cluster.stop();
}
try{await main();}finally{for(const child of children)child.kill("SIGKILL");rmSync(scratch,{recursive:true,force:true});}
