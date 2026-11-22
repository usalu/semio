"""🧶️ Generator of `elderly-person-knitting`: stage 1 (`knit_base`) plus her cardigan, glasses and grey hair, the knitting (two needles,
the work hanging from the left needle, a ball of yarn on the seat), poses, clips, thermal and structural data and all texts
(`knit_texts`). Run: python3 knit_build.py  →  writes the catalog asset."""
import copy
import math
import hw06lib as L
import knit_base as K
import knit_texts as X

SLUG = "elderly-person-knitting"
NEEDLE_L, NEEDLE_R_ = 0.30, 0.30
NEEDLE_RADIUS = 0.002
WORK = (0.15, 0.10, 0.004)
BALL_R = 0.05


def eye_centre(shape):
    xs = [p[0] for p in shape["points"]]; ys = [p[1] for p in shape["points"]]
    return (min(xs) + max(xs)) / 2, (min(ys) + max(ys)) / 2


def ellipse_pts(cx, cy, rx, ry, n=16, rot=0.0):
    c, s = math.cos(rot), math.sin(rot)
    return [(cx + rx * math.cos(2 * math.pi * i / n) * c - ry * math.sin(2 * math.pi * i / n) * s,
             cy + rx * math.cos(2 * math.pi * i / n) * s + ry * math.sin(2 * math.pi * i / n) * c) for i in range(n)]


def restyle(asset):
    """Her own look before rescaling: grey hair, dark wool trousers, dark leather shoes, a buttoned cardigan with placket over a fine
    roll-neck jumper, reading glasses drawn at 1:50 (side and front) and 1:100 (front)."""
    tones = {"hair": 0.8, "hair-bun": 0.76, "brow-l": 0.62, "brow-r": 0.62}
    for key, rep in asset["representations"].items():
        if rep["status"] != "authored": continue
        for view, shapes in rep["geometry"].items():
            for s in shapes:
                if s["id"] in tones: s["tone"] = tones[s["id"]]
                elif s.get("region") == "trousers": s["tone"] = round(s["tone"] - 0.08, 2)
                elif s.get("region") == "shoes": s["tone"] = 0.26 if s["tone"] < 0.85 else 0.5
                elif s.get("region") == "cardigan" and s["id"] != "collar": s["tone"] = round(s["tone"] - 0.06, 2)
        for h in rep["hatches"]:
            if h["id"] == "knit":
                h.update({"id": "moss", "title": L.t("Moss-stitch cardigan hatching", "Schraffur Perlmuster-Strickjacke"), "pattern": "cross", "angleDeg": 45,
                          "spacingMm": 0.7 if key == "50" else 0.8})
        for shapes in rep["geometry"].values():
            for s in shapes:
                if s.get("hatch") == "knit": s["hatch"] = "moss"
    for vol in asset["volumes"].values():
        if vol["status"] != "authored": continue
        for s in vol["body"]:
            if s["id"] in tones: s["tone"] = tones[s["id"]]
            elif s.get("material") == "wool-trousers": s["tone"] = round(s["tone"] - 0.08, 2)
            elif s.get("material") == "leather-shoe": s["tone"] = 0.26
            elif s.get("material") == "rubber-sole": s["tone"] = 0.5
            elif s.get("material") == "wool-cardigan" and not s["id"].startswith("collar"): s["tone"] = round(s["tone"] - 0.06, 2)
    for key in ("50", "100"):
        g = asset["representations"][key]["geometry"]
        det = dict(role="detail", filled=False, region="cardigan", material="wool-cardigan", tone=0.3)
        g["front"].append(L.poly_shape("placket-chest", "torso", [(0.0, 0.993), (0.0, 0.79)], closed=False, z=9, **det))
        g["front"].append(L.poly_shape("placket-waist", "lumbar", [(0.0, 0.79), (0.0, 0.535)], closed=False, z=9, **det))
        if key == "50":
            for i, y in enumerate([0.955, 0.895, 0.835]):
                g["front"].append(L.poly_shape(f"button-{i + 1}", "torso", ellipse_pts(0.0, y, 0.0075, 0.0075, 10), z=9.5, role="detail", tone=0.2, region="cardigan", material="wool-cardigan"))
            for i, y in enumerate([0.775, 0.715, 0.655]):
                g["front"].append(L.poly_shape(f"button-{i + 4}", "lumbar", ellipse_pts(0.0, y, 0.0075, 0.0075, 10), z=9.5, role="detail", tone=0.2, region="cardigan", material="wool-cardigan"))
        eyes = {s["id"]: s for s in asset["representations"]["50"]["geometry"]["front"] if s["id"] in ("eye-l", "eye-r")}
        gl = dict(role="detail", filled=False, region="face-skin", material="glasses", tone=0.15, z=12)
        cl, cr = eye_centre(eyes["eye-l"]), eye_centre(eyes["eye-r"])
        tilt = math.atan2(cl[1] - cr[1], cl[0] - cr[0])
        for side, c in (("l", cl), ("r", cr)):
            g["front"].append(L.poly_shape(f"glasses-lens-{side}", "head", ellipse_pts(c[0], c[1] + 0.002 * math.cos(tilt), 0.021, 0.016, 16, tilt), **gl))
        inner = lambda c, sgn: (c[0] - sgn * 0.021 * math.cos(tilt), c[1] - sgn * 0.021 * math.sin(tilt))
        g["front"].append(L.poly_shape("glasses-bridge", "head", [inner(cl, 1), inner(cr, -1)], closed=False, **gl))
        if key == "50":
            eye = next(s for s in asset["representations"]["50"]["geometry"]["elevation"] if s["id"] == "eye-r")
            ex, ey = eye_centre(eye)
            g["elevation"].append(L.poly_shape("glasses-lens-r", "head", [(ex - 0.017, ey - 0.012), (ex + 0.016, ey - 0.013)], closed=False, **gl))
            g["elevation"].append(L.poly_shape("glasses-temple-r", "head", [(ex + 0.016, ey - 0.013), (ex + 0.02, ey + 0.03), (0.016, 0.068)], closed=False, **gl))


def frame_from(u, up=(0.0, 1.0, 0.0)):
    n = math.sqrt(sum(x * x for x in u)); u = [x / n for x in u]
    d = sum(up[i] * u[i] for i in range(3))
    v = [up[i] - d * u[i] for i in range(3)]; n = math.sqrt(sum(x * x for x in v)); v = [x / n for x in v]
    w = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]]
    return [[u[i], v[i], w[i]] for i in range(3)]


def rest_params(parent_frame, target):
    m = L.mmul(L.mT(L.mat(parent_frame)), target)
    a, b, c = L.euler_of(m)
    return round(a, 4), round(b, 4), round(c, 4)


def place_knitting(asset, pose_bones):
    """Needles, work and yarn ball as bones; the needle rest placements are solved so that in the knitting pose the two needles
    cross 7 cm above and 3 cm ahead of the hands with their tips 3.5 cm past each other."""
    bones = asset["rig"]["bones"]
    fr = L.chain(bones, pose_bones)
    grip = (0.075 * K.S, -0.012 * K.S, 0.0)
    gl, gr = L.to_world(fr["hand-l"], grip), L.to_world(fr["hand-r"], grip)
    X = [(gl[i] + gr[i]) / 2 for i in range(3)]; X[0] += 0.08; X[1] += 0.04
    tl = [X[0] + 0.02, X[1] + 0.006, X[2] - 0.05]; tr = [X[0] + 0.026, X[1], X[2] + 0.05]
    out = {}
    for side, g, tip, hand in (("l", gl, tl, "hand-l"), ("r", gr, tr, "hand-r")):
        d = [tip[i] - g[i] for i in range(3)]; length = math.sqrt(sum(x * x for x in d))
        a, b, c = rest_params(fr[hand], frame_from(d))
        bones.append({"id": f"needle-{side}", "parent": hand, "title": X_TITLES[f"needle-{side}"], "x": round(grip[0], 4), "y": round(grip[1], 4), "angleDeg": a,
                      "abductionDeg": b, "twistDeg": c, "lengthM": round(length, 4), "joint": {"type": "revolute", "minDeg": -25, "maxDeg": 25},
                      "mass": {"kg": 0.012, "comX": round(length - NEEDLE_L / 2, 4), "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "knitting-needle-catalogues"}})
        out[side] = length
    fr = L.chain(bones, pose_bones)
    nl = fr["needle-l"]
    along = 0.45 * out["l"]
    origin = L.to_world(nl, [along, 0, 0])
    target = frame_from([0.35, -1.0, 0.0], up=nl["u"])
    a, b, c = rest_params(nl, target)
    bones.append({"id": "work", "parent": "needle-l", "title": X_TITLES["work"], "x": round(along, 4), "y": 0, "angleDeg": a, "abductionDeg": b, "twistDeg": c,
                  "lengthM": WORK[0], "joint": {"type": "revolute", "minDeg": -120, "maxDeg": 120},
                  "mass": {"kg": 0.12, "comX": WORK[0] / 2, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "knitting-needle-catalogues"}})
    bones.append({"id": "yarn-ball", "parent": "seat", "title": X_TITLES["yarn-ball"], "x": 0.16, "y": 0, "z": -0.30, "angleDeg": 90, "lengthM": 2 * BALL_R,
                  "joint": {"type": "fixed"}, "mass": {"kg": 0.1, "comX": BALL_R, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "knitting-needle-catalogues"}})
    return out


X_TITLES = {
    "needle-l": L.t("Left knitting needle (4 mm, 30 cm) with the stitches", "Linke Stricknadel (4 mm, 30 cm) mit den Maschen"),
    "needle-r": L.t("Right knitting needle (4 mm, 30 cm), the working needle", "Rechte Stricknadel (4 mm, 30 cm), die Arbeitsnadel"),
    "work": L.t("Knitted work (a scarf in progress) hanging from the left needle", "Strickstück (ein entstehender Schal) an der linken Nadel"),
    "yarn-ball": L.t("Ball of wool on the seat beside her", "Wollknäuel auf der Sitzfläche neben ihr"),
}


def solids(lengths):
    """The 3D parts in the frames of their bones."""
    ln, rn = lengths["l"], lengths["r"]
    needle = lambda side, n: {"id": f"needle-{side}", "kind": "capsule", "bone": f"needle-{side}", "material": "aluminium-needle", "role": "detail", "tone": 0.35,
                              "p0": [L.rnd(n - NEEDLE_L), 0, 0], "p1": [L.rnd(n), 0, 0], "r0": NEEDLE_RADIUS, "r1": NEEDLE_RADIUS * 0.6, "slender": True}
    work = {"id": "work", "kind": "box", "bone": "work", "material": "wool-yarn", "role": "body", "tone": 0.72, "size": [WORK[0], WORK[1], WORK[2]], "cornerRadius": 0.0015,
            "transform": {"translation": [WORK[0] / 2 - 0.004, 0, 0]}, "sheet": True}
    ball = {"id": "yarn-ball", "kind": "ellipsoid", "bone": "yarn-ball", "material": "wool-yarn", "role": "body", "tone": 0.66, "radii": [BALL_R * 0.94, BALL_R, BALL_R],
            "transform": {"translation": [BALL_R * 0.94, 0, 0]}}
    return {"needle-l": needle("l", ln), "needle-r": needle("r", rn), "work": work, "yarn-ball": ball}


def add_parts(asset, parts):
    rest = L.chain(asset["rig"]["bones"])
    show = {"500": ["work", "yarn-ball"], "200": ["work", "yarn-ball"], "100": ["needle-l", "needle-r", "work", "yarn-ball"], "50": ["needle-l", "needle-r", "work", "yarn-ball"]}
    region = {"needle-l": "knitting", "needle-r": "knitting", "work": "knitting", "yarn-ball": "yarn"}
    for key in L.SCALES:
        rep = asset["representations"][key]; vol = asset["volumes"][key]
        for pid in show[key]:
            so = copy.deepcopy(parts[pid])
            if key == "500" and pid == "work": so["size"] = [WORK[0], WORK[1], 0.004]
            vol["body"].append(so)
            vol["thermal"].append({"id": f"th-{pid}", "of": pid, "region": region[pid]})
            vol["structure"].append({"id": f"st-{pid}", "of": pid, "member": so["bone"], "part": "member"})
            fr = rest[so["bone"]]
            for view in L.VIEWS:
                if pid.startswith("needle"):
                    det = dict(role="detail", tone=0.35, z=14, region="knitting", material="aluminium-needle")
                    if view == "elevation":
                        sh = L.poly_shape(pid, so["bone"], [(so["p0"][0], 0), (so["p1"][0], 0)], filled=False, closed=False, **det)
                    else:
                        sh = L.poly_shape(pid, so["bone"], ellipse_pts(0, 0, NEEDLE_RADIUS, NEEDLE_RADIUS, 8), **det)
                        sh["solid"] = {"kind": "extruded", "sweep": "u", "z0": L.rnd(so["p0"][0]), "z1": L.rnd(so["p1"][0])}
                elif pid == "work":
                    kw = dict(z=13, tone=so["tone"], region="knitting", material="wool-yarn")
                    if key == "50": kw["hatch"] = "rows"
                    x0 = so["transform"]["translation"][0] - so["size"][0] / 2; x1 = x0 + so["size"][0]; hw = so["size"][1] / 2
                    sh = L.poly_shape(pid, so["bone"], [(x0, -hw), (x1, -hw), (x1, hw), (x0, hw)], **kw)
                    if view != "elevation":
                        oz = fr["o"][2]; ht = so["size"][2] / 2
                        sh["solid"] = {"kind": "extruded", "sweep": "w", "z0": L.rnd(oz - ht), "z1": L.rnd(oz + ht)}
                else:
                    sh = L.hull_shape(pid, so, fr, view, z=2, tone=so["tone"], region="yarn", material="wool-yarn")
                rep["geometry"][view].append(sh)
        if key in ("200", "500"):
            for side in ("l", "r"):
                so = copy.deepcopy(parts[f"needle-{side}"]); so["id"] = f"st-needle-{side}"; so.update({"member": so["bone"], "part": "member"})
                vol["structure"].append(so)
        if key == "50":
            rep["hatches"].append({"id": "rows", "title": L.t("Knitted rows of the work", "Strickreihen des Strickstücks"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.45, "penMm": 0.1})


def build():
    doc, asset, factor, old, new, f_leg, C = K.base(restyle)
    poses = X.poses(asset)
    asset["poses"] = poses
    knit = next(p for p in poses if p["id"] == "knitting")
    lengths = place_knitting(asset, knit["bones"])
    add_parts(asset, solids(lengths))
    X.finish(asset, f_leg)
    doc["asset"] = L.rnd(asset)
    return doc


if __name__ == "__main__":
    path, p = L.finalize(SLUG, build())
    print(path, p["powerW"], {k: (v["height"], v["width"], round(v["front"]["horizontal"], 3), round(v["plan"]["vertical"], 3)) for k, v in p["rows"].items()})
