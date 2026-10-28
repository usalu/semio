/** 🗂️ Tests for the suite sweep of the continuous integration: policy, shards, targets, skip and count reading, memory, verdict and the run itself. */
import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {delimiter,join} from "node:path";
import {captureOwnedCommandV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {placeholdersV1,textV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {PROCESS_TEXTS_V1} from "../../../../../../../../🔨️modules/🏃️process/📖️texts/🟦️.ts";
import {processTestSuiteRunsOnV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🗂️suites/🟦️.ts";
import {CI_SUITE_POLICY_V1,CI_SUITE_ROOT_V1,CI_SUITE_TEXTS_V1,addCiCountsV1,ciShardSuitesV1,ciSkipMatcherV1,ciSuiteTargetsV1,ciSuiteTextV1,failingCiSuitesV1,judgeCiSuiteRunV1,planCiMustRunV1,planCiShardV1,readCiSuitePolicyV1,readCiSuiteTablesV1,renderCiSuiteReportV1,runCiSuitesV1,summarizeCiMemoryV1,type CiOutcomeV1,type CiPlanV1,type CiSampleV1,type CiSuiteHostV1,type CiSuiteRunPortsV1} from "../🟦️.ts";
import {parseCiShardV1,runCiSuitesCommandV1} from "../📜️script.ts";
import policyDocument from "../🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),ajv=new Ajv({strict:true,allowUnionTypes:true}),scratch:string[]=[];
const check=(definition:string,value:unknown)=>{
 const validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`});
 if(!validate(value))throw Error(`${definition}: ${ajv.errorsText(validate.errors)}`);
};
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`ci-suites-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const python=():string=>{const found=testPythonV1()??Bun.which("python3")??Bun.which("python");if(!found)throw Error("Independent Python oracle required");return found;};
const ask=async(request:unknown):Promise<any>=>{
 const file=join(directory("oracle"),"request.json");writeFileSync(file,JSON.stringify(request));
 return JSON.parse(await captureOwnedCommandV1(python(),[oracle,file],CI_SUITE_ROOT_V1,{output:{maximumBytes:4194304,maximumLines:4096}}));
};
const tables=readCiSuiteTablesV1();
const hosts:readonly CiSuiteHostV1[]=["darwin","linux","win32"];

test("the policy, a plan and a report are closed documents of the schema and the schema refuses what it must",()=>{
 check("Policy",policyDocument);
 expect(readCiSuitePolicyV1()).toEqual(policyDocument as never);
 const plan=planCiShardV1(tables,"linux","process",1,2);check("Plan",plan);
 for(const row of fixture.verdicts)for(const outcome of row.outcomes)check("Outcome",outcome);
 for(const row of fixture.samples)for(const sample of row)check("Sample",sample);
 expect(()=>check("Policy",{...policyDocument,extra:1})).toThrow();
 expect(()=>check("Policy",{...policyDocument,tables:policyDocument.tables.map(row=>({...row,shards:0}))})).toThrow();
 expect(()=>check("Policy",{...policyDocument,maximumShardBudgetMinutes:600})).toThrow();
 expect(()=>check("Outcome",{...fixture.verdicts[0]!.outcomes[0],status:"green"})).toThrow();
 expect(()=>check("Outcome",{...fixture.verdicts[0]!.outcomes[0],log:"../x.log"})).toThrow();
 expect(()=>readCiSuitePolicyV1({...policyDocument,mustRun:{...policyDocument.mustRun,win32:[...policyDocument.mustRun.win32,policyDocument.mustRun.all[0]]}})).toThrow();
 expect(()=>readCiSuitePolicyV1({...policyDocument,tables:[policyDocument.tables[0],policyDocument.tables[0],policyDocument.tables[1]]})).toThrow();
},120000);

test("every suite of the three route tables resolves to the official target that runs it, equal to the independent Python reading",async()=>{
 for(const row of CI_SUITE_POLICY_V1.tables){
  const table=tables.find(item=>item.id===row.id)!,ids=table.table.suites.map(suite=>suite.id),project=join(CI_SUITE_ROOT_V1,...row.projectFile.split("/"));
  const independent=await ask({op:"targets",project,ids}),own=Object.fromEntries(ids.map(id=>[id,table.targets.get(id)??null]));
  expect(own,row.id).toEqual(independent);
  expect(ids.filter(id=>own[id]===null),`${row.id}: suites without a test target`).toEqual([]);
 }
 expect(ciSuiteTargetsV1({"test-a":{options:{command:"repo ./x.ts test a"}},"alias-a":{options:{command:"repo ./x.ts test a"}},"test-b-wrapped":{options:{command:"repo ./x.ts wrap -- repo ./y.ts test b"}},build:{}})).toEqual(new Map([["a","test-a"],["b","test-b-wrapped"]]));
},120000);

test("the shards partition every table: disjoint, complete, bounded by the budget and equal to the independent Python cut",async()=>{
 for(const row of CI_SUITE_POLICY_V1.tables){
  const table=tables.find(item=>item.id===row.id)!,all=table.table.suites.map(suite=>suite.id).sort(),own=Array.from({length:row.shards},(_,index)=>[...ciShardSuitesV1(table.table,index+1,row.shards)]);
  expect(own.flat().sort(),row.id).toEqual(all);
  expect(new Set(own.flat()).size).toBe(all.length);
  const independent=await ask({op:"shards",table:join(CI_SUITE_ROOT_V1,...(row.id==="process"?["🧰️framework","🔨️modules","🏃️process","🧪️testing","🗂️suites","🔣️.json"]:row.id==="caching"?["🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","⚡️caching","🗂️suites","🔣️.json"]:["🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","🗂️suites","🔣️.json"])),count:row.shards});
  expect(own,row.id).toEqual(independent.shards);
  for(const load of independent.loads as number[])expect(load/60000,`${row.id} shard budget`).toBeLessThanOrEqual(CI_SUITE_POLICY_V1.maximumShardBudgetMinutes);
 }
 expect(()=>ciShardSuitesV1(tables[0]!.table,0,2)).toThrow();expect(()=>ciShardSuitesV1(tables[0]!.table,3,2)).toThrow();
 expect(ciShardSuitesV1({version:1,codeSuites:[],helpers:[],suites:[{id:"a",budgetMs:3000,typecheck:"none",files:["x"]},{id:"b",budgetMs:2000,typecheck:"none",files:["y"]},{id:"c",budgetMs:2000,typecheck:"none",files:["z"]}]} as never,1,2)).toEqual(["a"]);
},120000);

test("a plan names the table, project and target of every suite and flags the platform the suite is declared for",()=>{
 const plan=planCiShardV1(tables,"win32","caching",1,3);
 expect(plan.rows.every(row=>row.project==="repo"&&/^test-/u.test(row.target))).toBe(true);
 const declared=structuredClone(tables.find(table=>table.id==="process")!),first=declared.table.suites[0]!;
 const tagged={...declared,table:{...declared.table,suites:[{...first,platforms:["darwin"]},...declared.table.suites.slice(1)]}};
 const rows=planCiShardV1([tagged as never,...tables.filter(table=>table.id!=="process")],"linux","process",1,2).rows.concat(planCiShardV1([tagged as never,...tables.filter(table=>table.id!=="process")],"linux","process",2,2).rows);
 expect(rows.find(row=>row.suite===first.id)!.runsHere).toBe(false);
 expect(rows.filter(row=>row.suite!==first.id).every(row=>row.runsHere)).toBe(true);
 const missing={...tables[0]!,targets:new Map<string,string>()};
 expect(planCiShardV1([missing,...tables.slice(1)],"linux","process",1,2).unrouted.length).toBeGreaterThan(0);
 expect(()=>planCiShardV1(tables,"linux","process",1,3)).toThrow();
 expect(parseCiShardV1("caching:2/3")).toEqual({table:"caching",index:2,count:3});
 for(const bad of ["caching:2/2","caching:4/3","nothing:1/1","process:0/2","process:1-2","process"])expect(()=>parseCiShardV1(bad),bad).toThrow();
});

test("the suites a host must run exist in the tables, have a target and are declared for that host",()=>{
 for(const host of hosts){
  const plan=planCiMustRunV1(tables,CI_SUITE_POLICY_V1,host),wanted=[...CI_SUITE_POLICY_V1.mustRun.all,...CI_SUITE_POLICY_V1.mustRun[host]];
  expect(plan.unrouted,host).toEqual([]);
  expect(plan.rows.length,host).toBe(wanted.length);
  expect(plan.rows.every(row=>row.runsHere),`${host}: a suite that must run is declared for other hosts`).toBe(true);
  for(const row of wanted){
   const suite=tables.find(table=>table.id===row.table)!.table.suites.find(item=>item.id===row.suite);
   expect(suite,`${row.table}/${row.suite}`).toBeDefined();
   expect(processTestSuiteRunsOnV1(suite!,host),`${row.table}/${row.suite} on ${host}`).toBe(true);
  }
 }
 expect(CI_SUITE_POLICY_V1.mustRun.win32.length).toBeGreaterThanOrEqual(10);
});

test("a count line of the test runner is read through colors and everything else is ignored, equal to the independent reading",async()=>{
 for(const lines of fixture.countLines){
  let own={pass:0,skip:0,fail:0};for(const line of lines)own=addCiCountsV1(own,line);
  const independent=await ask({op:"counts",lines,suite:"x",english:"-",german:"-"});
  expect(own).toEqual(independent.counts);
 }
 expect(fixture.countLines[0]).toBeDefined();
 let first={pass:0,skip:0,fail:0};for(const line of fixture.countLines[0]!)first=addCiCountsV1(first,line);
 expect(first).toEqual({pass:19,skip:5,fail:1});
},120000);

test("the line the route prints for a suite declared for other hosts is recognized in English and German for that suite only, built from the route's own catalog",async()=>{
 const messages=PROCESS_TEXTS_V1.catalog.messages["script.suite-platform-skipped"]!;
 for(const language of ["en","de"] as const){
  const line=textV1(PROCESS_TEXTS_V1.catalog,"script.suite-platform-skipped",language,{suite:"leases",platforms:"darwin, linux",host:"win32",count:7});
  expect(ciSkipMatcherV1("leases").test(line),language).toBe(true);
  expect(ciSkipMatcherV1("leases").test(`\u001b[33m${line}\u001b[0m`.replace(/\u001b\[[0-9;]*m/gu,"")),language).toBe(true);
  expect(ciSkipMatcherV1("lease").test(line),language).toBe(false);
  expect(ciSkipMatcherV1("leases").test("not "+line),language).toBe(false);
  const independent=await ask({op:"counts",lines:[line,"x"],suite:"leases",english:messages.en,german:messages.de});
  expect(independent.skipped,language).toBe(true);
  expect((await ask({op:"counts",lines:[line],suite:"lease",english:messages.en,german:messages.de})).skipped).toBe(false);
 }
 expect(ciSkipMatcherV1("a.b").test(textV1(PROCESS_TEXTS_V1.catalog,"script.suite-platform-skipped","en",{suite:"axb",platforms:"p",host:"h",count:1}))).toBe(false);
},120000);

test("the host memory samples of a suite reduce to the lowest free memory, the highest use, swap and pressure, equal to the independent reading",async()=>{
 for(const samples of fixture.samples){
  const own=summarizeCiMemoryV1(samples as CiSampleV1[]);
  check("Memory",own);
  expect(own).toEqual(await ask({op:"memory",samples}));
 }
 expect(summarizeCiMemoryV1(fixture.samples[2] as CiSampleV1[])).toEqual({samples:3,minimumAvailableMegabytes:3000,peakUsedMegabytes:13000,peakSwapMegabytes:512,peakTreeMegabytes:1500,worstPressure:"warn"});
},120000);

test("a report counts every status, never counts a skipped suite as passed and fails exactly the suites the independent verdict names",async()=>{
 for(const row of fixture.verdicts){
  const planRows=row.rows.map(([table,suite])=>({table,suite,project:"p",target:`test-${suite}`,budgetMs:1000,runsHere:true})),plan={version:1 as const,host:"linux" as const,label:"x",rows:planRows,unrouted:row.unrouted} as CiPlanV1;
  const report=judgeCiSuiteRunV1({plan,tool:"bazel",outcomes:row.outcomes as CiOutcomeV1[],enforce:row.enforce});
  check("Report",report);
  const independent=await ask({op:"verdict",rows:planRows,unrouted:row.unrouted,outcomes:row.outcomes,enforce:row.enforce});
  expect(failingCiSuitesV1(report),row.name).toEqual(independent);
  const unobserved=await ask({op:"unobserved",outcomes:row.outcomes});
  expect(report.unobserved,row.name).toEqual(unobserved);
  expect(report.ok,row.name).toBe(independent.length===0&&(!row.enforce||unobserved.length===0));
  expect(report.totals.passed,row.name).toBe(row.outcomes.filter(outcome=>outcome.status==="passed").length);
  expect(report.totals.skipped,row.name).toBe(row.outcomes.filter(outcome=>outcome.status==="skipped").length);
 }
 const skipped=judgeCiSuiteRunV1({plan:{version:1,host:"linux",label:"x",rows:[{table:"process",suite:"leases",project:"p",target:"test-leases",budgetMs:1000,runsHere:false}],unrouted:[]},tool:"bazel",outcomes:fixture.verdicts[1]!.outcomes as CiOutcomeV1[],enforce:true});
 expect(skipped.ok).toBe(false);expect(skipped.violations).toEqual([{table:"process",suite:"leases",reason:"skipped"}]);expect(skipped.totals.passed).toBe(0);
 const undeclared=judgeCiSuiteRunV1({plan:{version:1,host:"linux",label:"x",rows:[],unrouted:[]},tool:"bazel",outcomes:[],enforce:true,undeclared:[{table:"build",suite:"gone",reason:"not-declared"}]});
 expect(undeclared.ok).toBe(false);
},120000);

test("the Markdown report names the skipped suites as skipped, not passed, in English and German and shows what each must-run suite proves",()=>{
 const outcomes=[...fixture.verdicts[3]!.outcomes,{...fixture.verdicts[1]!.outcomes[0]!,suite:"native-storage"}] as CiOutcomeV1[],plan={version:1 as const,host:"win32" as const,label:"must-run",rows:[],unrouted:[]};
 const report=judgeCiSuiteRunV1({plan,tool:"bazel",outcomes:outcomes.map(row=>row.suite==="c"?{...row,table:"process" as const,suite:"leases"}:row),enforce:false});
 const english=renderCiSuiteReportV1(report,"en",CI_SUITE_POLICY_V1.mustRun.all),german=renderCiSuiteReportV1(report,"de",CI_SUITE_POLICY_V1.mustRun.all);
 expect(english).toContain("SKIPPED, not passed");expect(german).toContain("ÜBERSPRUNGEN, nicht bestanden");
 expect(english).toContain("process/native-storage");expect(english).toContain("Proves:");expect(german).toContain("Beweist:");
 expect(english).toContain("| process/leases | passed |");
 expect(english).not.toContain("| process/native-storage | passed |");
 expect(english).toContain("Verdict: 3 suite(s) failed");
 for(const [key,leaf] of Object.entries(CI_SUITE_TEXTS_V1.catalog.messages)){
  expect([...placeholdersV1(leaf.en)].sort(),key).toEqual([...placeholdersV1(leaf.de)].sort());
  const values=Object.fromEntries(placeholdersV1(leaf.en).map(name=>[name,"x"]));
  for(const language of ["en","de"] as const)expect(ciSuiteTextV1(language,key as never,values).length,`${key} ${language}`).toBeGreaterThan(0);
 }
});

test("every text of the sweep is used by the code",()=>{
 const source=["🟦️.ts","📜️script.ts"].map(name=>readFileSync(join(import.meta.dir,"..",name),"utf8")).join("\n");
 for(const key of Object.keys(CI_SUITE_TEXTS_V1.catalog.messages))expect(source.includes(`"${key}"`),key).toBe(true);
});

const clock=()=>{let now=1000;return {now:()=>now,advance:(ms:number)=>{now+=ms;}};};
const observing=async():Promise<CiSampleV1>=>({atMs:1,availableMegabytes:5000,totalMegabytes:8000,swapMegabytes:null,treeMegabytes:90,pressure:"normal"});
const row=(suite:string,table:"process"|"caching"|"build"="process")=>({table,suite,project:"p",target:`test-${suite}`,budgetMs:60000,runsHere:true});

function ports(script:Record<string,Readonly<{lines:readonly string[];exitCode?:number|null;timedOut?:boolean;advance?:number}>>,seen:string[],time=clock(),sample:()=>Promise<CiSampleV1|null>=async()=>null):CiSuiteRunPortsV1{
 return {
  start:async(planned,tool,timeoutMs,onLine)=>{
   seen.push(`${tool}:${planned.project}:${planned.target}:${timeoutMs}`);
   const behavior=script[planned.suite]!;for(const line of behavior.lines)onLine(line);time.advance(behavior.advance??1500);
   return {exitCode:behavior.exitCode===undefined?0:behavior.exitCode,timedOut:behavior.timedOut??false};
  },
  sample,now:time.now,write:()=>{},group:()=>{}
 };
}

test("a run starts every suite through the official route in order, writes one log per suite with the plan, samples, report and Markdown, and reads the outcome of each suite",async()=>{
 const out=directory("run"),seen:string[]=[],messages=PROCESS_TEXTS_V1.catalog.messages["script.suite-platform-skipped"]!;
 const plan:CiPlanV1={version:1,host:"linux",label:"process-1of2",rows:[row("alpha"),row("beta"),row("gamma"),row("delta"),row("lost","caching")],unrouted:[{table:"build",suite:"nowhere"}]};
 let step=0;const time=clock();
 const sample=async():Promise<CiSampleV1>=>({atMs:time.now(),availableMegabytes:[8000,6000,4000,9000][step++%4]!,totalMegabytes:10000,swapMegabytes:null,treeMegabytes:null,pressure:"normal"});
 const report=await runCiSuitesV1({plan,policy:CI_SUITE_POLICY_V1,tool:"bazel",out,language:"en",enforce:false,ports:ports({
  alpha:{lines:["a.ts:"," 4 pass"," 1 skip"," 0 fail"]},
  beta:{lines:[textV1(PROCESS_TEXTS_V1.catalog,"script.suite-platform-skipped","de",{suite:"beta",platforms:"darwin",host:"linux",count:2})]},
  gamma:{lines:[" 1 pass"," 2 fail"],exitCode:1},
  delta:{lines:[],exitCode:null,timedOut:true,advance:60000},
  lost:{lines:[" 9 pass"]}
 },seen,time,sample)});
 expect(seen).toEqual(["bazel:p:test-alpha:660000","bazel:p:test-beta:660000","bazel:p:test-gamma:660000","bazel:p:test-delta:660000","bazel:p:test-lost:660000"]);
 expect(report.outcomes.map(item=>`${item.suite}:${item.status}`)).toEqual(["alpha:passed","beta:skipped","gamma:failed","delta:timeout","lost:passed","nowhere:unrouted"]);
 expect(report.outcomes[0]!.tests).toEqual({pass:4,skip:1,fail:0});
 expect(report.outcomes[2]!.exitCode).toBe(1);expect(report.outcomes[3]!.exitCode).toBeNull();
 expect(report.outcomes[0]!.wallMs).toBe(1500);expect(report.outcomes[3]!.wallMs).toBe(60000);
 expect(report.outcomes[0]!.memory.samples).toBe(2);
 expect(report.outcomes[0]!.memory.peakUsedMegabytes).toBe(4000);
 expect(report.ok).toBe(false);expect(failingCiSuitesV1(report)).toEqual(["build/nowhere","process/delta","process/gamma"]);
 check("Report",report);
 const base=join(out,"process-1of2");
 expect(readdirSync(join(base,"logs")).sort()).toEqual(["alpha.log","beta.log","delta.log","gamma.log","lost.log"]);
 expect(readFileSync(join(base,"logs","alpha.log"),"utf8")).toBe("a.ts:\n 4 pass\n 1 skip\n 0 fail\n");
 expect(readFileSync(join(base,"logs","beta.log"),"utf8")).toContain(messages.de.split("{")[0]!);
 expect(JSON.parse(readFileSync(join(base,"report.json"),"utf8"))).toEqual(JSON.parse(JSON.stringify(report)));
 expect(JSON.parse(readFileSync(join(base,"plan.json"),"utf8"))).toEqual(plan);
 expect(readFileSync(join(base,"report.md"),"utf8")).toContain("SKIPPED, not passed");
 const samples=readFileSync(join(base,"samples.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(samples.length).toBeGreaterThanOrEqual(10);for(const sampleRow of samples)check("Sample",sampleRow);
});

test("a run stops between suites when it is cancelled, logs nothing for the suites it did not start and still writes the report",async()=>{
 const out=directory("cancel"),seen:string[]=[],controller=new AbortController(),time=clock(),messages:string[]=[];
 const base=ports({one:{lines:[" 1 pass"]},two:{lines:[" 1 pass"]}},seen,time);
 const report=await runCiSuitesV1({plan:{version:1,host:"linux",label:"cancel",rows:[row("one"),row("two")],unrouted:[]},policy:CI_SUITE_POLICY_V1,tool:"nx",out,language:"de",enforce:true,signal:controller.signal,ports:{...base,write:line=>{messages.push(line);},start:async(...input)=>{const result=await base.start(...input);controller.abort();return result;}}});
 expect(seen.length).toBe(1);
 expect(report.outcomes.map(item=>item.suite)).toEqual(["one"]);
 expect(report.violations).toEqual([{table:"process",suite:"two",reason:"missing"}]);
 expect(report.ok).toBe(false);
 expect(messages.join("\n")).toContain("Durchlauf vor process/two abgebrochen");
 expect(existsSync(join(out,"cancel","logs","two.log"))).toBe(false);
});

test("a must-run sweep fails when a suite that has to run on the host was skipped or never ran, and passes when all passed",async()=>{
 const wanted=planCiMustRunV1(tables,CI_SUITE_POLICY_V1,"linux"),text=(suite:string)=>textV1(PROCESS_TEXTS_V1.catalog,"script.suite-platform-skipped","en",{suite,platforms:"darwin",host:"linux",count:1});
 const good=await runCiSuitesV1({plan:wanted,policy:CI_SUITE_POLICY_V1,tool:"bazel",out:directory("must-good"),language:"en",enforce:true,ports:ports(Object.fromEntries(wanted.rows.map(item=>[item.suite,{lines:[" 2 pass"]}])),[],clock(),observing)});
 expect(good.ok).toBe(true);expect(good.violations).toEqual([]);expect(good.unobserved).toEqual([]);
 const blind=await runCiSuitesV1({plan:wanted,policy:CI_SUITE_POLICY_V1,tool:"bazel",out:directory("must-blind"),language:"de",enforce:true,ports:ports(Object.fromEntries(wanted.rows.map(item=>[item.suite,{lines:[" 2 pass"]}])),[])});
 expect(blind.ok).toBe(false);expect(blind.unobserved).toEqual(["host"]);expect(blind.violations).toEqual([]);
 expect(renderCiSuiteReportV1(blind,"de")).toContain("Während des Durchlaufs nie beobachtet: Rechnerspeicher");
 const treeless=await runCiSuitesV1({plan:wanted,policy:CI_SUITE_POLICY_V1,tool:"bazel",out:directory("must-treeless"),language:"en",enforce:true,ports:ports(Object.fromEntries(wanted.rows.map(item=>[item.suite,{lines:[" 2 pass"]}])),[],clock(),async()=>({...(await observing()),treeMegabytes:null}))});
 expect(treeless.ok).toBe(false);expect(treeless.unobserved).toEqual(["tree"]);
 const sweep=await runCiSuitesV1({plan:wanted,policy:CI_SUITE_POLICY_V1,tool:"bazel",out:directory("sweep-blind"),language:"en",enforce:false,ports:ports(Object.fromEntries(wanted.rows.map(item=>[item.suite,{lines:[" 2 pass"]}])),[])});
 expect(sweep.ok).toBe(true);expect(sweep.unobserved).toEqual(["host"]);
 const bad=await runCiSuitesV1({plan:wanted,policy:CI_SUITE_POLICY_V1,tool:"bazel",out:directory("must-bad"),language:"en",enforce:true,ports:ports(Object.fromEntries(wanted.rows.map(item=>[item.suite,{lines:item.suite==="leases"?[text("leases")]:[" 2 pass"]}])),[],clock(),observing)});
 expect(bad.ok).toBe(false);expect(bad.violations).toEqual([{table:"process",suite:"leases",reason:"skipped"}]);
});

test("the command line plans and refuses what it cannot run, in English and German, and prints the Markdown report of a run",async()=>{
 const output:string[]=[],write=(text:string)=>{output.push(text);};
 expect(await runCiSuitesCommandV1(["plan","--shard","process:1/2","--host","linux"],{SEMIO_LANGUAGE:"en"},write,undefined)).toBe(0);
 const plan=JSON.parse(output.join(""));check("Plan",plan);expect(plan.label).toBe("process-1of2");
 for(const [argv,language,expected] of [[["plan"],"en","Name either"],[["plan","--shard","process:1/2","--must-run"],"en","Name either"],[["plan","--shard","bogus"],"en","Shard"],[["plan","--bogus"],"en","Unknown or incomplete option"],[["plan","--shard","process:1/2","--lang","de","--bogus"],"de","Unbekannte"],[["run","--shard","process:1/2"],"en","--tool"],[["plan","--must-run","--host","plan9"],"en","--host"]] as const){
  output.length=0;
  expect(await runCiSuitesCommandV1(argv,{SEMIO_LANGUAGE:language},write,undefined),argv.join(" ")).toBe(2);
  expect(output.join(""),argv.join(" ")).toContain(expected);
 }
 output.length=0;expect(await runCiSuitesCommandV1([],{SEMIO_LANGUAGE:"en"},write,undefined)).toBe(0);expect(output.join("")).toContain("Usage");
 output.length=0;expect(await runCiSuitesCommandV1(["bogus"],{SEMIO_LANGUAGE:"de"},write,undefined)).toBe(2);expect(output.join("")).toContain("Aufruf");
 output.length=0;
 const out=directory("command"),seen:string[]=[],wanted=planCiMustRunV1(tables,CI_SUITE_POLICY_V1,"linux");
 const code=await runCiSuitesCommandV1(["run","--must-run","--host","linux","--tool","bazel","--out",out,"--lang","de"],{SEMIO_LANGUAGE:"en",GITHUB_ACTIONS:"true"},write,ports(Object.fromEntries(wanted.rows.map(item=>[item.suite,{lines:[" 2 pass"]}])),seen,clock(),observing));
 expect(code).toBe(0);expect(seen.length).toBe(wanted.rows.length);expect(output.join("")).toContain("Urteil: Jede ausgewählte Suite lief");
 output.length=0;
 const failed=await runCiSuitesCommandV1(["run","--must-run","--host","linux","--tool","bazel","--out",directory("command-bad")],{SEMIO_LANGUAGE:"en",GITHUB_ACTIONS:"true"},write,ports(Object.fromEntries(wanted.rows.map(item=>[item.suite,{lines:[],exitCode:1}])),[]));
 expect(failed).toBe(1);expect(output.join("")).toContain("::error::must-run failed");
});

test.skipIf(process.platform==="win32")("the default route starts `repo run <project>:<target>` for real, reads its lines and its exit status, and ends a suite that outlives its budget",async()=>{
 const home=directory("repo"),program=join(home,"repo");
 mkdirSync(home,{recursive:true});
 writeFileSync(program,`#!/bin/sh\ncase "$*" in *test-hangs*) sleep 30;; *test-fails*) echo " 1 fail"; exit 3;; *) echo " 5 pass"; echo "args: $*";; esac\n`);chmodSync(program,0o755);
 const {defaultCiSuitePortsV1}=await import("../📜️script.ts"),real=defaultCiSuitePortsV1({}),lines:string[]=[],previous=process.env.PATH;
 process.env.PATH=home+delimiter+(previous??"");
 try{
  const passed=await real.start(row("good"),"bazel",20000,line=>lines.push(line),new AbortController().signal);
  expect(passed).toEqual({exitCode:0,timedOut:false});expect(lines).toContain(" 5 pass");
  expect(lines.join("\n")).toContain("args: --tool=bazel run p:test-good --skip-cache --output=stream");
  expect(await real.start(row("fails"),"nx",20000,()=>{},new AbortController().signal)).toEqual({exitCode:3,timedOut:false});
  const slow=await real.start(row("hangs"),"nx",1500,()=>{},new AbortController().signal);
  expect(slow.timedOut).toBe(true);
 }finally{if(previous===undefined)delete process.env.PATH;else process.env.PATH=previous;}
});
