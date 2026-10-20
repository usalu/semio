import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {bazelGrantedWorkerHostV1,bazelHostPlanV1,bazelHostResourceSettingsV1,bazelLocalConcurrencyV1,bazelResourceSetsV1} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

test("The plan renders into Bazel's local resource flags, and a plan travels through the environment",()=>{
 const plan=fixture.plan.value as never,settings=bazelHostResourceSettingsV1(plan);
 expect(settings.flags).toEqual(fixture.plan.flags);
 expect(settings.mapping.map(row=>row.field)).toEqual(["jobs","cpu","memoryMegabytes","fixedMegabytes","floorMegabytes"]);
 expect(settings.mapping.filter(row=>row.support==="exact").map(row=>row.field)).toEqual(["jobs","cpu","memoryMegabytes"]);
 expect(bazelHostPlanV1({SEMIO_HOST_CAPACITY_PLAN:JSON.stringify(fixture.plan.value)})).toEqual(fixture.plan.value as never);
 expect(bazelHostPlanV1({})).toBeUndefined();
 expect(()=>bazelHostPlanV1({SEMIO_HOST_CAPACITY_PLAN:"{}"})).toThrow();
});

test("Worker pools see the granted cores and memory, never more than the host has",()=>{
 const host={cores:fixture.plan.workerHost.hostCores,memoryBytes:fixture.plan.workerHost.hostMemoryBytes};
 expect(bazelGrantedWorkerHostV1(fixture.plan.value as never,host)).toEqual({cores:fixture.plan.workerHost.cores,memoryBytes:fixture.plan.workerHost.memoryBytes});
 expect(bazelGrantedWorkerHostV1(undefined,host)).toEqual(host);
 expect(bazelGrantedWorkerHostV1({...fixture.plan.value,cpu:64,memoryMegabytes:10_000_000} as never,host)).toEqual(host);
});

test("Per-kind estimates render into Starlark resource sets keyed by mnemonic, and services get none",()=>{
 expect(bazelResourceSetsV1(fixture.resourceSets.kinds as never)).toBe(fixture.resourceSets.expect);
});

test("Bazel's local resource manager model: the narrowest of jobs, cores and memory decides, and an idle pool always starts one action",()=>{
 for(const item of fixture.concurrency)expect(bazelLocalConcurrencyV1(item.pool,item.action,item.count),item.name).toBe(item.expect);
});

const binary=process.env.SEMIO_BAZEL_BINARY??"",live=process.env.SEMIO_BAZEL_INTEGRATION==="1"&&binary!==""&&existsSync(binary);
test.skipIf(!live)("Real Bazel runs exactly as many local actions at once as the rendered flags and the resource sets allow",()=>{
 const root=mkdtempSync(join(tmpdir(),"hb-bz-")),installBase=join(root,"ib");
 try{
  for(const [index,item] of fixture.live.entries()){
   const workspace=join(root,`w${index}`);mkdirSync(workspace,{recursive:true});
   writeFileSync(join(workspace,"MODULE.bazel"),'module(name = "hc", version = "0")\n');
   writeFileSync(join(workspace,"rules.bzl"),`def _resources(os, n):\n    return {"cpu": ${item.action.cpu}, "memory": ${item.action.memoryMegabytes}}\n\ndef _impl(ctx):\n    out = ctx.actions.declare_file(ctx.label.name + ".txt")\n    ctx.actions.run_shell(outputs = [out], command = "perl -MTime::HiRes=time -e 'printf q(%.3f ), time' > " + out.path + "; sleep 1.5; perl -MTime::HiRes=time -e 'printf q(%.3f), time' >> " + out.path, mnemonic = "HcAction", resource_set = _resources, execution_requirements = {"no-sandbox": "1", "no-cache": "1"})\n    return [DefaultInfo(files = depset([out]))]\n\nhc = rule(implementation = _impl)\n`);
   writeFileSync(join(workspace,"BUILD.bazel"),'load(":rules.bzl", "hc")\n'+Array.from({length:item.count},(_,row)=>`hc(name = "a${row}")\n`).join(""));
   const flags=bazelHostResourceSettingsV1({version:1,id:"aaaaaaaa1",jobs:item.jobs,cpu:item.cpu,memoryMegabytes:item.memoryMegabytes,fixedMegabytes:0,floorMegabytes:0}).flags;
   const run=spawnSync(binary,["--batch","--host_jvm_args=-Xmx512m","--nosystem_rc","--nohome_rc",`--install_base=${installBase}`,`--output_base=${join(root,`o${index}`)}`,"build","//:all","--spawn_strategy=local","--nobuild_runfile_links",`--jobs=${item.jobs}`,...flags],{cwd:workspace,encoding:"utf8"});
   expect(run.status,run.stderr.slice(-2000)).toBe(0);
   const bin=join(workspace,"bazel-bin"),spans=readdirSync(bin).filter(name=>name.endsWith(".txt")).map(name=>readFileSync(join(bin,name),"utf8").trim().split(/\s+/u).map(Number) as [number,number]);
   expect(spans.length).toBe(item.count);
   const peak=Math.max(...spans.map(([start])=>spans.filter(([from,to])=>from<=start&&start<to).length));
   expect(peak,item.name).toBe(bazelLocalConcurrencyV1({jobs:item.jobs,cpu:item.cpu,memoryMegabytes:item.memoryMegabytes},item.action,item.count));
  }
 }finally{
  spawnSync(binary,["--nosystem_rc","--nohome_rc",`--install_base=${join(root,"ib")}`,"shutdown"],{encoding:"utf8"});
  rmSync(root,{recursive:true,force:true});
 }
},600_000);
