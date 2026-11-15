import sys, json, math; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib")
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/shower-head/🔣️.json"
asset = json.load(open(A))["asset"]
P = "volumes/appliances/shower-head.json"
cur = json.load(open(P))
EL = [(0.01, -0.004), (0.07, -0.01), (0.22, -0.14), (0.2, -0.26), (0.18, -0.38), (0.07, -0.36), (0.09, -0.22), (0.11, -0.06), (0.14, 0.26), (0.02196, 0.51843)]
def bez(i, t):
    p = [EL[3 * (i // 1) + k] for k in range(4)] if False else None
def bezier(seg, t):
    a, b, c, d = EL[0] if seg == 0 else EL[3 * seg], EL[3 * seg + 1], EL[3 * seg + 2], EL[3 * seg + 3]
    u = 1 - t
    return tuple(u**3 * a[k] + 3 * u * u * t * b[k] + 3 * u * t * t * c[k] + t**3 * d[k] for k in (0, 1))
def plan_z(sc):
    g = asset["representations"][str(sc)]["geometry"]["plan"]
    return [s for s in g if s["id"] == "hose"][0]["points"]
def hose_points(sc, per):
    pl = plan_z(sc); n = len(pl) - 1
    # plan polyline has n points per whole curve: sample parameter s in [0,3] by index
    pts = []
    total = 3 * per
    for i in range(total + 1):
        s = 3 * i / total; seg = min(int(s), 2); t = s - seg
        x, y = bezier(seg, t)
        f = s / 3 * n; j = min(int(f), n - 1); z = pl[j][1] + (pl[j + 1][1] - pl[j][1]) * (f - j)
        pts.append((round(x, 5), round(y, 5), round(z, 5)))
    return pts
def weights(k, n):
    f = k / n
    if f >= 0.9167: return [{"bone": "shower", "weight": 1}]
    if f >= 0.8333: return [{"bone": "elbow", "weight": 0.4}, {"bone": "shower", "weight": 0.6}]
    if f >= 0.75: return [{"bone": "elbow", "weight": 0.7}, {"bone": "shower", "weight": 0.3}]
    return None
def hose_caps(sc, per, r, idp="hose", region=None, layer=None):
    pts = hose_points(sc, per); out = []
    for k in range(len(pts) - 1):
        d = L.cap(f"{idp}-{k:02d}", "elbow", "epdm-hose", "body", 0.55, pts[k], pts[k + 1], r)
        w = weights(k, len(pts) - 1)
        if w: d["weights"] = w
        if region: d["region"] = region; d["layer"] = layer
        out.append(d)
    return out

def zat(sc, s):
    pl = plan_z(sc); n = len(pl) - 1
    f = s / 3 * n; j = min(int(f), n - 1)
    return pl[j][1] + (pl[j + 1][1] - pl[j][1]) * (f - j)
def pt(sc, s):
    seg = min(int(s), 2); x, y = bezier(seg, s - seg)
    return (round(x, 5), round(y, 5), round(zat(sc, s), 5))
def climb_params(parts):
    ss = [2 + k / 400 for k in range(401)]
    ps = [pt(50, s) for s in ss]
    cum = [0]
    for a, b in zip(ps, ps[1:]): cum.append(cum[-1] + math.dist(a, b))
    out = [2.0]
    for k in range(1, parts):
        target = cum[-1] * k / parts
        out.append(ss[min(range(401), key=lambda i: abs(cum[i] - target))])
    return out + [3.0]
def hose_pieces(sc, per, r_loop, r_climb, parts=3, idp="hose", region=None, layer=None, thermal_true=False):
    out = []
    ss = [2 * k / (2 * per) for k in range(2 * per + 1)]
    pts = [pt(sc, s) for s in ss]
    for k in range(len(pts) - 1):
        out.append(L.cap(f"{idp}-{k:02d}", "elbow", "epdm-hose", "body", 0.55, pts[k], pts[k + 1], r_loop))
    cp = climb_params(parts)
    wts = [None, [{"bone": "elbow", "weight": 0.4}, {"bone": "shower", "weight": 0.6}], [{"bone": "shower", "weight": 1}]]
    for k in range(parts):
        d = L.cap(f"{idp}-c{k}", "elbow", "epdm-hose", "body", 0.55, pt(sc, cp[k]), pt(sc, cp[k + 1]), r_climb, slender=True)
        if wts[min(k, 2) if parts == 3 else 0]: d["weights"] = wts[min(k, 2)]
        out.append(d)
    for d in out:
        if region: d["region"] = region; d["layer"] = layer
    return out
def hose_refs(sc, per, parts=3):
    ids = [f"hose-{k:02d}" for k in range(2 * per)] + [f"hose-c{k}" for k in range(parts)]
    return [L.th(f"th-{i}", i, "hose", "rubber") for i in ids]
def stepshared(sc):
    v = cur["volumes"][str(sc)]
    for s in v["structure"]:
        if s["id"] == "mass-shower":
            r = {100: 0.0155, 200: 0.0155}[sc]
            s.update(L.ell("mass-shower", "shower", "abs-chrome", "detail", 0.82, (r, r, r), (0.03, 0.08, 0)), member="shower", part="mass")
def thin_hose(sc, per, r, idp="hose"):
    n = 3 * per; pts = [pt(sc, 3 * i / n) for i in range(n + 1)]; out = []
    for k in range(n):
        d = L.cap(f"{idp}-{k:02d}", "elbow", "epdm-hose", "body", 0.55, pts[k], pts[k + 1], r)
        w = weights(k, n)
        if w: d["weights"] = w
        out.append(d)
    return out
SL_END = 2 + 125 / 300
def mixed_hose(sc, per_loop, per_rest, r_thin, r_slender=0.006, idp="hose"):
    ss = [2 * k / (2 * per_loop) for k in range(2 * per_loop + 1)]
    rest = [SL_END + (3 - SL_END) * k / per_rest for k in range(per_rest + 1)]
    out = []
    pts = [pt(sc, s) for s in ss]
    for k in range(len(pts) - 1): out.append(L.cap(f"{idp}-{k:02d}", "elbow", "epdm-hose", "body", 0.55, pts[k], pts[k + 1], r_thin))
    out.append(L.cap(f"{idp}-c0", "elbow", "epdm-hose", "body", 0.55, pt(sc, 2.0), pt(sc, SL_END), r_slender, slender=True))
    pts = [pt(sc, s) for s in rest]
    for k in range(len(pts) - 1):
        d = L.cap(f"{idp}-r{k:02d}", "elbow", "epdm-hose", "body", 0.55, pts[k], pts[k + 1], r_thin)
        w = weights(k + 1000 * 0, per_rest) if False else None
        f = (rest[k] + rest[k + 1]) / 2
        if f >= 2.75: w = [{"bone": "shower", "weight": 1}] if f >= 2.917 else ([{"bone": "elbow", "weight": 0.4}, {"bone": "shower", "weight": 0.6}] if f >= 2.833 else [{"bone": "elbow", "weight": 0.7}, {"bone": "shower", "weight": 0.3}])
        if w: d["weights"] = w
        out.append(d)
    return out
def hose_ids(body): return [s["id"] for s in body if s["id"].startswith("hose-")]
def step100():
    v = cur["volumes"]["100"]
    v["body"] = [s for s in v["body"] if not s["id"].startswith("hose-")] + mixed_hose(100, 10, 10, 0.0075)
    v["thermal"] = [s for s in v["thermal"] if not s["id"].startswith("th-hose-")] + [L.th("th-" + i, i, "hose", "rubber") for i in hose_ids(v["body"])]
    v["thermal"] = [s for s in v["thermal"] if s["id"] != "th-holder"]
    if not any(s["id"] == "th-bracket-low" for s in v["thermal"]):
        v["thermal"] += [dict(L.th("th-bracket-low", "bracket-low", "rail", "tube"), sectionFaces=["-u"]), dict(L.th("th-bracket-top", "bracket-top", "rail", "tube"), sectionFaces=["-u"])]
    stepshared(100)
def step200():
    v = cur["volumes"]["200"]
    v["body"] = [s for s in v["body"] if not s["id"].startswith("hose-")] + hose_pieces(200, 4, 0.015, 0.006)
    v["thermal"] = [s for s in v["thermal"] if not s["id"].startswith("th-hose-")] + [(L.cap("th-" + s["id"], "elbow", "epdm-hose", "body", 0.55, s["p0"], s["p1"], 0.0063, region="hose", layer="rubber") if not s.get("slender") else L.th("th-" + s["id"], s["id"], "hose", "rubber")) for s in v["body"] if s["id"].startswith("hose-")]
    stepshared(200)
def step50():
    v = cur["volumes"]["50"]
    v["body"] = [s for s in v["body"] if not s["id"].startswith("hose-")] + hose_caps(50, 12, 0.006)
    v["thermal"] = [s for s in v["thermal"] if not s["id"].startswith("th-hose-")]
    v["thermal"] += [L.th(f"th-hose-{k:02d}", f"hose-{k:02d}", "hose", "rubber") for k in range(36)]
    if not any(s["id"] == "th-holder" for s in v["thermal"]):
        v["thermal"].append(L.box("th-holder", "holder", "chrome", "body", 0.7, (0.057, 0.061, 0.057), (0, 0, 0), r=0.008, region="rail", layer="tube"))
        v["thermal"].append(L.th("th-lever", "lever", "valve", "body"))
        v["thermal"].append(L.th("th-lever-boss", "lever-boss", "valve", "body"))
    for s in v["structure"]:
        if s["id"] == "mass-shower":
            s.update(L.ell("mass-shower", "shower", "abs-chrome", "detail", 0.82, (0.012, 0.012, 0.012), (0.03, 0.08, 0)), member="shower", part="mass")
        if s["id"] == "joint-lever":
            s.update(L.ell("joint-lever", "lever", "chrome", "detail", 0.7, (0.009, 0.009, 0.004), (0.0, 0, -0.0295)), member="lever", part="joint")
def setwidth(w):
    t = open(A).read(); k = t.index('"widthM":', t.index('"dimensions"')); e = t.index(",", k)
    old = t[k + 9:e].strip()
    if float(old) != w:
        open(A, "w").write(t[:k] + f'"widthM": {w}' + t[e:]); open("author-57-dimfix.log", "a").write(f"appliances/shower-head widthM {old} -> {w}\n")
def stepdim(): setwidth(0.238)
if __name__ == "__main__":
    for a in sys.argv[1:]: globals()["step" + a]()
    json.dump(cur, open(P, "w"), ensure_ascii=False, indent=1)
