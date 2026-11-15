import os, re, glob
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "C:/git/semio"
SCRIPT = os.path.join(ROOT, "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts")
ENERGY = "🎓️teaching/🏛️architecture/⚡️energy"
reg_text = open(SCRIPT, encoding="utf-8").read()
reg = set(re.findall(r'"(🎓️teaching[^"]*)"', reg_text))
energy_reg = {r for r in reg if "⚡️energy" in r}
files = []
for dp, dn, fn in os.walk(os.path.join(ROOT, ENERGY)):
    dn[:] = [d for d in dn if d not in (".venv", "__pycache__", "📦️packages", "node_modules", ".ruff_cache")]
    for f in fn:
        if "/🧪️" in dp.replace(os.sep, "/") + "/" and f.endswith(".py"):
            files.append(os.path.relpath(os.path.join(dp, f), ROOT).replace("\\", "/"))
        elif "🧪️tests" in dp:
            pass
print("energy test .py files:", len(files))
def modid(p):
    return "🎓️teaching." + p[len("🎓️teaching/"):-3].replace("/", ".")
def dotted(p):
    return p[:-3].replace("/", ".")
registered_hits = 0
missing = []
for p in sorted(files):
    m = modid(p)
    cands = {m, m.replace("🐍️.py", "🐍️")}
    base = p[:-3]
    d = os.path.dirname(p)
    cand2 = "🎓️teaching." + p[len("🎓️teaching/"):].replace("/", ".")[:-3]
    cand3 = "🎓️teaching." + p[len("🎓️teaching/"):].replace("/", ".")
    ok = any(c in reg for c in (cand2, cand2 + ".", cand3)) or any(r.rstrip(".") == cand2 for r in reg)
    if ok:
        registered_hits += 1
    else:
        missing.append(p)
print("registered:", registered_hits)
print("NOT registered:", len(missing))
for p in missing:
    print("MISSING", p)
print("---- registry entries under energy with no test file match:")
fileset = set("🎓️teaching." + p[len("🎓️teaching/"):].replace("/", ".")[:-3] for p in files)
for r in sorted(energy_reg):
    rr = r.rstrip(".")
    if rr not in fileset and not any(x.startswith(rr) for x in fileset):
        print("EXTRA", r)
