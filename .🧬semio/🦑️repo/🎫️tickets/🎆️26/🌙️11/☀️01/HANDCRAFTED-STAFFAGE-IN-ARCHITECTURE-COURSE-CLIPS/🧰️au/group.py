import sys, collections
rows = [l.rstrip("\n") for l in open(sys.argv[1], encoding="utf-8")]
start = rows.index("## 5 functions calling primitives without any staffage call (* = object noun)") + 1
d = collections.OrderedDict()
for r in rows[start:]:
    if r.startswith("wide count"):
        break
    star = r[0] == "*"
    path, rest = r[2:].split(" ", 1)
    f = path.rsplit(":", 1)[0]
    name = rest.split(" ")[0]
    d.setdefault(f, []).append(("*" if star else "") + name)
for f, ns in d.items():
    print(f.replace("⚡️energy/", "E/").replace("🎬️clip/", "c/") + ": " + " ".join(dict.fromkeys(ns)))
