import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as gate from "../../🎫️gate/🟦️.ts";
import {growHostGrantV1,type HostUpgradeLedgerV1} from "../../🔼️upgrade/🟦️.ts";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,takeHostLockV1,withHostCapacityV1,type HostGrantV1,type HostRequestV1,type HostSampleV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import decisionFixture from "../🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hcf-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(available:number):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:65536,availableMegabytes:available,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const owner=(directory:string):string|undefined=>{try{return readFileSync(join(directory,"lock","owner"),"utf8");}catch{return undefined;}};
const peer=process.ppid>1?process.ppid:1;
const breakAndTake=(directory:string):string=>{const later=Date.now()+policy.wait.lockStaleMilliseconds+1000;expect(takeHostLockV1(directory,later,policy,peer),"the peer finds the stalled holder's lock stale and breaks it").toBe(false);expect(takeHostLockV1(directory,later,policy,peer),"then the peer takes the lock").toBe(true);return owner(directory)!;};

test("A holder that stalled past the lock's staleness while a peer broke and took the lock gives back nothing: the peer's lock survives the holder's release",async()=>{
 const directory=fresh();let taken:string|undefined;
 const request:HostRequestV1={id:newHostCapacityIdV1(),operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:1000,perJobMegabytes:0,cpuPerJob:1};
 await withHostCapacityV1({directory,policy,request,pid:process.pid,environment:{},sample:async()=>({host:host(40000),swappedOutMegabytes:0}),control:{signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined},write:line=>{if(taken===undefined&&line.includes("admitted"))taken=breakAndTake(directory);}},async()=>undefined);
 expect(taken,"the stall happened inside the admission's lock").toBeDefined();
 expect([existsSync(join(directory,"lock")),owner(directory)],"the peer still holds the lock it took").toEqual([true,taken]);
 (gate as unknown as {releaseHostLockV1?:(directory:string,pid:number)=>void}).releaseHostLockV1?.(directory,peer);
});

test("A growth whose holder lost the lock while it stalled writes no grown row under the peer's lock",async()=>{
 const directory=fresh(),published:HostGrantV1[]=[];let taken:string|undefined;
 const full:HostRequestV1={id:"growfenced1",operation:"run",priority:"batch",minimumJobs:1,preferredJobs:8,fixedMegabytes:1000,perJobMegabytes:1000,cpuPerJob:1};
 const row:HostGrantV1={id:full.id,operation:"run",jobs:1,cpu:1,reservedMegabytes:2000,admittedAtMs:Date.now()-3_600_000,observedMegabytes:500,pid:process.pid,bootAtMs:1};
 const optional=gate as unknown as {releaseHostLockV1?:(directory:string,pid:number)=>void;holdsHostLockV1?:(directory:string,pid:number)=>boolean};
 const ledger={protocol:schema.definitions.Protocol.const,takeLock:takeHostLockV1,releaseLock:optional.releaseHostLockV1,holdsLock:optional.holdsHostLockV1,readGrants:()=>[row],readWaiters:()=>[],queuePath:(root:string,at:number,id:string)=>join(root,"queue",`${at}-${id}.json`),sample:async()=>host(40000)} as unknown as HostUpgradeLedgerV1;
 const grown=await growHostGrantV1({ledger,directory,policy,full,now:Date.now,pid:process.pid,language:"en",write:()=>undefined,state:()=>{taken??=breakAndTake(directory);return row;},publish:next=>{published.push(next);},settled:()=>({attempts:1,jobs:8}),settle:()=>undefined});
 expect(taken,"the stall happened inside the growth's lock").toBeDefined();
 expect([grown,published.length],"no growth is decided or written under a lock the holder no longer owns").toEqual([undefined,0]);
 expect(owner(directory),"the peer still holds the lock it took").toBe(taken);
 optional.releaseHostLockV1?.(directory,peer);
});

test("A waiting request publishes its progress to the caller only after it gave the gate lock back, so a slow caller never holds the machine-wide lock",async()=>{
 const directory=fresh(),controller=new AbortController(),seen:(string|undefined)[]=[];
 const request:HostRequestV1={id:newHostCapacityIdV1(),operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:30000,perJobMegabytes:0,cpuPerJob:1};
 const waiting=withHostCapacityV1({directory,policy,request,pid:process.pid,environment:{},sample:async()=>({host:host(2000),swappedOutMegabytes:0}),control:{signal:controller.signal,remainingMilliseconds:()=>null,publish:async()=>{seen.push(owner(directory));controller.abort(Object.assign(Error("stop"),{code:"cancelled"}));},yieldContinuation:async()=>undefined},write:()=>undefined},async()=>undefined);
 await expect(waiting).rejects.toThrow();
 expect(seen.length,"the wait published its progress").toBeGreaterThan(0);
 expect(seen.every(text=>text===undefined||!text.startsWith(process.pid+":")),"no progress was published while this process held the gate lock").toBe(true);
});
