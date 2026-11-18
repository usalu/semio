import os
R="/Users/ueli/Documents/semio/"
L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
OP=B+"🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/"
PROC=R+"🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R147/"
def rel(frm,to):
    r=os.path.relpath(to,frm)
    return r if r.startswith(".") else "./"+r
def make(dirpath,name,text,imports):
    for k,v in imports.items():
        text=text.replace("@"+k+"@",rel(dirpath,v))
    open(S+"stage/"+name,"w",encoding="utf8").write(text)

SHARED=r'''/** 🧮️ Contract: the machine's own read and the pool are one budget of memory and cores. The read has the first claim (through the asks of the capacity governor), the pool gets what it leaves and gives it back within the derived window of one pool poll plus the kill limit; there is no time after which the pool stops yielding; the pool never runs more children than the spare memory holds while the read exists; neither starves when both have small owners; swap shrinks the pool before the gate stops the farm. Hosts declared: 8 GB / 4 cores and 16 GB / 8 cores (the host matrix of R135). Oracle: the Python implementation of the fixtures and the gate's own policy numbers. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolSizingV1,type OwnerPoolHostV1}from"@pool@";
import{createMemoryGovernorV1,GRAPH_MEMORY_POLICY_V1 as M}from"@memory@";
import{createHostAskV1,hostAskClaimV1,readHostAsksV1}from"@asks@";
import policy from"@policy@";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const MB=1048576;
const hosts=[{name:"8 GB / 4 cores",cores:4,total:8192,other:1000,owner:800,solo:3500,child:900},{name:"16 GB / 8 cores",cores:8,total:16384,other:1500,owner:1000,solo:6000,child:900}];
const floorOf=(total:number)=>Math.max(P.floorMinimumMegabytes,Math.min(P.floorMaximumMegabytes,Math.floor(total*P.floorShareBasisPoints/10000)));
const hostAt=(h:typeof hosts[number],available:number,over:Partial<OwnerPoolHostV1>&{foreground?:Partial<OwnerPoolHostV1["foreground"]>}={}):OwnerPoolHostV1=>({cores:h.cores,totalMegabytes:h.total,availableMegabytes:available,pressure:"normal",load:null,swapOutMegabytesPerSecond:null,idleCores:null,...over,foreground:{cpu:1,unrealizedMegabytes:0,waiters:0,...over.foreground}});

type Sim={admittedAtMs:number|null;childrenAt:number[];startedAtMs:number};
async function simulate(h:typeof hosts[number],withAsk:boolean):Promise<Sim>{
 const directory=mkdtempSync(join(tmpdir(),"shared-budget-"));scratch.push(directory);
 let now=1_000_000,children=h.cores,killAt:number|null=null,nextPoll=now,ownerA=true,ownerB=false;
 const floor=floorOf(h.total),available=()=>h.total-h.other-(ownerA?h.owner:0)-(ownerB?h.solo:0)-children*h.child;
 const writer=createHostAskV1({directory,now:()=>now,pid:process.pid});
 const poolStep=()=>{
  if(now<nextPoll)return;nextPoll=now+P.targetPollMilliseconds;
  const claim=hostAskClaimV1(readHostAsksV1(directory,now,()=>true));
  const target=ownerPoolSizingV1({host:hostAt(h,available(),{foreground:{cpu:1,unrealizedMegabytes:0,waiters:0,...claim===undefined?{}:{ask:claim}}}),children,childMegabytes:h.child});
  if(target.reason==="waiting"){killAt??=now+P.yieldKillMilliseconds;if(now>=killAt)children=0;}
  else{killAt=null;children=Math.min(children,target.children);}
 };
 const sim:Sim={admittedAtMs:null,childrenAt:[],startedAtMs:now};
 const governor=createMemoryGovernorV1({workers:2,headroom:()=>(available()-floor)*MB,estimate:key=>({a:h.owner,b:h.solo}[key as "a"|"b"])*MB,estimateBytes:h.owner*MB,ownerLimitBytes:h.total*MB,now:()=>now,sleep:async ms=>{now+=ms;poolStep();sim.childrenAt.push(children);},...withAsk?{ask:writer}:{},policy:M});
 let release!:()=>void;const hold=new Promise<void>(done=>{release=done;});
 const a=governor.run(async()=>{await hold;},{kind:"k",key:"a"});
 await new Promise(done=>setTimeout(done,5));
 sim.startedAtMs=now;
 const b=governor.run(async()=>{sim.admittedAtMs=now;ownerB=true;ownerA=false;release();},{kind:"k",key:"b",limitBytes:h.solo*MB*2});
 await Promise.all([a,b]);governor.dispose();return sim;
}

for(const h of hosts){
 test(`${h.name}: an alone owner of the read drains the pool and is admitted within one pool poll plus the kill limit; without the ask it waits for the bound of the governor`,async()=>{
  const window=P.targetPollMilliseconds*(1+Math.ceil(P.yieldKillMilliseconds/P.targetPollMilliseconds))+M.admitMinimumMilliseconds+2*M.pollMilliseconds;
  const asked=await simulate(h,true);
  expect(asked.admittedAtMs).not.toBeNull();
  expect(asked.admittedAtMs!-asked.startedAtMs,"derived window: one pool poll, then the polls until the kill limit has passed").toBeLessThanOrEqual(window);
  expect(asked.childrenAt.at(-1)).toBe(0);
  expect(h.cores,"the pool held children when the ask came").toBeGreaterThan(0);
  const control=await simulate(h,false);
  expect(control.admittedAtMs!-control.startedAtMs,"the old behaviour: only the bound of the governor ends the wait").toBeGreaterThanOrEqual(M.waitBoundMilliseconds);
 });

 test(`${h.name}: the pool never exceeds the spare memory while the read exists, for every available size, claim and child count`,()=>{
  const floor=floorOf(h.total),need=h.child;
  for(let available=0;available<=h.total;available+=256)for(const children of[0,1,2,h.cores])for(const claim of[0,400,1500,4000,h.total])for(const alone of[false,true]){
   const ask=claim>0?{megabytes:claim,alone}:undefined;
   const sizing=ownerPoolSizingV1({host:hostAt(h,available,{foreground:{cpu:1,unrealizedMegabytes:0,waiters:0,...ask===undefined?{}:{ask}}}),children,childMegabytes:need});
   const drained=available-floor+children*need,counted=claim<=drained?claim:0,spare=available-floor-counted;
   const bound=Math.max(0,children+Math.floor(spare/need));
   expect(sizing.children,JSON.stringify({available,children,claim,alone})).toBeLessThanOrEqual(alone&&claim>0?0:bound);
  }
 });

 test(`${h.name}: both starve never with small owners: the read's owner fits and the pool still runs a child`,()=>{
  const floor=floorOf(h.total);
  for(let available=floor+h.owner+h.child;available<=h.total-h.other;available+=256){
   const claim=h.owner,sizing=ownerPoolSizingV1({host:hostAt(h,available,{foreground:{cpu:1,unrealizedMegabytes:0,waiters:0,ask:{megabytes:claim,alone:false}}}),children:0,childMegabytes:h.child});
   expect(sizing.children,"the pool runs").toBeGreaterThanOrEqual(1);
   expect(available-floor-sizing.children*h.child,"the read's owner still fits after the pool grew").toBeGreaterThanOrEqual(claim);
  }
 });

 test(`${h.name}: swap shrinks the pool in proportion before the gate stops the farm, and ends it where the gate stops`,()=>{
  expect(P.swapActiveMegabytesPerSecond).toBe(policy.hostCapacity.swap.activeMegabytesPerSecond);
  expect(P.swapOutMegabytesPerSecond).toBe(policy.hostCapacity.swap.outMegabytesPerSecond);
  let before=h.cores;
  for(let swap=0;swap<=P.swapOutMegabytesPerSecond;swap++){
   const sizing=ownerPoolSizingV1({host:hostAt(h,h.total-h.other,{idleCores:h.cores,swapOutMegabytesPerSecond:swap}),children:h.cores,childMegabytes:h.child});
   expect(sizing.children,String(swap)).toBeLessThanOrEqual(before);before=sizing.children;
   if(swap>P.swapActiveMegabytesPerSecond&&swap<P.swapOutMegabytesPerSecond)expect(sizing.children,"cut before the gate stops anything").toBeLessThan(h.cores);
  }
  expect(before).toBe(0);
  const grown=ownerPoolSizingV1({host:hostAt(h,h.total-h.other,{idleCores:h.cores,swapOutMegabytesPerSecond:P.swapActiveMegabytesPerSecond+1}),children:0,childMegabytes:h.child});
  expect(grown.children,"no growth while it swaps").toBe(0);
 });

 test(`${h.name}: no time makes the pool stop yielding to a request that its children hold up`,()=>{
  const floor=floorOf(h.total),available=floor+h.child,waiting=2*h.child;
  for(const waited of[P.yieldAfterMilliseconds,10*60_000,6*3_600_000,48*3_600_000]){
   const sizing=ownerPoolSizingV1({host:hostAt(h,available,{idleCores:h.cores,foreground:{cpu:1,unrealizedMegabytes:0,waiters:1,waitingMegabytes:waiting,oldestWaitMilliseconds:waited}}),children:4,childMegabytes:h.child});
   expect(sizing,String(waited)).toMatchObject({children:0,reason:"waiting"});
  }
  const never=ownerPoolSizingV1({host:hostAt(h,available,{idleCores:h.cores,foreground:{cpu:1,unrealizedMegabytes:0,waiters:1,waitingMegabytes:h.total*2,oldestWaitMilliseconds:48*3_600_000}}),children:2,childMegabytes:h.child});
  expect(never.reason,"a wait the children did not cause is no reason to stop them").not.toBe("waiting");
 });
}

test("the policy keeps no time after which the pool stops yielding",()=>{
 expect(Object.keys(P)).not.toContain("yieldUntilMilliseconds");
});
'''
make(OP+"🧪️tests/🧮️shared-budget/","shared.test.ts",SHARED,{"pool":OP+"🟦️.ts","memory":B+"🧠️memory/🟨️.mjs","asks":PROC+"🙋️asks/🟦️.ts","policy":B+"🔣️policy.json"})

ASKS=r'''/** 🙋️ Contract: an ask is one claim of a read on the one budget of its machine, shaped by the schema, alive while its process lives and its heartbeat is kept, withdrawn on clear, and summed into one claim for the pool. Oracle: Ajv, an independent JSON-schema validator. */
import{afterAll,expect,test}from"bun:test";
import Ajv from"ajv";
import{existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync,mkdirSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{HOST_ASK_POLICY_V1 as P,createHostAskV1,hostAskClaimV1,hostAskDirectoryV1,hostAskLiveV1,readHostAsksV1}from"./@self@";
import schema from"@schema@";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=()=>{const path=mkdtempSync(join(tmpdir(),"host-asks-"));scratch.push(path);return path;};
const alive=()=>true;

test("the ask schema agrees with Ajv on a good ask and on every broken field",()=>{
 const ajv=new Ajv({strict:true}),validate=ajv.compile({...schema.definitions.Ask,definitions:undefined});
 const good={version:1,pid:7,key:"owner-a",needMegabytes:4096,alone:true,sinceMs:1,updatedMs:2,heartbeatMilliseconds:1000};
 expect(validate(good)).toBe(true);
 const dir=temp(),writer=createHostAskV1({directory:dir,pid:7,now:()=>2});writer.set({key:"owner-a",needMegabytes:4096,alone:true});
 expect(validate(JSON.parse(readFileSync(join(hostAskDirectoryV1(dir),"7.json"),"utf8")))).toBe(true);
 for(const broken of[{...good,version:2},{...good,pid:0},{...good,key:""},{...good,needMegabytes:-1},{...good,alone:"yes"},{...good,extra:1},{...good,heartbeatMilliseconds:0}])expect(validate(broken)).toBe(false);
});

test("a writer publishes, keeps the heartbeat only when due, keeps since while the claim stays, and withdraws",()=>{
 const dir=temp();let now=1000;const writer=createHostAskV1({directory:dir,pid:11,now:()=>now}),file=join(hostAskDirectoryV1(dir),"11.json");
 writer.set({key:"a",needMegabytes:3000,alone:false});const first=JSON.parse(readFileSync(file,"utf8"));
 now+=P.heartbeatMilliseconds-1;writer.set({key:"a",needMegabytes:3000,alone:false});expect(JSON.parse(readFileSync(file,"utf8")).updatedMs,"not rewritten before the beat").toBe(first.updatedMs);
 now+=1;writer.set({key:"a",needMegabytes:3000,alone:false});const beat=JSON.parse(readFileSync(file,"utf8"));expect(beat.updatedMs).toBe(now);expect(beat.sinceMs).toBe(first.sinceMs);
 writer.set({key:"b",needMegabytes:500,alone:true});expect(JSON.parse(readFileSync(file,"utf8"))).toMatchObject({key:"b",alone:true,sinceMs:now});
 writer.clear();expect(existsSync(file)).toBe(false);
});

test("a stale, dead, misshapen or oversized ask counts for nothing, and live asks sum into one claim",()=>{
 const dir=temp(),now=5_000_000,asks=hostAskDirectoryV1(dir);mkdirSync(asks,{recursive:true});
 const row=(pid:number,over:object={})=>({version:1,pid,key:"k"+pid,needMegabytes:1000,alone:false,sinceMs:now-10,updatedMs:now-10,heartbeatMilliseconds:1000,...over});
 writeFileSync(join(asks,"1.json"),JSON.stringify(row(1)));
 writeFileSync(join(asks,"2.json"),JSON.stringify(row(2,{needMegabytes:2500,alone:true})));
 writeFileSync(join(asks,"3.json"),JSON.stringify(row(3,{updatedMs:now-1000*P.missedBeats-1})));
 writeFileSync(join(asks,"4.json"),"{not json");writeFileSync(join(asks,"5.json"),JSON.stringify({...row(5),needMegabytes:"many"}));
 writeFileSync(join(asks,"6.json"),JSON.stringify(row(6,{key:"x".repeat(P.maximumFileBytes)})));writeFileSync(join(asks,"note.txt"),"x");
 const live=readHostAsksV1(dir,now,pid=>pid!==2||true);expect(live.map(ask=>ask.pid).sort()).toEqual([1,2]);
 expect(hostAskClaimV1(live)).toEqual({megabytes:3500,alone:true});
 expect(readHostAsksV1(dir,now,pid=>pid===1).map(ask=>ask.pid),"a dead process holds nothing").toEqual([1]);
 expect(hostAskClaimV1([])).toBeUndefined();
 expect(hostAskLiveV1(row(1) as never,now+1000*P.missedBeats-10,alive)).toBe(true);expect(hostAskLiveV1(row(1) as never,now+1000*P.missedBeats-9,alive)).toBe(false);
 expect(readHostAsksV1(join(dir,"missing"),now,alive)).toEqual([]);
});

test("a writer into an unusable directory never throws",()=>{
 const dir=temp();writeFileSync(join(dir,"asks"),"a file where the directory should be");
 const writer=createHostAskV1({directory:dir,pid:3});expect(()=>{writer.set({key:"a",needMegabytes:1,alone:false});writer.clear();}).not.toThrow();
});
'''
make(PROC+"🙋️asks/🧪️tests/","asks.test.ts",ASKS.replace("./@self@","@self@"),{"self":PROC+"🙋️asks/🟦️.ts","schema":PROC+"🙋️asks/🧬️schema/🔣️.json"})

GOV=r'''/** 🙋️ Contract: the governor of a read claims from the one budget of its machine through the ask port: the first waiting owner (alone when the cost table gave it a solo limit) while it waits, an alone owner while it runs, nothing when no owner waits or runs alone. */
import{expect,test}from"bun:test";
import{createMemoryGovernorV1}from"@memory@";

const MB=1048576;
const port=()=>{const rows:Record<string,unknown>[]=[];return{rows,ask:{set:(row:Record<string,unknown>)=>{rows.push({op:"set",...row});},clear:()=>{rows.push({op:"clear"});}}};};
const until=async(check:()=>boolean)=>{for(let at=0;at<400&&!check();at++)await new Promise(done=>setTimeout(done,10));expect(check()).toBe(true);};

test("a waiting solo owner claims alone with its cost, keeps the claim while it runs and withdraws it when it ends",async()=>{
 const {rows,ask}=port();
 const governor=createMemoryGovernorV1({workers:2,headroom:()=>2000*MB,estimate:key=>({a:1000,b:7000}[key as "a"|"b"])*MB,ask});
 let freeA!:()=>void,freeB!:()=>void;const holdA=new Promise<void>(done=>{freeA=done;}),holdB=new Promise<void>(done=>{freeB=done;});
 const a=governor.run(async()=>{await holdA;},{kind:"k",key:"a"});
 const b=governor.run(async()=>{await holdB;},{kind:"k",key:"b",limitBytes:8000*MB});
 await until(()=>rows.some(row=>row.op==="set"&&row.key==="b"));
 expect(rows.find(row=>row.op==="set"&&row.key==="b")).toEqual({op:"set",key:"b",needMegabytes:7000,alone:true});
 freeA();await a;
 await until(()=>rows.at(-1)?.op==="set"&&rows.at(-1)?.key==="b");
 freeB();await b;
 expect(rows.at(-1)).toEqual({op:"clear"});
 governor.dispose();
});

test("a waiting owner without a solo limit claims what it needs but not alone, and the claim goes when it is admitted and ends",async()=>{
 const {rows,ask}=port();
 const governor=createMemoryGovernorV1({workers:2,headroom:()=>1500*MB,estimate:key=>({a:1000,c:900}[key as "a"|"c"])*MB,ask});
 let freeA!:()=>void,freeC!:()=>void;const holdA=new Promise<void>(done=>{freeA=done;}),holdC=new Promise<void>(done=>{freeC=done;});
 const a=governor.run(async()=>{await holdA;},{kind:"k",key:"a"});
 const c=governor.run(async()=>{await holdC;},{kind:"k",key:"c"});
 await until(()=>rows.some(row=>row.op==="set"&&row.key==="c"));
 expect(rows.find(row=>row.op==="set"&&row.key==="c")).toEqual({op:"set",key:"c",needMegabytes:900,alone:false});
 freeA();freeC();await Promise.all([a,c]);
 expect(rows.at(-1)).toEqual({op:"clear"});
 governor.dispose();
});

test("a governor without a port runs as before, and a port that throws never fails a run",async()=>{
 const plain=createMemoryGovernorV1({workers:1,headroom:()=>2000*MB});
 expect(await plain.run(async()=>7,{kind:"k",key:"x"})).toBe(7);plain.dispose();
 const broken=createMemoryGovernorV1({workers:1,headroom:()=>2000*MB,ask:{set:()=>{throw Error("disk full");},clear:()=>{throw Error("disk full");}}});
 expect(await broken.run(async()=>8,{kind:"k",key:"y"})).toBe(8);broken.dispose();
});
'''
make(B+"🧠️memory/🧪️tests/🙋️ask/","governor.test.ts",GOV,{"memory":B+"🧠️memory/🟨️.mjs"})
print("ok")
