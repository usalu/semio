import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmod,mkdtemp,readdir,readFile,rm,stat,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import ts from "typescript";
import {BAZEL_ACTION_ENVIRONMENT_POLICY_V1,bazelActionEnvironmentAllowsV1,bazelActionEnvironmentV1,bazelDeclaredEnvironmentNamesV1,bazelDeclaredEnvironmentOfPlanV1,readBazelDeclaredEnvironmentV1,recordBazelDeclaredEnvironmentV1} from "../🟦️.ts";
import {releaseBuildGraphV1} from "../../../../../🕸️graph/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import bazelSchema from "../../../🧬️schema/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),bazel=resolve(here,"../../.."),upper=(rows:readonly string[])=>new Set(rows.map(name=>name.toUpperCase()));
const oracle=():{environments:string[][];delivered:boolean[];withheld:boolean[]}=>{
 const run=spawnSync(process.platform==="win32"?"python":"python3",["-I",join(here,"🧮️resources","🐍️oracle","🐍️.py"),join(here,"..","🧬️schema","🔣️.json")],{input:JSON.stringify(fixture),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);
};
const secrets=fixture.environments[0]!.environment,essentials=fixture.environments[0]!.kept;

test("An action receives exactly the declared and allowed names of every environment row, and an independent Python oracle agrees",()=>{
 const actual=fixture.environments.map(row=>Object.keys(bazelActionEnvironmentV1(row.environment,upper(row.declared))).sort());
 expect(actual).toEqual(fixture.environments.map(row=>[...row.kept].sort()));
 expect(oracle().environments).toEqual(actual);
 for(const row of fixture.environments)for(const name of Object.keys(row.environment).filter(name=>!row.kept.includes(name)))expect(bazelActionEnvironmentAllowsV1(name,upper(row.declared)),name).toBe(false);
 const classes=BAZEL_ACTION_ENVIRONMENT_POLICY_V1.classes;
 expect(new Set(classes.map(row=>row.id)).size).toBe(classes.length);
 for(const row of [...classes,BAZEL_ACTION_ENVIRONMENT_POLICY_V1.denied]){expect(row.reason.en.length).toBeGreaterThan(10);expect(row.reason.de.length).toBeGreaterThan(10);}
 expect(schema.description).toContain(" / Schema-first Vertrag");
});

const writerOf=async()=>{
 const text=await readFile(join(bazel,"📜️script.ts"),"utf8"),tree=ts.createSourceFile("script.ts",text,ts.ScriptTarget.Latest,true),node=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="writeBazelRuntimeV1");
 if(!node)throw Error("The Bazel runtime channel writer is required");
 const code=ts.transpileModule(node.getText(tree)+";return writeBazelRuntimeV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 return Function("join","writeFile","chmod","validateJsonSchemaSubset","runtimeSchema","bazelActionEnvironmentV1",code)(join,writeFile,chmod,validateJsonSchemaSubset,bazelSchema.definitions.Runtime,bazelActionEnvironmentV1) as (workspace:string,root:string,environment:NodeJS.ProcessEnv,declared:ReadonlySet<string>)=>Promise<void>;
};

test("The runtime channel the build writes never carries an ambient secret, desktop, agent or session variable",async()=>{
 const directory=await mkdtemp(join(tmpdir(),"actenv-")),write=await writerOf();
 try{
  const ambient=Object.fromEntries(Object.keys(secrets).map(name=>[name,"S3CRET-"+name]));
  await write(directory,"/workspace",{...ambient,SEMIO_BUILD_MODE:"release"},new Set());
  const file=join(directory,"runtime.json"),raw=await readFile(file,"utf8"),value=JSON.parse(raw) as {version:number;workspace:string;environment:Record<string,string>};
  expect(raw).not.toContain("S3CRET-NEO4J_PASSWORD");
  for(const name of ["NEO4J_PASSWORD","SSH_AUTH_SOCK","GITHUB_TOKEN","NPM_TOKEN","AWS_SECRET_ACCESS_KEY","ANTHROPIC_API_KEY","CLAUDE_CODE_SESSION_ID","XPC_FLAGS","TERM_SESSION_ID","DISPLAY","DBUS_SESSION_BUS_ADDRESS","DATABASE_URL"])expect(Object.keys(value.environment),name).not.toContain(name);
  expect(Object.keys(value.environment).sort()).toEqual([...essentials,"SEMIO_BUILD_MODE"].sort());
  expect(validateJsonSchemaSubset(bazelSchema.definitions.Runtime,value)).toEqual([]);
  if(process.platform!=="win32")expect((await stat(file)).mode&0o777).toBe(0o600);
  await write(directory,"/workspace",{...ambient,FIXTURE_DECLARED:"1"},new Set(["FIXTURE_DECLARED"]));
  expect(Object.keys(JSON.parse(await readFile(file,"utf8")).environment)).toContain("FIXTURE_DECLARED");
 }finally{await rm(directory,{recursive:true,force:true});}
});

const functionText=async(name:string):Promise<string>=>{
 const text=await readFile(join(bazel,"📜️script.ts"),"utf8"),tree=ts.createSourceFile("script.ts",text,ts.ScriptTarget.Latest,true),node=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text===name);
 if(!node)throw Error("The Bazel function is required: "+name);return node.getText(tree);
};
const AsyncFunction=Object.getPrototypeOf(async()=>{}).constructor as new(...parameters:string[])=>(...values:unknown[])=>Promise<unknown>;

test("Every write of the runtime channel passes the declared environment of its plan to the allowlist",async()=>{
 const script=await readFile(join(bazel,"📜️script.ts"),"utf8"),calls=[...script.matchAll(/await writeBazelRuntimeV1\(([^;]*)\);/gu)].map(row=>row[1]!);
 expect(calls.length).toBeGreaterThanOrEqual(3);
 for(const call of calls)expect(/,(?:declared|await recordBazelDeclaredEnvironmentV1\(workspace,graph,plan\))$/u.test(call),call).toBe(true);
});

const releasedGraph=()=>({version:1,projects:{app:{name:"app",root:"app",targets:{},namedInputs:{testEnv:[{env:"SEMIO_TEST_LEVEL"}]}}},dependencies:[],namedInputs:{production:[{env:"SEMIO_BUILD_MODE"},{env:"Rustflags"}]}}),plannedTasks=()=>({tasks:{"app:build":{definition:{inputs:["production",{env:"FIXTURE_DECLARED"}],metadata:{semio:{dashboard:{parameters:[{id:"phase",valueEnv:"PRINT_PHASE"}]}}}}}}});

test("The run phase obtains the declared environment after the build graph was released: the real statements of generate and run, executed against a really released graph, never read the graph",async()=>{
 const directory=await mkdtemp(join(tmpdir(),"actenv-released-")),generate=await functionText("generateBazelWorkspaceV1"),run=await functionText("runBazelTasksInStorageV1");
 try{
  const graph=releasedGraph() as unknown as Parameters<typeof recordBazelDeclaredEnvironmentV1>[1],plan=plannedTasks(),expected=["FIXTURE_DECLARED","PRINT_PHASE","RUSTFLAGS","SEMIO_BUILD_MODE","SEMIO_TEST_LEVEL"];
  const recording=/await writeBazelRuntimeV1\([^;]*,((?:await )?\w+\([^()]*\))\);/u.exec(generate)?.[1],reading=/const declared=((?:await )?\w+\([^()]*\));/u.exec(run)?.[1];
  expect(recording,"generate passes the declared names to the runtime channel").toBeDefined();
  expect(reading,"run obtains the declared names").toBeDefined();
  const bindings={bazelDeclaredEnvironmentOfPlanV1,readBazelDeclaredEnvironmentV1,recordBazelDeclaredEnvironmentV1},evaluate=(expression:string,live:unknown)=>new AsyncFunction("graph","plan","workspace",...Object.keys(bindings),"return "+expression)(live,plan,directory,...Object.values(bindings));
  const names=(await evaluate(recording!,graph)) as Set<string>;
  expect([...names].sort()).toEqual(expected);
  await releaseBuildGraphV1(graph as unknown as Parameters<typeof releaseBuildGraphV1>[0]);
  expect(()=>bazelDeclaredEnvironmentOfPlanV1(graph,plan),"the hazard is real: the graph throws once released").toThrow(/released-read/u);
  const read=(await evaluate(reading!,graph)) as Set<string>;
  expect([...read].sort()).toEqual(expected);
  expect(run).not.toContain("bazelDeclaredEnvironmentOfPlanV1");
  await rm(join(directory,"declared-environment.json"));
  await expect(readBazelDeclaredEnvironmentV1(directory)).rejects.toThrow(/missing or malformed.* \/ .*fehlt oder ist fehlerhaft/su);
 }finally{await rm(directory,{recursive:true,force:true});}
});

test("The record of the declared environment is written once, atomically and unchanged when the plan is the same",async()=>{
 const directory=await mkdtemp(join(tmpdir(),"actenv-record-"));
 try{
  const graph=releasedGraph() as unknown as Parameters<typeof recordBazelDeclaredEnvironmentV1>[1],plan=plannedTasks(),file=join(directory,BAZEL_ACTION_ENVIRONMENT_POLICY_V1.declared.file);
  await recordBazelDeclaredEnvironmentV1(directory,graph,plan);const first=await stat(file,{bigint:true});
  await recordBazelDeclaredEnvironmentV1(directory,graph,plan);const second=await stat(file,{bigint:true});
  expect(second.ino).toBe(first.ino);expect(second.mtimeNs).toBe(first.mtimeNs);
  expect((await readdir(directory)).sort()).toEqual([BAZEL_ACTION_ENVIRONMENT_POLICY_V1.declared.file]);
 }finally{await rm(directory,{recursive:true,force:true});}
});

test("A child process spawned with the action environment sees the repository's variables and no ambient secret",()=>{
 const environment=bazelActionEnvironmentV1({...fixture.environments[1]!.environment,...secrets}),child=spawnSync(process.execPath,["-e","process.stdout.write(JSON.stringify(Object.keys(process.env).sort()))"],{env:environment,encoding:"utf8"});
 expect(child.status,child.stderr).toBe(0);
 const seen=JSON.parse(child.stdout) as string[];
 for(const name of fixture.environments[1]!.kept)expect(seen,name).toContain(name);
 for(const name of ["NEO4J_PASSWORD","SSH_AUTH_SOCK","CLAUDE_CODE_SESSION_ID","XPC_FLAGS","DISPLAY"])expect(seen,name).not.toContain(name);
});

test("The declared environment names come from env input rows, valueEnv parameters and env maps anywhere in the task graph and tolerate cycles, and the repository's nx.json rows reach an action",async()=>{
 const loop:Record<string,unknown>={env:"loop_variable"};loop.self=loop;
 const names=bazelDeclaredEnvironmentNamesV1([{production:["{projectRoot}/**/*",{env:"Rustflags"},{runtime:"repo tools exec bun -- --version"},{input:"default",projects:["a"]}]},[{env:"SDKROOT"}],{semio:{dashboard:{parameters:[{id:"phase",valueEnv:"Print_Phase"},{id:"mode",values:[{id:"a",env:{FROM_MAP:"1"}}]}]}}},loop,undefined,"text"]);
 expect([...names].sort()).toEqual(["FROM_MAP","LOOP_VARIABLE","PRINT_PHASE","RUSTFLAGS","SDKROOT"]);
 let root=here;while(!(await stat(join(root,"nx.json")).then(()=>true,()=>false))&&dirname(root)!==root)root=dirname(root);
 const global=bazelDeclaredEnvironmentNamesV1([JSON.parse(await readFile(join(root,"nx.json"),"utf8")).namedInputs]);
 expect(global.has("SEMIO_BUILD_MODE")).toBe(true);
 for(const name of global)expect(bazelActionEnvironmentAllowsV1(name,global),name).toBe(true);
});

test("Every environment name the repository's own code reads is either delivered to an action or withheld on purpose with a reason, and the Python oracle agrees",async()=>{
 const verdict=oracle();
 expect(fixture.reads.delivered.map(name=>bazelActionEnvironmentAllowsV1(name))).toEqual(fixture.reads.delivered.map(()=>true));
 expect(verdict.delivered).toEqual(fixture.reads.delivered.map(()=>true));
 expect(fixture.reads.withheld.map(row=>bazelActionEnvironmentAllowsV1(row.name))).toEqual(fixture.reads.withheld.map(()=>false));
 expect(verdict.withheld).toEqual(fixture.reads.withheld.map(()=>false));
 for(const row of fixture.reads.withheld){expect(row.reason.en.length,row.name).toBeGreaterThan(10);expect(row.reason.de.length,row.name).toBeGreaterThan(10);}
 const known=new Set<string>([...fixture.reads.delivered,...fixture.reads.withheld.map(row=>row.name)]),unclassified=new Set<string>();
 const reads=/(?:\bprocess\.env|\b[A-Za-z_]*[eE]nv(?:ironment)?|\bbaseEnv)\??(?:\.([A-Z][A-Z0-9_]{2,})\b|\["([A-Z][A-Z0-9_]{2,})"\])/gu,walk=async(directory:string):Promise<void>=>{
  for(const entry of await readdir(directory,{withFileTypes:true})){
   if(entry.name==="node_modules"||entry.name==="🧪️tests")continue;
   const path=join(directory,entry.name);
   if(entry.isDirectory()){await walk(path);continue;}
   if(!/\.(?:ts|mjs)$/u.test(entry.name))continue;
   for(const match of (await readFile(path,"utf8")).matchAll(reads)){const name=match[1]??match[2]!;if(!known.has(name)&&!bazelActionEnvironmentAllowsV1(name))unclassified.add(name);}
  }
 };
 for(const wrapper of fixture.wrappers)await walk(resolve(bazel,wrapper));
 expect([...unclassified].sort(),"classify new names in the fixture: delivered by a class of the schema, or withheld with a reason").toEqual([]);
});
