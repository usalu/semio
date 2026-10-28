
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
