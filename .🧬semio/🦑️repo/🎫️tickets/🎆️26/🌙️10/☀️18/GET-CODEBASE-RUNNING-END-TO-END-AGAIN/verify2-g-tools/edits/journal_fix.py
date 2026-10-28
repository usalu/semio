"""Batch `journal-fix`: the journal service survives bursts on a busy tree, stops when its workspace vanished, and the plan-reuse oracle starts no service."""
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
sub(N + "/🧬️schema/🔣️.json", '"maximumResidentBytes": 52428800,', '"maximumResidentBytes": 134217728,')
sub(J + "/🟦️.ts", 'import {mkdirSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";', 'import {existsSync,mkdirSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";')
sub(J + "/🟦️.ts", '  if(at-activity>policy.idleShutdownMilliseconds){void stop("idle");return;}', '  if(!existsSync(root)){void stop("root-gone").then(()=>rmSync(directory,{recursive:true,force:true}));return;}\n  if(at-activity>policy.idleShutdownMilliseconds){void stop("idle");return;}')
put(J + "/🧪️tests/🌊️burst/🟦️.ts", '''import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CHANGE_JOURNAL_POLICY_V1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {runChangeJournalServiceV1,type ChangeWatchFactoryV1} from "../../🟦️.ts";

/** 🌊️ Child process of the burst law: a fresh runtime (a test runner's own memory would hide the effect) receives `batches` x `rows` change rows at once through a fake watcher, with the memory bound given on the command line, and prints `{result,peakBytes}` where `result` is `alive` or the reason the service stopped itself. */
const [batches,rows,bound]=process.argv.slice(2).map(Number),directory=mkdtempSync(join(tmpdir(),"journal-burst-")),policy={...CHANGE_JOURNAL_POLICY_V1,heartbeatMilliseconds:50,maximumResidentBytes:bound};
let emit!:(rows:[string,number,string][])=>Promise<void>;
const factory:ChangeWatchFactoryV1=options=>({async start(){emit=batch=>options.onBatch(batch,Date.now());return{backend:"fsevents",trust:"exact"};},async stop(){}});
const service=await runChangeJournalServiceV1({root:directory,directory:join(directory,"journal"),fence:".fence",exclude:{rooted:[],names:[]},watcherPolicy:{},createSupervisor:factory,policy});
let peak=0;const meter=setInterval(()=>{peak=Math.max(peak,process.memoryUsage().rss);},10);
for(let batch=0;batch<batches;batch++){
 await emit(Array.from({length:rows},(_,index)=>["m",0,"module/directory"+(batch%50)+"/file"+(batch*rows+index)+".ts"] as [string,number,string]));
 if(batch%20===0)await new Promise(resolve=>setTimeout(resolve,5));
}
await new Promise(resolve=>setTimeout(resolve,400));
const result=await Promise.race([service.closed,new Promise<string>(resolve=>setTimeout(()=>resolve("alive"),200))]);
clearInterval(meter);await service.stop("burst-end");rmSync(directory,{recursive:true,force:true});
console.log(JSON.stringify({result,peakBytes:peak}));
process.exit(0);
''')
sub(J + "/🧪️tests/🟦️.ts", 'test.skipIf(!binary)("the detached process starts on demand with the real watcher, is shared by concurrent starters, stays under 50 MB,', '''test("a service whose workspace vanished stops itself and removes its journal",async()=>{
 const{root,directory}=workspace(),tight={...policy,heartbeatMilliseconds:20};
 const service=await runChangeJournalServiceV1({root,directory,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake().factory,policy:tight,residentBytes:()=>0});
 expect(existsSync(directory)).toBe(true);
 rmSync(root,{recursive:true,force:true});
 expect(await service.closed).toBe("root-gone");
 await sleep(100);
 expect(existsSync(directory)).toBe(false);
});

test("a burst of one million change rows does not stop the service under its declared memory bound, and does stop it under the former 50 MiB bound (negative control)",()=>{
 const run=(bound:number)=>JSON.parse(execFileSync(process.execPath,[join(here,"🌊️burst","🟦️.ts"),"1000","1000",String(bound)],{encoding:"utf8"}).trim().split("\\n").pop()!) as {result:string;peakBytes:number};
 const declared=run(policy.maximumResidentBytes);
 expect(declared.result).toBe("alive");expect(declared.peakBytes).toBeLessThan(policy.maximumResidentBytes);
 expect(run(50*1024*1024).result).toBe("memory");
},60000);

test.skipIf(!binary)("the detached process starts on demand with the real watcher, is shared by concurrent starters, stays under its memory bound,''', marker="a service whose workspace vanished")
P = B + "/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts"
sub(P, 'import {bazelPlanKeyV1}from"../🟦️.ts";', 'import {bazelPlanKeyV1}from"../🟦️.ts";\nimport {changeJournalDirectoryV1}from"../../../../🏡️resident/📜️journal/🟦️.ts";')
sub(P, ' const previous=process.env.SEMIO_REPO_PLAN_REUSE;\n', ' const previous=process.env.SEMIO_REPO_PLAN_REUSE,journal=process.env.SEMIO_REPO_CHANGES;process.env.SEMIO_REPO_CHANGES="off";\n')
sub(P, ' }finally{if(previous===undefined)delete process.env.SEMIO_REPO_PLAN_REUSE;else process.env.SEMIO_REPO_PLAN_REUSE=previous;rmSync(base,{recursive:true,force:true});}', ' expect(existsSync(changeJournalDirectoryV1(root))).toBe(false);\n }finally{if(previous===undefined)delete process.env.SEMIO_REPO_PLAN_REUSE;else process.env.SEMIO_REPO_PLAN_REUSE=previous;if(journal===undefined)delete process.env.SEMIO_REPO_CHANGES;else process.env.SEMIO_REPO_CHANGES=journal;rmSync(base,{recursive:true,force:true});}')
sub(P, 'import {chmodSync,cpSync,lstatSync,', 'import {chmodSync,cpSync,existsSync,lstatSync,')

# a service never outlives its journal directory and a hanging watcher stop cannot keep it alive
sub(J + "/🟦️.ts", '  try{await supervisor?.stop();}catch{}\n', '  try{await Promise.race([supervisor?.stop(),new Promise<void>(resolve=>{const bound=setTimeout(resolve,policy.stopWaitMilliseconds);bound.unref?.();})]);}catch{}\n')
sub(J + "/🟦️.ts", '  if(!existsSync(root)){void stop("root-gone").then(()=>rmSync(directory,{recursive:true,force:true}));return;}\n', '  if(!existsSync(root)){void stop("root-gone").then(()=>rmSync(directory,{recursive:true,force:true}));return;}\n  if(!existsSync(directory)){void stop("directory-gone");return;}\n')
sub(J + "/🟦️.ts", ' for(const signal of["SIGTERM","SIGINT"] as const)process.once(signal,()=>void service.stop(signal));', ' for(const signal of["SIGTERM","SIGINT"] as const)process.once(signal,()=>{void service.stop(signal);setTimeout(()=>process.exit(1),CHANGE_JOURNAL_POLICY_V1.stopWaitMilliseconds*3).unref();});')
sub(N + "/🧬️schema/🔣️.json", '        "maximumResidentBytes": 134217728,\n', '        "maximumResidentBytes": 134217728,\n        "stopWaitMilliseconds": 3000,\n')
sub(N + "/🟦️.ts", 'unavailableMilliseconds:number;', 'unavailableMilliseconds:number;stopWaitMilliseconds:number;')
sub(J + "/🧪️tests/🟦️.ts", 'test("a burst of one million change rows', '''test("a service stops itself when its journal directory vanished, and a watcher whose stop never returns cannot keep it alive",async()=>{
 const tight={...policy,heartbeatMilliseconds:20,stopWaitMilliseconds:100},first=workspace();
 const service=await runChangeJournalServiceV1({...first,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake().factory,policy:tight,residentBytes:()=>0});
 rmSync(first.directory,{recursive:true,force:true});
 expect(await service.closed).toBe("directory-gone");
 const second=workspace(),hanging:ChangeWatchFactoryV1=()=>({async start(){return{backend:"fsevents",trust:"exact"};},stop:()=>new Promise<void>(()=>{})});
 const stuck=await runChangeJournalServiceV1({...second,fence:".fence",exclude,watcherPolicy:{},createSupervisor:hanging,policy:tight,residentBytes:()=>0});
 const started=performance.now();await stuck.stop("test");
 expect(await stuck.closed).toBe("test");expect(performance.now()-started).toBeLessThan(2000);
});

test("a burst of one million change rows''')
