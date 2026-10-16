"""🚚 Applies the staged carve to the live tree in one step (guarded: refuses when a base file changed under us)."""
import os, sys, json, shutil, hashlib, subprocess, re
from lib import *
from crates import *
import registry, edits, manifests, lock

DRY = SP + "/applydry/"
SHARED = {
    "framework_cargo": "🧰️framework/Cargo.toml",
    "root_cargo": "Cargo.toml",
    "owners": "📋️build-owners.json",
    "launch": ".vscode/launch.json",
    "seed": ".vscode/🧩️launch.seed.jsonc",
    "facade_script": P + "📦️packages/🦀️rust/📜️script.ts",
    "record_script": P + "🌱️value/📦️packages/🦀️rust/📜️script.ts",
    "root_script": "📜️script.ts",
    "db_test": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-db-io-direct-writer/🟦️.ts",
    "cause": P + "⚠️error/🧪️tests/🧫️fixtures/🧭️cause/🔣️.json",
    "ownership_ts": P + "🌱️value/🧪️tests/🧱️ownership/🟦️.ts",
    "ownership_json": P + "🌱️value/🧪️tests/🧫️fixtures/🧩️ownership/🔣️.json",
    "record_project": P + "🌱️value/📦️packages/🦀️rust/📋️project.json",
    "framework_lock": "🧰️framework/Cargo.lock",
    "root_lock": "Cargo.lock",
}

def read_live(rel): return open(REPO + rel, encoding="utf8").read()

def ownership_edits(ts, js):
    d = json.loads(js)
    rec_manifest = read_stage_or_base(pkgdir("record") + "/Cargo.toml")
    deps = []
    sect = None
    for l in rec_manifest.split("\n"):
        if l.startswith("["): sect = l.strip()
        elif sect == "[dependencies]" and "=" in l: deps.append(l.split("=")[0].strip())
    new = {"format": 1, "package": cname("record"), "facade": "semio-framework-pack", "module": "record", "manifest": "📦️packages/🦀️rust/Cargo.toml",
           "library": {"path": "../../🦀️.rs"}, "facadeLibrary": "../../../📦️packages/🦀️rust/🦀️.rs", "dependencies": sorted(deps),
           "publicEntries": ["encode_document", "decode_document", "encode_record_body", "decode_record_body_exact"], "schemaEntries": ["schema_hash"],
           "forbiddenProviders": d["forbiddenProviders"]}
    js2 = json.dumps(new, ensure_ascii=False, indent=2) + "\n"
    old_ts = ts
    ts = ts.replace('type Ownership={package:string;module:string;manifest:string;library:{name:string;path:string};mount:string;dependencies:string[];publicEntries:string[];forbiddenProviders:string[]};',
                    'type Ownership={package:string;facade:string;module:string;manifest:string;library:{path:string};facadeLibrary:string;dependencies:string[];publicEntries:string[];schemaEntries:string[];forbiddenProviders:string[]};')
    ts = ts.replace('expect(contract.package).toBe("semio-framework-pack");expect(contract.module).toBe("record");', 'expect(contract.package).toBe("' + cname("record") + '");expect(contract.facade).toBe("semio-framework-pack");expect(contract.module).toBe("record");')
    ts = ts.replace('for(const entry of contract.publicEntries)expect(read("🦀️.rs")).toContain("pub fn "+entry+"(");',
                    'for(const entry of contract.publicEntries)expect(read("🦀️.rs")).toContain("pub fn "+entry+"(");\n for(const entry of contract.schemaEntries)expect(read("🧬️schema/🦀️.rs")).toContain("pub fn "+entry+"(");')
    ts = ts.replace('test("one actual Pack library mounts Record and exposes no product source authority",()=>{\n expect(contract.manifest).toBe("../📦️packages/🦀️rust/Cargo.toml");',
                    'test("one actual Pack record library owns the codec and the facade exposes it with no product source authority",()=>{\n expect(contract.manifest).toBe("📦️packages/🦀️rust/Cargo.toml");')
    ts = ts.replace(' const library=readFileSync(resolve(dirname(path),contract.library.path),"utf8");expect(library).toContain(\'#[path = "\'+contract.mount+\'"]\');expect(library).toContain("pub mod "+contract.module+";");\n expect(resolve(dirname(path),contract.mount)).toBe(resolve(owner,"🦀️.rs"));',
                    ' expect(resolve(dirname(path),contract.library.path)).toBe(resolve(owner,"🦀️.rs"));\n const facade=readFileSync(resolve(dirname(path),contract.facadeLibrary),"utf8");expect(facade).toContain("pub mod "+contract.module+" {");for(const entry of [...contract.publicEntries,...contract.schemaEntries])expect(facade).toContain(entry);')
    assert ts != old_ts and "contract.mount" not in ts, "ownership test edit anchors"
    return ts, js2

def compute():
    out = {}
    live = {k: read_live(v) for k, v in SHARED.items()}
    out["framework_cargo"] = registry.profile_edit(registry.members_edit(live["framework_cargo"]))
    out["root_cargo"] = registry.profile_edit(live["root_cargo"])
    out["owners"] = registry.owners_edit(live["owners"])
    base = registry.pick_order_base([live["launch"], live["seed"]])
    anchor = "🦀️ semio-framework-pets-traversal 🧪️test 🚀️release"
    out["launch"] = registry.launch_edit(live["launch"], anchor, base)
    out["seed"] = registry.launch_edit(live["seed"], anchor, base)
    json.loads(out["launch"])
    out["facade_script"] = edits.facade_script(live["facade_script"])
    out["record_script"] = edits.record_script(live["record_script"])
    out["root_script"] = edits.root_script(live["root_script"])
    out["db_test"] = edits.db_test(live["db_test"])
    out["cause"] = edits.cause_fixture(live["cause"])
    out["ownership_ts"], out["ownership_json"] = ownership_edits(live["ownership_ts"], live["ownership_json"])
    out["framework_lock"] = lock.apply_lock(live["framework_lock"], lock.new_blocks(True))
    out["root_lock"] = lock.apply_lock(live["root_lock"], lock.new_blocks(False))
    pj = json.loads(live["record_project"])
    ordered = {}
    for k in pj:
        ordered[k] = pj[k]
        if k == "sourceRoot": ordered["tags"] = ["scope:framework", "language:rust"]
    assert "tags" in ordered
    out["record_project"] = json.dumps(ordered, ensure_ascii=False, indent=2) + "\n"
    return live, out, base

def stage_files():
    res = []
    for dp, dn, fn in os.walk(STAGE):
        for f in fn:
            res.append(os.path.relpath(os.path.join(dp, f), STAGE))
    return sorted(res)

def guard():
    bad = []
    for rel in stage_files():
        if rel.endswith(".deleted"): continue
        live = REPO + rel
        if os.path.exists(BASE + rel):
            if not os.path.exists(live) or open(live, "rb").read() != open(BASE + rel, "rb").read(): bad.append(rel)
        elif os.path.exists(live):
            bad.append("exists:" + rel)
    return bad

def main(mode):
    live, out, base = compute()
    if mode == "dry":
        shutil.rmtree(DRY, ignore_errors=True)
        for k, v in out.items():
            p = DRY + SHARED[k]; os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf8").write(v)
        print("dry-run ok; order base", base, "; changed shared files:", [SHARED[k] for k in out if out[k] != live[k]])
        # validate parses
        for k in ("owners", "cause", "ownership_json", "record_project"): json.loads(out[k])
        r = subprocess.run(["bun", "-e", "const fs=require('fs');for(const f of process.argv.slice(1)){Bun.JSONC.parse(fs.readFileSync(f,'utf8'))};console.log('launch+seed parse')", DRY + SHARED["launch"], DRY + SHARED["seed"]], capture_output=True, text=True)
        print(r.stdout.strip(), r.stderr[:300])
        return
    bad = guard()
    if bad:
        print("REFUSING: base drift", bad[:20]); sys.exit(1)
    for rel in stage_files():
        if rel.endswith(".deleted"): continue
        dst = REPO + rel
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(STAGE + rel, dst)
    for k, v in out.items():
        cur = read_live(SHARED[k])
        if cur != live[k]:
            print("shared file changed during apply, recomputing", SHARED[k]); live2, out2, _ = compute(); v = out2[k]
        open(REPO + SHARED[k], "w", encoding="utf8").write(v)
    print("applied", len(stage_files()), "staged files and", len(out), "shared edits")

if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "dry")
