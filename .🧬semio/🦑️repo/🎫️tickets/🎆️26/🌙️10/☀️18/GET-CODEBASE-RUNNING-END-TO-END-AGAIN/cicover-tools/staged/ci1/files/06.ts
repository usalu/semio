/** 🔀️ Tests for the workflows and composite actions of the repository: a local stand-in for a workflow linter (the tool registry pins none), checking structure, expression contexts, references, shell syntax and the laws of the suite sweep. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {parse as parseYaml} from "yaml";
import {CI_SUITE_POLICY_V1} from "../../🟦️.ts";
import definitions from "./🧫️fixtures/🔣️.json" with {type:"json"};

const root=((start:string):string=>{for(let path=start;;path=dirname(path)){if(existsSync(join(path,".github","workflows","build-system.yml")))return path;if(dirname(path)===path)throw Error("repository root with .github/workflows required");}})(import.meta.dir);
type Step={id?:string;name?:string;if?:string|boolean;uses?:string;with?:Record<string,unknown>;run?:string;shell?:string;env?:Record<string,unknown>;"continue-on-error"?:boolean|string;"working-directory"?:string;"timeout-minutes"?:number};
type Job={name?:string;needs?:string|string[];if?:string|boolean;"runs-on":string;"timeout-minutes"?:number|string;"continue-on-error"?:boolean|string;env?:Record<string,unknown>;outputs?:Record<string,string>;strategy?:{"fail-fast"?:boolean;matrix:{include:Record<string,string|number|boolean>[]}};steps:Step[]};
type Workflow={name:string;on:Record<string,unknown>;permissions:Record<string,string>;concurrency?:Record<string,string>;env?:Record<string,unknown>;jobs:Record<string,Job>};
type Action={name:string;description:string;inputs?:Record<string,{description:string;required?:boolean;default?:unknown}>;runs:{using:string;steps:Step[]}};

const files=(directory:string,name:(entry:string)=>boolean):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?files(join(directory,entry.name),name):name(entry.name)?[join(directory,entry.name)]:[]).sort();
const workflowFiles=files(join(root,".github","workflows"),name=>name.endsWith(".yml")),actionFiles=files(join(root,".github","actions"),name=>name==="action.yml");
const workflows=Object.fromEntries(workflowFiles.map(path=>[path,parseYaml(readFileSync(path,"utf8"),{strict:true,uniqueKeys:true}) as Workflow])),actions=Object.fromEntries(actionFiles.map(path=>[path,parseYaml(readFileSync(path,"utf8"),{strict:true,uniqueKeys:true}) as Action]));
const build=workflows[join(root,".github","workflows","build-system.yml")]!;
const ajv=new Ajv({strict:true,allowUnionTypes:true});
const validate=(definition:string,value:unknown)=>{const check=ajv.compile({$schema:definitions.$schema,definitions:definitions.definitions,$ref:`#/definitions/${definition}`});return check(value)?[]:[ajv.errorsText(check.errors)];};

test("every workflow and composite action is a closed document of the supported syntax",()=>{
 expect(workflowFiles.length).toBeGreaterThanOrEqual(3);expect(actionFiles.length).toBeGreaterThanOrEqual(5);
 for(const [path,value] of Object.entries(workflows))expect(validate("Workflow",value),path).toEqual([]);
 for(const [path,value] of Object.entries(actions))expect(validate("Action",value),path).toEqual([]);
 expect(validate("Job",{"runs-on":"x",steps:[{run:"x",uses:"y"}]}).length).toBeGreaterThan(0);
 expect(validate("Job",{"runs-on":"x",steps:[{run:"x"}],"continue_on_error":true}).length).toBeGreaterThan(0);
 expect(validate("Job",{"runs-on":"x","timeout-minutes":361,steps:[{run:"x"}]}).length).toBeGreaterThan(0);
});

const CONTEXTS:Readonly<Record<string,readonly string[]>>={
 "workflow.env":["github","secrets","inputs","vars"],"workflow.concurrency":["github","inputs","vars"],
 "job.name":["github","needs","strategy","matrix","vars","inputs"],"job.runs-on":["github","needs","strategy","matrix","vars","inputs"],"job.if":["github","needs","vars","inputs"],
 "job.env":["github","needs","strategy","matrix","vars","secrets","inputs"],"job.continue-on-error":["github","needs","strategy","vars","matrix","inputs"],"job.timeout-minutes":["github","needs","strategy","matrix","vars","inputs"],
 "job.outputs":["github","needs","strategy","matrix","job","runner","env","vars","secrets","steps","inputs"],
 "step":["github","needs","strategy","matrix","job","runner","env","vars","secrets","steps","inputs"],"step.if":["github","needs","strategy","matrix","job","runner","env","vars","steps","inputs"],
 "composite":["github","runner","env","vars","inputs","steps","job"]
};
const FUNCTIONS=new Set(["format","hashFiles","contains","startsWith","endsWith","fromJSON","toJSON","join","success","failure","always","cancelled"]);
type Located={where:string;kind:string;expression:string;stepIndex:number};

function expressions(text:string):string[]{
 const found:string[]=[],pattern=/\$\{\{([\s\S]*?)\}\}/gu;
 for(let match=pattern.exec(text);match!==null;match=pattern.exec(text))found.push(match[1]!.trim());
 const without=text.replace(pattern,"");
 if(without.includes("${{")||without.includes("}}"))throw Error(`unbalanced expression delimiters in ${text}`);
 return found;
}
function locate(workflow:Workflow):Located[]{
 const rows:Located[]=[],add=(where:string,kind:string,value:unknown,stepIndex=-1)=>{
  if(typeof value==="string")for(const expression of expressions(value))rows.push({where,kind,expression,stepIndex});
  else if(value!==null&&typeof value==="object")for(const [key,inner] of Object.entries(value as Record<string,unknown>))add(`${where}.${key}`,kind,inner,stepIndex);
 };
 add("env","workflow.env",workflow.env);add("concurrency","workflow.concurrency",workflow.concurrency);
 for(const [id,job] of Object.entries(workflow.jobs)){
  add(`${id}.name`,"job.name",job.name);add(`${id}.runs-on`,"job.runs-on",job["runs-on"]);add(`${id}.if`,"job.if",job.if);add(`${id}.env`,"job.env",job.env);
  add(`${id}.continue-on-error`,"job.continue-on-error",job["continue-on-error"]);add(`${id}.timeout-minutes`,"job.timeout-minutes",job["timeout-minutes"]);add(`${id}.outputs`,"job.outputs",job.outputs);
  job.steps.forEach((step,index)=>{
   const at=`${id}.steps[${index}]${step.name?` ${step.name}`:""}`;
   add(`${at}.if`,"step.if",step.if,index);for(const key of ["name","uses","with","run","env","working-directory","continue-on-error","shell"] as const)add(`${at}.${key}`,"step",step[key],index);
  });
 }
 return rows;
}
const stripStrings=(expression:string):string=>expression.replace(/'(?:[^']|'')*'/gu,"''");
const rootsOf=(expression:string):string[]=>[...stripStrings(expression).matchAll(/(?<![\w.-])([a-z]+)(?=[.[])/gu)].map(match=>match[1]!);
const balanced=(expression:string):boolean=>{let depth=0;for(const character of stripStrings(expression)){if(character==="(")depth++;if(character===")"&&--depth<0)return false;}return depth===0&&(expression.match(/'/gu)??[]).length%2===0;};

test("every expression is balanced, calls known functions and reads only the contexts the place allows",()=>{
 let total=0;
 for(const [path,workflow] of Object.entries(workflows))for(const row of locate(workflow)){
  total++;
  expect(balanced(row.expression),`${path} ${row.where}: ${row.expression}`).toBe(true);
  for(const call of stripStrings(row.expression).matchAll(/([A-Za-z]+)\(/gu))expect(FUNCTIONS.has(call[1]!),`${path} ${row.where}: function ${call[1]}`).toBe(true);
  for(const context of rootsOf(row.expression))expect(CONTEXTS[row.kind]!.includes(context),`${path} ${row.where}: context ${context} is not available in ${row.kind}`).toBe(true);
 }
 expect(total).toBeGreaterThan(60);
 for(const [path,action] of Object.entries(actions))for(const [index,step] of action.runs.steps.entries())for(const key of ["if","name","uses","with","run","env"] as const){
  const value=step[key];
  for(const expression of typeof value==="string"?expressions(value):[])for(const context of rootsOf(expression))expect(CONTEXTS.composite!.includes(context),`${path} step ${index}: context ${context}`).toBe(true);
 }
 expect(rootsOf("runner.temp")).toEqual(["runner"]);expect(rootsOf("format('{0}', github.sha) && steps.a.outputs.b")).toEqual(["github","steps"]);expect(balanced("a && (b")).toBe(false);
 const seeded=(kind:string,expression:string)=>rootsOf(expression).filter(context=>!CONTEXTS[kind]!.includes(context));
 expect(seeded("job.env","runner.temp")).toEqual(["runner"]);expect(seeded("workflow.env","needs.x.result")).toEqual(["needs"]);expect(seeded("step","runner.temp")).toEqual([]);
});

test("every reference resolves: needs name jobs without a cycle, matrix keys exist in every row, step outputs come from an earlier step, inputs are declared",()=>{
 for(const [path,workflow] of Object.entries(workflows)){
  const ids=new Set(Object.keys(workflow.jobs)),edges=new Map(Object.entries(workflow.jobs).map(([id,job])=>[id,[job.needs??[]].flat()]));
  for(const [id,needs] of edges)for(const need of needs)expect(ids.has(need),`${path} ${id} needs ${need}`).toBe(true);
  const visiting=new Set<string>(),done=new Set<string>(),visit=(id:string)=>{expect(visiting.has(id),`${path}: cycle through ${id}`).toBe(false);if(done.has(id))return;visiting.add(id);for(const need of edges.get(id)!)visit(need);visiting.delete(id);done.add(id);};
  for(const id of ids)visit(id);
  const inputs=Object.keys((workflow.on.workflow_dispatch as {inputs?:Record<string,unknown>}|undefined)?.inputs??{});
  for(const row of locate(workflow)){
   const [id]=row.where.split(".") as [string],job=workflow.jobs[id];
   for(const match of row.expression.matchAll(/(?<![\w.])matrix\.([A-Za-z_][\w-]*)/gu)){
    expect(job?.strategy,`${path} ${row.where} uses matrix`).toBeDefined();
    for(const include of job!.strategy!.matrix.include)expect(match[1]! in include,`${path} ${row.where}: matrix.${match[1]} missing in ${JSON.stringify(include)}`).toBe(true);
   }
   for(const match of row.expression.matchAll(/(?<![\w.])needs\.([\w-]+)(?:\.outputs\.([\w-]+))?/gu)){
    if(id==="env"||id==="concurrency"){continue;}
    expect([job?.needs??[]].flat().includes(match[1]!),`${path} ${row.where}: needs.${match[1]}`).toBe(true);
    if(match[2]!==undefined)expect(workflow.jobs[match[1]!]!.outputs?.[match[2]],`${path} ${row.where}: needs.${match[1]}.outputs.${match[2]}`).toBeDefined();
   }
   for(const match of row.expression.matchAll(/(?<![\w.])steps\.([\w-]+)\./gu)){
    const earlier=job!.steps.slice(0,row.stepIndex<0?job!.steps.length:row.stepIndex).map(step=>step.id);
    expect(earlier.includes(match[1]!),`${path} ${row.where}: steps.${match[1]} is not an earlier step`).toBe(true);
   }
   for(const match of row.expression.matchAll(/(?<![\w.])inputs\.([\w-]+)/gu))expect(inputs.includes(match[1]!),`${path} ${row.where}: inputs.${match[1]}`).toBe(true);
  }
  for(const [id,job] of Object.entries(workflow.jobs)){
   const seen=new Set<string>();
   for(const step of job.steps){
    if(step.id!==undefined){expect(seen.has(step.id),`${path} ${id}: step id ${step.id} twice`).toBe(false);seen.add(step.id);}
    if(step.uses?.startsWith("./")){
     const action=actions[join(root,step.uses.slice(2),"action.yml")];
     expect(action,`${path} ${id}: local action ${step.uses}`).toBeDefined();
     const declared=Object.keys(action!.inputs??{}),required=Object.entries(action!.inputs??{}).filter(([,input])=>input.required===true&&input.default===undefined).map(([name])=>name);
     for(const given of Object.keys(step.with??{}))expect(declared.includes(given),`${path} ${id}: ${step.uses} has no input ${given}`).toBe(true);
     for(const name of required)expect(Object.keys(step.with??{}).includes(name),`${path} ${id}: ${step.uses} needs input ${name}`).toBe(true);
    }
   }
  }
 }
 for(const [path,action] of Object.entries(actions)){
  const declared=Object.keys(action.inputs??{});
  for(const step of action.runs.steps)for(const key of ["if","name","with","run","env"] as const){const value=step[key];if(typeof value==="string")for(const match of value.matchAll(/(?<![\w.])inputs\.([\w-]+)/gu))expect(declared.includes(match[1]!),`${path}: inputs.${match[1]}`).toBe(true);}
 }
});

const bash=Bun.which("bash");

test.skipIf(bash===null)("every bash script of a step and of a composite action has valid shell syntax once its expressions are filled in",()=>{
 const scratch=mkdtempSync(join(tmpdir(),"workflow-shell-"));
 try{
  let checked=0;
  const scripts=[...Object.entries(workflows).flatMap(([path,workflow])=>Object.entries(workflow.jobs).flatMap(([id,job])=>job.steps.map(step=>({where:`${path} ${id} ${step.name??step.id}`,step})))),...Object.entries(actions).flatMap(([path,action])=>action.runs.steps.map(step=>({where:`${path} ${step.name}`,step})))];
  for(const {where,step} of scripts){
   if(step.run===undefined||(step.shell!==undefined&&step.shell!=="bash"))continue;
   const file=join(scratch,`s${checked++}.sh`);writeFileSync(file,step.run.replace(/\$\{\{[\s\S]*?\}\}/gu,"X"));
   const result=spawnSync(bash!,["-n",file],{encoding:"utf8"});
   expect(result.status,`${where}: ${result.stderr}`).toBe(0);
  }
  expect(checked).toBeGreaterThan(40);
  writeFileSync(join(scratch,"bad.sh"),"if true; then\n");
  expect(spawnSync(bash!,["-n",join(scratch,"bad.sh")]).status).not.toBe(0);
 }finally{rmSync(scratch,{recursive:true,force:true});}
});

test("the suite sweep of the workflow is the sweep of the policy: one row per operating system, table and shard, every row blocking, the devcontainer on the mount-bound tables",()=>{
 const rows=build.jobs.suites!.strategy!.matrix.include,expected=["linux","macos","windows"].flatMap(name=>CI_SUITE_POLICY_V1.tables.flatMap(table=>Array.from({length:table.shards},(_,index)=>({name,table:table.id,index:index+1,count:table.shards}))));
 expect(rows.map(row=>({name:row.name,table:row.table,index:row.index,count:row.count}))).toEqual(expected);
 expect(Object.fromEntries(["linux","macos","windows"].map(name=>[name,rows.filter(row=>row.name===name).map(row=>row.os)]))).toEqual({linux:Array(7).fill(RUNNERS.linux),macos:Array(7).fill(RUNNERS.macos),windows:Array(7).fill(RUNNERS.windows)});
 for(const row of rows)expect(Object.keys(row).sort(),`${row.name} ${row.table} ${row.index} has no advisory flag`).toEqual(["count","index","name","os","table"]);
 expect(build.jobs.suites!["continue-on-error"],"the sweep is blocking on every operating system").toBeUndefined();
 expect(build.jobs.suites!.needs).toEqual(["smoke"]);
 const container=build.jobs["devcontainer-suites"]!.strategy!.matrix.include;
 expect(container.map(row=>({table:row.table,index:row.index,count:row.count}))).toEqual(["process","caching"].flatMap(id=>{const table=CI_SUITE_POLICY_V1.tables.find(item=>item.id===id)!;return Array.from({length:table.shards},(_,index)=>({table:id,index:index+1,count:table.shards}));}));
 for(const name of ["host","devcontainer","devcontainer-suites","suites"])expect(build.jobs[name]!["timeout-minutes"],name).toBeGreaterThan(0);
 for(const name of ["host","devcontainer"]){
  const text=JSON.stringify(build.jobs[name]);
  expect(text,name).toContain("--must-run");expect(build.jobs[name]!["continue-on-error"],`${name} is blocking`).toBeUndefined();
 }
 expect(JSON.stringify(build.jobs.host)).toContain("repo tools setup all");
});

test("the sweep steps run the sweep script through the neutral entry, keep the Markdown report in the job summary and upload logs even when the sweep failed",()=>{
 for(const name of ["host","suites"]){
  const job=build.jobs[name]!,run=job.steps.find(step=>step.run?.includes('"$CI_SUITES" run'));
  expect(run,name).toBeDefined();
  expect(run!.run,name).toMatch(/repo "\$CI_SUITES" run --(must-run|shard "\$SHARD") --tool "\$BUILD_TOOL" --out "\$RUNNER_TEMP\/[a-z-]+" \| tee -a "\$GITHUB_STEP_SUMMARY"/u);
  const upload=job.steps.find(step=>step.uses?.startsWith("actions/upload-artifact@"));
  expect(upload,name).toBeDefined();expect(upload!.if).toBe("${{ !cancelled() }}");
 }
 for(const name of ["devcontainer","devcontainer-suites"]){
  const job=build.jobs[name]!,use=job.steps.find(step=>step.uses==="./.github/actions/devcontainer-suites");
  expect(use,name).toBeDefined();expect(use!.with!.script).toBe("${{ env.CI_SUITES }}");expect(String(use!.with!.selection)).toMatch(/^--(must-run|shard \$\{\{ matrix\.table \}\}:\$\{\{ matrix\.index \}\}\/\$\{\{ matrix\.count \}\})$/u);
 }
 const action=actions[join(root,".github","actions","devcontainer-suites","action.yml")]!,text=JSON.stringify(action);
 expect(text).toContain("repo tools setup all");expect(text).toContain("Suite sweep passed in the devcontainer");expect(text).toContain("repo preflight");
});

test("the artifact names of every expanded job are unique and free of the characters artifact storage refuses",()=>{
 const names:string[]=[];
 for(const [id,job] of Object.entries(build.jobs)){
  const rows=job.strategy?.matrix.include??[{}];
  for(const upload of job.steps.filter(step=>step.uses?.startsWith("actions/upload-artifact@"))){
   for(const row of rows)names.push(`${id}/`+String(upload.with!.name).replace(/\$\{\{\s*matrix\.([\w-]+)\s*\}\}/gu,(_,key:string)=>String((row as Record<string,unknown>)[key])));
  }
 }
 expect(names.length).toBeGreaterThan(20);
 const bare=names.map(name=>name.split("/")[1]!);
 expect(new Set(bare).size,bare.filter((name,index)=>bare.indexOf(name)!==index).join(", ")).toBe(bare.length);
 for(const name of bare)expect(/^[A-Za-z0-9._-]+$/u.test(name),name).toBe(true);
});

const RUNNERS={linux:"ubuntu-24.04",macos:"macos-26",windows:"windows-2025"} as const;
const DEFAULT_BRANCH="⛳wip";
const PINNED_RUNNER=/^(?:ubuntu-\d{2}\.\d{2}|macos-\d{2}|windows-\d{4})$/u;
const PINNED_USES=/^[\w.-]+\/[\w./-]+@[0-9a-f]{40}$/u;
const workflowNamed=(name:string):Workflow=>workflows[join(root,".github","workflows",name)]!;
const literalsOf=(label:string):string[]=>label.includes("${{")?[...label.matchAll(/'([^']+)'/gu)].map(match=>match[1]!):[label];
const runnerLabels=(workflow:Workflow):string[]=>Object.values(workflow.jobs).flatMap(job=>[...literalsOf(job["runs-on"]),...(job.strategy?.matrix.include??[]).flatMap(row=>typeof row.os==="string"?literalsOf(row.os):[])]);
const minutesOf=(job:Job):number[]=>{
 const value=job["timeout-minutes"],reference=typeof value==="string"?/^\$\{\{\s*matrix\.([\w-]+)\s*\}\}$/u.exec(value):null;
 if(typeof value==="number")return [value];
 if(reference!==null&&job.strategy!==undefined)return job.strategy.matrix.include.map(row=>Number(row[reference[1]!]));
 return [Number.NaN];
};
const allSteps=():Step[]=>[...Object.values(workflows).flatMap(workflow=>Object.values(workflow.jobs).flatMap(job=>job.steps)),...Object.values(actions).flatMap(action=>action.runs.steps)];

test("every runner image is pinned to a version: no -latest alias, so a migration of the alias is a reviewed change",()=>{
 const labels=Object.values(workflows).flatMap(runnerLabels);
 expect(labels.length).toBeGreaterThan(30);
 expect(labels.filter(label=>!PINNED_RUNNER.test(label))).toEqual([]);
 expect(new Set(labels)).toEqual(new Set(Object.values(RUNNERS)));
 for(const alias of ["ubuntu-latest","macos-latest","windows-latest","macos-latest-large","ubuntu"])expect(PINNED_RUNNER.test(alias),alias).toBe(false);
 for(const label of Object.values(RUNNERS))expect(PINNED_RUNNER.test(label),label).toBe(true);
 expect(literalsOf("${{ matrix.large && vars.LARGE || 'ubuntu-24.04' }}")).toEqual(["ubuntu-24.04"]);expect(literalsOf("${{ matrix.os }}")).toEqual([]);
});

test("every external action is pinned to a full commit SHA, in workflows and in composite actions, and every local action a step names exists",()=>{
 const uses=allSteps().flatMap(step=>step.uses===undefined?[]:[step.uses]);
 expect(uses.length).toBeGreaterThan(40);
 for(const value of uses){
  expect(value.startsWith("./.github/actions/")||PINNED_USES.test(value),value).toBe(true);
  if(value.startsWith("./"))expect(actions[join(root,value.slice(2),"action.yml")],value).toBeDefined();
 }
 for(const unpinned of ["actions/checkout@v7","actions/checkout@v7.0.1","actions/checkout@main","docker://alpine:3"])expect(PINNED_USES.test(unpinned),unpinned).toBe(false);
});

test("a push to the default branch starts every workflow that gates a landing, and the manual dispatch compares against the default branch by default",()=>{
 for(const name of ["build-system.yml","dashboard.yml","go.yml","products.yml"]){
  const workflow=workflowNamed(name);
  expect(workflow,name).toBeDefined();
  expect((workflow.on.push as {branches?:string[]}|undefined)?.branches,`${name} push branches`).toContain(DEFAULT_BRANCH);
  expect(workflow.on,name).toHaveProperty("pull_request");expect(workflow.on,name).toHaveProperty("workflow_dispatch");
 }
 expect((workflowNamed("architecture-course.yml").on as Record<string,unknown>).push,"the release stays manual").toBeUndefined();
 const base=(build.on.workflow_dispatch as {inputs:{base:{default:string}}}).inputs.base;
 expect(base.default).toBe("");
 const plan=build.jobs.plan!.steps.find(step=>step.id==="range")!;
 expect(String(plan.env!.DEFAULT_BRANCH)).toBe("${{ github.event.repository.default_branch }}");expect(plan.run).toContain("origin/$DEFAULT_BRANCH");
});

test("every job is bounded below the hosted maximum, the sweep jobs fit the shard budget of the policy and every provisioning step has its own limit",()=>{
 let jobs=0;
 for(const [path,workflow] of Object.entries(workflows))for(const [id,job] of Object.entries(workflow.jobs)){
  jobs++;
  for(const minutes of minutesOf(job))expect(Number.isInteger(minutes)&&minutes>=5&&minutes<=360,`${path} ${id}: ${minutes}`).toBe(true);
 }
 expect(jobs).toBeGreaterThan(18);
 const lower=CI_SUITE_POLICY_V1.maximumShardBudgetMinutes+CI_SUITE_POLICY_V1.suiteSlackMinutes+20;
 for(const name of ["suites","devcontainer-suites"])for(const minutes of minutesOf(build.jobs[name]!)){
  expect(minutes,`${name} leaves room for provisioning`).toBeGreaterThanOrEqual(lower);
  expect(minutes,`${name} stays far below the hosted maximum`).toBeLessThanOrEqual(lower+90);
 }
 for(const [path,workflow] of Object.entries(workflows))for(const [id,job] of Object.entries(workflow.jobs))for(const step of job.steps)if(/^repo (?:tools setup|dependencies sync)\b/mu.test(step.run??""))expect(step["timeout-minutes"],`${path} ${id}: ${step.name}`).toBeLessThanOrEqual(60);
 expect(minutesOf({"runs-on":"x",steps:[{run:"x"}]})).toEqual([Number.NaN]);expect(minutesOf({"runs-on":"x","timeout-minutes":"${{ matrix.minutes }}",strategy:{matrix:{include:[{minutes:90},{minutes:120}]}},steps:[{run:"x"}]})).toEqual([90,120]);
});

test("the launcher laws run on Linux, macOS and Windows, the Windows evidence never fails the job, and every Go project is reached by its official route",()=>{
 const go=workflowNamed("go.yml"),launcher=go.jobs.launcher!;
 expect(launcher.strategy!.matrix.include.map(row=>row.os)).toEqual([RUNNERS.linux,RUNNERS.macos,RUNNERS.windows]);
 expect(launcher["continue-on-error"],"the launcher laws are blocking on every operating system").toBeUndefined();
 expect(launcher.steps.filter(step=>step.run?.includes("run @semio-tech/repo-launcher-go:test")).length).toBe(1);
 const evidence=launcher.steps.filter(step=>step.name?.startsWith("Windows evidence"));
 expect(evidence.length).toBeGreaterThanOrEqual(3);
 for(const step of evidence){expect(step["continue-on-error"],step.name).toBe(true);expect(String(step.if),step.name).toContain("runner.os == 'Windows'");expect(step.shell,step.name).toBe("pwsh");}
 const modules=go.jobs.modules!,rows=modules.strategy!.matrix.include;
 expect(modules["continue-on-error"]).toBe("${{ matrix.advisory }}");
 expect(rows.filter(row=>row.group==="services").every(row=>String(row.projects).startsWith("@semio-tech/repo-*-go,"))).toBe(true);
 expect(rows.filter(row=>row.group==="clients").every(row=>["@semio-tech/repo-client","repo-mcp","@semio-tech/repo-coordinator"].every(name=>String(row.projects).split(",").includes(name)))).toBe(true);
 expect(rows.map(row=>[row.name,row.advisory])).toEqual([["linux",false],["linux",false],["macos",false],["macos",false],["windows",true],["windows",true]]);
 expect(modules.steps.find(step=>step.run?.includes("run-many"))!.run).toContain('--targets=test');
 const products=workflowNamed("products.yml").jobs.products!;
 expect(products["continue-on-error"]).toBe("${{ matrix.advisory }}");
 expect(products.strategy!.matrix.include.filter(row=>row.name!=="linux").every(row=>row.advisory===true)).toBe(true);
 expect(products.strategy!.matrix.include.filter(row=>row.name==="linux").every(row=>row.advisory===false)).toBe(true);
});

test("a cache is saved only by a push or a manual run on the default branch and only when the exact key is not stored, so a pull request can neither evict nor poison it",()=>{
 const saves=allSteps().filter(step=>step.uses?.startsWith("actions/cache/save@"));
 expect(saves.length).toBeGreaterThanOrEqual(4);
 for(const step of saves)expect(/github\.event\.repository\.default_branch|steps\.promote\.outputs\.promoted/u.test(String(step.if)),step.name).toBe(true);
 const action=actions[join(root,".github","actions","tool-cache","action.yml")]!,text=JSON.stringify(action);
 expect(text).toContain("lookup-only");expect(text).toContain("steps.stored.outputs.cache-hit != 'true'");
 expect(action.runs.steps.filter(step=>step.uses?.startsWith("actions/cache/restore@")).length).toBe(2);
});

test("the artifact names of every workflow are unique within the workflow and free of the characters artifact storage refuses",()=>{
 for(const [path,workflow] of Object.entries(workflows)){
  const names:string[]=[];
  for(const [id,job] of Object.entries(workflow.jobs))for(const upload of job.steps.filter(step=>step.uses?.startsWith("actions/upload-artifact@")))for(const row of job.strategy?.matrix.include??[{}])names.push(`${id}/`+String(upload.with!.name).replace(/\$\{\{\s*matrix\.([\w-]+)\s*\}\}/gu,(_,key:string)=>String((row as Record<string,unknown>)[key])));
  const bare=names.map(name=>name.split("/")[1]!);
  expect(new Set(bare).size,`${path}: ${bare.filter((name,index)=>bare.indexOf(name)!==index).join(", ")}`).toBe(bare.length);
  for(const name of bare)expect(/^[A-Za-z0-9._-]+$/u.test(name),`${path}: ${name}`).toBe(true);
 }
});
