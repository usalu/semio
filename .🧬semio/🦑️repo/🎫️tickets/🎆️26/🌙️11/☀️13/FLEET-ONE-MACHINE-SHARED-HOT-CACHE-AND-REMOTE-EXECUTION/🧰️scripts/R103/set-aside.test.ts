/** 🪂️ Contract: one owner above the alone cap of a worker process never fails the read of the machine: it is set aside with a plain reason in both languages, recorded so that the fleet sees what it needs, left to a machine that can hold it, and the read goes on with every other owner. Any other failure is not touched.
 * @see ../../🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{graphOwnerEnforcementV1,setAsideOverrunV1}from"../../🟨️.mjs";
import{GraphFarmOverrunErrorV1}from"../../🏊️project-farm/🏊️pool/🟨️.mjs";

const MB=1048576,scratch:string[]=[],source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8");
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const overrun=()=>new GraphFarmOverrunErrorV1({project:"@semio-tech/wfc-bitmap",megabytes:4358,capMegabytes:4200},{});
const withTotal=async<T>(megabytes:number,work:()=>Promise<T>)=>{const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=String(megabytes);try{return await work();}finally{if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}};

test("an overrun is set aside: never here, alone, recorded with what it needed, said in English and German; every other failure is left alone",()=>{
 const never=new Set<string>(),alone=new Set<string>(),rows:any[]=[],said:string[][]=[];
 const aside=setAsideOverrunV1({error:overrun(),never,alone,recorder:{record:(row:any)=>rows.push(row)},factor:1,say:(text:(language:string)=>string)=>said.push([text("en"),text("de")])});
 expect(aside).toBe(true);expect([...never]).toEqual(["@semio-tech/wfc-bitmap"]);expect([...alone]).toEqual(["@semio-tech/wfc-bitmap"]);
 expect(rows).toEqual([{key:"@semio-tech/wfc-bitmap",kind:"project",takenBytes:4358*MB,seconds:0,stopped:true}]);
 expect(said.length).toBe(1);
 for(const line of said[0]!){expect(line).toContain("@semio-tech/wfc-bitmap");expect(line).toContain("4358");expect(line).toContain("4200");}
 expect(said[0]![0]).not.toBe(said[0]![1]);
 expect(setAsideOverrunV1({error:Object.assign(Error("other"),{code:"x"}),never,alone,recorder:undefined,factor:1,say:()=>{throw Error("silent");}})).toBe(false);
 expect(never.size).toBe(1);
});

test("what the set-aside records is what the fleet and the next read see: the cost is known and the owner never fits this 16 GB machine",async()=>{
 const root=mkdtempSync(join(tmpdir(),"set-aside-"));scratch.push(root);
 const enforcement:any=await withTotal(16384,()=>graphOwnerEnforcementV1(root,process.env,1024*MB,()=>7,()=>4000*MB,()=>16384));
 expect(enforcement.known("@semio-tech/wfc-bitmap")).toBeUndefined();
 const factor=enforcement.cost.ownerCostPolicyV1.footprintFactor;
 setAsideOverrunV1({error:overrun(),never:new Set(),alone:new Set(),recorder:enforcement.recorder,factor,say:()=>{}});
 expect(enforcement.known("@semio-tech/wfc-bitmap")).toBe(4358);
 expect(await withTotal(16384,async()=>enforcement.neverHere(["@semio-tech/wfc-bitmap"]))).toEqual(["@semio-tech/wfc-bitmap"]);
});

test("the owner loop sets an overrun aside instead of failing, and the end of the read waits for the set-aside owners from the fleet",()=>{
 expect(source).toContain("const setAside=error=>");
 const loop=source.slice(source.indexOf("const loop=async end=>"),source.indexOf("const ticker="));
 expect(loop).toContain("inferProject(job,away)).catch(error=>{if(!setAside(error))throw error;});");
 expect(loop).toContain("catch(error){if(ownerTooHeavyOfV1(error)===undefined)throw error;}");
 expect(source).toMatch(/setAsideOverrunV1\(\{error,never,alone,recorder:enforcement\.recorder/u);
 expect(source.indexOf("await waitHeavy();")).toBeGreaterThan(source.indexOf("await loop(Infinity)"));
});
