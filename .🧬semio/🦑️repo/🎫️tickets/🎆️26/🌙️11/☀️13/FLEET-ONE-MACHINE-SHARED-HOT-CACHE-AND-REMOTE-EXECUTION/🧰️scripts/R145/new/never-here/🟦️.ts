/** 🐘️ Contract: an owner is refused up front only by what this machine measured (a recorded cost), never by a declared seed, and a read that infers owners in child processes judges that cost as the physical footprint of a child against the alone cap of a child (the cap the farm itself enforces), not against the managed bytes of the parent; the measured peak of a child is recorded in the unit of the cost table. Measured 2026-11-17 with the declared 512 MB heap of a child: s-dev-composition 858 MB of footprint, a seed said 27952.
 * @see ../../🟨️.mjs
 * @see ../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{graphOwnerEnforcementV1,ownerMeasuredManagedBytesV1}from"../../🟨️.mjs";
import{farmAloneCapV1}from"../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import{recordOwnerCostFileV1}from"../../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";

const MB=1048576,scratch:string[]=[],HEAVY=["@semio-tech/s-dev-composition","@semio-tech/repo-lib","@semio-tech/framework-rs","workspace"];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const withTotal=async<T>(megabytes:number,work:()=>Promise<T>)=>{const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=String(megabytes);try{return await work();}finally{if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}};
const enforce=(root:string,host:number)=>withTotal(host,()=>graphOwnerEnforcementV1(root,process.env,1024*MB,undefined,()=>host*MB*3/4,()=>host));

test("a declared seed never refuses an owner up front: the heavy seeds pass on a 16 GB and on an 8 GB machine, in process and in a child",async()=>{
 const root=mkdtempSync(join(tmpdir(),"never-seed-"));scratch.push(root);
 for(const host of [16384,8192]){
  const enforcement=await enforce(root,host);
  expect(enforcement.neverHere!(HEAVY),`in process on ${host} MB`).toEqual([]);
  expect(enforcement.neverHere!(HEAVY,farmAloneCapV1(host)),`in a child on ${host} MB`).toEqual([]);
 }
});

test("a recorded cost is judged as footprint against the alone cap of a child: 3000 MB of footprint fits 8 GB and 16 GB, 6000 MB fits neither; in process the managed solo limit still decides",async()=>{
 const root=mkdtempSync(join(tmpdir(),"never-recorded-"));scratch.push(root);
 const path=join(root,".cache/f/graph-costs/owner-costs.3.json"),now=Date.now();
 recordOwnerCostFileV1(path,{name:"@semio-tech/fits",kind:"project",megabytes:1500,seconds:90,rows:10,stopped:false,atMs:now});
 recordOwnerCostFileV1(path,{name:"@semio-tech/over",kind:"project",megabytes:3000,seconds:90,rows:10,stopped:true,atMs:now});
 for(const host of [16384,8192]){
  const enforcement=await enforce(root,host),cap=farmAloneCapV1(host);
  expect(enforcement.neverHere!(["@semio-tech/fits","@semio-tech/over","@semio-tech/unknown"],cap),`child on ${host} MB`).toEqual(["@semio-tech/over"]);
 }
 const small=await enforce(root,8192);
 expect(small.neverHere!(["@semio-tech/fits"])).toEqual(["@semio-tech/fits"]);
});

test("the peak a child reports is its physical footprint; the cost table holds managed megabytes, so it is recorded divided by the footprint factor",()=>{
 expect(ownerMeasuredManagedBytesV1(undefined,2)).toBeUndefined();
 expect(ownerMeasuredManagedBytesV1(858*MB,2)).toBe(429*MB);
 expect(ownerMeasuredManagedBytesV1(858*MB,undefined)).toBe(858*MB);
});
