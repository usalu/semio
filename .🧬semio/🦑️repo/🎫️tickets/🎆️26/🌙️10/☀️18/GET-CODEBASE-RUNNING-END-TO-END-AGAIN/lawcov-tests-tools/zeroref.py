# 🔎 Exported functions of the build and caching libraries that no test file names, ranked by body size: zeroref.py
import os, re, subprocess, collections
ROOT = "/Users/ueli/Documents/semio"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
N = lambda s: s.replace("️", "")
raw = subprocess.run(["git", "-c", "core.quotepath=off", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", LIB], cwd=ROOT, capture_output=True).stdout.decode().split("\0")
files = {N(f[len(LIB)+1:]): os.path.join(ROOT, f) for f in raw if f}
idents = set()
for r, p in files.items():
    if "🧪tests" in r.split("/") and r.endswith((".ts", ".mjs", ".mts", ".py", ".json")):
        idents.update(re.findall(r"[A-Za-z0-9_$]+", open(p, encoding="utf8", errors="replace").read()))
EXP = re.compile(r"^export\s+(?:async\s+)?function\*?\s+([A-Za-z0-9_$]+)", re.M)
rows = []
for r, p in files.items():
    if not (r.startswith("🏗build/") or r.startswith("⚡caching/")) or "🧪tests" in r.split("/") or "🧫fixtures" in r.split("/"):
        continue
    if r.split("/")[-1] not in ("🟦.ts", "🟨.mjs"):
        continue
    text = open(p, encoding="utf8", errors="replace").read()
    ms = list(EXP.finditer(text))
    for i, m in enumerate(ms):
        end = ms[i+1].start() if i + 1 < len(ms) else len(text)
        if m.group(1) not in idents:
            d=os.path.dirname(r)
            own=any(k.startswith(d+"/🧪tests/") for k in files)
            rows.append((text[m.start():end].count("\n"), m.group(1), d, own))
rows.sort(reverse=True)
for n, name, unit, own in rows:
    if not own: print(n, name, unit)
print(len(rows))
