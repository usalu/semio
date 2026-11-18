/** 📚️ Contract: a computation that observed hundreds of thousands of facts and then failed hands its own failure to the caller; the settle loop never turns the width of its row list into a `RangeError` (the youngest source of the rows was a `Math.max` over a spread of all of them: a selected read of two Rust owners died with `Maximum call stack size exceeded`). */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{observedFs,settleObservedComputationV1}from"{{OBS}}";

test("a failure after 1200000 observed facts reaches the caller as itself",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"wide-failure-")));
 try{
  const failure=new Error("the computation failed on purpose");
  await expect(settleObservedComputationV1(()=>{for(let at=0;at<1200000;at++)observedFs.existsSync(join(root,"absent-"+at));throw failure;})).rejects.toBe(failure);
 }finally{rmSync(root,{recursive:true,force:true});}
},120_000);
