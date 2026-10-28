/** 📐️ The portable graph model agrees with its independent schema, bundler and projection oracles. */
import {testValueV1} from "../../../../⚡️caching/🧪️tests/📋️values/🟦️.ts";
import {resolve,posix,win32} from "node:path";
import {projectBuildTargetForNxV1} from "../../../../🟨️.mjs";
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import {build} from "esbuild";
import fixture from "../../🧫️fixtures/🕸️graph/🔣️.json";
import initialAuthoringSchema from "../../../🧬️schema/📥️authoring.json";
import schema from "../../../🧬️schema/🔣️.json";
import {readBuildGraphV1,neutralBuildTargetV1,resolveBuildTargetV1} from "../../../🕸️graph/🟦️.ts";
import {graph} from "../🧰️support/🟦️.ts";

const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;
test("portable graph fixture agrees with the independent JSON Schema implementation",()=>{
  const validate=new Ajv({strict:false}).compile(schema);
  expect(validate(fixture.graph)).toBe(true);
  expect(testValueV1(graph().projects.app).name).toBe("app");
  for(const value of [{...fixture.graph,version:2},{...fixture.graph,projects:{bad:{name:"bad",root:"bad",targets:{build:{}}}}}]){
    expect(validate(value)).toBe(false);expect(()=>readBuildGraphV1(value)).toThrow();
  }
});
test("portable configuration and Nx projection preserve command semantics",()=>{
  const target={executor:"repo:script",options:{command:"repo ./📜️script.ts build",cwd:"app",env:{BASE:"yes",OVERRIDE:"base"}},configurations:{release:{env:{MODE:"release",OVERRIDE:"release"}}},outputs:["dist"],dependsOn:["^build"]};
  const neutral=neutralBuildTargetV1(target),resolved=resolveBuildTargetV1(neutral,"release"),nx=projectBuildTargetForNxV1(neutral);
  expect(neutral.execution.kind).toBe("command");expect(neutral).not.toHaveProperty("executor");
  expect(resolved.execution.env).toEqual({BASE:"yes",OVERRIDE:"release",MODE:"release"});expect(resolved.outputs).toEqual(["dist"]);expect(resolved.dependsOn).toEqual(["^build"]);expect(nx).toEqual({...target,executor:"@semio-tech/repo-lib:command",inputs:["default","^default",fixture.nxEngineNamedInput.name],configurations:{release:{...target.configurations.release,env:{BASE:"yes",OVERRIDE:"release",MODE:"release"}}}});
  const sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),oracle=sdk("nx/src/utils/params").combineOptionsForExecutor({},"release",structuredClone(nx),sdk("nx/src/executors/run-commands/schema.json"),null,null);expect(oracle.env).toEqual(resolved.execution.env);
  expect(()=>resolveBuildTargetV1(neutral,"unknown")).toThrow();
});
test("portable models compile under the independent bundler without Nx runtime",async()=>{
  const result=await build({entryPoints:[fileURLToPath(new URL("../../../🕸️graph/🟦️.ts",import.meta.url))],bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,logLevel:"silent"});
  expect(Object.keys(result.metafile!.inputs).some(path=>path.includes("node_modules/nx/"))).toBe(false);
});
test("canonical graph refuses competing identities at one physical root",()=>{
  const duplicate=graph();duplicate.projects.alias={...testValueV1(duplicate.projects.app),name:"alias"};
  expect(new Ajv({strict:true}).compile({type:"array",items:{type:"string"},uniqueItems:true})(Object.values(duplicate.projects).map(project=>project.root))).toBe(false);
  expect(()=>readBuildGraphV1(duplicate)).toThrow("Duplicate build project root");
});
test("portable relative GUI declarations survive Windows and POSIX JSON path serialization",()=>{
 const validate=new(require("ajv").default)({strict:true}).compile(initialAuthoringSchema);
 for(const row of fixture.initialAuthoring.paths){
  const path=row.platform==="win32"?win32:posix,source=fixture.initialAuthoring.cases[0]!,request=["repo",...source.arguments],declared={workspaceRoot:source.workspaceRoot,tool:source.tool,arguments:request,selection:request,launches:[request],buildMode:source.buildMode,native:source.native},serialized=JSON.stringify({SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(declared),CARGO_TARGET_DIR:row.artifact}),environment=JSON.parse(serialized),parsed=JSON.parse(environment.SEMIO_SCRIPT_CAPABILITIES);
  expect(validate(parsed)).toBe(true);
  expect(parsed.workspaceRoot).toBe(".");expect(path.resolve(row.root,parsed.workspaceRoot)).toBe(row.root);expect(path.resolve(row.root,parsed.native.artifactDirectory)).toBe(row.artifact);expect(environment.CARGO_TARGET_DIR).toBe(row.artifact);expect(environment.SEMIO_SCRIPT_CAPABILITIES).not.toContain("${workspaceFolder}");
 }
});
test("configuration options preserve task identity and command form under both schema oracles",()=>{
 const authored=require("../../../🧬️schema/📋️project/🔣️.json"),authoring=new(require("ajv").default)({strict:true}).compile(authored),portable=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.configurationRefusals){const target={executor:"repo:script",options:{command:"one"},configurations:{release:row.configuration}},project={name:"app",targets:{build:target}};expect(authoring(project),row.name).toBe(false);expect(()=>neutralBuildTargetV1(target),row.name).toThrow();const value=graph();value.projects.app!.targets.build!.configurations={release:row.name==="commands"?{execution:row.configuration}:row.name==="kind"||row.name==="ownerArguments"||row.name==="nativeOwnerCommand"?{execution:row.configuration}:row.configuration};expect(portable(value),row.name).toBe(false);expect(()=>readBuildGraphV1(value),row.name).toThrow();}
});
test("authored command shorthand normalizes to the complete portable execution",()=>{
  const source=fixture.topCommand,author=require("../../../🧬️schema/📋️project/🔣️.json"),Draft7=require("ajv").default,validate=new Draft7({strict:true}).compile(author),neutral=neutralBuildTargetV1(source.target),resolved=resolveBuildTargetV1(neutral,"release"),sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),projection=projectBuildTargetForNxV1(neutral),oracle=sdk("nx/src/utils/params").combineOptionsForExecutor({},"release",projection,sdk("nx/src/executors/run-commands/schema.json"),null,null);
  const inferred=require("../../../🕸️graph/📥️inference/🟨️.mjs").cacheInternals.targetWithDefaults(source.target,"app",true);expect(inferred).not.toHaveProperty("command");expect(inferred.options.command).toBe(source.target.command);
  expect(validate({name:"app",targets:{build:source.target}})).toBe(true);expect(neutral).not.toHaveProperty("command");expect(neutral.execution.command).toBe(source.target.command);expect(resolved.execution.command).toBe(source.target.configurations.release.command);expect(oracle.command).toBe(resolved.execution.command);expect(oracle.env).toEqual(resolved.execution.env);
  for(const target of source.invalid){expect(validate({name:"app",targets:{build:target}})).toBe(false);expect(()=>neutralBuildTargetV1(target)).toThrow();}
});
