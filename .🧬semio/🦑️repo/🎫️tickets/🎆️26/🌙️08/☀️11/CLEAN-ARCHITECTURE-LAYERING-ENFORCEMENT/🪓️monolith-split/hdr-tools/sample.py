import json, re, collections, sys
t = open(sys.argv[1]).read()
j = json.loads(t[t.index("\n{\n") + 1:t.rindex("\n}\n") + 2])
for cfg, v in j["layout"].items():
    s = v["sampleOnlyCombined"]
    print(cfg, "onlyCombined", v["onlyCombined"], "samples", len(s))
    c = collections.Counter()
    for x in s:
        code, rest = x.split("\t", 1)
        msg = rest.split("  @")[0]
        c[code + " " + re.sub(r"`[^`]*`", "`X`", msg)] += 1
    for k, n in c.most_common():
        print(" ", n, k)
    for x in s[:3] + s[-2:]:
        print("   ", x[:240])
