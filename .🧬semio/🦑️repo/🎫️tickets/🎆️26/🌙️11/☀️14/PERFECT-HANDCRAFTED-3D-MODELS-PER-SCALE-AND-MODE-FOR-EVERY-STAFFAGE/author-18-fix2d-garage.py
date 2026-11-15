import json, math, re
PATH = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/sectional-garage-door/🔣️.json"
text = open(PATH).read()
doc = json.loads(text)
a = doc["asset"]


def flag_poly():
    cx, cy, r = 0.12, 2.75, 0.095
    t = math.asin(0.04 / r)
    pts = [[0.16, 2.30]]
    n = 24
    a0, a1 = -(math.pi / 2 - t), math.pi + (math.pi / 2 - t)
    pts += [[round(cx + r * math.cos(a0 + (a1 - a0) * k / n), 5), round(cy + r * math.sin(a0 + (a1 - a0) * k / n), 5)] for k in range(n + 1)]
    pts.append([0.08, 2.30])
    return pts


def patch200():
    g = a["representations"]["200"]["geometry"]
    el = g["elevation"]
    if any(s["id"] == "bearing-flag" for s in el):
        return
    base = {"bone": "track", "role": "detail", "z": 1, "filled": True, "tone": 0.4, "region": "track-steel", "material": "steel"}
    for k, x in ((0, 1.3), (1, 2.5)):
        el.append(dict(base, id=f"hanger-{k}", kind="poly", points=[[x - 0.02, 2.5], [x + 0.02, 2.5], [x + 0.02, 2.85], [x - 0.02, 2.85]], closed=True))
    el.append(dict(base, id="bearing-flag", kind="poly", points=flag_poly(), closed=True))
    for sd, x in (("left", 1.25), ("right", -1.28)):
        g["front"].append(dict(base, id=f"bearing-flag-{sd}", kind="rect", x=x, y=2.3, w=0.03, h=0.545))
    for sd, y in (("left", 1.25), ("right", -1.28)):
        g["plan"].append(dict(base, id=f"bearing-flag-{sd}", kind="rect", x=0.025, y=y, w=0.19, h=0.03))


def patch(scale):
    g = a["representations"][scale]["geometry"]
    el = g["elevation"]
    ids = [s["id"] for s in el]
    if "track-band" in ids:
        return
    i = ids.index("track")
    tr, tb = el[i], el[ids.index("track-b")]
    band = dict(tr)
    band.update({"id": "track-band", "filled": True, "closed": True, "points": tr["points"] + tb["points"][::-1]})
    band.pop("penMm", None)
    el[i] = band
    el.remove(tb)
    for k in (0, 1):
        h = [s for s in el if s["id"] == f"hanger-{k}"][0]
        x = h["points"][0][0]
        h.update({"filled": True, "closed": True, "points": [[x - 0.02, 2.5], [x + 0.02, 2.5], [x + 0.02, 2.85], [x - 0.02, 2.85]]})
    base = {"bone": "track", "role": "detail", "z": 1, "filled": True, "tone": 0.4, "region": "track-steel", "material": "steel"}
    el.append(dict(base, id="bearing-flag", kind="poly", points=flag_poly(), closed=True))
    fr = g["front"]
    for sd, x in (("left", 1.25), ("right", -1.26)):
        fr.append(dict(base, id=f"bearing-flag-{sd}", kind="rect", x=x, y=2.3, w=0.01, h=0.545))
    for sd, x in (("left", 1.2), ("right", -1.26)):
        fr.append(dict(base, id=f"shaft-end-{sd}", bone="spring-shaft", region="spring-assembly", tone=0.2, kind="rect", x=x, y=2.7375, w=0.06, h=0.025))
    if scale == "50":
        fr.append(dict(base, id="track-end-bar", kind="rect", x=-1.21, y=2.48, w=2.42, h=0.04))
    pl = g["plan"]
    for sd, y in (("left", 1.25), ("right", -1.26)):
        pl.append(dict(base, id=f"bearing-flag-{sd}", kind="rect", x=0.025, y=y, w=0.19, h=0.01))



def member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        c = text[index]
        if quoted:
            if c == "\\":
                index += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit("unbalanced")


ranges = []
pos = text.index('"representations"')
for m in re.finditer(r'\n( +)"geometry"\s*:\s*', text[pos:]):
    ranges.append((pos + m.start() + 1, m.group(1), member_end(text, pos + m.end())))
    if len(ranges) == 4:
        break
out, last = [], 0
for (st, ind, en), scale in zip(ranges, ("50", "100", "200", "500")):
    orig = text[st:en]
    g = doc["asset"]["representations"][scale]["geometry"]
    enc = lambda g_: ind + '"geometry": ' + json.dumps(g_, indent=2, ensure_ascii=False).replace("\n", "\n" + ind)
    assert enc(json.loads(json.dumps(g))) is not None
    out.append((st, en, scale, ind, orig))
orig_doc = json.loads(text)
for (st, en, scale, ind, orig) in out:
    og = orig_doc["asset"]["representations"][scale]["geometry"]
    assert ind + '"geometry": ' + json.dumps(og, indent=2, ensure_ascii=False).replace("\n", "\n" + ind) == orig, "geometry roundtrip " + scale
for s_ in ("50", "100"):
    patch(s_)
patch200()
def patch500():
    pl = a["representations"]["500"]["geometry"]["plan"]
    if any(s["id"] == "drum-left" for s in pl):
        return
    base = {"bone": "spring-shaft", "role": "body", "z": 2, "filled": True, "tone": 0.4, "region": "spring-assembly", "material": "steel", "kind": "rect", "w": 0.18, "h": 0.1, "x": 0.03}
    pl.append(dict(base, id="drum-left", y=1.1))
    pl.append(dict(base, id="drum-right", y=-1.2))


patch500()
for sc_, th_ in (("50", 0.01), ("100", 0.015), ("200", 0.03)):
    gg = a["representations"][sc_]["geometry"]
    for sh in gg["front"]:
        if sh["id"].startswith("bearing-flag"):
            sh["w"] = th_
            sh["x"] = 1.25 if sh["id"].endswith("left") else round(-1.25 - th_, 4)
    for sh in gg["plan"]:
        if sh["id"].startswith("bearing-flag"):
            sh["h"] = th_
            sh["y"] = 1.25 if sh["id"].endswith("left") else round(-1.25 - th_, 4)
res = text
for (st, en, scale, ind, orig) in reversed(out):
    g = a["representations"][scale]["geometry"]
    res = res[:st] + ind + '"geometry": ' + json.dumps(g, indent=2, ensure_ascii=False).replace("\n", "\n" + ind) + res[en:]
chk = json.loads(res)
assert chk["asset"]["representations"]["200"]["geometry"]["front"] is not None
open(PATH, "w").write(res)
print("patched 2D")
