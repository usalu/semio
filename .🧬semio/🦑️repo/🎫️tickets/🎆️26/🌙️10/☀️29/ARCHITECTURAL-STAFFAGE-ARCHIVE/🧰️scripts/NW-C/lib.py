"""NW-C scratch helpers: world-space drawing helpers that emit bone-local v2 shapes plus bookkeeping for assembling the asset JSON.
Every shape, number and text is authored in the per-asset scripts; this module only converts coordinates, checks references and writes the file."""
import json, math, os, subprocess, sys

S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
CAT = S + "/🗂️catalog/🧱️components/"
T_DIR = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
GEN = T_DIR + "/🗑️generated/NW-C"


def T(en, de):
    return {"en": en, "de": de}


def r5(v):
    v = round(v, 5)
    return 0.0 if v == 0 else v


def num(v):
    return r5(v)


class Frame:
    def __init__(self, bones):
        self.b = {x["id"]: x for x in bones}
        self.cache = {}

    def world(self, bid):
        if bid in self.cache:
            return self.cache[bid]
        b = self.b[bid]
        if b["parent"] is None:
            res = (b["x"], b["y"], b["angleDeg"])
        else:
            px, py, pa = self.world(b["parent"])
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            res = (px + c * b["x"] - s * b["y"], py + s * b["x"] + c * b["y"], pa + b["angleDeg"])
        self.cache[bid] = res
        return res

    def loc(self, bid, x, y):
        ox, oy, a = self.world(bid)
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        dx, dy = x - ox, y - oy
        return (r5(c * dx + s * dy), r5(-s * dx + c * dy))

    def angle(self, bid):
        return self.world(bid)[2] % 360


class Draw:
    """Shape factory: every input is a world metre of the default (rest) pose."""

    def __init__(self, bones):
        self.f = Frame(bones)

    def _base(self, id, bone, role, z, filled, tone, kw):
        d = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
        for k in ("region", "regions", "material", "hatch", "penMm"):
            if kw.get(k) is not None:
                d[k] = kw[k]
        return d

    def poly(self, id, bone, role, z, filled, tone, pts, closed=True, **kw):
        d = self._base(id, bone, role, z, filled, tone, kw)
        d.update({"kind": "poly", "points": [list(self.f.loc(bone, x, y)) for x, y in pts], "closed": closed})
        return d

    def rect(self, id, bone, role, z, filled, tone, x0, y0, x1, y1, **kw):
        a = self.f.angle(bone)
        if abs(a) < 1e-6 or abs(a - 360) < 1e-6:
            lx, ly = self.f.loc(bone, x0, y0)
            d = self._base(id, bone, role, z, filled, tone, kw)
            d.update({"kind": "rect", "x": lx, "y": ly, "w": r5(x1 - x0), "h": r5(y1 - y0)})
            return d
        return self.poly(id, bone, role, z, filled, tone, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], True, **kw)

    def line(self, id, bone, role, z, tone, pts, **kw):
        return self.poly(id, bone, role, z, False, tone, pts, False, **kw)

    def circ(self, id, bone, role, z, filled, tone, cx, cy, r, **kw):
        lx, ly = self.f.loc(bone, cx, cy)
        d = self._base(id, bone, role, z, filled, tone, kw)
        d.update({"kind": "ellipse", "cx": lx, "cy": ly, "rx": r5(r), "ry": r5(r)})
        return d

    def path(self, id, bone, role, z, filled, tone, cmds, **kw):
        out = []
        for c in cmds:
            op = c[0]
            if op == "Z":
                out.append("Z")
                continue
            pts = c[1:]
            s = []
            for i in range(0, len(pts), 2):
                lx, ly = self.f.loc(bone, pts[i], pts[i + 1])
                s.append(f"{lx:g} {ly:g}")
            out.append(op + " " + " ".join(s))
        d = self._base(id, bone, role, z, filled, tone, kw)
        d.update({"kind": "path", "d": " ".join(out)})
        return d

    def rings(self, id, bone, role, z, filled, tone, rings, **kw):
        """Even-odd path of several closed polygon rings (an outline with holes)."""
        cmds = []
        for ring in rings:
            cmds.append(("M",) + tuple(ring[0]))
            for p in ring[1:]:
                cmds.append(("L",) + tuple(p))
            cmds.append(("Z",))
        return self.path(id, bone, role, z, filled, tone, cmds, **kw)


def bbox(shapes, frame):
    xs, ys = [], []

    def w(bone, x, y):
        ox, oy, a = frame.world(bone)
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        return ox + c * x - s * y, oy + s * x + c * y

    for sh in shapes:
        if sh["role"] == "ground":
            continue
        b, k = sh["bone"], sh["kind"]
        pts = []
        if k == "poly":
            pts = [w(b, *p) for p in sh["points"]]
        elif k == "rect":
            pts = [w(b, sh["x"] + dx * sh["w"], sh["y"] + dy * sh["h"]) for dx in (0, 1) for dy in (0, 1)]
        elif k == "ellipse":
            pts = [w(b, sh["cx"] + dx * sh["rx"], sh["cy"] + dy * sh["ry"]) for dx in (-1, 0, 1) for dy in (-1, 0, 1)]
        elif k == "path":
            nums = [float(t) for t in sh["d"].replace(",", " ").split() if not t.isalpha()]
            pts = [w(b, nums[j], nums[j + 1]) for j in range(0, len(nums) - 1, 2)]
        xs += [p[0] for p in pts]
        ys += [p[1] for p in pts]
    return min(xs), min(ys), max(xs), max(ys)


def H(id, en, de, pattern, angle, spacing, pen):
    return {"id": id, "title": T(en, de), "pattern": pattern, "angleDeg": angle, "spacingMm": spacing, "penMm": pen}


def dec(sil, ret, mer, omi, thin, hat, sty, ovl):
    return {"silhouette": T(*sil), "retained": T(*ret), "merged": T(*mer), "omitted": T(*omi), "thinElements": T(*thin),
            "hatching": T(*hat), "styles": T(*sty), "overlays": T(*ovl)}


def pens(body, detail, texture, glazing, ground):
    return {"body": body, "detail": detail, "texture": texture, "glazing": glazing, "ground": ground}


def rep(den, shapes, decisions, hatches=(), pens_=None, rng=None, plan=None):
    lo, hi = rng
    geom = {"elevation": shapes}
    if plan:
        geom["plan"] = plan
    return {"status": "authored", "denominator": den, "validRange": {"minDenominator": lo, "maxDenominator": hi},
            "pens": pens_, "hatches": list(hatches), "geometry": geom, "decisions": decisions}


def unavailable(den, reason, feature, size, alts):
    return {"status": "unavailable", "denominator": den, "reason": T(*reason), "limitingFeature": T(*feature), "limitingSizeM": size, "alternatives": alts}


def ref(id, citation, url=None):
    d = {"id": id, "citation": citation}
    if url:
        d["url"] = url
    return d


def layer(id, en, de, kind, material, d, R, src="literature", reference=None, c=None, vap=None):
    o = {"id": id, "title": T(en, de), "kind": kind, "material": material, "resistanceM2KW": r5(R), "source": src}
    if d is not None:
        o["thicknessM"] = d
    if vap is not None:
        o["vapourResistanceM2PaW"] = vap
    if c is not None:
        o["heatCapacityJM2K"] = round(c)
    if reference:
        o["reference"] = reference
    return o


def region(id, en, de, kind, model, material, area, L, geom, sky, layers, cap, src, reference=None, conditioned=None, operating=None, hidden=None):
    o = {"id": id, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": r5(area),
         "characteristicLengthM": L, "geometry": geom, "skyViewFactor": sky, "layers": layers, "heatCapacityJM2K": round(cap)}
    if conditioned is not None:
        o["conditioned"] = {"interiorTempC": conditioned[0], "interiorFilmM2KW": conditioned[1]}
    if operating is not None:
        o["operating"] = {"key": operating[0], "heatW": operating[1]}
    if hidden is not None:
        o["hidden"] = T(*hidden)
    o["source"] = src
    if reference:
        o["reference"] = reference
    return o


def RB(exposed=1.0, sun=0.0, **kw):
    d = {"exposedFraction": exposed, "sunlitFraction": sun}
    d.update(kw)
    return d


def mat(id, en, de, emis, absorb=None, rho=None, lam=None, c=None, E=None, strength=None, src="literature", reference=None):
    o = {"id": id, "title": T(en, de), "emissivity": emis}
    if absorb is not None:
        o["solarAbsorptivity"] = absorb
    if rho is not None:
        o["densityKgM3"] = rho
    if lam is not None:
        o["conductivityWmK"] = lam
    if c is not None:
        o["specificHeatJKgK"] = c
    if E is not None:
        o["elasticModulusPa"] = E
    if strength is not None:
        o["strengthPa"] = strength
    o["source"] = src
    if reference:
        o["reference"] = reference
    return o


def bone(id, parent, en, de, x, y, ang, length, joint, mass=None, section=None):
    o = {"id": id, "parent": parent, "title": T(en, de), "x": r5(x), "y": r5(y), "angleDeg": r5(ang), "lengthM": r5(length), "joint": joint}
    if mass is not None:
        o["mass"] = mass
    if section is not None:
        o["section"] = section
    return o


def bone_world(id, parent, en, de, wx, wy, wang, length, joint, bones, mass=None, section=None):
    """A bone placed by its WORLD rest position and angle; the local placement in the parent frame is derived."""
    if parent is None:
        return bone(id, None, en, de, wx, wy, wang, length, joint, mass, section)
    fr = Frame(bones)
    lx, ly = fr.loc(parent, wx, wy)
    return bone(id, parent, en, de, lx, ly, wang - fr.world(parent)[2], length, joint, mass, section)


def fixed():
    return {"type": "fixed"}


def revolute(lo, hi):
    return {"type": "revolute", "minDeg": lo, "maxDeg": hi}


def prismatic(axis, lo, hi):
    return {"type": "prismatic", "axisDeg": axis, "minM": lo, "maxM": hi}


def mass(kg, cx, cy, dist="point", src="literature", reference=None):
    o = {"kg": r5(kg), "comX": r5(cx), "comY": r5(cy), "distribution": dist, "source": src}
    if reference:
        o["reference"] = reference
    return o


def anchor(id, bone_, x, y, kind, en, de, support=None, normal=None, fl=None, fw=None, mu=None):
    o = {"id": id, "bone": bone_, "x": r5(x), "y": r5(y), "kind": kind, "title": T(en, de)}
    if support:
        o["support"] = support
    if normal is not None:
        o["normalDeg"] = normal
    if fl is not None:
        o["footprintLengthM"] = fl
    if fw is not None:
        o["footprintWidthM"] = fw
    if mu is not None:
        o["frictionCoefficient"] = mu
    return o


def load(id, kind, en, de, fx, fy, src, reference=None, anchor_=None, bone_=None, x=None, y=None, moment=None, dist=None):
    o = {"id": id, "kind": kind, "title": T(en, de)}
    if anchor_:
        o["anchor"] = anchor_
    if bone_:
        o["bone"] = bone_
        o["x"] = x
        o["y"] = y
    o["fxN"] = r5(fx)
    o["fyN"] = r5(fy)
    if moment is not None:
        o["momentNm"] = moment
    if dist:
        o["distributed"] = dist
    o["source"] = src
    if reference:
        o["reference"] = reference
    return o


def pose(id, en, de, den, dde, bones, loads, regions, emitters=None, operating=None, contacts=(), key=False):
    p = {"id": id, "title": T(en, de), "description": T(den, dde)}
    if key:
        p["key"] = True
    p["bones"] = bones
    p["ground"] = "authored"
    p["contacts"] = list(contacts)
    th = {"regions": regions, "emitters": emitters or {}}
    if operating is not None:
        th["operating"] = operating
    p["thermal"] = th
    p["loads"] = loads
    return p


def clip(id, en, de, den, dde, dur, loop, keys, phases, events, tracks=(), fps=30):
    return {"id": id, "title": T(en, de), "description": T(den, dde), "durationS": dur, "fps": fps, "loop": loop, "rootMotion": "none",
            "keys": keys, "contacts": [], "phases": phases, "events": events, "tracks": list(tracks)}


def phase(id, a, b, en, de):
    return {"id": id, "fromS": a, "toS": b, "title": T(en, de)}


def event(t, kind, en, de, target=None):
    o = {"t": t, "kind": kind, "title": T(en, de)}
    if target:
        o["target"] = target
    return o


def key(t, pose_, ease="smooth", bones=None):
    o = {"t": t, "pose": pose_, "ease": ease}
    if bones:
        o["bones"] = bones
    return o


def track(target, keys, ease="smooth"):
    return {"target": target, "ease": ease, "keys": [{"t": t, "v": v} for t, v in keys]}


def env(air, mrt, sky, ground, rh, wind, sol, diff, wdir=0):
    return {"airTempC": air, "meanRadiantTempC": mrt, "skyTempC": sky, "groundTempC": ground, "relativeHumidity": rh, "windMS": wind,
            "windDirectionDeg": wdir, "solarWM2": sol, "diffuseFraction": diff, "pressurePa": 101325}


ORDER = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations",
         "symbols", "poses", "clips", "thermal", "structure", "presentation", "exports", "completion"]

ALL_EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]


def sym(id, en, de, den, dde, sem, units, denoms, pens_, hatches, elevation, plan, note_en, note_de):
    g = {"elevation": elevation}
    if plan:
        g["plan"] = plan
    return {"id": id, "title": T(en, de), "description": T(den, dde), "semantics": sem, "units": units, "denominators": denoms,
            "pens": pens_, "hatches": hatches, "geometry": g, "dimensionalNote": T(note_en, note_de)}


def assemble(a, name):
    """Fill dimensions from the bones, verify references and every representation extent, write the asset file."""
    bones = a["rig"]["bones"]
    fr = Frame(bones)
    total = sum(b["mass"]["kg"] for b in bones if "mass" in b)
    ids = {r["id"] for r in a["thermal"]["regions"]}
    boxes = {}
    for k, rp in a["representations"].items():
        if rp["status"] != "authored":
            continue
        sh = rp["geometry"]["elevation"]
        boxes[k] = bbox(sh, fr)
        used = set()
        for s in sh:
            if "region" in s:
                used.add(s["region"])
            used.update(s.get("regions", []))
        for r in a["thermal"]["regions"]:
            if "hidden" not in r and r["id"] not in used:
                raise SystemExit(f"{name} 1:{k}: region {r['id']} undrawn")
        bad = used - ids
        if bad:
            raise SystemExit(f"{name} 1:{k}: unknown regions {bad}")
        hs = {h["id"] for h in rp["hatches"]}
        for s in sh:
            if "hatch" in s and s["hatch"] not in hs:
                raise SystemExit(f"{name} 1:{k}: hatch {s['hatch']} missing")
        sid = [s["id"] for s in sh]
        assert len(sid) == len(set(sid)), f"{name} 1:{k}: duplicate shape ids " + str({x for x in sid if sid.count(x) > 1})
        bid = {b["id"] for b in bones}
        for s in sh:
            assert s["bone"] in bid, s["id"]
    for p in a["poses"]:
        miss = ids - set(p["thermal"]["regions"])
        assert not miss, f"pose {p['id']} lacks {miss}"
    ref_box = boxes[a["_dimsFrom"]]
    del a["_dimsFrom"]
    w = ref_box[2] - ref_box[0]
    h = ref_box[3] - ref_box[1]
    for k, b in boxes.items():
        if abs((b[2] - b[0]) - w) > 0.01 or abs((b[3] - b[1]) - h) > 0.01:
            print(f"WARN {name} 1:{k} bbox {b[2]-b[0]:.3f} x {b[3]-b[1]:.3f} vs {w:.3f} x {h:.3f}")
    a["dimensions"] = {"heightM": round(h, 3), "widthM": round(w, 3), "depthM": a.pop("_depth"), "massKg": round(total, 3)}
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: a[k] for k in ORDER}}
    os.makedirs(CAT + name, exist_ok=True)
    with open(CAT + name + "/🔣️.json", "w") as f:
        f.write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
    print(name, "written", a["dimensions"])


def check(name):
    r = subprocess.run(["repo", "tools", "exec", "bun", "--", S + "/🔨️modules/🗂️catalog/🩺️check/🟦️.ts", "--asset", name], capture_output=True, text=True, cwd=S)
    print(r.stdout[-6000:], r.stderr[-1500:])
    return r.returncode


if __name__ == "__main__":
    check(sys.argv[1])
