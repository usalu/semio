import json, re, sys, glob
F = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*umans*/person-with-umbrella/🔣️.json")[0]
text = open(F, encoding="utf-8").read()
a = json.loads(text)["asset"]
rep0 = text.index('"representations"')


def under(d):
    toks = re.findall(r"[MLCZ]|-?\d+\.?\d*", d)
    if "C" in d:
        nums = [float(x) for x in re.findall(r"-?\d+\.?\d*", d)]
        p = [nums[0:2], nums[2:4], nums[4:6], nums[6:8], nums[8:10], nums[10:12], nums[12:14]]
        p0, c1, c2, p1 = p[3], p[4], p[5], p[6]
        pts = []
        for t in (0.1, 0.25, 0.4, 0.55, 0.7, 0.85):
            m = 1 - t
            pts.append([round(m**3 * p0[i] + 3 * m * m * t * c1[i] + 3 * m * t * t * c2[i] + t**3 * p1[i], 5) for i in (0, 1)])
        return [list(p0)] + pts
    nums = [float(x) for x in re.findall(r"-?\d+\.?\d*", d)]
    pts = [nums[i : i + 2] for i in range(0, len(nums), 2)]
    imax = max(range(len(pts)), key=lambda i: pts[i][0])
    return [p for p in pts[imax:] if 0.02 < p[0] < 0.555 and abs(p[1]) > 0.002]


new = {}
for sc in ("50", "100", "200", "500"):
    shapes = {s["id"]: s for s in a["representations"][sc]["geometry"]["elevation"]}
    for side, sgn in (("front", 1), ("back", -1)):
        u = under(shapes["canopy-" + side]["d"])
        poly = [[0, 0], [0.0926, -0.2081 * sgn], [0.56, 0]] + [[round(p[0], 5), round(p[1], 5)] for p in u]
        new[(sc, side)] = poly
pos = rep0
for sc in ("50", "100", "200", "500"):
    sc_pos = text.index(f'"denominator": {sc}', pos)
    for side in ("front", "back"):
        i = text.index(f'"id": "canopy-core-{side}"', sc_pos)
        j = text.index('"points": ', i)
        k = j + len('"points": ')
        depth, q = 0, k
        while True:
            ch = text[q]
            if ch == "[":
                depth += 1
            elif ch == "]":
                depth -= 1
                if depth == 0:
                    break
            q += 1
        old = text[k : q + 1]
        ls = text.rfind(chr(10), 0, j) + 1
        indent = re.match(r" *", text[ls:]).group(0)
        rows = ",\n".join(indent + "  " + json.dumps(p) for p in new[(sc, side)])
        text = text[:k] + "[\n" + rows + "\n" + indent + "]" + text[q + 1 :]
        print(sc, side, len(new[(sc, side)]))
    pos = sc_pos + 10
json.loads(text)
open(F, "w", encoding="utf-8").write(text)
