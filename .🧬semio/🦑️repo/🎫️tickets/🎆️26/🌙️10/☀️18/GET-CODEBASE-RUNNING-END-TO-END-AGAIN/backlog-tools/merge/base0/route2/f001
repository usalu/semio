import {strict as assert} from "node:assert";
import {writeFile} from "node:fs/promises";
import {resolve,relative,basename} from "node:path";
import {receiveScriptProcessInvocation,readIssuedScriptProcessEnvelopeV1,scriptProcessEnvironment,requireIssuedScriptInvocationV1,createIssuedResourceControlV1} from "../../../../../../../../\ud83d\udd28\ufe0fmodules/\ud83c\udfc3\ufe0fprocess/\ud83d\udce6\ufe0fpackages/\ud83d\udfe6\ufe0ftypescript/\ud83e\udded\ufe0frouting/\ud83d\udfe6\ufe0f.ts";
import {captureOwnedCommandV1} from "@semio-tech/process/execution";
import {projectOwnedNxProcessEnvelopeV1} from "../../../🔧️tools/🧶️nx/📨️envelope/🟦️.ts";
import fixture from "./🔣️.json";

const root=process.env.SEMIO_WORKSPACE_ROOT!,receipt=resolve(process.argv[2]??""),artifact=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??"");
if(process.argv.length!==3||!root||basename(receipt)!=='birth.json'||!relative(artifact,receipt)||relative(artifact,receipt).startsWith('..'))throw new Error('Owned prebirth receipt path required');
await receiveScriptProcessInvocation(process.env,async original=>{
 const before=readIssuedScriptProcessEnvelopeV1(original),control=original.control,caps=original.capabilities,selected=fixture.childArguments,child=projectOwnedNxProcessEnvelopeV1(root,original,selected),env=scriptProcessEnvironment(child,process.env);
 requireIssuedScriptInvocationV1(original);assert.equal(original.control,control);assert.equal(original.capabilities,caps);assert.equal(child.deadlineEpochMilliseconds,before.deadlineEpochMilliseconds);assert.deepEqual((child.capabilities as any).native,(caps as any).native);assert.throws(()=>projectOwnedNxProcessEnvelopeV1(root,{...original},selected),/Live Original/);assert.throws(()=>projectOwnedNxProcessEnvelopeV1(root,original,['reset']),/Undeclared/);
 const program="const x=JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION);console.log(JSON.stringify({policy:x.policy,deadline:x.deadlineEpochMilliseconds,capabilities:x.capabilities,argv:process.argv.slice(1)}))",limit=(caps as any).native.child.maximumElapsedMilliseconds,output=JSON.parse(await captureOwnedCommandV1('node',['-e',program,'nx',...selected],root,{env,control:createIssuedResourceControlV1(original,'fixture:domain-node',limit===0?5000:Math.min(limit,5000)),output:{maximumBytes:8192,maximumLines:32}}));
 const funded=Math.min(limit===0?5000:limit,5000);assert.deepEqual({...output.policy,maximumElapsedMilliseconds:0},{...before.policy,maximumElapsedMilliseconds:0});assert.ok(output.policy.maximumElapsedMilliseconds>0&&output.policy.maximumElapsedMilliseconds<=funded);assert.ok(output.deadline===null||typeof output.deadline==="number"&&output.deadline-Date.now()<=funded);assert.deepEqual(output.capabilities,JSON.parse(JSON.stringify(child.capabilities)));assert.deepEqual(output.argv,['nx',...selected]);requireIssuedScriptInvocationV1(original);assert.equal(readIssuedScriptProcessEnvelopeV1(original),before);
 await writeFile(receipt,JSON.stringify({version:1,policy:before.policy,deadline:before.deadlineEpochMilliseconds,native:(caps as any).native,node:output,originalUnchanged:true,forgedRefused:true,undeclaredRefused:true}));
});
