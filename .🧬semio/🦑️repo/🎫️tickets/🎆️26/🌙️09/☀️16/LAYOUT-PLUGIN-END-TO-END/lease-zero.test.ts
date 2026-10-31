import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1,createRequestedIssuedResourceControlV1,requireIssuedScriptInvocationV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {acquireResourceLease,resourceLeasePathV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import corpus from "./lease-zero.json";
import schema from "./lease-zero.schema.json";
declareBuildTestEnvelopeV1({owner:"test:lease-zero-admission",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});
const configured=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!configured)throw Error("Task artifact directory required");
const root=mkdtempSync(join(resolve(configured),"z-"));process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=join(root,"r");
afterAll(()=>rmSync(root,{recursive:true,force:true}));
test("closed portable zero-timeout vectors agree with independent Ajv and SQLite",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const db=new Database(":memory:");try{for(const row of corpus.cases){const expected=db.query("select case when ?1 then 'cancelled' when ?2 then 'busy' else 'granted' end outcome").get(Number(row.cancelled),Number(row.held)) as {outcome:string};expect(expected.outcome).toBe(row.expected);}}finally{db.close();}
});
for(const [index,row]of corpus.cases.entries())test(row.id,()=>withBuildTestInvocationV1(async original=>{
 requireIssuedScriptInvocationV1(original);const control=createIssuedResourceControlV1(original,"fixture:lease-zero",0),directory=join(root,String(index)),resource="lease-zero";mkdirSync(directory);
 const file=resourceLeasePathV1(directory,resource);expect(file.length).toBeLessThanOrEqual(256);
 const coldBegan=performance.now(),warm=await acquireResourceLease({directory,resource,mode:"exclusive",control,signal:control.signal,timeoutMs:0});warm.release();const coldSetupMs=performance.now()-coldBegan;
 const oracle=new Database(file);oracle.exec("PRAGMA busy_timeout=0");
 let held=false;try{if(row.held){oracle.exec("BEGIN");expect(oracle.query("SELECT resource FROM semio_resource_lease WHERE id=1").get()).toEqual({resource});held=true;}else{oracle.exec("BEGIN EXCLUSIVE");oracle.exec("ROLLBACK");}
 const cancel=new AbortController(),reason=Error("fixture cancellation"),selected=createRequestedIssuedResourceControlV1(control,0,cancel.signal);if(row.cancelled)cancel.abort(reason);
 const began=performance.now();let outcome="granted",failure:unknown,lease:Awaited<ReturnType<typeof acquireResourceLease>>|undefined;
 try{lease=await acquireResourceLease({directory,resource,mode:"exclusive",control:selected,signal:selected.signal,timeoutMs:0});}catch(error){failure=error;outcome=error===reason?"cancelled":"busy";}finally{lease?.release();}
 console.log("[DEBUG] "+JSON.stringify({id:row.id,outcome,coldSetupMs,elapsedMs:performance.now()-began,error:failure instanceof Error?failure.message:null,admission:failure instanceof Error?Object.getOwnPropertyDescriptor(failure,"resourceLeaseAdmission")?.value:null}));
 expect(outcome).toBe(row.expected);if(row.expected==="busy")expect((failure as Error).message).toBe("Resource lease timed out: "+resource);if(row.cancelled)expect(failure).toBe(reason);
 }finally{if(held)oracle.exec("ROLLBACK");oracle.close();}
}));
