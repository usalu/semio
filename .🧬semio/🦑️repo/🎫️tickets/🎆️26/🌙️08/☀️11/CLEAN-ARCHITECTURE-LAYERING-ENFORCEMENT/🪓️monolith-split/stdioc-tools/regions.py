#!/usr/bin/env python3
# 🏷️ Region markers: every baseline `//#region` name still exists exactly as often, and every region is balanced in each file.
import collections, os, re, sys
SCRATCH = os.environ["SCRATCH"]
OLD = SCRATCH + "/baseline/✏️s/🔌️plugins/🗄️stdio/📇️registry"
NEW = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry"
def scan(tree):
    starts, ends, per_file = collections.Counter(), collections.Counter(), {}
    for directory, _, names in os.walk(tree):
        if "node_modules" in directory or "/dist" in directory:
            continue
        for name in names:
            if name.endswith(".rs"):
                path = os.path.join(directory, name)
                text = open(path, encoding="utf8").read()
                s = re.findall(r"^\s*//#region (.+)$", text, re.M)
                e = re.findall(r"^\s*//#endregion (.+)$", text, re.M)
                starts.update(s); ends.update(e)
                per_file[path] = (collections.Counter(s), collections.Counter(e))
    return starts, ends, per_file
os_, oe, _ = scan(OLD)
ns, ne, per = scan(NEW)
print("baseline regions", sum(os_.values()), "balanced", os_ == oe)
print("new regions", sum(ns.values()), "balanced", ns == ne)
print("missing from new:", sorted((os_ - ns).keys()))
print("added in new:", sorted((ns - os_).keys()))
bad = [p for p, (s, e) in per.items() if s != e]
print("unbalanced files:", bad)
