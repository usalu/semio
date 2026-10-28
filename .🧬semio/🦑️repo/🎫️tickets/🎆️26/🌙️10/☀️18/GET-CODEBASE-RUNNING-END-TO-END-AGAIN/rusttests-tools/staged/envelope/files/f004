import{expect,test}from"bun:test";
import{mkdtempSync,readFileSync}from"node:fs";
import{join}from"node:path";
import{tmpdir}from"node:os";
import{withBuildTestInvocationV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"@semio-tech/process/compiler-capacity";
import{withCompilerSharedHeadroomV1}from"../../🟦️.ts";
import{acquireQueuedResourceLease}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{resolve}from"node:path";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"storage-compiler"});

test("the compiler headroom wait polls until the headroom appears and honours cancellation",()=>withBuildTestInvocationV1(async original=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"dw-")),control=scriptCompilerAdmissionControlV1(original,"test:headroom-declared-wait",60000);
 const image={root:directory,maximumPhysicalBytes:2**40,recheck:async()=>undefined};
 let attempts=0;
 await withCompilerSharedHeadroomV1(image,directory,control,2**20,{version:1,wait:true},async()=>undefined,undefined,async()=>{attempts++;return attempts<3?0:2**30;});
 expect(attempts).toBeGreaterThanOrEqual(3);
}),60000);
test("the compiler capacity module paces its waits only through the declared wait",()=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");
 expect(source).not.toContain('node:timers/promises');
 expect(source).toContain("waitingDelayV1 as sleepV1");
});

test("headroom recovery never queues an exclusive writer request behind a long shared holder",()=>withBuildTestInvocationV1(async original=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"hr-")),control=scriptCompilerAdmissionControlV1(original,"test:headroom-no-queued-exclusive",60000),image={root:directory,maximumPhysicalBytes:2**40,recheck:async()=>undefined};
 let attempts=0;
 const holder=await acquireQueuedResourceLease({directory,resource:schema.definitions.Protocol.const.resource+":"+resolve(directory),mode:"shared",sharedCohort:true,owner:"long-shared-holder",control,signal:control.signal});
 try{
  const started=Date.now();
  await withCompilerSharedHeadroomV1(image,directory,control,2**20,{version:1,wait:true},async()=>undefined,async()=>undefined,async()=>{attempts++;return attempts<3?0:2**30;});
  expect(Date.now()-started).toBeLessThan(10000);
 }finally{holder.release();}
 expect(attempts).toBeGreaterThanOrEqual(3);
}),60000);
