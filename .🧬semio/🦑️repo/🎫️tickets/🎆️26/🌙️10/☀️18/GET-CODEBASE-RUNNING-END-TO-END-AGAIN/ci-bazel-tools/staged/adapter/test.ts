import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {cpSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {pathToFileURL} from "node:url";
import schema from "../../../../🧾️ci-report/🧬️schema/🔣️.json" with {type:"json"};
import {readCiTestResultsIndexV1,summarizeCiTestsV1} from "../../../../🧾️ci-report/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {bazelTestResultFileV1,bazelTestResultsDirectoryV1,bazelTestResultsFromEventsV1,exportBazelTestResultsV1} from "../🟦️.ts";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/TestResultsIndex"});
const scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`bazel-test-results-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

type Stream=(typeof fixture.streams)[number];
const rootOf=(root:string)=>pathToFileURL(root).pathname.replace(/\/$/u,"");
const withRoot=(stream:Stream,root:string)=>stream.events.replaceAll(fixture.rootToken,rootOf(root));
function populate(stream:Stream):string{
 const root=directory("root");
 for(const [relative,kind] of Object.entries(stream.files)){
  const path=join(root,relative);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,fixture.xml[kind as "pass"|"fail"]);
 }
 return root;
}

test("the final attempt of every label, run and shard is read with neutral statuses, cache flags and the local XML path",()=>{
 for(const stream of fixture.streams){
  const root=populate(stream),rows=bazelTestResultsFromEventsV1(withRoot(stream,root));
  expect(rows.map(row=>({label:row.label,run:row.run,shard:row.shard,attempt:row.attempt,status:row.status,cached:row.cached,durationMillis:row.durationMillis,xml:row.xml===undefined?null:row.xml.slice(root.length+1).split("\\").join("/")})).sort((a,b)=>(a.label+a.shard<b.label+b.shard?-1:1)),stream.name).toEqual(stream.expected.map(row=>({label:row.label,run:row.run,shard:row.shard,attempt:row.attempt,status:row.status,cached:row.cached,durationMillis:row.durationMillis,xml:row.xml})).sort((a,b)=>(a.label+a.shard<b.label+b.shard?-1:1)) as never);
 }
});

test("exported file names are ASCII, unique per label, run and shard, and equal the independent SHA-256 naming",()=>{
 const names=new Set<string>();
 for(const stream of fixture.streams)for(const row of stream.expected){
  const multiple=stream.expected.filter(other=>other.label===row.label).length>1;
  expect(bazelTestResultFileV1(row,multiple),`${stream.name} ${row.label}`).toBe(row.file);
  expect(row.file).toMatch(/^[A-Za-z0-9._-]+$/u);
  if(stream.name==="synthetic-shards-remote-missing-unicode")names.add(row.file);
 }
 expect(names.size).toBe(7);
});

test("an export copies the XML of every available result and writes a closed, sorted index that the neutral report reads",async()=>{
 for(const stream of fixture.streams){
  const root=populate(stream),target=join(directory("target"),"results"),exports=stream.expected.filter(row=>row.xml!==null&&existsFile(root,row.xml));
  const outcome=await exportBazelTestResultsV1(withRoot(stream,root),target);
  expect(outcome,stream.name).toEqual({exported:exports.length,unavailable:stream.expected.length-exports.length});
  const index=JSON.parse(readFileSync(join(target,"index.json"),"utf8"));
  expect(validate(index),ajv.errorsText(validate.errors)).toBe(true);
  expect(index.results,stream.name).toEqual(exports.map(row=>({label:row.label,status:row.status,cached:row.cached,durationMillis:row.durationMillis,file:row.file})).sort((a,b)=>(a.file<b.file?-1:1)));
  for(const row of exports)expect(readFileSync(join(target,row.file),"utf8")).toBe(readFileSync(join(root,row.xml!),"utf8"));
  const files=index.results.map((row:{file:string})=>({file:row.file,xml:readFileSync(join(target,row.file),"utf8")}));
  const summary=summarizeCiTestsV1(files,readCiTestResultsIndexV1(JSON.stringify(index)));
  expect(summary.files).toBe(exports.length);
  expect(summary.cached).toBe(exports.filter(row=>row.cached).length);
  expect(summary.failed,stream.name).toEqual(exports.filter(row=>row.status==="failed"||row.status==="timeout").map(row=>row.label).sort());
 }
});
const existsFile=(root:string,relative:string)=>existsSync(join(root,relative));

test("several builds of one run add up in one index and a repeated result replaces its row",async()=>{
 const target=join(directory("merge"),"results"),first=fixture.streams[0]!,second=fixture.streams[1]!,third=fixture.streams[3]!;
 const rootA=populate(first),rootB=populate(second),rootC=populate(third);
 await exportBazelTestResultsV1(withRoot(first,rootA),target);
 await exportBazelTestResultsV1(withRoot(third,rootC),target);
 const afterTwo=JSON.parse(readFileSync(join(target,"index.json"),"utf8")).results.length;
 await exportBazelTestResultsV1(withRoot(second,rootB),target);
 const index=JSON.parse(readFileSync(join(target,"index.json"),"utf8"));
 expect(index.results.length).toBe(afterTwo);
 expect(index.results.filter((row:{label:string})=>row.label==="//:pass_one")[0].cached).toBe(true);
 expect(validate(index)).toBe(true);
});

test("a corrupt index is replaced, a missing directory is created, and an unusable directory never throws",async()=>{
 const stream=fixture.streams[0]!,root=populate(stream),target=join(directory("corrupt"),"a","b");
 mkdirSync(target,{recursive:true});writeFileSync(join(target,"index.json"),"{ not json");
 expect((await exportBazelTestResultsV1(withRoot(stream,root),target)).exported).toBe(3);
 const blocker=join(directory("blocked"),"file");writeFileSync(blocker,"x");
 const warnings:string[]=[],original=console.warn;console.warn=(...args:unknown[])=>{warnings.push(args.join(" "));};
 try{expect(await exportBazelTestResultsV1(withRoot(stream,root),join(blocker,"inside"))).toEqual({exported:0,unavailable:0});}finally{console.warn=original;}
 expect(warnings.join("\n")).toContain("[bazel] test results not exported");
 expect(await exportBazelTestResultsV1("not json\n{}\n",join(directory("none"),"x"))).toEqual({exported:0,unavailable:0});
});

test("the results directory comes from SEMIO_BUILD_TEST_RESULTS_DIR and must be absolute and short",()=>{
 expect(bazelTestResultsDirectoryV1({})).toBeUndefined();
 expect(bazelTestResultsDirectoryV1({SEMIO_BUILD_TEST_RESULTS_DIR:""})).toBeUndefined();
 const absolute=join(tmpdir(),"results");
 expect(bazelTestResultsDirectoryV1({SEMIO_BUILD_TEST_RESULTS_DIR:absolute})).toBe(absolute);
 expect(()=>bazelTestResultsDirectoryV1({SEMIO_BUILD_TEST_RESULTS_DIR:"relative/results"})).toThrow("absolute path");
 expect(()=>bazelTestResultsDirectoryV1({SEMIO_BUILD_TEST_RESULTS_DIR:join(tmpdir(),"x".repeat(201))})).toThrow("at most 200");
});

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,catalog=process.env.SEMIO_TEST_BAZEL_CATALOG;
test.skipIf(binary===undefined||catalog===undefined)("real Bazel: tests run by the pinned binary export their own test.xml",async()=>{
 const base=directory("bazel"),workspace=join(base,"ws");mkdirSync(workspace,{recursive:true});
 cpSync(join(catalog!,"workspace","MODULE.bazel.lock"),join(workspace,"MODULE.bazel.lock"));cpSync(join(catalog!,"repositories"),join(base,"repositories"),{recursive:true});
 writeFileSync(join(workspace,"MODULE.bazel"),'module(name = "semio_tasks", version = "0.0.0")\n');
 writeFileSync(join(workspace,"t.bzl"),'def _impl(ctx):\n    out = ctx.actions.declare_file(ctx.label.name + ".sh")\n    ctx.actions.write(out, "#!/bin/sh\\n" + ctx.attr.script + "\\n", is_executable = True)\n    return [DefaultInfo(executable = out)]\n\nt_test = rule(implementation = _impl, test = True, attrs = {"script": attr.string()})\n');
 writeFileSync(join(workspace,"BUILD.bazel"),'load(":t.bzl", "t_test")\nt_test(name = "pass_one", script = "echo ok")\nt_test(name = "fail_one", script = "echo no; exit 3")\n');
 const events=join(base,"events.json"),bazel=(...args:string[])=>spawnSync(binary!,["--nosystem_rc","--nohome_rc","--noworkspace_rc","--bazelrc=/dev/null",`--output_base=${join(base,"ob")}`,`--install_base=${join(base,"install")}`,"--max_idle_secs=15","--host_jvm_args=-Xmx1024m",...args],{cwd:workspace,encoding:"utf8",timeout:280000});
 const run=bazel("test",`--build_event_json_file=${events}`,"--keep_going",`--repository_cache=${join(base,"repositories")}`,"--repository_disable_download","--lockfile_mode=error","--incompatible_autoload_externally=","--experimental_convenience_symlinks=ignore","--color=no","--curses=no","//:pass_one","//:fail_one");
 bazel("shutdown");
 expect(run.status,run.stderr.slice(-800)).toBe(3);
 const target=join(base,"out");
 expect(await exportBazelTestResultsV1(readFileSync(events,"utf8"),target)).toEqual({exported:2,unavailable:0});
 const index=JSON.parse(readFileSync(join(target,"index.json"),"utf8"));
 expect(index.results.map((row:{label:string;status:string})=>`${row.label}=${row.status}`).sort()).toEqual(["//:fail_one=failed","//:pass_one=passed"]);
 const summary=summarizeCiTestsV1(index.results.map((row:{file:string})=>({file:row.file,xml:readFileSync(join(target,row.file),"utf8")})),readCiTestResultsIndexV1(JSON.stringify(index)));
 expect(summary).toMatchObject({files:2,tests:2,errors:1,failed:["//:fail_one"]});
},300000);
