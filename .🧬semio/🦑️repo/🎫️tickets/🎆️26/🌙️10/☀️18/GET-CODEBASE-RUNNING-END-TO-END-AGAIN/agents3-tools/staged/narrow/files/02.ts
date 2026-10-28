/** 👥️ Contract: three agents run builds side by side on a declared 16 GB / 8-core machine. An agent session with nothing running gets its first job whenever memory fits, a growth that our other runs' cores leave short of jobs runs with the jobs they leave and keeps its whole memory, the project farm's elastic request never makes a run wait, a nested invocation that inherits a quiet grant works with the jobs of the full request at once (never the quiet row's single job) and leaves the growth to the executors, and the reduced growth says so in English and German; the decision equals the independent Python model on every agent-session world.
 * @see ../🐍️oracle/🐍️.py
 * @see ./🥒️.feature
 * @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-agents16.md */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {evaluateHostCapacityV1,formatHostCapacityV1,hostBootAtV1,hostFloorMegabytesV1,planHostCapacityV1,quietHostRequestV1,readHostCapacityPolicyV1,upgradeHostGrantV1,withHostCapacityV1,type HostGrantV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../../../🟦️.ts";
import decisionFixture from "../../🧫️fixtures/🔣️.json";
import upgradeFixture from "../🧫️fixtures/🔣️.json";
import followupFixture from "../../🌤️followup/🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1(decisionFixture.policy),fast=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:10}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"ag16-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const machine=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:0,platform:"darwin",cores:8,totalMegabytes:16384,availableMegabytes:15490,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:27,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const session=(index:number)=>`a0000000000${index}.claude`;
const running=(id:string,owner:number,reservedMegabytes:number,observedMegabytes=400):HostGrantV1=>({id,operation:"run",jobs:8,cpu:8,reservedMegabytes,admittedAtMs:900_000,observedMegabytes,pid:1000+owner,bootAtMs:1,session:session(owner),rampMilliseconds:168_836,resumedAtMs:901_000,upgradedAtMs:901_000});
const quiet=(id:string,owner:number,arrivedAtMs:number):HostGrantV1=>({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:1502,admittedAtMs:arrivedAtMs,observedMegabytes:200,pid:1000+owner,bootAtMs:1,session:session(owner),quiet:true,arrivedAtMs,rampMilliseconds:60_000});
const full=(id:string,fixedMegabytes:number,perJobMegabytes:number):HostRequestV1=>({id,operation:"run",priority:"interactive",minimumJobs:1,preferredJobs:8,fixedMegabytes,perJobMegabytes,cpuPerJob:1,rampMilliseconds:168_836});
const growth=(grant:HostGrantV1,request:HostRequestV1):HostWaiterV1=>({request:{...request,minimumJobs:request.preferredJobs},arrivedAtMs:grant.arrivedAtMs!,pid:grant.pid,upgrade:true,session:grant.session!,weight:1});
const verdict=(decisions:readonly {id:string;verdict:string;jobs:number;reservedMegabytes:number;reason:string}[],id:string)=>{const row=decisions.find(item=>item.id===id)!;return [row.verdict,row.jobs,row.reservedMegabytes,row.reason];};

test("The decision equals the independent Python model on every agent-session world, and the worlds reach the first job of a session, the growth short of cores and the elastic request",()=>{
 const worlds=(upgradeFixture as unknown as {agents:{name:string;host:HostSampleV1;ledger:HostGrantV1[];queue:HostWaiterV1[];nowMs:number;expect:unknown}[]}).agents;
 expect(worlds.length).toBeGreaterThanOrEqual(100);
 for(const world of worlds)expect(evaluateHostCapacityV1(policy,world.host,world.ledger,world.queue,world.nowMs),world.name).toEqual(world.expect as never);
 for(const key of ["firstJob","upgradeCores","upgradeFirstJob","elasticAdmit","elasticQueued","elasticWait"])expect((upgradeFixture.coverage as Record<string,number>)[key]??0,key).toBeGreaterThan(0);
});

test("agents16 hot replay: two 8-job runs book every core of the declared 8-core machine, and the third agent's growth runs at once with one job and the memory of its whole request instead of waiting for a whole run",()=>{
 const a=running("courseaaa1",1,2861),b=running("printaaaa1",2,3649),c=quiet("dashaaaaa1",3,906_000),request=full(c.id,404,410);
 const decisions=evaluateHostCapacityV1(policy,{...machine(),atMs:907_000},[a,b,c],[growth(c,request)],907_000).decisions;
 expect(verdict(decisions,c.id)).toEqual(["admit",1,404+8*410,"reduced"]);
 for(const load of [null,2])expect(verdict(evaluateHostCapacityV1(policy,machine({load}),[a,b,c],[growth(c,request)],907_000).decisions,c.id),`load ${load}`).toEqual(["admit",1,404+8*410,"reduced"]);
});

test("agents16 mixed-long replay: beside a long 8-job run, the two quiet agents that have to grow both run, with the 7 jobs and the 1 job the cores leave, instead of both waiting 404 s for the long run to end",()=>{
 const a=running("printaaaa1",1,3649,2900),b=quiet("courseaaa1",2,960_000),c=quiet("dashaaaaa1",3,963_000);
 for(const nowMs of [961_000,1_000_000,1_200_000]){
  const decisions=evaluateHostCapacityV1(policy,{...machine(),atMs:nowMs},[a,b,c],[growth(b,full(b.id,279,384)),growth(c,full(c.id,404,410))],nowMs).decisions;
  expect([verdict(decisions,b.id),verdict(decisions,c.id)],`at ${nowMs} ms`).toEqual([["admit",7,279+8*384,"reduced"],["admit",1,404+8*410,"reduced"]]);
 }
});

test("An agent session with nothing running gets its first job past the cores our other runs book, a second request of a session that already runs waits for cores",()=>{
 const a=running("courseaaa1",1,2861),b=running("printaaaa1",2,3649),first:HostWaiterV1={request:{...full("dashaaaaa1",245,327),priority:"batch"},arrivedAtMs:905_000,pid:3,session:session(3),weight:1},second:HostWaiterV1={request:{...full("courseaaa2",245,327),priority:"batch"},arrivedAtMs:905_500,pid:4,session:session(1),weight:1};
 const decisions=evaluateHostCapacityV1(policy,machine(),[a,b],[first,second],906_000).decisions;
 expect(verdict(decisions,first.request.id)).toEqual(["admit",1,245+327,"reduced"]);
 expect(verdict(decisions,second.request.id)).toEqual(["wait",0,0,"load"]);
 const anonymous=evaluateHostCapacityV1(policy,machine(),[a,b],[{...first,session:undefined} as HostWaiterV1],906_000).decisions;
 expect(verdict(anonymous,first.request.id),"a request without an agent session keeps the cores ceiling").toEqual(["wait",0,0,"load"]);
});

test("The project farm's elastic request is decided after every other request: it never makes a run wait, never becomes a barrier head and waits behind a waiting run",()=>{
 const a=running("printaaaa1",1,3649,3400),other:HostGrantV1={...running("courseaaa1",2,5000,4800),jobs:2,cpu:2,admittedAtMs:100_000,resumedAtMs:100_000};
 const farm:HostWaiterV1={request:{id:"farmaaaaa1",operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:9,fixedMegabytes:1536,perJobMegabytes:2048,cpuPerJob:1,elastic:true},arrivedAtMs:700_000,pid:1001,session:session(1),weight:1};
 const run=(fixedMegabytes:number):HostWaiterV1=>({request:{id:"dashaaaaa1",operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes,perJobMegabytes:0,cpuPerJob:1},arrivedAtMs:899_000,pid:3,session:session(2),weight:1});
 const host={...machine({availableMegabytes:7000,load:2}),atMs:900_000};
 const fits=evaluateHostCapacityV1(policy,host,[a,other],[farm,run(1500)],900_000).decisions;
 expect(verdict(fits,"dashaaaaa1"),"the run is admitted although the farm request arrived 200 s earlier and does not fit").toEqual(["admit",1,1500,"fits"]);
 expect(verdict(fits,"farmaaaaa1")).toEqual(["wait",0,0,"memory"]);
 expect(fits.map(row=>row.id),"the farm request is decided last").toEqual(["dashaaaaa1","farmaaaaa1"]);
 const waits=evaluateHostCapacityV1(policy,host,[a,other],[farm,run(4000)],900_000).decisions;
 expect([verdict(waits,"dashaaaaa1"),verdict(waits,"farmaaaaa1")]).toEqual([["wait",0,0,"memory"],["wait",0,0,"queue-order"]]);
 const roomy=evaluateHostCapacityV1(policy,{...machine({availableMegabytes:14000,load:2}),atMs:900_000},[a],[farm],900_000).decisions;
 expect(verdict(roomy,"farmaaaaa1")[0],"alone and with memory the farm request is admitted").toBe("admit");
});

test("A nested invocation that inherits a quiet grant works with the jobs of the full request at once, never with the quiet row's single job, and follows the grant once it grew",async()=>{
 const directory=fresh(),id="inheritedaaaaa01",now=Date.now();
 mkdirSync(join(directory,"grants"),{recursive:true});mkdirSync(join(directory,"queue"),{recursive:true});
 const request=full(id,300,400),plan=planHostCapacityV1(policy,{totalMegabytes:16384},request,8),row:HostGrantV1={id,operation:"run",jobs:1,cpu:1,reservedMegabytes:2048,admittedAtMs:now,observedMegabytes:null,pid:process.pid,bootAtMs:hostBootAtV1(directory,now),quiet:true,arrivedAtMs:now};
 writeFileSync(join(directory,"grants",id+".json"),JSON.stringify(row));
 const grown=()=>{const{quiet:_,...rest}=row;writeFileSync(join(directory,"grants",id+".json"),JSON.stringify({...rest,jobs:6,cpu:6,reservedMegabytes:3500,upgradedAtMs:Date.now(),resumedAtMs:Date.now()}));};
 const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined};
 const nested=()=>withHostCapacityV1({directory,policy,request:quietHostRequestV1(request,{quiet:true,megabytes:2048,rampMilliseconds:60_000,samples:1,shareBasisPoints:10000}),upgrade:request,control,environment:{SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:id,SEMIO_HOST_CAPACITY_PLAN:JSON.stringify(plan)},write:()=>undefined},async handle=>({inherited:handle.inherited,jobs:handle.jobs,cpu:handle.cpu,plan:handle.plan.jobs,quiet:JSON.parse(readFileSync(join(directory,"grants",id+".json"),"utf8")).quiet}));
 expect(await nested(),"at once, the full request's jobs, the grant still quiet").toEqual({inherited:true,jobs:8,cpu:8,plan:8,quiet:true});
 grown();
 expect(await nested(),"after the growth: the grant's jobs").toEqual({inherited:true,jobs:6,cpu:6,plan:6,quiet:undefined});
});

test("A growth admitted with fewer jobs than it asked for says so in English and German and keeps the memory of its whole request",async()=>{
 for(const language of ["en","de"] as const){
  const a=running("courseaaa1",1,2861),b=running("printaaaa1",2,3649),c=quiet("dashaaaaa1",3,Date.now()-1000),request=full(c.id,404,410),lines:string[]=[],grants=[a,b,c];let state=c,polls=0;
  const ledger={takeLock:()=>true,releaseLock:()=>undefined,holdsLock:()=>true,writeJson:()=>undefined,remove:()=>undefined,queuePath:()=>join(fresh(),"waiter.json"),readGrants:()=>grants.map(grant=>grant.id===state.id?state:grant),readWaiters:()=>[] as HostWaiterV1[],sample:async()=>({...machine(),atMs:Date.now()})};
  const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined};
  const outcome=await upgradeHostGrantV1({ledger:ledger as never,directory:fresh(),policy:fast,request:{...request,minimumJobs:request.preferredJobs},control,state:()=>state,publish:next=>{state=next;},requested:()=>++polls<20,stopped:()=>false,now:Date.now,pid:c.pid,language,write:line=>lines.push(line),session:{address:c.session!,weight:1}});
  expect(outcome,language).toBe("upgraded");
  expect([state.jobs,state.reservedMegabytes,state.quiet],language).toEqual([1,404+8*410,undefined]);
  const template=formatHostCapacityV1(language,"upgradedReduced",{operation:"run",from:1502,to:404+8*410,jobs:1,preferred:8,seconds:0,available:15490,floor:hostFloorMegabytesV1(fast,{totalMegabytes:16384})});
  expect(lines,language).toEqual([template]);
 }
});

test("audit8 followup-150 replay: a growth short of cores sets aside the cores of the small requests served after it, so the one-job request without an agent session is still admitted",()=>{
 const world=(followupFixture as unknown as {decisions:{name:string;host:HostSampleV1;ledger:HostGrantV1[];queue:HostWaiterV1[];nowMs:number}[]}).decisions.find(row=>row.name==="followup-150")!;
 const on=(followupFixture as unknown as {policy:{on:{quietSoftStopGrants:number;quietFollowUpMilliseconds:number;envelopes:readonly object[]}}}).policy.on,followupPolicy=readHostCapacityPolicyV1({...decisionFixture.policy,memory:{...decisionFixture.policy.memory,quietSoftStopGrants:on.quietSoftStopGrants},estimates:{...decisionFixture.policy.estimates,quietFollowUpMilliseconds:on.quietFollowUpMilliseconds},envelopes:on.envelopes}),decisions=evaluateHostCapacityV1(followupPolicy,world.host,world.ledger,world.queue,world.nowMs).decisions;
 expect(verdict(decisions,"blg4rt9kiwov")).toEqual(["admit",1,900,"fits"]);
 expect(verdict(decisions,"l34n3kfv0hah").slice(0,2)).toEqual(["admit",5]);
});

test("A growth short of cores leaves the core of a one-job request without an agent session that is served after it",()=>{
 const a=running("printaaaa1",1,3649,2900),small:HostGrantV1={...running("dashaaaaa2",4,1500),jobs:2,cpu:2},b=quiet("courseaaa1",2,960_000),later:HostWaiterV1={request:{id:"terminal01",operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:900,perJobMegabytes:0,cpuPerJob:1},arrivedAtMs:999_000,pid:77};
 const decisions=evaluateHostCapacityV1(policy,{...machine({load:2}),atMs:1_000_000},[a,small,b],[growth(b,full(b.id,279,384)),later],1_000_000).decisions;
 expect([verdict(decisions,b.id),verdict(decisions,later.request.id)],"8 + 2 + 1 cores booked: the growth takes 5 of the 6 left, the later one-job request the last").toEqual([["admit",5,279+8*384,"reduced"],["admit",1,900,"fits"]]);
 const crowded=evaluateHostCapacityV1(policy,{...machine({load:2}),atMs:1_000_000},[a,running("dashaaaaa1",4,2000),b],[growth(b,full(b.id,279,384)),later],1_000_000).decisions;
 expect([verdict(crowded,b.id),verdict(crowded,later.request.id)],"16 cores booked: the growth runs with the session rule's one job, the session-less request keeps the ceiling").toEqual([["admit",1,279+8*384,"reduced"],["wait",0,0,"load"]]);
});

test("The first job of an agent session stops past the cores ceiling plus one core per core of the host",()=>{
 const grants=[1,2,3].map(owner=>running(`runaaaaa${owner}`,owner,1500)),fresh:HostWaiterV1={request:{...full("dashaaaaa1",245,327),priority:"batch"},arrivedAtMs:905_000,pid:9,session:session(9),weight:1};
 expect(verdict(evaluateHostCapacityV1(policy,machine({load:2}),grants.slice(0,2),[fresh],906_000).decisions,fresh.request.id)[0],"16 of 16 booked: the session still gets its first job").toBe("admit");
 expect(verdict(evaluateHostCapacityV1(policy,machine({load:2}),grants,[fresh],906_000).decisions,fresh.request.id),"24 booked = 16 + 8 cores: it waits").toEqual(["wait",0,0,"load"]);
});

test("An upgrade that waited for all its jobs past the bypass window takes the jobs that fit beside a busy peer instead of waiting for the peer's whole run, and says so in English and German",async()=>{
 const floor=hostFloorMegabytesV1(fast,{totalMegabytes:16384}),peerUnrealized=5486-3600,available=floor+peerUnrealized+4000,request=full("courseaaa9",404,700),held=200,spare=available-floor-peerUnrealized,expected=Math.min(7,Math.floor((spare+held-request.fixedMegabytes-request.perJobMegabytes)/request.perJobMegabytes)),reserved=request.fixedMegabytes+(expected+1)*request.perJobMegabytes;
 expect(expected,"the world is built so that the whole request does not fit, a part of it does, and one job of memory stays spare").toBe(4);
 for(const language of ["en","de"] as const)for(const narrowing of [true,false]){
  const peer={...running("printaaaa1",2,5486,3600),rampMilliseconds:10_000_000},other=quiet("dashaaaaa1",3,500_000),mine:HostGrantV1={...quiet("courseaaa9",1,500_000),reservedMegabytes:1502,observedMegabytes:held},lines:string[]=[],grants=[peer,other,mine];let state=mine,polls=0,clock=905_000;
  const ledger={takeLock:()=>true,releaseLock:()=>undefined,holdsLock:()=>true,writeJson:()=>undefined,remove:()=>undefined,queuePath:()=>join(fresh(),"waiter.json"),readGrants:()=>grants.map(grant=>grant.id===state.id?state:grant),readWaiters:()=>[] as HostWaiterV1[],sample:async()=>({...machine({availableMegabytes:available,load:2}),atMs:clock})};
  const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined};
  const tuned=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:10,bypassMilliseconds:narrowing?decisionFixture.policy.wait.bypassMilliseconds:3_600_000}});
  const outcome=await upgradeHostGrantV1({ledger:ledger as never,directory:fresh(),policy:tuned,request:{...request,minimumJobs:request.preferredJobs},control,state:()=>state,publish:next=>{state=next;},requested:()=>++polls<(narrowing?300:40),stopped:()=>false,now:()=>(clock+=1000),pid:mine.pid,language,write:line=>lines.push(line),session:{address:mine.session!,weight:1}});
  if(!narrowing){expect([outcome,state.jobs,state.quiet],`${language} without the bypass window: it keeps waiting for the peer`).toEqual(["withdrawn",1,true]);continue;}
  expect(outcome,language).toBe("upgraded");
  expect([state.jobs,state.reservedMegabytes,state.quiet],`${language}: the jobs that fit and the memory of those jobs`).toEqual([expected,reserved,undefined]);
  expect(lines.at(-1),language).toMatch(language==="en"?new RegExp("from 1502 MB to "+reserved+" MB with "+expected+" of 8 job\\(s\\) after \\d+s"):new RegExp("von 1502 MB auf "+reserved+" MB mit "+expected+" von 8 Job\\(s\\) vergrößert"));
 }
});
