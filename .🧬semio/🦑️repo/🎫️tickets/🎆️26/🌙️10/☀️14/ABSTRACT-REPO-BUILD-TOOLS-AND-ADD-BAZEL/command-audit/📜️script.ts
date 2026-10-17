import assert from "node:assert/strict";
import type{ScriptInvocation}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { readFileSync, existsSync, writeFileSync } from "node:fs";
import { isAbsolute, resolve, join, dirname } from "node:path";
import { createHash } from "node:crypto";
import { pathToFileURL } from "node:url";
import {withQualificationGraphV1} from "../plan/📜️script.ts";
if(process.argv[2]==="--source"||process.argv[2]==="--strict"){
 const root=process.cwd(),artifacts=resolve(import.meta.dir,"../🗑️generated"),mode=process.argv[2];assert(process.argv.length===3&&process.env.SEMIO_WORKSPACE_ROOT===root&&process.env.SEMIO_TEST_ARTIFACT_DIR===artifacts&&process.env.SEMIO_SCRIPT_PROCESS_INVOCATION===undefined,"Standalone audit Source entry required");const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),{runOwnedCommand}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"));
 await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"command-audit-source",maximumElapsedMilliseconds:60000},{workspaceRoot:root},Date.now()),async(original:ScriptInvocation)=>{const env=routing.scriptInvocationEnvironment(original,{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"}),options={env,signal:original.control.signal,onProgress:()=>routing.advanceScriptInvocation(original,"command-audit:source","running")};if(mode==="--strict"){const{createRequire}=await import("node:module"),require=createRequire(join(root,"package.json"));await runOwnedCommand(process.execPath,[require.resolve("typescript/lib/tsc.js"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--module","preserve","--moduleResolution","bundler","--target","esnext","--lib","esnext,dom,dom.iterable,dom.asynciterable","--types","bun","--pretty","false",import.meta.filename,join(import.meta.dir,"🧪️tests/🟦️.ts")],root,"command-audit:strict",routing.scriptInvocationBudget(original,0),options);}await runOwnedCommand(process.execPath,["test","./"+join(import.meta.dir,"🧪️tests/🟦️.ts").slice(root.length+1)],root,"command-audit:source",routing.scriptInvocationBudget(original,0),options);console.log("[DEBUG] Command audit Source Original "+JSON.stringify({maximumElapsedMilliseconds:original.policy.maximumElapsedMilliseconds,strict:mode==="--strict",nativeEffects:0}));});
}else await withQualificationGraphV1(async({root,artifacts,library,original,graph,pin,check,progress})=>{
const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const digest=pin.digest;
const failures:unknown[]=[],dynamic:unknown[]=[],seen=new Set<string>();
check();
const ownerManifest=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json"),oraclePath=await Bun.resolve("string-argv",dirname(ownerManifest));
const {parseArgsStringToArgv}=await import(pathToFileURL(oraclePath).href);
let targets=0,configurations=0,continuous=0,commandRecipes=0,scriptReferences=0,nestedCommandBoundaries=0;
const processScript=join(root,"🧰️framework/🔨️modules/🏃️process/📜️script.ts");
const{buildExecutionArgumentsV1}=await import(join(library,"🏗️build/📥️caller/🟦️.ts")),{resolveBuildTargetV1}=await import(join(library,"🏗️build/🕸️graph/🟦️.ts"));
const {parseCommandArgumentsV1,resolveCommandConfigurationV1}=await import(join(dirname(processScript),"🧭️routing/🎛️command/⚙️configuration/🟦️.ts"));
const inspect=(argv:string[],cwd:string,id:string,command:string)=>{
 check();
 const boundary=argv.indexOf("--"),wrapper=argv.findIndex(argument=>argument.endsWith("📜️script.ts")&&resolve(cwd,argument)===processScript);
 const owned=wrapper>=0&&argv[wrapper+1]==="command"&&boundary>wrapper;
 for(const argument of owned?argv.slice(0,boundary):argv)if(argument.endsWith("📜️script.ts")){
  scriptReferences++;if(/[${}]/u.test(argument)){dynamic.push({id,argument});continue;}
  const script=resolve(cwd,argument);if(!existsSync(script))failures.push({id,command,cwd,script,error:"Missing literal script"});
 }
 if(owned){
  const options=argv.slice(wrapper+2,boundary),configuration=options.indexOf("--config"),owner=options.indexOf("--cwd");
  assert(configuration>=0&&owner>=0,"Owned command requires explicit configuration and owner");
  const configurationPath=resolve(cwd,options[configuration+1]!),value=JSON.parse(readFileSync(configurationPath,"utf8")),workspace=resolve(dirname(configurationPath),value.workspace),nested=resolve(workspace,options[owner+1]!);
  const selection=parseCommandArgumentsV1([...options.map((value,index)=>index===configuration+1?configurationPath:value),"--",...argv.slice(boundary+1)]);
  assert.equal(resolveCommandConfigurationV1(selection,{...process.env,SEMIO_BUILD_MODE:"prototype"},original).request.cwd,nested);
  if(!existsSync(nested))failures.push({id,cwd:nested,error:"Missing nested execution directory"});
  nestedCommandBoundaries++;inspect(argv.slice(boundary+1),nested,id,command);
 }
};
for(const project of Object.values(graph.projects) as any[])for(const [name,target]of Object.entries(project.targets)as any[]){
 targets++;check();if(targets%64===0)await progress();if(target.continuous)continuous++;
 for(const [configuration,overrides]of[["",{}],...Object.entries(target.configurations??{})]as any[]){
  if(configuration)configurations++;
  const execution=resolveBuildTargetV1(target,configuration||undefined).execution,expand=(value:string)=>value.replaceAll("{workspaceRoot}",root).replaceAll("{projectRoot}",project.root);
  if(execution.kind!=="command")continue;
  const cwd=resolve(root,execution.program===undefined?expand(execution.cwd??"."):execution.cwd??"."),id=project.name+":"+name+(configuration?":"+configuration:"");
  if(!existsSync(cwd))failures.push({id,cwd,error:"Missing execution directory"});
  const defaults=buildExecutionArgumentsV1({...execution,forwardAllArgs:true},[]),operations=execution.program===undefined?(execution.commands??[execution.command]).map((item:any)=>({command:typeof item==="string"?item:item?.command,forward:typeof item==="object"?item.forwardAllArgs??execution.forwardAllArgs:execution.forwardAllArgs})): [{command:execution.program,forward:execution.forwardAllArgs}];
  if(!operations.length||operations.some((item:any)=>!item.command))failures.push({id,error:"Missing command operation"});
  for(const{command,forward}of operations){if(!command)continue;const argv=execution.program===undefined?parseArgsStringToArgv(command):[command];if(!execution.ownerArguments&&forward!==false)argv.push(...defaults);const key=cwd+"\0"+JSON.stringify(argv);if(seen.has(key))continue;seen.add(key);commandRecipes++;
   inspect(argv,cwd,id,execution.program===undefined?command:JSON.stringify(argv));
  }
 }
}
check();
const result={originalMaximumElapsedMilliseconds:original.policy.maximumElapsedMilliseconds,digest,projects:Object.keys(graph.projects).length,dependencies:graph.dependencies.length,targets,configurations,continuous,commandRecipes,scriptReferences,nestedCommandBoundaries,dynamicReferences:dynamic.length,failures};
writeFileSync(join(artifacts,"graph-command-audit-"+process.pid+".json"),JSON.stringify(result,null,2)+"\n",{flag:"wx"});
console.log("[DEBUG] Canonical command audit "+JSON.stringify(result));assert.equal(failures.length,0);
});
