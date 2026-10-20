#!/usr/bin/env python3
# 🚀 Applies the staged input precision set to the live repo, all or nothing: creates the precision files (rules regenerated from the live manifests), wires the hook into project-assembly and adds the whole-repository-input rule to the cache-correctness law, each by an anchored edit of the CURRENT live text. Dry run by default; --apply writes; --undo restores; --strict also refuses any edited file whose hash moved since staging.
import hashlib, importlib.util, json, os, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from layout import NEW, L
def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path); module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module); return module
hook, law = load("hook_patch", HERE + "/hook-patch.py"), load("law_patch", HERE + "/law-patch.py")
BACKUP = os.path.normpath(HERE + "/../🗑️generated/input-precision/apply-backup-" + hashlib.sha256(L.encode()).hexdigest()[:8])
ASSEMBLY = L + "/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"
LAW = L + "/⚖️laws/🔒️cache-correctness"
EDITS = [(ASSEMBLY, hook.apply), (LAW + "/🟦️.ts", law.law), (LAW + "/🧪️tests/🔒️rules/🧬️schema/🔣️.json", law.schema), (LAW + "/🧪️tests/🔒️rules/🔣️.json", law.corpus)]
REGENERATED = ("rules.json", "fixture.json")
def sha(data): return hashlib.sha256(data).hexdigest()
def rel(path): return path[len(L) + 1:]
def read(path):
    try: return open(path, "rb").read()
    except FileNotFoundError: return None
def plan(strict):
    refused, ops = [], []
    if subprocess.run(["pgrep", "-f", "dependencies (sync|lock)"], capture_output=True).returncode == 0: refused.append("a dependency sync/lock is running")
    subprocess.run([sys.executable, HERE + "/gen-rules.py"], check=True, capture_output=True)
    bases = json.load(open(HERE + "/bases.json")) if os.path.exists(HERE + "/bases.json") else {}
    for flat, dest in NEW.items():
        data = open(HERE + "/staged/" + flat, "rb").read(); current = read(dest)
        if current == data: continue
        if current is not None and flat not in REGENERATED: refused.append("exists with other content: " + rel(dest))
        ops.append({"path": dest, "before": current, "after": data})
    for path, edit in EDITS:
        current = read(path)
        if current is None: refused.append("missing: " + rel(path)); continue
        if strict and rel(path) in bases and bases[rel(path)] != sha(current): refused.append("drifted since staging: " + rel(path))
        try: text, changed = edit(current.decode("utf8"))
        except SystemExit as error: refused.append("%s: %s" % (rel(path), error)); continue
        if changed: ops.append({"path": path, "before": current, "after": text.encode("utf8")})
    return ops, refused
def restore(done):
    for op in reversed(done):
        if op["before"] is None: os.path.exists(op["path"]) and os.remove(op["path"])
        else: replace(op["path"], op["before"])
def replace(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    handle, temporary = tempfile.mkstemp(dir=os.path.dirname(path), prefix=".ip-")
    with os.fdopen(handle, "wb") as out: out.write(data)
    os.chmod(temporary, os.stat(path).st_mode & 0o7777 if os.path.exists(path) else 0o644)
    os.replace(temporary, path)
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
    ops, refused = plan("--strict" in sys.argv)
    print("input precision set: %d file operation(s)" % len(ops))
    for op in ops: print("  ", "create" if op["before"] is None else "edit  ", rel(op["path"]))
    if refused:
        print("REFUSED (nothing written):"); [print("  -", row) for row in refused]; sys.exit(1)
    if "--apply" not in sys.argv: return print("dry run: nothing written; rerun with --apply")
    os.makedirs(BACKUP, exist_ok=True)
    manifest, done = [], []
    try:
        for index, op in enumerate(ops):
            if read(op["path"]) != op["before"]: raise RuntimeError("changed during apply: " + rel(op["path"]))
        for index, op in enumerate(ops):
            name = None if op["before"] is None else "%03d.bak" % index
            if name: open(BACKUP + "/" + name, "wb").write(op["before"])
            replace(op["path"], op["after"]); done.append(op)
            manifest.append({"path": op["path"], "backup": name, "after": sha(op["after"])})
        json.dump(manifest, open(BACKUP + "/manifest.json", "w"), ensure_ascii=False, indent=1)
    except Exception as error:
        restore(done); sys.exit("REFUSED, all changes rolled back: %s" % error)
    print("applied. Next: SEMIO_INPUT_PRECISION_REPO=1 bun test ./%s/🏗️build/📥️inputs/🎯️precision/🧪️tests/🟦️.ts, the cache-correctness law test, then `bun repo show projects`." % os.path.basename(L))
main()
