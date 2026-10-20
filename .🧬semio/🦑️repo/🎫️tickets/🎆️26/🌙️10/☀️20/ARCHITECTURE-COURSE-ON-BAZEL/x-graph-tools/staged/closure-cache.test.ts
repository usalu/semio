import {test,expect} from "bun:test";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixtureData from "./🔣️.json" with {type:"json"};
import fixtureSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import admissionSchema from "../../../../🧬️schema/🚪️admission.json" with {type:"json"};
import {annotateBuildOwnerAdmissionFailureV1,buildAdmissionReceiptPathV1,buildAdmissionReceiptPublishableV1,buildOwnerAdmissionChainV1,buildOwnerAdmissionRecordV1,buildGraphScopeKeyV1,buildInvocationGraphSelectionV1,closeBuildOwnerAdmissionV1,createBuildAdmissionReceiptV1,observeBuildOwnerAdmissionV1,readBuildAdmissionReceiptV1,slimBuildOwnerAdmissionFactsV1,readBuildOwnerRegistryV1,type BuildOwnerAdmissionOriginV1} from "../../🟦️.ts";

/** 🗃️ Law of the owner admission closure and its persisted vocabulary over `./🔣️.json`. Oracles: an independent Python fixpoint with hashlib and jsonschema (`./🐍️oracle.py`), Ajv for every schema, graphlib for reachability. */
const fixture:any=fixtureData,require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default,graphlib=require("graphlib");
const here=import.meta.dir,worlds=new Map<string,any>(fixture.closure.worlds.map((world:any)=>[world.id,world]));
const oracle=(()=>{const run=Bun.spawnSync(["python3",join(here,"🐍️oracle.py"),join(here,"🔣️.json"),join(here,"../../../../🧬️schema/🚪️admission.json")],{stdout:"pipe",stderr:"pipe"});if(run.exitCode!==0)throw Error("oracle failed: "+run.stderr.toString());return JSON.parse(run.stdout.toString());})();

const registryOf=(world:any)=>readBuildOwnerRegistryV1({version:1,owners:world.owners.map((row:any)=>({name:row.name,root:row.root,manifest:row.manifest,packages:[],...(row.nativeRoots?{nativeRoots:row.nativeRoots}:{})})),catalogProviders:world.catalogProviders});
const factsOf=(world:any,name:string,targets:readonly string[],inputs:readonly string[])=>{
 const entry=world.owners.find((row:any)=>row.name===name),files=new Set<string>([entry.manifest,...entry.base.files]),dependencies=new Set<string>(entry.base.dependencies),grouped:Record<string,Set<string>>={},generated:Record<string,string[]>={};
 for(const target of targets){const row=entry.targets[target];row.files.forEach((file:string)=>files.add(file));row.dependencies.forEach((dependency:string)=>dependencies.add(dependency));for(const[owner,groups]of Object.entries<string[]>(row.dependencyInputs))groups.forEach(group=>(grouped[owner]??=new Set()).add(group));if(row.generatedDependencies.length)generated[target]=[...row.generatedDependencies].sort();}
 for(const group of inputs)(entry.inputFiles[group]??[]).forEach((file:string)=>files.add(file));
 return {project:{name,targets:Object.fromEntries(Object.entries<any>(entry.targets).map(([target,row])=>[target,{dependsOn:row.dependsOn}]))},files:[...files].sort(),dependencies:[...dependencies].sort(),...(Object.keys(generated).length?{generatedDependencies:generated}:{}),dependencyInputs:Object.fromEntries(Object.entries(grouped).map(([owner,groups])=>[owner,[...groups].sort()]))};
};
const run=async(world:any,selection:any)=>{
 const log:string[]=[];
 try{const value=await closeBuildOwnerAdmissionV1(selection,registryOf(world),async(owner,targets,inputs)=>{log.push(owner.name);if(world.broken.includes(owner.name))throw Object.assign(Error("broken owner "+owner.name),{code:"FIXTURE_BROKEN"});return factsOf(world,owner.name,targets,inputs) as any;});return {value,log};}
 catch(error){return {error:error as Error,log};}
};

test("the fixture is closed under its schema and the oracle reproduces every frozen answer",()=>{
 const validate=new Ajv({strict:true,allErrors:false}).compile(fixtureSchema);expect(validate(fixture),JSON.stringify(validate.errors?.slice(0,3))).toBe(true);
 expect(oracle).toEqual(fixture.expected);
});

test("closure selection equals the full graph filtered to the selected tasks, never inspects anything else and ignores unrelated broken owners",async()=>{
 const admission=new Ajv({strict:false}).compile(admissionSchema);let throwing=0,unrelated=0;
 for(const row of fixture.closure.cases){
  const world=worlds.get(row.world),expected:any=(fixture.expected.closure as any)[row.id],actual=await run(world,row.selection);
  if(expected.throws){
   throwing++;expect(actual.error,row.id).toBeDefined();expect(actual.error!.message,row.id).toContain("broken owner "+expected.blocked);expect(actual.error!.message,row.id).toContain("Build owner admission chain: "+expected.blocked+" <- ");expect(actual.error!.message,row.id).toMatch(/ <- selection$/u);
   expect((actual.error as any).code,row.id).toBe("FIXTURE_BROKEN");continue;
  }
  expect(actual.error,row.id).toBeUndefined();const value:any=actual.value;
  expect(admission(value),JSON.stringify(admission.errors)).toBe(true);
  expect(value.owners,row.id).toEqual(expected.owners);expect(value.files,row.id).toEqual(expected.files);expect(value.targets,row.id).toEqual(expected.targets);expect(value.inputs,row.id).toEqual(expected.inputs);expect(value.catalogProviders,row.id).toEqual(expected.catalogProviders);expect(value.generatedDependencies,row.id).toEqual(expected.generatedDependencies);
  expect(new Set(actual.log).size,row.id).toBe(expected.owners.length);for(const name of actual.log)expect(expected.owners,row.id+" inspected "+name).toContain(name);
  for(const name of world.broken)if(!expected.owners.includes(name)){unrelated++;expect(actual.log).not.toContain(name);}
  if(row.selection.kind==="tasks"){
   const graph=new graphlib.Graph({directed:true}),entryOf=(name:string)=>world.owners.find((owner:any)=>owner.name===name);
   for(const name of value.owners)graph.setNode(name);
   for(const name of value.owners){const entry=entryOf(name),selected:string[]=value.targets[name]??[];
    const facts=factsOf(world,name,selected,value.inputs[name]??[]);
    for(const file of facts.files){const owner=world.owners.filter((candidate:any)=>[candidate.root,...(candidate.nativeRoots??[])].some((root:string)=>file===root||file.startsWith(root+"/"))).sort((a:any,b:any)=>b.root.length-a.root.length)[0];if(owner&&owner.name!==name)graph.setEdge(name,owner.name);}
    for(const dependency of facts.dependencies)graph.setEdge(name,dependency);
    for(const target of selected)for(const dependency of entry.targets[target].dependsOn){
     if(typeof dependency==="string"){if(dependency.includes(":"))graph.setEdge(name,dependency.slice(0,dependency.lastIndexOf(":")));}
     else for(const project of dependency.dependencies?facts.dependencies:dependency.projects==="*"?world.owners.map((owner:any)=>owner.name):typeof dependency.projects==="string"?[dependency.projects]:dependency.projects??[name])if(project!==name)graph.setEdge(name,project);
    }
    for(const producers of Object.values<string[]>(facts.generatedDependencies??{}))for(const producer of producers)graph.setEdge(name,producer.slice(0,producer.lastIndexOf(":")));
   }
   for(const provider of value.catalogProviders)for(const name of value.owners)if(value.inputs[name]?.includes("{workspaceRoot}/"+provider.sourcePath))for(const seed of row.selection.tasks)graph.setEdge(seed.project,name);
   const reachable=new Set<string>(graphlib.alg.preorder(graph,row.selection.tasks.map((task:any)=>task.project)));
   for(const name of value.owners)expect(reachable.has(name),row.id+" admitted "+name+" without a path from the selection").toBe(true);
  }
 }
 expect(throwing).toBeGreaterThanOrEqual(2);expect(unrelated).toBeGreaterThanOrEqual(2);
});

test("a failing owner names the requester chain and keeps its identity, class and code",async()=>{
 const world=worlds.get("generated"),actual=await run(world,fixture.closure.cases.find((row:any)=>row.id==="generated producer failing is in the closure")!.selection);
 expect(actual.error!.message).toContain("broken owner shell-gen\nBuild owner admission chain: shell-gen <- consumer:build <- selection");
 expect(actual.error).toBeInstanceOf(Error);expect((actual.error as any).admissionChain).toEqual(["shell-gen","consumer:build","selection"]);
 const original=Object.assign(new Error("boom"),{code:"X"}),map=new Map<string,BuildOwnerAdmissionOriginV1>([["a",null]]);
 expect(annotateBuildOwnerAdmissionFailureV1(original,"a",map)).toBe(original);annotateBuildOwnerAdmissionFailureV1(original,"a",map);expect(original.message).toBe("boom\nBuild owner admission chain: a <- selection");expect((original as any).code).toBe("X");
 const abort=new DOMException("stop","AbortError");expect(annotateBuildOwnerAdmissionFailureV1(abort,"a",map)).toBe(abort);expect(abort.name).toBe("AbortError");expect(annotateBuildOwnerAdmissionFailureV1("plain","a",map)).toBe("plain");
});

test("requester chains equal the oracle's",()=>{
 for(const row of fixture.chains){const map=new Map<string,BuildOwnerAdmissionOriginV1>(row.requestedBy.map(([name,origin]:[string,any])=>[name,origin]));expect(buildOwnerAdmissionChainV1(map,row.name),row.id).toEqual((fixture.expected.chains as any)[row.id]);}
});

test("record leaves and keys equal hashlib's, stay inside the optional cache grammar and separate implementation, scope, registry and order",()=>{
 const grammar=/^np\/[a-f0-9]{24}\.json$/u,seen=new Map<string,string>();
 for(const row of fixture.keys){
  const record=buildOwnerAdmissionRecordV1(row.implementation,row.scope,row.registry,row.owner,row.targets,row.inputs),expected=(fixture.expected.keys as any)[row.id];
  expect(record.leaf,row.id).toBe(expected.leaf);expect(record.leaf).toMatch(grammar);expect(createHash("sha256").update(JSON.stringify(record.key)).digest("hex"),row.id).toBe(expected.keySha256);seen.set(row.id,expected.keySha256);
 }
 expect(new Set(seen.values()).size).toBe(seen.size);
 const stageTwoLeaf="np/"+createHash("sha256").update(fixture.keys[0]!.owner.root).digest("hex").slice(0,24)+".json";
 expect(buildOwnerAdmissionRecordV1("i","a".repeat(64),"b",fixture.keys[0]!.owner,[],[]).leaf).not.toBe(stageTwoLeaf);
});

test("receipts are closed: our reader and Ajv agree with the oracle, and only young facts withhold publication",()=>{
 const ajv=new Ajv({strict:false}),shape=ajv.compile({$defs:(admissionSchema as any).$defs,$ref:"#/$defs/admissionReceipt"}),closed=ajv.compile(admissionSchema);
 const module={implementation:"3".repeat(64),publication:"2".repeat(64)};
 for(const row of fixture.receipts){
  const value:any=row.value,verdict=shape(value)&&closed(value.admission),expected=(fixture.expected.receipts as any)[row.id];
  expect(verdict,row.id).toBe(row.valid);expect(expected,row.id).toBe(row.valid);
  if(typeof value?.scope==="string"&&value.module)expect(readBuildAdmissionReceiptV1(JSON.stringify(value),{scope:value.scope,module:value.module})!==undefined,row.id).toBe(row.valid);
  if(row.valid)expect(readBuildAdmissionReceiptV1(createBuildAdmissionReceiptV1(value),{scope:value.scope,module:value.module})).toEqual(value);else expect(()=>createBuildAdmissionReceiptV1(value)).toThrow();
 }
 const good=fixture.receipts[0]!.value as any;
 expect(readBuildAdmissionReceiptV1(JSON.stringify(good),{scope:"c".repeat(64),module:good.module})).toBeUndefined();
 expect(readBuildAdmissionReceiptV1(JSON.stringify(good),{scope:good.scope,module})).toBeUndefined();
 expect(readBuildAdmissionReceiptV1("{",{scope:good.scope,module:good.module})).toBeUndefined();
 for(const row of fixture.publishable)expect(buildAdmissionReceiptPublishableV1(row.observations,row.startedAt),row.id).toBe((fixture.expected.publishable as any)[row.id]);
 expect(buildAdmissionReceiptPathV1("/s","a".repeat(64))).toBe("/s/"+"a".repeat(64)+".admission.json");expect(()=>buildAdmissionReceiptPathV1("/s","nope")).toThrow();
});

test("records keep only what the closure reads, as plain JSON, and hits equal fresh observations",async()=>{
 const noisy:any={project:{name:"o",targets:{build:{dependsOn:["^build",{target:"x",projects:["p"]}],options:{command:"bun x",inputs:Array.from({length:1200},(_,index)=>"{workspaceRoot}/f"+index)},cwd:undefined,run:()=>1},bare:{cache:true}}},files:["a","b"],dependencies:["p"],dependencyInputs:{p:["default"]},setLike:new Set([1])};
 const slim=slimBuildOwnerAdmissionFactsV1(noisy) as any;expect(slim.project).toEqual({name:"o",targets:{build:{dependsOn:["^build",{target:"x",projects:["p"]}]},bare:{}}});expect(slim.files).toEqual(["a","b"]);expect(slim.setLike).toEqual({});
 const stored=new Map<string,unknown>(),ports={directory:"/d",implementation:"i",scope:"a".repeat(64),registry:"r",check:()=>{},advance:async()=>{},observe:async(options:any,compute:any)=>{if(stored.has(options.cachePath))return stored.get(options.cachePath);const value=JSON.parse(JSON.stringify(await compute()));stored.set(options.cachePath,value);return value;}};
 const owner={name:"o",root:"o",manifest:"o/"+"📋️project.json"};let computed=0;const run=()=>observeBuildOwnerAdmissionV1(ports as any,owner,["build"],[],async()=>{computed++;return noisy;});
 const first=await run(),second=await run();expect(second).toEqual(first);expect(computed).toBe(1);expect(first).toEqual(slim);
});

const settle=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));
test("cross-process reuse: warm runs compute nothing, an edit recomputes exactly the changed owner, registry or implementation changes recompute all, a cancelled closure resumes",async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"closure-cache-")),step=join(here,"🏃️step/🟦️.ts");
 try{
  for(const scenario of fixture.scenarios){
   const dir=join(root,scenario.id.replace(/[^a-z]+/gu,"-"));mkdirSync(join(dir,"world"),{recursive:true});
   const world=structuredClone(scenario.world) as any,write=(name:string,entry:any)=>writeFileSync(join(dir,"world",name+".json"),JSON.stringify(entry));
   for(const entry of world.owners)write(entry.name,entry);await settle(2300);
   let registry="b".repeat(64),implementation="rev-one",done=new Set<string>();const expectedRows=(fixture.expected.scenarios as any)[scenario.id] as any[];
   for(const[index,item]of scenario.steps.entries()){
    if(item.op==="replace"){write(item.owner,item.declaration);await settle(2300);continue;}
    if(item.op==="registry"){registry=item.digest;continue;}
    if(item.op==="implementation"){implementation=item.revision;continue;}
    const output=Bun.spawnSync(["bun",step,JSON.stringify({dir,world,selection:item.selection,implementation,registry,scope:"a".repeat(64),...(item.cancelAfter?{cancelAfter:item.cancelAfter}:{})})],{stdout:"pipe",stderr:"pipe"}),line=output.stdout.toString().trim().split("\n").at(-1)!,reported=JSON.parse(line) as {computed:string[];result?:any;error?:string},owners=[...new Set(reported.computed.map(value=>value.split("|")[0]!))].sort(),row=expectedRows[index]!;
    if(item.expect==="cancel"){expect(reported.error,scenario.id).toBe("cancelled");expect(reported.computed.length).toBe(item.cancelAfter);done=new Set(reported.computed);continue;}
    expect(reported.error,scenario.id+" step "+index+" "+(output.stderr.toString().slice(-400))).toBeUndefined();expect(reported.result.owners).toEqual(row.owners);
    if(item.expect==="cold")expect(owners).toEqual(row.owners);
    else if(item.expect==="warm")expect(reported.computed).toEqual([]);
    else if(item.expect==="changed")expect(owners).toEqual([...new Set([...item.changed,...row.added])].sort());
    else if(item.expect==="all")expect(owners).toEqual(row.owners);
    else if(item.expect==="resume"){for(const value of reported.computed)expect(done.has(value),"recomputed "+value).toBe(false);expect(new Set([...done,...reported.computed]).size).toBe(done.size+reported.computed.length);expect(reported.computed.length).toBeGreaterThan(0);expect(reported.result.owners).toEqual(row.owners);}
   }
  }
 }finally{rmSync(root,{recursive:true,force:true});}
},240_000);

test("the staged graph glue reuses the closure, persists it only with the graph it admitted and resumes through per-owner records",()=>{
 const source=readFileSync(join(here,"../../../🟦️.ts"),"utf8"),inventory=source.slice(source.indexOf("const inventory=async()=>"),source.indexOf("let initial:Awaited<ReturnType<typeof inventory>>"));
 expect(source).toContain("const reuseAdmission=async()");expect(source).toContain("const publishAdmission=async(");
 expect(inventory.indexOf("reuseAdmission()")).toBeGreaterThan(-1);expect(inventory.indexOf("reuseAdmission()")).toBeLessThan(inventory.indexOf("closeBuildOwnerAdmissionV1("));
 expect(inventory.indexOf("await producer.libraryBootstrap")).toBeLessThan(inventory.indexOf("closeBuildOwnerAdmissionV1("));
 expect(inventory).toContain("observeBuildOwnerAdmissionV1(records(),owner,targets,inputs,()=>producer.discoverBuildOwnerAdmissionFactsV1(");
 expect(source).toContain("observe:(request,compute)=>observation.observeProjectInferenceV1(request,compute)");expect(source).toContain("implementation:producer.currentImplementationRevisionV1()");
 expect(inventory).not.toContain("publishAdmission(");
 const attempt=source.slice(source.indexOf("async function loadBuildGraphAttemptV1"),source.indexOf("export async function loadBuildGraphV1"));
 expect(attempt.indexOf("await rename(receipt,authorityPath)")).toBeLessThan(attempt.indexOf("await publishAdmission(initial.admission,initial.observations,initial.startedAt)"));
 expect(attempt.match(/publishAdmission\(/gu)?.length).toBe(1);
 expect(inventory.match(/workspaceBuildFilesV1\(/gu)?.length).toBe(1);expect(inventory.indexOf("workspaceBuildFilesV1(")).toBeGreaterThan(inventory.indexOf("admission?.files??"));
 expect(source).toContain('"(?:\\\\.authority|\\\\.admission)?\\\\.json\\\\.[a-f0-9-]{36}$"');
 expect(source.match(/closeBuildOwnerAdmissionV1\(/gu)?.length).toBe(1);
});

test("Nx and Bazel resolve one neutral selected graph: one selection function, one loader, a scope key without any tool",()=>{
 const build=readFileSync(join(here,"../../../../📜️script.ts"),"utf8"),nx=readFileSync(join(here,"../../../../../⚡️caching/🚀️bootstrap/📜️script.ts"),"utf8");
 expect(build.indexOf("loadBuildInvocationGraphV1(root,resolved.args")).toBeGreaterThan(-1);expect(build.indexOf("loadBuildInvocationGraphV1(root,resolved.args")).toBeLessThan(build.indexOf('if(selected.tool==="nx")'));
 expect(nx).toContain("loadBuildGraphV1(this.root,this.invocation,phaseControl,{selection:buildInvocationGraphSelectionV1(segments)})");
 const registry=readBuildOwnerRegistryV1({version:1,owners:[{name:"a",root:"a",manifest:"a/📋️project.json",packages:[]}],catalogProviders:[]});
 for(const tool of["nx","bazel"]){
  const previous=process.env.SEMIO_BUILD_TOOL;process.env.SEMIO_BUILD_TOOL=tool;
  try{const selection=buildInvocationGraphSelectionV1(["run","a:build"]);expect(selection).toEqual({kind:"tasks",tasks:[{project:"a",target:"build"}]});expect(buildGraphScopeKeyV1(selection,registry)).toBe(createHash("sha256").update(JSON.stringify([{kind:"tasks",tasks:[{project:"a",target:"build"}]},registry])).digest("hex"));}
  finally{if(previous===undefined)delete process.env.SEMIO_BUILD_TOOL;else process.env.SEMIO_BUILD_TOOL=previous;}
 }
});

const loadAttempt=(bindings:Record<string,unknown>)=>{
 const ts=require("typescript"),source=readFileSync(join(here,"../../../🟦️.ts"),"utf8"),tree=ts.createSourceFile("graph.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find((statement:any)=>ts.isFunctionDeclaration(statement)&&statement.name?.text==="loadBuildGraphAttemptV1");
 const transformed=ts.transform(node,[(context:any)=>{const visit=(child:any):any=>ts.isMetaProperty(child)?ts.factory.createIdentifier("__meta"):ts.visitEachChild(child,visit,context);return (child:any)=>ts.visitNode(child,visit);}]);
 const code=ts.transpileModule(ts.createPrinter().printNode(ts.EmitHint.Unspecified,transformed.transformed[0],tree).replace("export ","").replaceAll("import(","__import("),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
 transformed.dispose();return Function(...Object.keys(bindings),code+";return loadBuildGraphAttemptV1;")(...Object.values(bindings)) as (...args:any[])=>Promise<any>;
};
const glueBindings=(setup:{receipt?:string;authority?:string;graph?:string;stale?:(row:any)=>boolean;youngRows?:boolean;membershipFails?:number})=>{
 const scope="a".repeat(64),module={implementation:"1".repeat(64),publication:"2".repeat(64)},files:Record<string,string>={},operations:string[]=[],counts={closure:0,discover:0,observed:[] as any[],inspected:0};
 const registry={version:1,owners:[{name:"app",root:"app",manifest:"app/📋️project.json",packages:[]}],catalogProviders:[]},admission={owners:["app"],files:["app/📋️project.json"],targets:{app:["build"]},catalogProviders:[],inputs:{}};
 const rows=[{kind:"file",path:"/w/app/📋️project.json",fact:{size:1,mtimeMs:setup.youngRows?Date.now():1,ctimeMs:setup.youngRows?Date.now():1,ino:1,sha256:"e".repeat(64)}}];
 const producer={readBuildOwnerRegistrySourceV1:()=>JSON.stringify(registry),libraryBootstrap:Promise.resolve(),prepareSourceAnalysesV1:()=>{},currentImplementationRevisionV1:()=>"revision",discoverBuildOwnerAdmissionFactsV1:async()=>{counts.inspected++;return {project:{name:"app",targets:{}},files:[],dependencies:[]};}};
 const observation={observedFs:{readFileSync:()=>""},captureObservedSourceInputsV1:async(body:()=>Promise<unknown>)=>({value:await body(),observations:rows}),settleObservedComputationV1:async(body:()=>Promise<unknown>)=>body(),observeProjectInferenceV1:async(request:any,compute:()=>Promise<unknown>)=>{counts.observed.push(request);return compute();}};
 const graph={version:1,projects:{},dependencies:[],admission:{selection:{kind:"tasks",tasks:[{project:"app",target:"build"}]},registry:"r",closure:"c"}};
 if(setup.receipt==="valid")files["/owned/cache/selected/"+scope+".admission.json"]=createBuildAdmissionReceiptV1({version:1,scope,module,startedAt:1000,admission:admission as any,observations:[{kind:"file",path:"/w/app/📋️project.json",fact:{size:1,mtimeMs:1,ctimeMs:1,ino:1,sha256:"c".repeat(64)}}]});
 if(setup.authority!==undefined)files["/owned/cache/selected/"+scope+".authority.json"]=setup.authority;
 if(setup.graph!==undefined)files["/owned/cache/selected/"+scope+".json"]=setup.graph;
 let membershipCalls=0;const invalidated=Object.freeze({kind:"invalidated",reason:"owner-registry"});
 const bindings:Record<string,unknown>={
  __meta:{url:"file:///owned/graph.ts"},__import:async(specifier:string)=>specifier.includes("inference")?producer:{default:{createNodesV2:[0,async()=>[]]},discoverTestBuildProjectsV1:async()=>[]},
  advanceScriptInvocation:async()=>{},buildAdmissionReceiptPathV1,buildAdmissionReceiptPublishableV1,buildGraphErrorCodeV1:(error:any)=>error?.code,buildGraphMembersV1:(value:string[])=>value,
  buildGraphMembershipV1:async()=>{if(++membershipCalls<=(setup.membershipFails??0))throw Object.assign(Error("changed"),{code:"BUILD_MEMBERSHIP_CHANGED"});return "membership";},
  buildGraphPathV1:()=>"/owned/cache/canonical.json",buildGraphProducerAuthorityV1:()=>true,buildGraphScopeKeyV1:()=>scope,buildGraphSourceFactsHoldV1:async(list:any[])=>!list.some(setup.stale??(()=>false)),
  checkResourceLeaseControl:()=>{},checkScriptInvocation:()=>{},closeBuildOwnerAdmissionV1:async(_selection:unknown,_registry:unknown,inspect:(owner:any,targets:string[],inputs:string[])=>Promise<unknown>)=>{counts.closure++;if(counts.closure===1)await inspect(registry.owners[0],["build"],[]);return admission;},
  createBuildAdmissionReceiptV1,createHash,dirname,discoverBuildGraphV1:async()=>{counts.discover++;return graph;},graphDeclarationV1:()=>({properties:{authority:{const:"authority.json"}}}),inventoryDirectoriesV1:new WeakMap(),isOwnerRegistryInvalidationV1:(value:unknown)=>value===invalidated,join,
  loadedGraphModuleAuthorityV1:module,mkdir:async()=>{},observationV1:observation,observeBuildOwnerAdmissionV1,ownerRegistryInvalidationV1:invalidated,physicalSourceOriginV1:()=>"file:///owned/graph.ts",portableInputs:async(_root:string,value:unknown)=>value,randomUUID:()=>"nonce",
  readBuildAdmissionReceiptV1,readBuildGraphV1:(value:unknown)=>value,readBuildOwnerRegistryV1:(value:unknown)=>value,readFile:async(path:string)=>{if(path in files)return files[path];throw Object.assign(Error("absent"),{code:"ENOENT"});},
  rename:async(from:string,to:string)=>{operations.push("rename "+to.replace(/^.*\//u,"").replace(scope,"<scope>"));},repositoryBuildGraphWriterDirectoryV1:()=>"/owned/cache/inference",requireCurrentGraphModuleV1:()=>{},requireIssuedResourceControlInvocationV1:()=>{},rm:async()=>{},
  workspaceBuildFilesV1:async()=>{throw Error("selected inventory never lists the workspace");},writeFile:async(path:string,text:string)=>{operations.push("write "+path.replace(/^.*\//u,"").replace(scope,"<scope>").replace(/\.nonce$/u,""));files[path]=text;},
 };
return {bindings,scope,module,files,operations,counts,registry,admission,graph,invocation:{control:{signal:new AbortController().signal}}};
};
const attemptOptions={selection:{kind:"tasks",tasks:[{project:"app",target:"build"}]}};
test("graph attempt: a cold run derives the closure once, persists it after the graph and reads it back",async()=>{
 const world=glueBindings({}),run=loadAttempt(world.bindings),graph=await run("/owned",world.invocation,{},attemptOptions,world.registry,world.scope);
 expect(graph).toBe(world.graph);expect(world.counts.closure).toBe(1);expect(world.counts.discover).toBe(1);expect(world.counts.inspected).toBe(1);
 expect(world.operations).toEqual(["write "+"<scope>.json","write <scope>.authority.json","rename <scope>.json","rename <scope>.authority.json","write <scope>.admission.json","rename <scope>.admission.json"]);
 const request=world.counts.observed[0];expect(request.cachePath).toMatch(/^\/owned\/cache\/inference\/np\/[a-f0-9]{24}\.json$/u);expect(request.owner).toBe("owner-admission:app");expect(request.key[1]).toBe("revision");expect(request.key[2]).toBe(world.scope);
 const stored=Object.entries(world.files).find(([path])=>path.includes(".admission.json"))![1],receipt=readBuildAdmissionReceiptV1(stored,{scope:world.scope,module:world.module});expect(receipt?.admission).toEqual(world.admission);
});
test("graph attempt: a warm run with a valid receipt and graph never derives the closure and writes nothing",async()=>{
 const graphText=JSON.stringify({version:1,projects:{},dependencies:[],admission:{selection:attemptOptions.selection,registry:"r",closure:"c"}}),authority=JSON.stringify({version:3,module:{implementation:"1".repeat(64),publication:"2".repeat(64)},membership:"membership",graph:createHash("sha256").update(graphText).digest("hex"),observations:[]});
 const world=glueBindings({receipt:"valid",authority,graph:graphText}),run=loadAttempt(world.bindings),graph=await run("/owned",world.invocation,{},attemptOptions,world.registry,world.scope);
 expect(graph.admission.closure).toBe("c");expect(world.counts.closure).toBe(0);expect(world.counts.discover).toBe(0);expect(world.counts.inspected).toBe(0);expect(world.operations).toEqual([]);
});
test("graph attempt: stale receipt facts or a changed membership fall back to exactly one derivation",async()=>{
 for(const[name,setup]of Object.entries({"stale receipt facts":{receipt:"valid" as const,stale:(row:any)=>row.fact?.sha256==="c".repeat(64)},"membership changed on reuse":{receipt:"valid" as const,membershipFails:1}})){
  const world=glueBindings(setup),run=loadAttempt(world.bindings);await run("/owned",world.invocation,{},attemptOptions,world.registry,world.scope);
  expect(world.counts.closure,name).toBe(1);expect(world.counts.discover,name).toBe(1);expect(world.operations.at(-1),name).toBe("rename <scope>.admission.json");
 }
});
test("graph attempt: facts younger than the racy window are neither remembered nor persisted",async()=>{
 const world=glueBindings({youngRows:true}),run=loadAttempt(world.bindings);await run("/owned",world.invocation,{},attemptOptions,world.registry,world.scope);
 expect(world.counts.closure).toBe(2);expect(world.operations.filter(value=>value.includes("admission"))).toEqual([]);
});
