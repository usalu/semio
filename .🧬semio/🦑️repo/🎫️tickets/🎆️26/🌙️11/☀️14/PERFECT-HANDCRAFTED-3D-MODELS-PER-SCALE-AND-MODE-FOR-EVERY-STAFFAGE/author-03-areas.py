import json, re, subprocess, sys, glob, os
here = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
slug, scale = sys.argv[1], sys.argv[2]
only = set(sys.argv[3:])
path = glob.glob(os.path.join(here, "../../../../../../../*teaching/*architecture/*staffage/*catalog/*machinery", slug, "*.json"))[0]


def devs():
    out = subprocess.run([BUN, "volumes-check.ts", "machinery/" + slug, "--scale", scale, "--rules", "volumes-thermal-coverage", "--json"], cwd=here, capture_output=True, text=True).stdout
    return {m["name"].split(":")[1]: m["value"] for m in json.loads(out)["measurements"] if m["name"].startswith("areaDeviation:")}


def regions():
    lines = open(path, encoding="utf8").read().split("\n")
    i0 = next(i for i, l in enumerate(lines) if '"thermal": {' in l)
    res = {}
    for i in range(i0, len(lines)):
        m2 = re.match(r'^( +)("areaM2": )([\d.e-]+)(,?)$', lines[i])
        if m2:
            for k in range(i - 1, i0, -1):
                m = re.match(r'^( +)"id": "([^"]+)",?$', lines[k])
                if m and m.group(1) == m2.group(1):
                    res[m.group(2)] = (i, float(m2.group(3)), m2.group(1) + m2.group(2), m2.group(4))
                    break
    return lines, res


def setv(rid, v):
    lines, res = regions()
    i, old, pre, post = res[rid]
    lines[i] = pre + repr(v) + post
    open(path, "w", encoding="utf8").write("\n".join(lines))
    return old


d0 = devs()
for rid, d in d0.items():
    if only and rid not in only or (not only and d <= 0.12):
        continue
    lines, res = regions()
    old = res[rid][1]
    cand = []
    for f in (1 + d, 1 - d):
        v = float("%.3g" % (old * f))
        if v > 0:
            cand.append(v)
    best = None
    for v in cand:
        setv(rid, v)
        dd = devs().get(rid, 9)
        if dd < 0.03:
            best = v
            break
    if best is None:
        setv(rid, old)
        print("no fit", rid)
    else:
        print("area", rid, old, "->", best)
