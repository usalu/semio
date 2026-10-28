import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {BAZEL_KEYED_COMMAND_POLICY_V1,bazelKeyedCommandEnvironmentV1,runBazelRuntimeCommandV1} from "../🟦️.ts";
import {REPO_PROFILE_VARIABLE_V1} from "../../../../⏱️profile/🟨️.mjs";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../📥️caller/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import witnessSchema from "../../../../🔭️witness/🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url));

test("The keyed command environment drops every diagnostic switch and every ambient variable, keeps every other name, and an independent Python oracle agrees",()=>{
 expect(BAZEL_KEYED_COMMAND_POLICY_V1.diagnostic).toContain(REPO_PROFILE_VARIABLE_V1);
 expect(witnessSchema.definitions.Policy.const.ignoredEnvironment).toContain(REPO_PROFILE_VARIABLE_V1);
 const actual=fixture.environments.map(row=>Object.keys(bazelKeyedCommandEnvironmentV1(row.environment)).sort());
 expect(actual).toEqual(fixture.environments.map(row=>[...row.kept].sort()));
 const python=spawnSync(process.platform==="win32"?"python":"python3",["-I",join(here,"🧮️resources","🐍️oracle","🐍️.py"),join(here,"..","🧬️schema","🔣️.json"),join(here,"..","..","..","..","🔭️witness","🧬️schema","🔣️.json")],{input:JSON.stringify(fixture),encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);
 expect(JSON.parse(python.stdout)).toEqual(actual);
 for(const language of ["en","de"] as const)expect(schema.definitions.Messages.const[language].failed).toContain("{runtime}");
});

const program=`
const noise=process.env.${REPO_PROFILE_VARIABLE_V1}!==undefined?JSON.stringify({pid:process.pid,total:performance.now()}):"";
process.stdout.write("v1.2.3\\n");
if(noise)process.stderr.write(noise+"\\n");
process.stderr.write(JSON.stringify({session:process.env.CLAUDE_FIXTURE_SESSION??null,at:Date.now(),pid:process.pid})+"\\n");
if(process.env.FIXTURE_FAIL)process.exit(Number(process.env.FIXTURE_FAIL));
`;

test("A runtime command is keyed by its standard output alone: the profile switch, an agent variable, timestamps and pids on standard error never change it, and the old merged tail did",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"keyed-"));
 try{
  writeFileSync(join(directory,"fixture.ts"),program);
  const command=JSON.stringify(process.execPath)+" "+JSON.stringify(join(directory,"fixture.ts")),root=process.cwd();
  const merged=async(extra:Record<string,string>)=>await captureOwnedCommandV1(process.execPath,["exec",command],root,{env:{...process.env,...extra}});
  const first=await merged({[REPO_PROFILE_VARIABLE_V1]:"json"}),second=await merged({[REPO_PROFILE_VARIABLE_V1]:"json"});
  expect(first).not.toBe(second);
  const capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{});
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:keyed-command",maximumElapsedMilliseconds:60000},capabilities,Date.now()),async original=>{
   const control=createIssuedResourceControlV1(original,"bazel:keyed-command",0),keyed=(extra:Record<string,string>)=>runBazelRuntimeCommandV1(root,command,{...process.env,...extra},control),outputs=[
    await keyed({}),await keyed({[REPO_PROFILE_VARIABLE_V1]:"json"}),await keyed({[REPO_PROFILE_VARIABLE_V1]:"json"}),await keyed({[REPO_PROFILE_VARIABLE_V1]:"1"}),await keyed({CLAUDE_FIXTURE_SESSION:"agent-7",XPC_FLAGS:"0x1"}),
   ];
   expect(new Set(outputs).size).toBe(1);
   expect(outputs[0]).toBe("v1.2.3\n");
   await expect(keyed({FIXTURE_FAIL:"3"})).rejects.toThrow(/Runtime input .* exited with status 3 .* \/ Laufzeiteingabe .* endete mit Status 3/su);
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);
