import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {EventEmitter} from "node:events";
import {dirname,join,resolve} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,withScriptInvocationSignalV1,requireIssuedScriptInvocationV1} from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🛑️ Checks real browser orchestrator bodies with genuinely issued cancellation and explicit process ports. */
export function registerBrowserCancellationTestsV1(it:(name:string,body:()=>Promise<void>)=>unknown,workspace:string):void{
 const oracle=new Ajv({strict:false});assert.equal(oracle.validate(schema,fixture),true,JSON.stringify(oracle.errors));assert.deepEqual(validateJsonSchemaSubset(schema,fixture),[]);
 for(const owner of fixture.owners)for(const row of fixture.cases)it(owner.id+" browser lifetime "+row.id,async()=>{
  const file=resolve(workspace,owner.source),text=readFileSync(file,"utf8"),tree=ts.createSourceFile(file,text,ts.ScriptTarget.Latest,true),declaration=tree.statements.find(node=>(ts.isFunctionDeclaration(node)||ts.isClassDeclaration(node))&&node.name?.text===owner.entry);if(!declaration)throw Error("Actual browser orchestrator required");
  const code=ts.transpileModule(declaration.getText(tree).replace(/^export\s+/u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
  let recorded=false;
  try{await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"browser-cancellation-law",maximumElapsedMilliseconds:15000},{},Date.now()),async original=>{
   const caller=new AbortController(),reason=Object.freeze({case:row.id}),host=Object.assign(new EventEmitter(),{env:{},exitCode:0}),budget=Object.freeze({readyTimeoutMs:1000,progressIntervalMs:10});
   let entered!:(value?:void)=>void,readyResolve!:(url:string)=>void,readyReject!:(reason:unknown)=>void,exitResolve!:(status:number)=>void,readySignal:AbortSignal|undefined,toolSignal:AbortSignal|undefined,kills=0,failed=false,innerFailure:unknown,toolCleanup:(()=>void)|undefined;
   const phase=new Promise<void>(done=>{entered=done;}),ready=new Promise<string>((done,fail)=>{readyResolve=done;readyReject=fail;}),exit=new Promise<number>(done=>{exitResolve=done;});ready.catch(()=>undefined);
   const child={kill:()=>{kills++;exitResolve(143);return true;}};
   const operation=withScriptInvocationSignalV1(original,caller.signal,async invocation=>{
    requireIssuedScriptInvocationV1(invocation);
    class Frame{root=dirname(file);repoRoot=workspace;invocation=invocation;}
    const ports={
     BundleScript:Frame,AbortController,AbortSignal,process:host,console:{log:()=>undefined},join,resolve,readinessBudget:budget,
     DEMONSTRATOR_E2E_OWNER:"fixture",PLAY_E2E_OWNER:"fixture",demonstratorE2eSessionRoot:()=>"/session",playE2eSessionRoot:()=>"/session",demonstratorE2eInvocationPid:()=>1,playE2eInvocationPid:()=>1,readServiceSession:()=>({id:"fixture"}),
     parseServiceReadinessBudgetV1:(value:unknown)=>{assert.equal(value,budget);return budget;},serviceReadyProgressLanguage:()=>"en",serviceReadyProgressLine:()=>"progress",repoCacheDirectory:()=>"/browsers",demonstratorE2eOutputDirectory:()=>"/artifacts",
     waitForServiceReady:(_root:unknown,_session:unknown,signal:AbortSignal)=>{
      readySignal=signal;const abort=()=>readyReject(signal.reason);signal.addEventListener("abort",abort,{once:true});void ready.then(()=>signal.removeEventListener("abort",abort),()=>signal.removeEventListener("abort",abort));
      if(signal.aborted)abort();if(row.phase==="ready")entered();else readyResolve("http://fixture");return ready;
     },
     startToolV1:(_tool:unknown,_args:unknown,options:{signal?:AbortSignal})=>{
      requireIssuedScriptInvocationV1(invocation);invocation.control.signal.throwIfAborted();toolSignal=options.signal;const abort=()=>child.kill();toolSignal?.addEventListener("abort",abort,{once:true});toolCleanup=()=>toolSignal?.removeEventListener("abort",abort);if(toolSignal?.aborted)abort();entered();return child;
     },
     toolExitV1:(value:unknown)=>{assert.equal(value,child);return exit;}
    };
    const run=new Function(...Object.keys(ports),code+";return "+owner.entry+";")(...Object.values(ports));
    return owner.kind==="function"?run({root:dirname(file),repoRoot:workspace,invocation},[]):new run().run([]);
   });operation.catch(()=>undefined);
   try{
    await Promise.race([phase,operation.then(()=>{throw Error("Browser operation ended before requested phase");})]);
    if(row.trigger==="caller")caller.abort(reason);else if(row.trigger!=="none")host.emit(row.trigger);
    await Promise.resolve();const stopped=row.phase==="ready"?readySignal?.aborted===true:kills>0;
    readyResolve("http://fixture");exitResolve(row.status);
    try{await operation;}catch(error){failed=true;innerFailure=error;}
    if(row.trigger==="caller")assert.equal(innerFailure,reason);
    assert.equal(host.listenerCount("SIGINT")+host.listenerCount("SIGTERM"),0);
    const actual={stop:stopped,exitCode:host.exitCode,fails:failed,kills,toolSignal:toolSignal instanceof AbortSignal},contract={...schema.definitions.Result,properties:Object.fromEntries(Object.entries(row.expected).map(([key,value])=>[key,{const:value}]))};
    assert.equal(oracle.validate(contract,actual),true,JSON.stringify({owner:owner.id,case:row.id,actual,errors:oracle.errors}));assert.deepEqual(validateJsonSchemaSubset(contract,actual),[]);assert.deepEqual(actual,row.expected);recorded=true;
   }finally{readyResolve("http://fixture");exitResolve(0);toolCleanup?.();await operation.catch(()=>undefined);}
  });}catch(error){if(!recorded)throw error;if(!row.expected.fails)throw error;}
  assert.equal(recorded,true);
 });
}
