"""🚚 Computes every shared-file edit from the live tree and applies the staged carve in one guarded step (dry mode writes to a scratch tree)."""
import os, sys, json, shutil, subprocess, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rcrates as C
import rregistry as G, redits as E

DRY = SP + "/applydry/"
FWC = "🧰️framework/Cargo.toml"
SHARED = {
    "framework_cargo": FWC, "root_cargo": "Cargo.toml", "owners": "📋️build-owners.json", "launch": ".vscode/launch.json", "seed": ".vscode/🧩️launch.seed.jsonc",
    "facade_script": FACADE_PKG + "/📜️script.ts", "facade_project": FACADE_PKG + "/📋️project.json",
    "own_ts": R + "🚪️io/🧪️tests/🏛️ownership/🟦️.ts", "own_fixture": R + "🚪️io/🧪️tests/🧫️fixtures/🏛️ownership/🔣️.json", "own_schema": R + "🚪️io/🧬️schema/🏛️ownership/🔣️.json",
    "framework_lock": "🧰️framework/Cargo.lock", "root_lock": "Cargo.lock",
    "source_fixture": FW + "🎒️pack/🚰️source/🧪️tests/🔣️.json",
    "ingress_ts": R + "📡️wire/🎮️command/📥️ingress/🏛️ownership/🧪️tests/🟦️.ts", "ingress_fixture": R + "📡️wire/🎮️command/📥️ingress/🧪️tests/🧫️fixtures/🔣️.json",
}
LOCKS = ["🎓️teaching/Cargo.lock", "✏️s/Cargo.lock", "🌎️hub/Cargo.lock", "✏️s/🔌️plugins/🀄️wfc/Cargo.lock", "✏️s/🔌️plugins/🧩️puzzle/Cargo.lock", "✏️s/🧑‍💻dev/🧩️composition/Cargo.lock", "✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock", "✏️s/🧑‍💻dev/🌊️flow/Cargo.lock"]

def read_live(rel): return open(REPO + rel, encoding="utf8").read()

def profile_manifests():
    out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-l", "-F", "wasm-prototype.package.semio-framework-replication]", "--", "*Cargo.toml", ":!.🧬semio"], cwd=REPO, capture_output=True, text=True).stdout
    return [l for l in out.split("\n") if l.strip()]

def compute():
    live = {k: read_live(v) for k, v in SHARED.items()}
    out = {}
    fw = G.members_edit(live["framework_cargo"])
    out["framework_cargo"] = G.profile_edit(fw)
    out["root_cargo"] = G.profile_edit(live["root_cargo"])
    out["owners"] = G.owners_edit(live["owners"])
    base = G.pick_order_base([live["launch"], live["seed"]])
    law_order = G.pick_law_order([live["launch"], live["seed"]])
    anchor = "🦀️ semio-framework-pack-io 🧪️test 🚀️release"
    out["launch"] = G.launch_edit(live["launch"], anchor, base, law_order)
    out["seed"] = G.launch_edit(live["seed"], anchor, base, law_order)
    json.loads(out["launch"])
    out["facade_script"] = E.facade_script(live["facade_script"])
    out["facade_project"] = E.facade_project(live["facade_project"])
    out["own_ts"] = E.ownership_ts(live["own_ts"])
    out["own_fixture"] = E.ownership_fixture(live["own_fixture"])
    out["own_schema"] = E.ownership_schema(live["own_schema"])
    out["source_fixture"] = E.source_fixture(live["source_fixture"])
    out["ingress_ts"] = E.ingress_test(live["ingress_ts"])
    out["ingress_fixture"] = E.ingress_fixture(live["ingress_fixture"], json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/ingress_closure.json", encoding="utf8")))
    out["framework_lock"] = G.apply_lock(live["framework_lock"], G.new_blocks(True))
    out["root_lock"] = G.apply_lock(live["root_lock"], G.new_blocks(False))
    extra = {}
    for rel in profile_manifests():
        if rel in (FWC, "Cargo.toml"): continue
        t = read_live(rel)
        n = G.profile_edit(t)
        if n != t: extra[rel] = (t, n)
    locks = {}
    for rel in LOCKS:
        t = read_live(rel)
        try:
            n = G.apply_lock(t, G.new_blocks(False))
            if n != t: locks[rel] = (t, n)
        except AssertionError as e:
            print("lock skipped", rel, e)
    return live, out, extra, locks, base, law_order

def stage_files():
    res = []
    for dp, dn, fn in os.walk(STAGE):
        for f in fn: res.append(os.path.relpath(os.path.join(dp, f), STAGE))
    return sorted(res)

def guard():
    bad = []
    for rel in stage_files():
        if rel.endswith(".deleted"):
            r = rel[:-len(".deleted")]
            if os.path.exists(BASE + r) and (not os.path.exists(REPO + r) or open(REPO + r, "rb").read() != open(BASE + r, "rb").read()): bad.append("delete-drift:" + r)
            continue
        live = REPO + rel
        if os.path.exists(BASE + rel):
            if not os.path.exists(live) or open(live, "rb").read() != open(BASE + rel, "rb").read(): bad.append(rel)
        elif os.path.exists(live):
            bad.append("exists:" + rel)
    return bad

def main(mode):
    live, out, extra, locks, base, law_order = compute()
    if mode == "dry":
        shutil.rmtree(DRY, ignore_errors=True)
        for k, v in out.items():
            p = DRY + SHARED[k]; os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf8").write(v)
        for rel, (a, b) in list(extra.items()) + list(locks.items()):
            p = DRY + rel; os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf8").write(b)
        print("dry ok; order bases", base, law_order, "; shared changed:", [SHARED[k] for k in out if out[k] != live[k]], "; profile manifests", len(extra), "; locks", len(locks))
        r = subprocess.run(["bun", "-e", "const fs=require('fs');for(const f of process.argv.slice(1)){Bun.JSONC.parse(fs.readFileSync(f,'utf8'))};console.log('launch+seed parse')", DRY + SHARED["launch"], DRY + SHARED["seed"]], capture_output=True, text=True)
        print(r.stdout.strip(), r.stderr[:300])
        return
    bad = guard()
    if bad:
        print("REFUSING: base drift", bad[:30]); sys.exit(1)
    for rel in stage_files():
        dst = REPO + rel[:-len(".deleted")] if rel.endswith(".deleted") else REPO + rel
        if rel.endswith(".deleted"):
            if os.path.exists(dst): os.remove(dst)
            continue
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(STAGE + rel, dst)
    live2, out2, extra2, locks2, _, _ = compute() if False else (live, out, extra, locks, None, None)
    for k, v in out.items():
        cur = read_live(SHARED[k])
        if cur != live[k]:
            print("shared file changed during apply, recomputing", SHARED[k]); l2, o2, e2, k2, _, _ = compute(); v = o2[k]
        open(REPO + SHARED[k], "w", encoding="utf8").write(v)
    for rel, (a, b) in list(extra.items()) + list(locks.items()):
        if read_live(rel) != a: print("changed during apply, skipped", rel); continue
        open(REPO + rel, "w", encoding="utf8").write(b)
    print("applied", len(stage_files()), "staged files,", len(out), "shared edits,", len(extra), "profile manifests,", len(locks), "locks")

if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "dry")
