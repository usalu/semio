import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import importlib, build, rules, law_germanqa as law
root = "/Users/ueli/Documents/semio"
ch = build.build(root)
overlay = {rel: n for rel, (b, n, c) in ch.items()}
f = law.check(root, overlay=overlay)
by = collections.defaultdict(list)
for rule, rel, path, detail in f: by[rule].append((rel, path, detail))
lim = int(sys.argv[1]) if len(sys.argv) > 1 else 4
for rule in sorted(by):
    print(f"{rule}: {len(by[rule])} in {len({r for r, _p, _d in by[rule]})} files")
    for rel, path, detail in by[rule][:lim]: print("    ", rel.split("/", 3)[-1][-70:], path[-40:], detail[:170])
print("residual", len(f), "changed files", len(ch))
