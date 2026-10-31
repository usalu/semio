"""NW-VE scratch assembly helpers: document-level constructors, self-check, preview. Geometry and numbers live in the asset scripts."""
import json, math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
from tcommon import *

ENV = {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 28, "relativeHumidity": 0.5, "windMS": 0.5, "windDirectionDeg": 180, "solarWM2": 400, "diffuseFraction": 0.2, "pressurePa": 101325}

REFS = {
    "incropera-2011": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley. Ch. 7 and 9 (external and natural convection), Appendix A (thermophysical properties of steel, glass, rubber, aluminium).",
    "ashrae-2021": "ASHRAE Handbook Fundamentals (2021), Chapter 4 Heat Transfer and Chapter 26 Heat, Air and Moisture Control: surface convection, radiation properties and U-values of glazing and sheet metal.",
    "heywood-2018": "Heywood, J. B. (2018). Internal Combustion Engine Fundamentals, 2nd ed., McGraw-Hill. Ch. 4 and 13: air flow, exhaust composition, gas temperature and mass flow of spark-ignition and diesel engines.",
    "wong-2008": "Wong, J. Y. (2008). Theory of Ground Vehicles, 4th ed., Wiley. Ch. 1 and 3: tyre contact patch, rolling resistance, ground pressure about equal to inflation pressure, longitudinal load transfer.",
    "gillespie-1992": "Gillespie, T. D. (1992). Fundamentals of Vehicle Dynamics, SAE International. Ch. 3 and 4: static axle loads, load transfer under acceleration and braking, rolling resistance.",
    "limpert-1999": "Limpert, R. (1999). Brake Design and Safety, 2nd ed., SAE International. Ch. 5: brake energy, disc and drum temperature rise.",
    "etrto-2023": "ETRTO Standards Manual (2023): tyre and rim designations (outer diameter from rim diameter plus twice the section height) for bicycle, scooter and car tyres.",
    "wilson-2020": "Wilson, D. G., Papadopoulos, J. (2020). Bicycling Science, 4th ed., MIT Press. Ch. 9 and 10: rolling resistance, drivetrain efficiency, braking energy, typical masses.",
    "spicer-2001": "Spicer, J. B., Richardson, C. J. K., Ehrlich, M. J., Bernstein, J. R., Fukuda, M., Terada, M. (2001). Effects of frictional loss on bicycle chain drive efficiency. Journal of Mechanical Design 123(4), 598-605.",
    "en-14764": "EN 14764:2005 City and trekking bicycles, safety requirements and test methods (frame, fork, handlebar, brakes, lighting, stand).",
    "iso-13732-1": "ISO 13732-1:2006, Ergonomics of the thermal environment: Methods for the assessment of human responses to contact with surfaces, Part 1: Hot surfaces.",
    "en-13452": "EN 13452-1:2003 Railway applications, Mass transit brake systems: performance requirements (brake energy, resistor and friction brake duty).",
    "en-13803": "EN 13803:2017 Railway applications, Track alignment design parameters, track gauges 1435 mm and wider (wheel and rail dimensions of tramways).",
    "en-50163": "EN 50163:2004 Railway applications, Supply voltages of traction systems (nominal 600 V and 750 V DC) and contact wire height conventions of tramways.",
    "vdv-154": "VDV-Schrift 154 / BOStrab (Verordnung ueber den Bau und Betrieb der Strassenbahnen): vehicle gauge, floor height 300 to 400 mm for low-floor tramways, door widths.",
    "ec-2007-715": "Regulation (EC) No 715/2007 and typical type-approval data of light commercial vehicles: kerb mass, dimensions and diesel exhaust conditions of Euro 6 vans.",
    "iec-62133": "IEC 62133-2:2017 Secondary cells and batteries containing alkaline or other non-acid electrolytes, lithium systems: charging and discharging conditions of portable lithium-ion packs.",
}


def refs(*ids):
    return [{"id": i, "citation": REFS[i]} for i in ids]


def provenance(ids, notes_en, notes_de):
    return {"author": "Semio staffage content worker NW-VE (vehicles)", "method": "handcrafted", "created": "2026-10-29", "references": refs(*ids), "notes": tx(notes_en, notes_de)}


def material(id, en, de, emis, solar, dens, cond, src="authored-estimate", ref=None, cp=None):
    d = {"id": id, "title": tx(en, de), "emissivity": emis, "solarAbsorptivity": solar, "densityKgM3": dens, "conductivityWmK": cond}
    if cp:
        d["specificHeatJKgK"] = cp
    d["source"] = src
    if ref:
        d["reference"] = ref
    return d


FIXED = {"type": "fixed"}
FREE = {"type": "revolute"}


def REV(lo, hi):
    return {"type": "revolute", "minDeg": lo, "maxDeg": hi}


def PRI(axis, lo, hi):
    return {"type": "prismatic", "axisDeg": axis, "minM": lo, "maxM": hi}


def mass(kg, cx=0.0, cy=0.0, dist="point", ref=None, src="authored-estimate"):
    d = {"kg": kg, "comX": cx, "comY": cy, "distribution": dist, "source": src}
    if ref:
        d["reference"] = ref
    return d


def bone(id, parent, en, de, x, y, ang, length, joint, m=None):
    d = {"id": id, "parent": parent, "title": tx(en, de), "x": x, "y": y, "angleDeg": ang, "lengthM": length, "joint": joint}
    if m:
        d["mass"] = m
    return d


def anchor(id, b, x, y, kind, en, de, support=None, normal=None, fl=None, fw=None, mu=None):
    d = {"id": id, "bone": b, "x": x, "y": y, "kind": kind, "title": tx(en, de)}
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


def contact(id, b, x, y, en, de, fl, fw, mu=0.8):
    return anchor(id, b, x, y, "contact", en, de, "contact", 90, fl, fw, mu)


def pose(id, en, de, den, dde, bones, root, contacts, thermal, loads=(), key=False, ground="auto"):
    d = {"id": id}
    d["title"] = tx(en, de)
    d["description"] = tx(den, dde)
    if key:
        d["key"] = True
    d.update({"bones": bones, "root": root, "ground": ground, "contacts": list(contacts), "thermal": thermal, "loads": list(loads)})
    return d


def load(id, kind, en, de, fx, fy, anchor=None, bone=None, x=None, y=None, dist=None, src="authored-estimate", ref=None):
    d = {"id": id, "kind": kind, "title": tx(en, de)}
    if anchor:
        d["anchor"] = anchor
    if bone:
        d["bone"] = bone
    if x is not None:
        d["x"] = x
        d["y"] = y
    d["fxN"] = fx
    d["fyN"] = fy
    if dist:
        d["distributed"] = dist
    d["source"] = src
    if ref:
        d["reference"] = ref
    return d


def clip(id, en, de, den, dde, dur, loop, keys, contacts, phases, events, tracks, root="none", adv=None, fps=30):
    d = {"id": id, "title": tx(en, de), "description": tx(den, dde), "durationS": dur, "fps": fps, "loop": loop, "rootMotion": root}
    if adv is not None:
        d["advanceM"] = adv
    d.update({"keys": keys, "contacts": contacts, "phases": phases, "events": events, "tracks": tracks})
    return d


def ck(t, pose, ease="linear", bones=None):
    d = {"t": t, "pose": pose, "ease": ease}
    if bones:
        d["bones"] = bones
    return d


def phase(id, a, b, en, de, ballistic=False):
    d = {"id": id, "fromS": a, "toS": b, "title": tx(en, de)}
    if ballistic:
        d["ballistic"] = True
    return d


def event(t, kind, en, de, target=None):
    d = {"t": t, "kind": kind, "title": tx(en, de)}
    if target:
        d["target"] = target
    return d


def track(target, keys, ease="smooth"):
    return {"target": target, "ease": ease, "keys": [{"t": t, "v": v} for t, v in keys]}


def span(anchor, a, b):
    return {"anchor": anchor, "fromS": a, "toS": b}


def structure(assumptions):
    return {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": [], "assumptions": [tx(*a) for a in assumptions], "clinical": False}


def wheel_turn(x, r):
    return -math.degrees(x / r)


ORDER = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "presentation", "exports", "completion"]


def finish(rel, a, ids_pose=None):
    a = {k: a[k] for k in ORDER}
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": a}
    problems = self_check(a)
    os.makedirs(S + rel, exist_ok=True)
    save(rel, doc)
    return a, problems


def self_check(a):
    ids = {r["id"] for r in a["thermal"]["regions"]}
    hidden = {r["id"] for r in a["thermal"]["regions"] if r.get("hidden")}
    keys = {k["id"] for k in a["thermal"]["operating"]}
    bones = {b["id"] for b in a["rig"]["bones"]}
    ems = {e["id"] for e in a["thermal"]["emitters"]}
    dims = a["dimensions"]
    p0 = [p for p in a["poses"] if p["id"] == a["presentation"]["defaultPose"]][0]
    P = []
    tot = sum(b.get("mass", {}).get("kg", 0) for b in a["rig"]["bones"])
    print("  mass sum %.2f vs %.2f" % (tot, dims["massKg"]))
    if abs(tot - dims["massKg"]) > 0.01 * dims["massKg"]:
        P.append("mass sum %.2f vs %.2f" % (tot, dims["massKg"]))
    for p in a["poses"]:
        t = p["thermal"]
        if set(t["regions"]) != ids:
            P.append("pose %s regions mismatch %s" % (p["id"], set(t["regions"]) ^ ids))
        if set(t["operating"]) != keys:
            P.append("pose %s operating mismatch" % p["id"])
        if set(t["emitters"]) != ems:
            P.append("pose %s emitters mismatch" % p["id"])
        for b in p["bones"]:
            if b not in bones:
                P.append("pose %s bone %s" % (p["id"], b))
    anch = {x["id"] for x in a["anchors"]}
    for x in a["anchors"]:
        if x["bone"] not in bones:
            P.append("anchor bone " + x["id"])
    for e in a["thermal"]["emitters"]:
        if e["anchor"] not in anch:
            P.append("emitter anchor " + e["id"])
    for p in a["poses"]:
        for c in p["contacts"]:
            if c not in anch:
                P.append("pose contact " + c)
    posens = {p["id"] for p in a["poses"]}
    for c in a["clips"]:
        for k in c["keys"]:
            if k["pose"] not in posens:
                P.append("clip %s pose %s" % (c["id"], k["pose"]))
            for b in k.get("bones", {}):
                if b not in bones:
                    P.append("clip bone " + b)
        for t in c["tracks"]:
            kind, _, rest = t["target"].partition(":")
            if kind == "operating" and rest not in keys:
                P.append("track " + t["target"])
            if kind == "emitter" and rest not in ems:
                P.append("track " + t["target"])
            if kind == "region" and rest.split(":")[0] not in ids:
                P.append("track " + t["target"])
        for s in c["contacts"]:
            if s["anchor"] not in anch:
                P.append("clip contact " + s["anchor"])
    for r in a["thermal"]["regions"]:
        if r.get("operating") and r["operating"]["key"] not in keys:
            P.append("region %s key" % r["id"])
        if r["material"] not in {m["id"] for m in a["materials"]}:
            P.append("region %s material %s" % (r["id"], r["material"]))
        for l in r["layers"]:
            if l["material"] not in {m["id"] for m in a["materials"]}:
                P.append("layer material %s" % l["material"])
    for den, rp in a["representations"].items():
        if rp["status"] != "authored":
            continue
        shapes = rp["geometry"]["elevation"]
        seen = set()
        sid = set()
        for s in shapes:
            if s["id"] in sid:
                P.append("%s dup shape %s" % (den, s["id"]))
            sid.add(s["id"])
            if s["bone"] not in bones:
                P.append("%s shape %s bone %s" % (den, s["id"], s["bone"]))
            for r in ([s["region"]] if s.get("region") else []) + s.get("regions", []):
                if r not in ids:
                    P.append("%s shape %s region %s" % (den, s["id"], r))
                seen.add(r)
            if s.get("hatch") and s["hatch"] not in {h["id"] for h in rp["hatches"]}:
                P.append("%s shape %s hatch" % (den, s["id"]))
            if rp["pens"].get(s["role"]) is None and not s.get("penMm"):
                P.append("%s shape %s role %s has no pen" % (den, s["id"], s["role"]))
        miss = ids - seen - hidden
        if miss:
            P.append("%s regions not drawn: %s" % (den, sorted(miss)))
        bb = bbox(a, shapes, p0)
        w, h = bb[2] - bb[0], bb[3] - bb[1]
        print("  %s bbox w %.3f (%.3f) h %.3f (%.3f) x %.2f..%.2f ymin %.3f shapes %d" % (den, w, dims["widthM"], h, dims["heightM"], bb[0], bb[2], bb[1], len(shapes)))
        for lab, v, ref in (("w", w, dims["widthM"]), ("h", h, dims["heightM"])):
            if abs(v - ref) > max(0.02 * ref, 0.015):
                P.append("%s bbox %s %.3f vs %.3f" % (den, lab, v, ref))
    for p in P:
        print("  PROBLEM", p)
    return P


def Wd(a):
    ws = world(a)
    return lambda bone, x, y: ap(ws[bone], (x, y))


def unavailable(den, reason_en, reason_de, feat_en, feat_de, size, alts):
    return {"status": "unavailable", "denominator": den, "reason": tx(reason_en, reason_de), "limitingFeature": tx(feat_en, feat_de), "limitingSizeM": size, "alternatives": alts}


def symbol(id, en, de, den, dde, semantics, units, denominators, pn, hatches, elevation, plan, note_en, note_de):
    g = {"elevation": elevation, "plan": plan}
    return {"id": id, "title": tx(en, de), "description": tx(den, dde), "semantics": semantics, "units": units, "denominators": denominators, "pens": pn, "hatches": hatches, "geometry": g, "dimensionalNote": tx(note_en, note_de)}


def preview(a, tag, poses=None, dens=(500, 200, 100, 50), ppm=24, zoom=(4,)):
    for den in dens:
        rp = a["representations"][str(den)]
        if rp["status"] != "authored":
            continue
        for pid in poses or [a["presentation"]["defaultPose"]]:
            ps = [p for p in a["poses"] if p["id"] == pid][0]
            for z in (1,) + tuple(zoom):
                f = OUT + "%s-%s-%d-x%d.png" % (tag, pid, den, z)
                render(a, rp, ps, den, f, ppm=ppm * z)
                print("  wrote", f)


def fmt(v):
    return ("%.3f" % v).rstrip("0").rstrip(".")


def inv(m):
    a, b, c, d, e, f = m
    det = a * d - b * c
    ia, ib, ic, id_ = d / det, -b / det, -c / det, a / det
    return (ia, ib, ic, id_, -(ia * e + ic * f), -(ib * e + id_ * f))


class Loc:
    """Converts model-frame rest coordinates into bone-local coordinates, so shapes are authored in model metres."""

    def __init__(self, rig_bones):
        self.a = {"rig": {"bones": rig_bones}}
        ws = world(self.a)
        self.inv = {k: inv(v) for k, v in ws.items()}
        self.ang = {b["id"]: sum_angle(rig_bones, b["id"]) for b in rig_bones}

    def pt(self, bone, x, y):
        p = ap(self.inv[bone], (x, y))
        return (round(p[0], 4), round(p[1], 4))

    def pts(self, bone, ps):
        return [self.pt(bone, x, y) for x, y in ps]

    def d(self, bone, cmds):
        out = []
        for c in cmds:
            k = c[0]
            if k == "Z":
                out.append("Z")
            elif k == "A":
                _, rx, ry, rot, la, sw, x, y = c
                p = self.pt(bone, x, y)
                out.append("A %s %s %s %d %d %s,%s" % (fmt(rx), fmt(ry), fmt(rot - self.ang[bone]), la, sw, fmt(p[0]), fmt(p[1])))
            else:
                vals = c[1:]
                ps = [self.pt(bone, vals[i], vals[i + 1]) for i in range(0, len(vals), 2)]
                out.append(k + " " + " ".join("%s,%s" % (fmt(p[0]), fmt(p[1])) for p in ps))
        return " ".join(out)


def sum_angle(bones, bid):
    by = {b["id"]: b for b in bones}
    t = 0
    while bid is not None:
        t += by[bid]["angleDeg"]
        bid = by[bid]["parent"]
    return t


def arc_poly(cx, cy, r, a0, a1, n=24):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def ring_cmds(cx, cy, ro, ri, n=48):
    out = []
    for r in (ro, ri):
        ps = [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]
        out.append(("M",) + ps[0])
        for p in ps[1:]:
            out.append(("L",) + p)
        out.append(("Z",))
    return out


def band_poly(cx, cy, ri, ro, a0, a1, n=24):
    return arc_poly(cx, cy, ro, a0, a1, n) + arc_poly(cx, cy, ri, a1, a0, n)


class Draw:
    """Shape builders in model-frame rest coordinates, bound to a Loc."""

    def __init__(self, L):
        self.L = L

    def stroke(self, id, bn, pts, pen, region=None, regions=None, z=3, material=None, role="body", tone=0.3):
        return poly(id, bn, role, z, self.L.pts(bn, pts), False, tone, False, region=region, regions=regions, material=material, pen=pen)

    def poly(self, id, bn, pts, region=None, regions=None, z=3, tone=0.5, material=None, role="body", pen=None, filled=True, closed=True):
        return poly(id, bn, role, z, self.L.pts(bn, pts), closed, tone, filled, region=region, regions=regions, material=material, pen=pen)

    def cap(self, id, bn, p0, p1, r0, r1=None, region=None, regions=None, z=3, tone=0.5, material=None, role="body", pen=None):
        a, b = self.L.pt(bn, *p0), self.L.pt(bn, *p1)
        return cap(id, bn, role, z, a[0], a[1], b[0], b[1], r0, r1, tone=tone, region=region, regions=regions, material=material, pen=pen)

    def ell(self, id, bn, c, rx, ry=None, region=None, regions=None, z=3, tone=0.5, material=None, role="body", pen=None, filled=True, rot=None):
        p = self.L.pt(bn, *c)
        return ell(id, bn, role, z, p[0], p[1], rx, ry, tone=tone, filled=filled, rot=rot, region=region, regions=regions, material=material, pen=pen)

    def ring(self, id, bn, c, ro, ri, region=None, regions=None, z=4, tone=0.1, material=None, role="body", pen=None):
        return path(id, bn, role, z, self.L.d(bn, ring_cmds(c[0], c[1], ro, ri)), tone, True, region=region, regions=regions, material=material, pen=pen)

    def rect(self, id, bn, x0, y0, x1, y1, region=None, regions=None, z=3, tone=0.5, material=None, role="body", pen=None):
        return self.poly(id, bn, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], region, regions, z, tone, material, role, pen)

    def band(self, id, bn, c, ri, ro, a0, a1, region=None, regions=None, z=3.5, tone=0.55, material=None, pen=None):
        return self.poly(id, bn, band_poly(c[0], c[1], ri, ro, a0, a1), region, regions, z, tone, material, "body", pen)

    def spokes(self, prefix, bn, c, n, r0, r1, pen, region, material, z=4.5, phase=0.1):
        out = []
        for i in range(n):
            a = 2 * math.pi * i / n + phase
            out.append(self.stroke("%s-%d" % (prefix, i + 1), bn, [(c[0] + r0 * math.cos(a), c[1] + r0 * math.sin(a)), (c[0] + r1 * math.cos(a), c[1] + r1 * math.sin(a))], pen, region, z=z, material=material, role="detail", tone=0.7))
        return out


def dec(*a):
    return decisions(*[tx(*x) for x in a])


def steel_layer(t, cap_, mat="paint", ref="incropera-2011"):
    return layer("wall", "Metal wall %.1f mm" % (t * 1000), "Metallwand %.1f mm" % (t * 1000), "metal", mat, t, t / 50, cap_, source="literature", ref=ref)


EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
