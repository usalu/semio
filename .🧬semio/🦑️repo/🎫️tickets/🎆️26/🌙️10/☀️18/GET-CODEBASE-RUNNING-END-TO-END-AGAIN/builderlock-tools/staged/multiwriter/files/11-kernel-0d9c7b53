/** 🔑️ Contract: beneath a real issued lease control, the waits of the slice custody and of the prefetch claims go through the REAL queued lease kernel, which admits only the control's own signal: a waiter behind an overlapping claim, and a prefetch that joins a running peer claim, wait and move on when the holder releases (no derived abort signal ever reaches the kernel), and cancelling the control ends a waiter promptly and leaves no claim behind. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireQueuedResourceLease} from "@semio-tech/process/leases";
import {createIssuedResourceControlV1} from "@semio-tech/process/routing";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {createClaimRegistryV1,createQueuedClaimLeasePortV1,waitAnyLeaseV1} from "../../../🧩️claims/🟨️.mjs";
import {createSliceCustodyV1} from "../../🟨️.mjs";

const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const settled=async(promise:Promise<unknown>,ms:number)=>Promise.race([promise.then(()=>true,(error:unknown)=>String((error as Error)?.message??error)),sleep(ms).then(()=>false)]);
declareBuildTestEnvelopeV1({owner:"test:graph-slice-custody-kernel",maximumElapsedMilliseconds:300000,capabilities:{}});
type Original=Parameters<Parameters<typeof withBuildTestInvocationV1>[0]>[0];
/** 🎛️ One control issued beneath the real test invocation (as every repository command issues one per lease user) and the kernel-backed lease port bound to it; aborting `controller` cancels that control only. */
function issued(original:Original,directory:string,name:string){
 const controller=new AbortController(),control=createIssuedResourceControlV1(original,"test:custody-kernel:"+name,120000,controller.signal);
 return{controller,control,lease:createQueuedClaimLeasePortV1({acquire:acquireQueuedResourceLease as never,directory,control})};
}
const scratch=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"custody-kernel-")));roots.push(root);return root;};

test("an overlapping claim waits through the real kernel with the control's own signal and holds once the first is released",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),leases=join(root,"leases"),first=issued(original,leases,"c1"),second=issued(original,leases,"c2");
 const one=createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:first.lease}),two=createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:second.lease});
 const a=await one.hold({owners:["x","y"],signal:first.control.signal});
 const waiting=two.hold({owners:["y"],signal:second.control.signal});
 expect(await settled(waiting,1500)).toBe(false);
 const disjoint=await two.hold({owners:["z"],signal:second.control.signal});
 await a.release();
 const b=await waiting;expect(b.owners).toEqual(["y"]);
 await b.release();await disjoint.release();
}),60000);

test("the shared wait helper answers true when a held lease is released and false after its timeout, through the real kernel",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),leases=join(root,"leases"),holder=issued(original,leases,"c3"),waiter=issued(original,leases,"c4");
 const held=await holder.lease.tryHold("graph-custody:law:held");expect(held).toBeDefined();
 expect(await waitAnyLeaseV1(waiter.lease,["graph-custody:law:held"],{signal:waiter.control.signal,timeoutMs:300})).toBe(false);
 const later=waitAnyLeaseV1(waiter.lease,["graph-custody:law:held","graph-custody:law:other-held"],{signal:waiter.control.signal,timeoutMs:20000});
 const other=await holder.lease.tryHold("graph-custody:law:other-held");expect(other).toBeDefined();
 await sleep(100);held!.release();
 expect(await later).toBe(true);other!.release();
}),60000);

test("a prefetch that joins a running peer claim waits through the real kernel instead of failing",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),leases=join(root,"leases"),a=issued(original,leases,"c5"),b=issued(original,leases,"c6");let release!:()=>void;const gate=new Promise<void>(resolve=>{release=resolve;});
 const registry=(port:ReturnType<typeof issued>)=>createClaimRegistryV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:port.lease});
 const ran:string[]=[];
 const first=registry(a).prefetch({base:1,implementation:"a".repeat(64),wanted:["p"],slots:2,signal:a.control.signal,run:async owners=>{ran.push("A:"+owners.join(","));await gate;}});
 await sleep(300);
 const second=registry(b).prefetch({base:1,implementation:"a".repeat(64),wanted:["p"],slots:2,signal:b.control.signal,run:async owners=>{ran.push("B:"+owners.join(","));}});
 expect(await settled(second,800)).toBe(false);
 release();
 expect(await first).toEqual({computed:1,served:false});expect(await second).toEqual({computed:0,served:false});expect(ran).toEqual(["A:p"]);
}),60000);

test("cancelling the control ends a waiting claim promptly and leaves only the holder's claim",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),leases=join(root,"leases"),first=issued(original,leases,"c7"),second=issued(original,leases,"c8");
 const one=createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:first.lease}),two=createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:second.lease});
 const a=await one.hold({owners:["x"],signal:first.control.signal}),waiting=two.hold({owners:["x"],signal:second.control.signal});
 expect(await settled(waiting,500)).toBe(false);
 const started=Date.now();second.controller.abort(new Error("stop"));
 await expect(waiting).rejects.toThrow();expect(Date.now()-started).toBeLessThan(5000);
 expect((await one.list()).map(claim=>claim.token)).toEqual([a.token]);
 await a.release();
}),60000);
