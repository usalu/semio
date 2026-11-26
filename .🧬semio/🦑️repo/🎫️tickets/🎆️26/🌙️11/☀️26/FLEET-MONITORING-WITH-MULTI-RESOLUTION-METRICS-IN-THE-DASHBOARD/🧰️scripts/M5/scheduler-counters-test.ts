/** 📈️ Contract: the counters of the scheduler survive a restart: `cumulative()` answers the ledger of the earlier runs plus this run, per machine (`offload.<origin>`) and for the scheduler as a whole, and the ledger file is valid against its schema (checked with Ajv).
 * @see ../../🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import Ajv from"ajv";
import{mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{openCounterLedgerV1}from"../../../../📈️counter-ledger/🟦️.ts";
import schema from"../../../../📈️counter-ledger/🧬️schema/🔣️.json"with{type:"json"};
import type{RemoteExecutionContentV1}from"../../../🗄️content/🟦️.ts";
import type{RemoteExecutionWorkerRegistrationV1}from"../../../🟦️.ts";
import{RemoteExecutionSchedulerV1}from"../../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const profile=(name:string):RemoteExecutionWorkerRegistrationV1=>({version:1,name,host:"h",platform:"darwin-arm64",properties:[{name:"OSFamily",value:"macos"}],concurrency:1,sandbox:"isolated-root",runtime:"test",origin:name,capacity:{cores:8,memoryMegabytes:16384},confinement:"none",network:"unenforced"});
const open=(path:string)=>{
 const ledger=openCounterLedgerV1({path,name:"remote-execution",flushMilliseconds:3600000});
 const scheduler=new RemoteExecutionSchedulerV1({content:{} as RemoteExecutionContentV1,unmatchedSeconds:30,lingerSeconds:0.05,leaseSeconds:10,maximumAttempts:3,defaultActionSeconds:60,maximumActionSeconds:600,maximumQueued:100,heartbeatSeconds:2,pollSeconds:5,loadExpireSeconds:15,refusalCooldownSeconds:2,cache:{url:"http://c",relative:false},language:"none",sweepMilliseconds:0,ledger});
 return {ledger,scheduler};
};

test("a restarted scheduler answers the counts of its earlier runs and its own, per machine and as a whole",()=>{
 const directory=mkdtempSync(join(tmpdir(),"sched-counters-"));scratch.push(directory);const path=join(directory,"build-scheduler.counters.json");
 const first=open(path);
 first.scheduler.register(profile("g1-m01"));first.scheduler.register(profile("g1-m01"));
 for(const field of["offloadedIn","offloadedIn","offloadedOut","refused"] as const)first.scheduler.load.count("g1-m01",field);
 first.scheduler.load.count("g1-m02","offloadedIn");
 expect(first.scheduler.cumulative()).toMatchObject({scheduler:{workersReused:1},"offload.g1-m01":{offloadedIn:2,offloadedOut:1,refused:1},"offload.g1-m02":{offloadedIn:1}});
 first.scheduler.close();first.ledger.close();
 const written=JSON.parse(readFileSync(path,"utf8")) as {restarts:number;groups:Record<string,Record<string,number>>};
 const valid=new Ajv({strict:true,allErrors:true}).compile({definitions:schema.definitions,$ref:"#/definitions/Ledger"});
 expect(valid(written),JSON.stringify(valid.errors)).toBe(true);expect(written.restarts).toBe(1);expect(written.groups.scheduler!.workersReused).toBe(1);
 const second=open(path);
 expect(second.scheduler.cumulative()["offload.g1-m01"]).toEqual({offloadedIn:2,offloadedOut:1,refused:1});
 second.scheduler.load.count("g1-m01","offloadedIn");second.scheduler.register(profile("g1-m03"));second.scheduler.register(profile("g1-m03"));
 const total=second.scheduler.cumulative();
 expect(total["offload.g1-m01"]!.offloadedIn).toBe(3);expect(total.scheduler!.workersReused).toBe(2);expect(second.ledger.restarts).toBe(2);
 expect(second.scheduler.status().counters.workersReused).toBe(1);
 second.scheduler.close();second.ledger.close();
});
