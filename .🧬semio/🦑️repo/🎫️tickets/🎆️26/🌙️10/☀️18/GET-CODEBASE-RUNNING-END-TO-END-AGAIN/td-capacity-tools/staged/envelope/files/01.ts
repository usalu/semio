import {expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdtempSync,mkdirSync,existsSync,writeFileSync,readFileSync,rmSync} from "node:fs";
import {spawn} from "node:child_process";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {readPaidProcessCustodianPlanV1,withPaidProcessCustodianV1,type PaidProcessCustodianPortV1} from "../🟦️.ts";
import {scriptInvocationEnvironment} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../🧪️testing/📥️invocation/🟦️.ts";
import {tmpdir} from "node:os";
declareBuildTestEnvelopeV1({owner:"test:compiler-capacity-custodian",maximumElapsedMilliseconds:300000,capabilities:{}});
import {scriptCompilerAdmissionControlV1,acquireCompilerCapacityLeaseV1} from "../../🟦️.ts";
import {resourceLeasePathV1} from "../../../../🔒️leases/🟦️.ts";
import {captureOwnedCommandResultV1 as capture} from "../../../../🎛️owned-execution/📥️capture/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import capacitySchema from "../../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const artifactOutput=process.env.SEMIO_TEST_ARTIFACT_DIR??mkdtempSync(join(tmpdir(),"semio-capacity-"));if(process.env.SEMIO_TEST_ARTIFACT_DIR===undefined)process.once("exit",()=>{rmSync(artifactOutput,{recursive:true,force:true});});
test("Paid custodian retains actual direct-child capacity through drain, replay refusal and cancellation",async()=>withBuildTestInvocationV1(async original=>{
 const output=artifactOutput;mkdirSync(output,{recursive:true});const directory=mkdtempSync(join(output,"f-")),ready=join(directory,"r"),gate=join(directory,"g"),control=scriptCompilerAdmissionControlV1(original,"test:paid-direct-child-custodian",120000);
 const job={program:process.execPath,args:[fileURLToPath(new URL("./👷️worker/📜️script.ts",import.meta.url)),ready,gate],cwd:process.cwd(),parallelism:fixture.parallelism,output:fixture.output},plan={version:1 as const,coverage:"direct-child" as const,directory,parallelism:fixture.parallelism,jobs:[job,job]},oracle=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/Plan"});
 const operations:Promise<unknown>[]=[],captureOwnedCommandResultV1=(...args:Parameters<typeof capture>)=>{const work=capture(...args);operations.push(work);void work.catch(()=>{});return work;};
 const wait=async(path:string,child?:Promise<unknown>)=>{let ended=false;void child?.then(()=>{ended=true;},()=>{ended=true;});while(!existsSync(path)){control.signal.throwIfAborted();if(ended&&!existsSync(path))throw Error("Custodian test child ended before its marker "+path);if((control.remainingMilliseconds()??1)<1)throw Error("Custodian test gate exhausted");await Bun.sleep(10);}};
 const independent=(args:string[])=>{const work=new Promise<{status:number|null;stdout:string;stderr:string}>((resolve,reject)=>{const env={...process.env};delete env.SEMIO_SCRIPT_PROCESS_INVOCATION;const child=spawn(process.execPath,args,{cwd:process.cwd(),env,stdio:["ignore","pipe","pipe"]});let stdout="",stderr="";child.stdout.on("data",chunk=>{stdout+=chunk;});child.stderr.on("data",chunk=>{stderr+=chunk;});control.signal.addEventListener("abort",()=>child.kill(),{once:true});child.on("error",reject);child.on("close",status=>resolve({status,stdout,stderr}));});operations.push(work);void work.catch(()=>{});return work;};
 const core=resourceLeasePathV1(directory,capacitySchema.definitions.Protocol.const.coreResourcePrefix+"0"),probe=async()=>{const result=await captureOwnedCommandResultV1(process.platform==="win32"?"python":"python3",["-c","import sqlite3,sys\nd=sqlite3.connect(sys.argv[1],timeout=0)\ntry:\n d.execute('begin');d.execute('select resource from semio_resource_lease').fetchone();d.rollback();print('free')\nexcept sqlite3.OperationalError as error:\n if 'locked' not in str(error):raise\n print('paid-busy')\nfinally:d.close()",core],process.cwd(),{control,output:fixture.output});expect(result.status,result.stderr).toBe(0);return result.stdout.trim();};
 expect(new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/Response"})(fixture.forgedResult)).toBe(true);expect(new Ajv({strict:true}).compile(schema.definitions.Request)(fixture.forgedResult)).toBe(false);
 expect(oracle(plan)).toBe(true);expect(readPaidProcessCustodianPlanV1(plan)).toEqual(plan);
 for(const value of[{...plan,extra:true},{...plan,parallelism:0}]){expect(oracle(value)).toBe(false);expect(()=>readPaidProcessCustodianPlanV1(value)).toThrow();}
 expect(()=>readPaidProcessCustodianPlanV1({...plan,jobs:[{...job,parallelism:fixture.invalid.jobParallelism}]})).toThrow(/reservation/);
 const oversized={...plan,jobs:Array.from({length:3},()=>({...job,args:Array.from({length:64},()=>"é".repeat(4096))}))};expect(oracle(oversized)).toBe(true);expect(new TextEncoder().encode(JSON.stringify(oversized)).length).toBeGreaterThan(schema.definitions.Protocol.const.maximumPlanBytes);expect(()=>readPaidProcessCustodianPlanV1(oversized)).toThrow(/UTF8/);expect(oracle({...plan,jobs:[{...job,program:"x".repeat(257)}]})).toBe(false);
 let accepted:Promise<unknown>|undefined,bodyReturned=false,settled=false,escaped:PaidProcessCustodianPortV1|undefined,directScope:Promise<unknown>|undefined;
 try{
  const scope=withPaidProcessCustodianV1(original,plan,async port=>{escaped=port;accepted=port.run(0);await wait(ready);expect(await probe()).toBe("paid-busy");bodyReturned=true;return "drained";}).then(value=>{settled=true;return value;});directScope=scope;void scope.catch(()=>{});
  await wait(ready);while(!bodyReturned)await Bun.sleep(10);expect(settled).toBe(false);expect(await probe()).toBe("paid-busy");writeFileSync(gate,"");expect(await scope).toBe("drained");expect(await accepted).toMatchObject(fixture.expected);expect(await probe()).toBe("free");expect(()=>escaped!.run(0)).toThrow(/live|closed|retired|issuance/i);
  for(const mode of["overgrant","detached","replay","error","undefined"]){
   rmSync(ready);if(existsSync(gate))rmSync(gate);if(["replay","error","undefined"].includes(mode))writeFileSync(gate,"");
   const negative=await captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./🛑️cancel/📜️script.ts",import.meta.url)),directory,ready,gate,join(directory,"c"),mode],process.cwd(),{control,output:fixture.output,env:scriptInvocationEnvironment(original,process.env)});expect(negative.status,negative.stderr).toBe(0);const observation=JSON.parse(negative.stdout);expect(observation.failed).toBe(true);if(["error","undefined"].includes(mode))expect(observation.primaryPreserved).toBe(true);expect(await probe()).toBe("free");
  }
  rmSync(ready);rmSync(gate);const trigger=join(directory,"c"),cancelled=captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./🛑️cancel/📜️script.ts",import.meta.url)),directory,ready,gate,trigger],process.cwd(),{control,output:fixture.output,env:scriptInvocationEnvironment(original,process.env)});
  await wait(ready);expect(await probe()).toBe("paid-busy");writeFileSync(trigger,"");const cancellation=await cancelled;expect(cancellation.status,cancellation.stderr).toBe(0);expect(JSON.parse(cancellation.stdout).failed).toBe(true);expect(cancellation.signal).toBeNull();expect(await probe()).toBe("free");
  if(process.platform!=="win32"){
   rmSync(ready);rmSync(trigger);const crashed=captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./🛑️cancel/📜️script.ts",import.meta.url)),directory,ready,gate,trigger,"crash"],process.cwd(),{control,output:fixture.output,env:scriptInvocationEnvironment(original,process.env)});
   await wait(ready);const custodian=JSON.parse(readFileSync(ready,"utf8")).parent;process.kill(custodian,"SIGSTOP");
   try{writeFileSync(trigger,"");const crash=await crashed;expect(crash.status).toBeNull();expect(crash.signal).toBe("SIGKILL");expect(await probe()).toBe("paid-busy");}
   finally{process.kill(custodian,"SIGCONT");}
   while(await probe()==="paid-busy"){control.signal.throwIfAborted();await Bun.sleep(10);}expect(await probe()).toBe("free");
  }
  rmSync(ready);const serverReady=join(directory,"s"),forgeryGate=join(directory,"t"),forged=captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./🎭️forged/📜️script.ts",import.meta.url)),JSON.stringify({...plan,jobs:[job]}),serverReady,forgeryGate],process.cwd(),{control,output:fixture.output,env:scriptInvocationEnvironment(original,process.env)});
  await wait(serverReady);expect(await probe()).toBe("paid-busy");expect(existsSync(ready)).toBe(false);writeFileSync(forgeryGate,"");const forgedResult=await forged;expect(forgedResult.status,forgedResult.stderr).toBe(0);expect(forgedResult.stdout).toBe("forged-wire-settled-without-local-spawn");expect(existsSync(ready)).toBe(false);expect(await probe()).toBe("free");
  rmSync(serverReady);rmSync(forgeryGate);const missing=join(directory,"missing");expect(existsSync(missing)).toBe(false);const serverFailure=captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./🎭️forged/📜️script.ts",import.meta.url)),JSON.stringify({...plan,jobs:[{...job,program:missing}]}),serverReady,forgeryGate,"failure"],process.cwd(),{control,output:fixture.output,env:scriptInvocationEnvironment(original,process.env)});await wait(serverReady);expect(await probe()).toBe("paid-busy");writeFileSync(forgeryGate,"");const actualFailure=await serverFailure;expect(actualFailure.status,actualFailure.stderr).toBe(0);expect(actualFailure.stdout).toBe("actual-custodian-original-failed-after-settlement");expect(await probe()).toBe("free");
  rmSync(serverReady);const idle=independent([fileURLToPath(new URL("./⏱️deadline/📜️script.ts",import.meta.url)),directory,serverReady]);await wait(serverReady,idle);expect(await probe()).toBe("paid-busy");const deadline=await idle;expect(deadline.status,deadline.stderr).toBe(0);const idleFacts=JSON.parse(deadline.stdout);expect(idleFacts).toMatchObject({failed:true,ready:true});expect(idleFacts.deadline-idleFacts.started).toBe(fixture.idleDeadlineMilliseconds);expect(idleFacts.readyAt).toBeLessThan(idleFacts.deadline);expect(idleFacts.finished).toBeGreaterThanOrEqual(idleFacts.deadline);expect(await probe()).toBe("free");
  rmSync(serverReady);const blocker=await acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"},directory,control,environment:{}});
  try{const waiting=await independent([fileURLToPath(new URL("./⏱️deadline/📜️script.ts",import.meta.url)),directory,serverReady]);expect(waiting.status,waiting.stderr).toBe(0);const waitingFacts=JSON.parse(waiting.stdout);expect(waitingFacts).toMatchObject({failed:true,ready:false,readyAt:null});expect(waitingFacts.deadline-waitingFacts.started).toBe(fixture.idleDeadlineMilliseconds);expect(waitingFacts.finished).toBeGreaterThanOrEqual(waitingFacts.deadline);expect(existsSync(serverReady)).toBe(false);}finally{blocker.release();}
 }finally{for(const path of[gate,join(directory,"c"),join(directory,"t")])writeFileSync(path,"");await Promise.allSettled([...operations,...(directScope?[directScope]:[])]);rmSync(directory,{recursive:true,force:true});}
}),120000);
