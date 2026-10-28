import {expect,test} from "bun:test";
import Ajv from "ajv";
import {describeHostReasonV1,formatHostCapacityErrorV1,hostCapacityErrorV1,evaluateHostCapacityV1,formatHostCapacityV1,hostFloorMegabytesV1,isHostCapacityV1,orderHostWaitersV1,planHostCapacityV1,readHostCapacityPolicyV1,requireHostCapacityV1,resolveHostCapacityLanguageV1,sizeHostRequestV1,type HostDefinitionV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {randomHostCaseV1,randomRampCaseV1,seededRandomV1,sqliteHostOracleV1,type HostCaseV1} from "./🔮️sqlite/🟦️.ts";

const policy=readHostCapacityPolicyV1(fixture.policy);
const run=(item:Omit<HostCaseV1,"expect">)=>evaluateHostCapacityV1(policy,item.host,item.ledger,item.queue,item.nowMs);

test("The host capacity schema agrees with Ajv, an independent validator, on every definition and every probe",()=>{
 const ajv=new Ajv({strict:true,allowUnionTypes:true});
 const probes:Record<string,{valid:unknown[];invalid:unknown[]}>={
  Policy:{valid:[fixture.policy],invalid:[{...fixture.policy,enabled:"yes"},{...fixture.policy,extra:1},{...fixture.policy,operations:[...fixture.policy.operations,fixture.policy.operations[0]]},{...fixture.policy,kinds:{...fixture.policy.kinds,test:{cpu:0,megabytes:1}}},{...fixture.policy,files:{ceilingBasisPoints:0}},{...fixture.policy,attaches:{maximum:0}},{...fixture.policy,attaches:undefined}]},
  Host:{valid:[fixture.cases[0]!.host,{...fixture.cases[0]!.host,load:null,swapUsedMegabytes:null,swapOutMegabytesPerSecond:null,openFiles:null,openFilesMaximum:null,attachedImages:null}],invalid:[{...fixture.cases[0]!.host,pressure:"high"},{...fixture.cases[0]!.host,platform:"freebsd"},{...fixture.cases[0]!.host,cores:0},{...fixture.cases[0]!.host,availableMegabytes:-1},{...fixture.cases[0]!.host,load:"1"},{...fixture.cases[0]!.host,openFiles:-1},{...fixture.cases[0]!.host,openFilesMaximum:0},{...fixture.cases[0]!.host,attachedImages:"3"},{...fixture.cases[0]!.host,attachedImages:undefined}]},
  Request:{valid:[fixture.cases[0]!.queue[0]!.request],invalid:[{...fixture.cases[0]!.queue[0]!.request,id:"short"},{...fixture.cases[0]!.queue[0]!.request,minimumJobs:0},{...fixture.cases[0]!.queue[0]!.request,priority:"urgent"},{...fixture.cases[0]!.queue[0]!.request,operation:"Run"}]},
  Waiter:{valid:[fixture.cases[0]!.queue[0]],invalid:[{...fixture.cases[0]!.queue[0],pid:0},{request:fixture.cases[0]!.queue[0]!.request,arrivedAtMs:0}]},
  Grant:{valid:[fixture.cases[2]!.ledger[0],{...fixture.cases[2]!.ledger[0],lentSinceMs:5,resumedAtMs:9}],invalid:[{...fixture.cases[2]!.ledger[0],lentSinceMs:-1},{...fixture.cases[2]!.ledger[0],resumedAtMs:"now"},{...fixture.cases[2]!.ledger[0],jobs:0},{...fixture.cases[2]!.ledger[0],observedMegabytes:-1},{...fixture.cases[2]!.ledger[0],id:"x"}]},
  Decision:{valid:[fixture.cases[0]!.expect.decisions[0]],invalid:[{...fixture.cases[0]!.expect.decisions[0],reason:"because"},{...fixture.cases[0]!.expect.decisions[0],verdict:"maybe"}]},
  Evaluation:{valid:[fixture.cases[0]!.expect],invalid:[{...fixture.cases[0]!.expect,binding:"other"}]},
  Plan:{valid:[{version:1,id:"aaaaaaaa1",jobs:2,cpu:2,memoryMegabytes:4096,fixedMegabytes:1536,floorMegabytes:6553}],invalid:[{version:1,id:"aaaaaaaa1",jobs:0,cpu:2,memoryMegabytes:4096,fixedMegabytes:1536,floorMegabytes:6553}]},
 };
 for(const [name,probe] of Object.entries(probes)){
  const oracle=ajv.compile({definitions:schema.definitions,$ref:`#/definitions/${name}`});
  for(const value of probe.valid){expect(oracle(value),`${name} valid ${JSON.stringify(value).slice(0,80)}`).toBe(true);expect(isHostCapacityV1(name as HostDefinitionV1,value),name).toBe(true);}
  for(const value of probe.invalid){expect(oracle(value),`${name} invalid ${JSON.stringify(value).slice(0,80)}`).toBe(false);expect(isHostCapacityV1(name as HostDefinitionV1,value),name).toBe(false);}
 }
 expect(ajv.validateSchema(schema)).toBe(true);
});

test("Every authored hand case gives exactly the decisions written down by hand",()=>{
 for(const item of fixture.cases){expect(run(item as never),item.name).toEqual(item.expect as never);}
});

test("The decision equals a recursive SQLite oracle on every hand case and on a seeded corpus, and the stored corpus is the oracle's output",()=>{
 for(const item of fixture.cases)expect(sqliteHostOracleV1(policy,item as never),item.name).toEqual(item.expect as never);
 const random=seededRandomV1(fixture.seed);
 fixture.corpus.forEach((item,index)=>{
  const fresh=randomHostCaseV1(random,index);
  expect(JSON.parse(JSON.stringify(fresh)),`corpus ${index} regenerates`).toEqual({name:item.name,host:item.host,ledger:item.ledger,queue:item.queue,nowMs:item.nowMs});
  expect(sqliteHostOracleV1(policy,fresh),`corpus ${index} oracle`).toEqual(item.expect as never);
  expect(JSON.parse(JSON.stringify(run(item as never))),`corpus ${index} implementation`).toEqual(item.expect as never);
 });
 const verdicts=new Set(fixture.corpus.flatMap(item=>item.expect.decisions.map(row=>row.reason)));
 for(const reason of ["fits","reduced","override","memory","queue-order","pressure-critical","pressure-warn","files","attaches","load","swap","swap-activity"])expect(verdicts.has(reason as never),`corpus reaches ${reason}`).toBe(true);
});

test("Ramp rules: a grant counts its reservation for its own time to peak and what it holds when that is more, and only maximumRamping shrunk reservations may ramp at once; the hand cases, the seeded corpus and the SQLite oracle agree",()=>{
 for(const item of fixture.rampCases){
  expect(run(item as never),item.name).toEqual(item.expect as never);
  expect(sqliteHostOracleV1(policy,item as never),`${item.name} oracle`).toEqual(item.expect as never);
 }
 const random=seededRandomV1(20261026);
 fixture.rampCorpus.forEach((item,index)=>{
  const fresh=randomRampCaseV1(random,index);
  expect(JSON.parse(JSON.stringify(fresh)),`ramp corpus ${index} regenerates`).toEqual({name:item.name,host:item.host,ledger:item.ledger,queue:item.queue,nowMs:item.nowMs});
  expect(sqliteHostOracleV1(policy,fresh),`ramp corpus ${index} oracle`).toEqual(item.expect as never);
  expect(JSON.parse(JSON.stringify(run(item as never))),`ramp corpus ${index} implementation`).toEqual(item.expect as never);
 });
 const reasons=new Set(fixture.rampCorpus.flatMap(item=>item.expect.decisions.map(row=>row.reason)));
 for(const reason of ["fits","memory","ramp","queue-order"])expect(reasons.has(reason as never),`ramp corpus reaches ${reason}`).toBe(true);
 for(const item of [...fixture.rampCases,...fixture.rampCorpus]){
  const live=item.ledger.filter(grant=>(grant as {lentSinceMs?:number}).lentSinceMs===undefined),ramping=live.filter(grant=>(grant as {rampSlot?:boolean}).rampSlot===true&&item.nowMs-((grant as {resumedAtMs?:number}).resumedAtMs??grant.admittedAtMs)<((grant as {rampMilliseconds?:number}).rampMilliseconds??policy.memory.rampMilliseconds)).length;
  const admittedSlots=item.expect.decisions.filter(row=>row.verdict==="admit"&&(item.queue.find(waiter=>waiter.request.id===row.id)!.request as {rampSlot?:boolean}).rampSlot===true).length;
  expect(ramping+admittedSlots,`${item.name}: shrunk reservations ramping at once`).toBeLessThanOrEqual(Math.max(policy.memory.maximumRamping,ramping));
  for(const row of item.expect.decisions.filter(candidate=>candidate.reason==="ramp"))expect((item.queue.find(waiter=>waiter.request.id===row.id)!.request as {rampSlot?:boolean}).rampSlot,`${item.name}: only shrunk requests wait for a slot`).toBe(true);
 }
});

test("Evaluation is pure: frozen inputs are not touched and order of arrival never changes the outcome of a permuted queue",()=>{
 const deep=<T>(value:T):T=>{if(value&&typeof value==="object"){Object.values(value).forEach(deep);Object.freeze(value);}return value;};
 for(const item of fixture.corpus.slice(0,60)){
  const frozen=deep(JSON.parse(JSON.stringify(item))),first=evaluateHostCapacityV1(policy,frozen.host as never,frozen.ledger as never,frozen.queue as never,frozen.nowMs);
  const shuffled=[...frozen.queue].reverse(),second=evaluateHostCapacityV1(policy,frozen.host as never,frozen.ledger as never,shuffled as never,frozen.nowMs);
  expect(second).toEqual(first);
 }
});

test("Invariants hold on the whole corpus: no admission under a hard block, a barrier that, once raised, holds back everything behind it, and reservations stay inside the budget",()=>{
 for(const item of fixture.corpus){
  const result=run(item as never),admitted=result.decisions.filter(row=>row.verdict==="admit"),floor=hostFloorMegabytesV1(policy,item.host as never),cap=Math.max(0,item.host.totalMegabytes-floor);
  if(item.host.pressure==="critical")expect(admitted.length,item.name).toBe(0);
  const barrier=result.decisions.findIndex(row=>row.reason==="queue-order");
  if(barrier>=0)for(const row of result.decisions.slice(barrier))expect(row,item.name).toMatchObject({verdict:"wait",jobs:0,reservedMegabytes:0});
  const committed=item.ledger.reduce((sum,grant)=>sum+grant.reservedMegabytes,0)+admitted.reduce((sum,row)=>sum+row.reservedMegabytes,0);
  const overridden=admitted.some(row=>row.reason==="override");
  if(!overridden&&item.ledger.length>0)expect(committed<=Math.max(cap,item.ledger.reduce((sum,grant)=>sum+grant.reservedMegabytes,0)),item.name).toBe(true);
  for(const row of admitted){const waiter=item.queue.find(candidate=>candidate.request.id===row.id)!;expect(row.jobs>=waiter.request.minimumJobs&&row.jobs<=waiter.request.preferredJobs,item.name).toBe(true);}
 }
});

test("Free memory cannot fall below the floor: admitting every request the decision lets through keeps projected headroom non-negative",()=>{
 for(const item of fixture.corpus){
  const result=run(item as never),admitted=result.decisions.filter(row=>row.verdict==="admit"&&row.reason!=="override"),spent=admitted.reduce((sum,row)=>sum+row.reservedMegabytes,0);
  expect(result.headroomMegabytes-spent>=0||admitted.length===0,item.name).toBe(true);
 }
});

test("Waiters are served interactive first, then by arrival, then by id",()=>{
 const waiters:HostWaiterV1[]=[{request:{...(fixture.cases[0]!.queue[0]!.request),id:"bbbbbbbb1"} as never,arrivedAtMs:5,pid:1},{request:{...(fixture.cases[0]!.queue[0]!.request),id:"aaaaaaaa1"} as never,arrivedAtMs:5,pid:1},{request:{...(fixture.cases[0]!.queue[0]!.request),id:"cccccccc1",priority:"interactive"} as never,arrivedAtMs:9,pid:1},{request:{...(fixture.cases[0]!.queue[0]!.request),id:"dddddddd1"} as never,arrivedAtMs:1,pid:1}];
 expect(orderHostWaitersV1(waiters).map(row=>row.request.id)).toEqual(["cccccccc1","dddddddd1","aaaaaaaa1","bbbbbbbb1"]);
});

test("The floor follows the host between the authored bounds",()=>{
 expect([2048,8192,16384,32768,65536,262144].map(total=>hostFloorMegabytesV1(policy,{totalMegabytes:total}))).toEqual([2048,4096,4096,6553,8192,8192]);
});

test("Operations are sized from the authored catalogue and unknown operations pass ungated",()=>{
 const run=sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"run",preferredJobs:8,kinds:["check"]});
 expect(run).toMatchObject({minimumJobs:1,preferredJobs:8,fixedMegabytes:1536,perJobMegabytes:4096,cpuPerJob:1,priority:"batch"});
 expect(sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"run",preferredJobs:4,kinds:["test"]})).toMatchObject({perJobMegabytes:2048});
 expect(sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"cargo",preferredJobs:6,kinds:["check"]})).toMatchObject({perJobMegabytes:2048,preferredJobs:6});
 expect(sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"show"})).toMatchObject({minimumJobs:1,preferredJobs:1,fixedMegabytes:5120,perJobMegabytes:0,cpuPerJob:2});
 expect(sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"help"})).toBeUndefined();
 expect(sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"watch",preferredJobs:2})).toMatchObject({priority:"interactive"});
});

test("A plan carries the admitted jobs into tool-neutral cores and megabytes and refuses jobs outside the request",()=>{
 const request=sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"run",preferredJobs:8,kinds:["test"]})!;
 expect(planHostCapacityV1(policy,{totalMegabytes:32768},request,5)).toEqual({version:1,id:"aaaaaaaa1",jobs:5,cpu:5,memoryMegabytes:10240,fixedMegabytes:1536,floorMegabytes:6553});
 expect(()=>planHostCapacityV1(policy,{totalMegabytes:32768},request,9)).toThrow();
 expect(()=>planHostCapacityV1(policy,{totalMegabytes:32768},request,0)).toThrow();
});

test("Policy ordering rules are enforced beyond the schema",()=>{
 expect(()=>readHostCapacityPolicyV1({...fixture.policy,memory:{...fixture.policy.memory,floorMinimumMegabytes:9000}})).toThrow();
 expect(()=>readHostCapacityPolicyV1({...fixture.policy,pressure:{...fixture.policy.pressure,psiWarnPercent:50}})).toThrow();
 expect(()=>requireHostCapacityV1("Grant",{} as HostGrantV1)).toThrow();
});

test("Messages exist for every reason in English and German and fill every placeholder",()=>{
 const values={files:8000,images:10,total:32768,pressure:"normal",swap:0,grants:1,waiters:0,seconds:12,operation:"run",reason:"x",need:4096,spare:100,floor:6553,load:3.5,running:2,ahead:1,jobs:3,preferred:8,reserved:7000,available:9000,switch:"SEMIO_HOST_CAPACITY"};
 for(const language of ["en","de"] as const)for(const name of ["waiting","admitted","reduced","override","bypassed","status"] as const)expect(formatHostCapacityV1(language,name,values)).not.toMatch(/\{[a-z]+\}/u);
 expect(formatHostCapacityV1("de","waiting",values)).toContain("wartet seit 12 s");
 expect(formatHostCapacityV1("en","waiting",values)).toContain("waiting 12s");
 for(const language of ["en","de"] as const)for(const reason of ["memory","swap","swap-activity","pressure-critical","pressure-warn","files","attaches","load","queue-order"] as const)expect(describeHostReasonV1(language,reason).length,`${language} ${reason}`).toBeGreaterThan(8);
 const none=()=>[] as string[];
 expect(resolveHostCapacityLanguageV1({LANG:"de_DE.UTF-8"},none)).toBe("de");
 expect(resolveHostCapacityLanguageV1({SEMIO_LANGUAGE:"en",LANG:"de_DE.UTF-8"},none)).toBe("en");
 expect(()=>resolveHostCapacityLanguageV1({},none)).toThrow("No language selected");
});

test("Every refusal has an English and a German message with the same placeholders and the raised error speaks the chosen language",()=>{
 const english=schema.definitions.Messages.const.en.errors,german=schema.definitions.Messages.const.de.errors,names=Object.keys(english) as (keyof typeof english)[];
 expect(Object.keys(german).sort()).toEqual([...names].sort());
 const placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 const values={definition:"Grant",detail:"missing id",platform:"plan9"};
 for(const name of names){
  expect(placeholders(german[name]),name).toEqual(placeholders(english[name]));
  for(const language of ["en","de"] as const)expect(formatHostCapacityErrorV1(language,name,values),`${language} ${name}`).not.toMatch(/\{[a-z]+\}/u);
 }
 expect(hostCapacityErrorV1("planJobs",{},{SEMIO_LANGUAGE:"de"}).message).toBe(german.planJobs);
 expect(hostCapacityErrorV1("planJobs",{},{SEMIO_LANGUAGE:"en",LANG:"de_DE.UTF-8"}).message).toBe(english.planJobs);
 expect(hostCapacityErrorV1("definitionRequired",{definition:"Grant",detail:"missing id"},{}).message).toBe("Closed host capacity Grant required: missing id");
});

const fair=(availableMegabytes:number,load:number|null=0):HostSampleV1=>({version:1,atMs:0,platform:"darwin",cores:8,totalMegabytes:32768,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load,openFiles:null,openFilesMaximum:null,attachedImages:null});
const asker=(id:string,megabytes:number,arrivedAtMs:number,priority:"batch"|"interactive"="batch"):HostWaiterV1=>({arrivedAtMs,pid:100,request:{id:id.padEnd(8,"x"),operation:"command",priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1}});
const holder=(id:string,observed:number,admittedAtMs:number):HostGrantV1=>({id:id.padEnd(8,"g"),operation:"command",jobs:1,cpu:1,reservedMegabytes:observed,admittedAtMs,observedMegabytes:observed,pid:200,bootAtMs:0});
const verdictOf=(evaluation:ReturnType<typeof evaluateHostCapacityV1>,id:string)=>evaluation.decisions.find(row=>row.id===id.padEnd(8,"x"))!;
const wait=policy.wait;

test("A head held back only by foreign pressure never blocks smaller requests, however long it has waited",()=>{
 const sample=fair(10000),queue=[asker("big",5200,0),asker("small",500,1000)],late=wait.bypassMilliseconds*100;
 for(const now of [1000,late]){
  const evaluation=evaluateHostCapacityV1(policy,sample,[],queue,now);
  expect(verdictOf(evaluation,"big")).toMatchObject({verdict:"wait",reason:"memory"});
  expect(verdictOf(evaluation,"small"),String(now)).toMatchObject({verdict:"admit"});
 }
});

test("When finishing our own grants would admit the head it is overtaken at most maximumBypasses times and for bypassMilliseconds, then it is a barrier",()=>{
 const sample=fair(10000),running=[holder("run",4000,0)],big=asker("big",5200,5);
 const smalls=Array.from({length:wait.maximumBypasses+3},(_,index)=>asker("s"+index,100,10+index));
 const early=evaluateHostCapacityV1(policy,sample,running,[big,...smalls],1000);
 expect(early.decisions.filter(row=>row.verdict==="admit").length).toBe(wait.maximumBypasses);
 expect(early.decisions.filter(row=>row.reason==="queue-order").length).toBe(3);
 const late=evaluateHostCapacityV1(policy,sample,running,[big,asker("small",100,wait.bypassMilliseconds)],wait.bypassMilliseconds+10);
 expect(verdictOf(late,"small")).toMatchObject({verdict:"wait",reason:"queue-order"});
});

test("Grants admitted since the head arrived count as overtakes",()=>{
 const sample=fair(8000),since=Array.from({length:wait.maximumBypasses},(_,index)=>holder("r"+index,1000,500)),before=since.map(row=>({...row,admittedAtMs:0}));
 const queue=[asker("big",5000,10),asker("small",500,11)];
 expect(verdictOf(evaluateHostCapacityV1(policy,sample,since,queue,1000),"small")).toMatchObject({verdict:"wait",reason:"queue-order"});
 expect(verdictOf(evaluateHostCapacityV1(policy,sample,before,queue,1000),"small")).toMatchObject({verdict:"admit"});
});

test("Starvation bound: in a stream of small requests a drainable large request is admitted within the bypass time plus one small duration",()=>{
 const smallMs=20000,sample=(running:HostGrantV1[])=>fair(17000-running.reduce((sum,row)=>sum+(row.observedMegabytes??0),0));
 let running:HostGrantV1[]=[],queue:HostWaiterV1[]=[asker("big",9000,0)],admittedAt:number|undefined,serial=0;
 for(let now=0;now<=300000&&admittedAt===undefined;now+=1000){
  running=running.filter(row=>now-row.admittedAtMs<smallMs);
  queue.push(asker("s"+serial++,2500,now));
  for(const decision of evaluateHostCapacityV1(policy,sample(running),running,queue,now).decisions)if(decision.verdict==="admit"){
   const entry=queue.find(row=>row.request.id===decision.id)!;
   if(entry.request.fixedMegabytes===9000)admittedAt=now;else running.push(holder(entry.request.id,entry.request.fixedMegabytes,now));
   queue=queue.filter(row=>row!==entry);
  }
 }
 expect(admittedAt).toBeDefined();expect(admittedAt!).toBeLessThanOrEqual(wait.bypassMilliseconds+smallMs+2000);
});

test("A batch request that has waited agingMilliseconds ranks as interactive, a younger one does not",()=>{
 const old=asker("old",3000,0),fresh=asker("fresh",3000,wait.agingMilliseconds,"interactive");
 expect(evaluateHostCapacityV1(policy,fair(10000),[],[fresh,old],wait.agingMilliseconds).decisions.map(row=>[row.id,row.verdict])).toEqual([["oldxxxxx","admit"],["freshxxx","wait"]]);
 expect(evaluateHostCapacityV1(policy,fair(10000),[],[fresh,old],wait.agingMilliseconds-1).decisions.map(row=>[row.id,row.verdict])).toEqual([["freshxxx","admit"],["oldxxxxx","wait"]]);
});

test("Lending equals the SQLite oracle on a seeded corpus in which grants are lent or resumed, and a lent grant counts for nobody",()=>{
 const random=seededRandomV1(20261023),pick=random;
 for(let index=0;index<300;index++){
  const base=randomHostCaseV1(random,index),ledger=base.ledger.map(grant=>{const roll=pick();return roll<0.4?{...grant,lentSinceMs:grant.admittedAtMs+1000}:roll<0.6?{...grant,resumedAtMs:base.nowMs-Math.floor(pick()*400000)}:grant;}),item={...base,ledger};
  const expected=sqliteHostOracleV1(policy,item);
  expect(JSON.parse(JSON.stringify(run(item))),`lending corpus ${index}`).toEqual(JSON.parse(JSON.stringify(expected)));
  const without={...item,ledger:ledger.filter(grant=>grant.lentSinceMs===undefined)},sameRoom=run(without);
  expect(run(item).unrealizedMegabytes,"a lent grant adds no unrealized memory").toBe(sameRoom.unrealizedMegabytes);
  expect(run(item).headroomMegabytes,"a lent grant takes no room").toBe(sameRoom.headroomMegabytes);
  expect(run(item).loadRoom,"a lent grant holds no cores").toBe(sameRoom.loadRoom);
 }
});

test("The status line of a lent grant says so in English and German and fills every placeholder",()=>{
 for(const language of ["en","de"] as const){
  const text=formatHostCapacityV1(language,"grantLent",{id:"gggggggg1",operation:"run",cpu:8,reserved:3584,observed:270,pid:42,seconds:600});
  expect(text).not.toContain("{");expect(text).toContain("3584");expect(text).toContain("270");expect(text).toContain("gggggggg1");
 }
 expect(formatHostCapacityV1("de","grantLent",{id:"a",operation:"run",cpu:1,reserved:1,observed:1,pid:1,seconds:1})).toContain("verliehen");
});
