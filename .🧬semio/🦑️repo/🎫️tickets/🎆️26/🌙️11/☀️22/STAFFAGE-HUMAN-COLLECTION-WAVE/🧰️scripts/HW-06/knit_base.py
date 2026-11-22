"""🧶️ Stage 1 of `elderly-person-knitting`: derive the figure from `seated-reader` (same bench, same handcrafted body), rename what she
wears, drop notebook and cup, rescale the person to an 80-year-old woman of 1.61 m and return the asset with the old/new rest frames.

Anthropometry: DIN 33402-2 (women 61-65, P50 1.61 m) with the age loss of stature after 65 (about 1 cm per decade, Sorkin et al. 1999),
so 1.61 m at 80 is about the 55th percentile; mass 63 kg (BMI 24.3); de Leva (1996) female segment fractions scaled from the reader."""
import copy
import hw06lib as L

STATURE_OLD, STATURE = 1.68, 1.61
S = STATURE / STATURE_OLD
BODY_OLD, BODY = 61.0, 63.0
BENCH = {"ground", "frame-l", "frame-r", "seat", "backrest"}
LEG = {"shank-l", "shank-r", "foot-l", "foot-r", "toe-l", "toe-r"}

REGION = {"sweater": "cardigan", "jeans": "trousers", "notebook": "knitting", "cup": "yarn"}
MATERIAL = {"knit-wool": "wool-cardigan", "denim": "wool-trousers", "canvas-sneaker": "leather-shoe", "paper": "wool-yarn"}
TOKEN = {"sweater": "cardigan", "tee": "blouse", "jeans": "trousers", "sneaker": "shoe", "cowl": "collar"}
LAYER = {"sweater": "cardigan", "tee": "blouse", "jeans": "trousers", "sneaker": "shoe", "socks": "socks", "underwear": "underwear", "hair": "hair"}


def tok(x):
    return "-".join(TOKEN.get(p, p) for p in x.split("-"))


def rename(asset):
    def reg(x): return REGION.get(x, x)

    def mat(x): return MATERIAL.get(x, x)
    for rep in asset["representations"].values():
        if rep["status"] != "authored": continue
        for shapes in rep["geometry"].values():
            for s in shapes:
                if "region" in s: s["region"] = reg(s["region"])
                if "regions" in s: s["regions"] = [reg(r) for r in s["regions"]]
                if "material" in s: s["material"] = mat(s["material"])
                s["id"] = tok(s["id"])
    for vol in asset["volumes"].values():
        if vol["status"] != "authored": continue
        for lst in ("body", "thermal", "structure"):
            for s in vol[lst]:
                if "material" in s: s["material"] = mat(s["material"])
                if "region" in s: s["region"] = reg(s["region"])
                if "layer" in s: s["layer"] = LAYER.get(s["layer"], s["layer"])
                s["id"] = tok(s["id"])
                if "of" in s: s["of"] = tok(s["of"])
    for r in asset["thermal"]["regions"]:
        r["id"] = reg(r["id"]); r["material"] = mat(r["material"])
        for layer in r["layers"]:
            layer["id"] = LAYER.get(layer["id"], layer["id"]); layer["material"] = mat(layer["material"])
    for e in asset["thermal"]["emitters"]:
        if "region" in e: e["region"] = reg(e["region"])
    for p in asset["poses"]:
        th = p["thermal"]
        th["regions"] = {reg(k): v for k, v in th["regions"].items()}
    for m in asset["materials"]:
        m["id"] = mat(m["id"])


def base(pre=None):
    doc = L.load("seated-reader")
    asset = copy.deepcopy(doc["asset"])
    L.drop_bones(asset, ["page", "pad", "cup"])
    asset["anchors"] = [a for a in asset["anchors"] if a["id"] != "pad-grip"]
    asset["thermal"]["regions"] = [r for r in asset["thermal"]["regions"] if r["id"] != "cup"]
    for p in asset["poses"]:
        p["thermal"]["regions"].pop("cup", None)
    asset["materials"] = [m for m in asset["materials"] if m["id"] != "paper-cup"]
    rename(asset)
    if pre: pre(asset)
    bones = asset["rig"]["bones"]
    rest0 = L.chain(bones)
    seat_anchor = next(a for a in asset["anchors"] if a["id"] == "seat")
    C = L.to_world(rest0["pelvis"], [seat_anchor["x"], seat_anchor["y"], 0])
    heel0 = next(a for a in asset["anchors"] if a["id"] == "heel-l")
    ball0 = next(a for a in asset["anchors"] if a["id"] == "ball-l")
    y_heel0 = L.to_world(rest0["foot-l"], [heel0["x"], heel0["y"], 0])[1]

    def trial(f_leg):
        b = copy.deepcopy(bones)
        hip = next(x for x in b if x["id"] == "hip-lift")
        hip["x"], hip["y"] = C[0] + S * (hip["x"] - C[0]), C[1] + S * (hip["y"] - C[1])
        fac = {x["id"]: (1.0 if x["id"] in BENCH else f_leg if x["id"] in LEG else S) for x in b}
        for x in b:
            if x["id"] == "hip-lift": continue
            fp = fac.get(x["parent"], 1.0) if x["parent"] else 1.0
            x["x"] *= fp; x["y"] *= fp
            if "z" in x: x["z"] *= fp
            x["lengthM"] *= fac[x["id"]]
        fr = L.chain(b)
        return L.to_world(fr["foot-l"], [heel0["x"] * f_leg, heel0["y"] * f_leg, 0])[1] - y_heel0
    lo, hi = 0.9, 1.1
    for _ in range(60):
        mid = (lo + hi) / 2
        if trial(mid) > 0: lo = mid
        else: hi = mid
    f_leg = (lo + hi) / 2
    factor = {b["id"]: (1.0 if b["id"] in BENCH else f_leg if b["id"] in LEG else S) for b in bones}
    hip = next(x for x in bones if x["id"] == "hip-lift")
    hx, hy = C[0] + S * (hip["x"] - C[0]), C[1] + S * (hip["y"] - C[1])
    old, new = L.rescale(asset, factor, lambda b: (hx, hy) if b["id"] == "hip-lift" else None)
    k = BODY / BODY_OLD
    for b in bones:
        if "mass" in b and b["id"] not in BENCH: b["mass"]["kg"] = round(b["mass"]["kg"] * k, 3)
    for p in asset["poses"]:
        for key in ("hip-lift", "hip-shift"):
            off = p["bones"].get(key)
            if off:
                for d in ("dx", "dy"):
                    if d in off: off[d] = round(off[d] * S, 4)
    for c in asset["clips"]:
        for key in c.get("keys", []):
            for b in ("hip-lift", "hip-shift"):
                off = key.get("bones", {}).get(b)
                if off:
                    for d in ("dx", "dy"):
                        if d in off: off[d] = round(off[d] * S, 4)
    return doc, asset, factor, old, new, f_leg, C
