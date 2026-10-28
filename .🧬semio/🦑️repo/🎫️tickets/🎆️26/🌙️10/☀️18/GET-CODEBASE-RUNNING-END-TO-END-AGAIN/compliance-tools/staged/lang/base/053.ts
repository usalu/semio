import{test,expect}from"bun:test";
import {testPythonV1} from "../../🧰️toolchain/🟦️.ts";
import{mkdtemp,rm}from"node:fs/promises";
import{join}from"node:path";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import corpus from"./🔣️.json"with{type:"json"};
import lawSchema from"../🧬️schema/📐️law.json"with{type:"json"};
import schema from"./schema.json"with{type:"json"};
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{scriptInvocationEnvironment,createIssuedResourceControlV1}from"../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{resourceLeasePathV1}from"../../../🔒️leases/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../🎛️owned-execution/📥️capture/🟦️.ts";
import{captureOwnedCommandV1}from"../../../🎛️owned-execution/🟦️.ts";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../🟦️.ts";
import{spawnSync}from"node:child_process";
import{existsSync}from"node:fs";
import{tmpdir}from"node:os";
declareBuildTestEnvelopeV1({owner:"test:testing",maximumElapsedMilliseconds:300000,capabilities:{artifacts:true}});
test("build receiver neutral lifetime corpus matches Ajv and SQLite order",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);expect(validateJsonSchemaSubset(schema,corpus)).toEqual([]);const plans=lawSchema.properties.plans.const;expect(new Ajv({strict:true}).validate(lawSchema,{version:1,plans})).toBe(true);expect(Object.values(plans).every(row=>row.totalMilliseconds===row.bodyMilliseconds+row.drainMilliseconds)).toBe(true);const db=new Database(":memory:");try{db.exec("create table phase(name text primary key,position integer)");corpus.order.forEach((name,position)=>db.query("insert into phase values(?,?)").run(name,position));expect(db.query("select name from phase order by position").all()).toEqual(corpus.order.map(name=>({name})));expect(db.query("select a.position<b.position as ordered from phase a,phase b where a.name='lease-released' and b.name='cleanup'").get()).toEqual({ordered:1});}finally{db.close();}});
for(const row of[...corpus.cases.map(row=>({...row,suite:""})),...corpus.suiteCases.map(row=>({...row,suite:row.name,name:"normal"}))])test("build receiver actual child "+(row.suite?"suite "+row.suite:row.name),()=>withBuildTestInvocationV1(async original=>{
 const facts=original.capabilities as{artifacts?:unknown};if(typeof facts.artifacts!=="string")throw Error("Authored receiver fixture artifacts required");const directory=await mkdtemp(join(facts.artifacts,"receiver-")),control=createIssuedResourceControlV1(original,"fixture:receiver-child",20000);try{
  const result=await captureOwnedCommandResultV1(process.execPath,["test",join(import.meta.dir,"🧫️harness/🟦️.ts")],process.cwd(),{control,env:scriptInvocationEnvironment(original,{...process.env,SEMIO_RECEIVER_CASE:row.name,SEMIO_RECEIVER_SUITE:row.suite,SEMIO_RECEIVER_DIRECTORY:directory}),output:{maximumBytes:65536,maximumLines:512}});process.stdout.write(result.stdout);process.stderr.write(result.stderr);
  const events=result.stdout.split(/\r?\n/u).filter(line=>line.startsWith("[TRACE] Receiver event ")).map(line=>JSON.parse(line.slice("[TRACE] Receiver event ".length))),names=events.map(event=>event.phase),python=testPythonV1()??Bun.which("python");if(!python)throw Error("Independent Python SQLite oracle required");
  if(row.suite==="early"){expect(result.status).toBe(1);expect(result.stderr).toContain("before receiver close");expect(names).toContain("late-suite-refused");return;}expect(result.status).toBe(row.status);expect(result.signal).toBe(null);expect(result.stderr).not.toContain("Unhandled error between tests");expect(names).not.toContain("late-effect");expect(names).not.toContain("late-birth-accepted");expect(names).toContain("independent-ex-blocked");expect(names).toContain("late-birth-refused");expect(events.find(event=>event.phase==="cleanup")?.released).toBe(true);expect(names.lastIndexOf("lease-released")).toBeLessThan(names.indexOf("cleanup"));expect(events.filter(event=>event.phase==="alias").every(event=>event.samePolicy&&event.differentSignal)).toBe(true);
  if(row.suite){expect(names).toContain("suite-released");expect(names).toContain("late-suite-refused");expect(names.indexOf("suite-released")).toBeLessThan(names.indexOf("cleanup"));if(row.suite==="normal"||row.suite==="detached"){expect(names).toContain("suite-user-drained");expect(names.indexOf("suite-user-drained")).toBeLessThan(names.indexOf("suite-released"));}}
  if(row.name==="timeout"){expect(names).toContain("cancelled");expect(result.stderr).toContain("timed out");expect(result.stderr).toContain("Previous build test law remains unsettled");}
  if(row.name==="funded"){expect(names).toContain("cancelled");expect(result.stderr).toContain("body deadline exhausted");expect(result.stderr).not.toContain("timed out");}
  if(row.name==="error"||row.name==="undefined"||row.name.startsWith("registered-")||row.name==="detached"||row.suite==="error"||row.suite==="undefined"||row.suite==="detached")expect(events.find(event=>event.phase==="primary")?.preserved).toBe(true);
  if(row.name==="registered-undefined")expect(result.stderr).toContain("rejected with undefined");
  const value=await captureOwnedCommandV1(python,["-c","import sqlite3,sys,json\nfor path in sys.argv[1:]:\n c=sqlite3.connect('file:'+path+'?mode=rw',uri=True,timeout=0);c.execute('BEGIN EXCLUSIVE');c.close()\nprint(json.dumps(True))",resourceLeasePathV1(directory,"receiver-gate"),...(row.suite?[resourceLeasePathV1(directory,"receiver-suite")]:[])],directory,{control,output:{maximumBytes:4096,maximumLines:16}});expect(JSON.parse(value)).toBe(true);
 }finally{await rm(directory,{recursive:true});}
}),30000);
/** 🪪️ A file that declares its birth runs bare under exactly the declared capabilities (a private artifact directory published as SEMIO_TEST_ARTIFACT_DIR and gone afterwards, or the configured one kept), one process accepts only one birth, an undeclared bare file is still refused, and beneath a real parent the inherited capabilities must carry the declared ones. */
test("a declared birth runs a bare file under exactly its declared capabilities and is held to them beneath a real parent",()=>withBuildTestInvocationV1(async original=>{
 const facts=original.capabilities as{artifacts?:unknown};if(typeof facts.artifacts!=="string")throw Error("Authored declared-birth fixture artifacts required");
 const fixture=join(import.meta.dir,"🧫️declared/🟦️.ts"),control=createIssuedResourceControlV1(original,"fixture:declared-birth",120000),bare=Object.fromEntries(Object.entries(process.env).filter(([name])=>name!=="SEMIO_SCRIPT_PROCESS_INVOCATION"&&name!=="SEMIO_TEST_ARTIFACT_DIR")),configured=await mkdtemp(join(facts.artifacts,"declared-"));
 const run=(kind:string,env:Record<string,string|undefined>)=>{const result=spawnSync(process.execPath,["test",fixture],{cwd:process.cwd(),env:{...env,TMPDIR:tmpdir(),SEMIO_DECLARED_CASE:kind},encoding:"utf8",timeout:60000});return Promise.resolve({status:result.status,stdout:result.stdout,stderr:result.stderr});},child=(kind:string)=>captureOwnedCommandResultV1(process.execPath,["test",fixture],process.cwd(),{control,env:scriptInvocationEnvironment(original,{...bare,TMPDIR:tmpdir(),SEMIO_DECLARED_CASE:kind}),output:{maximumBytes:65536,maximumLines:512}});
 const births=(stdout:string)=>stdout.split(/\r?\n/u).filter(line=>line.startsWith("[TRACE] Declared birth ")).map(line=>JSON.parse(line.slice("[TRACE] Declared birth ".length)) as{artifacts:string|null;command:string|null;published:string|null;exists:boolean});
 try{
  const owned=await run("writer",bare),[birth]=births(owned.stdout);expect(owned.status,owned.stderr.slice(-4000)).toBe(0);expect(births(owned.stdout).length).toBe(1);expect(birth!.command).toBe("writer");expect(typeof birth!.artifacts).toBe("string");expect(birth!.published).toBe(birth!.artifacts);expect(birth!.exists).toBe(true);expect(existsSync(birth!.artifacts!)).toBe(false);
  const kept=await run("writer",{...bare,SEMIO_TEST_ARTIFACT_DIR:configured});expect(kept.status,kept.stderr.slice(-4000)).toBe(0);expect(births(kept.stdout)).toEqual([{artifacts:configured,command:"writer",published:configured,exists:true}]);expect(existsSync(configured)).toBe(true);
  const repeated=await run("repeated",bare);expect(repeated.status,repeated.stderr.slice(-4000)).toBe(0);expect(births(repeated.stdout).length).toBe(1);
  const different=await run("different",bare);expect(different.status).not.toBe(0);expect(different.stderr).toContain("One test process receives one birth");expect(births(different.stdout)).toEqual([]);
  const undeclared=await run("undeclared",bare);expect(undeclared.status).not.toBe(0);expect(undeclared.stderr).toContain("Original parent Script process invocation required");expect(births(undeclared.stdout)).toEqual([]);
  const inherited=await child("artifacts");expect(inherited.status,inherited.stderr.slice(-4000)).toBe(0);expect(births(inherited.stdout)).toEqual([{artifacts:facts.artifacts,command:null,published:null,exists:true}]);
  const drifted=await child("writer");expect(drifted.status).not.toBe(0);expect(drifted.stderr).toContain("Inherited build test capabilities differ from the declared envelope of test:declared-writer");expect(births(drifted.stdout)).toEqual([]);
 }finally{await rm(configured,{recursive:true,force:true});}
}),120000);

/** 🎫️ A declared grant is issued by its registered issuer to a bare file (environment prepared, capabilities carried, released and restored afterwards), accepted beneath a real parent that carries exactly that grant, and refused beneath a parent that does not, for an unregistered issuer and for a malformed declaration. */
test("a declared grant is issued to a bare file, accepted beneath a parent carrying it and refused otherwise",()=>withBuildTestInvocationV1(async original=>{
 const fixture=join(import.meta.dir,"🧫️declared/🟦️.ts"),control=createIssuedResourceControlV1(original,"fixture:declared-grant",120000),bare=Object.fromEntries(Object.entries(process.env).filter(([name])=>name!=="SEMIO_SCRIPT_PROCESS_INVOCATION"&&name!=="SEMIO_TEST_ARTIFACT_DIR"));
 const run=(kind:string)=>{const result=spawnSync(process.execPath,["test",fixture],{cwd:process.cwd(),env:{...bare,TMPDIR:tmpdir(),SEMIO_DECLARED_CASE:kind},encoding:"utf8",timeout:60000});return{status:result.status,stdout:result.stdout,stderr:result.stderr};};
 const lines=(stdout:string,name:string)=>stdout.split(/\r?\n/u).filter(line=>line.startsWith("[TRACE] Declared "+name+" ")||line==="[TRACE] Declared "+name).map(line=>line.slice(("[TRACE] Declared "+name).length).trim()).map(text=>text===""?null:JSON.parse(text));
 const fake={tool:"fake",arguments:["repo","run","fixture:declared"],fake:"prepared",artifacts:null};
 const granted=run("granted");expect(granted.status,granted.stderr.slice(-4000)).toBe(0);expect(lines(granted.stdout,"grant")).toEqual([fake]);expect(lines(granted.stdout,"release")).toEqual([{fake:null}]);
 const nested=run("granted-nested");expect(nested.status,nested.stderr.slice(-4000)).toBe(0);expect(lines(nested.stdout,"grant")).toEqual([fake,{...fake}]);expect(lines(nested.stdout,"release")).toEqual([{fake:null}]);
 const inherited=await captureOwnedCommandResultV1(process.execPath,["test",fixture],process.cwd(),{control,env:scriptInvocationEnvironment(original,{...bare,TMPDIR:tmpdir(),SEMIO_DECLARED_CASE:"granted"}),output:{maximumBytes:65536,maximumLines:512}});expect(inherited.status).not.toBe(0);expect(inherited.stderr).toContain("Inherited build test capabilities differ from the declared envelope of test:declared-granted");expect(lines(inherited.stdout,"grant")).toEqual([]);
 const unregistered=run("unregistered");expect(unregistered.status).not.toBe(0);expect(unregistered.stderr).toContain("Registered build test grant issuer required: native-build-caller");
 const malformed=run("malformed");expect(malformed.status).not.toBe(0);expect(malformed.stderr).toContain("Finite closed build test envelope required");
}),120000);
