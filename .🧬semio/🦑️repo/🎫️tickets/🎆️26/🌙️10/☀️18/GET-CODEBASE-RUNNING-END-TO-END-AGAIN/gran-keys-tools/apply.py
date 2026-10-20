#!/usr/bin/env python3
# 🚀 Applies the gran-keys set to the live repo, all or nothing: creates the Cargo resolution identity module (engine, schema, policy, tests, fixture, Python oracle) and edits the lock reader, the task-environment digest, the cache policy and its schema, the project input inference, the graph assembly, the Bazel closure adapter and its tests, the cache-correctness law and its corpus, the workspace named inputs and the authored manifests, each edit anchored on the CURRENT live text (exactly one occurrence, else refused as drift). Dry run by default; --apply writes; --undo restores. Environment: GK_ROOT (repository), GK_LIBRARY (library), GK_STAGED (staged files).
import hashlib, json, os, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from layout import ROOT, L, B, NEW
import patches_core as core, patches_bazel as bazel, patches_law as law, patches_named as named, patches_manifests as manifests, patches_precision as precision
STAGED = os.environ.get("GK_STAGED") or os.path.join(HERE, "staged")
BACKUP = os.path.normpath(HERE + "/../🗑️generated/gran-keys/apply-backup-" + hashlib.sha256(L.encode()).hexdigest()[:8]) if "🧬semio" in HERE else HERE + "/backup"
BZ = B + "/🔧️tools/🏗️bazel"
LAW = L + "/⚖️laws/🔒️cache-correctness"
EDITS = [
    (B + "/📥️inputs/🔒️lock/📥️read/🟨️.mjs", core.lock_read), (B + "/📥️inputs/🔒️lock/📥️read/🟨️.d.mts", core.lock_dts), (B + "/📥️inputs/🧮️digest/🟨️.mjs", core.digest),
    (L + "/⚡️caching/🔣️policy.json", core.policy), (L + "/⚡️caching/🧬️schema/🔣️.json", core.policy_schema),
    (B + "/📥️inputs/🟨️.mjs", core.inputs_mjs), (B + "/📥️inputs/💾️storage/🧬️schema/🔣️.json", core.storage_schema), (B + "/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json", core.graph_fixture), (B + "/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs", core.assembly), (B + "/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs", precision.assembly_markers), (B + "/📥️inputs/🎯️precision/🟦️.ts", precision.engine), (B + "/📥️inputs/🎯️precision/🧪️tests/🟦️.ts", precision.test),
    (BZ + "/🔗️closure/🟦️.ts", bazel.closure), (BZ + "/🔗️closure/🧬️schema/🔣️.json", bazel.closure_schema), (BZ + "/📜️script.ts", bazel.script),
    (BZ + "/🧪️tests/📥️closure/🟦️.ts", bazel.closure_test), (BZ + "/🧪️tests/📥️fingerprint/🟦️.ts", bazel.fingerprint_test), (BZ + "/🧪️tests/📥️closure/🔣️.json", bazel.closure_corpus_text),
    (LAW + "/🟦️.ts", law.law), (LAW + "/🧪️tests/🔒️rules/🧬️schema/🔣️.json", law.schema), (LAW + "/🧪️tests/🔒️rules/🟦️.ts", law.law_test), (LAW + "/🧪️tests/🔒️rules/🔣️.json", law.corpus),
    (B + "/🔣️policy.json", named.named), (ROOT + "/nx.json", named.named),
    (ROOT + "/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json", named.proctor),
    (ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust/📋️project.json", manifests.flow_browser),
    (ROOT + "/🌎️hub/📦️packages/🦀️rust/📋️project.json", manifests.os_hub),
]
def sha(data): return hashlib.sha256(data).hexdigest()
def rel(path): return os.path.relpath(path, ROOT)
def read(path):
    try: return open(path, "rb").read()
    except FileNotFoundError: return None
def project_manifests():
    try:
        out = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", "-z", "*📋️project.json"], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf8")
        return sorted(p for p in out.split("\0") if p and not p.startswith(".🧬semio/"))
    except Exception:
        found = []
        for base, dirs, files in os.walk(ROOT):
            dirs[:] = [d for d in dirs if d not in ("node_modules", ".git") and not d.startswith(".")]
            found += [os.path.relpath(os.path.join(base, f), ROOT) for f in files if f == "📋️project.json"]
        return sorted(found)
def plan():
    refused, ops = [], []
    for flat, dest in NEW.items():
        data = open(STAGED + "/" + flat, "rb").read(); current = read(dest)
        if current != data: ops.append({"path": dest, "before": current, "after": data})
    edits = list(EDITS)
    for manifest in project_manifests():
        path = ROOT + "/" + manifest
        text = read(path)
        if text is not None and b'"cargoToolchain": [' in text: edits.append((path, manifests.drop_cargo_toolchain))
    chains = {}
    for path, edit in edits: chains.setdefault(path, []).append(edit)
    for path, fns in chains.items():
        current = read(path)
        if current is None: refused.append("missing: " + rel(path)); continue
        try:
            text, changed = current.decode("utf8"), False
            for edit in fns:
                text, c = edit(text); changed = changed or c
        except SystemExit as error: refused.append("%s: %s" % (rel(path), error)); continue
        if changed: ops.append({"path": path, "before": current, "after": text.encode("utf8")})
    return ops, refused
def replace(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    handle, temporary = tempfile.mkstemp(dir=os.path.dirname(path), prefix=".gk-")
    with os.fdopen(handle, "wb") as out: out.write(data)
    os.chmod(temporary, os.stat(path).st_mode & 0o7777 if os.path.exists(path) else 0o644)
    os.replace(temporary, path)
def restore(done):
    for op in reversed(done):
        if op["before"] is None: os.path.exists(op["path"]) and os.remove(op["path"])
        else: replace(op["path"], op["before"])
def undo():
    manifest = json.load(open(BACKUP + "/manifest.json"))
    for row in manifest:
        current = read(row["path"])
        if current is None or sha(current) != row["after"]: sys.exit("REFUSED: changed since apply: " + rel(row["path"]))
    for row in reversed(manifest):
        if row["backup"] is None: os.remove(row["path"])
        else: replace(row["path"], open(BACKUP + "/" + row["backup"], "rb").read())
    print("undone", len(manifest), "files")
def main():
    if "--undo" in sys.argv: return undo()
    ops, refused = plan()
    created = sum(1 for op in ops if op["before"] is None); manifests_n = sum(1 for op in ops if op["path"].endswith("📋️project.json"))
    print("gran-keys set: %d file operation(s): %d created, %d edited (%d project manifests)" % (len(ops), created, len(ops) - created, manifests_n))
    if "--list" in sys.argv:
        for op in ops: print("  ", "create" if op["before"] is None else "edit  ", rel(op["path"]))
    if refused:
        print("REFUSED (nothing written):"); [print("  -", row) for row in refused]; sys.exit(1)
    if "--apply" not in sys.argv: return print("dry run: nothing written; rerun with --apply (add --list to name every file)")
    os.makedirs(BACKUP, exist_ok=True)
    manifest, done = [], []
    try:
        for op in ops:
            if read(op["path"]) != op["before"]: raise RuntimeError("changed during apply: " + rel(op["path"]))
        for index, op in enumerate(ops):
            name = None if op["before"] is None else "%03d.bak" % index
            if name: open(BACKUP + "/" + name, "wb").write(op["before"])
            replace(op["path"], op["after"]); done.append(op)
            manifest.append({"path": op["path"], "backup": name, "after": sha(op["after"])})
        json.dump(manifest, open(BACKUP + "/manifest.json", "w"), ensure_ascii=False, indent=1)
    except Exception as error:
        restore(done); sys.exit("REFUSED, all changes rolled back: %s" % error)
    print("applied. Next: SEMIO_RESOLUTION_REPO=1 bun test the resolution, digest, bazel closure and cache-correctness suites, then `bun repo show projects`.")
main()
