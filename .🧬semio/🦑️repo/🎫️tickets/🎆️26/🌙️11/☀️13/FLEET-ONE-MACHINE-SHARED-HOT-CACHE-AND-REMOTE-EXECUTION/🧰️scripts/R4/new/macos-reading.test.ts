import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fleetReadStaleGenerationV1}from"../../../🤝️prefetch/🟦️.ts";
import{createHostAvailableBytesV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";

/** 🍏️ Contract: a 16 GB Mac worker reads its memory from `vm_stat` like the kernel does (free pages plus the cooling and cached ones), so the first plan line of an idle worker says "it can give about 3512 MB per project" and "192 of the others (48 per wave)" and names only measured heavy projects; a busy moment lowers the projects per wave and keeps the grant; a Mac that cannot run a wave asks the fleet for everything (a grant of 0 MB). Before: "226 too heavy ... about 0 MB per project ... 0 of the others (0 per wave)" on an idle worker.
 * @see ../../../🤝️prefetch/🟦️.ts */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const stat=(free:number,inactive:number,file:number)=>`Mach Virtual Memory Statistics: (page size of 16384 bytes)\nPages free:      ${free}.\nPages active:    200000.\nPages inactive:  ${inactive}.\nPages speculative: 5000.\nPages wired down: 150000.\nPages purgeable: 8000.\nFile-backed pages: ${file}.\nPages occupied by compressor: 40000.\n`;
const reading=(text:string)=>createHostAvailableBytesV1({platform:"darwin",run:()=>text,total:()=>16384*1048576,environment:{}})();
const request=(count:number,extra:readonly string[],roots:Record<string,string>)=>({generation:3,dirty:[...Array.from({length:count},(_,at)=>`p${String(at).padStart(5,"0")}`),...extra],roots,structural:false,implementation:"a".repeat(64)});
const invocation={control:{signal:new AbortController().signal}} as never;
async function run(text:string){
 const root=mkdtempSync(join(tmpdir(),"fleet-read-macos-"));scratch.push(root);
 const lines:string[]=[],write=process.stderr.write.bind(process.stderr);
 process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as never;
 const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES="16384";
 try{await fleetReadStaleGenerationV1({root,request:request(517,["workspace","@semio-tech/s-dev-composition"],{workspace:"."}),invocation,language:"en",environment:{PATH:process.env.PATH,HOME:root,SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"},available:()=>reading(text)});return{lines:lines.join(""),code:undefined as string|undefined};}
 catch(error){return{lines:lines.join(""),code:(error as {code?:string}).code};}
 finally{process.stderr.write=write;if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}
}

test("an idle 16 GB Mac (10 GB available): grant about 3512 MB per project, 192 inferred itself (48 per wave), two measured heavy projects named",async()=>{
 const result=await run(stat(15000,620000,330000));
 expect(result.code).toBe("unserved");
 expect(result.lines).toContain("2 too heavy for this machine");expect(result.lines).toContain("it can give about 3512 MB per project");expect(result.lines).toContain("it can infer 192 of the others (48 per wave)");
 expect(result.lines).toContain("@semio-tech/s-dev-composition (about 27952 MB)");expect(result.lines).not.toContain("about 0 MB");
});

test("a busy 16 GB Mac (6.7 GB available): fewer parallel inferences, the same grant",async()=>{
 const result=await run(stat(9000,420000,300000));
 expect(result.lines).toContain("it can give about 3512 MB per project");expect(result.lines).toContain("it can infer 96 of the others (24 per wave)");expect(result.lines).toContain("2 too heavy for this machine");
});

test("a 16 GB Mac that cannot run the smallest wave (1 GB available) has a grant of 0 MB and asks the fleet for everything",async()=>{
 const result=await run(stat(3000,60000,40000));
 expect(result.lines).toContain("it can give about 0 MB per project");expect(result.lines).toContain("it can infer 0 of the others (0 per wave)");
});
