"""📥 C5B installer: copies the staged tree of the given tags into the repo. Preconditions per tag: the live root equals its snapshot byte for byte and no child folder exists yet. Children are written first, then the root (compile-atomic order).
usage: install.py check|apply <tag>..."""
import json, os, sys, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
STAGE = os.environ.get("C6_STAGE", "/Users/ueli/.cache/c6plug/stage-frag")
mode = sys.argv[1]
problems = 0
plan = []
for tag in sys.argv[2:]:
    st = json.load(open(f"{HERE}/staged-{tag}.json", encoding="utf-8"))
    rel = st["root"]
    live = open(os.path.join(REPO, rel), "rb").read()
    snap = open(f"{HERE}/orig/{tag}.rs", "rb").read()
    if live != snap:
        print(f"[{tag}] PROBLEM live root differs from snapshot"); problems += 1; continue
    bdir = os.path.dirname(rel)
    for f in st["folders"]:
        if os.path.exists(os.path.join(REPO, bdir, f)):
            print(f"[{tag}] PROBLEM child exists {f}"); problems += 1
        p = os.path.join(bdir, f, "🦀️.rs")
        if len(p.encode()) > 240 or len(os.path.join(REPO, p)) > 256:
            print(f"[{tag}] PROBLEM path too long {p}"); problems += 1
        if not os.path.exists(os.path.join(STAGE, p)):
            print(f"[{tag}] PROBLEM staged file missing {p}"); problems += 1
    plan.append((tag, rel, bdir, st["folders"]))
print("problems", problems, "tags", [p[0] for p in plan])
if mode == "apply" and problems == 0:
    n = 0
    for tag, rel, bdir, folders in plan:
        for f in folders:
            dst = os.path.join(REPO, bdir, f, "🦀️.rs")
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(os.path.join(STAGE, bdir, f, "🦀️.rs"), dst)
            n += 1
        shutil.copyfile(os.path.join(STAGE, rel), os.path.join(REPO, rel))
        n += 1
        print(f"[{tag}] installed {len(folders)} children + root")
    print("installed", n, "files")
sys.exit(1 if problems else 0)
