/** 🧪️ Bounds actual late closure queries over one owner's authored targets. */
import fs from "node:fs";import path from "node:path";import {pathToFileURL} from "node:url";import {syncBuiltinESMExports} from "node:module";
const started=performance.now(),budget=30000,counts=new Map(),rows=[];let current;
const finish=reason=>{console.log(JSON.stringify({reason,milliseconds:Math.round(performance.now()-started),current,counts:Object.fromEntries(counts),rows}));process.exit(reason==="complete"?0:2)};
for(const name of ["readFileSync","statSync","lstatSync","existsSync","readdirSync"]){const original=fs[name];fs[name]=function(...args){counts.set(name,(counts.get(name)??0)+1);if(performance.now()-started>budget)finish("bounded");return original.apply(this,args)}}syncBuiltinESMExports();
const timer=setTimeout(()=>finish("bounded"),budget),workspace=process.cwd(),base="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",load=p=>import(pathToFileURL(path.join(workspace,base,p)));
let invocation,processApi;try{const run=async()=>{
 const api=await load("🕸️graph/📥️inference/🟨️.mjs");await api.libraryBootstrap;
 const observation=await load("👁️observation/🟨️.mjs"),scripts=await load("🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),args=await load("📤️arguments/🟨️.mjs"),root=process.argv[2],cache=api.createScriptInputCache({workspaceRoot:workspace,invocation,phaseControl:processApi.createIssuedResourceControlV1(invocation,"build:command-source-imports",0)});cache.lateClosures=true;
 const closures=scripts.closureApiV1(cache,workspace),project=JSON.parse(fs.readFileSync(path.join(root,"📋️project.json"),"utf8"));
 await observation.observeInferenceSnapshotV1(async()=>{for(const [name,authored]of Object.entries(project.targets)){
  current=name;const at=performance.now(),before=Object.fromEntries(counts),target=structuredClone(authored);
  if(target.metadata?.semio?.execution?.kind!=="source"){
   const manifest=root+"/Cargo.toml",cwd=target.options.cwd??root,bound=args.bindBuildCommandV1(target.options.command,workspace,cwd,root,[]).command,owner=args.declaredNativeOwnerCommandV1(manifest,cwd,bound),driver="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
   target.options={...target.options,ownerArguments:true,cwd:".",nativeOwnerCommand:owner,command:"repo "+JSON.stringify(driver)+" native owner-command --manifest "+JSON.stringify(manifest)+" --cwd "+JSON.stringify(cwd)+" -- "+bound};
  }
  const result=await observation.captureObservedSourceInputsV1(async()=>{const list=await closures.nativeTargetCommandInputs(target,undefined,root);await closures.some(list,["/🏃️process/🧪️testing/🦀️cargo/"]);await closures.query(list,"workspaceKeys",target.options.command);return closures.generatedDependencies(target,root,root+"/Cargo.toml")},undefined,false);
  const row={name,milliseconds:Math.round(performance.now()-at),observations:result.observations.length,modules:cache.modules.size,reads:(counts.get("readFileSync")??0)-(before.readFileSync??0),stats:(counts.get("statSync")??0)-(before.statSync??0)};rows.push(row);console.log(JSON.stringify(row));
 }});
 };processApi=await import(pathToFileURL(path.join(workspace,"🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts")));await processApi.withScriptProcessEnvelope(processApi.createScriptProcessEnvelope({version:1,owner:"repository:build",maximumElapsedMilliseconds:30000},{fixture:"provider-issued-cost"},Date.now()),async original=>{invocation=original;await run()});clearTimeout(timer);finish("complete");
}catch(error){clearTimeout(timer);console.log(JSON.stringify({error:String(error?.message??error),stack:error?.stack}));finish("refused");}
