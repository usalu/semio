/** 🧎️ Laws of `releaseAfterTurnV1`: the order turn, release, collection; the collection also after a failing release; the runtime liveness of what the store of a finished async run holds (WeakRef is the oracle, not our code); the graph host releases through it. */
import{test,expect}from"bun:test";
import{AsyncLocalStorage}from"node:async_hooks";
import{readFileSync}from"node:fs";
import{releaseAfterTurnV1}from"../../🟨️.mjs";

const graphSource=readFileSync(new URL("../../../🕸️graph/🟦️.ts",import.meta.url),"utf8");

test("the release runs after one turn and the collection comes last",async()=>{
 const events:string[]=[];
 await releaseAfterTurnV1(()=>{events.push("release");},{ports:{collect:()=>{events.push("collect");}},turn:async()=>{events.push("turn");}});
 expect(events).toEqual(["turn","release","collect"]);
});

test("an asynchronous release is awaited before the collection",async()=>{
 const events:string[]=[];
 await releaseAfterTurnV1(async()=>{await new Promise(resolve=>setTimeout(resolve,5));events.push("release");},{ports:{collect:()=>{events.push("collect");}},turn:async()=>{events.push("turn");}});
 expect(events).toEqual(["turn","release","collect"]);
});

test("[adversarial] a failing release is collected after and its error surfaces",async()=>{
 const events:string[]=[];
 await expect(releaseAfterTurnV1(()=>{throw Error("release failed");},{ports:{collect:()=>{events.push("collect");}},turn:async()=>{events.push("turn");}})).rejects.toThrow("release failed");
 expect(events).toEqual(["turn","collect"]);
});

test("the turn is a real turn of the event loop: a timer due now runs before the release",async()=>{
 const events:string[]=[];
 setTimeout(()=>events.push("timer"),0);
 await releaseAfterTurnV1(()=>{events.push("release");},{ports:{collect:()=>{}}});
 expect(events).toEqual(["timer","release"]);
});

test("what the store of a finished async run holds is gone after the release",async()=>{
 const als=new AsyncLocalStorage<Map<number,object>>(),refs:WeakRef<object>[]=[];
 const work=async()=>als.run(new Map<number,object>(),async()=>{
  const store=als.getStore()!,big={payload:new Array(20000).fill("x")};
  refs.push(new WeakRef(big));store.set(1,big);
  await Promise.all([1,2,3].map(async n=>{await Promise.resolve();const local={n,big};refs.push(new WeakRef(local));store.set(n+1,local);await new Promise(resolve=>setImmediate(resolve));return local;}));
  return 1;
 });
 await work();
 await releaseAfterTurnV1(()=>{});
 expect(refs.length).toBe(4);
 expect(refs.map(ref=>ref.deref()===undefined)).toEqual([true,true,true,true]);
});

test("the graph host releases through it, so the collection never runs in the tick that ended the inference",()=>{
 const release=/release:\(\)=>releaseAfterTurnV1\(async\(\)=>\{[^\n]*\}\),\n/u.exec(graphSource);
 expect(release).not.toBeNull();
 expect(release![0]).not.toContain("createMemoryPortsV1");
 expect(release![0]).toContain("releaseInferenceMemoryV1");
 expect(release![0]).toContain("releaseInputMemoryV1");
 expect(release![0]).toContain("resetClosureEntriesV1");
 expect(release![0]).toContain("graphStringInternerV1.clear()");
});
