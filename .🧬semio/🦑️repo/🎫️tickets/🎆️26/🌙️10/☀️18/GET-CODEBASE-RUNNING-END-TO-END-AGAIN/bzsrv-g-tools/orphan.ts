import {mkdirSync,rmSync,writeFileSync,readFileSync,existsSync,appendFileSync} from "node:fs";
import {join} from "node:path";
import {spawn,spawnSync} from "node:child_process";
import {bazel,flagsFor,serverPid,shutdown,table} from "./probe.ts";
const tools=import.meta.dir,S="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/bz",root=join(S,"orphan");
rmSync(root,{recursive:true,force:true});const pkg=join(root,"w/s0/k");mkdirSync(pkg,{recursive:true});
const BAZEL="/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel",BUN=process.execPath;
writeFileSync(join(pkg,"MODULE.bazel"),'module(name="fx", version="0")\n');
writeFileSync(join(pkg,"fx-worker.ts"),readFileSync(join(tools,"fx-worker.ts"),"utf8"));
writeFileSync(join(pkg,"tasks.bzl"),`
def _slow(ctx):
    out = ctx.actions.declare_file(ctx.label.name + ".out")
    ctx.actions.run_shell(outputs = [out], command = "sleep 300; echo done > " + out.path, mnemonic = "FxSlow", execution_requirements = {"no-sandbox": "1", "no-cache": "1"})
    return [DefaultInfo(files = depset([out]))]
slow = rule(implementation = _slow)
def _work(ctx):
    out = ctx.actions.declare_file(ctx.label.name + ".out")
    args = ctx.actions.args()
    args.use_param_file("@%s", use_always = True)
    args.set_param_file_format("multiline")
    args.add(out.path)
    ctx.actions.run(executable = ctx.attr.bun, arguments = [ctx.file.script.path, args], inputs = [ctx.file.script], outputs = [out], mnemonic = "FxWork", execution_requirements = {"supports-workers": "1", "requires-worker-protocol": "json", "no-cache": "1"})
    return [DefaultInfo(files = depset([out]))]
work = rule(implementation = _work, attrs = {"bun": attr.string(), "script": attr.label(allow_single_file = True)})
`);
writeFileSync(join(pkg,"BUILD.bazel"),`load(":tasks.bzl", "slow", "work")\nslow(name = "slow")\nwork(name = "w1", bun = "${BUN}", script = "fx-worker.ts")\n`);
const flags=flagsFor(tools,join(root,"p"),"s0",1024);
flags.startup=flags.startup.filter(value=>!value.startsWith("--max_idle_secs=")).concat(["--max_idle_secs=20"]);
const base=flags.startup.find(value=>value.startsWith("--output_base="))!.slice(14),rows:Record<string,unknown>={};
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
const descendants=(root:number)=>{const rows=table(),out:number[]=[],stack=[root];while(stack.length){const pid=stack.pop()!;for(const[child,row]of rows)if(row.ppid===pid){out.push(child);stack.push(child);}}return out;};
// 1. worker leaves with its server
const worker=await bazel(flags,[],pkg,["build",...flags.command,"--strategy=FxWork=worker,sandboxed,local","--worker_max_instances=FxWork=1","--noworker_multiplex","--experimental_worker_strict_flagfiles","//:w1"],{});
rows.workerBuild={code:worker.code,tail:worker.log.split("\n").filter(line=>/ERROR|error|FxWork/.test(line)).slice(0,6).join(" | ").slice(0,900)};if(process.argv[2]==="worker-only"){console.log(JSON.stringify(rows));console.log(worker.log.slice(-1800));const wsv=serverPid(base);const wk=wsv?descendants(wsv).filter(pid=>table().get(pid)?.command.includes("fx-worker")):[];console.log("workers",wk);if(wsv){const t0=Date.now();process.kill(wsv,"SIGTERM");while(alive(wsv)&&Date.now()-t0<20000)await Bun.sleep(200);await Bun.sleep(1500);console.log("server gone",!alive(wsv),"workers gone",wk.map(pid=>!alive(pid)));}rmSync(root,{recursive:true,force:true});process.exit(0);}
const server=serverPid(base);rows.serverAlive=server!==undefined&&alive(server);
const workers=server?descendants(server).filter(pid=>table().get(pid)?.command.includes("fx-worker")):[];rows.workerPids=workers;
// 2. client killed mid-action
const client=spawn(BAZEL,[...flags.startup,"build",...flags.command,"--spawn_strategy=local","//:slow"],{cwd:pkg,stdio:"ignore",env:{...process.env,...flags.environment}});
let sleeper:number|undefined;for(let tries=0;tries<100&&sleeper===undefined;tries++){await Bun.sleep(200);const s=serverPid(base);if(s)sleeper=descendants(s).find(pid=>table().get(pid)?.command.includes("sleep 300"));}
rows.sleeperStarted=sleeper!==undefined;const killedAt=Date.now();client.kill("SIGKILL");
let sleeperGoneMs:number|undefined;for(let tries=0;tries<300;tries++){await Bun.sleep(200);if(sleeper===undefined||!alive(sleeper)){sleeperGoneMs=Date.now()-killedAt;break;}}
rows.actionKilledAfterClientDeathMs=sleeperGoneMs??"still running";
// 3. idle exit bounds the orphan server and its workers
let serverGoneMs:number|undefined;const idleStart=Date.now();for(let tries=0;tries<400;tries++){await Bun.sleep(500);if(server===undefined||!alive(server)){serverGoneMs=Date.now()-killedAt;break;}}
rows.serverGoneAfterKillMs=serverGoneMs??"still running";rows.workersGone=workers.every(pid=>!alive(pid));rows.maxIdleSecs=20;
const out=join(tools,"..","🗑️generated","bzsrv-g");mkdirSync(out,{recursive:true});appendFileSync(join(out,"orphan.jsonl"),JSON.stringify(rows)+"\n");console.log(JSON.stringify(rows,null,1));
if(server&&alive(server))shutdown(flags,pkg);rmSync(root,{recursive:true,force:true});
