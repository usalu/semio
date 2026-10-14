import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFile,mkdir,writeFile} from "node:fs/promises";
import {isAbsolute,join,resolve} from "node:path";

const root=process.env.SEMIO_WORKSPACE_ROOT,ticket=resolve(import.meta.dir,".."),output=join(ticket,"🗑️generated");
if(!root||!isAbsolute(root))throw Error("Original absolute workspace required");
const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
if(!process.argv.includes("--child")){
 if(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION!==undefined)throw Error("Original standalone measurement required");
 const {runOwnedCommand}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"));
 await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"native-memo-measurement",maximumElapsedMilliseconds:60000},{workspaceRoot:root},Date.now()),original=>runOwnedCommand(process.execPath,[import.meta.path,"--child"],root,"native-memo-measurement",Math.floor(original.control.remainingMilliseconds()!),{signal:original.control.signal,env:routing.scriptInvocationEnvironment(original,process.env),onProgress:async()=>routing.advanceScriptInvocation(original,"native-memo-measurement","running")}));
}else await routing.receiveScriptProcessInvocation(process.env,async original=>{
 const library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),api=(await import(join(library,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"))).cacheInternals,{readBuildOwnerRegistryV1}=await import(join(library,"🏗️build/🕸️graph/🚪️admission/🟦️.ts")),registry=readBuildOwnerRegistryV1(JSON.parse(await readFile(join(root,"📋️build-owners.json"),"utf8"))),owner=registry.owners.find(row=>row.name==="@semio-tech/animate-plugin");
 assert(owner);const seed=api.nativeDependencyRoots(owner.root,root,true,new Map(),new Map()),roots=[...new Set([owner.root,...[0,40,80,120,160,200].filter(index=>index<seed.length).map(index=>seed[index])])],vectors=roots.flatMap(root=>[false,true].map(tests=>({root,tests})));
 const runs=[];
 for(const shared of [false,true,true,false]){
  const manifests=new Map(),closures=new Map(),results=[],started=performance.now();
  for(const vector of vectors){await routing.advanceScriptInvocation(original,"native-memo-measurement","running");const began=performance.now(),members=api.nativeDependencyRoots(vector.root,root,vector.tests,shared?manifests:new Map(),shared?closures:new Map());results.push({...vector,members,milliseconds:performance.now()-began});}
  runs.push({shared,milliseconds:performance.now()-started,results});
 }
 for(const run of runs.slice(1))assert.deepEqual(run.results.map(({root,tests,members})=>({root,tests,members})),runs[0].results.map(({root,tests,members})=>({root,tests,members})));
 const receipt={scope:"actual-native-closure-map-scope-measurement",maximumElapsedMilliseconds:original.policy.maximumElapsedMilliseconds,owner:owner.name,seedMembers:seed.length,vectors:vectors.length,equal:true,orderPairs:[{freshMilliseconds:runs[0].milliseconds,sharedMilliseconds:runs[1].milliseconds},{freshMilliseconds:runs[3].milliseconds,sharedMilliseconds:runs[2].milliseconds}],inferenceSha256:createHash("sha256").update(await readFile(join(library,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"))).digest("hex"),runs};
 await mkdir(output,{recursive:true});await writeFile(join(output,"native-memo-measurement.json"),JSON.stringify(receipt,null,2)+"\n");console.log("[DEBUG] Native memo measurement "+JSON.stringify({...receipt,runs:undefined}));
});

