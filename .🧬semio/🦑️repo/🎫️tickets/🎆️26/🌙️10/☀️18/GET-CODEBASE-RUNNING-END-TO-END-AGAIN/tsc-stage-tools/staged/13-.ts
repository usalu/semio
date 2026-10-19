/** 🛠️ Cmake: 5 tests covering neutral actual CMake owner replaces raw probes and ambient state with retained custody; neutral CMake intent and authored Source agree with Ajv and SQLite; actual CMake command stage releases CPU while r… */
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
/** 🏗️ Requires the actual root producer to retain genuine state before CPU and separate test work. */
test("neutral actual CMake owner replaces raw probes and ambient state with retained custody",()=>{const script=readFileSync(new URL("../📜️script.ts",import.meta.url),"utf8");expect(script).toContain("runRepositoryCmakeV1");for(const raw of["spawnSync","readProcessOwnerContextV1","acquireCompilerCapacityLeaseV1","acquireQueuedResourceLease","availableParallelism","uvTool","SEMIO_COMPILER_CAPACITY_HELD"])expect(script).not.toContain(raw);const text=readFileSync(new URL("../🏃️execution/🟦️.ts",import.meta.url),"utf8");for(const owner of["withRepositoryToolingStoreV1","withRepositoryCacheStoreV1","readNativeStorageReservationPolicyV1","withCompilerState","withCmakeInputsV1","runCmakeOwnedStageV1"])expect(text).toContain(owner+"(");expect(text).toContain("capacity:null");expect(text).not.toContain("as NativeStorage");});

import Ajv from"ajv";
import Ajv2020 from"ajv/dist/2020";
import{Database}from"bun:sqlite";
import{join}from"node:path";
import{mkdtempSync,writeFileSync,rmSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{createHash}from"node:crypto";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1,withRequestedIssuedResourceControlV1}from"@semio-tech/process/routing";
import{captureOwnedCommandResultV1}from"@semio-tech/process/execution";
import{withResourceLeases,resourceLeasePathV1}from"@semio-tech/process/leases";
import capacitySchema from"@semio-tech/process/build/capacity/schema.json";
import{readCmakeCacheFactsV1,cmakeIntentV1,cmakeEnvironmentV1}from"../📡️data/🟦️.ts";
import{withCmakeInputsV1,cmakeInputBytesV1,cmakeInputFactsV1,type CmakeInputsV1}from"../📥️inputs/🟦️.ts";
import{runCmakeOwnedStageV1}from"../🎬️stage/🟦️.ts";
import fixture from"./🔣️.json";
import schema from"../🧬️schema/🔣️.json";
/** 🗺️ Compares closed portable mode and command taxonomy with independent relational rules. */
test("neutral CMake intent and authored Source agree with Ajv and SQLite",()=>{const db=new Database(":memory:");try{for(const row of fixture.intents){const{preset,configuration,compiler,...input}=row;expect(new Ajv({strict:true}).validate(schema,input)).toBe(true);const actual=cmakeIntentV1(input);expect(JSON.stringify(actual)).toBe(JSON.stringify(row));expect(db.query("select ?1||'-'||?2 preset,case ?2 when 'prototype' then 'Debug' else 'Release' end configuration,?3 in ('configure','build') compiler").get(row.host,row.mode,row.operation)).toEqual({preset,configuration,compiler:Number(compiler)});}for(const[name,expected]of Object.entries(schema.definitions.Policy.const.source)){const bytes=readFileSync(join(process.cwd(),name));expect(createHash("sha256").update(bytes).digest("hex")).toBe(expected);}const environment=cmakeEnvironmentV1({PATH:"captured",CCACHE_DIR:"unowned",CMAKE_BUILD_PARALLEL_LEVEL:"99"});expect(environment).toMatchObject({PATH:"captured",CCACHE_DISABLE:"1",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"});expect(environment).not.toHaveProperty("CCACHE_DIR");expect(environment).not.toHaveProperty("CMAKE_BUILD_PARALLEL_LEVEL");for(const name of schema.definitions.Policy.const.unsupportedEnvironment)expect(()=>cmakeEnvironmentV1({[name]:"escape"})).toThrow();}finally{db.close();}});
const probeCode="const{DatabaseSync}=require('node:sqlite'),fs=require('node:fs'),crypto=require('node:crypto');const busy=p=>{const d=new DatabaseSync(p);try{d.exec('begin exclusive');d.exec('rollback');return false}catch(e){if(!/locked/.test(e.message))throw e;return true}finally{d.close()}};console.log(JSON.stringify({cpuBusy:busy(process.argv[1]),stateBusy:busy(process.argv[2]),sha:crypto.createHash('sha256').update(fs.readFileSync(process.argv[3])).digest('hex')}));";
/** 🧵️ Exercises the consumed production stage with honest Node endpoints and genuine ordinary state/input locks. */
test("actual CMake command stage releases CPU while retaining ordinary Source and state",()=>receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"cmake:ordinary",30000),output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Ticket ordinary output required");const root=mkdtempSync(join(output,"cm-")),file=join(root,"input.txt"),directory=root,source="actual retained ordinary CMake input",gate=resourceLeasePathV1(directory,capacitySchema.properties.resource.const),state=resourceLeasePathV1(directory,"cmake:ordinary-state");writeFileSync(file,source);let escaped:CmakeInputsV1|undefined,late=false;try{await withResourceLeases({directory,resources:[{resource:"cmake:ordinary-state",mode:"exclusive"}],control,signal:control.signal},()=>withCmakeInputsV1(new Map([["source/input.txt",file]]),control,async(inputs,actual)=>{escaped=inputs;const bytes=cmakeInputBytesV1(inputs,"source/input.txt");bytes.fill(0);expect(new TextDecoder().decode(cmakeInputBytesV1(inputs,"source/input.txt"))).toBe(source);expect(()=>cmakeInputFactsV1({...inputs})).toThrow();const command={program:"node",arguments:["-e",probeCode,gate,state,file],cwd:root};const pending=runCmakeOwnedStageV1({kind:"configure",directory,commands:[command ]},inputs,actual,process.env,async(rows,consumer)=>{const cpu=JSON.parse(rows[0]!.stdout);expect(cpu).toMatchObject({cpuBusy:true,stateBusy:true,sha:createHash("sha256").update(source).digest("hex")});const result=await captureOwnedCommandResultV1("node",["-e",probeCode,gate,state,file],root,{control:consumer,output:{maximumBytes:65536,maximumLines:16},env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"}});expect(result.status,result.stderr).toBe(0);expect(result.signal).toBeNull();const after=JSON.parse(result.stdout);expect(after).toEqual({...cpu,cpuBusy:false});void withRequestedIssuedResourceControlV1(consumer,0,async phase=>{await phase.yieldContinuation();expect(cmakeInputFactsV1(inputs).length).toBe(1);late=true;});return after;});command.arguments.splice(0,command.arguments.length,"--version");const observed=await pending;expect(observed.stateBusy).toBe(true);await runCmakeOwnedStageV1({kind:"test",directory,commands:[{program:"node",arguments:["-e",probeCode,gate,state,file],cwd:root}]},inputs,actual,process.env,async rows=>{expect(JSON.parse(rows[0]!.stdout)).toMatchObject({cpuBusy:false,stateBusy:true});});}));expect(late).toBe(true);expect(()=>cmakeInputFactsV1(escaped!)).toThrow();const db=new Database(state);try{db.exec("begin exclusive");db.exec("rollback");}finally{db.close();}for(const kind of fixture.failures){expect(new Ajv({strict:true}).validate(schema.definitions.Failure,kind)).toBe(true);const child=await captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url)),root,kind],root,{control,output:{maximumBytes:65536,maximumLines:128},env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"}});expect(child.status,child.stderr).toBe(1);expect(child.signal).toBeNull();expect(JSON.parse(child.stdout)).toEqual({kind,exact:true,late:true,retired:true});}}finally{rmSync(root,{recursive:true,force:true});}}),30000);
/** 🧪️ Requires ctest and probes to use genuine requested phases outside the compiler scope. */
test("neutral actual CMake stage and output consumers preserve the closed operation boundary",()=>{const stage=readFileSync(new URL("../🎬️stage/🟦️.ts",import.meta.url),"utf8"),execution=readFileSync(new URL("../🏃️execution/🟦️.ts",import.meta.url),"utf8");expect(stage).toContain('request.kind==="probe"||request.kind==="test"');expect(stage).toContain("return()=>consumeCompleted(results)");expect(stage).toContain("withRequestedIssuedResourceControlV1(control,0");expect(stage).toContain('request:{kind:"whole-host"}');expect(stage).toContain("captureOwnedCommandResultV1(");expect(stage).toContain("scriptResourceProcessEnvironmentV1(actual,");expect(execution).toContain('kind:"test"');expect(execution).toContain('jsonSchemaSubsetValueEquals(receipt,{...expected,phase:"built"})');expect(execution).toContain('const sourceRoot=requireNativeStoragePathV1');expect(execution).toContain("requireRoot(staged)");expect(execution).toContain("realpathSync(actual.sourceRoot)!==realpathSync(sourceRoot)");expect(execution).not.toContain('spawnSync');});

/** 📐️ Compares actual finite cache decoding with independent SQLite line and key projection. */
test("neutral CMake consumed cache facts match independent SQLite and mode schema",()=>{const db=new Database(":memory:");try{for(const row of fixture.caches){const oracle=db.query("with recursive lines(line,rest) as(select '',?1 union all select substr(rest,1,instr(rest,char(10))-1),substr(rest,instr(rest,char(10))+1) from lines where instr(rest,char(10))>0),facts as(select substr(line,1,instr(line,':')-1) name,rtrim(substr(line,instr(line,'=')+1),char(13)) value from lines where instr(line,':')>0 and instr(line,'=')>instr(line,':')) select count(*) n,count(distinct name) d,max(case name when 'CMAKE_HOME_DIRECTORY' then value end) sourceRoot,max(case name when 'CMAKE_CXX_COMPILER' then value end) compiler,max(case name when 'CMAKE_BUILD_TYPE' then value end) configuration,max(case name when 'CMAKE_GENERATOR' then value end) generator from facts where name in ('CMAKE_HOME_DIRECTORY','CMAKE_CXX_COMPILER','CMAKE_BUILD_TYPE','CMAKE_GENERATOR')").get(row.text)as{n:number;d:number;sourceRoot:string;compiler:string;configuration:string;generator:string};const{n,d,...facts}=oracle;expect(n===4&&d===4&&new Ajv({strict:true}).validate(schema.definitions.CacheFacts,facts)).toBe(row.accepted);if(row.accepted){expect(readCmakeCacheFactsV1(row.text)).toEqual(facts);expect(facts).toEqual(row.facts);}else expect(()=>readCmakeCacheFactsV1(row.text)).toThrow();}}finally{db.close();}});

import { parseEnv as parse } from "node:util";
import { compilerEnvironmentFromOutput } from "../🪟️toolchain/🟦️.ts";
const toolchainFixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8")), validateToolchain=new Ajv2020({strict:false}).compile(schema.definitions.NativePresets);
test("all native host presets use the compiler-only generator and agree with Ajv", () => {
  for (const row of toolchainFixture.presets) {
    const native = row.preset.hidden === true || row.preset.generator === "Ninja" && !Object.hasOwn(row.preset, "architecture") && ["Debug", "Release"].includes(row.preset.cacheVariables?.CMAKE_BUILD_TYPE);
    expect(native, row.name).toBe(row.accepted);
    expect(validateToolchain({ configurePresets: [row.preset] }), row.name).toBe(row.accepted);
  }
  const root=process.cwd(), presets = JSON.parse(readFileSync(join(root, "CMakePresets.json"),"utf8"));
  expect(validateToolchain(presets), JSON.stringify(validateToolchain.errors)).toBe(true);
  expect(presets.configurePresets.filter((preset: { hidden?: boolean }) => !preset.hidden)).toHaveLength(6);
  const bootstrap = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1"), "utf8");
  expect(bootstrap).toContain('Sync-WingetPackage -Id "Microsoft.VisualStudio.BuildTools"');
  expect(bootstrap).not.toContain('Sync-WingetPackage -Id "Microsoft.VisualStudio.Community"');
  console.log("[DEBUG] all six native CMake presets use Ninja; Windows setup installs standalone Build Tools");
});

test("compiler environment vectors agree with the independent Node dotenv-format parser", () => {
  for (const row of toolchainFixture.environments) {
    const actual = compilerEnvironmentFromOutput(row.output, row.base);
    expect(actual, row.name).toEqual(row.expected);
    if (row.oracle) {
      const normalized = Object.fromEntries(Object.entries(row.base).filter(([key]) => !Object.keys(parse(row.output)).some(name => name.toLowerCase() === key.toLowerCase())));
      expect({ ...normalized, ...parse(row.output) }, row.name).toEqual(actual);
    }
  }
  console.log(`[DEBUG] verified ${toolchainFixture.environments.length} shared compiler environment vectors`);
});

