import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {hostUpgradePathV1,newHostCapacityIdV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,requestHostCapacityUpgradeV1,withHostCapacityV1,type HostCapacityHandleV1,type HostCapacityOptionsV1,type HostGrantV1,type HostRequestV1,type HostRunV1,type HostSampleV1} from "../../../🟦️.ts";
import decisionFixture from "../../🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:600000},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hu-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const sized=(id:string,fixed:number,over:object={}):HostRequestV1=>({id,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:fixed,perJobMegabytes:0,cpuPerJob:1,...over});
const quietOf=(full:HostRequestV1):HostRequestV1=>({...full,fixedMegabytes:500,minimumJobs:1,preferredJobs:1,quiet:true,rampSlot:false,perJobMegabytes:0});
const options=(directory:string,request:HostRequestV1,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request,control:control(),environment:{},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),...over});
const environmentOf=(directory:string,handle:HostCapacityHandleV1)=>({SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:handle.id});
const grants=async(directory:string)=>(await readHostCapacityStatusV1({directory,policy,control:control(),sample:async()=>({host:host(),swappedOutMegabytes:0})}));
const holdBig=(directory:string)=>{let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});const done=withHostCapacityV1(options(directory,sized(newHostCapacityIdV1(),8000)),async()=>hold);return{release,done};};

test("A quiet grant answers at once beside a full machine and its handle carries the full envelope",async()=>{
 const directory=fresh(),big=holdBig(directory),lines:string[]=[];
 await sleep(40);
 const full=sized(newHostCapacityIdV1(),9000,{preferredJobs:3,perJobMegabytes:100,minimumJobs:1}),started=Date.now();
 let seen:HostCapacityHandleV1|undefined;
 await withHostCapacityV1(options(directory,quietOf(full),{upgrade:full,write:line=>lines.push(line)}),async handle=>{seen=handle;await sleep(30);});
 expect(Date.now()-started,"no queue wait").toBeLessThan(2000);
 expect(seen!.reservedMegabytes).toBe(500);expect(seen!.jobs).toBe(3);expect(seen!.plan.jobs).toBe(3);expect(seen!.plan.memoryMegabytes).toBe(300);
 expect(lines.at(-1)).toMatch(/^\[capacity\] admitted run: 1 job\(s\), 500 MB reserved/u);
 big.release();await big.done;
});

test("Work that has to run grows the quiet reservation in the fair queue, with progress in English and German, and the run starts only when it fits",async()=>{
 const directory=fresh(),big=holdBig(directory),english:string[]=[],german:string[]=[];
 await sleep(40);
 const full=sized(newHostCapacityIdV1(),9000);
 let outcome:string|undefined,upgradedAt=0,released=0;
 const run=withHostCapacityV1(options(directory,quietOf(full),{upgrade:full,write:line=>english.push(line),environment:{LANG:"en_US.UTF-8"}}),async handle=>{
  const waiting=requestHostCapacityUpgradeV1(environmentOf(directory,handle),control());
  await sleep(400);
  const status=await grants(directory);
  expect(status.grants.find(row=>row.id===handle.id)!.quiet,"still quiet while the machine is full").toBe(true);
  const queued=status.waiters.find(row=>row.request.id===handle.id);
  expect(queued?.upgrade).toBe(true);expect(queued!.arrivedAtMs,"keeps the arrival of the run").toBe(status.grants.find(row=>row.id===handle.id)!.arrivedAtMs as number);
  expect(existsSync(hostUpgradePathV1(directory,handle.id))).toBe(true);
  released=Date.now();big.release();
  outcome=await waiting;upgradedAt=Date.now();
  const after=(await grants(directory)).grants.find(row=>row.id===handle.id)!;
  expect(after.quiet).toBeUndefined();expect(after.reservedMegabytes).toBe(9000);expect(after.upgradedAtMs).toBeGreaterThan(0);
  expect((await grants(directory)).waiters).toEqual([]);
 });
 await run;await big.done;
 expect(outcome).toBe("upgraded");expect(upgradedAt-released).toBeLessThan(3000);
 expect(english.some(line=>/^\[capacity\] run: work has to run, waiting \d+s to grow the reservation from 500 MB to 9000 MB: not enough free memory/u.test(line))).toBe(true);
 expect(english.at(-1)).toMatch(/^\[capacity\] run: reservation grown from 500 MB to 9000 MB after \d+s \(1 job\(s\)/u);
 const big2=holdBig(directory);await sleep(40);
 const second=sized(newHostCapacityIdV1(),9000);
 await withHostCapacityV1(options(directory,quietOf(second),{upgrade:second,write:line=>german.push(line),environment:{LANG:"de_DE.UTF-8"}}),async handle=>{const waiting=requestHostCapacityUpgradeV1(environmentOf(directory,handle),control());await sleep(400);big2.release();await waiting;});
 await big2.done;
 expect(german.some(line=>/^\[capacity\] run: es muss Arbeit laufen, wartet seit \d+ s darauf, die Reservierung von 500 MB auf 9000 MB zu vergrößern: nicht genug freier Speicher/u.test(line))).toBe(true);
 expect(german.at(-1)).toMatch(/^\[capacity\] run: Reservierung nach \d+ s von 500 MB auf 9000 MB vergrößert/u);
},30000);

test("The upgrade is served in the order the run arrived: a request that came after the run, but before its upgrade, waits behind it",async()=>{
 const directory=fresh(),big=holdBig(directory);
 await sleep(40);
 const full=sized(newHostCapacityIdV1(),9000),order:string[]=[];
 let late:Promise<void>|undefined;
 const run=withHostCapacityV1(options(directory,quietOf(full),{upgrade:full}),async handle=>{
  late=withHostCapacityV1(options(directory,sized(newHostCapacityIdV1(),9000)),async()=>{order.push("late");await sleep(20);});
  await sleep(80);
  const waiting=requestHostCapacityUpgradeV1(environmentOf(directory,handle),control());
  await sleep(400);big.release();
  await waiting;order.push("upgraded");await sleep(50);
 });
 await run;await late;await big.done;
 expect(order,"the upgrade is admitted first although it asked later").toEqual(["upgraded","late"]);
},30000);

test("Cancelling the wait withdraws the request, keeps the small grant and says so",async()=>{
 const directory=fresh(),big=holdBig(directory),lines:string[]=[];
 await sleep(40);
 const full=sized(newHostCapacityIdV1(),9000);
 await withHostCapacityV1(options(directory,quietOf(full),{upgrade:full,write:line=>lines.push(line)}),async handle=>{
  const controller=new AbortController(),waiting=requestHostCapacityUpgradeV1(environmentOf(directory,handle),control(controller.signal));
  await sleep(400);
  expect((await grants(directory)).waiters.some(row=>row.upgrade===true)).toBe(true);
  controller.abort(Error("stop upgrading"));
  await expect(waiting).rejects.toThrow("stop upgrading");
  await sleep(400);
  const status=await grants(directory);
  expect(status.waiters.filter(row=>row.upgrade===true)).toEqual([]);expect(existsSync(hostUpgradePathV1(directory,handle.id))).toBe(false);
  expect(status.grants.find(row=>row.id===handle.id)).toMatchObject({quiet:true,reservedMegabytes:500});
 });
 expect(lines.some(line=>/^\[capacity\] run: the request to grow the reservation was withdrawn after \d+s; the run keeps its 500 MB/u.test(line))).toBe(true);
 big.release();await big.done;
 expect(readdirSync(join(directory,"grants"))).toEqual([]);expect(readdirSync(join(directory,"queue"))).toEqual([]);
},30000);

test("A run that asks for work on a full grant only records that it executed, and a quiet run that never asks is learned as quiet with the full jobs",async()=>{
 const directory=fresh(),learned:HostRunV1[]=[];
 const full=sized(newHostCapacityIdV1(),9000,{preferredJobs:2,perJobMegabytes:10});
 await withHostCapacityV1(options(directory,full,{upgrade:full,learn:run=>{learned.push(run);},observe:async()=>1200}),async handle=>{
  expect(await requestHostCapacityUpgradeV1(environmentOf(directory,handle),control())).toBe("held");
  await sleep(60);
 });
 const quiet=sized(newHostCapacityIdV1(),9000,{preferredJobs:2,perJobMegabytes:10});
 await withHostCapacityV1(options(directory,quietOf(quiet),{upgrade:quiet,learn:run=>{learned.push(run);},observe:async()=>900}),async()=>{await sleep(80);});
 const plain=sized(newHostCapacityIdV1(),9000);
 await withHostCapacityV1(options(directory,plain,{learn:run=>{learned.push(run);},observe:async()=>800}),async()=>{await sleep(60);});
 expect(learned.map(run=>[run.executed,run.jobs,run.peakMegabytes])).toEqual([[true,2,1200],[false,2,900],[undefined,1,800]]);
 expect(await requestHostCapacityUpgradeV1({},control())).toBe("absent");
 expect(await requestHostCapacityUpgradeV1({SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:"missing-grant-1"},control())).toBe("absent");
});

test("A nested invocation that inherits a quiet grant grows it before it runs its body",async()=>{
 const directory=fresh(),big=holdBig(directory);
 await sleep(40);
 const full=sized(newHostCapacityIdV1(),9000);let nestedRan=0;
 await withHostCapacityV1(options(directory,quietOf(full),{upgrade:full}),async handle=>{
  const nested=withHostCapacityV1(options(directory,sized(newHostCapacityIdV1(),9000),{environment:{...environmentOf(directory,handle),SEMIO_HOST_CAPACITY_PLAN:JSON.stringify(handle.plan)}}),async inherited=>{nestedRan++;expect(inherited.inherited).toBe(true);expect((await grants(directory)).grants.find(row=>row.id===handle.id)!.quiet).toBeUndefined();});
  await sleep(300);expect(nestedRan,"the body waits for the growth").toBe(0);
  big.release();await nested;expect(nestedRan).toBe(1);
 });
 await big.done;
},30000);

test("A failing quiet run is never parked for a restart and gives its small grant back",async()=>{
 const directory=fresh(),full=sized(newHostCapacityIdV1(),9000);let asked=0;
 await expect(withHostCapacityV1(options(directory,quietOf(full),{upgrade:full,park:()=>{asked++;return true;}}),async()=>{throw Error("body failed");})).rejects.toThrow("body failed");
 expect(asked).toBe(0);expect(readdirSync(join(directory,"grants"))).toEqual([]);
 const grantFile=readdirSync(join(directory,"grants"));expect(grantFile.some(name=>name.endsWith(".upgrade"))).toBe(false);void readFileSync;void ({} as HostGrantV1);
});

test("A quiet request without the full request it may grow into is refused in English and German, and a bypassed gate hands the full envelope",async()=>{
 const directory=fresh(),full=sized(newHostCapacityIdV1(),9000,{preferredJobs:3});
 await expect(withHostCapacityV1(options(directory,quietOf(full),{environment:{LANG:"en_US.UTF-8"}}),async()=>undefined)).rejects.toThrow("a quiet request needs the full request it may grow into, under the same id");
 await expect(withHostCapacityV1(options(directory,quietOf(full),{environment:{LANG:"de_DE.UTF-8"},upgrade:sized(newHostCapacityIdV1(),9000)}),async()=>undefined)).rejects.toThrow("eine ruhige Anfrage braucht die volle Anfrage, in die sie wachsen darf, unter derselben Kennung");
 await expect(withHostCapacityV1(options(directory,quietOf(full),{upgrade:quietOf(full)}),async()=>undefined)).rejects.toThrow("a quiet request needs the full request");
 const lines:string[]=[];
 await withHostCapacityV1(options(directory,quietOf(full),{upgrade:full,environment:{SEMIO_HOST_CAPACITY:"off",LANG:"en_US.UTF-8"},write:line=>lines.push(line)}),async handle=>{expect(handle.bypassed).toBe(true);expect(handle.jobs).toBe(3);expect(handle.plan.jobs).toBe(3);expect(handle.environment).toEqual({});});
 expect(lines.length).toBe(1);
 expect(existsSync(join(directory,"grants")),"a bypassed gate leaves no ledger").toBe(false);
});

test("A quiet grant whose process only waited in a queue is taken back from the lenders before it asks for work",async()=>{
 const directory=fresh(),lender=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...policy.wait,lendAfterMilliseconds:1000}});
 const full=sized(newHostCapacityIdV1(),9000);
 await withHostCapacityV1(options(directory,quietOf(full),{policy:lender,upgrade:full,waited:()=>10000,children:()=>[]}),async handle=>{
  await sleep(1500);
  expect((await grants(directory)).grants.find(row=>row.id===handle.id)!.lentSinceMs,"lent while it only waits").toBeGreaterThan(0);
  const waiting=requestHostCapacityUpgradeV1(environmentOf(directory,handle),control());
  expect(await waiting).toBe("upgraded");
  const row=(await grants(directory)).grants.find(row=>row.id===handle.id)!;
  expect(row.quiet).toBeUndefined();expect(row.reservedMegabytes).toBe(9000);expect(row.resumedAtMs).toBeGreaterThan(0);
 });
},30000);
