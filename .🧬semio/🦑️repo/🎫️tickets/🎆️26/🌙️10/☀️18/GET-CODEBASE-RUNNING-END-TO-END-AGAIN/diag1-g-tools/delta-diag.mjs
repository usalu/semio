// [asm-g] graph-free delta build of the REAL production code on a scratch copy of the sealed generation: real generationHostV1, real loadGenerationGraphV1, real discoverBuildGraphPartV1; only leases/store are scratch and the tree is "quiet" except the TOUCHed projects.
// usage: bun delta-build.mjs <scratch-gen-dir> <project> <target>   env: TOUCH=a,b  RSS_CAP_MB=3000  MARKS=1  LEGACY=1 (no tests/policy cache)

import{execFileSync}from"node:child_process";import{readFileSync as __rf}from"node:fs";
const __idx=new Set((process.env.DIAG_INDEX??"").split(",").filter(Boolean).map(r=>"/Users/ueli/Documents/semio/"+r));
const srcText=async path=>{if(__idx.has(path)){console.error("[DEBUG] index version: "+path.slice(-48));return execFileSync("git",["show",":"+path.slice("/Users/ueli/Documents/semio/".length)],{cwd:"/Users/ueli/Documents/semio",maxBuffer:1<<28}).toString("utf8");}return __rf(path,"utf8");};
import{readFileSync,mkdirSync}from"node:fs";import{join}from"node:path";
const ROOT="/Users/ueli/Documents/semio",B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",TEST=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs",GRAPH=B+"🕸️graph/🟦️.ts",GENS=B+"🕸️graph/🕰️generations/🟨️.mjs",STOR=B+"📥️inputs/💾️storage/🟨️.mjs",INVK=B+"🕸️graph/📥️inference/🚦️invocation/🟨️.mjs",E=process.env;
const D=process.argv[2],PROJECT=process.argv[3],TARGET=process.argv[4]??"test";
process.chdir(ROOT);
const mb=n=>Math.round(n/1048576),t0=performance.now();let peak=0,last=t0;
const sample=()=>{const r=process.memoryUsage().rss;if(r>peak)peak=r;if(r>Number(E.RSS_CAP_MB??"3000")*1048576){console.error("[DEBUG] ABORT rss "+mb(r));process.exit(3);}};setInterval(sample,100).unref();
const marks=[];globalThis.__asmMark=label=>{sample();const now=performance.now(),row={label,ms:Math.round(now-last),t:Math.round(now-t0),rss:mb(process.memoryUsage().rss),peak:mb(peak)};last=now;marks.push(row);console.error("[DEBUG] "+JSON.stringify(row));};
const must=(text,anchor,make,label)=>{const at=text.indexOf(anchor);if(at<0||text.indexOf(anchor,at+1)>=0&&!label?.includes("*")){console.error("[DEBUG] anchor "+(at<0?"missing":"ambiguous")+": "+label);return text;}return text.slice(0,at)+make(anchor)+text.slice(at+anchor.length);};
Bun.plugin({name:"asm",setup(b){
 b.onLoad({filter:/modules\/[^/]*test\/[^/]*\.mjs$/},async args=>{if(args.path!==TEST)return undefined;const t=await srcText(args.path);return{contents:t+"\nexport const __asm={testCaseProjects,testCaseDependencies,testShared};\n",loader:"js"};});
 b.onLoad({filter:/invocation\/[^/]*\.mjs$/},async args=>{if(args.path!==INVK)return undefined;let t=await srcText(args.path);
  const a="return context.invocation?run(context.invocation,context.phaseControl):receiveScriptProcessInvocation(";
  if(!t.includes(a)){console.error("[DEBUG] invocation anchor missing");return undefined;}
  return{contents:t.replace(a,"return true?run(undefined,undefined):receiveScriptProcessInvocation("),loader:"js"};});
 b.onLoad({filter:/storage\/[^/]*\.mjs$/},async args=>{if(args.path!==STOR)return undefined;let t=await srcText(args.path);
  const a="export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return withGraphFactWriterV1(workspace,invocation,phaseControl,body);}";
  if(!t.includes(a)){console.error("[DEBUG] storage anchor missing");return undefined;}
  return{contents:t.replace(a,"export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return body({},phaseControl);}"),loader:"js"};});
 b.onLoad({filter:/graph\/[^/]*\.ts$/},async args=>{if(args.path!==GRAPH)return undefined;let t=await srcText(args.path);
  const start=t.indexOf("export async function discoverBuildGraphPartV1("),end=t.indexOf("/** 🧺️ Keeps exactly the discovered files",start);let part=t.slice(start,end);
  part=must(part,"requireIssuedResourceControlInvocationV1(phaseControl,invocation);",()=>"","guard");
  part=must(part,"context={workspaceRoot:root,invocation,phaseControl,registry}",()=>"context={workspaceRoot:root,registry,preparationControl:{advance:async()=>{}}}","ctx");
  part=must(part,"invocation,phaseControl,selection:{kind:\"all\"}",()=>"selection:{kind:\"all\"},preparationControl:{advance:async()=>{}}","gctx");
  if(E.FORCE_ALL==="1")part=must(part,"const incremental=!retained&&",()=>"const incremental=false&&","force");
  if(E.NO_RETAIN==="1")part=must(part,"const tagged=stubs.filter(([,stub])=>stub.tag),retained=taggedOwnersRetainedV1(",()=>"const tagged=stubs.filter(([,stub])=>stub.tag),retained=false&&taggedOwnersRetainedV1(","noretain");
  const M=l=>a=>'globalThis.__asmMark?.("'+l+'");'+a;
  part=must(part,"const projects:Record<string,any>={},externalNodes",M("part:primary"),"primary");
  part=must(part,"const tagged=stubs.filter(([,stub])=>stub.tag)",M("part:env"),"env");
  part=must(part,"const policyOwners=Object.assign(",M("part:tests"),"policy");
  part=must(part,"merge(testContributions as [string,any][],false);",M("part:policies"),"merge");
  part=must(part,"const edges=[...await owner.discoverBuildDependenciesV1(",M("part:testedges"),"edges");
  part=must(part,"const metadata=await owner.createMetadata(",M("part:edges"),"metadata");
  part=must(part,"return {...captured.value,rows:",M("part:captured"),"ret");
  part=part.split('import("../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs")').join("Promise.resolve(globalThis.__asmTestModule)");
  t=t.slice(0,start)+part+t.slice(end)+"\nexport const __asm={generationHostV1,discoverBuildGraphPartV1};\n";
  return{contents:t,loader:"ts"};});
 b.onLoad({filter:/generations\/[^/]*\.mjs$/},async args=>{if(args.path!==GENS)return undefined;let t=await srcText(args.path);const M=l=>a=>'globalThis.__asmMark?.("'+l+'");'+a;
  t=must(t,"const result=await compute(spec,signal);",a=>M("build:plan+start")(a)+'globalThis.__asmMark?.("build:compute-start");',"compute");
  t=must(t,"stats.computed+=result.owners.length;",M("build:compute"),"computed");
  t=must(t,"for(const name of names)if(!returned.has(name)&&!dropped.has(name))",M("build:fragments"),"returned");
  t=must(t,"let index=cold?createFactIndexV1(upserts)",M("build:owners"),"index");
  t=must(t,"const orphans=orphanedDerivedOwnersV1(index);",M("build:index-patch"),"orph");
  t=must(t,"const globalStored=await store(P.fragments",M("build:index-pack"),"global");
  t=must(t,"const manifestDigest=await writeManifest(next);",M("build:frontier+global"),"manifest");
  t=must(t,"const outcome=await publish({manifestDigest,expectedGeneration,verified,signal});",M("build:manifest"),"publish");
  t=must(t,"indexCache.set(indexStored.digest,index);\n  await retire",M("build:publish"),"retire*");
  t=must(t,"const index=await readIndexView(manifest.factIndex);if(!index)return cold(\"index\");",a=>a+M("plan:index")(""),"planidx");
  t=must(t,"const{stale}=await staleFactRowsV1(",M("plan:select"),"stale");
  return{contents:t,loader:"js"};});
}});

const __EXT="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🟦️.ts";
const __oldExt=process.env.OVERLAY_EXT==="old"?execFileSync("git",["show",":"+__EXT.slice("/Users/ueli/Documents/semio/".length)],{cwd:"/Users/ueli/Documents/semio",maxBuffer:1<<26}):undefined;
globalThis.__extOld=__oldExt!==undefined;
globalThis.__overlay={get:p=>globalThis.__extOld&&p===__EXT?__oldExt:undefined,set(){},epoch:()=>1,size:()=>1,calls:{read:0}};
const __here="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/cutoff-g-tools/";
const __shim=text=>__oldExt===undefined||!/["'](?:node:)?fs(?:\/promises)?["']/.test(text)?text:text.replace(/(["'])(?:node:)?fs\/promises\1/g,(_m,q)=>q+__here+"overlay-fsp-shim.mjs"+q).replace(/(["'])(?:node:)?fs\1/g,(_m,q)=>q+__here+"overlay-fs-shim.mjs"+q);
const __OBS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",__targets=new Set([...__idx,__OBS+"👁️observation/🟨️.mjs",__OBS+"🕸️graph/📥️inference/📍️coordinates/🟨️.mjs"]),__esc=s=>s.replace(/[.*+?^${}()|[\]\\]/g,"\\$&");
Bun.plugin({name:"diag-generic",setup(b){b.onLoad({filter:new RegExp("^(?:"+[...__targets].map(__esc).join("|")+")$")},async args=>{const text=await srcText(args.path);return{contents:/(?:👁️observation|📍️coordinates)\/🟨️\.mjs$/.test(args.path)?__shim(text):text,loader:args.path.endsWith(".ts")?"ts":"js"};});}});
const gen=await import(GENS),load=await import(B+"🕸️graph/🕰️generations/🔌️load/🟨️.mjs"),graph=await import(GRAPH),tests=await import(TEST);
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const admission=await import(B+"🕸️graph/🚪️admission/🟦️.ts"),revision=await import(B+"📥️inputs/🔏️revision/🟨️.mjs");
const bare=context=>({...context,invocation:undefined,phaseControl:undefined});
globalThis.__asmTestModule={...tests,default:tests.default,discoverTestSharedV1:context=>tests.__asm.testShared(bare(context)),discoverTestBuildProjectsV1:(files,options,context)=>tests.__asm.testCaseProjects(files,options,bare(context)),discoverTestBuildDependenciesV1:(options,context)=>tests.__asm.testCaseDependencies(options,bare(context)),danglingTestDependenciesV1:tests.danglingTestDependenciesV1};
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(ROOT,"📋️build-owners.json"),"utf8")));
const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>{},yieldContinuation:async()=>{}};
const invocation=Object.freeze({policy:Object.freeze({version:1,owner:"asm-g",maximumElapsedMilliseconds:0}),control:Object.freeze(control),capabilities:Object.freeze({})});
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
let touched=new Set((E.TOUCH??"").split(",").filter(Boolean).map(file=>ROOT+"/"+file));
const quietHolds=async rows=>{if(!touched.size)return true;return!rows.some(row=>["file","analysis","digest","lstat","stat"].includes(row.kind)&&touched.has(row.path));};
globalThis.__asmMark("loaded");
const storage=await import(STOR);
const STORE=E.STORE??"/private/tmp/asm-g/store";mkdirSync(STORE,{recursive:true});
const runOnce=()=>revision.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:STORE,published:()=>{}},async()=>{
 const host0=graph.__asm.generationHostV1(ROOT,invocation,control,registry,true),store={root:D,recheck:async()=>{}},opts=host0.storeOptions(store,control);
 const host={...host0,prefetch:undefined,withShared:body=>body(store,control),withBuilder:body=>body(store,control),
  storeOptions:()=>({...opts,root:D,fs:gen.createNodeFilePortV1(),lease:undefined,holds:quietHolds,derive:undefined}),
  membership:async(previous,signal)=>{if(!previous)throw Error("[asm-g] cold build refused");const live=await host0.membership(previous,signal);return{...live,changes:[]};},
  expand:async projects=>projects,say:(key,fields)=>console.error("[DEBUG] say "+key+" "+JSON.stringify(fields)),release:async()=>{await host0.release();}};
 const roots=[PROJECT],selection={kind:"tasks",tasks:[{project:PROJECT,target:TARGET}]};
 return load.loadGenerationGraphV1(host,{policy:"fresh",selection,slice:{selection,roots,providers:registry.catalogProviders.map(({consumerRoots,sourcePath})=>({consumerRoots,sourcePath}))}});
}));
let result;
for(let run=1;run<=Number(E.RUNS??"1");run++){
 if(run===2){globalThis.__extOld=false;}
 if(run===2)touched=new Set((E.TOUCH2??"").split(",").filter(Boolean).map(file=>ROOT+"/"+file));
 globalThis.__asmMark("run "+run+" start");result=await runOnce();globalThis.__asmMark("run "+run+" done");
}
globalThis.__asmMark("loadGenerationGraphV1");
console.log(JSON.stringify({wallMs:Math.round(performance.now()-t0),peakRssMB:mb(peak),projects:Object.keys(result.projects).length,deps:result.dependencies.length}));
