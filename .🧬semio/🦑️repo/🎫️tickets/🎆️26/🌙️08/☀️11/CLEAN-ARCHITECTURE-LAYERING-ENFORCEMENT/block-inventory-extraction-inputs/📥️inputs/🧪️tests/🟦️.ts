import { acquireCargoBuildLeaseV1 } from "../../../../../🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { test,expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFile, mkdir } from "node:fs/promises";
import { createRequire } from "node:module";
import { join,isAbsolute } from "node:path";
import { setImmediate } from "node:timers/promises";
import { runMutationInventoryProcess,readMutationInventoryCapture,readMutationInventoryExecutionPolicy,MutationInventoryProcessWorkspace,MutationInventoryProcessError,mutationInventoryPolicyArguments,admitMutationInventoryProducerArguments } from "../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import lawSchema from "../🧬️laws/🔣️.json" with {type:"json"};
const require=createRequire(import.meta.url),Ajv=require("ajv"),execa=require("execa"),validate=new Ajv({strict:false}).compile(schema);
const output=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!output||!isAbsolute(output))throw Error("Explicit mutation inventory test artifact directory required");
const policy={version:1 as const,maximumUnits:1000000,maximumOwnedBytes:1048576,maximumCaptureBytes:262144,budgetMs:10000,compilerStorage:{buildDirectory:join(output,"build"),leaseDirectory:join(output,"leases"),targetDirectory:join(output,"target"),captureDirectory:join(output,"captures")}};
function operation(w:MutationInventoryProcessWorkspace,controller=new AbortController(),overrides:Record<string,number>={}){return{signal:controller.signal,maximumUnits:policy.maximumUnits,maximumOwnedBytes:policy.maximumOwnedBytes,workspace:w,onProgress:async(_progress:{stage:string})=>{},yieldContinuation:()=>setImmediate(),...overrides};}
function request(source:string,overrides:Record<string,number>={}){return{command:process.execPath,argv:["-e",source],cwd:process.cwd(),environment:process.env,captureDirectory:policy.compilerStorage.captureDirectory,budgetMs:policy.budgetMs,maximumCaptureBytes:policy.maximumCaptureBytes,compiler:null,...overrides};}
test("authored policy uses the same closed admission as AJV",()=>{
  expect(new Ajv({strict:false}).compile(lawSchema)(fixture)).toBe(true);expect(validate(policy)).toBe(true);expect(mutationInventoryPolicyArguments(policy)).toEqual(["--maximum-units","1000000","--maximum-owned-bytes","1048576","--budget-ms","10000"]);for(const args of [["list-mutations","owner","1","any"],["list-mutations","owner","1","any",...mutationInventoryPolicyArguments({...policy,budgetMs:2})]])expect(()=>admitMutationInventoryProducerArguments(args,policy)).toThrow();expect(admitMutationInventoryProducerArguments(["list-mutations","owner","1","any",...mutationInventoryPolicyArguments(policy)],policy)).toHaveLength(10);expect(readMutationInventoryExecutionPolicy({SEMIO_MUTATION_INVENTORY_POLICY:JSON.stringify(policy)})).toEqual(policy);
  for(const value of [{...policy,budgetMs:0},{...policy,maximumUnits:0},{...policy,unknown:1},{...policy,compilerStorage:{...policy.compilerStorage,buildDirectory:"relative"}}]){expect(validate(value)).toBe(false);expect(()=>readMutationInventoryExecutionPolicy({SEMIO_MUTATION_INVENTORY_POLICY:JSON.stringify(value)})).toThrow();}
  expect(()=>readMutationInventoryExecutionPolicy({})).toThrow();
});
test("real owned captures equal installed execa and SQLite corpus",async()=>{
  await mkdir(output,{recursive:true});const db=new Database(":memory:");db.exec("CREATE TABLE expected(id TEXT,stdout TEXT,stderr TEXT,code INTEGER)");
  for(const row of fixture.cases){
    db.query("INSERT INTO expected VALUES(?,?,?,?)").run(row.id,row.stdout,row.stderr,row.code);
    const independent=await execa(process.execPath,["-e",row.source],{reject:false,stripEof:false,timeout:10000});
    const oracle=db.query("SELECT stdout,stderr,code FROM expected WHERE id=?").get(row.id) as {stdout:string;stderr:string;code:number};
    expect({stdout:independent.stdout,stderr:independent.stderr,code:independent.code}).toEqual(oracle);
    const w=new MutationInventoryProcessWorkspace(),op=operation(w),result=await runMutationInventoryProcess(request(row.source),op);
    expect(result.reason).toBe("exit");expect(result.code).toBe(oracle.code);
    expect(await readMutationInventoryCapture(result.stdout,op)).toBe(oracle.stdout);expect(await readMutationInventoryCapture(result.stderr,op)).toBe(oracle.stderr);
    expect(w.process?.closed).toBe(true);expect(w.stdout?.closed).toBe(true);expect(w.stderr?.closed).toBe(true);expect(w.lease).toBeUndefined();
    const receipt=JSON.parse(await readFile(result.receiptPath,"utf8"));expect(receipt.code).toBe(row.code);expect(receipt.capturedBytes).toBe(new TextEncoder().encode(row.stdout+row.stderr).length);
  }db.close();
},30000);
test("real refusal conserves exact partial capture and stopped process owners",async()=>{
  for(const row of fixture.controls){
    const w=new MutationInventoryProcessWorkspace(),controller=new AbortController(),op=operation(w,controller,row.maximumUnits!==undefined?{maximumUnits:row.maximumUnits}:row.maximumOwnedBytes!==undefined?{maximumOwnedBytes:row.maximumOwnedBytes}:{});
    let timer:ReturnType<typeof setTimeout>|undefined;
    if(row.kind==="cancel")timer=setTimeout(()=>controller.abort(),100);
    try{
      const result=await runMutationInventoryProcess(request(row.source,row.maximumCaptureBytes!==undefined?{maximumCaptureBytes:row.maximumCaptureBytes}:row.budgetMs!==undefined?{budgetMs:row.budgetMs}:{}),op);
      expect(String(result.reason)).toBe(row.reason);expect(w.process?.closed).toBe(true);expect(w.stdout?.closed).toBe(true);expect(w.stderr?.closed).toBe(true);
      if(row.kind==="capture"){expect(result.stdout.bytes+result.stderr.bytes).toBe(13);expect(await readFile(result.stdout.path,"utf8")).toBe("x".repeat(13));}
      expect(JSON.parse(await readFile(result.receiptPath,"utf8")).reason).toBe(row.reason);
    }catch(error){expect(error).toBeInstanceOf(MutationInventoryProcessError);expect(String((error as MutationInventoryProcessError).reason)).toBe(row.reason);expect((error as MutationInventoryProcessError).workspace).toBe(w);expect(w.process).toBeUndefined();expect(w.input).toBeDefined();}
    finally{if(timer)clearTimeout(timer);}
  }
  const independent=await execa(process.execPath,["-e",'setTimeout(()=>{},10000)'],{reject:false,timeout:100});expect(independent.timedOut).toBe(true);
},30000);

test("real compiler lease stays owned until shutdown and cancels in the queue",async()=>{
  const compiler={buildDirectory:policy.compilerStorage.buildDirectory,leaseDirectory:policy.compilerStorage.leaseDirectory};
  for(const row of fixture.leases){
    const w=new MutationInventoryProcessWorkspace(),op=operation(w),held=row.kind==="held"?await acquireCargoBuildLeaseV1({directory:compiler.leaseDirectory,buildDirectory:compiler.buildDirectory,args:[],signal:new AbortController().signal,onWait:()=>{}}):undefined;
    let queued=false;op.onProgress=async(progress:{stage:string})=>{if(progress.stage==="waiting"&&w.directory&&!w.process)queued=true;};
    try{
      const result=await runMutationInventoryProcess({...request("void 0"),command:"cargo",argv:["--version"],compiler,budgetMs:row.budgetMs??policy.budgetMs},op);
      expect(String(result.reason)).toBe(row.reason);expect(w.stdout?.closed).toBe(true);expect(w.stderr?.closed).toBe(true);expect(w.lease).toBeUndefined();
      if(row.kind==="cargo"){const independent=await execa("cargo",["--version"],{reject:false,stripEof:false,timeout:10000});expect(result.code).toBe(independent.code);expect(await readMutationInventoryCapture(result.stdout,op)).toBe(independent.stdout);expect(w.process?.closed).toBe(true);}
      else {expect(w.process).toBeUndefined();expect(queued).toBe(Boolean(row.queued));}
    }finally{held?.release();}
  }
},30000);

test("compiler paths are admitted before physical storage or process work",async()=>{
  for(const compiler of [{buildDirectory:"relative",leaseDirectory:policy.compilerStorage.leaseDirectory},{buildDirectory:policy.compilerStorage.buildDirectory,leaseDirectory:"relative"},{buildDirectory:join(output!,"bad\0path"),leaseDirectory:policy.compilerStorage.leaseDirectory}]){
    const w=new MutationInventoryProcessWorkspace();let failure:unknown;
    try{await runMutationInventoryProcess({...request("void 0"),compiler},operation(w));}catch(error){failure=error;}
    expect(failure).toBeDefined();expect(w.input).toBeDefined();expect(w.directory).toBeUndefined();expect(w.process).toBeUndefined();expect(w.lease).toBeUndefined();
  }
});
