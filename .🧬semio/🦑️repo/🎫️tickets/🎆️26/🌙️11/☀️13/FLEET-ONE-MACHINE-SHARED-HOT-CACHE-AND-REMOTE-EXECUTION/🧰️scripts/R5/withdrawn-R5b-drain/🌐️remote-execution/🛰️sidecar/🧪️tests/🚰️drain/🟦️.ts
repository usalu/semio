/** 🚰️ Contract: a sidecar whose code changed finishes the work in flight before it restarts. `drainWhenIdleV1` calls its callback once, and only after the drain signal fired and nothing is in flight; without a drain signal, or after it was stopped, never. */
import {expect,test} from "bun:test";
import {drainWhenIdleV1} from "../../../🛰️agent/🟦️.ts";

const wait=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));

test("without a drain signal nothing is ever called",async()=>{
 let calls=0;const stop=drainWhenIdleV1(undefined,()=>0,()=>{calls++;},2);
 await wait(20);stop();expect(calls).toBe(0);
});

test("the drain waits for the work in flight and then calls once",async()=>{
 let running=2,calls=0;const drain=new AbortController();
 drainWhenIdleV1(drain.signal,()=>running,()=>{calls++;},2);
 await wait(15);expect(calls).toBe(0);
 drain.abort();await wait(20);expect(calls,"still two in flight").toBe(0);
 running=1;await wait(20);expect(calls).toBe(0);
 running=0;await wait(20);expect(calls).toBe(1);
 await wait(20);expect(calls,"called once").toBe(1);
});

test("a drain that fired before the watch started with nothing in flight calls at once",()=>{
 const drain=new AbortController();drain.abort();let calls=0;
 drainWhenIdleV1(drain.signal,()=>0,()=>{calls++;},50);
 expect(calls).toBe(1);
});

test("a stopped watch never calls",async()=>{
 let running=1,calls=0;const drain=new AbortController();
 const stop=drainWhenIdleV1(drain.signal,()=>running,()=>{calls++;},2);
 drain.abort();await wait(10);stop();running=0;await wait(20);expect(calls).toBe(0);
});

test("the busy probe is asked again at every check, so work that starts during the drain is waited for",async()=>{
 let asked=0,running=1,calls=0;const drain=new AbortController();
 drainWhenIdleV1(drain.signal,()=>{asked++;return running;},()=>{calls++;},2);
 drain.abort();await wait(20);running=0;await wait(20);
 expect(asked).toBeGreaterThan(2);expect(calls).toBe(1);
});
