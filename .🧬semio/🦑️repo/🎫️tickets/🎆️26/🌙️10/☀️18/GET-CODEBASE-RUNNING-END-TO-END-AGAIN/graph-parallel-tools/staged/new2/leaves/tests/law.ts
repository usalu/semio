import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import Ajv from"ajv/dist/2020";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,realpathSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createBuildGraphOptionalQuotaV1}from"../../🟨️.mjs";
import{openBuildGraphOptionalLedgerV1}from"../../📒️ledger/🟨️.mjs";
import{createOptionalAccountingV1,createWorkerWriterV1,isOptionalLeafNameV1,OPTIONAL_LEAVES_POLICY_V1}from"../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};

const oracle=join(import.meta.dir,"🐍️oracle.py"),digest=(n:number)=>n.toString(16).padStart(64,"0");

function replay(row:any){
 let now=0,sinceCalls:any[]=[],accepts=true;
 const ledger={publish(plan:any,write:()=>void){sinceCalls.push({peakBytes:plan.peakBytes,retainedDeltaBytes:plan.retainedDeltaBytes,accepted:accepts});if(accepts)write();return accepts;}};
 const quota=createBuildGraphOptionalQuotaV1(row.quota),removed:string[]=[],results:string[]=[],grants:Array<()=>void>=[];
 const accounting=createOptionalAccountingV1({ledger,quota,policy:{...OPTIONAL_LEAVES_POLICY_V1,...row.policy},clock:()=>now,wait:()=>{}});
 for(const step of row.steps){
  accepts=step.ledgerAccepts??true;
  if(step.op==="publish")results.push(accounting.publish({peakBytes:step.bytes,retainedDeltaBytes:step.bytes},()=>{},step.deferred===true)?(step.deferred?"deferred":"published"):"declined");
  else if(step.op==="adopt"){const outcome=accounting.adopt(step.leaves,name=>removed.push(name));results.push("adopted:"+outcome.accepted.length);}
  else if(step.op==="clock"){now+=step.milliseconds;results.push("clock");}
  else if(step.op==="flush")results.push(accounting.flush(step.final===true)?"flushed":"postponed");
  else if(step.op==="grant"){const release=accounting.grant(step.bytes);if(release){grants.push(release);results.push("granted");}else results.push("refused");}
  else if(step.op==="release"){grants.pop()!();results.push("released");}
 }
 const facts=accounting.facts();
 return{results,ledger:sinceCalls,used:quota.usedBytes,owedBytes:facts.owedBytes,owedLeaves:facts.owedLeaves,removed,refused:removed.slice()};
}

test("the fixture is closed (Ajv) and equal to the independent model",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(schema as any);const validate=ajv.compile({$ref:schema.$id+"#/$defs/Fixture"});
 expect(validate(fixture)).toBe(true);
 const broken=structuredClone(fixture)as any;broken.cases[0].steps[0].op="rewrite";expect(validate(broken)).toBe(false);
 const python=spawnSync("python3",[oracle],{encoding:"utf8"});
 expect(python.status,python.stdout+python.stderr).toBe(0);
});

test("the accounting equals the recorded model for every case: admission, thresholds, adoption, debt and grants",()=>{
 for(const row of fixture.cases)expect(replay(row),row.name).toEqual(row.expect);
});

test("optional leaf names are exactly the declared optional paths",()=>{
 expect(isOptionalLeafNameV1("np/ob/"+digest(1)+".br")).toBe(true);
 expect(isOptionalLeafNameV1("ip/"+"a".repeat(24)+".json")).toBe(true);
 expect(isOptionalLeafNameV1("rr/"+"b".repeat(24)+".json")).toBe(true);
 for(const name of["np/ob/"+digest(1)+".json","../np/x","ip/xyz.json","selections/x/ip/"+"a".repeat(24)+".json",5,undefined])expect(isOptionalLeafNameV1(name)).toBe(false);
});

test("a worker writer writes within its grant, declines beyond it and reports what it wrote",()=>{
 const root=mkdtempSync(join(tmpdir(),"worker-writer-")),inference=join(root,"inference");
 try{
  const writer=createWorkerWriterV1({roots:{inference,"build-input-closures":join(root,"build-input-closures")},grantBytes:100});
  expect(writer.directory("build-input-closures")).toBe(join(root,"build-input-closures"));expect(existsSync(join(root,"build-input-closures"))).toBe(true);
  expect(()=>writer.directory("elsewhere")).toThrow("Declared graph worker namespace required");
  const put=(name:string,size:number)=>writer.publish(join(inference,name),{peakBytes:size,retainedDeltaBytes:size},()=>{mkdirSync(join(inference,name,".."),{recursive:true});writeFileSync(join(inference,name),Buffer.alloc(size,1));});
  const first="np/ob/"+digest(1)+".br",second="np/ob/"+digest(2)+".br",third="ip/"+"c".repeat(24)+".json";
  expect(put(first,60)).toBe(true);expect(put(second,60)).toBe(false);expect(existsSync(join(inference,second))).toBe(false);expect(put(third,40)).toBe(true);
  expect(()=>writer.publish(join(root,"elsewhere.json"),{peakBytes:1,retainedDeltaBytes:1},()=>{})).toThrow("Explicit optional graph cache leaf required");
  expect(()=>writer.publish(join(inference,"np","x.tmp"),{peakBytes:1,retainedDeltaBytes:1},()=>{})).toThrow("Explicit optional graph cache leaf required");
  writer.defer("crates/a/Cargo.toml");writer.defer("crates/a/Cargo.toml");writer.defer("crates/b/Cargo.toml");
  expect(writer.report()).toEqual({leaves:[{name:first,bytes:60},{name:third,bytes:40}],declined:[second],receipts:["crates/a/Cargo.toml","crates/b/Cargo.toml"]});
  expect(()=>createWorkerWriterV1({roots:{inference:"relative"},grantBytes:1})).toThrow("Absolute graph worker roots");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("deferred accounting keeps the real ledger equal to the physical bytes after the final flush, read back by SQLite from Python",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"leaves-ledger-"))),inference=join(root,"inference");mkdirSync(join(inference,"np","ob"),{recursive:true});
 const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>{},yieldContinuation:async()=>{}},big=6*2**30;
 const image={root,maximumPhysicalBytes:big,filesystemBytes:big,recheck:async()=>{},port:{observe:async()=>({root,maximumPhysicalBytes:big})}};
 const ledger=await openBuildGraphOptionalLedgerV1({image,control,leaseDirectory:join(root,"lease"),maximumBytes:2**30,minimumAvailableBytes:1});
 try{
  const quota=createBuildGraphOptionalQuotaV1({maximumBytes:2**30,usedBytes:ledger.observe().usedBytes,availableBytes:big,minimumAvailableBytes:1}),accounting=createOptionalAccountingV1({ledger,quota,policy:{...OPTIONAL_LEAVES_POLICY_V1,flushLeaves:16}});
  let physical=0;
  for(let n=1;n<=40;n++){
   const path=join(inference,"np","ob",digest(n)+".br"),size=100+n;
   expect(accounting.publish({peakBytes:size,retainedDeltaBytes:size},()=>{writeFileSync(path+".1.1.tmp",Buffer.alloc(size,2));renameSync(path+".1.1.tmp",path);},true)).toBe(true);physical+=size;
  }
  const worker=join(inference,"ip");mkdirSync(worker);writeFileSync(join(worker,"a".repeat(24)+".json"),Buffer.alloc(77));physical+=77;
  expect(accounting.adopt([{name:"ip/"+"a".repeat(24)+".json",bytes:77}],()=>{}).accepted.length).toBe(1);
  expect(accounting.facts().flushes).toBeGreaterThan(0);
  expect(accounting.flush(true)).toBe(true);expect(accounting.facts().owedLeaves).toBe(0);
  expect(ledger.observe().usedBytes).toBe(physical);expect(quota.usedBytes).toBe(physical);
  const python=spawnSync("python3",["-I","-c","import sqlite3,sys;print(sqlite3.connect('file:'+sys.argv[1]+'?mode=ro',uri=True).execute('select used_bytes,dirty from semio_graph_optional where id=1').fetchone())",join(root,"optional.sqlite")],{encoding:"utf8"});
  expect(python.stdout.trim()).toBe("("+physical+", 0)");
 }finally{ledger.close();rmSync(root,{recursive:true,force:true});}
});

test("deferral spends far fewer ledger transactions than one publication per leaf",async()=>{
 let calls=0;const quota=createBuildGraphOptionalQuotaV1({maximumBytes:2**30,usedBytes:0,availableBytes:2**40,minimumAvailableBytes:1}),accounting=createOptionalAccountingV1({ledger:{publish:(_plan:any,write:()=>void)=>{calls++;write();return true;}},quota});
 for(let n=0;n<640;n++)accounting.publish({peakBytes:1000,retainedDeltaBytes:1000},()=>{},true);
 accounting.flush(true);
 expect(calls).toBeLessThanOrEqual(640/OPTIONAL_LEAVES_POLICY_V1.flushLeaves+2);
});

const storage=readFileSync(fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),"utf8");

test("an adopted leaf must carry a declared name and a bounded size",()=>{
 const quota=createBuildGraphOptionalQuotaV1({maximumBytes:2**30,usedBytes:0,availableBytes:2**40,minimumAvailableBytes:1}),accounting=createOptionalAccountingV1({ledger:{publish:(_plan:any,write:()=>void)=>{write();return true;}},quota});
 for(const row of[{name:"../x",bytes:1},{name:"np/x.tmp",bytes:1},{name:"np/ob/"+digest(1)+".br",bytes:-1},{name:"np/ob/"+digest(1)+".br",bytes:OPTIONAL_LEAVES_POLICY_V1.maximumLeafBytes+1},{name:"np/ob/"+digest(1)+".br",bytes:1.5}])expect(()=>accounting.adopt([row],()=>{}),JSON.stringify(row)).toThrow("Optional graph leaf name and size required");
 expect(quota.usedBytes).toBe(0);
});

test("the graph writer pays optional leaves through the shared accounting: batched inside a batch, flushed before the ledger is measured, retired or closed",()=>{
 expect(storage).toContain("if(!optionalAccountingOfV1(scope).publish(plan,write,batches.getStore()!==undefined,reservation))return decline();");
 expect(storage).toContain("return batches.run({receipts},body);");
 expect(storage).toContain("flushBuildGraphOwedV1(writer.ledger);writer.presence.clear();writer.ledger.retire(");
 expect(storage).toContain("try{flushBuildGraphOwedV1(ledger);}catch(error){if(!failed)throw error;}try{ledger.close();}");
 expect(storage).toContain("try{if(ledger)flushBuildGraphOwedV1(ledger);}catch(error){cleanupFailed=true;cleanupFailure=error;}");
 expect(storage.match(/presence:new Set\(\),ledger,optionalQuota/gu)?.length).toBe(2);
 expect(storage).toContain("export function buildGraphWriterPresenceV1(){return writers.getStore()?.presence??workerWriters.getStore()?.presence;}");
 expect(storage.match(/stagings/gu)).toBeNull();
});

test("a worker job writes into the directories its parent named and nowhere else, and the parent accounts under its writer custody only",()=>{
 expect(storage).toContain("const worker=workerWriters.getStore();if(worker)return worker.publish(path,plan,write);");
 expect(storage).toContain("const worker=workerWriters.getStore();if(worker)return worker.directory(namespace);");
 expect(storage.match(/if\(worker\)return worker\.directory\(namespace\);/gu)?.length).toBe(2);
 const adopt=storage.slice(storage.indexOf("export function adoptBuildGraphWorkerLeavesV1"),storage.indexOf("export function grantBuildGraphWorkerBytesV1")),grant=storage.slice(storage.indexOf("export function grantBuildGraphWorkerBytesV1"),storage.indexOf("export function buildGraphWriterPresenceV1"));
 expect(adopt).toContain("requireRepositoryBuildGraphWriterV1(workspace);");expect(adopt).toContain("if(!scope.presence.has(path))rmSync(path,{force:true});");
 expect(grant).toContain("requireRepositoryBuildGraphWriterV1(workspace);");
});

test("a receipt is deferred from a worker job or from a batch that collects them, and from nothing else",async()=>{
 const{deferBuildGraphReceiptV1,withBuildGraphOptionalBatchV1,withBuildGraphWorkerWriterV1}=await import("../../🟨️.mjs");
 expect(deferBuildGraphReceiptV1("a/Cargo.toml")).toBe(false);
 const collected=new Set<string>();
 await withBuildGraphOptionalBatchV1(async()=>{expect(deferBuildGraphReceiptV1("b/Cargo.toml")).toBe(true);},collected);
 expect(await withBuildGraphOptionalBatchV1(async()=>deferBuildGraphReceiptV1("c/Cargo.toml"))).toBe(false);
 expect([...collected]).toEqual(["b/Cargo.toml"]);
 const root=mkdtempSync(join(tmpdir(),"defer-law-"));
 try{
  const report=await withBuildGraphWorkerWriterV1({roots:{inference:join(root,"inference")},grantBytes:1},async writer=>{expect(deferBuildGraphReceiptV1("d/Cargo.toml")).toBe(true);return writer.report();});
  expect(report.receipts).toEqual(["d/Cargo.toml"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
