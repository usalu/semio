import ts from "typescript";
import {readFileSync} from "node:fs";
import {hostCapacityErrorV1,isCurrentHostGrantV1,isHostCapacityV1,requireHostCapacityV1,readHostCapacityPolicyV1,readHostCapacityPlanV1,resolveHostCapacityLanguageV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts";
import {checkResourceLeaseControl} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🎛️control/🟦️.ts";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json";
import fixture from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧭️liveness/🔣️.json";
import policyFixture from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json";
const policy=readHostCapacityPolicyV1({...policyFixture.policy,wait:{...policyFixture.policy.wait,staleMilliseconds:fixture.staleMilliseconds}});
const source=readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts","utf8"),tree=ts.createSourceFile("host-gate.ts",source,ts.ScriptTarget.Latest,true),selected=tree.statements.filter((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="withHostCapacityV1");
const code=ts.transpileModule(selected[0]!.getText(tree).replace(/^export /u,"")+"\nreturn withHostCapacityV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
const control={signal:new AbortController().signal,remainingMilliseconds:()=>20000,publish:async()=>undefined,yieldContinuation:async()=>undefined};
let ok=0;
for(const row of fixture.cases.filter(row=>!["invalid-id-path","observation-authority"].includes(row.id))){
 const input=row.input as any,fallback=Error("ordinary admission reached"),protocol=schema.definitions.Protocol.const;let bodies=0,queue=0,reads=0;
 const ports={protocol,schema,hostCapacityErrorV1,requireHostCapacityV1,isHostCapacityV1,isCurrentHostGrantV1,resolveHostCapacityLanguageV1,readHostCapacityPlanV1,prepare:(path:string)=>path,grantPath:()=>"/model/grant.json",readJson:()=>{reads++;return input.grant;},modified:()=>input.modifiedAtMs,alive:()=>input.pidAlive,bootAt:()=>input.bootAtMs,checkResourceLeaseControl,names:()=>{queue++;throw fallback;}};
 const run=new Function(...Object.keys(ports),code)(...Object.values(ports));let result:unknown,failure:unknown;
 try{result=await run({directory:"/model",policy,request:policyFixture.cases[0]!.queue[0]!.request,control,environment:{SEMIO_LANGUAGE:"en",[protocol.grantEnvironment]:input.expectedId},write:()=>{},now:()=>input.nowMs,pid:12345},async(handle:{inherited:boolean})=>{bodies++;return"inherited";});}catch(error){failure=error;}
 const good=reads===1&&bodies===Number(row.expected)&&queue===Number(!row.expected)&&(row.expected?result==="inherited"&&failure===undefined:failure===fallback);
 console.log(row.id,good?"ok":"FAIL",{reads,bodies,queue,failure:String(failure)});if(good)ok++;
}
console.log(ok,"of",fixture.cases.length-2);
