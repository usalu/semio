import {existsSync,mkdirSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const [bazel,base,rc]=process.argv.slice(2),bun=process.execPath,BASERC=readFileSync(rc!,"utf8"),results:Record<string,unknown>={};
const record=(key:string,value:unknown)=>{results[key]=value;console.error(`${key}: ${typeof value==="string"?value:JSON.stringify(value)}`);};
function workspace(name:string){
 const root=join(base!,name);rmSync(root,{recursive:true,force:true});mkdirSync(join(root,"src/🧰️framework"),{recursive:true});
 writeFileSync(join(root,"MODULE.bazel"),'module(name = "xplat", version = "0.0.0")\n');writeFileSync(join(root,".bazelrc"),BASERC);
 writeFileSync(join(root,"src/🧰️framework/🟦️.txt"),"emoji input\n");
 writeFileSync(join(root,"task.ts"),'import {mkdirSync,readFileSync,writeFileSync,copyFileSync} from "node:fs";const [mode,input,out]=process.argv.slice(2);if(mode==="launcher")copyFileSync(process.execPath,out);else{mkdirSync(out+"/🧪️d",{recursive:true});writeFileSync(out+"/🧪️d/📜️result.txt",readFileSync(input,"utf8").toUpperCase());}\n');
 writeFileSync(join(root,"tasks.bzl"),`def _impl(ctx):
    out = ctx.actions.declare_directory(ctx.label.name + ".snapshot")
    req = {"no-remote-exec": "1"}
    if ctx.attr.host_state:
        req["no-sandbox"] = "1"
    ctx.actions.run(executable = ctx.attr.bun, arguments = ["task.ts", "task", ctx.files.srcs[0].path, out.path], inputs = depset([ctx.file.script] + ctx.files.srcs), outputs = [out], mnemonic = "RepoTask", execution_requirements = req, use_default_shell_env = True)
    return [DefaultInfo(files = depset([out]))]
repo_task = rule(implementation = _impl, attrs = {"srcs": attr.label_list(allow_files = True), "script": attr.label(allow_single_file = True), "bun": attr.string(mandatory = True), "host_state": attr.bool(default = True)})
def _launch(ctx):
    launcher = ctx.actions.declare_file(ctx.label.name + ctx.attr.extension)
    ctx.actions.run(executable = ctx.attr.bun, arguments = ["task.ts", "launcher", ctx.files.srcs[0].path, launcher.path], inputs = depset([ctx.file.script] + ctx.files.srcs), outputs = [launcher], mnemonic = "RepoLauncher", execution_requirements = {"no-sandbox": "1"})
    return [DefaultInfo(executable = launcher, runfiles = ctx.runfiles(files = [ctx.file.script]))]
repo_launch = rule(implementation = _launch, executable = True, attrs = {"srcs": attr.label_list(allow_files = True), "script": attr.label(allow_single_file = True), "bun": attr.string(mandatory = True), "extension": attr.string(default = "")})
`);
 writeFileSync(join(root,"BUILD.bazel"),`load(":tasks.bzl", "repo_launch", "repo_task")
repo_task(name = "t_host", srcs = ["s/🧰️framework/🟦️.txt"], script = "task.ts", bun = ${JSON.stringify(bun)})
repo_task(name = "t_sandbox", srcs = ["s/🧰️framework/🟦️.txt"], script = "task.ts", bun = ${JSON.stringify(bun)}, host_state = False)
repo_launch(name = "l_launch", srcs = ["s/🧰️framework/🟦️.txt"], script = "task.ts", bun = ${JSON.stringify(bun)})
`);
 symlinkSync(join(root,"src"),join(root,"s"),"dir");return root;
}
const run=(root:string,args:string[],env:Record<string,string|undefined>={})=>{const r=spawnSync(bazel!,args,{cwd:root,env:{PATH:process.env.PATH,HOME:process.env.HOME,...env} as NodeJS.ProcessEnv,encoding:"utf8",timeout:240000});return{code:r.status,out:((r.stdout??"")+(r.stderr??"")).split("\n").filter(line=>!/^\s*$/u.test(line)&&!/^(Computing main repo mapping|Loading|Analyzing)/u.test(line))};};
const lines=(r:{out:string[]},pattern:RegExp)=>r.out.filter(line=>pattern.test(line)).join(" | ").slice(0,900);
const common=(dir:string)=>[`--output_base=${dir}/ob`,`--install_base=${dir}/install`,"--max_idle_secs=20","--nosystem_rc","--nohome_rc",`--bazelrc=${join(base!,"..","semio.bazelrc")}`];
const cmd=(dir:string)=>["--lockfile_mode=off","--incompatible_strict_action_env",`--disk_cache=${dir}/actions`,"--jobs=4","--keep_going"];
const summaryPattern=/(Build completed|processes?:|ERROR|FATAL|WARNING|disk cache hit)/u;
const root=workspace("p"),dir=join(base!,"p-state");mkdirSync(dir,{recursive:true});writeFileSync(join(base!,"..","semio.bazelrc"),BASERC);
{const r=run(root,[...common(dir),"--nowindows_enable_symlinks","build","//:t_host","//:l_launch",...cmd(dir),"--nobuild_runfile_links","--noenable_runfiles"]);record("P1-startup-nowindows_enable_symlinks+noenable_runfiles",{code:r.code,summary:lines(r,summaryPattern)});}
{const r=run(root,[...common(dir),"--windows_enable_symlinks","build","//:t_host",...cmd(dir)]);record("P2-startup-windows_enable_symlinks",{code:r.code,summary:lines(r,summaryPattern)});}
{const r=run(root,[...common(dir),"help","startup_options"]);record("P3-startup-help-has-windows",lines(r,/windows|symlink/u));}
{const r=run(root,[...common(dir),"help","build","--long"]);record("P4-build-help-flags",Object.fromEntries(["build_runfile_links","enable_runfiles","windows_enable_symlinks","experimental_disk_cache_gc_max_size","lockfile_mode","incompatible_strict_action_env","repository_disable_download","experimental_convenience_symlinks"].map(name=>[name,r.out.some(line=>line.includes(`--[no]${name}`)||line.includes(`--${name}`))])));}
{const r=run(root,[...common(dir),"build","//:l_launch",...cmd(dir),"--nobuild_runfile_links"]);const out=run(root,[...common(dir),"info","bazel-bin"]).out.at(-1)!;record("P5-launcher-nobuild_runfile_links",{code:r.code,summary:lines(r,summaryPattern),launcher:existsSync(out+"/l_launch"),runfilesTree:existsSync(out+"/l_launch.runfiles"),manifest:existsSync(out+"/l_launch.runfiles_manifest")});}
{const r=run(root,[...common(dir),"build","//:l_launch",...cmd(dir)]);const out=run(root,[...common(dir),"info","bazel-bin"]).out.at(-1)!;record("P6-launcher-default-runfiles",{code:r.code,runfilesTree:existsSync(out+"/l_launch.runfiles")});}
for(const [name,env] of Object.entries({unset:{},posix:{LC_ALL:"POSIX"},"C.UTF-8":{LC_ALL:"C.UTF-8"},"en_US.UTF-8":{LC_ALL:"en_US.UTF-8",LANG:"en_US.UTF-8"}} as Record<string,Record<string,string>>)){const bad=join(base!,"🧬install-"+name.replace(/\W/g,"")),r=run(root,["--output_base="+join(base!,"p-ob2-"+name.replace(/\W/g,"")),"--install_base="+bad,"--nosystem_rc","--nohome_rc","info"],env);record("P7-non-ascii-install-base-linux-"+name,{code:r.code,output:r.out.slice(-4).join(" | ").slice(0,500)});run(root,["--output_base="+join(base!,"p-ob2-"+name.replace(/\W/g,"")),"shutdown"],env);}
{const r=run(root,[...common(dir),"build","//:t_sandbox",...cmd(dir),"--spawn_strategy=linux-sandbox"]);record("P8-explicit-linux-sandbox",{code:r.code,summary:lines(r,summaryPattern)});}
{const r=run(root,[...common(dir),"build","//:t_sandbox",...cmd(dir),"--sandbox_debug","--spawn_strategy=sandboxed"]);record("P9-sandboxed-strategy",{code:r.code,summary:lines(r,summaryPattern)});}
run(root,[...common(dir),"shutdown"]);
console.log(JSON.stringify(results,null,1));
