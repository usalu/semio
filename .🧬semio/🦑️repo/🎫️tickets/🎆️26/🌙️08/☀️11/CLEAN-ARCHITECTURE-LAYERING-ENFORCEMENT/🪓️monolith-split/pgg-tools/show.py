import json, sys, re
F = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/pgg-%s.log" % sys.argv[1]
width = int(sys.argv[2]) if len(sys.argv) > 2 else 260
try:
    d = json.load(open(F, encoding="utf8"))
except Exception as e:
    print("unreadable", e); print(open(F, encoding="utf8").read()[:2000]); sys.exit(1)
print({k: v for k, v in d.items() if k != "layout"})
for k, v in d["layout"].items():
    print(k, "base", v["base"], "combined", v["combined"], "onlyCombined", v["onlyCombined"])
    seen = {}
    for s in v["sampleOnlyCombined"]:
        key = re.sub(r"@.*", "", s)
        seen.setdefault(key, []).append(s)
    for key, lst in seen.items():
        print("  x%d %s" % (len(lst), lst[0][:width]))
