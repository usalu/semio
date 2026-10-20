/** 🧪️ Bazel test actions: kind classification inside the real planner, the rendered rule, and `bazel test` against the generated rule with a recording runner. */
import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync,appendFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {bazelActionsV1,bazelStarlarkRulesV1,planBazelTasksV1,renderBazelBuildV1} from "../run/adapter.ts";

const graphOf=(targets:Record<string,Record<string,unknown>>,projects=1)=>({version:1 as const,projects:Object.fromEntries(Array.from({length:projects},(_v,index)=>[`p${index}`,{name:`p${index}`,root:`p${index}`,targets:structuredClone(targets)}])),dependencies:[],namedInputs:{}}) as never;
const kinds=(targets:Record<string,Record<string,unknown>>,select:string[])=>{
 const graph=graphOf(targets),plan=planBazelTasksV1(graph,{tasks:select}),emitted=bazelActionsV1(graph,plan,false);
 return Object.fromEntries([...emitted.values()].map(action=>[action.id,action.kind]));
};
const command={execution:{kind:"command",command:"true"}};

test("the planner emits a Bazel test only for a cached test-family task with empty outputs and no dependents",()=>{
 expect(kinds({test:{...command,cache:true,outputs:[]}},["p0:test"])).toEqual({"p0:test":"test"});
 expect(kinds({"test-unit":{...command,cache:true,outputs:[]}},["p0:test-unit"])).toEqual({"p0:test-unit":"test"});
 expect(kinds({test:{...command,cache:true,outputs:["{projectRoot}/coverage"]}},["p0:test"])).toEqual({"p0:test":"task"});
 expect(kinds({test:{...command,cache:true}},["p0:test"])).toEqual({"p0:test":"task"});
 expect(kinds({test:{...command,cache:false,outputs:[]}},["p0:test"])).toEqual({"p0:test":"task"});
 expect(kinds({build:{...command,cache:true,outputs:[]}},["p0:build"])).toEqual({"p0:build":"task"});
 expect(kinds({check:{...command,cache:true,outputs:[],dependsOn:["test"]},test:{...command,cache:true,outputs:[]}},["p0:check"])).toEqual({"p0:check":"task","p0:test":"task"});
 expect(kinds({test:{...command,cache:true,outputs:[]},build:{...command,cache:true,outputs:[]}},["p0:test","p0:build"])).toEqual({"p0:test":"test","p0:build":"task"});
});

const sources=["a.txt","b.txt"],actionRows=(count:number)=>Array.from({length:count},(_v,index)=>({id:`p:t${index}`,label:`t_${index.toString(16).padStart(16,"0")}`,kind:index===0?"task" as const:"test" as const,restore:index===0?[]:[`t_${(0).toString(16).padStart(16,"0")}`],cache:{cache:true},sources:`i_${index}`}));
const render=(count:number,files:readonly string[]=sources)=>renderBazelBuildV1({actions:actionRows(count),groups:new Map(files.slice(0,count).map((file,index)=>[`i_${index}`,{groups:[],files:[file]}])),bun:"/bin/sh",sourceIdentity:"a".repeat(64),extension:""});

test("the rendered BUILD declares tests with the eternal timeout and parses as Starlark data",()=>{
 const build=render(2);
 expect(build).toContain('load(":tasks.bzl", "repo_launch", "repo_noop", "repo_task", "repo_test")');
 expect(build).toContain('repo_test(\n    name = "t_0000000000000001",\n    timeout = "eternal",');
 expect(build).toContain('repo_task(\n    name = "t_0000000000000000",');
 const run=spawnSync("python3",["-I","-c","import ast,sys;ast.parse(sys.stdin.read())"],{input:build.replace(/^load\(.*$/mu,""),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(bazelStarlarkRulesV1()).toContain("repo_test = rule(implementation = _test_impl, test = True");
});

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,real=binary?test:test.skip;
real("bazel test runs the generated rule with the exact birth paths, caches its result and reruns on a changed source",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"bzl-test-rule-"))),workspace=join(directory,"ws"),checkout=join(directory,"checkout"),output=join(directory,"ob"),disk=join(directory,"disk"),log=join(directory,"runs.log");
 mkdirSync(workspace);mkdirSync(checkout);mkdirSync(disk);
 try{
  for(const name of sources)writeFileSync(join(checkout,name),"content "+name);
  symlinkSync(checkout,join(workspace,"s"));
  writeFileSync(join(workspace,"MODULE.bazel"),'module(name = "semio_tasks", version = "0.0.0")\n');
  writeFileSync(join(workspace,"tasks.bzl"),bazelStarlarkRulesV1());
  writeFileSync(join(workspace,"BUILD.bazel"),render(2));
  writeFileSync(join(workspace,"implementation.json"),"{}");
  writeFileSync(join(workspace,"📜️script.ts"),`#!/bin/sh\nprintf '%s\\n' "$*" >> "${log}"\ncase "$3" in birth-action) ;; *) exit 9;; esac\nmkdir -p "$5" || exit 8\necho '{}' > "$5/snapshot.json"\n[ -e "${directory}/fail" ] && exit 1\nexit 0\n`);
  for(const index of [0,1]){writeFileSync(join(workspace,`t_${index.toString(16).padStart(16,"0")}.json`),"{}");writeFileSync(join(workspace,`t_${index.toString(16).padStart(16,"0")}.input`),"x");}
  const bazel=(verb:string,args:string[],extra:NodeJS.ProcessEnv={})=>spawnSync(binary!,[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","--max_idle_secs=30",verb,"--lockfile_mode=off","--incompatible_autoload_externally=","--incompatible_strict_action_env",`--disk_cache=${disk}`,...args],{cwd:workspace,encoding:"utf8",env:{...process.env,LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8",...extra}});
  const label="//:t_0000000000000001",flags=["--build_tests_only","--test_output=errors","--cache_test_results=auto"];
  const binPath=bazel("info",["bazel-bin"]).stdout.trim().split("\n").at(-1)!;
  const first=bazel("test",[...flags,label]);
  expect(first.status,first.stdout+first.stderr).toBe(0);
  expect(/PASSED/u.test(first.stdout+first.stderr)).toBe(true);
  const lines=readFileSync(log,"utf8").trim().split("\n");
  expect(lines.length).toBe(2);
  const parameters=lines.at(-1)!.split(" ");
  expect(parameters[1]).toBe("a".repeat(64));
  expect(parameters[2]).toBe("birth-action");
  expect(parameters[4]).toBe(join(binPath,"t_0000000000000001.snapshot"));
  expect(parameters[5]).toBe(join(binPath,"t_0000000000000000.snapshot"));
  expect(parameters.length).toBe(6);
  const second=bazel("test",[...flags,label]);
  expect(second.status).toBe(0);
  expect(/\(cached\) PASSED/u.test(second.stdout+second.stderr)).toBe(true);
  expect(readFileSync(log,"utf8").trim().split("\n").length).toBe(2);
  const forced=bazel("test",[...flags.filter(flag=>!flag.startsWith("--cache_test")),"--nocache_test_results",label]);
  expect(forced.status).toBe(0);
  expect(readFileSync(log,"utf8").trim().split("\n").length).toBe(3);
  appendFileSync(join(checkout,"a.txt"),"changed");
  const dependency=bazel("test",[...flags,label]);
  expect(dependency.status).toBe(0);
  const afterDependency=readFileSync(log,"utf8").trim().split("\n").length;
  expect(afterDependency).toBe(4);
  appendFileSync(join(checkout,"b.txt"),"changed");
  const changed=bazel("test",[...flags,label]);
  expect(changed.status).toBe(0);
  expect(/\(cached\)/u.test(changed.stdout+changed.stderr)).toBe(false);
  expect(readFileSync(log,"utf8").trim().split("\n").length).toBe(5);
  bazel("clean",[]);
  const restored=bazel("test",[...flags,label]);
  expect(restored.status).toBe(0);
  expect(readFileSync(log,"utf8").trim().split("\n").length).toBe(5);
  expect(/PASSED/u.test(restored.stdout+restored.stderr)).toBe(true);
  appendFileSync(join(checkout,"b.txt"),"changed");
  writeFileSync(join(directory,"fail"),"");
  const failing=bazel("test",[...flags,label]);
  expect(failing.status).toBe(3);
  expect(/FAILED/u.test(failing.stdout+failing.stderr)).toBe(true);
 }finally{
  spawnSync(binary!,[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","shutdown"],{cwd:workspace});
  spawnSync("chmod",["-R","u+w",directory]);
  rmSync(directory,{recursive:true,force:true});
 }
},600000);
