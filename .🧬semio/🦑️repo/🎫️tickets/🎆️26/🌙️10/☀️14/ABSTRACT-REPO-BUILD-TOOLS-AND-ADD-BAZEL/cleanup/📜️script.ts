import {readdir,readFile,writeFile} from "node:fs/promises";
import {join,resolve} from "node:path";
import {createHash} from "node:crypto";
const root=process.env.SEMIO_WORKSPACE_ROOT!,generated=process.env.SEMIO_TEST_ARTIFACT_DIR!;
if(!root||!generated)throw Error("Workspace and ticket output paths required");
const {acquireResourceLease}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts")),workspaces=join(root,".🧬semio/🦑️repo/⚡️cache/build/bazel/w"),servers=join(root,".🧬semio/🦑️repo/⚡️cache/bz"),binary=join(root,".🧬semio/🦑️repo/⚡️cache/tools/bazel/45e9388abf21d110/bazel"),results:unknown[]=[];
const scopes=[servers,...(await readdir(servers,{withFileTypes:true})).filter(entry=>entry.isDirectory()&&/^[a-f0-9]{16}$/u.test(entry.name)).map(entry=>join(servers,entry.name))];
for(const namespace of await readdir(workspaces)){
 const workspace=join(workspaces,namespace),manifests=(await readdir(workspace)).filter(file=>/^t_[a-f0-9]+\.json$/u.test(file));if(!manifests.length)continue;
 const entries=await Promise.all(manifests.map(async file=>JSON.parse(await readFile(join(workspace,file),"utf8"))));if(entries.some(entry=>resolve(entry.workspace)!==root||!resolve(root,entry.task.root).startsWith(resolve(generated)+"/")||!["fixture","runtime-probe"].includes(entry.task.project)))continue;
 for(const scope of scopes){const base=join(scope,createHash("md5").update(workspace).digest("hex"));let command:string,pid:number;try{command=await readFile(join(base,"server/cmdline"),"utf8");pid=Number(await readFile(join(base,"server/server.pid.txt"),"utf8"));process.kill(pid,0);}catch{continue;}
 const args=command.split("\0");if(!args.some(arg=>arg==="--max_idle_secs=10800"||arg==="--max_idle_secs=60")||!args.includes("--workspace_directory="+workspace)||!args.includes("--output_base="+base))continue;
 let lease;try{lease=await acquireResourceLease({directory:join(root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),resource:"bazel-workspace-"+namespace,mode:"exclusive",signal:new AbortController().signal,timeoutMs:0,onWait:()=>{}});}catch{results.push({namespace,pid,skipped:"active namespace lease"});continue;}
 try{const child=Bun.spawn([binary,"--output_user_root="+scope,"--output_base="+base,"--install_base="+join(root,".cache/bazel/install/8.4.2",process.platform+"-"+process.arch),"--host_jvm_args=-Xmx2048m","--max_idle_secs=60","--noblock_for_lock","--nosystem_rc","--nohome_rc","--noworkspace_rc","shutdown"],{cwd:workspace,stdout:"pipe",stderr:"pipe",timeout:10000,env:{...process.env,LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}}),[status,stdout,stderr]=await Promise.all([child.exited,new Response(child.stdout).text(),new Response(child.stderr).text()]);results.push({namespace,pid,status,stdout,stderr});console.log("[DEBUG] Fixture-only idle server shutdown "+JSON.stringify({namespace,pid,status}));}finally{lease.release();}}
}
await writeFile(join(generated,"fixture-server-cleanup.json"),JSON.stringify(results,null,2)+"\n");console.log("[DEBUG] Fixture server cleanup candidates "+results.length);
