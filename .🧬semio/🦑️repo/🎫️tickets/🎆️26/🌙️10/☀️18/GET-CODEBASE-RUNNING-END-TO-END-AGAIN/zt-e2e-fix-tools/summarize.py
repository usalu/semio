import json, collections, sys
d = json.load(open(sys.argv[1]))
print("files", d["files"], "bare edges", d["count"])
c = collections.defaultdict(list)
for r in d["rows"]:
    c[(r["specifier"], r["kind"])].append(r["file"])
for k, v in sorted(c.items()):
    print(k, len(v), v[0][-110:])
