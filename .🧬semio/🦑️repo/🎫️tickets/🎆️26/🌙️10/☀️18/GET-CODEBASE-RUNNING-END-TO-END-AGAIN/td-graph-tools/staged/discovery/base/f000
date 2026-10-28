/** 🌐️ Portable source and manifest discovery excludes generated sources and honours cancellation. */
import {mkdir,mkdtemp,writeFile,readFile,rm,access} from "node:fs/promises";
import {dirname,join,resolve,posix} from "node:path";
import {expect,test} from "bun:test";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {build} from "esbuild";
import fixture from "../../🧫️fixtures/🕸️graph/🔣️.json";
import {readBuildOwnerRegistryV1} from "../../../🕸️graph/🚪️admission/🟦️.ts";
import {workspaceBuildFilesV1} from "../../../🕸️graph/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope,advanceScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {independentGlobV1} from "../🧰️support/🟦️.ts";

const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;
test("Cargo edges resolve inherited paths against their actual nested workspace",async()=>{
  const directory=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"cargo-"));
  try{
    for(const[path,source]of Object.entries(fixture.cargoWorkspace.files)){const file=join(directory,path);await mkdir(dirname(file),{recursive:true});await writeFile(file,source);}
    const owner=require("../../../🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
    const projects={owner:{name:"owner",root:"nested/owner",targets:{}},dependency:{name:"dependency",root:"nested/dependency",targets:{}}},fileMap={projectFileMap:{owner:[{file:"nested/owner/Cargo.toml"}],dependency:[{file:"nested/dependency/Cargo.toml"}]}};
    const oracle=require("@iarna/toml").parse(fixture.cargoWorkspace.files["nested/Cargo.toml"]);expect(resolve(directory,"nested",oracle.workspace.dependencies.local.path)).toBe(join(directory,projects.dependency.root));
    expect(await owner.cacheInternals.createDependenciesImplementation({}, {workspaceRoot:directory,projects,fileMap})).toEqual(fixture.cargoWorkspace.expected);
    const names=new Map(Object.entries(projects).map(([name,project])=>[project.root,name])),before=owner.cacheInternals.projectAssemblyAuthorityV1(directory,projects.owner.root,names);
    names.set("unrelated/new-crate","unrelated");expect(owner.cacheInternals.projectAssemblyAuthorityV1(directory,projects.owner.root,names)).toBe(before);
    names.set(projects.dependency.root,"renamed-dependency");expect(owner.cacheInternals.projectAssemblyAuthorityV1(directory,projects.owner.root,names)).not.toBe(before);
  }finally{await rm(directory,{recursive:true,force:true});}
});
test("portable source discovery excludes generated dependency sources before graph admission",async()=>{
  const artifacts=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??join(process.cwd(),".🧬semio/🦑️repo/⚡️cache/tests/build-membership"));await mkdir(artifacts,{recursive:true});const root=await mkdtemp(join(artifacts,"membership-"));
  try{
    for(const[path,source]of Object.entries(fixture.membership.files)){const destination=join(root,path);await mkdir(dirname(destination),{recursive:true});await writeFile(destination,source);}
    const original=Object.keys(fixture.membership.files).sort(),excluded:boolean[]=require("nx/src/native").matchGlobPaths(fixture.membership.excludedPatterns,original),independent=original.filter((_,index)=>!excluded[index]);
    expect(independent).toEqual(fixture.membership.expected);
    expect(await workspaceBuildFilesV1(root)).toEqual(independent);
  }finally{await rm(root,{recursive:true,force:true});}
});
test("bounded manifest discovery delegates original progress, continuation and cancellation",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"inference-control-test",maximumElapsedMilliseconds:10000},{},Date.now()),async original=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"inference-control-")),controller=new AbortController(),phases:string[]=[],files:string[]=[];let continuations=0;
  const invocation={...original,control:{...original.control,signal:AbortSignal.any([original.control.signal,controller.signal]),publish:(event:any)=>{phases.push(event.command);return original.control.publish(event);},yieldContinuation:async()=>{continuations++;controller.abort(Error("fixture-inference-cancelled"));await original.control.yieldContinuation();}}};
  try{
    for(let index=0;index<fixture.inferenceControl.owners;index++){const directory="owner-"+index,file=directory+"/Cargo.toml",manifest=fixture.inferenceControl.manifest.replaceAll("{{index}}",String(index));expect(require("@iarna/toml").parse(manifest).package.name).toBe(directory);await mkdir(join(root,directory));await writeFile(join(root,file),manifest);files.push(file);}
    const owner=require("../../../🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
    const {checkScriptInvocation,advanceScriptInvocation}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
    await expect(owner.cacheInternals.walkCargoTomlControlledV1(root,{check:()=>checkScriptInvocation(invocation),advance:(phase:string)=>advanceScriptInvocation(invocation,"build:"+phase,"running")})).rejects.toThrow("fixture-inference-cancelled");
    expect(phases).toEqual(["build:manifest-discovery"]);expect(continuations).toBe(fixture.inferenceControl.continuations);expect(invocation.control.remainingMilliseconds).toBe(original.control.remainingMilliseconds);
  }finally{await rm(root,{recursive:true,force:true});}
}));
test("primary native owner discovery retains its original phase cancellation before any fact write",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"owner-control-test",maximumElapsedMilliseconds:10000},{},Date.now()),async original=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"oc-")),controller=new AbortController(),phases:string[]=[],files:string[]=[];let continuations=0;
 const invocation={...original,control:{...original.control,signal:AbortSignal.any([original.control.signal,controller.signal]),publish:(event:any)=>{phases.push(event.command);return original.control.publish(event);},yieldContinuation:async()=>{continuations++;controller.abort(Error("fixture-owner-cancelled"));await original.control.yieldContinuation();}}};
 try{
  for(let index=0;index<fixture.inferenceControl.owners;index++){const directory="owner-"+index,file=directory+"/Cargo.toml",manifest=fixture.inferenceControl.manifest.replaceAll("{{index}}",String(index));expect(require("@iarna/toml").parse(manifest).package.name).toBe(directory);await mkdir(join(root,directory));await writeFile(join(root,file),manifest);files.push(file);}
  const owner=await import("../../../🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;await expect(owner.discoverNativeBuildOwnersV1(files,root,{workspaceRoot:root,invocation,phaseControl:createIssuedResourceControlV1(invocation,"graph:owner-source",0)})).rejects.toThrow("fixture-owner-cancelled");expect(phases).toEqual([fixture.inferenceControl.phase]);expect(continuations).toBe(fixture.inferenceControl.continuations);expect(invocation.control.remainingMilliseconds).toBe(original.control.remainingMilliseconds);await expect(access(join(root,".cache"))).rejects.toThrow();
 }finally{await rm(root,{recursive:true,force:true});}
}));
test("relative import resources retain physical bytes and source-only closure with bundler suffixes",async()=>{
 const row=fixture.relativeImportResources,root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"ri-")),inference=await import("../../../🕸️graph/📥️inference/🟨️.mjs"),registry=readBuildOwnerRegistryV1({version:1,owners:[row.owner],catalogProviders:[]});
 try{for(const[path,source]of Object.entries({...row.files,[row.owner.manifest]:JSON.stringify(row.project)})){expect(join(root,path).length).toBeLessThanOrEqual(256);await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),source);}await writeFile(join(root,row.binary.path),Buffer.from(row.binary.hex,"hex"));
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"relative-resource-source-test",maximumElapsedMilliseconds:20000},{workspaceRoot:root},Date.now()),async invocation=>{
   for(const value of row.cases){const source="import resource from "+JSON.stringify(value.specifier)+"; console.log(resource);\n"+("secondSpecifier"in value?"import executable from "+JSON.stringify(value.secondSpecifier)+"; console.log(executable);\n":"");await writeFile(join(root,"a/entry.ts"),source);const loaded:{path:string;suffix:string}[]=[],oracle=()=>build({entryPoints:["a/entry.ts"],absWorkingDir:root,bundle:true,write:false,outdir:join(root,"unwritten"),metafile:true,loader:{".pdf":"file"},logLevel:"silent",plugins:[{name:"resource-query-loaders",setup(build){build.onLoad({filter:/.*/},async args=>{loaded.push({path:posix.normalize(args.path.slice(root.length+1).replaceAll("\\","/")),suffix:args.suffix});const query=new URL("file:///resource"+args.suffix).searchParams;if((query.has("raw")||query.has("url"))&&!query.has("worker"))return{contents:await readFile(args.path),loader:query.has("raw")?"text":"file"};return undefined;});}}]});
    if("refused"in value){await expect(oracle()).rejects.toThrow();await expect(inference.cacheInternals.relativeScriptInputs([join(root,"a/entry.ts")],root)).rejects.toThrow();await expect(inference.discoverBuildOwnerAdmissionFactsV1(row.owner,["test"],registry,{workspaceRoot:root,invocation,phaseControl:createIssuedResourceControlV1(invocation,"graph:owner-source",0)})).rejects.toThrow();continue;}
    const bundled=await oracle();expect(bundled.errors).toEqual([]);expect([...new Set(loaded.map(value=>value.path))].sort()).toEqual(["a/entry.ts",...value.files].sort());expect(loaded.some(record=>record.suffix===new URL(value.specifier,"file:///a/entry.ts").search+new URL(value.specifier,"file:///a/entry.ts").hash)).toBe(true);
    const closure=(await inference.cacheInternals.relativeScriptInputs([join(root,"a/entry.ts")],root)).map((path:string)=>path.slice("{workspaceRoot}/".length));expect(closure).toEqual(["a/entry.ts",...value.files].sort());const captured=await require("../../../👁️observation/🟨️.mjs").captureObservedSourceInputsV1(()=>inference.discoverBuildOwnerAdmissionFactsV1(row.owner,["test"],registry,{workspaceRoot:root,invocation,phaseControl:createIssuedResourceControlV1(invocation,"graph:owner-source",0)})),selected=captured.value;expect(selected.files).toEqual([row.owner.manifest,"a/entry.ts",...value.files].sort());expect(selected.files).not.toContain("a/missing-raw-child.ts");expect(selected.files.some((path:string)=>/[?#]/u.test(path))).toBe(false);for(const leaf of value.files.filter(path=>path===row.binary.path||path==="a/raw.ts")){const absolute=join(root,leaf),bytes=await readFile(absolute);expect(captured.observations.some((record:any)=>record.kind==="file"&&record.path===absolute&&record.fact.sha256===createHash("sha256").update(bytes).digest("hex"))).toBe(true);expect(captured.observations.some((record:any)=>record.kind==="analysis"&&record.path===absolute)).toBe(false);}await expect(access(join(root,"unwritten"))).rejects.toThrow();
   }
  });
 }finally{await rm(root,{recursive:true,force:true});}
});
test("catalog native candidates distinguish source owners from explicit and actual Cargo manifests",async()=>{
 const row=fixture.catalogNativeManifests,registry=readBuildOwnerRegistryV1(row.registry),root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"cn-")),module=await import("../../../🕸️graph/📥️inference/🟨️.mjs");
 try{for(const path of row.files){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),path.endsWith("Cargo.toml")?"[package]\nname='fixture'\nversion='0.0.0'\n":"{}");}
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"catalog-native-candidates-test",maximumElapsedMilliseconds:10000},{workspaceRoot:root},Date.now()),async invocation=>{const actual=await module.cacheInternals.catalogNativeManifestCandidatesV1(root,registry,(path:string)=>path.startsWith(row.prefix),{workspaceRoot:root,invocation,phaseControl:createIssuedResourceControlV1(invocation,"graph:owner-source",0)}),oracle=independentGlobV1.sync(row.prefix+"**/Cargo.toml",{cwd:root,onlyFiles:true}).sort();expect(actual).toEqual(row.expected);expect(actual).toEqual(oracle);for(const path of actual)expect(require("@iarna/toml").parse(await readFile(join(root,path),"utf8")).package.name).toBe("fixture");await rm(join(root,row.missing));await expect(module.cacheInternals.catalogNativeManifestCandidatesV1(root,registry,(path:string)=>path.startsWith(row.prefix),{workspaceRoot:root,invocation,phaseControl:createIssuedResourceControlV1(invocation,"graph:owner-source",0)})).rejects.toThrow();});
 }finally{await rm(root,{recursive:true,force:true});}
});
