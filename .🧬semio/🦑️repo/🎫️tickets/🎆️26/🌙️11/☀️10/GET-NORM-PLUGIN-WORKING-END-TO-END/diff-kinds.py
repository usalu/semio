import collections, glob, json, os, re, sys
produced, fixtures = sys.argv[1:3]
def walk(a, b, path, out):
    if type(a) != type(b): out.append((path, a, b)); return
    if isinstance(a, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a: out.append((path + "/" + k, "<missing>", b[k]))
            elif k not in b: out.append((path + "/" + k, a[k], "<missing>"))
            else: walk(a[k], b[k], path + "/" + k, out)
    elif isinstance(a, list):
        if len(a) != len(b): out.append((path, "len%d" % len(a), "len%d" % len(b))); return
        for i, (x, y) in enumerate(zip(a, b)): walk(x, y, path + "[%d]" % i, out)
    elif a != b: out.append((path, a, b))
kinds = collections.Counter()
for f in sorted(glob.glob(produced + "/*.json")):
    leaf, case = os.path.basename(f)[:-5].split("__")
    out = []
    walk(json.load(open(f)), json.load(open(f"{fixtures}/{leaf}/{case}/🔣️.diff.json")), "", out)
    for p, a, b in out:
        kinds[(re.sub(r"\[\d+\]", "[]", p).split("/")[-1], str(a)[:25], str(b)[:25])] += 1
for k, v in kinds.most_common(14): print(v, k)
