"""🗂 Builds /Users/ueli/.cache/ohp/defs.json: item name -> [[file, line]] for struct/enum/fn/trait/type/const/static/mod/union/macro_rules! definitions in all tracked and untracked .rs files (one git grep pass).
usage: mkdefs.py"""
import json, re, subprocess, collections, os
ROOT = "/Users/ueli/Documents/semio"
rx = re.compile(r"(?:^|[^A-Za-z0-9_'])(?:struct|enum|fn|trait|type|const|static|mod|union|macro_rules!) +([A-Za-z_][A-Za-z0-9_]*)")
G = ["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false"]
d = collections.defaultdict(list)
p = subprocess.run(G + ["grep", "-n", "-E", r"(^|[^A-Za-z0-9_'])(struct|enum|fn|trait|type|const|static|mod|union|macro_rules!) +[A-Za-z_]", "--", ":(glob)**/*.rs", ":!.🧬semio"], cwd=ROOT, capture_output=True, text=True, errors="replace")
for l in p.stdout.split("\n"):
    if not l:
        continue
    try:
        f, ln, t = l.split(":", 2)
    except ValueError:
        continue
    for m in rx.finditer(t):
        d[m.group(1)].append([f, int(ln)])
un = subprocess.run(G + ["ls-files", "--others", "--exclude-standard", "--", ":(glob)**/*.rs", ":!.🧬semio"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
for f in un:
    if not f:
        continue
    try:
        for i, t in enumerate(open(os.path.join(ROOT, f), encoding="utf-8").read().split("\n")):
            for m in rx.finditer(t):
                d[m.group(1)].append([f, i + 1])
    except Exception:
        pass
json.dump(d, open("/Users/ueli/.cache/ohp/defs.json", "w", encoding="utf-8"), ensure_ascii=False)
print(len(d), sum(len(v) for v in d.values()))
