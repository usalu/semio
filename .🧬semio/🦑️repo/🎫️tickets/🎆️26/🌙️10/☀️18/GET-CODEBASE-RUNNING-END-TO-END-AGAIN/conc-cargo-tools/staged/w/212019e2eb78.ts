import{test,expect}from"bun:test";
import{mkdirSync,mkdtempSync}from"node:fs";
import{join,resolve}from"node:path";
import{receiveScriptProcessInvocation}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"@semio-tech/process/compiler-capacity";
import{withCompilerSharedHeadroomV1,withCompilerWriterCustodyV1}from"../../🟦️.ts";

/** 🤝️ Compiles hold the writer custody together; only recovering missing headroom or maintenance is exclusive, and an exclusive request waits for the running compiles without being starved by later ones. */
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
test("concurrent shared compiles overlap, headroom recovery and maintenance are exclusive, and a short image recovers once",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original diagnostic fixture directory required");
 const root=mkdtempSync(join(resolve(base),"sh-")),lease=join(root,"lease"),image={root:join(root,"img"),maximumPhysicalBytes:1<<40,recheck:async()=>{}};mkdirSync(image.root,{recursive:true});
 const control=scriptCompilerAdmissionControlV1(original,"rust:shared-writer",120000),log:string[]=[],t0=Date.now(),request={version:1 as const,wait:true};
 const compile=(name:string,hold:number)=>withCompilerSharedHeadroomV1(image,lease,control,1024,request,async()=>{log.push(name+"+");await pause(hold);log.push(name+"-");});
 await Promise.all([compile("A",600),compile("B",600),compile("C",600)]);
 expect(log.slice(0,3).sort()).toEqual(["A+","B+","C+"]);expect(Date.now()-t0).toBeLessThan(2400);
 log.length=0;
 const maintenance=async()=>{await pause(150);return withCompilerWriterCustodyV1(image,lease,control,request,async()=>{log.push("M+");await pause(100);log.push("M-");});};
 await Promise.all([compile("A",500),maintenance(),(async()=>{await pause(300);return compile("L",50);})()]);
 expect(log.indexOf("M+")).toBeGreaterThan(log.indexOf("A-"));expect(log.indexOf("L+")).toBeGreaterThan(log.indexOf("M-"));
 let recovered=0;
 await withCompilerSharedHeadroomV1(image,lease,control,1024,request,async()=>1,async()=>{recovered++;});expect(recovered,"enough headroom never escalates").toBe(0);
 let available=0;const measure=async()=>available,exclusive:string[]=[];
 await withCompilerSharedHeadroomV1(image,lease,control,4096,request,async()=>{exclusive.push("body");return 1;},async()=>{available=8192;recovered++;},measure);
 expect(recovered,"a short image recovers once, exclusively, and then compiles").toBe(1);expect(exclusive).toEqual(["body"]);
}));
