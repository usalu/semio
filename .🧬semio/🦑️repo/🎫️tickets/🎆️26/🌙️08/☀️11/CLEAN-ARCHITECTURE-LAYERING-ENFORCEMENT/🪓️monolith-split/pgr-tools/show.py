import json, sys, re, os
F = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "pgr-%s.log" % sys.argv[1])
width = int(sys.argv[2]) if len(sys.argv) > 2 else 300
d = json.load(open(F, encoding="utf8"))
print({k: v for k, v in d.items() if k != "layout"})
for k, v in d["layout"].items():
    print(k, "base", v["base"], "combined", v["combined"], "onlyCombined", v["onlyCombined"])
    for s in v["sampleOnlyCombined"]:
        print("   ", s[:width])
