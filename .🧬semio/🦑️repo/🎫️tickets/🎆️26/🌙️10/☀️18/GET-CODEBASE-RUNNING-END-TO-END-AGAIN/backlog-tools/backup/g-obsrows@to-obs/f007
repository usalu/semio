import {buildBackendDefaultIdV1} from "../../../🔌️backends/🟦️.ts";
import ts from"typescript";
/** 🧭️ The portable CLI, task selections and native caller keep literal arguments and original resources. */
import {testValueV1} from "../../../../⚡️caching/🧪️tests/📋️values/🟦️.ts";
import {registerBuildTestLawV1} from "@semio-tech/process/testing/invocation";
import {mkdir,mkdtemp,writeFile,readFile,rm,access} from "node:fs/promises";
import * as rawPromises from "node:fs/promises";
import {join,resolve,relative} from "node:path";
import {expect,test,spyOn} from "bun:test";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import fixture from "../../🧫️fixtures/🕸️graph/🔣️.json";
import callerSchema from "../../../🧬️schema/📥️caller.json";
import initialAuthoringSchema from "../../../🧬️schema/📥️authoring.json";
import {readBuildAuthoringCapabilitiesV1,createNativeBuildCallerCapabilitiesV1,issueBuildTaskEnvironmentV1,readOwnedBuildInvocationV1} from "../../../📥️caller/🟦️.ts";
import {runOwnedBuildChildInvocationV1} from "../../../📥️caller/📤️child/🟦️.ts";
import schema from "../../../🧬️schema/🔣️.json";
import {readBuildGraphV1,neutralBuildTargetV1,resolveBuildTargetV1} from "../../../🕸️graph/🟦️.ts";
import {parseBuildInvocationV1,nxBuildArgumentsV1,resolveBuildInvocationV1} from "../../../🧭️invocation/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json";
import {watchBuildInvocationV1,selectBuildTasksV1,selectBuildProjectsV1} from "../../../🧭️selection/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope,advanceScriptInvocation,withScriptInvocationSignalV1,acknowledgeScriptInvocationSignalStopV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {nxOracleRequireV1,graph} from "../🧰️support/🟦️.ts";

const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;
test("portable CLI keeps backend selection separate from literal task arguments",()=>{
  expect(parseBuildInvocationV1(["--tool=bazel","run","app:build","--","--tool=nx"],{})).toEqual({tool:"bazel",args:["run","app:build","--","--tool=nx"]});
  expect(parseBuildInvocationV1(["show","projects"],{SEMIO_BUILD_TOOL:"bazel"}).tool).toBe("bazel");
  expect<unknown>(parseBuildInvocationV1(["run","app:build"],{}).tool).toBe(buildBackendDefaultIdV1());expect(parseBuildInvocationV1(["run","app:build"],{SEMIO_BUILD_TOOL:"nx"}).tool).toBe("nx");expect(parseBuildInvocationV1(["--tool=nx","run","app:build"],{SEMIO_BUILD_TOOL:"bazel"}).tool).toBe("nx");expect(parseBuildInvocationV1(["--tool=bazel","run","app:build"],{SEMIO_BUILD_TOOL:"nx"}).tool).toBe("bazel");
  expect(()=>parseBuildInvocationV1(["--tool=unknown"],{})).toThrow();
  expect(nxBuildArgumentsV1(["run","app:build","--skip-cache","--exclude-dependencies","--output=stream","--","--skip-cache"])).toEqual(["run","app:build","--skip-nx-cache","--excludeTaskDependencies","--output-style=stream","--","--skip-cache"]);
  expect(resolveBuildInvocationV1(process.cwd(),["run","app:build:release"],{}).env.SEMIO_BUILD_MODE).toBe("release");
});
test("language-neutral task selections preserve configurations, tags and exclusions",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"build-graph-test",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{
  for(const row of fixture.selectionCases){const result=await selectBuildTasksV1(process.cwd(),graph(),row.args,invocation);expect(result.tasks).toEqual(row.tasks);expect(result.args).toEqual(row.forwarded);}
  expect(selectBuildProjectsV1(graph(),["tag:kind:app"])).toEqual(["app"]);
  const skipped=await selectBuildTasksV1(process.cwd(),graph(),testValueV1(fixture.selectionCases[2]).args,invocation);expect(skipped.skipCache).toBe(true);expect(skipped.excludeDependencies).toBe(true);
}));
test("portable native caller preserves exact original resources and literal task arguments",async()=>{
  const root=process.cwd(),source=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"nc-")),manifest=join(source,"Cargo.toml"),manifestPath=relative(root,manifest).replaceAll("\\","/"),workingDirectory=relative(root,source).replaceAll("\\","/"),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","app:build"],"bazel",{SEMIO_BUILD_MODE:"release"}),validate=new (require("ajv").default)({strict:false}).compile(callerSchema);
  try{await writeFile(manifest,fixture.nativeCallerSource.manifest);expect(require("@iarna/toml").parse(await readFile(manifest,"utf8"))).toEqual(Bun.TOML.parse(fixture.nativeCallerSource.manifest));expect(createHash("sha256").update(await readFile(manifest)).digest("hex")).toBe(fixture.nativeCallerSource.sha256);expect((await rawPromises.lstat(manifest)).isFile()).toBe(true);
  expect(validate(capabilities)).toBe(true);
  const envelope=createScriptProcessEnvelope({version:1,owner:"build-native-test",maximumElapsedMilliseconds:0},capabilities,Date.now());
  await withScriptProcessEnvelope(envelope,async invocation=>{
    const execution={kind:"command" as const,ownerArguments:true,nativeOwnerCommand:{kind:"native-owner-command",manifest:manifestPath,workingDirectory,program:"bun",arguments:["./📜️script.ts","build"]}};
    const environment=issueBuildTaskEnvironmentV1(root,invocation,execution,["literal space","$literal"],{SEMIO_BUILD_MODE:"release"}),issued=JSON.parse(environment.SEMIO_SCRIPT_PROCESS_INVOCATION!);
    expect(issued.policy).toEqual(envelope.policy);expect(issued.deadlineEpochMilliseconds).toBeNull();expect(issued.capabilities.command.arguments).toEqual(["./📜️script.ts","build","literal space","$literal"]);
    expect(issued.capabilities.transport).toEqual(capabilities.native.transport);expect(environment.SEMIO_BUILD_TOOL).toBe("bazel");expect(JSON.parse(environment.SEMIO_OWNER_ARGUMENTS!)).toEqual({version:1,arguments:["literal space","$literal"]});
    expect(invocation.capabilities).toEqual(capabilities);expect(invocation.capabilities).not.toBe(capabilities);expect(issued.capabilities.command.manifest).toBe(manifestPath);expect(()=>issueBuildTaskEnvironmentV1(root,invocation,{...execution,nativeOwnerCommand:{...execution.nativeOwnerCommand,manifest:manifestPath+".missing"}},[],{SEMIO_BUILD_MODE:"release"})).toThrow();
  });expect(createHash("sha256").update(await readFile(manifest)).digest("hex")).toBe(fixture.nativeCallerSource.sha256);
  }finally{await rm(source,{recursive:true,force:true});}
});
test("portable selection refuses original cancellation before graph execution",async()=>{
  const controller=new AbortController();controller.abort(Error("cancelled-build"));
  const invocation={policy:{version:1 as const,owner:"cancelled-build",maximumElapsedMilliseconds:0},capabilities:{},control:{signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>{},yieldContinuation:async()=>{}}};
  await expect(selectBuildTasksV1(process.cwd(),graph(),["run","app:build"],invocation)).rejects.toThrow("cancelled-build");
});
registerBuildTestLawV1("source watch filters owned projects, debounces events and returns on original cancellation","watch-lifecycle",async original=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"watch-")),controller=new AbortController(),receipt=join(root,".🧬semio","watch.json");let failed=false;
  try{await mkdir(join(root,testValueV1(graph().projects.app).root),{recursive:true});await mkdir(join(root,testValueV1(graph().projects.lib).root),{recursive:true});await mkdir(join(root,".🧬semio"));
  await withScriptInvocationSignalV1(original,controller.signal,async invocation=>{
   const phaseControl=createIssuedResourceControlV1(invocation,"graph:watch-fixture",0),execution=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts"),actualRun=execution.runOwnedCommand;let childClosed=false;const childCompletion=spyOn(execution,"runOwnedCommand").mockImplementation(async(...args)=>{await actualRun(...args);if(args[2]===root&&args[3]==="build:watch")childClosed=true;});
   const command=[process.execPath,"-e",'require("node:fs").writeFileSync(process.argv[1],JSON.stringify({files:JSON.parse(process.env.SEMIO_BUILD_CHANGED_FILES),projects:JSON.parse(process.env.SEMIO_BUILD_CHANGED_PROJECTS)}))',receipt],watching=watchBuildInvocationV1(root,graph(),["watch","--projects="+fixture.watch.selected,"--",...command],invocation,phaseControl,{child:{readOwnedBuildInvocationV1,runOwnedBuildChildInvocationV1}});void watching.catch(()=>{});
   let primary:{error:unknown}|undefined;try{
    await writeFile(join(root,fixture.watch.excluded),"unselected");await Bun.sleep(300);await expect(access(receipt)).rejects.toThrow();
    await writeFile(join(root,fixture.watch.included),"selected");
    for(let attempt=0;attempt<100;attempt++){try{await access(receipt);break;}catch{await Bun.sleep(20);}}
    const result=JSON.parse(await readFile(receipt,"utf8"));expect(result.projects).toEqual(fixture.watch.expectedChangedProjects);expect(result.files).toEqual([fixture.watch.included]);
    for(let attempt=0;attempt<100&&!childClosed;attempt++){await advanceScriptInvocation(invocation,"graph:watch-child-close","running");await Bun.sleep(20);}expect(childClosed).toBe(true);
   }catch(error){primary={error};}finally{controller.abort(Error("watch-test-complete"));try{await watching;}catch(error){primary??={error};}acknowledgeScriptInvocationSignalStopV1(invocation);childCompletion.mockRestore();}if(primary)throw primary.error;
  });}catch(error){failed=true;throw error;}finally{try{await rm(root,{recursive:true,force:true});}catch(error){if(!failed)throw error;}}
});
test.each(fixture.initialAuthoring.bootstrap)("neutral prebirth grants derive only exact implementation launches ($tool $arguments)",async row=>{
 const {createBuildBootstrapCapabilitiesV1}=await import("../../../📥️caller/📤️child/🟦️.ts"),root=process.cwd(),tool=row.tool as "nx"|"bazel",request=["repo",...row.arguments],declared=createNativeBuildCallerCapabilitiesV1(root,row.arguments,tool,{SEMIO_BUILD_MODE:"release"}),authored={...declared,native:{...declared.native,transport:{maximumBytes:12345,maximumLines:123},child:{maximumElapsedMilliseconds:123456},network:{offline:true}}},wire=JSON.stringify(authored),validate=new(require("ajv").default)({strict:true}).compile(initialAuthoringSchema),equal=nxOracleRequireV1("lodash/isEqual");
 expect(validate(authored)).toBe(true);expect(authored.launches).toEqual([request]);
 const actual=await createBuildBootstrapCapabilitiesV1(root,[`--tool=${tool}`,...row.arguments],{SEMIO_SCRIPT_CAPABILITIES:wire}) as typeof authored;
 expect(equal(actual,{...authored,launches:row.launches})).toBe(true);expect(actual.native).toEqual(authored.native);expect(actual.buildMode).toBe(authored.buildMode);expect(JSON.stringify(authored)).toBe(wire);
 expect<unknown>(await createBuildBootstrapCapabilitiesV1(root,[`--tool=${tool}`,...row.arguments],{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(actual)})).toEqual(actual);
 await expect(createBuildBootstrapCapabilitiesV1(root,[`--tool=${tool}`,"run","unauthorized:build"],{SEMIO_SCRIPT_CAPABILITIES:wire})).rejects.toThrow("authored build request");
 for(const changed of [{...authored,workspaceRoot:".."},{...authored,tool:tool==="nx"?"bazel":"nx"},{...authored,native:{...authored.native,transport:{...authored.native.transport,maximumBytes:0}}}])await expect(createBuildBootstrapCapabilitiesV1(root,[`--tool=${tool}`,...row.arguments],{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(changed)})).rejects.toThrow();
 await expect(createBuildBootstrapCapabilitiesV1(root,[`--tool=${tool}`,...row.arguments],{SEMIO_SCRIPT_CAPABILITIES:wire,SEMIO_BUILD_MODE:"prototype"})).rejects.toThrow();
});
test("explicit editor grants retain their original native resource bounds before the CLI envelope",async()=>{
  const {createBuildBootstrapCapabilitiesV1}=await import("../../../📥️caller/📤️child/🟦️.ts"),root=process.cwd(),args=["run","app:build"],declared=createNativeBuildCallerCapabilitiesV1(root,args,"nx",{SEMIO_BUILD_MODE:"release"}),{tool,...authored}=declared;
  const custom={...authored,native:{...authored.native,transport:{maximumBytes:12345,maximumLines:123},child:{maximumElapsedMilliseconds:123456}}};
  const admitted=readBuildAuthoringCapabilitiesV1(root,args,"nx",custom),schema=(await import("../../../🧬️schema/📥️authoring.json")).default;
  expect(new (require("ajv").default)({strict:false}).compile(schema)(custom)).toBe(true);expect(admitted.native).toEqual(custom.native);expect(admitted.tool).toBe("nx");
  const actual=await createBuildBootstrapCapabilitiesV1(root,["repo",...args],{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(custom),SEMIO_BUILD_TOOL:"nx"});expect(actual).toEqual({...admitted,launches:[...admitted.launches,["nx","run","app:build"]]});
  expect(()=>readBuildAuthoringCapabilitiesV1(root,["run","other:build"],"nx",custom)).toThrow("authored build request");
});
test.each(fixture.initialAuthoring.cases)("relative initial declarations preserve exact GUI resources through original orchestration and native task issuance ($name)",async row=>{
 const {createBuildBootstrapCapabilitiesV1}=await import("../../../📥️caller/📤️child/🟦️.ts"),authoringSchema=(await import("../../../🧬️schema/📥️authoring.json")).default,validateAuthoring=new(require("ajv").default)({strict:true}).compile(authoringSchema),validateCaller=new(require("ajv").default)({strict:true}).compile(callerSchema),root=process.cwd();
 const source=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"ng-")),manifest=join(source,"Cargo.toml"),manifestPath=relative(root,manifest).replaceAll("\\","/"),workingDirectory=relative(root,source).replaceAll("\\","/");try{await writeFile(manifest,fixture.nativeCallerSource.manifest);expect(require("@iarna/toml").parse(await readFile(manifest,"utf8")).package.name).toBe(fixture.nativeCallerSource.package);expect(createHash("sha256").update(await readFile(manifest)).digest("hex")).toBe(fixture.nativeCallerSource.sha256);expect((await rawPromises.lstat(manifest)).isFile()).toBe(true);
 expect(require("@iarna/toml").parse(await readFile(join(root,".cargo/config.toml"),"utf8")).build["target-dir"]).toBe(fixture.initialAuthoring.cases[0]!.native.artifactDirectory);
  const args=[...row.arguments,"--help"],request=["repo",...args],authored={workspaceRoot:row.workspaceRoot,tool:row.tool,arguments:request,selection:request,launches:[request],buildMode:row.buildMode,native:row.native};
  expect(validateAuthoring(authored),row.name).toBe(true);
  const admitted=readBuildAuthoringCapabilitiesV1(root,args,"nx",authored),initial=await createBuildBootstrapCapabilitiesV1(root,request,{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(authored),SEMIO_BUILD_TOOL:"nx"});
  expect(initial).toEqual({...admitted,launches:[request,["nx","run",row.arguments[1],"--skip-nx-cache","--excludeTaskDependencies","--parallel=1","--configuration=prototype","--help"]]});expect(validateCaller(initial),row.name).toBe(true);expect(admitted.workspaceRoot).toBe(root);expect(admitted.native).toEqual({...row.native,artifactDirectory:resolve(root,row.native.artifactDirectory)});
  const refusal=fixture.initialAuthoring.missingLaunches,missing=JSON.parse(JSON.stringify(authored));delete missing[refusal.omit];expect(validateAuthoring(missing)).toBe(refusal.valid);expect(()=>readBuildAuthoringCapabilitiesV1(root,args,"nx",missing)).toThrow(refusal.error);
  const missingCaller=JSON.parse(JSON.stringify(admitted));delete missingCaller[refusal.omit];expect(validateCaller(missingCaller)).toBe(refusal.valid);
  expect(()=>readBuildAuthoringCapabilitiesV1(root,args,"nx",{...authored,workspaceRoot:".."})).toThrow("authored build request");
  const envelope=createScriptProcessEnvelope({...fixture.initialAuthoring.parent,version:1},initial,Date.now());
  await withScriptProcessEnvelope(envelope,async invocation=>{
   let handed:any;
   const source=readFileSync(new URL("../../../📜️script.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("build.ts",source,ts.ScriptTarget.Latest,true),fn=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="runOwnedBuildInvocationV1");if(!fn)throw Error("Actual Root publication wrapper required");const phase=createIssuedResourceControlV1(invocation,"graph:dispatcher-fixture",0),routing=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),ports={relative,process,repositoryCommandKernelV1:(request:readonly string[])=>{expect(request).toEqual(args);return {kind:"preflight",entry:join(root,"captured-preflight.ts")};},executeCapturedBuildSourceV1:async(workspace:string,entry:string,request:readonly string[],original:any,control:any)=>{expect(workspace).toBe(root);expect(entry).toBe("captured-preflight.ts");expect(request).toEqual(args);expect(original).toBe(invocation);expect(control).toBe(phase);handed=JSON.parse(routing.scriptResourceProcessEnvironmentV1(control,process.env).SEMIO_SCRIPT_PROCESS_INVOCATION!);},load:async(path:string)=>path==="./⚙️operations/🔤️options/🟦️.ts"?{normalizeBuildOptionsV1:(value:readonly string[])=>value}:path==="./🌡️host-capacity/🟦️.ts"?{withBuildCapacityV1:async(value:readonly string[],_env:any,body:any,options:any)=>{expect(options.control).toBe(phase);await body({args:value});}}:path==="./📥️inputs/🔏️revision/🟨️.mjs"?{withImplementationPinV1:async(body:any)=>body()}:Promise.reject(Error("Unexpected Root publication dependency: "+path))};const run=new Function(...Object.keys(ports),ts.transpileModule(fn.getText(tree).replace(/^export /u,"").replace(/await import\(([^)]+)\)/gu,"await load($1)"),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText+";return runOwnedBuildInvocationV1")(...Object.values(ports));await run(root,args,invocation,phase);
   expect(handed.capabilities).toEqual(initial);expect(handed.policy).toEqual(envelope.policy);expect(handed.deadlineEpochMilliseconds).toBeLessThanOrEqual(envelope.deadlineEpochMilliseconds!);expect(handed.deadlineEpochMilliseconds).toBeGreaterThan(Date.now());
   const environment=issueBuildTaskEnvironmentV1(root,invocation,{...fixture.initialAuthoring.task,kind:"command",nativeOwnerCommand:{...fixture.initialAuthoring.task.nativeOwnerCommand,manifest:manifestPath,workingDirectory}},fixture.initialAuthoring.taskArguments,{}),native=JSON.parse(environment.SEMIO_SCRIPT_PROCESS_INVOCATION!);
   expect(native.policy).toEqual(envelope.policy);expect(native.deadlineEpochMilliseconds).toBeLessThanOrEqual(envelope.deadlineEpochMilliseconds!);expect(native.deadlineEpochMilliseconds).toBeGreaterThan(Date.now());expect(native.capabilities.child).toEqual(row.native.child);expect(native.capabilities.transport).toEqual(row.native.transport);expect(native.capabilities.network).toEqual(row.native.network);expect(native.capabilities.buildMode).toBe(row.buildMode);expect(native.capabilities.artifactDirectory).toBe(admitted.native.artifactDirectory);expect(native.capabilities.repositoryRoot).toBe(root);expect(native.capabilities.command.arguments).toEqual([...fixture.initialAuthoring.task.nativeOwnerCommand.arguments,...fixture.initialAuthoring.taskArguments]);expect(native.capabilities.command.manifest).toBe(manifestPath);
  });
 expect(createHash("sha256").update(await readFile(manifest)).digest("hex")).toBe(fixture.nativeCallerSource.sha256);
 }finally{await rm(source,{recursive:true,force:true});}
},35000);
test("dashboard target selection preserves the portable initial authority corpus",()=>{
 const validate=new(require("ajv").default)({strict:true}).compile(initialAuthoringSchema),root=process.cwd(),registry=JSON.parse(readFileSync(resolve(root,"📋️build-owners.json"),"utf8")),dashboard=JSON.parse(readFileSync(resolve(root,"📋️project.json"),"utf8")).metadata.semio.dashboard;
 expect(dashboard.tools.find((tool:any)=>tool.id==="run-targets").command.slice(0,3)).toEqual(["repo","run-many"]);
 for(const row of fixture.initialAuthoring.cases){const request=["repo",...row.arguments],declared={workspaceRoot:row.workspaceRoot,tool:row.tool,arguments:request,selection:request,launches:[request],buildMode:row.buildMode,native:row.native};expect(validate(declared),row.name).toBe(true);const [project,target]=row.arguments[1]!.split(":");const owner=registry.owners.find((owner:any)=>owner.name===project);expect(owner,row.name).toBeDefined();expect(JSON.parse(readFileSync(resolve(root,owner.manifest),"utf8")).targets[target],row.name).toBeDefined();}
});
test("neutral arguments admit literal arrays and refuse scalar shell fragments before execution",()=>{
 const validate=new(require("ajv").default)({strict:true}).compile(require("../../../🧬️schema/📋️project/🔣️.json")),portable=new Ajv({strict:true}).compile(schema),row=fixture.literalArguments;
 for(const args of row.rejected){
  const base={executor:"repo:script",options:{command:"repo ./📜️script.ts",args}},configured={executor:"repo:script",options:{command:"repo ./📜️script.ts"},configurations:{release:{args}}};
  for(const target of [base,configured]){expect(validate({name:"app",targets:{build:target}})).toBe(false);expect(()=>neutralBuildTargetV1(target,"app:build")).toThrow(/arguments.*app:build/);}
  const baseGraph=graph();testValueV1(testValueV1(baseGraph.projects.app).targets.build).execution.args=args as any;expect(portable(baseGraph)).toBe(false);expect(()=>readBuildGraphV1(baseGraph)).toThrow();const configuredGraph=graph();testValueV1(testValueV1(configuredGraph.projects.app).targets.build).configurations={release:{execution:{args}}};expect(portable(configuredGraph)).toBe(false);expect(()=>readBuildGraphV1(configuredGraph)).toThrow();
 }
 for(const converted of row.converted){const project=JSON.parse(readFileSync(resolve(process.cwd(),converted.path),"utf8")),target=project.targets[converted.target],neutral=neutralBuildTargetV1(target,project.name+":"+converted.target),selected=resolveBuildTargetV1(neutral,converted.configuration);expect(validate(project)).toBe(true);expect(selected.execution.args).toEqual(converted.args);expect(require("../../../📥️caller/🟦️.ts").buildExecutionArgumentsV1(selected.execution,row.boundaryArguments)).toEqual([...converted.args,...row.boundaryArguments]);}
});
