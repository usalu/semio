/** 👁️ Bazel evidence: the directory contract, the flag, the closure record, and the execution log of the pinned Bazel as the oracle for the verifier's reader. */
import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {bazelEvidenceDirectoryV1,bazelEvidenceEventsPathV1,bazelEvidenceFlagsV1,writeBazelClosureEvidenceV1} from "../🟦️.ts";
import {groupParityBazelSpawnsV1,parseParityBazelEventsV1,parseParityBazelExecutionLogV1} from "../../../../🪞️parity/📥️evidence/🟦️.ts";

const task=(id:string,label:string,env?:Record<string,string>)=>({id,project:id.split(":")[0]!,target:id.split(":")[1]!,label,dependencies:[],args:[],root:".",definition:{execution:{kind:"command" as const,...(env?{env}:{})}}});

test("evidence is requested by one absolute directory and adds exactly one option",()=>{
 expect(bazelEvidenceDirectoryV1({})).toBeUndefined();
 expect(bazelEvidenceDirectoryV1({SEMIO_BUILD_EVIDENCE_DIR:""})).toBeUndefined();
 expect(bazelEvidenceDirectoryV1({SEMIO_BUILD_EVIDENCE_DIR:"/tmp/e"})).toBe("/tmp/e");
 expect(()=>bazelEvidenceDirectoryV1({SEMIO_BUILD_EVIDENCE_DIR:"relative/e"})).toThrow("absolute");
 expect(()=>bazelEvidenceDirectoryV1({SEMIO_BUILD_EVIDENCE_DIR:"/"+"a".repeat(201)})).toThrow("200");
 expect(bazelEvidenceFlagsV1(undefined)).toEqual([]);
 expect(bazelEvidenceFlagsV1("/tmp/e")).toEqual(["--execution_log_json_file=/tmp/e/bazel.execlog.jsonl"]);
 expect(bazelEvidenceEventsPathV1("/tmp/e")).toBe("/tmp/e/bazel.bep.jsonl");
});

test("the closure record maps labels to tasks, strips configuration suffixes and lists declared environment names",async()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"bzl-evidence-")));
 try{
  const plan={version:1 as const,tasks:{"p:build":task("p:build","t_a",{B:"1",A:"2"}),"q:gen":task("q:gen","t_b")},selected:["p:build"]};
  let asked="";
  await writeBazelClosureEvidenceV1(join(directory,"e"),{plan:plan as never,query:async expression=>{asked=expression;return "//:t_b (abc123)\n//:t_a (abc123)\n\nINFO: ignored\n";}});
  expect(asked).toBe('kind("repo_", deps(set(//:t_a)))');
  expect(JSON.parse(readFileSync(join(directory,"e/bazel.closure.json"),"utf8"))).toEqual({version:1,labelToTask:{"//:t_a":"p:build","//:t_b":"q:gen"},queried:["//:t_a","//:t_b"],declaredEnvironment:{"//:t_a":["A","B"],"//:t_b":[]}});
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,real=binary?test:test.skip;
real("the pinned Bazel writes the execution log and event stream in the shape the verifier reads",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"bzl-evlog-"))),output=join(directory,"ob"),startup=[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","--max_idle_secs=10","--host_jvm_args=-Xmx512m"];
 try{
  writeFileSync(join(directory,"MODULE.bazel"),'module(name = "evidence", version = "0.0.0")\n');
  writeFileSync(join(directory,"rules.bzl"),`def _task(ctx):
    out = ctx.actions.declare_file(ctx.label.name + ".out")
    ctx.actions.run_shell(inputs = ctx.files.srcs, outputs = [out], command = "cat $@ > " + out.path + "; echo $FOO >> " + out.path, arguments = [f.path for f in ctx.files.srcs], env = {"FOO": "bar"})
    return [DefaultInfo(files = depset([out]))]
repo_task = rule(implementation = _task, attrs = {"srcs": attr.label_list(allow_files = True)})
`);
  writeFileSync(join(directory,"a.txt"),"hello\n");
  writeFileSync(join(directory,"BUILD.bazel"),'load(":rules.bzl", "repo_task")\nexports_files(["a.txt"])\nrepo_task(name = "t_one", srcs = ["a.txt"])\n');
  mkdirSync(join(directory,"e"));
  const common=["--lockfile_mode=off","--incompatible_autoload_externally=","--spawn_strategy=local"],events=bazelEvidenceEventsPathV1(join(directory,"e"));
  const build=spawnSync(binary!,[...startup,"build",...common,...bazelEvidenceFlagsV1(join(directory,"e")),`--build_event_json_file=${events}`,"//:t_one"],{cwd:directory,encoding:"utf8"});
  expect(build.status,build.stderr).toBe(0);
  const spawns=parseParityBazelExecutionLogV1(readFileSync(join(directory,"e/bazel.execlog.jsonl"),"utf8")),row=spawns.find(entry=>entry.label==="//:t_one")!;
  expect(row.exit).toBe(0);expect(row.env).toContain("FOO");expect(row.inputs).toContain("a.txt");
  expect(groupParityBazelSpawnsV1(spawns,{"//:t_one":"p:t"})["p:t"]!.env).toContain("FOO");
  const summary=parseParityBazelEventsV1(readFileSync(events,"utf8"));
  expect(summary.exit).toBe(0);expect(summary.targets["//:t_one"]).toBe(true);
  const query=spawnSync(binary!,[...startup,"cquery",...common,"--output=label",'kind("repo_", deps(set(//:t_one)))'],{cwd:directory,encoding:"utf8"});
  expect(query.status,query.stderr).toBe(0);
  expect(query.stdout.split("\n").map(line=>line.replace(/\s+\(.*\)\s*$/u,"").trim()).filter(line=>line.startsWith("//"))).toEqual(["//:t_one"]);
  expect(existsSync(join(directory,"e/bazel.execlog.jsonl"))).toBe(true);
 }finally{
  spawnSync(binary!,[...startup,"shutdown"],{cwd:directory});
  spawnSync("chmod",["-R","u+w",directory]);
  rmSync(directory,{recursive:true,force:true});
 }
},600000);
