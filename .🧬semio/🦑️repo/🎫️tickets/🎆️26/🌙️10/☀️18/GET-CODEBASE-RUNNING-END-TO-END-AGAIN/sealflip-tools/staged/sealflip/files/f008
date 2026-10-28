import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import Ajv from "ajv";
import yargsParser from "yargs-parser";
import catalogue from "../../⚙️operations/🔣️catalog.json" with {type:"json"};
import policy from "../🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {findWorkspaceRoot} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {buildDoorLightLoadersV1,buildDoorRequestsHelpV1,findBuildDoorRootV1,planBuildDoorV1,renderBuildDoorRefusalV1,runBuildDoorV1,selectBuildDoorArgumentsV1,type BuildDoorPlanV1,type BuildDoorPolicyV1} from "../🟦️.ts";
import {buildHelpForArgumentsV1} from "../../⚙️operations/📖️help/🟦️.ts";
import {buildInvocationRequestsHelpV1,parseBuildInvocationV1} from "../../🧭️invocation/🟦️.ts";

const scratch=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"door-"));
afterAll(()=>rmSync(scratch,{recursive:true,force:true}));
const operations=catalogue.operations.map(operation=>operation.name).join(", "),subcommandsOf=(name:string)=>(catalogue.operations.find(operation=>operation.name===name)?.subcommands??[]).map(row=>row.name).join(", "),rules=policy as unknown as BuildDoorPolicyV1;
const entry=resolve(import.meta.dir,"../📜️script.ts"),script=resolve(import.meta.dir,"../../📜️script.ts"),cleanEnvironment={...Object.fromEntries(Object.entries(process.env).filter(([key])=>!key.startsWith("SEMIO_SCRIPT")&&!key.startsWith("SEMIO_BUILD_")&&key!=="SEMIO_LANGUAGE")),SEMIO_REPO_SEAL:"live"};
const run=(program:string,args:readonly string[],language:string,extra:Record<string,string>={})=>spawnSync(process.execPath,[program,...args],{encoding:"utf8",timeout:60_000,env:{...cleanEnvironment,SEMIO_LANGUAGE:language,...extra}});

test("the policy is closed, every refusal is bilingual with the same placeholders, and it only names what the catalogue declares",()=>{
 const ajv=new Ajv({strict:true,allErrors:true}),validate=ajv.compile(schema);
 expect(validate(policy),JSON.stringify(validate.errors)).toBe(true);
 expect(validateJsonSchemaSubset(schema,policy)).toEqual([]);
 const mutate=(change:(copy:any)=>void)=>{const copy=structuredClone(policy);change(copy);return copy;};
 for(const bad of [mutate(copy=>{copy.light[0].call="spawn";}),mutate(copy=>{delete copy.texts.invalidValue.de;}),mutate(copy=>{copy.extra=1;}),mutate(copy=>{copy.refusedExitCode=1;}),mutate(copy=>{copy.light.push({operation:"Leases",export:"x",call:"args-options"});})]){
  expect(validate(bad)).toBe(false);
  expect(validateJsonSchemaSubset(schema,bad).length).toBeGreaterThan(0);
 }
 const names=new Map(catalogue.operations.map(operation=>[operation.name,operation]));
 expect(Object.keys(buildDoorLightLoadersV1).sort()).toEqual(rules.light.map(row=>row.operation).sort());
 for(const row of rules.light)expect(names.get(row.operation)?.requires,row.operation).toBe("none");
 for(const name of rules.closedSubcommands)expect((names.get(name) as {subcommands?:unknown[]}|undefined)?.subcommands?.length,name).toBeGreaterThan(0);
 for(const id of Object.keys(rules.texts) as (keyof BuildDoorPolicyV1["texts"])[]){
  expect(Object.keys(rules.texts[id]).sort(),id).toEqual([...catalogue.languages].sort());
  const placeholders=(text:string)=>[...text.matchAll(/\{([A-Za-z]+)\}/gu)].map(match=>match[1]).sort().join(",");
  expect(new Set(catalogue.languages.map(language=>placeholders(rules.texts[id][language]!))).size,id).toBe(1);
 }
 expect(catalogue.flags.find(row=>row.name==="--workers")?.valueName).toMatch(/\|/u);
 expect(catalogue.flags.find(row=>row.name==="--tool")?.valueName,"the build implementations come from the backend registry, so the door cannot close the list").not.toMatch(/\|/u);
});

test("every route vector plans as declared, in English and German, from the catalogue alone",()=>{
 for(const row of fixture.plans){
  const plan=planBuildDoorV1(row.args,{SEMIO_LANGUAGE:row.language,...(row as {environment?:Record<string,string>}).environment}),expected=structuredClone(row.route) as {values?:Record<string,string>};
  if(expected.values?.known==="@operations")expected.values.known=operations;
  if(expected.values?.known?.startsWith("@subcommands:"))expected.values.known=subcommandsOf(expected.values.known.slice(13));
  expect(plan,row.name).toEqual({language:row.language,route:expected} as BuildDoorPlanV1);
 }
});

test("the selection equals the invocation parser and the command equals yargs-parser, an independent reader",()=>{
 for(const row of fixture.plans){
  const typed=row.args,prefix=typed[0]==="repo"?1:0,body=typed.slice(prefix),environment={SEMIO_LANGUAGE:row.language,...(row as {environment?:Record<string,string>}).environment} as NodeJS.ProcessEnv;
  if(["build","test"].includes(typed[0]!)||row.route.kind==="refused"&&(row.route as {id:string}).id==="invalidValue")continue;
  const selection=selectBuildDoorArgumentsV1(body,environment);
  if("refused" in selection)throw Error(row.name);
  expect(selection.args,row.name).toEqual(parseBuildInvocationV1(body,environment).args);
  expect(buildDoorRequestsHelpV1(selection.args),row.name).toBe(buildInvocationRequestsHelpV1(selection.args));
  const boundary=body.indexOf("--"),head=body.slice(0,boundary<0?undefined:boundary),parsed=yargsParser(head,{string:["tool","workers","graph-policy","lang"],boolean:["help"],alias:{help:["h"]}});
  const first=selection.args[0]??"";expect(parsed._[0]===undefined?"":String(parsed._[0]),row.name).toBe(first.startsWith("-")?"":first);
  expect(parsed.help===true||selection.args.length===0,row.name).toBe(buildDoorRequestsHelpV1(selection.args));
 }
});

test("a refusal renders in every catalogue language exactly as the fixture states",()=>{
 for(const row of fixture.refusals)for(const language of catalogue.languages)expect(renderBuildDoorRefusalV1({kind:"refused",id:row.id as "unknownCommand",values:Object.fromEntries(Object.entries(row.values).flatMap(([key,value])=>value===undefined?[]:[[key,value]]))},language),`${row.id} ${language}`).toBe(row.text[language as "en"|"de"]);
 expect(()=>renderBuildDoorRefusalV1({kind:"refused",id:"invalidValue",values:{}},"fr")).toThrow(/No invalidValue text for fr/u);
});

test("the front door acts on its plan: help is rendered, refusals go to the error stream with the policy exit code, light commands get their declared call, the rest asks for the full implementation",async()=>{
 const log:{write:string[];error:string[];exit:number[];load:string[];call:unknown[]}={write:[],error:[],exit:[],load:[],call:[]},environment={SEMIO_LANGUAGE:"de"} as NodeJS.ProcessEnv;
 const ports=(modules:Record<string,Record<string,unknown>>)=>({write:(line:string)=>log.write.push(line),error:(line:string)=>log.error.push(line),exit:(code:number)=>log.exit.push(code),load:async(operation:string)=>{log.load.push(operation);return modules[operation]!;}});
 const modules={leases:{runLeasesCliV1:async(...call:unknown[])=>{log.call.push(["leases",...call.slice(0,3)]);return 3;}},doctor:{runDoctorCliV1:async(...call:unknown[])=>{log.call.push(["doctor",...call.slice(0,3)]);return 4;}},capacity:{runCapacityCommandV1:async(...call:unknown[])=>{log.call.push(["capacity",call[0],(call[1] as {environment:{SEMIO_LANGUAGE:string}}).environment.SEMIO_LANGUAGE]);return 5;}}};
 expect(await runBuildDoorV1("/w",["repo","run","--help"],environment,ports(modules))).toBe("handled");
 expect(log.write).toEqual([buildHelpForArgumentsV1(["run","--help"],environment)]);
 expect(await runBuildDoorV1("/w",["repo","tools","status"],environment,ports(modules))).toBe("handled");
 expect(log.error).toEqual([rules.texts.unknownSubcommand.de!.replace("{subcommand}","status").replaceAll("{command}","tools").replace("{known}",subcommandsOf("tools"))]);
 expect(log.exit).toEqual([rules.refusedExitCode]);
 expect(await runBuildDoorV1("/w",["repo","leases","status","--json"],environment,ports(modules))).toBe("handled");
 expect(await runBuildDoorV1("/w",["repo","doctor","--json"],environment,ports(modules))).toBe("handled");
 expect(await runBuildDoorV1("/w",["repo","capacity","status"],environment,ports(modules))).toBe("handled");
 expect(log.load).toEqual(["leases","doctor","capacity"]);
 expect(log.call).toEqual([["leases","/w",["status","--json"],{...environment,SEMIO_LANGUAGE:"de"}],["doctor","/w",["--json"],{...environment,SEMIO_LANGUAGE:"de"}],["capacity",["status"],"de"]]);
 expect(log.exit).toEqual([rules.refusedExitCode,3,4,5]);
 const before=log.load.length;
 expect(await runBuildDoorV1("/w",["repo","run","a:b"],environment,ports(modules))).toBe("full");
 expect(await runBuildDoorV1("/w",["build","x"],environment,ports(modules))).toBe("full");
 expect(log.load.length).toBe(before);
 expect(await runBuildDoorV1("/w",["repo","--lang=fr","leases","status"],{},ports(modules))).toBe("handled");
 expect(log.error.at(-1)).not.toBe("");
 expect(log.exit.at(-1)).toBe(rules.refusedExitCode);
});

test("the front door finds the workspace root exactly as the routing module does",()=>{
 expect(findBuildDoorRootV1(import.meta.dir)).toBe(findWorkspaceRoot(import.meta.dir));
 for(const row of fixture.roots){
  const base=mkdtempSync(join(scratch,"root-"));
  for(const [name,text] of Object.entries(row.files)){mkdirSync(dirname(join(base,name)),{recursive:true});writeFileSync(join(base,name),text);}
  const start=join(base,row.start);
  if(row.expected===null){
   const messages=[()=>findBuildDoorRootV1(start,{}),()=>findWorkspaceRoot(start)].map(call=>{try{call();return "";}catch(error){return (error as Error).message;}});
   expect(messages[0],row.name).not.toBe("");
   expect(messages[0],row.name).toBe(messages[1]!);
  }else{
   expect(findBuildDoorRootV1(start,{}),row.name).toBe(join(base,row.expected));
   expect(findBuildDoorRootV1(start,{}),row.name).toBe(findWorkspaceRoot(start));
  }
 }
});

test("the real front door prints the same help as the build script, refuses before any storage in both languages, and delegates a full command",()=>{
 for(const operation of ["leases","capacity","doctor"])for(const language of ["en","de"]){
  const door=run(entry,["repo",operation,"--help"],language),old=run(script,["repo",operation,"--help"],language);
  expect(door.status,`${operation} ${language}`).toBe(0);
  expect(door.stdout,`${operation} ${language}`).toBe(old.stdout);
  expect(door.stderr,`${operation} ${language}`).toBe("");
 }
 const bare=run(entry,["repo"],"en");
 expect(bare.status).toBe(0);
 expect(bare.stdout).toBe(`${buildHelpForArgumentsV1([],{SEMIO_LANGUAGE:"en"})}\n`);
 const german=run(entry,["repo","run","--help","--lang=de"],"en");
 expect(german.stdout).toContain("Verwendung");
 const refused=run(entry,["repo","tools","status"],"de");
 expect(refused.status).toBe(rules.refusedExitCode);
 expect(refused.stdout).toBe("");
 expect(refused.stderr).toContain("Unbekannter Unterbefehl");
 const unknown=run(entry,["repo","nonsense"],"en");
 expect(unknown.status).toBe(rules.refusedExitCode);
 expect(unknown.stderr).toContain(operations);
});
