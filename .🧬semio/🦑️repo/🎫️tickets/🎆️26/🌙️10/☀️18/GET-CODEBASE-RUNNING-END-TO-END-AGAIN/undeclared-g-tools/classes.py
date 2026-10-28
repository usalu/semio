"""undeclared-g classes: groups the learned (undeclared) reads of an oracle report into classes by path; prints counts and a few examples per class.
usage: python3 classes.py <report.json> [per-class examples=3]"""
import json, os, sys, collections
r = json.load(open(sys.argv[1], encoding="utf8"))
n = int(sys.argv[2]) if len(sys.argv) > 2 else 3
def classify(p):
    base = os.path.basename(p)
    if base in ("package.json",) or base.startswith("tsconfig"): return "module-config (nearest package.json/tsconfig)"
    if base in (".gitignore", ".ignore", ".npmrc"): return "ignore-file"
    if "🔨️modules/📚️library/" in p and "🧪️" not in p: return "build-library module/data (loaded at run time)"
    if "/🖱️ui/" in p: return "ui tree (tailwind scan region)"
    if "/📚️course/" in p: return "course tree"
    return "other"
groups = collections.defaultdict(list)
for p in r["learned"]: groups[classify(p)].append(p)
for k, v in sorted(groups.items(), key=lambda kv: -len(kv[1])):
    print(f"{len(v):5d} {k}")
    for p in v[:n]: print("        ", p)
