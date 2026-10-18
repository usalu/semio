"""📏 Re-implements `test lib-wiring-budget` (plugin crate lib roots vs taxonomy budget and ratchet pins) without bun.
usage: libwiring.py   prints breaches"""
import glob, json, os, re, subprocess
R = "/Users/ueli/Documents/semio"
T = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json"
rat = json.load(open(T, encoding="utf-8"))
budget = rat["budget"]
idx = json.load(open(R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/ohp-manifest-index.json", encoding="utf-8"))
roots = {}
for n, rel in idx.items():
    if not rel.startswith("✏️s/🔌️plugins/"): continue
    t = open(os.path.join(R, rel), encoding="utf-8").read()
    m = re.search(r"^\[lib\][^\[]*?^path\s*=\s*\"([^\"]+)\"", t, re.M | re.S)
    if not m: continue
    lib = os.path.normpath(os.path.join(os.path.dirname(rel), m.group(1)))
    try: roots[lib] = len(open(os.path.join(R, lib), encoding="utf-8").read().split("\n"))
    except Exception: pass
b = []
for p, n in sorted(roots.items()):
    pin = rat["entries"].get(p)
    if n <= budget: continue
    if pin is None: b.append((p, n, "unpinned"))
    elif n > pin: b.append((p, n, f"pin {pin}"))
print("budget", budget, "roots", len(roots), "breaches", len(b))
for p, n, w in b: print(n, w, p.split("artifacts/")[-1][-110:])
