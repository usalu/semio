#!/usr/bin/env python3
"""🧪️ Orchestration self-test of run.py on a throw-away git copy with a stub `bun` (no real bun repo, no real install): lock regenerates the six locks from the manifests, sync switches node_modules, the second sync is current.
Usage: selftest.py [--keep]   (needs git and python only; works under /private/tmp or the platform temp dir)"""
import json, os, shutil, stat, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
sys.path.insert(0, HERE)
from lockio import ROOTS, load_lock
ST = os.path.join("/private/tmp" if os.path.isdir("/private/tmp") else tempfile.gettempdir(), "lock-sync", "st"); shutil.rmtree(ST, ignore_errors=True); os.makedirs(ST)
plan = json.load(open(os.path.join(HERE, "staged", "plan.json"), encoding="utf-8"))
copy = {f["path"] for f in plan["files"]} | {os.path.join(r, n) for r in ROOTS for n in ("package.json", "bun.lock")}
for rel in sorted(copy):
    dst = os.path.join(ST, rel); os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copyfile(os.path.join(LIVE, rel), dst)
for r in ROOTS:
    for m in (json.load(open(os.path.join(ST, r, "package.json"), encoding="utf-8")).get("workspaces") or []):
        src = os.path.join(LIVE, r, m, "package.json")
        if os.path.isfile(src): dst = os.path.join(ST, r, m, "package.json"); os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copyfile(src, dst) if not os.path.exists(dst) else None
os.makedirs(os.path.join(ST, ".nx", "installation")); os.makedirs(os.path.join(ST, ".cache", "b", "d", "m", "bun-cache", "pkg"))
for r in ROOTS:
    g = os.path.join(ST, ".gen", "g1", r); os.makedirs(os.path.join(g, "node_modules", "typescript")); os.makedirs(os.path.join(g, "node_modules", "@semio-tech"))
    open(os.path.join(g, "node_modules", "typescript", "package.json"), "w").write("{}"); os.symlink(os.path.join(g, "node_modules"), os.path.join(ST, r, "node_modules"))
subprocess.run(["git", "init", "-q"], cwd=ST, check=True); subprocess.run(["git", "add", "-A", "--", "*package.json"], cwd=ST, check=True)
bin_dir = os.path.join(ST, ".bin"); os.makedirs(bin_dir)
stub = os.path.join(bin_dir, "bun")
open(stub, "w", encoding="utf-8").write('''#!/usr/bin/env python3
import json, os, sys, time
sys.path.insert(0, %r)
from lockio import ROOTS, FIELDS, load_lock
a = sys.argv[1:]; root = os.getcwd()
if a[:2] == ["repo", "--help"]: print("usage: repo"); sys.exit(0)
if a[:3] == ["repo", "dependencies", "ensure"]: print("ensure ok"); sys.exit(0)
if a[:3] == ["repo", "dependencies", "lock"]:
    time.sleep(1)
    for r in ROOTS:
        lock = load_lock(os.path.join(root, r, "bun.lock")); pj = json.load(open(os.path.join(root, r, "package.json"), encoding="utf-8")); ws = {"": pj}
        for m in pj.get("workspaces", []): ws[m] = json.load(open(os.path.join(root, r, m, "package.json"), encoding="utf-8"))
        new = {}
        for m, man in ws.items():
            e = {"name": man.get("name")}
            for f in FIELDS:
                if man.get(f): e[f] = man[f]
            new[m] = e
        for m, man in ws.items():
            if m and m not in lock["workspaces"]: lock["packages"][man["name"]] = [man["name"] + "@workspace:" + m]
        lock["workspaces"] = new
        json.dump(lock, open(os.path.join(root, r, "bun.lock"), "w"))
    print("[DATA] Store dependency lock complete {}"); sys.exit(0)
if a[:3] == ["repo", "dependencies", "sync"]:
    marker = os.path.join(root, ".synced")
    if os.path.exists(marker): print("[DATA] Store dependency sync current {}"); sys.exit(0)
    print("[DATA] Store dependency sync stale {\\"reason\\":\\"manifest\\"}"); time.sleep(3)
    for r in ROOTS:
        g = os.path.join(root, ".gen", "g2", r, "node_modules"); os.makedirs(os.path.join(g, "typescript")); os.makedirs(os.path.join(g, "@semio-tech", "build-parsers"))
        open(os.path.join(g, "typescript", "package.json"), "w").write("{}"); tmp = os.path.join(root, r, "node_modules.tmp"); os.symlink(g, tmp); os.replace(tmp, os.path.join(root, r, "node_modules"))
    open(marker, "w").write("1"); print("[DATA] Store dependency sync complete {}"); sys.exit(0)
sys.exit(2)
''' % HERE)
os.chmod(stub, os.stat(stub).st_mode | stat.S_IEXEC)
env = dict(os.environ, PATH=bin_dir + os.pathsep + os.environ["PATH"])
rc = subprocess.run([sys.executable, os.path.join(HERE, "run.py"), "--yes", "--root", ST, "--ignore-running", "--max-load", "999", "--run-dir", os.path.join(ST, ".run")], env=env).returncode
state = json.load(open(os.path.join(ST, ".run", "state.json"))) if os.path.exists(os.path.join(ST, ".run", "state.json")) else {}
print(json.dumps({"rc": rc, "failed": state.get("failed"), "sync2Current": state.get("sync2Current"), "verify": {k: v for k, v in (state.get("verify") or {}).items() if k in ("nlink0", "locksMatchManifests", "nodeModulesSwitched", "rootHasBuildParsers", "ensureRc")}}, ensure_ascii=False, indent=1))
if "--keep" not in sys.argv: shutil.rmtree(ST, ignore_errors=True)
sys.exit(rc)
