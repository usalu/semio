/** 🔁️ Incremental build conformance: the evaluation model, verdicts, mutations, observers, runner and command are checked against language-agnostic fixtures, an independent Python oracle, Bazel's own binary (execution log, summary line, `rdeps`) and the installed Nx's own run-record class. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync,lstatSync,readlinkSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import scenarios from "../📋️scenarios/🔣️.json" with {type:"json"};
import {incrementalComparisonsV1,incrementalExpectedV1,incrementalLabelV1,incrementalMutateV1,incrementalQuiescentV1,incrementalRdepsExpressionV1,incrementalRdepsPackageV1,incrementalRdepsTasksV1,incrementalReportV1,incrementalScenarioIssuesV1,incrementalSliceV1,incrementalVerdictV1,renderIncrementalReportV1,renderIncrementalScenariosV1,type IncrementalExpectedV1,type IncrementalObservationV1,type IncrementalScenarioSetV1,type IncrementalSliceV1,type IncrementalToolV1} from "../🟦️.ts";
import {incrementalBazelObservationV1,incrementalBazelSpawnsV1,incrementalNxObservationV1} from "../👁️observers/🟦️.ts";
import {incrementalChildEnvironmentV1,incrementalCliArgumentsV1,incrementalCliPortsV1,incrementalCloneV1,incrementalDisposableV1,incrementalInsideV1,incrementalPlanTasksV1,incrementalRunScenarioV1,incrementalRunV1,incrementalSelectV1,incrementalSliceFromGraphV1,type IncrementalAdapterV1,type IncrementalHostV1,type IncrementalPortsV1} from "../🏃️runner/🟦️.ts";
import {incrementalArgumentsV1,incrementalDefaultRootV1,incrementalOptionNamesV1,renderIncrementalPlanV1,runIncrementalCommandV1,type IncrementalCommandPortsV1} from "../⌨️command/🟦️.ts";
import catalog from "../../⚙️operations/🔣️catalog.json" with {type:"json"};
import {bazelCommandPlanV1,bazelTaskReportOptionsV1} from "../../🔧️tools/🏗️bazel/🧭️commands/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const findRoot=(from:string):string=>{for(let at=from;;at=dirname(at)){if(existsSync(join(at,"bunfig.toml"))&&existsSync(join(at,"bun.lock")))return at;if(dirname(at)===at)throw Error("repository root not found");}};
const root=process.env.SEMIO_TEST_REPO_ROOT??findRoot(import.meta.dir),set=scenarios as unknown as IncrementalScenarioSetV1;
const ajv=new Ajv({strict:true,allowUnionTypes:true}),define=(name:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${name}`});
const oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),sens=fixture.sensitivity as never;
const python=(input:unknown):any=>{const run=spawnSync("python3",["-I",oracle],{input:JSON.stringify(input),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const slice=(value:unknown)=>value as IncrementalSliceV1;

test("the schema is valid and validates the fixture, the scenarios and every row the way the repository validator does",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 expect(define("Fixture")(fixture)).toBe(true);
 expect(define("ScenarioSet")(scenarios)).toBe(true);
 expect(validateJsonSchemaSubset(schema.definitions.ScenarioSet,scenarios,schema)).toEqual([]);
 for(const[name,value,ok]of [["Edit",{kind:"config",files:["a"]},true],["Edit",{kind:"config",files:[]},false],["Edit",{kind:"nope",files:["a"]},false],["Expected",{direct:[],must:["a:b"],may:["a:b"],cutoff:[]},true],["Expected",{direct:[],must:["ab"],may:[],cutoff:[]},false],["Verdict",{status:"pass",stale:[],excess:[],exact:true},true],["Verdict",{status:"ok",stale:[],excess:[],exact:true},false]] as const){
  expect(define(name)(value),name+JSON.stringify(value)).toBe(ok);
  expect(validateJsonSchemaSubset(schema.definitions[name],value,schema).length===0,name+JSON.stringify(value)).toBe(ok);
 }
});

test("the authored scenarios are closed, consistent, bilingual and cover every area and edit kind in at least eight scenarios",()=>{
 expect(incrementalScenarioIssuesV1(scenarios)).toEqual([]);
 expect(set.scenarios.length).toBeGreaterThanOrEqual(8);
 expect(new Set(set.scenarios.map(row=>row.area))).toEqual(new Set(schema.definitions.Scenario.properties.area.enum));
 expect(new Set<string>(set.scenarios.map(row=>row.kind))).toEqual(new Set<string>(schema.definitions.EditKind.enum));
 for(const label of Object.values(set.labels))expect(label.de,label.en).not.toBe(label.en);
 for(const row of set.scenarios){expect(row.title.de,row.id).not.toBe(row.title.en);expect(existsSync(join(root,row.mutation.file)),row.id+" "+row.mutation.file).toBe(true);}
 const duplicate=structuredClone(scenarios) as {scenarios:{id:string}[]};duplicate.scenarios.push({...duplicate.scenarios[0]!});
 expect(incrementalScenarioIssuesV1(duplicate).join()).toContain("duplicate scenario");
 const wrong=structuredClone(scenarios) as {scenarios:{mutation:{op:string}}[]};wrong.scenarios[0]!.mutation.op="whitespace";
 expect(incrementalScenarioIssuesV1(wrong).join()).toContain("does not realise");
 expect(renderIncrementalScenariosV1(set,"en")).toContain(set.scenarios[0]!.title.en);
 expect(renderIncrementalScenariosV1(set,"de")).toContain(set.scenarios[0]!.title.de);
});

test("the evaluation gives every fixture case its authored answer, in TypeScript and in the independent Python oracle",()=>{
 const answers=python(fixture).evaluation as IncrementalExpectedV1[];
 for(const[index,row]of fixture.evaluation.entries()){
  expect(incrementalExpectedV1(row.slice,row.tool as IncrementalToolV1,row.edit as never,sens),row.name).toEqual(row.expected as never);
  expect(answers[index],row.name).toEqual(row.expected as never);
  expect(row.expected.must.every(id=>row.expected.may.includes(id)),row.name).toBe(true);
  expect(row.expected.cutoff.some(id=>row.expected.may.includes(id)),row.name).toBe(false);
 }
});

test("Bazel and Nx differ exactly where a dependency snapshot reaches a task that does not consume its outputs",()=>{
 const pick=(name:string)=>fixture.evaluation.find(row=>row.name.startsWith(name))!;
 expect(pick("body change in a generator reaches every").expected.must).toContain("app:test");
 expect(pick("body change in a generator reaches only").expected.must).not.toContain("app:test");
 expect(pick("body change in a file only the builder reads reaches").expected.must).toContain("app:test");
 expect(pick("body change in a file only the builder reads leaves").expected.must).not.toContain("app:test");
 expect(incrementalQuiescentV1(fixture.evaluation[0]!.slice)).toEqual({direct:[],must:["shell:activate"],may:["shell:activate"],cutoff:[]});
});

test("slices that reference unknown tasks, cycle, or break the id and family contracts are refused",()=>{
 for(const row of fixture.refusals)expect(()=>incrementalSliceV1(row.value),row.name).toThrow();
 expect(()=>incrementalExpectedV1(fixture.refusals[0]!.value,"bazel",{kind:"config",files:["a"]},sens)).toThrow();
 expect(()=>incrementalExpectedV1(fixture.evaluation[0]!.slice,"bazel",{kind:"config",files:[]} as never,sens)).toThrow("Closed incremental edit required");
});

test("verdicts match the fixture and the Python oracle",()=>{
 const answers=python(fixture).verdicts;
 for(const[index,row]of fixture.verdicts.entries()){
  expect(incrementalVerdictV1(row.expected,row.executed),row.name).toEqual(row.verdict as never);
  expect(answers[index],row.name).toEqual(row.verdict);
 }
 expect(()=>incrementalVerdictV1({direct:[],must:["x:y"],may:[],cutoff:[]} as never,[])).not.toThrow();
 expect(()=>incrementalVerdictV1({direct:[],must:["x:y"],may:[]} as never,[])).toThrow();
});

test("mutations match the fixture and the Python oracle, always differ from the input and refuse an op a language cannot render",()=>{
 const answers=python(fixture).mutations;
 for(const[index,row]of fixture.mutations.entries()){
  const after=incrementalMutateV1(row.before,row.op as never,row.language as never,row.nonce);
  expect(after,row.name).toBe(row.after);expect(answers[index],row.name).toBe(row.after);expect(after,row.name).not.toBe(row.before);
  expect(after.startsWith(row.before),row.name).toBe(true);
 }
 expect(()=>incrementalMutateV1("a","private-body","text","k3j9x2a")).toThrow("unsupported for text");
 expect(()=>incrementalMutateV1("a","comment","ts","BAD NONCE")).toThrow("Closed incremental nonce required");
});

test("task labels equal the adapter's sha256 scheme and the Python oracle",()=>{
 const answers=python(fixture).labels;
 for(const[index,row]of fixture.labels.entries()){expect(incrementalLabelV1("t",row.task),row.task).toBe(row.label);expect(answers[index]).toBe(row.label);}
});

test("the child environment drops invocation, capability, build and Nx state and keeps the language and the tools",()=>{
 const answers=python(fixture).environments;
 for(const[index,row]of fixture.environments.entries()){expect(incrementalChildEnvironmentV1(row.parent as unknown as Record<string,string>,row.extra as unknown as Record<string,string>),row.name).toEqual(row.child as unknown as Record<string,string>);expect(answers[index]).toEqual(row.child);}
 expect(incrementalCliArgumentsV1("/bin/bun","/tmp/sinc","script.ts","bazel",["run","a:b"])).toEqual({command:"/bin/bun",args:["/tmp/sinc/script.ts","repo","--tool=bazel","run","a:b"]});
});

test("compact Bazel execution logs decode to the same spawns and observations as the Python wire decoder and Bazel's own summary",()=>{
 const answers=python(fixture).bazelLogs;
 for(const[index,row]of fixture.bazelLogs.entries()){
  const spawns=incrementalBazelSpawnsV1(Uint8Array.from(Buffer.from(row.compact,"base64")));
  expect(spawns,row.name).toEqual(row.spawns);expect(answers[index].spawns,row.name).toEqual(row.spawns);
  const observation=incrementalBazelObservationV1(spawns,row.tasks);
  expect({executed:observation.executed,restored:observation.restored,cached:observation.cached,unmapped:observation.unmapped},row.name).toEqual(row.observation);
 }
 const cold=fixture.bazelLogs[0]!,comment=fixture.bazelLogs[2]!,clean=fixture.bazelLogs[4]!;
 expect(cold.observation.executed.length).toBe(3);expect(comment.observation.executed).toEqual(["lib:generate"]);expect(comment.observation.cached.length).toBe(2);expect(clean.observation.restored.length).toBe(3);
 expect(()=>incrementalBazelSpawnsV1(Uint8Array.from([1,2,3]))).toThrow();
 expect(incrementalBazelObservationV1([{label:"@@//:other",mnemonic:"x",runner:"local",cacheHit:false}],["a:b"]).unmapped).toEqual(["//:other"]);
});

test("Nx run records decode like Nx's own record writer and refuse foreign shapes",()=>{
 const answers=python(fixture).nxRecords;
 for(const[index,row]of fixture.nxRecords.entries()){expect(incrementalNxObservationV1(row.record,row.tasks),row.name).toEqual({tool:"nx",...row.observation});expect(answers[index]).toEqual(row.observation);}
 expect(()=>incrementalNxObservationV1({tasks:[{taskId:"a:b",cacheStatus:"weird",status:0}]},["a:b"])).toThrow("Closed Nx run record required");
 expect(()=>incrementalNxObservationV1({},[])).toThrow("Closed Nx run record required");
});

const installed=(()=>{const path=join(root,"node_modules/nx/dist/src/tasks-runner/life-cycles/store-run-information-life-cycle.js");return existsSync(path)?realpathSync(path):undefined;})();
(installed?test:test.skip)("the installed Nx writes a run record that the observer reads: a miss executed, local and remote hits restored",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"incr-nx-")));
 try{
  writeFileSync(join(directory,"nx.json"),"{}\n");
  const source=`const {StoreRunInformationLifeCycle}=require(process.env.NX_LIFE);const out=[];const lc=new StoreRunInformationLifeCycle("nx run-many",d=>out.push(d),()=>"2026-10-20T00:00:00.000Z");
const t=(id)=>({id,hash:"h",target:{project:id.split(":")[0],target:id.split(":")[1]},startTime:1,endTime:2});
lc.startCommand();lc.startTasks([t("p:build"),t("q:build"),t("r:build")]);
lc.endTasks([{task:t("p:build"),status:"success",code:0},{task:t("q:build"),status:"local-cache",code:0},{task:t("r:build"),status:"remote-cache",code:0}]);lc.endCommand();process.stdout.write(JSON.stringify(out[0]));`;
  const run=spawnSync("node",["-e",source],{cwd:directory,encoding:"utf8",env:{...process.env,NX_LIFE:installed!,NX_WORKSPACE_ROOT_PATH:directory,NX_DAEMON:"false"} as NodeJS.ProcessEnv});
  expect(run.status,run.stderr).toBe(0);
  expect(incrementalNxObservationV1(JSON.parse(run.stdout),["p:build","q:build","r:build"])).toEqual({tool:"nx",executed:["p:build"],restored:["q:build","r:build"],cached:[],unmapped:[]});
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,real=binary&&existsSync(binary)?test:test.skip;
const fileOf=(id:string)=>id.replace(/[^a-z0-9]/gu,"_");
function bazelWorkspace(directory:string,sliceValue:IncrementalSliceV1,kind:string,files:readonly string[]):void{
 for(const file of new Set([...Object.values(sliceValue.tasks).flatMap(task=>task.inputs),...files])){mkdirSync(dirname(join(directory,file)),{recursive:true});writeFileSync(join(directory,file),"# header\nvalue = 1\n");}
 writeFileSync(join(directory,"MODULE.bazel"),'module(name = "incr_real", version = "0.0.0")\n');
 bazelBuild(directory,sliceValue,kind,"run1");
}
function bazelBuild(directory:string,sliceValue:IncrementalSliceV1,kind:string,run:string):void{
 const quote=(value:string)=>JSON.stringify(value),lines:string[]=[];
 lines.push(`exports_files([${[...new Set(Object.values(sliceValue.tasks).flatMap(task=>task.inputs))].map(quote).join(", ")}])`);
 for(const[id,task]of Object.entries(sliceValue.tasks)){
  const effect=!task.outputs?"identical":(fixture.sensitivity.find(row=>row.family===task.family&&row.kind===kind)?.effect??"unknown"),own=task.inputs.map(file=>`$(location ${file})`).join(" "),deps=task.dependsOn.map(other=>`$(location :${fileOf(other)}.out)`).join(" ");
  const body=!task.outputs?"echo ok > $@":effect==="identical"?`(cat ${deps} ${own} | grep -v -e '^#' -e '^$$' -e '^_incremental_' -e '^INCREMENTAL_') > $@ || true`:`cat ${deps} ${own} > $@`;
  lines.push(`genrule(name = ${quote(incrementalLabelV1("t",id))}, srcs = [${[...task.inputs,...task.dependsOn.map(other=>`:${fileOf(other)}.out`)].map(quote).join(", ")}], outs = [${quote(fileOf(id)+".out")}], cmd = ${quote(task.cache?body:body+" # "+run)}${task.cache?"":', tags = ["no-cache"]'})`);
 }
 writeFileSync(join(directory,"BUILD.bazel"),lines.join("\n")+"\n");
}
const bazelRun=(directory:string,output:string,log:string,query?:string)=>spawnSync(binary!,[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","--max_idle_secs=60","--host_jvm_args=-Xmx512m",query?"query":"build","--lockfile_mode=off","--incompatible_autoload_externally=",...(query?["--keep_going","--output=label","--noshow_progress",query]:["--noshow_progress",`--execution_log_compact_file=${log}`,"--","//:all"])],{cwd:directory,encoding:"utf8",env:{...process.env,LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}});
real("real Bazel executes exactly the tasks the model expects for every Bazel fixture case, and its summary line counts the same executions",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"incr-bz-"))),output=join(directory,"ob");
 try{
  for(const[index,row]of fixture.evaluation.entries()){
   if(row.tool!=="bazel")continue;
   const work=join(directory,"w"+index);mkdirSync(work);
   const mapping={"comment-only":["comment","py"],"body-change":["private-body","py"],"signature-change":["public-api","py"],lockfile:["whitespace","py"],config:["whitespace","py"],"unrelated-sibling":["whitespace","py"]} as const,[op]=mapping[row.edit.kind as keyof typeof mapping];
   bazelWorkspace(work,slice(row.slice),row.edit.kind,row.edit.files);
   const log=join(work,"log.zst"),first=bazelRun(work,output+index,log);expect(first.status,row.name+first.stderr).toBe(0);
   for(const file of row.edit.files){const path=join(work,file);writeFileSync(path,incrementalMutateV1(readFileSync(path,"utf8"),op,"py","r"+index+"x9k2m"));}
   bazelBuild(work,slice(row.slice),row.edit.kind,"run2");
   const second=bazelRun(work,output+index,log);expect(second.status,row.name+second.stderr).toBe(0);
   const ids=Object.keys(row.slice.tasks),observation=incrementalBazelObservationV1(incrementalBazelSpawnsV1(readFileSync(log)),ids),verdict=incrementalVerdictV1(row.expected as never,observation.executed);
   expect(verdict,row.name+"\n"+JSON.stringify(observation)).toMatchObject({status:"pass"});
   const processes=/INFO: (\d+) processes?: ([^\n]*)/u.exec(second.stderr),counted=processes?processes[2]!.replace(/\.$/u,"").split(/,\s*/u).map(part=>/^(\d+) (.+)$/u.exec(part.trim())!).filter(m=>!["internal","action cache hit","disk cache hit","remote cache hit"].includes(m[2]!)).reduce((sum,m)=>sum+Number(m[1]),0):0;
   expect(counted,row.name+" "+second.stderr).toBe(observation.executed.length);
   spawnSync(binary!,[`--output_base=${output+index}`,"--nosystem_rc","--nohome_rc","shutdown"],{cwd:work});
  }
 }finally{spawnSync("chmod",["-R","u+w",directory]);rmSync(directory,{recursive:true,force:true});}
},600000);

real("an action marked only no-cache is an action cache hit on the same output base, and only a changed action key executes it again",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"incr-nc-"))),output=join(directory,"ob"),log=join(directory,"log.zst");
 try{
  writeFileSync(join(directory,"MODULE.bazel"),'module(name = "incr_nocache", version = "0.0.0")\n');
  const write=(run:string)=>writeFileSync(join(directory,"BUILD.bazel"),`genrule(name = "volatile", outs = ["v.out"], cmd = "echo v > $@ # ${run}", tags = ["no-cache"])\ngenrule(name = "stable", outs = ["s.out"], cmd = "echo s > $@")\n`);
  const labels=()=>incrementalBazelSpawnsV1(readFileSync(log)).filter(spawn=>!spawn.cacheHit).map(spawn=>spawn.label.replace(/^@@?\/\//u,"//")).sort();
  write("n1");expect(bazelRun(directory,output,log).status).toBe(0);expect(labels()).toEqual(["//:stable","//:volatile"]);
  write("n1");expect(bazelRun(directory,output,log).status).toBe(0);expect(labels(),"no-cache alone re-executes nothing").toEqual([]);
  write("n2");expect(bazelRun(directory,output,log).status).toBe(0);expect(labels(),"a changed key executes only the volatile action").toEqual(["//:volatile"]);
 }finally{spawnSync(binary!,[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","shutdown"],{cwd:directory});spawnSync("chmod",["-R","u+w",directory]);rmSync(directory,{recursive:true,force:true});}
},300000);

real("Bazel's own rdeps over the generated package returns the dependents the model reaches when every output changes",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"incr-rd-"))),output=join(directory,"ob");
 try{
  writeFileSync(join(directory,"MODULE.bazel"),'module(name = "incr_rdeps", version = "0.0.0")\n');
  const all=Object.fromEntries(["bun-script","tsc","vitest","cargo","vite"].map(family=>[family,0]));
  for(const row of fixture.evaluation){
   const tool=row.tool as IncrementalToolV1,flat=structuredClone(row.slice) as unknown as {tasks:Record<string,{cache:boolean;outputs:boolean}>};
   for(const task of Object.values(flat.tasks)){task.cache=true;task.outputs=true;}
   const everything=Object.keys(all).map(family=>({family,kind:row.edit.kind,effect:"changed"})) as never,expected=incrementalExpectedV1(flat,tool,row.edit as never,everything);
   if(!expected.direct.length)continue;
   writeFileSync(join(directory,"BUILD.bazel"),incrementalRdepsPackageV1(flat,tool,row.edit.files));
   const run=bazelRun(directory,output,"",incrementalRdepsExpressionV1(row.edit.files));expect(run.status,row.name+run.stderr).toBe(0);
   expect(incrementalRdepsTasksV1(flat,run.stdout),row.name).toEqual([...expected.may]);
  }
 }finally{spawnSync(binary!,[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","shutdown"],{cwd:directory});spawnSync("chmod",["-R","u+w",directory]);rmSync(directory,{recursive:true,force:true});}
},300000);

const scenario=set.scenarios.find(row=>row.id==="fw-ts-comment")!;
const scenarioSlice=slice({...fixture.evaluation[0]!.slice,tasks:{...fixture.evaluation[0]!.slice.tasks,"lib:generate":{...fixture.evaluation[0]!.slice.tasks["lib:generate"]!,inputs:[scenario.mutation.file]}}});
const edited=incrementalExpectedV1(scenarioSlice,"bazel",{kind:scenario.kind,files:[scenario.mutation.file]},set.sensitivity);
const hosted=(files:Record<string,string>)=>{
 const data=new Map(Object.entries(files)),calls:string[]=[];let clock=0,cancelled=false;
 const ports=(over:Partial<IncrementalPortsV1>={},observed:(call:number)=>IncrementalObservationV1|Error=()=>({tool:"bazel",executed:[],restored:[],cached:[],unmapped:[]}),sliceValue:IncrementalSliceV1=scenarioSlice):IncrementalPortsV1=>{
  let executions=0;
  return {
   read:async path=>{calls.push("read:"+path);return data.get(path)!;},
   write:async(path,text)=>{calls.push("write:"+path);data.set(path,text);},
   slice:async()=>sliceValue,
   execute:async()=>{calls.push("execute");const result=observed(++executions);if(result instanceof Error)throw result;return result;},
   now:()=>(clock+=7),
   nonce:()=>"k3j9x2a",
   check:()=>{if(cancelled)throw Error("cancelled");},
   progress:async stage=>{calls.push("progress:"+stage);},
   ...over
  };
 };
 return {data,calls,ports,cancel:()=>{cancelled=true;}};
};
const exact=(tool:IncrementalToolV1):IncrementalObservationV1=>({tool,executed:[...edited.must],restored:[],cached:[],unmapped:[]});

test("a scenario primes, runs warm, applies the edit, runs, restores the file and runs again, judging each phase",async()=>{
 const text="const a = 1;\n",world=hosted({[scenario.mutation.file]:text});
 let seen="";
 const ports=world.ports({execute:async()=>{world.calls.push("execute");seen=world.data.get(scenario.mutation.file)!;return exactFor(world.calls.filter(call=>call==="execute").length);}});
 function exactFor(call:number):IncrementalObservationV1{return call===3?exact("bazel"):{tool:"bazel",executed:["shell:activate"],restored:[],cached:[],unmapped:[]};}
 const rows=await incrementalRunScenarioV1(set,scenario,"bazel",["warm","edit","revert"],ports);
 expect(rows.map(row=>[row.phase,row.verdict.status])).toEqual([["warm","pass"],["edit","pass"],["revert","pass"]]);
 expect(rows.every(row=>row.milliseconds>0)).toBe(true);
 expect(world.data.get(scenario.mutation.file)).toBe(text);
 expect(seen).toBe(text);
 const order=world.calls.filter(call=>call.startsWith("execute")||call.startsWith("write")||call.startsWith("read"));
 expect(order).toEqual(["execute","execute","read:"+scenario.mutation.file,"write:"+scenario.mutation.file,"execute","write:"+scenario.mutation.file,"execute"]);
 expect(world.calls.filter(call=>call.startsWith("progress:"))).toEqual(["slice","prime","warm","edit","revert"].map(stage=>`progress:fw-ts-comment:bazel:${stage}`));
 const report=incrementalReportV1(rows);
 expect(report).toMatchObject({passed:3,failed:0,skipped:0});
 expect(define("Report")(report)).toBe(true);
 expect(renderIncrementalReportV1(report,set,"en")).toContain("pass 3");
 expect(renderIncrementalReportV1(report,set,"de")).toContain("bestanden 3");
});

test("a run that executes too much is excess, one that misses a task is stale, and the failure names the tasks in both languages",async()=>{
 const world=hosted({[scenario.mutation.file]:"x\n"});
 const rows=await incrementalRunScenarioV1(set,scenario,"bazel",["warm","edit"],world.ports({},call=>call===2?{tool:"bazel",executed:["shell:activate","app:test"],restored:[],cached:[],unmapped:[]}:call===3?{tool:"bazel",executed:["app:build","zzz:extra"],restored:[],cached:[],unmapped:[]}:exact("bazel")));
 expect(rows.map(row=>row.verdict.status)).toEqual(["excess","stale-excess"]);
 expect(rows[1]!.verdict).toMatchObject({stale:["lib:generate","shell:activate"],excess:["zzz:extra"]});
 const report=incrementalReportV1(rows);
 expect(report.failed).toBe(2);
 expect(renderIncrementalReportV1(report,set,"en")).toContain("excess: app:test");
 expect(renderIncrementalReportV1(report,set,"de")).toContain("überzählig: app:test");
 expect(renderIncrementalReportV1(report,set,"de")).toContain("veraltet+überzählig");
});

test("the original text is restored when the edited run fails or the caller cancels, and a failure becomes a failed row",async()=>{
 const failing=hosted({[scenario.mutation.file]:"keep\n"});
 const rows=await incrementalRunScenarioV1(set,scenario,"nx",["warm","edit","revert"],failing.ports({},call=>call===3?Error("exit 1"):{tool:"nx",executed:["shell:activate"],restored:[],cached:[],unmapped:[]}));
 expect(rows.map(row=>[row.phase,row.verdict.status,row.note])).toEqual([["warm","pass",undefined],["edit","failed","exit 1"],["revert","pass",undefined]]);
 expect(failing.data.get(scenario.mutation.file)).toBe("keep\n");
 const cancelling=hosted({[scenario.mutation.file]:"keep\n"});
 const ports=cancelling.ports({execute:async()=>{cancelling.calls.push("execute");if(cancelling.calls.filter(call=>call==="execute").length===3){cancelling.cancel();throw Error("aborted");}return {tool:"bazel" as const,executed:["shell:activate"],restored:[],cached:[],unmapped:[]};}});
 await expect(incrementalRunScenarioV1(set,scenario,"bazel",["warm","edit","revert"],ports)).rejects.toThrow("cancelled");
 expect(cancelling.data.get(scenario.mutation.file)).toBe("keep\n");
});

test("scenarios outside a tool are skipped, a slice failure fails every phase, and a failed prime stops before any edit",async()=>{
 const only={...scenario,tools:["bazel" as const]},world=hosted({[scenario.mutation.file]:"x\n"});
 expect((await incrementalRunScenarioV1(set,only,"nx",["warm","edit"],world.ports())).map(row=>row.verdict.status)).toEqual(["skipped","skipped"]);
 expect((await incrementalRunScenarioV1(set,scenario,"bazel",["edit"],world.ports({slice:async()=>{throw Error("graph unavailable");}}))).map(row=>[row.verdict.status,row.note])).toEqual([["failed","slice: graph unavailable"]]);
 const priming=hosted({[scenario.mutation.file]:"x\n"}),rows=await incrementalRunScenarioV1(set,scenario,"bazel",["warm","edit","revert"],priming.ports({},()=>Error("boom")));
 expect(rows.map(row=>row.verdict.status)).toEqual(["failed","failed","failed"]);
 expect(priming.calls.filter(call=>call.startsWith("write"))).toEqual([]);
 const all=await incrementalRunV1(set,[scenario],["nx","bazel"],["warm"],world.ports({},()=>exact("nx")));
 expect(all.map(row=>[row.tool,row.phase])).toEqual([["nx","warm"],["bazel","warm"]]);
});

test("file access and the disposable root cannot reach outside the copy or into the live checkout",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"incr-root-")));
 try{
  const live=join(base,"live"),copy=join(base,"copy");mkdirSync(live);mkdirSync(copy);
  await expect(incrementalDisposableV1(live,copy)).resolves.toBeUndefined();
  await expect(incrementalDisposableV1(live,live)).rejects.toThrow("separate from the live checkout");
  await expect(incrementalDisposableV1(live,join(live,"inner"))).rejects.toThrow("separate from the live checkout");
  await expect(incrementalDisposableV1(join(base,"copy"),base)).rejects.toThrow("separate from the live checkout");
  await expect(incrementalDisposableV1(live,join(base,"does","not","exist"))).resolves.toBeUndefined();
  expect(incrementalInsideV1(copy,"a/b.ts")).toBe(join(copy,"a/b.ts"));
  for(const bad of ["../x","a/../../x","/etc/passwd",""])expect(()=>incrementalInsideV1(copy,bad),bad).toThrow("repository-relative path");
  expect(()=>incrementalInsideV1(copy,"a/".repeat(130)+"b")).toThrow("exceeds");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("cloning copies tracked files including emoji names and links into an empty root, reports progress, refuses a used or nested root and stops when cancelled",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"incr-clone-")));
 try{
  const live=join(base,"live");mkdirSync(join(live,"🧰️x/y"),{recursive:true});
  writeFileSync(join(live,"🧰️x/y/🟦️.ts"),"export {};\n");writeFileSync(join(live,"plain.txt"),"hello");symlinkSync("plain.txt",join(live,"link"));
  const files=["🧰️x/y/🟦️.ts","plain.txt","link"],ticks:string[]=[],copy=join(base,"copy");
  const done=await incrementalCloneV1(live,copy,files,{signal:new AbortController().signal,progress:async(count,total)=>{ticks.push(`${count}/${total}`);}});
  expect(done).toBe(3);expect(ticks.at(-1)).toBe("3/3");
  expect(readFileSync(join(copy,"🧰️x/y/🟦️.ts"),"utf8")).toBe("export {};\n");expect(readFileSync(join(copy,"plain.txt"),"utf8")).toBe("hello");
  expect(lstatSync(join(copy,"link")).isSymbolicLink()).toBe(true);expect(readlinkSync(join(copy,"link"))).toBe("plain.txt");
  await expect(incrementalCloneV1(live,copy,files,{signal:new AbortController().signal,progress:async()=>{}})).rejects.toThrow("must be empty");
  await expect(incrementalCloneV1(live,join(live,"nested"),files,{signal:new AbortController().signal,progress:async()=>{}})).rejects.toThrow("separate from the live checkout");
  const stop=new AbortController();stop.abort();
  await expect(incrementalCloneV1(live,join(base,"stopped"),files,{signal:stop.signal,progress:async()=>{}})).rejects.toThrow("Cancelled");
  await expect(incrementalCloneV1(live,join(base,"deep"),["a/".repeat(130)+"b"],{signal:new AbortController().signal,progress:async()=>{}})).rejects.toThrow("exceeds");
  expect(readdirSync(live).sort()).toEqual(["link","plain.txt","🧰️x"].sort());
 }finally{rmSync(base,{recursive:true,force:true});}
});

const stubAdapter:IncrementalAdapterV1={
 planBazelTasksV1:()=>({selected:["s:test"],tasks:{"s:gen":{id:"s:gen",dependencies:[],definition:{outputs:["o"]}},"s:test":{id:"s:test",dependencies:["s:gen"],definition:{inputs:[{dependentTasksOutputFiles:"**/*"}],outputs:[]}},"s:live":{id:"s:live",dependencies:["s:test"],definition:{outputs:["x"]}}}}),
 bazelCacheDecisionV1:task=>({cache:(task as {id:string}).id!=="s:live"}),
 bazelTaskCoverageV1:task=>({family:(task as {id:string}).id==="s:gen"?"bun-script":(task as {id:string}).id==="s:test"?"vitest":"bun-script"}),
 bazelSourceIndexV1:inventory=>({files:[...inventory].sort(),selectAll:inputs=>({marks:Uint8Array.from([...inventory].sort().map(file=>inputs.includes(file)?1:0))})}),
 bazelTaskInputsV1:(graph,task)=>(task as {id:string}).id==="s:gen"?["a.ts"]:(task as {id:string}).id==="s:test"?["a.ts","b.ts"]:[],
};

test("the slice builder keeps the declared inputs of the edited file, the outputs flag, the family, the consumers and the uncached tasks",()=>{
 const built=incrementalSliceFromGraphV1(stubAdapter,{},"s:test",["a.ts","zzz.ts"],["a.ts","b.ts","c.ts"]);
 expect(built).toEqual({version:1,selected:["s:test"],tasks:{
  "s:gen":{family:"bun-script",cache:true,outputs:true,dependsOn:[],consumes:[],inputs:["a.ts"]},
  "s:live":{family:"bun-script",cache:false,outputs:true,dependsOn:["s:test"],consumes:[],inputs:[]},
  "s:test":{family:"vitest",cache:true,outputs:false,dependsOn:["s:gen"],consumes:["s:gen"],inputs:["a.ts"]},
 }});
 expect(incrementalExpectedV1(built,"bazel",{kind:"comment-only",files:["a.ts"]},[{family:"bun-script",kind:"comment-only",effect:"identical"}])).toEqual({direct:["s:gen","s:test"],must:["s:gen","s:live","s:test"],may:["s:gen","s:live","s:test"],cutoff:[]});
 expect(()=>incrementalSliceFromGraphV1({...stubAdapter,bazelTaskCoverageV1:()=>({family:"make"})},{},"s:test",[],[])).toThrow("Unknown task family make");
});

test("a dry-run plan is read from the first line that opens the JSON object, whatever the CLI printed before it",()=>{
 const plan={tasks:{roots:["b:x"],tasks:{"b:x":{id:"b:x"},"a:x":{id:"a:x"}},dependencies:{"b:x":["a:x"],"a:x":[]}}};
 expect(incrementalPlanTasksV1("[bazel] preparing\n"+JSON.stringify(plan,null,2))).toEqual({ids:["a:x","b:x"],dependencies:{"b:x":["a:x"],"a:x":[]}});
 expect(()=>incrementalPlanTasksV1("no json")).toThrow("A dry-run plan is required");
 expect(()=>incrementalPlanTasksV1('{\n"x":1\n}')).toThrow("A dry-run plan is required");
 expect(incrementalSelectV1(set,"all").length).toBe(set.scenarios.length);
 expect(incrementalSelectV1(set,["fw-ts-body"]).map(row=>row.id)).toEqual(["fw-ts-body"]);
 expect(()=>incrementalSelectV1(set,["nope"])).toThrow("Unknown scenario: nope");
});

test("the command arguments are closed, bilingual in their errors and default to both implementations and all phases",()=>{
 expect(incrementalArgumentsV1(["list"])).toMatchObject({operation:"list",scenarios:"all",tools:["nx","bazel"],phases:["warm","edit","revert"],json:false,fresh:false});
 expect(incrementalArgumentsV1(["run","--scenario=a,b","--implementations","bazel","--phases=edit","--json","--root=/tmp/x","--file=r.json","--lang=de"])).toMatchObject({operation:"run",scenarios:["a","b"],tools:["bazel"],phases:["edit"],json:true,root:"/tmp/x",file:"r.json",language:"de"});
 expect(incrementalArgumentsV1(["plan","--scenario=all"]).scenarios).toBe("all");
 for(const[bad,message]of [[[],"Expected repo incremental"],[["go"],"Expected repo incremental"],[["run","extra"],"Expected repo incremental"],[["run","--wat"],"Unknown incremental option"],[["run","--json","--json"],"Repeated option"],[["run","--scenario"],"needs a value"],[["run","--implementations=make"],"Unknown implementation"],[["run","--phases=cold"],"Unknown phase"],[["run","--lang=fr"],"Unknown language"],[["run","--json=1"],"Unknown incremental option"]] as const)
  expect(()=>incrementalArgumentsV1(bad as never),JSON.stringify(bad)).toThrow(message);
 expect(()=>incrementalArgumentsV1(["go"])).toThrow("Erwartet: repo incremental");
 expect(incrementalDefaultRootV1("darwin",{})).toBe("/tmp/sinc");expect(incrementalDefaultRootV1("win32",{SystemDrive:"D:"})).toBe("D:\\sinc");expect(incrementalDefaultRootV1("win32",{})).toBe("C:\\sinc");
 expect(renderIncrementalPlanV1(set,[{scenario:"s",tool:"nx",expected:{direct:[],must:["a:b"],may:["a:b"],cutoff:[]}}],"de")).toBe("s / nx: muss 1, darf 1, abgeschnitten 0\n  a:b");
});

const stubCli=(world:{base:string},sliceValue:IncrementalSliceV1,log:string[]):IncrementalCommandPortsV1["run"]=>async(command,args,cwd,env)=>{
 log.push([command,...args].join(" "));
 const rest=args.slice(args.indexOf("repo")+1).filter(argument=>!argument.startsWith("--tool=")),tool=args.find(argument=>argument.startsWith("--tool="))!.slice(7);
 if(rest[0]==="graph"){const file=rest[1]!.slice(7);writeFileSync(file,"{}");return file+"\n";}
 if(rest[1]&&rest[2]==="--dry-run")return JSON.stringify({tasks:{roots:[],tasks:Object.fromEntries(Object.keys(sliceValue.tasks).map(id=>[id,{id}])),dependencies:Object.fromEntries(Object.keys(sliceValue.tasks).map(id=>[id,[]]))}},null,2);
 if(rest[0]==="run"){
  if(tool==="bazel")writeFileSync(env.SEMIO_BUILD_TASK_REPORT!,Buffer.from(fixture.bazelLogs[0]!.compact,"base64"));
  else writeFileSync(join(cwd,"run.json"),JSON.stringify({tasks:Object.keys(sliceValue.tasks).map(taskId=>({taskId,cacheStatus:"cache-miss",status:0}))}));
  return "";
 }
 return "";
};

test("the command lists, prepares, plans, runs and discards through its ports with progress, bilingual output, a report file and a failing exit",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"incr-cmd-")));
 try{
  const live=join(base,"live"),copy=join(base,"copy"),script="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts";mkdirSync(join(live,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),{recursive:true});writeFileSync(join(live,script),"// cli\n");writeFileSync(join(live,"a.ts"),"a\n");
  const out:string[]=[],stages:string[]=[],log:string[]=[],reports:unknown[]=[],single=slice({version:1,selected:["lib:generate"],tasks:{"lib:generate":{family:"bun-script",cache:true,outputs:true,dependsOn:[],consumes:[],inputs:[]}}});
  const ports=(over:Partial<IncrementalCommandPortsV1>={}):IncrementalCommandPortsV1=>({write:line=>out.push(line),progress:async stage=>{stages.push(stage);},check:()=>{},signal:new AbortController().signal,environment:{SEMIO_LANGUAGE:"en",SEMIO_X:"1",PATH:"/usr/bin"},platform:"linux",now:()=>Date.now(),run:stubCli({base},single,log),report:async(file,value)=>{reports.push([file,value]);},files:async()=>[script,"a.ts"],adapter:async()=>({...stubAdapter,planBazelTasksV1:()=>({selected:["lib:generate"],tasks:{"lib:generate":{id:"lib:generate",dependencies:[],definition:{outputs:["o"]}}}}),bazelTaskCoverageV1:()=>({family:"bun-script"}),bazelTaskInputsV1:()=>[]}),nxRecord:directory=>join(directory,"run.json"),exists:async path=>existsSync(path),scratch:async()=>mkdtempSync(join(base,"scratch-")),...over});
  await runIncrementalCommandV1(live,["incremental","list","--lang=de"],ports());
  expect(out.at(-1)).toContain(set.scenarios[0]!.title.de);
  await expect(runIncrementalCommandV1(live,["incremental","run","--root="+copy],ports())).rejects.toThrow("Prepare the disposable root first");
  await runIncrementalCommandV1(live,["incremental","prepare","--root="+copy],ports());
  expect(existsSync(join(copy,script))).toBe(true);expect(stages.some(stage=>stage.startsWith("clone:"))).toBe(true);
  expect(log.map(line=>line.split(" repo ")[1])).toEqual(["--tool=nx preflight","--tool=nx tools setup all","--tool=nx dependencies sync --all"]);
  await expect(runIncrementalCommandV1(live,["incremental","plan","--scenario=lib-none","--root="+copy],ports())).rejects.toThrow("Unknown scenario");
  out.length=0;
  await runIncrementalCommandV1(live,["incremental","plan","--scenario=fw-ts-comment","--implementations=bazel","--root="+copy,"--json"],ports());
  expect(JSON.parse(out[0]!)[0]).toMatchObject({scenario:"fw-ts-comment",tool:"bazel",expected:{direct:[],must:[]}});
  await expect(runIncrementalCommandV1(live,["incremental","run","--scenario=fw-ts-comment","--implementations=bazel","--phases=warm","--root="+copy,"--file=report.json"],ports())).rejects.toThrow("Incremental conformance failed in");
  expect(reports.length).toBe(1);expect(out.join("\n")).toContain("fw-ts-comment");
  await runIncrementalCommandV1(live,["incremental","discard","--root="+copy],ports());
  expect(existsSync(copy)).toBe(false);expect(existsSync(join(live,"a.ts"))).toBe(true);
  await expect(runIncrementalCommandV1(live,["incremental","discard","--root="+live],ports())).rejects.toThrow("separate from the live checkout");
  const cancelled=ports({check:()=>{throw Error("cancelled");}});
  await expect(runIncrementalCommandV1(live,["incremental","prepare","--root="+join(base,"copy2")],cancelled)).rejects.toThrow("cancelled");
 }finally{rmSync(base,{recursive:true,force:true});}
},60000);

test("the real ports read and write only inside the disposable root and pass the task report path only to Bazel",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"incr-ports-")));
 try{
  const copy=join(base,"copy"),scratch=join(base,"scratch"),calls:string[][]=[],envs:Record<string,string>[]=[];mkdirSync(copy);mkdirSync(scratch);writeFileSync(join(copy,"f.txt"),"one");
  const host:IncrementalHostV1={live:join(base,"live"),root:copy,script:"s.ts",executable:"bun",scratch,signal:new AbortController().signal,environment:{PATH:"/bin",SEMIO_X:"1"},
   run:async(command,args,cwd,env)=>{calls.push([command,...args]);envs.push({...env});const report=env.SEMIO_BUILD_TASK_REPORT;if(report)writeFileSync(report,Buffer.from(fixture.bazelLogs[0]!.compact,"base64"));else writeFileSync(join(copy,"run.json"),JSON.stringify(fixture.nxRecords[0]!.record));return "";},
   nxRecord:directory=>join(directory,"run.json"),adapter:async()=>stubAdapter,inventory:async()=>[],now:()=>1,check:()=>{},progress:async()=>{}};
  const ports=incrementalCliPortsV1(host),ids=fixture.bazelLogs[0]!.tasks,full=Object.fromEntries(ids.map(id=>[id,{family:"bun-script",cache:true,outputs:true,dependsOn:[],consumes:[],inputs:[]}]));
  expect(await ports.read("f.txt")).toBe("one");await ports.write("f.txt","two");expect(readFileSync(join(copy,"f.txt"),"utf8")).toBe("two");
  await expect(ports.write("../escape.txt","x")).rejects.toThrow("repository-relative path");
  expect(existsSync(join(base,"escape.txt"))).toBe(false);
  const bazel=await ports.execute("bazel","lib:generate",slice({version:1,selected:[],tasks:full}));
  expect(bazel.executed).toEqual(["app:build","app:test","lib:generate"]);
  expect(envs[0]!.SEMIO_BUILD_TASK_REPORT).toBe(join(scratch,"bazel-1.zst"));expect(envs[0]!.SEMIO_X).toBeUndefined();expect(calls[0]).toEqual(["bun",join(copy,"s.ts"),"repo","--tool=bazel","run","lib:generate"]);
  expect(existsSync(join(scratch,"bazel-1.zst"))).toBe(false);
  const nx=await ports.execute("nx","a:build",slice({version:1,selected:[],tasks:Object.fromEntries(["a:build","b:build","c:build"].map(id=>[id,full["lib:generate"]!]))}));
  expect(nx).toEqual({tool:"nx",executed:["b:build"],restored:["a:build","c:build"],cached:[],unmapped:["z:other"]});
  expect(envs[1]!.SEMIO_BUILD_TASK_REPORT).toBeUndefined();
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the Bazel adapter asks for the compact execution log only when the environment names one absolute path, under the name the schema declares",()=>{
 expect(schema.definitions.Policy.const.bazelReport).toBe("SEMIO_BUILD_TASK_REPORT");
 expect(bazelTaskReportOptionsV1({})).toEqual([]);
 expect(bazelTaskReportOptionsV1({SEMIO_BUILD_TASK_REPORT:""})).toEqual([]);
 expect(bazelTaskReportOptionsV1({SEMIO_BUILD_TASK_REPORT:join(tmpdir(),"run.zst")})).toEqual([`--execution_log_compact_file=${join(tmpdir(),"run.zst")}`]);
 for(const bad of ["run.zst","./run.zst",join(tmpdir(),"a\nb")])expect(()=>bazelTaskReportOptionsV1({SEMIO_BUILD_TASK_REPORT:bad}),bad).toThrow("Absolute Bazel task report path required");
 const plan=bazelCommandPlanV1({mode:"execute",base:["--lockfile_mode=error",...bazelTaskReportOptionsV1({SEMIO_BUILD_TASK_REPORT:join(tmpdir(),"run.zst")})],jobs:2,bail:false,verbose:false,skipCache:false,tasks:[{label:"//:t_0123456789abcdef",kind:"task"}]});
 expect(plan.steps[0]!.options).toContain(`--execution_log_compact_file=${join(tmpdir(),"run.zst")}`);
});

test("the neutral catalogue documents repo incremental in both languages with exactly the options and subcommands the parser accepts",()=>{
 const row=catalog.operations.find(operation=>operation.name==="incremental")!;
 expect(row).toBeDefined();
 expect(row.implementations).toEqual({nx:"neutral",bazel:"neutral"});
 expect(row.progress&&row.cancellable).toBe(true);
 expect("subcommands" in row?row.subcommands!.map(sub=>sub.name):[]).toEqual(["list","plan","prepare","run","discard"]);
 const names=incrementalOptionNamesV1(),accepted=new Set([...names.valued,...names.plain].filter(name=>name!=="--lang"));
 expect(new Set(row.flags)).toEqual(accepted);
 for(const name of accepted){const flag=catalog.flags.find(entry=>entry.name===name)!;expect(flag,name).toBeDefined();expect(flag.operations,name).toContain("incremental");expect(flag.value,name).toBe(names.plain.includes(name)?"none":"required");}
 for(const text of [row.summary,...row.examples.map(example=>example.description),...row.subcommands!.map(sub=>sub.summary)]){expect(text.en).not.toBe("");expect(text.de).not.toBe(text.en);}
 for(const example of row.examples)expect(example.command.startsWith("repo incremental")).toBe(true);
});

test("the comparison pairs Nx and Bazel rows of the same scenario and phase, lists the tasks only one executed and renders in both languages",()=>{
 const row=(tool:IncrementalToolV1,phase:"warm"|"edit",executed:string[],status:"pass"|"failed"|"skipped"="pass")=>({scenario:"s",tool,phase,expected:{direct:[],must:[],may:executed,cutoff:[]},observation:{tool,executed,restored:[],cached:[],unmapped:[]},verdict:{status,stale:[],excess:[],exact:true},milliseconds:1}) as never;
 const rows=[row("nx","warm",["a:b"]),row("bazel","warm",["a:b"]),row("nx","edit",["a:b","c:d"]),row("bazel","edit",["a:b","e:f"]),row("nx","edit",["z:z"],"failed")];
 expect(incrementalComparisonsV1(rows)).toEqual([{scenario:"s",phase:"warm",same:true,onlyNx:[],onlyBazel:[]},{scenario:"s",phase:"edit",same:false,onlyNx:["c:d"],onlyBazel:["e:f"]}]);
 expect(incrementalComparisonsV1([row("nx","warm",["a:b"],"skipped"),row("bazel","warm",["a:b"])])).toEqual([]);
 expect(incrementalComparisonsV1([row("nx","warm",["a:b"])])).toEqual([]);
 const report=incrementalReportV1(rows);
 expect(renderIncrementalReportV1(report,set,"en")).toContain("comparison of the implementations: nx / bazel\n  s / warm run: same tasks\n  s / edit: different tasks\n    only nx: c:d\n    only bazel: e:f");
 expect(renderIncrementalReportV1(report,set,"de")).toContain("Vergleich der Implementierungen: nx / bazel");
 expect(python({...fixture,comparisonRows:rows}).comparisons).toEqual(incrementalComparisonsV1(rows));
});
