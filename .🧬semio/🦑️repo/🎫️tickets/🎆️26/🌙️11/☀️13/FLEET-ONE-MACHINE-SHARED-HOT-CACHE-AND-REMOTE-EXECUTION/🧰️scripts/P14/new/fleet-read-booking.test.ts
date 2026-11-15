import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fleetReadStaleGenerationV1}from"../../../🤝️prefetch/🟦️.ts";

/** 🌍️ Contract: the Linux worker of the fleet (16 GB, 9987 MB available, asked from a slice booked with 1 GB) takes the decision by what it can give, not by the booking: it infers a wave of unknown projects itself, names only measured heavy ones, and says in plain text what it can infer. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const request=(count:number,extra:readonly string[]=[],roots:Record<string,string>={})=>({generation:3,dirty:[...Array.from({length:count},(_,at)=>`p${String(at).padStart(5,"0")}`),...extra],roots,structural:false,implementation:"a".repeat(64)});
const invocation={control:{signal:new AbortController().signal}} as never;
async function run(count:number,environment:Record<string,string>,available:number|undefined,extra:readonly string[]=[],roots:Record<string,string>={}){
 const root=mkdtempSync(join(tmpdir(),"fleet-read-booking-"));scratch.push(root);
 const lines:string[]=[],write=process.stderr.write.bind(process.stderr);
 process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as never;
 const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=environment.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;
 try{await fleetReadStaleGenerationV1({root,request:request(count,extra,roots),invocation,language:"en",environment:{PATH:process.env.PATH,HOME:root,...environment},available:()=>available});return{lines,error:undefined};}
 catch(error){return{lines,error:error as {name:string;key:string;code:string}};}
 finally{process.stderr.write=write;if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}
}
const MB=1048576,LINUX={SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"15848"};

test("517 stale projects on the Linux worker: a small booking of the asking slice no longer turns them all into refused ones; the machine can infer a wave of 48 four times itself",async()=>{
 const booked=await run(517,{...LINUX,SEMIO_GRAPH_MEMORY_MB:"1024"},9987*MB,["workspace","@semio-tech/s-dev-composition"],{workspace:"."});
 const text=booked.lines.join("");
 expect(booked.error?.code).toBe("unserved");
 expect(text).toContain("2 too heavy for this machine");expect(text).toContain("workspace (about 6200 MB)");expect(text).toContain("@semio-tech/s-dev-composition (about 27952 MB)");
 expect(text).toContain("it can infer 192 of the others (48 per wave)");
 expect(text).toContain("posix-shared");
});

test("without a measured heavy project 517 stale projects are light: the plan says the machine can infer 192 of them and asks the fleet for the rest",async()=>{
 const result=await run(517,{...LINUX,SEMIO_GRAPH_MEMORY_MB:"1024"},9987*MB);
 const text=result.lines.join("");
 expect(result.error?.code).toBe("unserved");expect(text).toContain("this machine can infer 192 (48 per wave)");expect(text).not.toContain("too heavy");
});

test("a few unknown projects stay local on that machine whatever the booking, and a machine with almost nothing available asks for everything",async()=>{
 expect(await run(40,{...LINUX,SEMIO_GRAPH_MEMORY_MB:"1024"},9987*MB)).toEqual({lines:[],error:undefined});
 const starved=await run(40,LINUX,2300*MB);
 expect(starved.error?.code).toBe("unserved");expect(starved.lines.join("")).toContain("this machine can infer 0");
});
