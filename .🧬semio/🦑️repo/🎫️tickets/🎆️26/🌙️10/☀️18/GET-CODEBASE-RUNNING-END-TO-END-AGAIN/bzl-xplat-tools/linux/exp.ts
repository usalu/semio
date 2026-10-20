import {cpSync,existsSync,mkdirSync,readdirSync,readFileSync,rmSync,symlinkSync,writeFileSync,lstatSync,realpathSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const [bazel,base,rc]=process.argv.slice(2);
const bun=process.execPath;
const results:Record<string,unknown>={};
const BASERC=readFileSync(rc!,"utf8");
function workspace(name:string){
 const root=join(base!,name);rmSync(root,{recursive:true,force:true});mkdirSync(join(root,"src/🧰️framework"),{recursive:true});mkdirSync(join(root,"src/.🧬semio"),{recursive:true});
 writeFileSync(join(root,"MODULE.bazel"),'module(name = "xplat", version = "0.0.0")\n');
 writeFileSync(join(root,".bazelrc"),BASERC);
 writeFileSync(join(root,"src/🧰️framework/🟦️.txt"),"emoji input\n");writeFileSync(join(root,"src/.🧬semio/x.txt"),"dot emoji input\n");
 writeFileSync(join(root,"task.ts"),'import {mkdirSync,readFileSync,writeFileSync} from "node:fs";const [input,out]=process.argv.slice(2);mkdirSync(out+"/🧪️d",{recursive:true});writeFileSync(out+"/🧪️d/📜️result.txt",readFileSync(input,"utf8").toUpperCase());\n');
 writeFileSync(join(root,"tasks.bzl"),`def _impl(ctx):
    out = ctx.actions.declare_directory(ctx.label.name + ".snapshot")
    req = {"no-remote-exec": "1"}
    if ctx.attr.host_state:
        req["no-sandbox"] = "1"
    ctx.actions.run(executable = ctx.attr.bun, arguments = [ctx.file.script.path, ctx.files.srcs[0].path, out.path], inputs = depset([ctx.file.script] + ctx.files.srcs), outputs = [out], mnemonic = "RepoTask", execution_requirements = req, use_default_shell_env = True)
    return [DefaultInfo(files = depset([out]))]
repo_task = rule(implementation = _impl, attrs = {"srcs": attr.label_list(allow_files = True), "script": attr.label(allow_single_file = True), "bun": attr.string(mandatory = True), "host_state": attr.bool(default = True)})
def _launch(ctx):
    launcher = ctx.actions.declare_file(ctx.label.name + ctx.attr.extension)
    ctx.actions.run(executable = ctx.attr.bun, arguments = [ctx.file.script.path, ctx.files.srcs[0].path, launcher.path + ".d"], inputs = depset([ctx.file.script] + ctx.files.srcs), outputs = [launcher], mnemonic = "RepoLauncher", execution_requirements = {"no-sandbox": "1"})
    return [DefaultInfo(executable = launcher, runfiles = ctx.runfiles(files = [ctx.file.script]))]
repo_launch = rule(implementation = _launch, executable = True, attrs = {"srcs": attr.label_list(allow_files = True), "script": attr.label(allow_single_file = True), "bun": attr.string(mandatory = True), "extension": attr.string(default = "")})
`);
 writeFileSync(join(root,"BUILD.bazel"),`load(":tasks.bzl", "repo_launch", "repo_task")
repo_task(name = "t_host", srcs = ["s/🧰️framework/🟦️.txt"], script = "task.ts", bun = ${JSON.stringify(bun)})
repo_task(name = "t_dot", srcs = ["s/.🧬semio/x.txt"], script = "task.ts", bun = ${JSON.stringify(bun)})
repo_task(name = "t_sandbox", srcs = ["s/🧰️framework/🟦️.txt"], script = "task.ts", bun = ${JSON.stringify(bun)}, host_state = False)
repo_launch(name = "l_launch", srcs = ["s/🧰️framework/🟦️.txt"], script = "task.ts", bun = ${JSON.stringify(bun)})
`);
 symlinkSync(join(root,"src"),join(root,"s"),"dir");
 return root;
}
type Run={code:number|null;out:string};
function bz(root:string,startup:string[],command:string[],args:string[],env:Record<string,string|undefined>={}):Run{
 const environment:Record<string,string|undefined>={PATH:process.env.PATH,HOME:process.env.HOME,USER:process.env.USER,TMPDIR:process.env.TMPDIR,...env};
 const result=spawnSync(bazel!,[...startup,...args,...command],{cwd:root,env:environment as NodeJS.ProcessEnv,encoding:"utf8",timeout:240000});
 return {code:result.status,out:((result.stdout??"")+(result.stderr??"")).split("\n").filter(line=>!/^\s*$/u.test(line)).slice(-14).join("\n")};
}
const summary=(run:Run)=>run.out.split("\n").filter(line=>/(INFO: (Build completed|Elapsed|\d+ process)|ERROR|FATAL|WARNING|processes?:|disk cache hit|linux-sandbox|processwrapper)/u.test(line)).join(" | ").slice(0,700);
function flags(name:string,over:{outputUserRoot?:boolean;outputBase?:string;command?:string[];}={}){
 const dir=join(base!,name+"-state");mkdirSync(dir,{recursive:true});
 const ob=over.outputBase??join(dir,"outputs/server/abcdef0123456789");
 const startup=[...(over.outputUserRoot===false?[]:[`--output_user_root=${join(dir,"outputs/root")}`]),`--output_base=${ob}`,`--install_base=${join(dir,"install")}`,"--max_idle_secs=30","--host_jvm_args=-Xmx2048m","--nosystem_rc","--nohome_rc",`--bazelrc=${join(base!,"..","semio.bazelrc")}`];
 const command=["--lockfile_mode=off","--incompatible_strict_action_env",`--disk_cache=${join(dir,"actions")}`,"--jobs=4","--keep_going",...(over.command??[])];
 return {startup,command,dir,ob};
}
function finish(root:string,startup:string[]){bz(root,startup,[],["shutdown"]);}
const record=(key:string,value:unknown)=>{results[key]=value;console.error(`${key}: ${typeof value==="string"?value:JSON.stringify(value)}`);};
writeFileSync(join(base!,"..","semio.bazelrc"),BASERC);

{ const root=workspace("a"),f=flags("a"); const r=bz(root,f.startup,f.command,["build","//:t_host","//:t_dot","//:l_launch"]);
  record("A-production-flags-incl-output_user_root",{code:r.code,summary:summary(r)});
  const out=bz(root,f.startup,[],["info","bazel-bin"]).out.trim().split("\n").at(-1)!;record("A-bazel-bin",out);
  const tree=join(out,"t_host.snapshot/🧪️d/📜️result.txt");record("A-emoji-output-exists",existsSync(tree)?readFileSync(tree,"utf8"):"MISSING");
  const warm=bz(root,f.startup,f.command,["build","//:t_host","//:t_dot","//:l_launch"]);record("A-warm",summary(warm));
  const runfiles=existsSync(out+"/l_launch.runfiles");record("A-runfiles-tree-default",runfiles);
  bz(root,f.startup,[],["clean"]);const re=bz(root,f.startup,f.command,["build","//:t_host","//:t_dot","//:l_launch"]);record("A-after-clean",summary(re));
  finish(root,f.startup);}
{ const root=workspace("b"),f=flags("b",{outputUserRoot:false,command:["--nobuild_runfile_links"]}); const r=bz(root,f.startup,f.command,["build","//:t_host","//:l_launch"]);
  record("B-no-output_user_root-nobuild_runfile_links",{code:r.code,summary:summary(r)});
  const out=bz(root,f.startup,[],["info","bazel-bin"]).out.trim().split("\n").at(-1)!;
  record("B-runfiles-tree",existsSync(out+"/l_launch.runfiles")); record("B-runfiles-manifest",existsSync(out+"/l_launch.runfiles_manifest")||existsSync(out+"/l_launch.runfiles/MANIFEST"));
  const r2=bz(root,f.startup,[...f.command,"--noenable_runfiles","--nowindows_enable_symlinks"],["build","//:t_host","//:l_launch"]);record("B-extra-platform-flags-accepted",{code:r2.code,summary:summary(r2)});
  finish(root,f.startup);}
for(const [name,env] of Object.entries({"posix":{LC_ALL:"POSIX"},"unset":{},"C":{LC_ALL:"C",LANG:"C"},"C.UTF-8":{LC_ALL:"C.UTF-8",LANG:"C.UTF-8"},"en_US.UTF-8":{LC_ALL:"en_US.UTF-8",LANG:"en_US.UTF-8"}} as Record<string,Record<string,string>>)){
 const root=workspace("loc-"+name.replace(/\W/g,"")),f=flags("loc-"+name.replace(/\W/g,""),{outputUserRoot:false});const r=bz(root,f.startup,f.command,["build","//:t_host","//:t_dot"],env);
 const out=bz(root,f.startup,[],["info","bazel-bin"],env).out.trim().split("\n").at(-1)!;
 record("D-locale-"+name,{code:r.code,output:existsSync(join(out,"t_host.snapshot/🧪️d/📜️result.txt")),summary:summary(r).slice(0,300)});finish(root,f.startup);
}
{ const root=workspace("e"),dir=join(base!,"🧬non-ascii-ob");const f=flags("e",{outputUserRoot:false,outputBase:join(dir,"server/abc")});const r=bz(root,f.startup,f.command,["build","//:t_host"]);
  record("E-non-ascii-output-base",{code:r.code,summary:summary(r)});finish(root,f.startup);}
{ const root=workspace("e2"),f=flags("e2",{outputUserRoot:false});const f2={...f,startup:f.startup.map(v=>v.startsWith("--install_base=")?`--install_base=${join(base!,"🧬install")}`:v)};const r=bz(root,f2.startup,f.command,["build","//:t_host"]);
  record("E2-non-ascii-install-base",{code:r.code,summary:summary(r)});finish(root,f2.startup);}
{ const root=workspace("f"),f=flags("f",{outputUserRoot:false});
  const r=bz(root,f.startup,f.command,["build","//:t_sandbox","--spawn_strategy=linux-sandbox"]);record("F-linux-sandbox-explicit",{code:r.code,summary:summary(r),tail:r.out.slice(-600)});
  const r2=bz(root,f.startup,[...f.command,"--sandbox_debug"],["build","//:t_sandbox"],{});record("F-default-strategy-sandboxed-action",{code:r2.code,summary:summary(r2)});
  const r3=bz(root,f.startup,f.command,["build","//:t_sandbox","--spawn_strategy=processwrapper-sandbox"]);record("F-processwrapper-sandbox",{code:r3.code,summary:summary(r3)});
  const r4=bz(root,f.startup,f.command,["build","//:t_host"]);record("F-host-state-default",{code:r4.code,summary:summary(r4)});
  finish(root,f.startup);}
{ const f=flags("g");const r=bz(base!,f.startup,[],["help","build","--long"]);record("G-help-long-chars",r.out.length);
  const full=spawnSync(bazel!,[...f.startup,"help","build","--long"],{cwd:workspace("g"),encoding:"utf8",env:{PATH:process.env.PATH,HOME:process.env.HOME}});
  const text=(full.stdout??"")+(full.stderr??"");
  record("G-flags-known",Object.fromEntries(["build_runfile_links","enable_runfiles","windows_enable_symlinks","experimental_convenience_symlinks","incompatible_strict_action_env","disk_cache","experimental_disk_cache_gc_max_size","lockfile_mode","repository_cache","repository_disable_download"].map(name=>[name,text.includes(`--[no]${name}`)||text.includes(`--${name}`)])));
  finish(workspace("g"),f.startup);}
console.log(JSON.stringify(results,null,1));
