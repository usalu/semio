import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import fixture from "./🧫️fixtures/🔣️.json";
import {HERMETIC_TABLE_V1,hermeticStrategiesV1} from "../../../../../🏃️execution/🔒️hermetic/🟦️.ts";

const binary=process.env.SEMIO_TEST_BAZEL_BINARY;

test("every platform lists only strategies that every Bazel host registers, so an unprivileged container or a stripped host never refuses the command",()=>{
 for(const platform of Object.keys(HERMETIC_TABLE_V1.strategies)){
  const names=hermeticStrategiesV1(platform);
  expect(names.every(name=>(fixture.portable as readonly string[]).includes(name)),platform+": "+names.join(",")).toBe(true);
  expect(names).toContain("local");
 }
 expect(hermeticStrategiesV1("linux")).toContain("sandboxed");expect(hermeticStrategiesV1("darwin")).toContain("sandboxed");expect(hermeticStrategiesV1("win32")).not.toContain("sandboxed");
});

const scratch=()=>mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"strat-"));
const bazel=(root:string,args:string[])=>Bun.spawnSync([binary!,"--install_base="+join(root,"install"),"--output_base="+join(root,"out"),"--max_idle_secs=5","--host_jvm_args=-Xmx384m",...args],{cwd:join(root,"ws"),stdout:"pipe",stderr:"pipe",env:{...process.env,LANG:"C.UTF-8"}});
const workspace=(root:string)=>{mkdirSync(join(root,"ws"),{recursive:true});writeFileSync(join(root,"ws","WORKSPACE"),'workspace(name = "strategies")\n');writeFileSync(join(root,"ws","BUILD.bazel"),'genrule(name = "probe", outs = ["probe.txt"], cmd = "echo ok > $@")\n');};
const build=(root:string,list:string)=>bazel(root,["build","--noenable_bzlmod","--enable_workspace","--nobuild_runfile_links","--strategy=Genrule="+list,"//:probe"]);

test.skipIf(!binary)("with the real Bazel the host's own list runs an action in a sandbox the host has, and a platform-specific name from another host is refused by name",()=>{
 const root=scratch();try{
  workspace(root);const own=hermeticStrategiesV1(process.platform).filter(name=>name!=="remote").join(",");
  const ok=build(root,own),output=ok.stdout.toString()+ok.stderr.toString();
  expect(ok.exitCode,output).toBe(0);expect(new RegExp("[0-9]+ ("+fixture.runners[process.platform as "darwin"].join("|")+")").test(output),output).toBe(true);
  bazel(root,["clean"]);
  const foreign=fixture.registeredNowhereOnTheHost[process.platform as "darwin"],refused=build(root,foreign+",local"),text=refused.stdout.toString()+refused.stderr.toString();
  expect(refused.exitCode,text).not.toBe(0);expect(text).toContain(fixture.refusal);expect(text).toContain("'"+foreign+"'");
 }finally{bazel(root,["shutdown"]);rmSync(root,{recursive:true,force:true});}
},300000);

test("the staged fixture of the adapter renders the same portable list for every host",()=>{
 const adapter=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")) as unknown;
 const flags=JSON.stringify(adapter).match(/--strategy=RepoHermetic=[^"]+/gu)??[];
 expect(flags.length).toBeGreaterThan(0);
 for(const flag of flags)for(const name of flag.split("=")[2]!.split(","))expect(fixture.portable as readonly string[],flag).toContain(name);
 expect(existsSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"))).toBe(true);
});
