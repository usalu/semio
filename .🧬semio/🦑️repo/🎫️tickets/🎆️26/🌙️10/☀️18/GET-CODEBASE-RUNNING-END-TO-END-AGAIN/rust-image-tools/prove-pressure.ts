import {spawnSync} from "node:child_process";
import {mkdirSync,statSync,utimesSync,writeFileSync,existsSync,readFileSync,rmSync} from "node:fs";
import {join} from "node:path";
import {pathToFileURL} from "node:url";

/** 🧪️ Live proof on a real small APFS image (not the shared one): the high-water mark launches the real detached collector, and a command that really hits ENOSPC is repeated once after a real collection. Usage: bun prove-pressure.ts <root> <mount> */
const [root,mount]=process.argv.slice(2),library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache");
const launch=await import(pathToFileURL(join(library,"🚪️launch/🟦️.ts")).href),gc=await import(pathToFileURL(join(library,"🧹️gc/🟦️.ts")).href);
const cache=join(mount,"u"),MIB=1024*1024,capacity=1024*MIB,say=(...parts:unknown[])=>console.log(new Date().toISOString().slice(11,19),...parts);
const hex=(index:number)=>index.toString(16).padStart(64,"0");
const fill=(count:number,size:number,start:number)=>{for(let index=0;index<count;index++){const shard=join(cache,"o",hex(start+index).slice(0,2));mkdirSync(shard,{recursive:true});const path=join(shard,hex(start+index));writeFileSync(path,Buffer.alloc(size,1));const old=new Date(Date.now()-(count-index)*60000);utimesSync(path,old,old);}};
const free=()=>{const m=gc.measureVolumeV1(cache,capacity);return m;};
const show=(label:string)=>{const m=free();say(label,"volume",(m.volumeBytes/MIB).toFixed(0),"MiB, free",(m.freeBytes/MIB).toFixed(0),"MiB, used",(100*(m.volumeBytes-m.freeBytes)/m.volumeBytes).toFixed(1),"%");};
mkdirSync(join(cache,"k"),{recursive:true});
fill(48,5*MIB,1);fill(4,5*MIB,100);
show("before");
const reports:string[]=[];
for(const language of["en","de"] as const){
 const decision=await launch.relieveUnitCachePressureV1({root:cache,capacityBytes:capacity,emergencyBytes:8*MIB,environment:process.env,language,report:(line:string)=>{reports.push(line);say("report:",line);},nowMs:Date.now()+(language==="en"?0:1000000)});
 say("decision",language,decision);
 if(decision==="launch"||decision==="maintain"){for(let attempt=0;attempt<100;attempt++){await new Promise(done=>setTimeout(done,100));try{if(readFileSync(join(cache,"gc.log"),"utf8").includes("\"running\""))break;}catch{}}}
 say("collector log:",readFileSync(join(cache,"gc.log"),"utf8").trim().slice(0,200));
 show("after "+language);
 if(language==="en"){fill(20,5*MIB,300);show("refilled");}
}
say("--- ENOSPC drill: fill the volume completely, then run a command that needs 5 MiB");
const output=join(mount,"output.bin");
let runs=0;
const run=async()=>{runs++;const result=spawnSync("/bin/sh",["-c",`dd if=/dev/zero of='${output}' bs=1048576 count=5 2>&1 >/dev/null; echo "exit=$?"`],{encoding:"utf8"});const text=result.stdout;say("attempt",runs,text.trim().replace(/\n/gu," | ").slice(0,160));return{full:gc.gcOutOfSpaceV1(text),text:text.trim()};};
for(const language of["en","de"] as const){
 runs=0;rmSync(output,{force:true});fill(10,5*MIB,500+(language==="en"?0:100));
 const filler=join(mount,"filler");spawnSync("/bin/sh",["-c",`dd if=/dev/zero of='${filler}' bs=1048576 2>/dev/null; true`]);
 show("full ("+language+")");
 const last=await launch.retryWhenFullV1(run,{collect:()=>launch.collectAfterOutOfSpaceV1({root:cache,capacityBytes:capacity,language,report:(line:string)=>say("report:",line)}),stillFull:()=>launch.reportUnitCacheStillFullV1({root:cache,capacityBytes:capacity,language,report:(line:string)=>say("report:",line)})});
 say("final",language,"runs",runs,"full",last.full,"output bytes",existsSync(output)?statSync(output).size:0);
 rmSync(filler,{force:true});
}
say("--- the drill when nothing is collectable: the command is repeated once, then the image is reported still full");
rmSync(output,{force:true});rmSync(cache,{recursive:true,force:true});mkdirSync(join(cache,"k"),{recursive:true});
spawnSync("/bin/sh",["-c",`dd if=/dev/zero of='${join(mount,"filler")}' bs=1048576 2>/dev/null; true`]);
runs=0;const stuck=await launch.retryWhenFullV1(run,{collect:()=>launch.collectAfterOutOfSpaceV1({root:cache,capacityBytes:capacity,language:"en",report:(line:string)=>say("report:",line)}),stillFull:()=>launch.reportUnitCacheStillFullV1({root:cache,capacityBytes:capacity,language:"de",report:(line:string)=>say("report:",line)})});
say("final stuck","runs",runs,"full",stuck.full);
rmSync(join(mount,"filler"),{force:true});
