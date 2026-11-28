import hashlib, os, shutil, sys
# H60 landing: new members resolve-consulted + file-memo test, in-place update of journal-clean (only when the live bytes are the bytes the patch was built on)
B, H, DRY = sys.argv[1], os.path.dirname(os.path.abspath(__file__)), os.environ.get("DRY") == "1"
base = open(os.path.join(H, "base.sha")).read().split()
load = os.path.join(B, "🕸️graph", "📥️load")
jobs = [(os.path.join(H, "staged", "journal-clean.ts"), os.path.join(load, "🗞️journal-clean", "🟦️.ts"), base[0]),
        (os.path.join(H, "staged", "journal-clean-tests.ts"), os.path.join(load, "🗞️journal-clean", "🧪️tests", "🟦️.ts"), base[1])]
def sha(path): return hashlib.sha256(open(path, "rb").read()).hexdigest()
for _, live, expected in jobs:
    if sha(live) != expected: sys.exit("anchor moved: " + live)
adds = [(os.path.join(H, "staged", "🧭️resolve-consulted"), os.path.join(load, "🧭️resolve-consulted")),
        (os.path.join(H, "staged", "🧠️file-memo"), os.path.join(B, "👁️observation", "🧪️tests", "🧠️file-memo"))]
for _, live in adds:
    if os.path.exists(live): sys.exit("exists: " + live)
if DRY: print("dry ok"); sys.exit(0)
for src, live in adds: shutil.copytree(src, live)
for src, live, _ in jobs:
    temporary = live + ".h60.tmp"; shutil.copyfile(src, temporary); os.replace(temporary, live)
print("applied")
