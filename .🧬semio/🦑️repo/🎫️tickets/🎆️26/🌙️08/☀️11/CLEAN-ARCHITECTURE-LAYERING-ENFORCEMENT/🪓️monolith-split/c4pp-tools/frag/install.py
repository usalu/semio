"""📥 C4P39 installer: copies the staged files into the repo after proving that the live originals still equal the snapshots taken before the first edit (a peer edit aborts the install).
Children are written first, then the roots; every write is atomic (tmp + rename). No git command runs. usage: install.py check|apply"""
import glob, hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
F = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
STAGE = "/Users/ueli/.cache/c4p39/stage"
mode = sys.argv[1]
sha = lambda b: hashlib.sha256(b).hexdigest()
plan, roots, problems = [], [], []
for path in sorted(glob.glob(HERE + "/specs/*.json")):
    spec = json.load(open(path, encoding="utf-8"))
    live = f"{REPO}/{spec['file']}"
    if sha(open(live, "rb").read()) != sha(open(spec["original"], "rb").read()):
        problems.append(f"{spec['file']}: live file differs from the snapshot (a peer edited it)")
    base = os.path.dirname(spec["file"])
    for c in spec["chunks"]:
        rel = f"{base}/{c['folder']}/🦀️.rs"
        if os.path.exists(f"{REPO}/{rel}"):
            problems.append(f"{rel}: already exists")
        plan.append(rel)
    roots.append(spec["file"])
for rel in plan + roots:
    if not os.path.isfile(f"{STAGE}/{rel}"):
        problems.append(f"{rel}: not staged")
print(f"children {len(plan)}, roots {len(roots)}, problems {len(problems)}")
for p in problems:
    print("  PROBLEM", p)
if mode == "apply":
    if problems:
        sys.exit(1)
    for rel in plan + roots:
        dst = f"{REPO}/{rel}"
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        tmp = dst + ".c4p39-tmp"
        with open(tmp, "wb") as f:
            f.write(open(f"{STAGE}/{rel}", "rb").read())
        os.replace(tmp, dst)
    json.dump(dict(created=plan, rewritten=roots), open(F + "/c4p39-tools/installed.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    print("installed", len(plan) + len(roots), "files")
sys.exit(1 if problems else 0)
