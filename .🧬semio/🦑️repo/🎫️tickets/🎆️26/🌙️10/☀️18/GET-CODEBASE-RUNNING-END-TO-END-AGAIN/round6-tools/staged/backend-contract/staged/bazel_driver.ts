import {chmodSync,copyFileSync,existsSync,mkdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {randomUUID} from "node:crypto";
import {dirname,join} from "node:path";
import {conformanceClosureV1,type ConformanceWorkspaceV1} from "../../../📐️conformance/🟦️.ts";
import {conformanceSpawnV1,type ConformanceDriverV1,type ConformancePrepareV1,type ConformanceSessionV1} from "../../../📐️conformance/🏃️runner/🟦️.ts";
import {incrementalLabelV1} from "../../../📏️incremental/🟦️.ts";
import {incrementalBazelObservationV1,incrementalBazelSpawnsV1} from "../../../📏️incremental/👁️observers/🟦️.ts";

const RULES=`def _impl(ctx):
    outs = [ctx.actions.declare_file(path) for path in ctx.attr.outs]
    stamp = None
    if not outs:
        stamp = ctx.actions.declare_file(".stamp/" + ctx.label.name)
        outs = [stamp]
    inputs = ctx.files.srcs + [file for dep in ctx.attr.deps for file in dep[DefaultInfo].files.to_list()]
    ctx.actions.run(
        executable = ctx.attr.runtime,
        arguments = [ctx.attr.stub, ctx.attr.spec, ctx.attr.task_id, ctx.bin_dir.path, stamp.path if stamp else "", ctx.attr.nonce],
        inputs = inputs,
        outputs = outs,
        mnemonic = "ConformanceTask",
        execution_requirements = {"no-sandbox": "1"},
        use_default_shell_env = True,
    )
    return [DefaultInfo(files = depset(outs))]

conformance_task = rule(
    implementation = _impl,
    attrs = {
        "srcs": attr.label_list(allow_files = True),
        "deps": attr.label_list(),
        "outs": attr.string_list(),
        "task_id": attr.string(),
        "stub": attr.string(),
        "spec": attr.string(),
        "runtime": attr.string(),
        "nonce": attr.string(),
    },
)
`;

const quote=(value:string)=>JSON.stringify(value);

/** 🏗️ Renders the BUILD file of the workspace: one rule per stub task; a task that must not be cached carries a fresh nonce, which changes only its own action key. */
export function conformanceBazelBuildV1(workspace:ConformanceWorkspaceV1,input:Readonly<{runtime:string;stub:string;spec:string}>,nonce:()=>string):string{
 const lines=['load(":defs.bzl", "conformance_task")',""];
 for(const project of workspace.projects)for(const[target,row]of Object.entries(project.targets)){
  const id=`${project.name}:${target}`;
  lines.push(`conformance_task(`,`    name = ${quote(incrementalLabelV1("t",id))},`,`    task_id = ${quote(id)},`,`    srcs = glob([${row.reads.map(quote).join(", ")}]),`,`    deps = [${(row.dependsOn??[]).map(dependency=>quote(":"+incrementalLabelV1("t",dependency))).join(", ")}],`,`    outs = [${row.outputs.map(quote).join(", ")}],`,`    runtime = ${quote(input.runtime)},`,`    stub = ${quote(input.stub)},`,`    spec = ${quote(input.spec)},`,`    nonce = ${quote(row.cache?"":nonce())},`,`)`,"");
 }
 return lines.join("\n");
}

/** 🔌️ Binds Bazel to the conformance suite by running a real Bazel in a generated workspace: every stub task becomes a rule instance whose action runs the stub with outputs under bazel-bin, which the binding copies into the workspace the way the repository adapter restores snapshots. `SEMIO_TEST_BAZEL_BINARY` names the executable. */
export const driver:ConformanceDriverV1=Object.freeze({
 id:"bazel",
 async prepare(input:ConformancePrepareV1):Promise<ConformanceSessionV1>{
  const bazel=process.env.SEMIO_TEST_BAZEL_BINARY;
  if(!bazel||!existsSync(bazel))throw Error("SEMIO_TEST_BAZEL_BINARY must name a Bazel executable");
  const base=join(input.cache,"bazel"),outputBase=join(base,"ob"),disk=join(base,"disk"),log=join(base,"exec.zst"),ids=input.workspace.projects.flatMap(project=>Object.keys(project.targets).map(target=>`${project.name}:${target}`));
  const label=(id:string)=>incrementalLabelV1("t",id),params={runtime:input.runtime,stub:input.stub,spec:input.specPath};
  const uncached=new Set(input.workspace.projects.flatMap(project=>Object.entries(project.targets).filter(([,row])=>!row.cache).map(([target])=>`${project.name}:${target}`)));
  const outputs=(id:string)=>input.spec.tasks[id]!.outputs;
  const writeBuild=()=>writeFileSync(join(input.directory,"BUILD.bazel"),conformanceBazelBuildV1(input.workspace,params,randomUUID));
  writeFileSync(join(input.directory,"MODULE.bazel"),'module(name = "conformance", version = "0.0.0")\n');
  writeFileSync(join(input.directory,"defs.bzl"),RULES);writeBuild();
  const startup=["--output_base="+outputBase],bazelEnvironment=()=>({...process.env});
  const bazelRun=(arguments_:readonly string[],signal:AbortSignal)=>conformanceSpawnV1(bazel,[...startup,...arguments_],{cwd:input.directory,env:bazelEnvironment(),signal});
  const owner=new Map(ids.map(id=>[`//:${label(id)}`,id]));
  return {
   observes:new Set(["run-record"]),
   async plan(target,signal){
    const result=await bazelRun(["query",`deps(//:${label(target)})`,"--noimplicit_deps","--output=graph"],signal);
    if(result.exitCode!==0)throw Error("bazel could not query the plan: "+result.output.slice(-400));
    const tasks=new Set<string>(),dependencies:Record<string,string[]>={};
    for(const line of result.output.split("\n")){
     const edge=/^\s*"([^"]+)" -> "([^"]+)"/u.exec(line),node=/^\s*"([^"]+)"\s*$/u.exec(line),from=edge?owner.get(edge[1]!):undefined,to=edge?owner.get(edge[2]!):undefined;
     if(from&&to){tasks.add(from);tasks.add(to);(dependencies[from]??=[]).push(to);}
     else if(node&&owner.has(node[1]!))tasks.add(owner.get(node[1]!)!);
    }
    for(const id of tasks)dependencies[id]??=[];
    tasks.add(target);dependencies[target]??=[];
    return {tasks:[...tasks],dependencies};
   },
   async run(targets,options,signal){
    const started=Date.now();
    if(conformanceClosureV1(input.workspace,targets).some(id=>uncached.has(id)))writeBuild();
    rmSync(log,{force:true});mkdirSync(base,{recursive:true});
    const remote=options.remote==="read-write"&&input.remote?[`--remote_cache=${input.remote.url}`,"--remote_upload_local_results=true"]:["--remote_cache="];
    const result=await bazelRun(["build","--keep_going","--noshow_progress","--color=no","--spawn_strategy=local",`--disk_cache=${disk}`,`--execution_log_compact_file=${log}`,...(options.parallel?[`--jobs=${options.parallel}`]:[]),...remote,...targets.map(id=>`//:${label(id)}`)],signal);
    for(const id of conformanceClosureV1(input.workspace,targets))for(const path of outputs(id)){const from=join(input.directory,"bazel-bin",path);if(existsSync(from)){const to=join(input.directory,path);mkdirSync(dirname(to),{recursive:true});rmSync(to,{force:true});copyFileSync(from,to);chmodSync(to,0o644);}}
    let restored:string[]|undefined;
    try{const seen=incrementalBazelObservationV1(incrementalBazelSpawnsV1(readFileSync(log)),ids),closure=new Set(conformanceClosureV1(input.workspace,targets));restored=[...seen.restored,...seen.cached].filter(id=>closure.has(id)&&!seen.executed.includes(id));}catch{restored=undefined;}
    return {exitCode:result.exitCode,milliseconds:Date.now()-started,output:result.output,...(restored?{restored}:{})};
   },
   async affected(files,signal){
    const result=await bazelRun(["query",`rdeps(//:all, set(${files.map(file=>"//:"+file).join(" ")}))`,"--noimplicit_deps","--output=label"],signal);
    if(result.exitCode!==0)throw Error("bazel could not query affected targets: "+result.output.slice(-400));
    return [...new Set(result.output.split("\n").map(line=>owner.get(line.trim())).filter((id):id is string=>id!==undefined).map(id=>id.split(":")[0]!))];
   },
   async clearLocalCache(){await bazelRun(["shutdown"],new AbortController().signal);rmSync(outputBase,{recursive:true,force:true});rmSync(disk,{recursive:true,force:true});},
   async close(){await bazelRun(["shutdown"],new AbortController().signal).catch(()=>undefined);}
  };
 }
});
