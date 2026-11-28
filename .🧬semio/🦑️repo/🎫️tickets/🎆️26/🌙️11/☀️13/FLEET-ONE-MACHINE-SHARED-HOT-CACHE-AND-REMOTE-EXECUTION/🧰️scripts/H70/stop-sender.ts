import{expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync,chmodSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawn,spawnSync}from"node:child_process";
import{READY_TREE_POLICY_V1 as TREE,readyAdoptionClockV1,readyKillRowsV1,readyStalledV1,readyStopSenderV1,type ReadyStopSenderInputV1}from"../../🌲️process-tree/🟦️.ts";
import{READY_CONTAINMENT_POLICY_V1 as CONTAIN,readyKillRecorderV1}from"../../🌲️process-tree/🧹️containment/🟦️.ts";
import{READY_POLICY_V1 as P,READY_STAGES_V1 as STAGES,runReadyStageV1}from"../../🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};

/** 🛑️ Contract: a stage that ended by a signal names its sender from what the machine recorded BEFORE the signal was sent (the stop reason in the stage record, the wrapper's own deadline, the warden's cancel row, the kill record of the stage runner), a sender that left no record is named by what it was not, a stage stopped by another supervisor than the one that watches its end is classified by the recorded reason and not as an unknown SIGTERM, and a stage adopted without history is never stopped for the time before the adoption.
 * @see ../../🌲️process-tree/🟦️.ts */
const oracle=(mode:string,payload:unknown)=>{const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),mode,JSON.stringify(payload)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const END=1_000_000_000;
const base:ReadyStopSenderInputV1={ended:"SIGTERM",stagePid:100,endedMs:END,observer:5,kills:[]};
const cases:{name:string;input:ReadyStopSenderInputV1;want:string}[]=[
 {name:"not a signal end",input:{...base,ended:null},want:"none"},
 {name:"wrapper deadline",input:{...base,sent:[{signal:"SIGTERM",reason:"deadline",atMs:END-1000}]},want:"deadline"},
 {name:"this supervisor",input:{...base,stopBy:{supervisor:5,reason:"stall",atMs:END-10_000}},want:"supervisor"},
 {name:"another supervisor",input:{...base,stopBy:{supervisor:7,reason:"restart",atMs:END-10_000}},want:"supervisor"},
 {name:"stale supervisor record explains nothing, wrapper reached",input:{...base,stopBy:{supervisor:7,reason:"stall",atMs:END-120_000},received:[{signal:"SIGTERM",atMs:END-50}]},want:"outside-wrapper"},
 {name:"warden",input:{...base,warden:{atMs:END-5000,kind:"stage",reason:"danger",megabytes:70}},want:"warden"},
 {name:"kill record names the stage process",input:{...base,kills:[{atMs:END-500,pid:100,signal:"SIGTERM",killer:5,stage:"workspace"}]},want:"runner"},
 {name:"kill record of another process only",input:{...base,kills:[{atMs:END-500,pid:999,signal:"SIGTERM",killer:5,stage:"workspace"}]},want:"outside-child"},
 {name:"wrapper reached, nothing recorded",input:{...base,received:[{signal:"SIGTERM",atMs:END-40}]},want:"outside-wrapper"},
 {name:"exit status 143 without a signal",input:{...base,ended:"exit status 143"},want:"outside-child"},
 {name:"precedence: supervisor before warden before kill record",input:{...base,stopBy:{supervisor:7,reason:"cancel",atMs:END-1000},warden:{atMs:END-900,kind:"stage",reason:"danger",megabytes:70},kills:[{atMs:END-800,pid:100,signal:"SIGTERM",killer:5,stage:"workspace"}]},want:"supervisor"},
 {name:"old warden row does not explain a later end, the kill record does",input:{...base,warden:{atMs:END-500_000,kind:"stage",reason:"danger",megabytes:70},kills:[{atMs:END-800,pid:100,signal:"SIGTERM",killer:5,stage:"workspace"}]},want:"runner"},
];
test("the sender of a stopped stage equals an independent Python implementation",()=>{
 const expected=oracle("sender",cases.map(row=>row.input));
 cases.forEach((row,index)=>{expect(readyStopSenderV1(row.input).kind as string,row.name).toBe(row.want);expect(expected[index],row.name).toBe(row.want);});
});
test("the sender line is English and German, names the supervisor and says that the platform does not tell the pid of an unrecorded sender",()=>{
 const other=readyStopSenderV1(cases[3]!.input).line,self=readyStopSenderV1(cases[2]!.input).line,wrapper=readyStopSenderV1(cases[8]!.input).line,child=readyStopSenderV1(cases[9]!.input).line;
 expect(other).toContain("another readiness supervisor (process 7");expect(other).toContain("anderen Bereitschafts-Aufseher (Prozess 7");expect(other).toContain(" / ");
 expect(self).toContain("this readiness supervisor (process 5");expect(self).toContain("diesem Bereitschafts-Aufseher (Prozess 5");
 expect(wrapper).toContain("The sender is outside the stage runner");expect(wrapper).toContain("Der Absender liegt außerhalb des Stufenläufers");expect(wrapper).toContain("does not tell its pid");
 expect(child).toContain("received no signal");expect(child).toContain("Wrapper erhielt kein Signal");expect(child).toContain("exit status 143");
});
test("the kill record of the stage runner is read back exactly as the recorder wrote it, names with brackets included, and only rows since the start count",()=>{
 const directory=mkdtempSync(join(tmpdir(),"h70-kills-"));
 try{
  const record=readyKillRecorderV1({directory,stage:"workspace",owner:"ready:abc:workspace:1",reason:"stall",killer:4242,seconds:600});
  record({pid:777,ppid:1,startMs:1,name:"bun (worker) x"},"SIGTERM");record({pid:778,ppid:1,startMs:1},"SIGKILL");
  const text=readFileSync(join(directory,CONTAIN.killRecordName),"utf8"),rows=readyKillRowsV1(text,0);
  expect(rows.map(row=>[row.pid,row.signal,row.killer,row.stage])).toEqual([[777,"SIGTERM",4242,"workspace"],[778,"SIGKILL",4242,"workspace"]]);
  expect(readyKillRowsV1(text,Date.now()+60_000)).toEqual([]);
  expect(readyKillRowsV1("garbage\n2026-01-01T00:00:00.000Z Process x\n",0)).toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("a stage adopted without history starts its stall clock at the adoption and is not stopped for the time before it",()=>{
 const started=1_000_000,observed=started+3000*1000,clockCases=[{startedMs:started,observedMs:observed},{startedMs:started,observedMs:started-5000},{startedMs:started,observedMs:started}];
 expect(clockCases.map(row=>readyAdoptionClockV1(row))).toEqual(oracle("clock",clockCases));
 const clock=readyAdoptionClockV1({startedMs:started,observedMs:observed});
 expect(readyStalledV1(observed-clock,TREE),"just adopted").toBe(false);
 expect(readyStalledV1(observed-started,TREE),"the old rule would stop it at once").toBe(true);
 expect(readyStalledV1(observed+TREE.stallStopMilliseconds-clock,TREE),"a stage that stays quiet after the adoption is still stopped").toBe(true);
});

const posix=process.platform==="win32"?test.skip:test;
const wrapperFile=join(import.meta.dir,"..","..","🏃️wrapper","🟦️.ts");
const temporary:string[]=[];
const scratch=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`h70-${label}-`));temporary.push(path);return path;};
const sleepMs=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const children=(pid:number):number[]=>spawnSync("pgrep",["-P",String(pid)],{encoding:"utf8"}).stdout.split("\n").filter(Boolean).map(Number);
const wrapperRun=async(extra:readonly string[],act:(wrapper:number)=>Promise<void>)=>{
 const exitFile=join(scratch("wrapper"),"stage.exit"),child=spawn(process.execPath,[wrapperFile,...extra,exitFile,"sleep","30"],{stdio:"ignore"});
 const ended=new Promise<void>(done=>child.on("close",()=>done()));
 for(let waited=0;children(child.pid!).length===0&&waited<10_000;waited+=50)await sleepMs(50);
 await act(child.pid!);await ended;
 const row=JSON.parse(readFileSync(exitFile,"utf8")) as Record<string,unknown>;
 expect(validateJsonSchemaSubset(schema.definitions.Exit,row,schema),JSON.stringify(row)).toEqual([]);
 return row as {exit:number|null;signal:string|null;received?:{signal:string;atMs:number}[];sent?:{signal:string;reason:string;atMs:number}[]};
};
posix("the wrapper records the signal it received, and a command signalled below the wrapper leaves no such record",async()=>{
 const reached=await wrapperRun([],async wrapper=>{process.kill(wrapper,"SIGTERM");});
 expect(reached.signal).toBe("SIGTERM");expect(reached.received?.map(row=>row.signal)).toEqual(["SIGTERM"]);
 const below=await wrapperRun([],async wrapper=>{process.kill(children(wrapper)[0]!,"SIGTERM");});
 expect(below.signal).toBe("SIGTERM");expect(below.received).toBeUndefined();
},60000);
posix("the wrapper records its own deadline before it sends the signal",async()=>{
 const row=await wrapperRun([`--deadline=${Date.now()+1500}`],async()=>undefined);
 expect(row.signal).toBe("SIGTERM");expect(row.sent?.[0]?.reason).toBe("deadline");expect(row.sent?.[0]?.signal).toBe("SIGTERM");
},60000);

const fakeHome=(script:string):string=>{const home=scratch("home"),bin=join(home,".local","bin");mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);return home;};
const stage=STAGES.find(row=>row.name==="tools")!;
const recordFile=(root:string)=>join(root,...P.directory,P.inflightName);
const start=(home:string,root:string,signal:AbortSignal)=>runReadyStageV1({home,root,environment:{},stage,signal,jobs:1,onLine:()=>undefined});
const recordOf=async(root:string)=>{for(let waited=0;!existsSync(recordFile(root))&&waited<15_000;waited+=50)await sleepMs(50);await sleepMs(300);return JSON.parse(readFileSync(recordFile(root),"utf8")) as {pid:number;[key:string]:unknown};};
posix("a signal that nobody recorded is named as coming from outside the stage runner, and is no stall",async()=>{
 const home=fakeHome("#!/bin/sh\nexec sleep 400\n"),root=scratch("root"),running=start(home,root,new AbortController().signal),record=await recordOf(root);
 process.kill(record.pid,"SIGTERM");
 const result=await running;
 expect(result.signal).toBe("SIGTERM");expect(result.sender).toBe("outside-wrapper");expect(result.stalled).toBe(false);
 expect(result.tail.join("\n")).toContain("The sender is outside the stage runner");expect(result.failure?.join("\n")).toContain("Der Absender liegt außerhalb des Stufenläufers");
},120000);
posix("a stage stopped by another supervisor while this one watched is classified by the recorded reason, not as an unknown SIGTERM",async()=>{
 const home=fakeHome("#!/bin/sh\nexec sleep 400\n"),root=scratch("root"),first=new AbortController(),firstRun=start(home,root,first.signal),record=await recordOf(root);
 first.abort();await firstRun;
 writeFileSync(recordFile(root),JSON.stringify({...record,stopBy:{supervisor:424242,reason:"stall",atMs:Date.now()}}));
 const second=start(home,root,new AbortController().signal);
 await sleepMs(1500);
 process.kill(record.pid,"SIGTERM");
 const result=await second;
 expect(result.sender).toBe("supervisor");expect(result.stalled,"the recorded reason classifies the end").toBe(true);
 expect(result.tail.join("\n")).toContain("another readiness supervisor (process 424242");
},120000);
posix("a stop this supervisor sends is recorded in the stage record before the signal and named as its own",async()=>{
 const home=fakeHome("#!/bin/sh\nexec sleep 400\n"),root=scratch("root"),running=start(home,root,new AbortController().signal);
 await recordOf(root);
 writeFileSync(join(root,...P.directory,P.cancelName),"");
 const result=await running;
 expect(result.cancelled).toBe(true);expect(result.sender).toBe("supervisor");expect(result.tail.join("\n")).toContain("this readiness supervisor (process "+process.pid);
},120000);
test("cleanup",()=>{for(const path of temporary)rmSync(path,{recursive:true,force:true});});
