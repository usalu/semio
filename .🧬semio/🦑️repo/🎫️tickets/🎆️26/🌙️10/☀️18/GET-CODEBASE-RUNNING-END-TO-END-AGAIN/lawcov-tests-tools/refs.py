# 🔎 Units of build/caching without an own test whose exported names no test file mentions: refs.py <units.json> <out.json>
import json, os, re, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
N = lambda s: s.replace("️", "")
raw = subprocess.run(["git", "-c", "core.quotepath=off", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", LIB], cwd=ROOT, capture_output=True, check=True).stdout.decode().split("\0")
files = {N(f[len(LIB)+1:]): os.path.join(ROOT, f) for f in raw if f}
tests = {r: p for r, p in files.items() if "🧪tests" in r.split("/") and r.endswith((".ts", ".mjs", ".mts", ".py"))}
blob = "\n".join(open(p, encoding="utf8", errors="replace").read() for p in tests.values())
idents = set(re.findall(r"[A-Za-z0-9_$]+", blob))
rows = json.load(open(sys.argv[1]))
EXP = re.compile(r"export\s+(?:async\s+)?(?:function\*?|const|class|let)\s+([A-Za-z0-9_$]+)")
out = []
for r in rows:
    u = r["unit"]
    if not (u.startswith("🏗build") or u.startswith("⚡caching")) or r["own_tests"]:
        continue
    names = []
    for f, p in files.items():
        if os.path.dirname(f) == u and f.split("/")[-1] in ("🟦.ts", "🟨.mjs"):
            names += EXP.findall(open(p, encoding="utf8", errors="replace").read())
    hit = [n for n in names if n in idents]
    out.append({"unit": u, "lines": r["lines"], "exports": len(names), "referenced": len(hit)})
json.dump(out, open(sys.argv[2], "w"), ensure_ascii=False, indent=1)
for o in sorted(out, key=lambda o: -o["lines"]):
    if o["referenced"] == 0:
        print(o["lines"], o["exports"], o["unit"])
