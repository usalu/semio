"""📥 C4P28 installer: copies the staged files into the repo after proving that the live originals still equal the snapshots taken before the first edit (a peer edit aborts the install).
Children are written first, then the roots and the three consumer test files; every write is atomic (tmp + rename). No git command runs.
usage: install.py check|apply"""
import glob, hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
F = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
mode = sys.argv[1]
sha = lambda b: hashlib.sha256(b).hexdigest()
plan = []
problems = []
roots = []
for path in sorted(glob.glob(HERE + "/specs/*.json")):
    spec = json.load(open(path, encoding="utf-8"))
    live = f"{REPO}/{spec['file']}"
    snap = open(spec["original"], "rb").read()
    now = open(live, "rb").read()
    if sha(now) != sha(snap):
        problems.append(f"{spec['file']}: live file differs from the snapshot (a peer edited it)")
    base = os.path.dirname(spec["file"])
    for c in spec["chunks"]:
        rel = f"{base}/{c['folder']}/🦀️.rs"
        if os.path.exists(f"{REPO}/{rel}"):
            problems.append(f"{rel}: already exists")
        plan.append(rel)
    roots.append(spec["file"])
consumers = json.load(open(F + "/🗑️generated/c4p28-consumers.json", encoding="utf-8"))
for rel, meta in consumers.items():
    if sha(open(f"{REPO}/{rel}", "rb").read()) != meta["base_sha256"]:
        problems.append(f"{rel}: consumer file changed since staging")
stage = F + "/🗑️generated/c4p28-stage"
for rel in plan + roots + list(consumers):
    if not os.path.isfile(f"{stage}/{rel}"):
        problems.append(f"{rel}: not staged")
print(f"children {len(plan)}, roots {len(roots)}, consumers {len(consumers)}; problems {len(problems)}")
for p in problems:
    print("  PROBLEM", p)
if mode == "apply":
    if problems:
        sys.exit(1)
    done = []
    for rel in plan + roots + list(consumers):
        dst = f"{REPO}/{rel}"
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        tmp = dst + ".c4p28-tmp"
        with open(tmp, "wb") as f:
            f.write(open(f"{stage}/{rel}", "rb").read())
        os.replace(tmp, dst)
        done.append(rel)
    json.dump(dict(created=plan, rewritten=roots + list(consumers)), open(F + "/🗑️generated/c4p28-installed.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    print("installed", len(done), "files")
sys.exit(1 if problems else 0)
