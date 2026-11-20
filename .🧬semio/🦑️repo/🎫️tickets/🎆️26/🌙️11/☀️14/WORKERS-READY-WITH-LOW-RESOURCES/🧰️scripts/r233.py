from pathlib import Path
import argparse
from datetime import datetime, timezone
import difflib
import hashlib
import json
import os
import subprocess
import tempfile
import time

build = Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build")
graph = build / "🕸️graph"
files = {
    "schema": graph / "🧬️schema/🔣️.json",
    "test": build / "🧪️tests/🕸️graph/🧭️registry-follow/🟦️.ts",
    "helper": graph / "📥️load/🧭️registry-follow/🟦️.ts",
    "authority": graph / "🔐️authority/🟦️.ts",
    "load": graph / "📥️load/🟦️.ts",
}

def replace(source, old, new):
    if source.count(old) != 1:
        raise ValueError("Expected one anchor: " + old)
    return source.replace(old, new, 1)

def updated(before):
    after = dict(before)
    schema = json.loads(before["schema"])
    assert "OwnerRegistryRefusal" not in schema["definitions"]
    schema["definitions"]["OwnerRegistryRefusal"] = {
        "description": "EN: A graph read could not resolve a native dependency after following the owner registry. DE: Der Graph konnte eine native Abhängigkeit nach dem Abgleich mit dem Eigentümerregister nicht auflösen.",
        "const": {
            "code": "BUILD_OWNER_REGISTRY_INCONSISTENT",
            "en": "The build owner registry is inconsistent for {roots} after {rounds} follow-up rounds (registry moved: {moved}). Run `repo run workspace:verify -- build-owners sync`, then read again. Cause: {cause}",
            "de": "Das Eigentümerregister des Builds ist nach {rounds} Folgerunden für {roots} inkonsistent (Register geändert: {moved}). `repo run workspace:verify -- build-owners sync` ausführen und erneut lesen. Ursache: {cause}",
        },
    }
    assert json.dumps(json.loads(before["schema"]), ensure_ascii=False, indent=2) + "\n" == before["schema"]
    after["schema"] = json.dumps(schema, ensure_ascii=False, indent=2) + "\n"

    test = replace(before["test"], 'import {settleOwnerRegistryV1}', '''import {Database} from "bun:sqlite";
import {withImplementationPinV1} from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import {readBuildOwnerRegistrySourceV1} from "../../../🕸️graph/📥️inference/🟨️.mjs";
import {settleOwnerRegistryV1}''')
    test = replace(test, 'test("an owner a peer registered after the enumeration is followed, not an internal error",async()=>{', 'for(const pinned of [false,true])test(`follows an owner registered after enumeration (inherited pin: ${pinned})`,async()=>{')
    test = replace(test, '''  const value=await settleOwnerRegistryV1(l.root,async()=>{calls++;seen.push(l.registry());if(calls===1){l.put([row("@t/a","a/rust"),row("@t/new","m/rust")]);throw Error(refusal("m/rust"));}return "sealed";});''', '''  const updatedRegistry=[row("@t/a","a/rust"),row("@t/new","m/rust")];
  const read=()=>settleOwnerRegistryV1(l.root,async()=>{
   calls++;
   seen.push(JSON.parse(readBuildOwnerRegistrySourceV1(l.root)).owners.map((owner:{root:string})=>owner.root));
   if(calls===1){l.put(updatedRegistry);throw Error(refusal("m/rust"));}
   return "sealed";
  });
  const value=await (pinned?withImplementationPinV1(read):read());
  const reference=new Database(":memory:");
  try{
   const expected=reference.query("select json_group_array(json_extract(value, '$.root')) as roots from json_each(?1)").get(JSON.stringify(updatedRegistry)) as {roots:string};
   expect(JSON.stringify(seen[1])).toBe(expected.roots);
  }finally{reference.close();}''')
    after["test"] = test

    helper = replace(before["helper"], 'import {reconcileBuildOwnerRegistry}', '''import {join} from "node:path";
import {clearPinnedAuthorityV1} from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import {readPhysicalSource} from "../../📥️inference/📍️coordinates/🟨️.mjs";
import {BUILD_OWNER_REGISTRY_FILE,reconcileBuildOwnerRegistry}''')
    helper = replace(helper, '  const producer=await import("../../📥️inference/🟨️.mjs"),source=()=>String(producer.readBuildOwnerRegistrySourceV1(root));', '''  const {readBuildOwnerRegistrySourceV1}=await import("../../📥️inference/🟨️.mjs");
  const registryPath=join(root,BUILD_OWNER_REGISTRY_FILE);''')
    helper = replace(helper, '    const before=source();', '    const before=readBuildOwnerRegistrySourceV1(root);')
    helper = replace(helper, '''      let adopted=false;try{adopted=adoptPendingBuildOwnersV1(root,message);}catch{}
      const moved=source()!==before;
      if(!adopted&&!moved||round>=GRAPH_LIMITS_V1.repairRounds)throw ownerRegistryRefusalV1({roots,rounds:round,moved,cause:error});''', '''      const registryMoved=()=>readPhysicalSource(registryPath).toString("utf8")!==before;
      let moved=registryMoved();
      if(round>=GRAPH_LIMITS_V1.repairRounds)throw ownerRegistryRefusalV1({roots,rounds:round,moved,cause:error});
      let adopted=false;
      if(!moved){
        try{adopted=adoptPendingBuildOwnersV1(root,message);}catch{}
        if(!adopted)moved=registryMoved();
      }
      if(!moved&&!adopted)throw ownerRegistryRefusalV1({roots,rounds:round,moved,cause:error});
      clearPinnedAuthorityV1();''')
    helper = replace(helper, 'import {GRAPH_LIMITS_V1,graphDeclarationV1} from "../../🔐️authority/🟦️.ts";', '''import {GRAPH_LIMITS_V1} from "../../🔐️authority/🟦️.ts";
import graphSchema from "../../🧬️schema/🔣️.json" with{type:"json"};''')
    helper = replace(helper, 'graphDeclarationV1("🧬️schema/🔣️.json").definitions.OwnerRegistryRefusal.const as {code:string;en:string;de:string}', 'graphSchema.definitions.OwnerRegistryRefusal.const')
    start = helper.index("/** 🧭️ Runs one read attempt")
    end = helper.index("export async function settleOwnerRegistryV1", start)
    helper = helper[:start] + "/** 🧭️ Retries a refused native-owner lookup when the registry changed or reconciliation repaired the named roots. */\n" + helper[end:]
    after["helper"] = helper

    after["authority"] = replace(before["authority"], 'return ["../🟦️.ts",...[', 'return ["../🟦️.ts","../📥️load/🧭️registry-follow/🟦️.ts",...[')

    load = replace(before["load"], 'import {reconcileBuildOwnerRegistry} from "../🧾️registry-consistency/🟦️.ts";', 'import {settleOwnerRegistryV1} from "./🧭️registry-follow/🟦️.ts";')
    start = load.index('/** 💾️ Publishes only a current graph')
    end = load.index('/** 🔪️ Whether a selection',start)
    assert load[start:end].count('export async function loadBuildGraphV1') == 1
    after["load"] = load[:start] + '''export {pendingOwnerRootsV1,vanishedOwnerRootsV1,adoptPendingBuildOwnersV1} from "./🧭️registry-follow/🟦️.ts";

/** 🕸️ Loads the graph and follows the owner registry when inference started before a peer added, moved or removed a native owner.
 * @see ./🧭️registry-follow/🟦️.ts */
export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  const attempt=()=>loadBuildGraphOnceV1(root,invocation,phaseControl,options);
  if(options.reuse)return attempt();
  return settleOwnerRegistryV1(root,attempt,{aborted:()=>invocation?.control.signal.aborted===true,say:line=>{process.stderr.write(line);}});
}


''' + load[end:]
    return after

ticket = Path(__file__).resolve().parent.parent
fleet = ticket.parent.parent / "☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
lock = fleet / "landing.lock"
stale_owner = b'claude-coordinator-R219 whole-generation-fleet-gate 2026-11-19T01:10:04Z\n'
suites = ["🧭️registry-follow", "🕓️pending-crates", "🧲️adopt-registry", "🩹️repair-rounds", "🌊️seal-under-churn"]

def digest(value):
    return hashlib.sha256(value.encode("utf-8")).hexdigest()

def lock_metadata():
    owner = lock / "owner"
    assert not lock.is_symlink() and lock.is_dir()
    assert list(lock.iterdir()) == [owner] and not owner.is_symlink() and owner.is_file()
    status = owner.stat()
    return owner.read_bytes(), status.st_ino, status.st_mtime_ns

def recover_lock(temporary):
    snapshot = lock_metadata()
    owner, _, modified = snapshot
    assert owner == stale_owner and time.time() - modified / 1e9 >= 40 * 60
    if os.name != "posix":
        raise ValueError("Verify the old coordinator process on its POSIX origin before re-acquiring")
    processes = subprocess.check_output(["ps", "-axo", "pid,command"], text=True)
    active = [line for line in processes.splitlines() if "land2.sh" in line or "land-ack.py" in line or "R219/land" in line]
    assert not active, "A landing process is still alive"
    assert lock_metadata() == snapshot
    os.rename(lock, temporary / "previous-landing.lock")
    assert (temporary / "previous-landing.lock/owner").read_bytes() == stale_owner
    print("[DEBUG] Preserved stale owner", owner.decode().strip(), flush=True)

def publish(path, expected, value, temporary):
    assert path.is_file() and not path.is_symlink()
    pending = temporary / "source"
    with pending.open("x", encoding="utf-8", newline="") as stream:
        stream.write(value)
    pending.chmod(path.stat().st_mode & 0o777)
    assert path.read_bytes() == expected.encode("utf-8"), str(path)
    os.replace(pending, path)

def land(before, after):
    start = datetime.now(timezone.utc)
    assert start.minute % 30 < 5, "Outside the 00–05 and 30–35 UTC landing windows"
    generated = ticket / "🗑️generated"
    generated.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="r233-", dir=generated) as directory:
        temporary = Path(directory)
        if lock.exists():
            recover_lock(temporary)
        lock.mkdir()
        owner = lock / "owner"
        claim = f"codex-R233 graph-owner-registry-follow {start.isoformat()}".encode() + bytes([10])
        changed = []
        try:
            with owner.open("xb") as stream:
                stream.write(claim)
            assert all(path.read_bytes() == before[name].encode("utf-8") for name, path in files.items())
            for name, path in files.items():
                publish(path, before[name], after[name], temporary)
                changed.append(name)
            env = dict(os.environ)
            for key in ("SEMIO_FLEET_GROUP", "SEMIO_FLEET_NFS_HOST", "SEMIO_FLEET_NETWORK", "SEMIO_REMOTE_CACHE_ENDPOINT", "SEMIO_REMOTE_CACHE_ACCESS", "SEMIO_REMOTE_CACHE_INSTANCE", "SEMIO_REMOTE_CACHE_TOKEN"):
                env.pop(key, None)
            env.update(BUN_RUNTIME_TRANSPILER_CACHE_PATH="0", TMPDIR=directory, TEMP=directory, TMP=directory)
            tests = ["./" + str(build / "🧪️tests/🕸️graph" / suite / "🟦️.ts") for suite in suites]
            subprocess.run(["repo", "tools", "exec", "bun", "--", "test", *tests, "--timeout", "180000"], env=env, check=True, timeout=120)
            smoke = "await import(" + json.dumps("./" + str(graph / "🟦️.ts")) + "); console.log('graph import ok');"
            subprocess.run(["repo", "tools", "exec", "bun", "--", "-e", smoke], env=env, check=True, timeout=30)
            assert all(path.read_bytes() == after[name].encode("utf-8") for name, path in files.items())
            for name, path in files.items():
                print(f"[DEBUG] {path} {digest(before[name])} -> {digest(after[name])}", flush=True)
        except BaseException:
            for name in reversed(changed):
                if files[name].read_bytes() == after[name].encode("utf-8"):
                    publish(files[name], after[name], before[name], temporary)
            raise
        finally:
            if owner.exists() and owner.read_bytes() == claim:
                owner.unlink()
                lock.rmdir()
            print("[DEBUG] R233 landing ended", datetime.now(timezone.utc).isoformat(), flush=True)

parser = argparse.ArgumentParser()
parser.add_argument("command", choices=["diff", "overlay", "land"])
parser.add_argument("--base", action="store_true")
args = parser.parse_args()
before = {name: path.read_text(encoding="utf-8") for name, path in files.items()}
after = updated(before)
if args.command == "land":
    land(before, after)
elif args.command == "diff":
    for name, path in files.items():
        print("".join(difflib.unified_diff(
            before[name].splitlines(keepends=True), after[name].splitlines(keepends=True),
            str(Path("a") / path), str(Path("b") / path),
        )), end="")
else:
    if args.base:
        after["helper"] = replace(before["helper"], 'import {GRAPH_LIMITS_V1,graphDeclarationV1} from "../../🔐️authority/🟦️.ts";', 'import {GRAPH_LIMITS_V1} from "../../🔐️authority/🟦️.ts";\nimport graphSchema from "../../🧬️schema/🔣️.json" with{type:"json"};')
        after["helper"] = replace(after["helper"], 'graphDeclarationV1("🧬️schema/🔣️.json").definitions.OwnerRegistryRefusal.const as {code:string;en:string;de:string}', 'graphSchema.definitions.OwnerRegistryRefusal.const')
    targets = {str(files[name].resolve()): source for name, source in after.items()}
    plugin = '''import { plugin } from "bun";
const files = new Map(Object.entries(FILES));
plugin({
 name: "R233 staged test",
 setup(builder) {
  for(const[path,contents]of files)builder.onLoad({filter:new RegExp("^"+RegExp.escape(path)+"$")},()=>path.endsWith(".json")?{loader:"object",exports:{default:JSON.parse(contents)}}:{contents,loader:"ts"});
 }
});
'''
    print(plugin.replace("FILES",json.dumps(targets,ensure_ascii=False)))
