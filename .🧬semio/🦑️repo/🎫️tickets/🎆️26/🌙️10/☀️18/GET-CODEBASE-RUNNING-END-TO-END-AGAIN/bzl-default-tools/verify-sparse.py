"""Sparse overlay rehearsal of the whole plan: chain directories are real, every sibling is a symlink to the live tree (read only), edited files are copies.
Applies all groups, checks idempotence, JSON, syntax, runs the new pure tests, undoes and compares hashes, and proves the live tree was not written."""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.environ.get("SEMIO_ROOT", "/Users/ueli/Documents/semio")
OV = "/private/tmp/bzl-default/ov"
plan = json.load(open(os.path.join(HERE, "staged", "plan.json"), encoding="utf-8"))
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
edited = [f["path"] for f in plan["files"]]
created = [f["path"] for f in plan["newFiles"]]
shutil.rmtree(os.path.dirname(OV), ignore_errors=True)
chain = set()
for path in edited + created:
    parts = path.split("/")
    for i in range(len(parts)):
        chain.add("/".join(parts[:i]))
chain.add("")
def build(rel):
    real = os.path.join(ROOT, rel) if rel else ROOT
    target = os.path.join(OV, rel) if rel else OV
    os.makedirs(target, exist_ok=True)
    for name in os.listdir(real):
        child = (rel + "/" + name) if rel else name
        if child in chain:
            if os.path.isdir(os.path.join(real, name)) and not os.path.islink(os.path.join(real, name)): build(child)
            else: shutil.copyfile(os.path.join(real, name), os.path.join(target, name))
        elif child in edited:
            shutil.copyfile(os.path.join(real, name), os.path.join(target, name))
        else:
            os.symlink(os.path.join(real, name), os.path.join(target, name))
    for name in [c for c in chain if c.startswith(rel + "/" if rel else "") and c and "/" not in c[len(rel) + 1 if rel else 0:]]:
        pass
build("")
before = {p: sha(os.path.join(ROOT, p)) for p in edited}
run = lambda *cmd, **kw: subprocess.run(cmd, capture_output=True, text=True, **kw)
apply = lambda *a: run("python3", os.path.join(HERE, "apply.py"), "--root", OV, "--ignore-running", *a)
groups = ["--group", "all", "--group", "nx", "--group", "manifest", "--group", "manifest2", "--group", "flip"]
report = {}
r = apply(*groups, "--flip-gate-passed", "--write"); report["apply"] = (r.stdout + r.stderr).strip()[:700]
r = apply(*groups); report["second"] = (r.stdout + r.stderr).strip()[:300]
bad = []
for path in edited + created:
    full = os.path.join(OV, path)
    if path.endswith(".json"):
        try: json.load(open(full, encoding="utf-8"))
        except Exception as e: bad.append(("json", path[-60:], str(e)[:100]))
    if path.endswith((".ts", ".mjs")):
        r = run("bun", "build", "--no-bundle", full, "--outfile", "/private/tmp/bzl-default/out.js")
        if r.returncode: bad.append(("syntax", path[-70:], (r.stderr or r.stdout)[:300]))
report["syntax"] = bad
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
tests = [B + "/🚀️bootstrap/🧪️tests/🎯️closure/🟦️.ts", B + "/🚀️bootstrap/🧪️tests/🟦️.ts", B + "/🔧️tools/🏗️bazel/🔁️probe/🧪️tests/🟦️.ts"]
for t in tests:
    r = run("bun", "test", "./" + t, cwd=OV)
    report["test " + "/".join(t.split("/")[-3:-1])] = (r.stdout + r.stderr).strip().split("\n")[-6:]
for name, args in [("help", ["verify-help.ts", OV]), ("default", ["verify-default.ts", OV]), ("birth", ["verify-birth.ts", OV, ROOT]), ("setupLaw", ["verify-setup-law.ts", OV, ROOT])]:
    r = run("bun", os.path.join(HERE, args[0]), *args[1:]); report[name] = (r.stdout.strip() or r.stderr.strip())[:200]
report["liveUntouched"] = all(sha(os.path.join(ROOT, p)) == h for p, h in before.items())
r = apply("--group", "probe", "--group", "closure", "--group", "nx", "--group", "manifest", "--group", "manifest2", "--group", "flip", "--flip-gate-passed", "--write", "--undo"); report["undo"] = (r.stdout + r.stderr).strip()[:300]
report["leftover"] = [p for p in created if os.path.exists(os.path.join(OV, p))]
report["restored"] = all(sha(os.path.join(OV, p)) == h for p, h in before.items() if not p.endswith("💾️storage/🧪️tests/🟦️.ts"))
print(json.dumps(report, ensure_ascii=False, indent=1))
if "--keep" not in sys.argv: shutil.rmtree(os.path.dirname(OV), ignore_errors=True)
