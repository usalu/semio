import {appendFileSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {bindCatalogV1,textV1,type TextValuesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {repoErrorV1,type RepoErrorCodeV1,type RepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readProcessTestTableV1,processTestSuiteRunsOnV1,type ProcessTestTableV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🗂️suites/🟦️.ts";
import {PROCESS_TEXTS_V1} from "../../../../../../../🔨️modules/🏃️process/📖️texts/🟦️.ts";
import {readCachingTestTableV1} from "../../../⚡️caching/🗂️suites/🟦️.ts";
import {readBuildTestTableV1} from "../../🗂️suites/🟦️.ts";
import policyDocument from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import texts from "./📖️texts/🔣️.json" with {type:"json"};

/** 🖥️ The hosts the sweep runs on. */
export type CiSuiteHostV1="darwin"|"linux"|"win32";
/** 🗂️ The three closed route tables of the sweep. */
export type CiSuiteTableIdV1="process"|"caching"|"build";
/** 🌍️ A report language; English first, German second. */
export type CiSuiteLanguageV1="en"|"de";
/** 📜️ One suite that has to really run on a host, with the fact it proves there. */
export type CiMustRunV1=Readonly<{table:CiSuiteTableIdV1;suite:string;proves:Readonly<Record<CiSuiteLanguageV1,string>>}>;
/** 🏛️ The closed policy: how the tables are cut into shards, the budget bounds and the suites a host must run. */
export type CiSuitePolicyV1=Readonly<{version:1;maximumShardBudgetMinutes:number;suiteSlackMinutes:number;sampleSeconds:number;tables:readonly Readonly<{id:CiSuiteTableIdV1;project:string;projectFile:string;shards:number}>[];mustRun:Readonly<Record<"all"|CiSuiteHostV1,readonly CiMustRunV1[]>>}>;
/** 📋️ One suite of a plan: the official target that runs it and whether this host is one of its platforms. */
export type CiPlanRowV1=Readonly<{table:CiSuiteTableIdV1;suite:string;project:string;target:string;budgetMs:number;runsHere:boolean}>;
/** 🗺️ The suites one job runs, in order, and the suites of the selection that have no test target. */
export type CiPlanV1=Readonly<{version:1;host:CiSuiteHostV1;label:string;rows:readonly CiPlanRowV1[];unrouted:readonly Readonly<{table:CiSuiteTableIdV1;suite:string}>[]}>;
/** 🔢️ Test counts printed by the test runner. */
export type CiCountsV1=Readonly<{pass:number;skip:number;fail:number}>;
/** 🧠️ What the host memory samples said while one suite ran. */
export type CiMemoryV1=Readonly<{samples:number;minimumAvailableMegabytes:number|null;peakUsedMegabytes:number|null;peakSwapMegabytes:number|null;peakTreeMegabytes:number|null;worstPressure:"normal"|"warn"|"critical"|null}>;
/** 🧾️ The result of one suite. */
export type CiOutcomeV1=Readonly<{table:CiSuiteTableIdV1;suite:string;status:"passed"|"skipped"|"failed"|"timeout"|"unrouted";exitCode:number|null;wallMs:number;tests:CiCountsV1;memory:CiMemoryV1;log:string|null}>;
/** 🚩️ A suite that had to run on this host and did not. */
export type CiViolationV1=Readonly<{table:CiSuiteTableIdV1;suite:string;reason:"skipped"|"failed"|"timeout"|"unrouted"|"missing"|"not-declared"}>;
/** 📊️ The verdict of one job. */
export type CiSuiteReportV1=Readonly<{version:1;host:CiSuiteHostV1;label:string;tool:string;ok:boolean;unobserved:readonly ("host"|"tree")[];outcomes:readonly CiOutcomeV1[];violations:readonly CiViolationV1[];totals:Readonly<{passed:number;skipped:number;failed:number;timeout:number;unrouted:number;wallMs:number;tests:CiCountsV1}>}>;
/** 📈️ One host memory observation. */
export type CiSampleV1=Readonly<{atMs:number;availableMegabytes:number;totalMegabytes:number;swapMegabytes:number|null;treeMegabytes:number|null;pressure:"normal"|"warn"|"critical"|null}>;
/** 📚️ A route table with the project that owns its targets and, per suite, the target that runs it. */
export type CiSuiteTableV1=Readonly<{id:CiSuiteTableIdV1;project:string;shards:number;table:ProcessTestTableV1;targets:ReadonlyMap<string,string>}>;

/** 📖️ The bilingual texts of the sweep. */
export const CI_SUITE_TEXTS_V1=bindCatalogV1("ci-suites",texts);
/** 🔑️ Every message key of the sweep texts. */
export type CiSuiteTextKeyV1=keyof typeof texts.messages;
/** 🏠️ The repository root, found from this module. */
export const CI_SUITE_ROOT_V1:string=resolve(import.meta.dir,"..","..","..","..","..","..","..","..");

/** 💬️ Renders one message of the sweep texts. */
export const ciSuiteTextV1=(language:CiSuiteLanguageV1,key:CiSuiteTextKeyV1,values:TextValuesV1={}):string=>textV1(CI_SUITE_TEXTS_V1.catalog,key,language,values);

/** 🚨️ Builds a refusal of the sweep texts as data; the command line renders it in the language the developer chose. */
export const refuseCiSuiteV1=(code:RepoErrorCodeV1,key:CiSuiteTextKeyV1,values:TextValuesV1={}):RepoErrorV1=>repoErrorV1(CI_SUITE_TEXTS_V1,code,key,values);

const requireShape=(definition:string,value:unknown,document:string):void=>{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw refuseCiSuiteV1("invalid","refusal-document",{document,detail:errors[0]!});
};

/** 🛂️ Validates a policy document against its closed schema and its cross rules: one row per table, no suite twice in the host lists of one host. */
export function readCiSuitePolicyV1(value:unknown=policyDocument):CiSuitePolicyV1{
 requireShape("Policy",value,"policy");
 const policy=value as CiSuitePolicyV1;
 if(new Set(policy.tables.map(row=>row.id)).size!==3)throw refuseCiSuiteV1("invalid","refusal-document",{document:"policy",detail:"tables"});
 for(const host of ["darwin","linux","win32"] as const){
  const keys=[...policy.mustRun.all,...policy.mustRun[host]].map(row=>`${row.table}/${row.suite}`);
  if(new Set(keys).size!==keys.length)throw refuseCiSuiteV1("invalid","refusal-document",{document:"policy",detail:host});
 }
 return Object.freeze(policy);
}

/** 🔒️ The committed policy. */
export const CI_SUITE_POLICY_V1:CiSuitePolicyV1=readCiSuitePolicyV1();

const readers:Readonly<Record<CiSuiteTableIdV1,()=>ProcessTestTableV1>>={process:()=>readProcessTestTableV1(),caching:()=>readCachingTestTableV1(),build:()=>readBuildTestTableV1()};

/** 🎯️ Finds, per suite, the target of a project that runs it: the one whose command ends with `test <suite>`, the target named `test-<suite>` when several do. */
export function ciSuiteTargetsV1(targets:Readonly<Record<string,Readonly<{options?:Readonly<{command?:string}>}>>>):ReadonlyMap<string,string>{
 const routes=new Map<string,string>();
 for(const [name,target] of Object.entries(targets).sort(([left],[right])=>left<right?-1:1)){
  const match=/ test ([a-z][a-z0-9]*(?:-[a-z0-9]+)*)$/u.exec(target.options?.command??"");
  if(match!==null&&(!routes.has(match[1]!)||name===`test-${match[1]}`))routes.set(match[1]!,name);
 }
 return routes;
}

/** 🛤️ Reads the three route tables and the targets of the projects that own them. */
export function readCiSuiteTablesV1(policy:CiSuitePolicyV1=CI_SUITE_POLICY_V1,root:string=CI_SUITE_ROOT_V1,read:Readonly<Record<CiSuiteTableIdV1,()=>ProcessTestTableV1>>=readers):readonly CiSuiteTableV1[]{
 return policy.tables.map(row=>{
  const project=JSON.parse(readFileSync(join(root,...row.projectFile.split("/")),"utf8")) as {targets:Parameters<typeof ciSuiteTargetsV1>[0]};
  return Object.freeze({id:row.id,project:row.project,shards:row.shards,table:read[row.id](),targets:ciSuiteTargetsV1(project.targets)});
 });
}

const byWeight=(left:Readonly<{id:string;budgetMs:number}>,right:Readonly<{id:string;budgetMs:number}>)=>right.budgetMs-left.budgetMs||(left.id<right.id?-1:1);

/** ✂️ Cuts the suites of a table into `count` shards by longest budget first onto the lightest shard (ties: the lowest index) and returns the ids of the shard `index` (1-based) in id order. */
export function ciShardSuitesV1(table:ProcessTestTableV1,index:number,count:number):readonly string[]{
 if(!Number.isInteger(count)||count<1||!Number.isInteger(index)||index<1||index>count)throw refuseCiSuiteV1("invalid","refusal-shard",{value:`${index}/${count}`});
 const loads=Array.from({length:count},()=>0),members:string[][]=Array.from({length:count},()=>[]);
 for(const suite of [...table.suites].sort(byWeight)){
  let target=0;
  for(let candidate=1;candidate<count;candidate++)if(loads[candidate]!<loads[target]!)target=candidate;
  loads[target]!+=suite.budgetMs;members[target]!.push(suite.id);
 }
 return members[index-1]!.sort();
}

function rowsOf(tables:readonly CiSuiteTableV1[],host:CiSuiteHostV1,wanted:readonly Readonly<{table:CiSuiteTableIdV1;suite:string}>[]):Readonly<{rows:CiPlanRowV1[];unrouted:{table:CiSuiteTableIdV1;suite:string}[];undeclared:CiViolationV1[]}>{
 const rows:CiPlanRowV1[]=[],unrouted:{table:CiSuiteTableIdV1;suite:string}[]=[],undeclared:CiViolationV1[]=[];
 for(const {table:id,suite:name} of wanted){
  const table=tables.find(row=>row.id===id)!,suite=table.table.suites.find(row=>row.id===name);
  if(suite===undefined){undeclared.push({table:id,suite:name,reason:"not-declared"});continue;}
  const target=table.targets.get(suite.id);
  if(target===undefined){unrouted.push({table:id,suite:suite.id});continue;}
  rows.push({table:id,suite:suite.id,project:table.project,target,budgetMs:suite.budgetMs,runsHere:processTestSuiteRunsOnV1(suite,host)});
 }
 return {rows,unrouted,undeclared};
}

/** 🧭️ Plans one shard of one table for a host: every suite of the shard, each with the target of the official route that runs it; a suite without a target is reported, never dropped. */
export function planCiShardV1(tables:readonly CiSuiteTableV1[],host:CiSuiteHostV1,id:CiSuiteTableIdV1,index:number,count:number):CiPlanV1{
 const table=tables.find(row=>row.id===id);
 if(table===undefined||table.shards!==count)throw refuseCiSuiteV1("invalid","refusal-shard",{value:`${id}:${index}/${count}`});
 const {rows,unrouted}=rowsOf(tables,host,ciShardSuitesV1(table.table,index,count).map(suite=>({table:id,suite})));
 const plan={version:1 as const,host,label:`${id}-${index}of${count}`,rows,unrouted};
 requireShape("Plan",plan,"plan");return Object.freeze(plan);
}

/** 🥅️ Plans the suites a host must really run: the common list and the list of the host. */
export function planCiMustRunV1(tables:readonly CiSuiteTableV1[],policy:CiSuitePolicyV1,host:CiSuiteHostV1):CiPlanV1{
 const {rows,unrouted}=rowsOf(tables,host,[...policy.mustRun.all,...policy.mustRun[host]]);
 const plan={version:1 as const,host,label:"must-run",rows,unrouted};
 requireShape("Plan",plan,"plan");return Object.freeze(plan);
}

const ANSI=/\u001b\[[0-9;]*m/gu,COUNT=/^\s*(\d+)\s+(pass|skip|fail)\s*$/u;

/** 🧮️ Adds the count line of the test runner (` 12 pass`, ` 3 skip`, ` 0 fail`) in `line` to `counts`; other lines change nothing. */
export function addCiCountsV1(counts:CiCountsV1,line:string):CiCountsV1{
 const match=COUNT.exec(line.replace(ANSI,""));
 return match===null?counts:{...counts,[match[2]!]:counts[match[2] as keyof CiCountsV1]+Number(match[1])};
}

/** ⏭️ Builds the matcher of the line the route prints when a suite is declared for other hosts, from the catalog of the route itself so the text has one source: both languages, the suite filled in, every other placeholder open. */
export function ciSkipMatcherV1(suite:string):RegExp{
 const messages=PROCESS_TEXTS_V1.catalog.messages["script.suite-platform-skipped"]!,escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
 const expression=(template:string)=>"^"+template.split(/\{([a-z]+)\}/u).map((part,index)=>index%2===0?escape(part):part==="suite"?escape(suite):".+").join("")+"$";
 return new RegExp(`${expression(messages.en)}|${expression(messages.de)}`,"u");
}

const PRESSURE={normal:0,warn:1,critical:2} as const;

/** 🌡️ Reduces the samples taken while one suite ran. */
export function summarizeCiMemoryV1(samples:readonly CiSampleV1[]):CiMemoryV1{
 if(samples.length===0)return {samples:0,minimumAvailableMegabytes:null,peakUsedMegabytes:null,peakSwapMegabytes:null,peakTreeMegabytes:null,worstPressure:null};
 const swaps=samples.flatMap(sample=>sample.swapMegabytes===null?[]:[sample.swapMegabytes]),trees=samples.flatMap(sample=>sample.treeMegabytes===null?[]:[sample.treeMegabytes]),levels=samples.flatMap(sample=>sample.pressure===null?[]:[sample.pressure]);
 return {
  samples:samples.length,
  minimumAvailableMegabytes:Math.min(...samples.map(sample=>sample.availableMegabytes)),
  peakUsedMegabytes:Math.max(...samples.map(sample=>sample.totalMegabytes-sample.availableMegabytes)),
  peakSwapMegabytes:swaps.length?Math.max(...swaps):null,
  peakTreeMegabytes:trees.length?Math.max(...trees):null,
  worstPressure:levels.length?levels.reduce((worst,level)=>PRESSURE[level]>PRESSURE[worst]?level:worst):null
 };
}

/** 🛑️ The suites that failed a report: every outcome that failed, timed out or has no target, and every violation, each suite once. */
export function failingCiSuitesV1(report:CiSuiteReportV1):readonly string[]{
 return [...new Set([...report.outcomes.filter(row=>row.status==="failed"||row.status==="timeout"||row.status==="unrouted").map(row=>`${row.table}/${row.suite}`),...report.violations.map(row=>`${row.table}/${row.suite}`)])].sort();
}

/** 🔭️ The measurements that never produced a value in the whole run: the host memory when no suite has a sample, the memory of the process tree when no sample of a suite carries it. */
export function unobservedCiMeasurementsV1(outcomes:readonly CiOutcomeV1[]):readonly ("host"|"tree")[]{
 const ran=outcomes.filter(row=>row.status!=="unrouted"),sampled=ran.filter(row=>row.memory.samples>0),found:("host"|"tree")[]=[];
 if(ran.length>0&&sampled.length===0)found.push("host");
 if(sampled.length>0&&sampled.every(row=>row.memory.peakTreeMegabytes===null))found.push("tree");
 return found;
}

/** ⚖️ Judges a run: counts the outcomes and, for a plan whose suites must run (`enforce`), turns every suite that did not pass into a violation and refuses a run whose memory was never observed. A suite that is only skipped does not fail a sweep, but it is never counted as passed. */
export function judgeCiSuiteRunV1(input:Readonly<{plan:CiPlanV1;tool:string;outcomes:readonly CiOutcomeV1[];enforce:boolean;undeclared?:readonly CiViolationV1[]}>):CiSuiteReportV1{
 const outcomes=[...input.outcomes],count=(status:CiOutcomeV1["status"])=>outcomes.filter(row=>row.status===status).length,violations:CiViolationV1[]=[...input.undeclared??[]];
 if(input.enforce){
  for(const row of input.plan.rows){
   const outcome=outcomes.find(item=>item.table===row.table&&item.suite===row.suite);
   if(outcome===undefined)violations.push({table:row.table,suite:row.suite,reason:"missing"});
   else if(outcome.status!=="passed")violations.push({table:row.table,suite:row.suite,reason:outcome.status});
  }
  for(const item of input.plan.unrouted)violations.push({table:item.table,suite:item.suite,reason:"unrouted"});
 }
 const tests=outcomes.reduce<CiCountsV1>((sum,row)=>({pass:sum.pass+row.tests.pass,skip:sum.skip+row.tests.skip,fail:sum.fail+row.tests.fail}),{pass:0,skip:0,fail:0});
 const totals={passed:count("passed"),skipped:count("skipped"),failed:count("failed"),timeout:count("timeout"),unrouted:count("unrouted"),wallMs:outcomes.reduce((sum,row)=>sum+row.wallMs,0),tests};
 const unobserved=unobservedCiMeasurementsV1(outcomes),draft={version:1 as const,host:input.plan.host,label:input.plan.label,tool:input.tool,ok:false,unobserved,outcomes,violations,totals};
 const report={...draft,ok:failingCiSuitesV1(draft).length===0&&(!input.enforce||unobserved.length===0)};
 requireShape("Report",report,"report");return Object.freeze(report);
}

const seconds=(ms:number)=>(ms/1000).toFixed(1);
const STATUS_KEYS:Readonly<Record<CiOutcomeV1["status"],CiSuiteTextKeyV1>>={passed:"status-passed",skipped:"status-skipped",failed:"status-failed",timeout:"status-timeout",unrouted:"status-unrouted"};
const KIND_KEYS:Readonly<Record<"host"|"tree",CiSuiteTextKeyV1>>={host:"kind-host",tree:"kind-tree"};
const VIOLATION_KEYS:Readonly<Record<CiViolationV1["reason"],CiSuiteTextKeyV1>>={skipped:"violation-skipped",failed:"violation-failed",timeout:"violation-timeout",unrouted:"violation-unrouted",missing:"violation-missing","not-declared":"violation-not-declared"};

/** 📝️ Renders a report as Markdown: totals first, the skipped suites by name, the violations, then one table row per suite. */
export function renderCiSuiteReportV1(report:CiSuiteReportV1,language:CiSuiteLanguageV1,proves:readonly CiMustRunV1[]=[]):string{
 const text=(key:CiSuiteTextKeyV1,values:TextValuesV1={})=>ciSuiteTextV1(language,key,values),totals=report.totals;
 const lines=[`### ${text("title",{label:report.label,host:report.host,tool:report.tool})}`,"",`- ${text("totals",{passed:totals.passed,skipped:totals.skipped,failed:totals.failed,timeout:totals.timeout,unrouted:totals.unrouted,pass:totals.tests.pass,skip:totals.tests.skip,fail:totals.tests.fail,minutes:(totals.wallMs/60000).toFixed(1)})}`,`- ${report.ok?text("verdict-ok"):text("verdict-failed",{count:failingCiSuitesV1(report).length})}`,...report.unobserved.length?[`- ${text("unobserved",{kinds:report.unobserved.map(kind=>text(KIND_KEYS[kind])).join(", ")})}`]:[],""];
 if(report.violations.length)lines.push(text("violations"),"",...report.violations.map(row=>`- ${text(VIOLATION_KEYS[row.reason],{table:row.table,suite:row.suite})}`),"");
 const skipped=report.outcomes.filter(row=>row.status==="skipped");
 if(skipped.length)lines.push(text("status-skipped")+":","",...skipped.map(row=>`- ${row.table}/${row.suite}`),"");
 lines.push(`| ${[text("header-suite"),text("header-status"),text("header-wall"),text("header-tests"),text("header-memory"),text("header-tree")].join(" | ")} |`,"| --- | --- | ---: | ---: | ---: | ---: |");
 for(const row of report.outcomes)lines.push(`| ${row.table}/${row.suite} | ${text(STATUS_KEYS[row.status])} | ${seconds(row.wallMs)} | ${row.tests.pass}/${row.tests.skip}/${row.tests.fail} | ${row.memory.peakUsedMegabytes===null?text("memory-unavailable"):row.memory.peakUsedMegabytes} | ${row.memory.peakTreeMegabytes===null?text("memory-unavailable"):row.memory.peakTreeMegabytes} |`);
 const shown=proves.filter(row=>report.outcomes.some(outcome=>outcome.table===row.table&&outcome.suite===row.suite&&outcome.status==="passed"));
 if(shown.length)lines.push("",...shown.map(row=>`- ${row.table}/${row.suite}: ${text("proves",{text:row.proves[language]})}`));
 lines.push("");return lines.join("\n");
}

/** 🧪️ Ports of one sweep: how a suite is started, how host memory is read, the clock, where lines go and where a log group opens (a title) or closes (null). */
export type CiSuiteRunPortsV1=Readonly<{
 start:(row:CiPlanRowV1,tool:string,timeoutMs:number,onLine:(line:string)=>void,signal:AbortSignal)=>Promise<Readonly<{exitCode:number|null;timedOut:boolean}>>;
 sample:()=>Promise<CiSampleV1|null>;
 now:()=>number;
 write:(line:string)=>void;
 group:(title:string|null)=>void;
}>;

/** 🏃️ Runs the suites of a plan one after the other through the ports (the official `repo run` route), writes the log of each suite, the memory samples and the report into `out`, and stops between suites when `signal` aborts. */
export async function runCiSuitesV1(input:Readonly<{plan:CiPlanV1;policy:CiSuitePolicyV1;tool:string;out:string;language:CiSuiteLanguageV1;enforce:boolean;ports:CiSuiteRunPortsV1;signal?:AbortSignal;undeclared?:readonly CiViolationV1[]}>):Promise<CiSuiteReportV1>{
 const {plan,policy,ports}=input,signal=input.signal??new AbortController().signal,directory=join(input.out,plan.label),logs=join(directory,"logs"),outcomes:CiOutcomeV1[]=[];
 mkdirSync(logs,{recursive:true});writeFileSync(join(directory,"plan.json"),JSON.stringify(plan,null,1)+"\n");writeFileSync(join(directory,"samples.jsonl"),"");
 let window:CiSampleV1[]=[],busy=false,inflight:Promise<void>=Promise.resolve();
 const read=async()=>{
  busy=true;
  try{const sample=await ports.sample();if(sample!==null){window.push(sample);appendFileSync(join(directory,"samples.jsonl"),JSON.stringify(sample)+"\n");}}catch{}finally{busy=false;}
 };
 const take=():Promise<void>=>inflight=inflight.then(read);
 const ticker=setInterval(()=>{if(!busy)void take();},policy.sampleSeconds*1000);
 try{
  for(const [position,row] of plan.rows.entries()){
   if(signal.aborted){ports.write(ciSuiteTextV1(input.language,"sweep-stop",{table:row.table,suite:row.suite}));break;}
   ports.group(ciSuiteTextV1(input.language,"sweep-start",{position:position+1,count:plan.rows.length,table:row.table,suite:row.suite}));
   window=[];await take();
   const path=join(logs,`${row.suite}.log`),matcher=ciSkipMatcherV1(row.suite),started=ports.now();
   let counts:CiCountsV1={pass:0,skip:0,fail:0},skipped=false,batch:string[]=[];
   writeFileSync(path,"");
   const flush=()=>{if(batch.length){appendFileSync(path,batch.join("\n")+"\n");batch=[];}};
   const result=await ports.start(row,input.tool,row.budgetMs+policy.suiteSlackMinutes*60000,line=>{
    counts=addCiCountsV1(counts,line);if(!skipped&&matcher.test(line.replace(ANSI,"")))skipped=true;
    batch.push(line);if(batch.length>=200)flush();
   },signal);
   flush();await take();
   const status:CiOutcomeV1["status"]=result.timedOut?"timeout":result.exitCode!==0?"failed":skipped?"skipped":"passed";
   outcomes.push({table:row.table,suite:row.suite,status,exitCode:result.exitCode,wallMs:Math.max(0,ports.now()-started),tests:counts,memory:summarizeCiMemoryV1(window),log:`logs/${row.suite}.log`});
   ports.group(null);
  }
 }finally{clearInterval(ticker);}
 for(const item of plan.unrouted)outcomes.push({table:item.table,suite:item.suite,status:"unrouted",exitCode:null,wallMs:0,tests:{pass:0,skip:0,fail:0},memory:summarizeCiMemoryV1([]),log:null});
 const report=judgeCiSuiteRunV1({plan,tool:input.tool,outcomes,enforce:input.enforce,...input.undeclared?{undeclared:input.undeclared}:{}});
 writeFileSync(join(directory,"report.json"),JSON.stringify(report,null,1)+"\n");
 const markdown=renderCiSuiteReportV1(report,input.language,[...policy.mustRun.all,...policy.mustRun[plan.host]]);
 writeFileSync(join(directory,"report.md"),markdown);
 return report;
}
