import {mkdirSync,rmSync,writeFileSync,readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {join} from "node:path";
import buildPolicy from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import {readHostCapacityPolicyV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import {defaultHostWardenPortsV1,runHostWardenLoopV1,readHostWardenStateV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts";

const base=process.argv[2]!,hold=(megabytes:number)=>Bun.spawn(["python3","-c",`import time\nb=bytearray(${megabytes}*2**20)\nfor i in range(0,len(b),4096): b[i]=1\ntime.sleep(600)`],{stdout:"ignore",stderr:"ignore"});
rmSync(base,{recursive:true,force:true});mkdirSync(join(base,"grants"),{recursive:true});mkdirSync(join(base,"warden"),{recursive:true});
const grantProc=hold(900);let foreignProc=grantProc;
await Bun.sleep(1500);
const started=execFileSync("ps",["-o","etime=","-p",String(grantProc.pid)],{encoding:"utf8"}).trim();
writeFileSync(join(base,"grants","live.json"),JSON.stringify({id:"live",operation:"command",jobs:1,cpu:1,reservedMegabytes:300,pid:grantProc.pid,priority:"batch"}));
const hostPolicy=readHostCapacityPolicyV1((buildPolicy as unknown as {hostCapacity:unknown}).hostCapacity);
const real=defaultHostWardenPortsV1({hostPolicy,directory:base});
const allowed=new Set<number>([grantProc.pid]),log:string[]=[];
const seen=new Set<string>();
const run=async(label:string,available:number)=>{
 const ports={...real,write:(line:string)=>{log.push(line);},signal:(pid:number,name:"SIGSTOP"|"SIGCONT"|"SIGKILL"|"SIGTERM")=>{if(!allowed.has(pid))throw Error("refused "+pid);real.signal(pid,name);},sample:async()=>({...(await real.sample()),...available>0?{availableMegabytes:available}:{}}),sleep:async(ms:number)=>{await Bun.sleep(Math.min(ms,300));const s=execFileSync("ps",["-o","state=","-p",String(grantProc.pid)],{encoding:"utf8"}).trim();if(s.startsWith("T")&&!seen.has("T")){seen.add("T");console.log("  during the run the grant process state is",s,"(stopped by SIGSTOP)");}}};
 const result=await runHostWardenLoopV1({ports,maxTicks:4,within:process.pid,code:"live-"+label});
 const state=readHostWardenStateV1(base)!;
 const stat=(pid:number)=>execFileSync("ps",["-o","state=","-p",String(pid)],{encoding:"utf8"}).trim();
 console.log(`\n== ${label} (available ${available>0?available+" MB (simulated)":"real "+state.host.availableMegabytes+" MB"})`);
 console.log("level",state.level,"trigger",state.trigger,"counters",JSON.stringify(result.counters));
 console.log("grants",JSON.stringify(state.grants),"foreign",JSON.stringify(state.foreign));
 console.log("grant process state",stat(grantProc.pid),"| foreign process state",stat(foreignProc.pid));
 return state.foreign!.megabytes;
};
const baseline=await run("baseline: no foreign allocation",0);
foreignProc=hold(700);allowed.add(foreignProc.pid);await Bun.sleep(1500);
const after=await run("spare memory: the host is calm, the over-grant process is only counted",0);
await run("tight memory: the same process is throttled before danger",2300);
console.log(`\nforeign memory counted: ${baseline} MB before the 700 MB allocation outside every grant, ${after} MB after (delta ${after-baseline} MB)`);
console.log("\nalarm lines:\n"+log.filter(l=>l.includes("suspended")||l.includes("angehalten")).join("\n"));
for(const proc of [grantProc,foreignProc]){try{process.kill(proc.pid,"SIGCONT");process.kill(proc.pid,"SIGKILL");}catch{}}
