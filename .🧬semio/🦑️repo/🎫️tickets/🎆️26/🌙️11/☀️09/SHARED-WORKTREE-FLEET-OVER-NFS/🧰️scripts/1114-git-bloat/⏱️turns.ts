import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,truncateSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {widen,settleHeap} from "../📇️book/🟦️.ts";
import {closeReplicaV1,openReplicaV1,publishOnceV1,seedV1} from "../🟦️.ts";

const scratch:string[]=[],watchdogHeartbeatMilliseconds=1000;
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

function turns(){
 let last=performance.now(),longest=0,count=0,active=true;
 const tick=():void=>{if(!active)return;const now=performance.now();longest=Math.max(longest,now-last);last=now;count++;setImmediate(tick);};
 setImmediate(tick);
 return {stop:()=>{active=false;return {longest,count};}};
}

test("publishing a large sparse file never holds the event loop for longer than the watchdog's heartbeat interval",async()=>{
 const root=mkdtempSync(join(tmpdir(),"replica-turns-"));
 scratch.push(root);
 const worktree=join(root,"tree"),store=join(root,"store");
 mkdirSync(worktree);mkdirSync(store);
 writeFileSync(join(worktree,"a.txt"),"a\n");
 const options={worktree,storeRoot:store,originId:"g1-server",groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0};
 await seedV1(options);
 const replica=await openReplicaV1(options,()=>{});
 try{
  const size=1.5*2**30;
  writeFileSync(join(worktree,"big.bin"),"");truncateSync(join(worktree,"big.bin"),size);
  const meter=turns(),result=await publishOnceV1(replica),measured=meter.stop();
  expect(result.published).toBe(1);
  expect(result.bytes).toBe(size);
  expect(measured.count).toBeGreaterThan(100);
  expect(measured.longest).toBeLessThan(watchdogHeartbeatMilliseconds/2);
 }finally{await closeReplicaV1(replica);}
},180_000);

test("forced full collections are rate limited: thousands of settle requests and growths of large tables in one turn cost at most one collection",async()=>{
 const holder=globalThis as unknown as {Bun:{gc:(force:boolean)=>void}},original=holder.Bun.gc;
 let calls=0;
 holder.Bun.gc=(force:boolean)=>{calls++;original.call(holder.Bun,force);};
 try{
  await new Promise(done=>setTimeout(done,1100));
  const before=calls;
  for(let index=0;index<5000;index++)settleHeap();
  let array=new Uint8Array(1<<20);
  for(let index=0;index<40;index++)array=widen(array,array.length+(3<<20));
  await Promise.resolve();await new Promise(done=>setImmediate(done));
  expect(calls-before).toBeLessThanOrEqual(1);
 }finally{holder.Bun.gc=original;}
});
