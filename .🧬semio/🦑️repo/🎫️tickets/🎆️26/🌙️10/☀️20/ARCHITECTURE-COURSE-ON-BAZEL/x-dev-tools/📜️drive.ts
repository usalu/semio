/** 🎬 Runs one continuous target through `bun repo --tool=<tool> run` in the foreground with a hard deadline: records when the build tool reports ready against what an independent probe sees, cancels it like Ctrl+C and proves the ports are free.
 * usage: bun 📜️drive.ts <target> --ports=6061,8791 [--tool=bazel] [--signal=group|leader] [--deadline=900] [--hold=1] [--env=K=V]... [--log=path] */
import {spawn,spawnSync} from "node:child_process";
import {connect} from "node:net";
import {appendFileSync,writeFileSync} from "node:fs";
import {resolve} from "node:path";

const arg=(name:string,fallback?:string)=>process.argv.find(value=>value.startsWith(`--${name}=`))?.slice(name.length+3)??fallback;
const target=process.argv[2]!,ports=(arg("ports","")!).split(",").filter(Boolean).map(Number),tool=arg("tool","bazel")!,mode=arg("signal","group")!,deadline=Number(arg("deadline","900"))*1000,hold=Number(arg("hold","1"))*1000,log=resolve(arg("log","drive.log")!);
const env={...process.env};for(const value of process.argv.filter(value=>value.startsWith("--env=")))env[value.slice(6,value.indexOf("=",6))]=value.slice(value.indexOf("=",6)+1);
const root=resolve(import.meta.dir,"../../../../../../../..");
writeFileSync(log,"");
const started=performance.now(),at=()=>((performance.now()-started)/1000).toFixed(1).padStart(7);
const say=(line:string)=>{const row=`${at()}s ${line}\n`;appendFileSync(log,row);process.stdout.write(row);};
const listening=(port:number)=>new Promise<boolean>(accept=>{const socket=connect({host:"127.0.0.1",port});socket.setTimeout(500);socket.once("connect",()=>{socket.destroy();accept(true);});socket.once("error",()=>accept(false));socket.once("timeout",()=>{socket.destroy();accept(false);});});
const answers=async(port:number)=>{try{const response=await fetch(`http://127.0.0.1:${port}/`,{signal:AbortSignal.timeout(2000)});await response.arrayBuffer();return response.status;}catch{return 0;}};
const holders=(port:number)=>spawnSync("lsof",["-nP",`-iTCP:${port}`,"-sTCP:LISTEN","-t"],{encoding:"utf8"}).stdout.split("\n").filter(Boolean).map(Number);

const child=spawn("bun",["repo",`--tool=${tool}`,"run",target],{cwd:root,env,detached:true,stdio:["ignore","pipe","pipe"]});
say(`spawned pid=${child.pid} target=${target} tool=${tool}`);
const result:Record<string,unknown>={target,tool,ports};let readyAt:number|undefined,cancelAt:number|undefined,startedAt:number|undefined,exitCode:number|null|undefined,exitSignal:string|null|undefined,exited=false;
const lines:string[]=[];let buffer="";
const take=(chunk:Buffer)=>{buffer+=chunk.toString();let index;while((index=buffer.indexOf("\n"))>=0){const line=buffer.slice(0,index);buffer=buffer.slice(index+1);lines.push(line);say(`| ${line.slice(0,240)}`);if(/\[bazel\] task-started/.test(line)&&startedAt===undefined)startedAt=performance.now();if(/task-ready|\[semio\] ready/.test(line)&&readyAt===undefined){readyAt=performance.now();void atReady();}}};
child.stdout.on("data",take);child.stderr.on("data",take);
const exit=new Promise<void>(accept=>child.once("exit",(code,signal)=>{exited=true;exitCode=code;exitSignal=signal;say(`exit code=${code} signal=${signal}`);accept();}));
const kill=(signal:NodeJS.Signals)=>{try{process.kill(mode==="group"?-child.pid!:child.pid!,signal);}catch{}};
const hardStop=setTimeout(()=>{say("HARD DEADLINE: killing the process group");result.deadline=true;try{process.kill(-child.pid!,"SIGKILL");}catch{}},deadline);
async function atReady(){
 const probes=await Promise.all(ports.map(async port=>({port,connect:await listening(port),http:await answers(port)})));
 result.atReady={afterTaskStartedSeconds:startedAt===undefined?null:(readyAt!-startedAt)/1000,probes};say(`READY observed; independent probes ${JSON.stringify(probes)}`);
 await new Promise(accept=>setTimeout(accept,hold));
 say(`CANCEL (${mode==="group"?"SIGINT to the process group, like Ctrl+C":"SIGINT to the leader only"})`);cancelAt=performance.now();kill("SIGINT");
}
await Promise.race([exit,new Promise<void>(accept=>setTimeout(accept,deadline+30_000))]);
clearTimeout(hardStop);
const settled=cancelAt===undefined?undefined:(performance.now()-cancelAt)/1000;
let after=await Promise.all(ports.map(async port=>({port,listening:await listening(port),holders:holders(port)})));
for(let attempt=0;attempt<10&&after.some(row=>row.listening);attempt++){await new Promise(accept=>setTimeout(accept,1000));after=await Promise.all(ports.map(async port=>({port,listening:await listening(port),holders:holders(port)})));}
const leaked=after.filter(row=>row.listening);
for(const row of leaked)for(const pid of row.holders){say(`LEAK: pid ${pid} still holds ${row.port}; killing`);try{process.kill(pid,"SIGKILL");}catch{}}
Object.assign(result,{readyObserved:readyAt!==undefined,exitCode,exitSignal,secondsFromCancelToExit:settled,portsAfter:after,leaked:leaked.length>0});
say(`RESULT ${JSON.stringify(result)}`);
process.exit(readyAt!==undefined&&!leaked.length&&exited?0:1);
