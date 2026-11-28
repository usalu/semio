import {expect,test}from"bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import {tmpdir}from"node:os";
import {join}from"node:path";
import {issueNativeStorageRootCapabilityV1,nativeStorageLiveRootsV1,nativeStorageUnderTestRunnerV1,requireNativeStorageRootCapabilityV1}from"../🟦️.ts";
const live=nativeStorageLiveRootsV1()[0]!,partition=join(live,".cache","b","h64-capability-negative");
const refusal=(run:()=>unknown)=>{try{run();}catch(error){return error as{key?:string;code?:string};}return undefined;};
test("a test runner is recognised from its environment, never from its command line",()=>{
 expect(nativeStorageUnderTestRunnerV1({NODE_ENV:"test"})).toBe(true);
 expect(nativeStorageUnderTestRunnerV1({SEMIO_TEST_SANDBOX:"/x"})).toBe(true);
 expect(nativeStorageUnderTestRunnerV1({SEMIO_ISOLATED_BUILD_TEST:"a"})).toBe(true);
 expect(nativeStorageUnderTestRunnerV1({})).toBe(false);expect(nativeStorageUnderTestRunnerV1({NODE_ENV:"production"})).toBe(false);
});
test("a test-runner call against the live checkout is refused with a bilingual error, whatever the operation",()=>{
 for(const operation of["retire","delete","compact","reborn","detach"]){
  const error=refusal(()=>requireNativeStorageRootCapabilityV1(partition,operation));
  expect(error?.key,operation).toBe("process.storage.root-capability-refused");expect(error?.code).toBe("refused");
 }
 expect(refusal(()=>issueNativeStorageRootCapabilityV1(join(live,".cache","b","d"),"retire",{NODE_ENV:"test"}))?.key).toBe("process.storage.root-capability-refused");
 expect(refusal(()=>issueNativeStorageRootCapabilityV1("/Users/Shared/semio-store/v1/origins/x","retire",{NODE_ENV:"test"}))?.key).toBe("process.storage.root-capability-refused");
});
test("an existing directory outside the sandbox and the temporary directory is refused, a sandbox one is issued",()=>{
 const sandbox=mkdtempSync(join(tmpdir(),"h64-root-")),image=join(sandbox,".cache","b","d");mkdirSync(image,{recursive:true});
 try{
  const issued=issueNativeStorageRootCapabilityV1(image,"retire",{NODE_ENV:"test"});
  expect(issued.kind).toBe("sandbox");expect(issued.root).toBe(sandbox);
  expect(issueNativeStorageRootCapabilityV1(image,"retire",{SEMIO_TEST_SANDBOX:sandbox}).kind).toBe("sandbox");
  expect(refusal(()=>issueNativeStorageRootCapabilityV1(realpathSync("/usr"),"delete",{NODE_ENV:"test"}))?.key).toBe("process.storage.root-capability-refused");
  expect(issueNativeStorageRootCapabilityV1(join(sandbox,"absent"),"retire",{NODE_ENV:"test"}).kind).toBe("unmaterialized");
 }finally{rmSync(sandbox,{recursive:true,force:true});}
});
test("outside a test runner every root is admitted so production keeps its behaviour",()=>{
 expect(issueNativeStorageRootCapabilityV1(partition,"retire",{},process.cwd()).kind).toBe("production");
});
