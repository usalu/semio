import {scriptInvocationEnvironment} from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type {ScriptInvocation} from "../../../🧭️routing/📥️invocation/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../🧪️testing/📥️invocation/🟦️.ts";
import {testPythonV1} from "../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {availableParallelism} from "node:os";
import {createHash} from "node:crypto";
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,existsSync,readdirSync} from "node:fs";
import {join} from "node:path";
import {spawnSync,spawn} from "node:child_process";
import {acquireCompilerCapacityLeaseV1,scriptCompilerAdmissionControlV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import orderPolicy from "../../../🔒️leases/🧭️order/🔣️policy.json";
import fixture from "./🧫️fixtures/🔣️.json";
const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;
mkdirSync(output,{recursive:true});
const distinctClaimants=join(output,"lease-order-policy.json");writeFileSync(distinctClaimants,JSON.stringify({...orderPolicy,fairness:{...orderPolicy.fairness,reentrantShared:false}}));process.env.SEMIO_LEASE_ORDER_POLICY=distinctClaimants;process.env.SEMIO_LEASE_ORDER="off";
const storage=mkdtempSync(join(output,"cap-"));
const signal=()=>new AbortController().signal;
const quiet=()=>undefined;
let invocation!:ScriptInvocation;
const control=(additional?:AbortSignal)=>scriptCompilerAdmissionControlV1(invocation,"test:compiler-capacity",120_000,additional);
test("Compiler capacity request admission agrees with an independent schema oracle",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const oracle=new Ajv({strict:true}).compile(schema.definitions.Request);
 for(const row of fixture.requests){expect(oracle(row.value)).toBe(row.accepted);if(!row.accepted)await expect(acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:row.value} as any)).rejects.toThrow();}
 const claim=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:1}});try{expect(claim).toMatchObject({mode:fixture.shared.mode,parallelism:fixture.shared.parallelism});expect(new Ajv({strict:true}).compile(schema)({resource:claim.resource,mode:claim.mode,parallelism:claim.parallelism})).toBe(true);}finally{claim.release();}
}));
test("Bounded single-worker claims run real children concurrently and give a whole-host writer priority",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const held=[] as Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>>[];
 const root=mkdtempSync(join(storage,"c-"));
 try{
  const first=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:1}});held.push(first);expect(first.parallelism).toBe(1);expect(first.mode).toBe("shared");
  if(availableParallelism()<2)return;
  const second=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:1}});held.push(second);
  const body=`const start=Date.now();let cycles=0;while(Date.now()-start<${fixture.shared.bodyMs}){for(let i=0;i<100000;i++)cycles=(cycles+i)>>>0;}console.log(JSON.stringify({start,end:Date.now(),parallelism:Number(process.env.SEMIO_COMPILER_PARALLELISM),cycles}));`;
  const children=held.map(claim=>Bun.spawn([process.execPath,"--eval",body],{stdout:"pipe",stderr:"pipe",env:{...process.env,SEMIO_COMPILER_CAPACITY_HELD:"true",SEMIO_COMPILER_PARALLELISM:String(claim.parallelism)}}));
  const rows=await Promise.all(children.map(async child=>{const row=JSON.parse(await new Response(child.stdout).text());expect(await child.exited).toBe(0);expect(row.parallelism).toBe(1);return row;}));expect(Math.max(...rows.map(row=>row.start))).toBeLessThan(Math.min(...rows.map(row=>row.end)));
  const path=join(storage,createHash("sha256").update(schema.properties.resource.const).digest("hex")+".sqlite");const oracle=spawnSync(testPythonV1()??"python3",["-c","import sqlite3,sys;d=sqlite3.connect(sys.argv[1],timeout=0);d.execute('begin');print(d.execute('select resource from semio_resource_lease').fetchone()[0]);d.rollback();\ntry:d.execute('begin exclusive');print('unexpected')\nexcept sqlite3.OperationalError:print('whole-host-busy')",path],{encoding:"utf8"});expect(oracle.status,oracle.stderr).toBe(0);expect(oracle.stdout).toContain("whole-host-busy");
  for(let i=held.length;i<availableParallelism();i++)held.push(await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:1}}));
  let extraEntered=false;const extra=acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:1}}).then(claim=>{extraEntered=true;return claim;});await Bun.sleep(50);expect(extraEntered).toBe(false);held.pop()!.release();held.push(await extra);
  const order=[] as string[],writer=acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"},directory:storage,control:control(),environment:{}}).then(claim=>{order.push("writer");return claim;});await Bun.sleep(50);
  const reader=acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:1}}).then(claim=>{order.push("reader");return claim;});await Bun.sleep(50);expect(order).toEqual([]);
  for(const claim of held.splice(0))claim.release();const all=await writer;expect(all.parallelism).toBe(availableParallelism());expect(order).toEqual(["writer"]);all.release();const last=await reader;last.release();expect(order).toEqual(fixture.fairOrder);
 }finally{for(const claim of held)claim.release();}
}),120_000);
test("Multi-core claims are granted whole and first come first served behind the core picker",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const law=fixture.multiCore;if(availableParallelism()<law.waiterParallelism+law.holderShortfall)return;
 const holder=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:availableParallelism()-law.holderShortfall}}),order=[] as string[];
 try{expect(holder.parallelism).toBe(availableParallelism()-law.holderShortfall);
  const wide=acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:law.waiterParallelism}}).then(claim=>{order.push("wide");return claim;});await Bun.sleep(100);
  const narrow=acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:1}}).then(claim=>{order.push("narrow");return claim;});await Bun.sleep(100);expect(order).toEqual([]);
  holder.release();const first=await wide,second=await narrow;try{expect(first.parallelism).toBe(law.waiterParallelism);expect(second.parallelism).toBe(1);expect(order).toEqual(law.order);}finally{first.release();second.release();}
 }finally{holder.release();}
}),120_000);
test("Cancelled whole-host requests and crashed single-worker owners release gate and core claims",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const first=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:1}});try{expect(first.parallelism).toBe(1);const controller=new AbortController(),waiting=acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"},directory:storage,control:control(controller.signal),environment:{}});await Bun.sleep(50);controller.abort();await expect(waiting).rejects.toMatchObject({name:fixture.cancel});}finally{first.release();}
 const root=mkdtempSync(join(storage,"s-")),marker=join(root,"ready.json"),source=new URL("../🟦️.ts",import.meta.url).href,routing=new URL("../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts",import.meta.url).href,script=join(root,"📜️script.ts");writeFileSync(script,`import {acquireCompilerCapacityLeaseV1,scriptCompilerAdmissionControlV1} from ${JSON.stringify(source)};import {receiveScriptProcessInvocation} from ${JSON.stringify(routing)};await receiveScriptProcessInvocation(process.env,async original=>{const claim=await acquireCompilerCapacityLeaseV1({directory:${JSON.stringify(storage)},control:scriptCompilerAdmissionControlV1(original,"fixture:child-claim",120000),environment:{},request:{kind:"exact",parallelism:1}});await Bun.write(${JSON.stringify(marker)},JSON.stringify({parallelism:claim.parallelism,pid:process.pid}));await new Promise(()=>setInterval(()=>{},10000));});`);
 const child=Bun.spawn([process.execPath,script],{stdout:"ignore",stderr:"pipe",env:scriptInvocationEnvironment(invocation,process.env) as Record<string,string>});try{while(!existsSync(marker)){if(child.exitCode!==null)throw Error(await new Response(child.stderr).text());await Bun.sleep(fixture.shared.pollMs);}expect(JSON.parse(readFileSync(marker,"utf8")).parallelism).toBe(1);}finally{child.kill("SIGKILL");await child.exited;}
 const all=await acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"},directory:storage,control:control(),environment:{}});all.release();const shared=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:1}});shared.release();
}),120_000);

test("Original admission control preserves incoming fixture cancellation identity into a real child",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const local=new AbortController(),law=fixture.additionalCancellation,admission=scriptCompilerAdmissionControlV1(original,"fixture:incoming-cancel",law.holdMilliseconds,local.signal);
 expect(admission.signal.aborted).toBe(false);
 const child=spawn(process.execPath,["-e",`console.log("ready");setInterval(()=>{},${law.holdMilliseconds});`],{signal:admission.signal,stdio:["ignore","pipe","pipe"]});child.on("error",()=>undefined);const closed=new Promise<void>(accept=>child.once("close",()=>accept()));
 try{await new Promise<void>(accept=>child.stdout!.once("data",()=>accept()));local.abort(Error("fixture local cancellation"));await closed;expect(original.control.signal.aborted).toBe(law.originalAborted);let alive=true;try{process.kill(child.pid!,0);}catch{alive=false;}expect(alive).toBe(law.childAlive);}finally{child.kill();await closed;}
}),120000);
