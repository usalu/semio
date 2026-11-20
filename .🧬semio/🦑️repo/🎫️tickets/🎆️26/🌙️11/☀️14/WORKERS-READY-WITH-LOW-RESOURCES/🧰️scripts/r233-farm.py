from pathlib import Path
import argparse
from datetime import datetime, timezone
import difflib
import hashlib
import json
import os
import subprocess
import tempfile

build = Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build")
graph = build / "🕸️graph"
project_nodes = graph / "📥️inference/📦️project-nodes"
farm = project_nodes / "🏊️project-farm"
files = {
    "schema": graph / "🧬️schema/🔣️.json",
    "registry_reader": graph / "📥️inference/🔎️owner-discovery/🟨️.mjs",
    "inference_types": graph / "📥️inference/🟨️.d.mts",
    "follow": graph / "📥️load/🧭️registry-follow/🟦️.ts",
    "farm": farm / "🟨️.mjs",
    "nodes": project_nodes / "🟨️.mjs",
    "test": farm / "🧪️tests/🚧️no-inline/🟦️.ts",
}

def replace(source, old, new):
    if source.count(old) != 1:
        raise ValueError("Expected one anchor: " + old)
    return source.replace(old, new, 1)

def updated(before):
    result = dict(before)
    declarations = json.loads(before["schema"])
    assert "OwnerRegistryWorkerFailures" not in declarations["definitions"]
    assert json.dumps(declarations, ensure_ascii=False, indent=2) + "\n" == before["schema"]
    declarations["definitions"]["OwnerRegistryWorkerFailures"] = {
        "description": "EN: Native registry failures that must restart the current graph wave. DE: Fehler des nativen Eigentümerregisters, die den aktuellen Graph-Durchlauf neu starten.",
        "const": [
            {"code":"BUILD_SOURCE_AUTHORITY","prefix":"Native dependency has no build project owner: "},
            {"code":"BUILD_OWNER_MANIFEST_ABSENT","prefix":"Native execution contract requires a native project owner: "},
        ],
    }
    result["schema"] = json.dumps(declarations, ensure_ascii=False, indent=2) + "\n"

    result["registry_reader"] = replace(
        before["registry_reader"],
        'export function readBuildOwnerRegistrySourceV1(workspaceRoot){return pinnedAuthorityReadV1(join(workspaceRoot,"📋️build-owners.json"),()=>readPhysicalSource(join(workspaceRoot,"📋️build-owners.json")).toString("utf8"));}',
        '''/** 🧾️ Reads the pinned registry for inference, or the current physical registry when checking a native owner failure. */
export function readBuildOwnerRegistrySourceV1(workspaceRoot, policy) {
  const path=join(workspaceRoot,"📋️build-owners.json"),read=()=>readPhysicalSource(path).toString("utf8");
  return policy==="current"?read():pinnedAuthorityReadV1(path,read);
}''',
    )
    result["inference_types"] = replace(
        before["inference_types"],
        "export function readBuildOwnerRegistrySourceV1(workspaceRoot:string):string;",
        'export function readBuildOwnerRegistrySourceV1(workspaceRoot:string,policy?:"current"):string;',
    )
    follow = replace(before["follow"], 'import {join} from "node:path";\n', "")
    follow = replace(follow, 'import {readPhysicalSource} from "../../📥️inference/📍️coordinates/🟨️.mjs";\n', "")
    follow = replace(follow, 'import {BUILD_OWNER_REGISTRY_FILE,reconcileBuildOwnerRegistry}', 'import {reconcileBuildOwnerRegistry}')
    follow = replace(follow, "  const registryPath=join(root,BUILD_OWNER_REGISTRY_FILE);\n", "")
    result["follow"] = replace(follow, 'readPhysicalSource(registryPath).toString("utf8")!==before', 'readBuildOwnerRegistrySourceV1(root,"current")!==before')

    farm_source = replace(before["farm"], 'import { graphWaveMeterV1 }', '''import graphSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import { graphWaveMeterV1 }''')
    farm_source = replace(farm_source, "signal = undefined, processed", "signal = undefined, onOwnerRegistryFailure, processed")
    farm_source = replace(farm_source, """  const peaks = new Map();
  const dispatch""", """  const peaks = new Map();
  const forwardOwnerRegistryFailure = failure => {
    if (!graphSchema.definitions.OwnerRegistryWorkerFailures.const.some(({code, prefix}) => failure.code === code && failure.message.startsWith(prefix))) return;
    const error = Object.assign(new Error(failure.message), {name: failure.name, code: failure.code});
    onOwnerRegistryFailure?.(error);
    throw error;
  };
  const dispatch""")
    farm_source = replace(farm_source, "if (outcome.failure) { if (++counts.failed", "if (outcome.failure) { forwardOwnerRegistryFailure(outcome.failure); if (++counts.failed")
    farm_source = replace(farm_source, "const outcome = await work; if (outcome.overrun === undefined) return outcome;", "const outcome = await work; if (outcome.failure) forwardOwnerRegistryFailure(outcome.failure); if (outcome.overrun === undefined) return outcome;")
    result["farm"] = farm_source

    nodes = replace(before["nodes"], '  const farm=timedFarmV1(', '''  const warmStop=new AbortController(),passStop=new AbortController(),polling=new AbortController(),farmStop=new AbortController();
  const waveSignal=AbortSignal.any([farmStop.signal,context.invocation?.control.signal,context.phaseControl?.signal].filter(signal=>signal!==undefined));
  let registryFailure;
  const stopRegistryWave=error=>{
    if(registryFailure!==undefined)return;
    registryFailure=error;
    stopped=true;
    warmStop.abort();
    passStop.abort();
    polling.abort();
    farmStop.abort(error);
  };
  const farm=timedFarmV1(''')
    nodes = replace(nodes, "signal:context.invocation?.control.signal,processed:()=>projectIndex", "signal:waveSignal,onOwnerRegistryFailure:stopRegistryWave,processed:()=>projectIndex")
    nodes = replace(nodes, "signal:context.phaseControl?.signal,decide:event=>{decision=event;}", "signal:waveSignal,decide:event=>{decision=event;}")
    nodes = replace(nodes, "const flight={flying:0},warmStop=new AbortController(),passStop=new AbortController();", "const flight={flying:0};")
    nodes = replace(nodes, "byName=new Map(jobs.map(job=>[job.name,job])),polling=new AbortController(),lookProject", "byName=new Map(jobs.map(job=>[job.name,job])),lookProject")
    nodes = replace(nodes, "continue;}}clock.begin(job.name);", "continue;}}if(stopped)return;clock.begin(job.name);")
    nodes = replace(nodes, '()=>{missed.add(name);if(farm?.exists?.())throw farmJobRefusedV1', '()=>{if(registryFailure!==undefined)throw registryFailure;missed.add(name);if(farm?.exists?.())throw farmJobRefusedV1')
    nodes = replace(nodes, "await Promise.allSettled(extras);await sweepOrphans();", "await Promise.allSettled(extras);if(registryFailure!==undefined)throw registryFailure;await sweepOrphans();")
    nodes = replace(nodes, "if(!stalled)await Promise.allSettled(workers);throw error;", "if(!stalled)await Promise.allSettled(registryFailure===undefined?workers:[...workers,...extras]);throw registryFailure??error;")
    result["nodes"] = nodes

    test = replace(before["test"], 'type Mode="ok"|"fail"|"silent"|"verdict";', '''const rootOfNewOwner="owners/"+"a".repeat(180)+"/rust";
const ownerRegistryFailure="Native dependency has no build project owner: "+rootOfNewOwner+" (required by p:build; candidates=4788)";
type Mode="ok"|"fail"|"registry"|"silent"|"verdict";''')
    test = replace(test, '   else setImmediate(()=>this.emit("message",{type:"result"', '   else if(this.mode==="registry")setImmediate(()=>this.emit("message",{type:"failure",id:message.id,name:"Error",code:"BUILD_SOURCE_AUTHORITY",message:ownerRegistryFailure}));\n   else setImmediate(()=>this.emit("message",{type:"result"')
    test = replace(test, 'name:"Error",message:"the child broke on it"', 'name:"Error",code:"BUILD_SOURCE_AUTHORITY",message:"the child broke on it"')
    test = replace(test, "mkdtempSync,readFileSync,realpathSync,rmSync", "mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync")
    test = replace(test, 'import {inferenceRecordIdentityV1}', '''import {settleOwnerRegistryV1} from "../../../../../📥️load/🧭️registry-follow/🟦️.ts";
import {withImplementationPinV1} from "../../../../../../📥️inputs/🔏️revision/🟨️.mjs";
import {inferenceRecordIdentityV1}''')
    test += '''
test("a farm reports the full native owner fault at its first reply and the graph follows the new registry",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"registry-farm-")));roots.push(root);
 const registry=join(root,"📋️build-owners.json"),put=(owners:object[])=>writeFileSync(registry,JSON.stringify({version:1,owners,catalogProviders:[]}));
 put([]);
 const log:string[]=[],signal=new AbortController();
 const farm=open("registry",log,[],[],{signal:signal.signal,onOwnerRegistryFailure:(error:Error)=>signal.abort(error)});
 let reads=0;
 try{
  await expect(withImplementationPinV1(()=>settleOwnerRegistryV1(root,async()=>{
   if(++reads===2)return "sealed";
   put([{name:"new",root:rootOfNewOwner,manifest:rootOfNewOwner+"/Cargo.toml",packages:[]}]);
   return farm.prepare(job);
  }))).resolves.toBe("sealed");
  expect(log).toEqual(["run"]);expect(reads).toBe(2);expect(farm.exists()).toBe(false);
  expect(signal.signal.reason).toMatchObject({code:"BUILD_SOURCE_AUTHORITY",message:ownerRegistryFailure});
 }finally{await farm.close();}
},20000);

test("a farm reports a native owner fault before retrying a verification",async()=>{
 const log:string[]=[],farm=open("registry",log,[]);
 try{
  await expect(farm.observe({name:"p",observe:true} as never)).rejects.toMatchObject({code:"BUILD_SOURCE_AUTHORITY",message:ownerRegistryFailure});
  expect(log).toEqual(["observe"]);
 }finally{await farm.close();}
},20000);
'''
    result["test"] = test
    return result

ticket = Path(__file__).resolve().parent.parent
fleet = ticket.parent.parent / "☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
lock = fleet / "landing.lock"
test_files = [
    files["test"],
    farm / "🧪️tests/🗃️child-custody/🟦️.ts",
    farm / "🧪️tests/🏘️hub/🟦️.ts",
    farm / "🧪️tests/🌊️wave-tally/🟦️.ts",
    *(build / "🧪️tests/🕸️graph" / suite / "🟦️.ts" for suite in ("🧭️registry-follow","🕓️pending-crates","🧲️adopt-registry","🩹️repair-rounds","🌊️seal-under-churn")),
]
def publish(path, current, value, scratch):
    transient = scratch / "source"
    with transient.open("x", encoding="utf-8", newline="") as stream:
        stream.write(value)
    transient.chmod(path.stat().st_mode & 0o777)
    assert not path.is_symlink() and path.read_bytes() == current.encode("utf-8"), str(path)
    os.replace(transient, path)

def land(before, after):
    start = datetime.now(timezone.utc)
    assert start.minute % 30 < 5, "Outside the 00–05 and 30–35 UTC landing windows"
    generated = ticket / "🗑️generated"
    generated.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="r233-farm-", dir=generated) as directory:
        temporary = Path(directory)
        lock.mkdir()
        owner = lock / "owner"
        claim = f"codex-R233-farm graph-owner-registry-follow {start.isoformat()}\n".encode()
        changed = []
        try:
            owner.write_bytes(claim)
            assert all(path.read_bytes() == before[name].encode("utf-8") for name,path in files.items())
            for name,path in files.items():
                publish(path,before[name],after[name],temporary)
                changed.append(name)
            env = dict(os.environ)
            for key in ("SEMIO_FLEET_GROUP", "SEMIO_FLEET_NFS_HOST", "SEMIO_FLEET_NETWORK", "SEMIO_REMOTE_CACHE_ENDPOINT", "SEMIO_REMOTE_CACHE_ACCESS", "SEMIO_REMOTE_CACHE_INSTANCE", "SEMIO_REMOTE_CACHE_TOKEN"):
                env.pop(key,None)
            env.update(BUN_RUNTIME_TRANSPILER_CACHE_PATH="0", TMPDIR=directory, TEMP=directory, TMP=directory)
            subprocess.run(["repo","tools","exec","bun","--","test",*("./"+str(path) for path in test_files),"--timeout","180000"],env=env,check=True,timeout=120)
            smoke = "await import(" + json.dumps("./" + str(graph / "🟦️.ts")) + "); console.log('graph import ok');"
            subprocess.run(["repo","tools","exec","bun","--","-e",smoke],env=env,check=True,timeout=30)
            for name,path in files.items():
                assert path.read_bytes() == after[name].encode("utf-8")
                print("[DEBUG]",path,hashlib.sha256(before[name].encode()).hexdigest(),"->",hashlib.sha256(after[name].encode()).hexdigest(),flush=True)
        except BaseException:
            for name in reversed(changed):
                if files[name].read_bytes() == after[name].encode("utf-8"):
                    publish(files[name],after[name],before[name],temporary)
            raise
        finally:
            if owner.exists() and owner.read_bytes() == claim:
                owner.unlink()
                lock.rmdir()
            print("[DEBUG] R233 farm landing ended",datetime.now(timezone.utc).isoformat(),flush=True)

parser = argparse.ArgumentParser()
parser.add_argument("command", choices=["diff", "overlay", "land"])
parser.add_argument("--base", action="store_true")
args = parser.parse_args()
before = {name:path.read_text(encoding="utf-8") for name,path in files.items()}
after = updated(before)
if args.command == "land":
    land(before,after)
elif args.command == "diff":
    for name,path in files.items():
        print("".join(difflib.unified_diff(
            before[name].splitlines(keepends=True),after[name].splitlines(keepends=True),
            str(Path("a")/path),str(Path("b")/path),
        )),end="")
else:
    selected = {"test":after["test"]} if args.base else after
    targets = {str(files[name].resolve()):source for name,source in selected.items()}
    plugin = '''import { plugin } from "bun";
const files = new Map(Object.entries(FILES));
plugin({
 name: "R233 farm staged test",
 setup(builder) {
  for(const[path,contents]of files)builder.onLoad({filter:new RegExp("^"+RegExp.escape(path)+"$")},()=>path.endsWith(".json")?{loader:"object",exports:{default:JSON.parse(contents)}}:{contents,loader:path.endsWith(".mjs")?"js":"ts"});
 }
});
'''
    print(plugin.replace("FILES",json.dumps(targets,ensure_ascii=False)))
