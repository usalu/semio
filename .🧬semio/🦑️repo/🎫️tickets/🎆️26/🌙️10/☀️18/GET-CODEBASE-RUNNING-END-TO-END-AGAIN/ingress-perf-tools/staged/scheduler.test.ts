import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,unlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "./@JSON@" with {type:"json"};
import {captureSchedulerReceiptV1,readSchedulerReceiptV1,schedulerIdentityV1,schedulerInputsV1,schedulerReceiptHoldsV1,writeSchedulerReceiptV1,type SchedulerReceiptV1} from "../@TS@";

const pause=()=>new Promise<void>(done=>setTimeout(done,15));
for(const row of fixture.environment)test("scheduler identity: "+row.name,()=>{expect(schedulerIdentityV1("t",row.left)===schedulerIdentityV1("t",row.right)).toBe(row.equal);});

test("scheduler receipt: holds exactly while sources, compiler identity and executables are unchanged",async()=>{
 const root=mkdtempSync(join(tmpdir(),"scheduler-receipt-")),manifest=join(root,"Cargo.toml"),binary=join(root,"target","sched");
 try{
  mkdirSync(join(root,"src"));mkdirSync(join(root,"target"));writeFileSync(manifest,fixture.manifest);writeFileSync(join(root,"Cargo.lock"),fixture.lock);writeFileSync(join(root,"src","bin.rs"),fixture.source);
  expect(schedulerInputsV1(manifest).map(path=>path.slice(root.length+1))).toEqual(["Cargo.toml","Cargo.lock","src/bin.rs"]);
  let receipt:SchedulerReceiptV1|undefined;const identity=(toolchain:string)=>schedulerIdentityV1(toolchain,{});
  for(const step of fixture.steps as Array<{op:string;text?:string;toolchain?:string;holds?:boolean}>){
   await pause();
   if(step.op==="build"){writeFileSync(binary,"binary-"+Math.random());receipt=captureSchedulerReceiptV1(identity("t1"),manifest,new Map([["sched",binary]]));expect(receipt).toBeDefined();}
   else if(step.op==="editSource")writeFileSync(join(root,"src","bin.rs"),step.text!);
   else if(step.op==="editLock")writeFileSync(join(root,"Cargo.lock"),step.text!);
   else if(step.op==="rebuildBinary")writeFileSync(binary,"binary-"+Math.random());
   else if(step.op==="removeBinary")unlinkSync(binary);
   else if(step.op==="hold")expect(schedulerReceiptHoldsV1(receipt,identity(step.toolchain!),manifest)!==undefined).toBe(step.holds!);
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("scheduler receipt: round-trips through its file and refuses tampering",()=>{
 const root=mkdtempSync(join(tmpdir(),"scheduler-file-")),manifest=join(root,"Cargo.toml"),binary=join(root,"sched");
 try{
  mkdirSync(join(root,"src"));writeFileSync(manifest,fixture.manifest);writeFileSync(join(root,"Cargo.lock"),fixture.lock);writeFileSync(join(root,"src","bin.rs"),fixture.source);writeFileSync(binary,"x");
  const receipt=captureSchedulerReceiptV1(schedulerIdentityV1("t",{}),manifest,new Map([["sched",binary]]))!,file=join(root,"receipt.json");
  writeSchedulerReceiptV1(file,receipt);
  expect(readSchedulerReceiptV1(file)).toEqual(receipt);
  writeFileSync(file,JSON.stringify({...receipt,extra:1}));
  expect(readSchedulerReceiptV1(file)).toBeUndefined();
  writeFileSync(file,"{");
  expect(readSchedulerReceiptV1(file)).toBeUndefined();
  const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
  if(python){const digest=spawnSync(python,["-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",manifest],{encoding:"utf8"}).stdout.trim();expect(receipt.inputs[0]!.digest).toBe(digest);expect(digest).toBe(createHash("sha256").update(readFileSync(manifest)).digest("hex"));}
 }finally{rmSync(root,{recursive:true,force:true});}
});
