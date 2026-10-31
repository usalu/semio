"""NW-MA scratch library: author shapes in world-rest coordinates and convert them to bone-local geometry; thermal and document helpers.
Every shape and number in the asset scripts is designed by hand; nothing here simplifies or generates geometry on its own."""
import json, math, re, os

CAT = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/"
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/"


def tx(en, de):
    return {"en": en, "de": de}


def r4(v):
    if isinstance(v, float):
        return round(v, 4)
    if isinstance(v, list):
        return [r4(x) for x in v]
    if isinstance(v, dict):
        return {k: r4(x) for k, x in v.items()}
    return v


def mmul(m, n):
    a, b, c, d, e, f = m
    A, B, C, D, E, F = n
    return (a * A + c * B, b * A + d * B, a * C + c * D, b * C + d * D, a * E + c * F + e, b * E + d * F + f)


def trs(x, y, deg):
    r = math.radians(deg)
    return (math.cos(r), math.sin(r), -math.sin(r), math.cos(r), x, y)


def inv(m):
    a, b, c, d, e, f = m
    det = a * d - b * c
    ia, ib, ic, id_ = d / det, -b / det, -c / det, a / det
    return (ia, ib, ic, id_, -(ia * e + ic * f), -(ib * e + id_ * f))


def ap(m, p):
    return (m[0] * p[0] + m[2] * p[1] + m[4], m[1] * p[0] + m[3] * p[1] + m[5])


def joint_fixed():
    return {"type": "fixed"}


def joint_rev(lo=None, hi=None):
    j = {"type": "revolute"}
    if lo is not None:
        j["minDeg"] = lo
    if hi is not None:
        j["maxDeg"] = hi
    return j


def joint_pris(axis, lo, hi):
    return {"type": "prismatic", "axisDeg": axis, "minM": lo, "maxM": hi}


class Rig:
    def __init__(self, root):
        self.root = root
        self.bones = []
        self.index = {}
        self.W = {}

    def add(self, id, parent, x, y, angle, length, joint, en, de, kg=None, com=(0, 0), dist="point", section=None):
        b = {"id": id, "parent": parent, "title": tx(en, de), "x": x, "y": y, "angleDeg": angle, "lengthM": length, "joint": joint}
        if kg is not None:
            b["mass"] = {"kg": kg, "comX": com[0], "comY": com[1], "distribution": dist, "source": "authored-estimate"}
        if section:
            b["section"] = section
        self.bones.append(b)
        self.index[id] = b
        local = trs(x, y, angle)
        self.W[id] = local if parent is None else mmul(self.W[parent], local)
        return b

    def mass(self):
        return sum(b.get("mass", {}).get("kg", 0) for b in self.bones)

    def world(self, pose_bones=None, root=None):
        out = {}
        off = pose_bones or {}
        rt = root or {}

        def w(bid):
            if bid in out:
                return out[bid]
            b = self.index[bid]
            o = off.get(bid, {})
            m = trs(b["x"] + o.get("dx", 0), b["y"] + o.get("dy", 0), b["angleDeg"] + o.get("angleDeg", 0))
            if b["parent"] is None:
                m = mmul(trs(rt.get("x", 0), rt.get("y", 0), rt.get("angleDeg", 0)), m)
            else:
                m = mmul(w(b["parent"]), m)
            out[bid] = m
            return m

        for k in self.index:
            w(k)
        return out

    def doc(self):
        return {"kind": "mechanical", "root": self.root, "bones": self.bones}


class Geo:
    def __init__(self, rig):
        self.rig = rig

    def _c(self, id, bone, role, z, filled, tone, region, regions, material, hatch, pen):
        d = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
        if region:
            d["region"] = region
        if regions:
            d["regions"] = regions
        if material:
            d["material"] = material
        if hatch:
            d["hatch"] = hatch
        if pen:
            d["penMm"] = pen
        return d

    def poly(self, id, bone, role, z, pts, closed=True, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
        m = inv(self.rig.W[bone])
        return {"kind": "poly", "points": [[round(q, 4) for q in ap(m, p)] for p in pts], "closed": closed, **self._c(id, bone, role, z, filled and closed, tone, region, regions, material, hatch, pen)}

    def line(self, id, bone, role, z, pts, tone=0.3, region=None, regions=None, material=None, pen=None):
        return self.poly(id, bone, role, z, pts, closed=False, tone=tone, filled=False, region=region, regions=regions, material=material, pen=pen)

    def path(self, id, bone, role, z, d, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
        m = inv(self.rig.W[bone])
        toks = re.findall(r"[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?", d)
        out, i, nums = [], 0, {"M": 2, "L": 2, "C": 6, "Q": 4}
        while i < len(toks):
            t = toks[i]
            i += 1
            if t == "Z":
                out.append("Z")
                continue
            n = nums[t]
            vals = [float(v) for v in toks[i:i + n]]
            i += n
            pts = [ap(m, (vals[k], vals[k + 1])) for k in range(0, n, 2)]
            out.append(t + " " + " ".join(f"{q[0]:.4f},{q[1]:.4f}" for q in pts))
        return {"kind": "path", "d": " ".join(out), **self._c(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)}

    def ell(self, id, bone, role, z, cx, cy, rx, ry=None, rot=0.0, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
        W = self.rig.W[bone]
        lc = ap(inv(W), (cx, cy))
        ang = math.degrees(math.atan2(W[1], W[0]))
        d = {"kind": "ellipse", "cx": round(lc[0], 4), "cy": round(lc[1], 4), "rx": rx, "ry": rx if ry is None else ry}
        loc = rot - ang
        loc = (loc + 180) % 360 - 180
        if abs(loc) > 1e-6:
            d["rotationDeg"] = round(loc, 4)
        return {**d, **self._c(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)}

    def cap(self, id, bone, role, z, p0, p1, r0, r1=None, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
        m = inv(self.rig.W[bone])
        a, b = ap(m, p0), ap(m, p1)
        return {"kind": "capsule", "x0": round(a[0], 4), "y0": round(a[1], 4), "x1": round(b[0], 4), "y1": round(b[1], 4), "r0": r0, "r1": r0 if r1 is None else r1, **self._c(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)}

    def rect(self, id, bone, role, z, x, y, w, h, tone=0.5, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
        return self.poly(id, bone, role, z, [(x, y), (x + w, y), (x + w, y + h), (x, y + h)], tone=tone, filled=filled, region=region, regions=regions, material=material, hatch=hatch, pen=pen)


def arc(cx, cy, r, a0, a1, n=8):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def ellipse_pts(cx, cy, rx, ry, a0=0, a1=360, n=24, rot=0):
    c, s = math.cos(math.radians(rot)), math.sin(math.radians(rot))
    out = []
    for k in range(n + 1):
        t = math.radians(a0 + (a1 - a0) * k / n)
        x, y = rx * math.cos(t), ry * math.sin(t)
        out.append((cx + c * x - s * y, cy + s * x + c * y))
    return out


def bar(p0, p1, w):
    dx, dy = p1[0] - p0[0], p1[1] - p0[1]
    L = math.hypot(dx, dy)
    nx, ny = -dy / L * w / 2, dx / L * w / 2
    return [(p0[0] + nx, p0[1] + ny), (p1[0] + nx, p1[1] + ny), (p1[0] - nx, p1[1] - ny), (p0[0] - nx, p0[1] - ny)]


def rot_pts(pts, cx, cy, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [(cx + c * (x - cx) - s * (y - cy), cy + s * (x - cx) + c * (y - cy)) for x, y in pts]


def pip(pt, poly):
    x, y = pt
    inside = False
    n = len(poly)
    j = n - 1
    for i in range(n):
        xi, yi = poly[i]
        xj, yj = poly[j]
        if (yi > y) != (yj > y) and x < (xj - xi) * (y - yi) / (yj - yi) + xi:
            inside = not inside
        j = i
    return inside


def pens(body, detail, texture, glazing, ground):
    return {"body": body, "detail": detail, "texture": texture, "glazing": glazing, "ground": ground}


def hatch(id, en, de, pattern, angle, spacing, pen):
    return {"id": id, "title": tx(en, de), "pattern": pattern, "angleDeg": angle, "spacingMm": spacing, "penMm": pen}


def rep(den, lo, hi, pn, hatches, elevation, dec, plan=None):
    g = {"elevation": elevation}
    if plan:
        g["plan"] = plan
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {"status": "authored", "denominator": den, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": pn, "hatches": hatches, "geometry": g, "decisions": {k: v for k, v in zip(keys, dec)}}


def unavailable(den, reason, feature, size, alts):
    return {"status": "unavailable", "denominator": den, "reason": reason, "limitingFeature": feature, "limitingSizeM": size, "alternatives": alts}


def layer(id, en, de, kind, material, thk, R, C, vap=None, src="literature", ref=None):
    d = {"id": id, "title": tx(en, de), "kind": kind, "material": material}
    if thk is not None:
        d["thicknessM"] = thk
    d["resistanceM2KW"] = R
    if vap is not None:
        d["vapourResistanceM2PaW"] = vap
    d["heatCapacityJM2K"] = C
    d["source"] = src
    if ref:
        d["reference"] = ref
    return d


def region(id, en, de, kind, model, material, area, L, geom, svf, layers, C, op=None, cond=None, hidden=None, src="authored-estimate", ref=None):
    d = {"id": id, "title": tx(en, de), "kind": kind, "model": model, "material": material, "areaM2": area, "characteristicLengthM": L, "geometry": geom, "skyViewFactor": svf, "layers": layers, "heatCapacityJM2K": C}
    if op:
        d["operating"] = {"key": op[0], "heatW": op[1]}
    if cond:
        d["conditioned"] = {"interiorTempC": cond[0], "interiorFilmM2KW": cond[1]}
    if hidden:
        d["hidden"] = hidden
    d["source"] = src
    if ref:
        d["reference"] = ref
    return d


def opkey(id, en, de, levels):
    return {"id": id, "title": tx(en, de), "levels": [{"v": v, "title": tx(a, b)} for v, a, b in levels]}


def emitter(id, kind, en, de, anchor, direction, diameter, key, flow, rise, vapour, ref=None, anchors=None, src="authored-estimate"):
    d = {"id": id, "kind": kind, "title": tx(en, de), "anchor": anchor}
    if anchors:
        d["anchors"] = anchors
    d.update({"directionDeg": direction, "diameterM": diameter, "operatingKey": key, "volumeFlowM3S": flow, "temperatureRiseK": rise, "vapourKgPerKg": vapour, "source": src})
    if ref:
        d["reference"] = ref
    return d


def anchor(id, bone, x, y, kind, en, de, support=None, fl=None, fw=None, mu=None, normal=None):
    d = {"id": id, "bone": bone, "x": x, "y": y, "kind": kind, "title": tx(en, de)}
    if support:
        d["support"] = support
    if normal is not None:
        d["normalDeg"] = normal
    if fl:
        d["footprintLengthM"] = fl
    if fw:
        d["footprintWidthM"] = fw
    if mu:
        d["frictionCoefficient"] = mu
    return d


def material(id, en, de, eps, alpha, rho=None, k=None, cp=None, src="literature", ref=None, E=None, strength=None):
    d = {"id": id, "title": tx(en, de), "emissivity": eps}
    if alpha is not None:
        d["solarAbsorptivity"] = alpha
    if rho:
        d["densityKgM3"] = rho
    if k:
        d["conductivityWmK"] = k
    if cp:
        d["specificHeatJKgK"] = cp
    if E:
        d["elasticModulusPa"] = E
    if strength:
        d["strengthPa"] = strength
    d["source"] = src
    if ref:
        d["reference"] = ref
    return d


def tbind(regions, defaults, over, emitters, op, ground_contact=None):
    out = {}
    for r in regions:
        v = dict(defaults.get(r, {"exposedFraction": 1, "sunlitFraction": 0.5}))
        v.update(over.get(r, {}))
        out[r] = v
    return {"operating": op, "regions": out, "emitters": emitters}


def pose(id, en, de, den, dde, bones, contacts, thermal, loads=None, root=None, key=False):
    d = {"id": id, "title": tx(en, de), "description": tx(den, dde)}
    if key:
        d["key"] = True
    d.update({"bones": bones, "root": root or {"x": 0, "y": 0, "angleDeg": 0}, "ground": "auto", "contacts": contacts, "thermal": thermal, "loads": loads or []})
    return d


def clip(id, en, de, den, dde, dur, fps, loop, keys, contacts, phases, events, tracks, root="none", advance=None):
    d = {"id": id, "title": tx(en, de), "description": tx(den, dde), "durationS": dur, "fps": fps, "loop": loop, "rootMotion": root}
    if advance is not None:
        d["advanceM"] = advance
    d.update({"keys": keys, "contacts": contacts, "phases": phases, "events": events, "tracks": tracks})
    return d


def key(t, pose_id, ease="smooth", bones=None):
    d = {"t": t, "pose": pose_id, "ease": ease}
    if bones:
        d["bones"] = bones
    return d


def track(target, keys, ease="smooth"):
    return {"target": target, "ease": ease, "keys": [{"t": t, "v": v} for t, v in keys]}


def event(t, kind, en, de, target=None):
    d = {"t": t, "kind": kind, "title": tx(en, de)}
    if target:
        d["target"] = target
    return d


def phase(id, a, b, en, de, ballistic=False):
    d = {"id": id, "fromS": a, "toS": b, "title": tx(en, de)}
    if ballistic:
        d["ballistic"] = True
    return d


def write(asset, folder):
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": r4(asset)}
    path = CAT + folder + "/"
    os.makedirs(path, exist_ok=True)
    with open(path + "🔣️.json", "w", encoding="utf8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    return path + "🔣️.json"


def selfcheck(asset):
    problems = []
    rg = [r["id"] for r in asset["thermal"]["regions"]]
    em = [e["id"] for e in asset["thermal"]["emitters"] if e["kind"] != "breath"]
    ops = [o["id"] for o in asset["thermal"]["operating"]]
    bones = {b["id"] for b in asset["rig"]["bones"]}
    for p in asset["poses"]:
        th = p["thermal"]
        if set(th["regions"]) != set(rg):
            problems.append(f"pose {p['id']} regions {set(rg) ^ set(th['regions'])}")
        if set(th["emitters"]) != set(em):
            problems.append(f"pose {p['id']} emitters")
        if set(th.get("operating", {})) != set(ops):
            problems.append(f"pose {p['id']} operating")
        for b in p["bones"]:
            if b not in bones:
                problems.append(f"pose {p['id']} bone {b}")
    for k, r in asset["representations"].items():
        if r["status"] != "authored":
            continue
        drawn = set()
        for s in r["geometry"]["elevation"]:
            drawn.update(s.get("regions", []) + ([s["region"]] if "region" in s else []))
            if s["bone"] not in bones:
                problems.append(f"{k} shape {s['id']} bone")
        miss = set(rg) - drawn
        miss = {m for m in miss if not next(x for x in asset["thermal"]["regions"] if x["id"] == m).get("hidden")}
        if miss:
            problems.append(f"{k} regions not drawn {miss}")
        ids = [s["id"] for s in r["geometry"]["elevation"]]
        if len(ids) != len(set(ids)):
            problems.append(f"{k} duplicate shape ids")
    if abs(asset["rig"]["bones"] and sum(b.get("mass", {}).get("kg", 0) for b in asset["rig"]["bones"]) - asset["dimensions"]["massKg"]) > 0.5:
        problems.append("mass sum")
    return problems


def bbox(asset, rig, den, pose_bones=None, plan=False):
    from render import flat
    W = rig.world(pose_bones or {})
    xs, ys = [], []
    for s in asset["representations"][str(den)]["geometry"]["plan" if plan else "elevation"]:
        pts, _ = flat(s)
        for p in pts:
            q = ap(W[s["bone"]], p)
            xs.append(q[0]); ys.append(q[1])
    return min(xs), max(xs), min(ys), max(ys)
