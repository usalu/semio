"""Shared assembly of a vegetation asset document from an authored species description (see sp_*.py)."""
import math, json, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

S_ROOT = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation"
LEAF_KG_M2 = 0.22

PENS = {
    "500": {"body": 0.15, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
    "200": {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25},
    "100": {"body": 0.2, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.3},
    "50": {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": 0.1, "ground": 0.35},
}
RANGES = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}
HATCHES = {
    "500": [], "200": [],
    "100": [{"id": "shade-lines", "title": T("Shade line hatch", "Schattenschraffur"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.6, "penMm": 0.1}],
    "50": [{"id": "shade-lines", "title": T("Shade line hatch", "Schattenschraffur"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.1}],
}


class Mass:
    def __init__(self, id, parent, c, rx, ry, rot, region, kind, title, s=None, lean=0.0):
        self.id, self.parent, self.c, self.rx, self.ry, self.rot, self.region, self.kind, self.title = id, parent, c, rx, ry, rot, region, kind, title
        self.s, self.lean = s, lean
        self.leaf_m2 = 0.0

    def area(self):
        return ellipse_area(self.rx, self.ry)


def attach_masses(rig, masses, levels):
    for m in masses:
        p = rig.bones[m.parent]
        ux, uy = math.cos(p.ang * D2R), math.sin(p.ang * D2R)
        proj = (m.c[0] - p.bx) * ux + (m.c[1] - p.by) * uy
        s = m.s if m.s is not None else min(max(proj, 0.3 * p.L), p.L)
        base = p.at(s)
        ang = p.ang + m.lean
        b = rig.add(m.id, m.parent, base, ang, 0.1, m.title, 0, 0, kind="foliage", limit=40.0, section=False)
        c, sn = math.cos(ang * D2R), math.sin(ang * D2R)
        lx = (m.c[0] - base[0]) * c + (m.c[1] - base[1]) * sn
        b.L = max(round(lx, 3), 0.1)
        b.com = b.to_local(m.c)
        b.dist = "point"
        b.ref = "jones-2014"
        levels[m.id] = "fol"


def finish_masses(S):
    by_region = {}
    for m in S.masses:
        by_region.setdefault(m.region, []).append(m)
    for rid, ms in by_region.items():
        total = sum(m.area() for m in ms)
        leaf_area = S.leaf_area[rid]
        for m in ms:
            m.leaf_m2 = leaf_area * m.area() / total
            b = S.rig.bones[m.id]
            b.kg = m.leaf_m2 * LEAF_KG_M2 * S.twig_factor.get(rid, 1.6)
    for id, b in S.rig.bones.items():
        if b.kind == "wood" and b.kg is None:
            b.kg = (b.density or S.density) * wood_volume(b)
            if b.id == "root-plate":
                b.kg *= S.root_mass_factor


def region_areas(S):
    areas = {}
    for b in S.rig.bones.values():
        if b.kind == "wood" and b.region:
            areas[b.region] = areas.get(b.region, 0.0) + wood_area(b) * S.bark_extra.get(b.id, 1.0)
    return areas


def bone_wind_area(S, m):
    return math.pi * m.rx * m.ry * 0.9


def lean_deg(S, bone_id, v10, droop=0.0):
    level = S.levels[bone_id]
    b = S.rig.bones[bone_id]
    d = -0.3 * S.lean_ref[level] * (max(v10, 0.0) / 8.0) ** 1.4
    if droop and level in S.droop_levels:
        d += 0.6 * droop * S.droop_levels[level] * (1 if math.cos(b.ang * D2R) < 0 else -1)
    return d


def pose_json(S, p):
    bones = {}
    for id in S.rig.order:
        if id == S.rig.order[0]:
            continue
        a = lean_deg(S, id, p.get("v10", 0.0), p.get("droop", 0.0))
        a = round(a, 2)
        if a != 0.0:
            bones[id] = {"angleDeg": a}
    regs = {}
    for r in S.regions:
        base = S.sun[r["id"]]
        regs[r["id"]] = {"exposedFraction": 1.0, "sunlitFraction": round(min(1.0, base * p.get("sun", 1.0)), 3)}
        if p.get("wet"):
            regs[r["id"]]["wettedness"] = p["wet"]
    em = {e["id"]: round(min(1.0, e["strength"] * p.get("emit", 1.0)), 3) for e in S.emitters}
    loads = []
    v10 = p.get("v10", 0.0)
    for m in S.masses:
        b = S.rig.bones[m.id]
        zz = m.c[1]
        fx = wind_force(bone_wind_area(S, m), zz, v10, cd=S.cd.get(m.kind, 0.35))
        cx, cy = b.com
        loads.append({"id": "wind-" + m.id, "kind": "wind", "title": T("Drag on %s" % m.title["en"], "Widerstand an %s" % m.title["de"]), "bone": m.id, "x": round(cx, 3), "y": round(cy, 3), "fxN": round(fx, 1), "fyN": 0, "source": "literature", "reference": "en-1991-1-4"})
    tr = S.rig.bones[S.trunk_bone]
    wx = 0.5 * 1.25 * 1.0 * tr.d0 * (v10 * (max(tr.L, 1.0) / 10) ** 0.22) ** 2
    loads.append({"id": "wind-trunk", "kind": "wind", "title": T("Drag on the stem", "Widerstand am Stamm"), "fxN": 0, "fyN": 0, "distributed": {"bone": S.trunk_bone, "fromM": 0, "toM": round(tr.L, 3), "wxNPerM": round(wx, 2), "wyNPerM": 0}, "source": "literature", "reference": "en-1991-1-4"})
    for ex in p.get("extra", lambda S: [])(S):
        loads.append(ex)
    return {"id": p["id"], "title": p["title"], "description": p["desc"], "bones": bones, "ground": "authored", "contacts": [], "thermal": {"regions": regs, "emitters": em}, "loads": loads}


def keys_from_levels(S, pose_id, v10, times_vs, lag):
    out = []
    for t, v in times_vs:
        bones = {}
        for id in S.rig.order[1:]:
            bones[id] = {"angleDeg": round(lean_deg(S, id, v), 2)}
        out.append((t, bones))
    return out


def sway_clip(S, base_pose, v10):
    T0 = S.sway_period
    n = 8
    keys = []
    for i in range(n + 1):
        t = round(T0 * i / n, 4)
        bones = {}
        for id in S.rig.order[1:]:
            lvl = S.levels[id]
            depth = S.depth[id]
            mean = lean_deg(S, id, v10, getattr(S, "breeze_droop", 0.0))
            amp = abs(mean) * S.sway_amp[lvl]
            ph = 2 * math.pi * (i % n) / n - S.sway_lag * depth
            bones[id] = {"angleDeg": round(mean + amp * math.sin(ph), 2)}
        keys.append({"t": t, "pose": base_pose, "ease": "smooth", "bones": bones})
    keys[-1]["bones"] = json.loads(json.dumps(keys[0]["bones"]))
    tracks = [{"target": "emitter:" + e["id"], "ease": "smooth", "keys": [{"t": 0, "v": e["strength"]}, {"t": round(T0 / 2, 3), "v": round(e["strength"] * 0.92, 3)}, {"t": T0, "v": e["strength"]}]} for e in S.emitters]
    return {"id": "breeze-sway", "title": T("Steady breeze sway", "Gleichmäßiges Schwanken im Wind"),
            "description": T(S.sway_desc_en % T0, S.sway_desc_de % T0.__str__()),
            "durationS": T0, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [],
            "phases": [{"id": "sway", "fromS": 0, "toS": T0, "title": T("Sway cycle", "Schwingungszyklus")}],
            "events": [{"t": 0, "kind": "motion", "title": T("Cycle start at the mean breeze lean", "Zyklusbeginn in der mittleren Windneigung")}], "tracks": tracks}


def gust_clip(S, base_pose, v_base, v_peak):
    D = 8.0
    dt = 0.5
    times = [round(i * dt, 3) for i in range(int(D / dt) + 1)]

    def vgust(t):
        g = math.exp(-((t - 2.5) / 0.9) ** 2) if t < 2.5 else math.exp(-((t - 2.5) / 1.8) ** 2)
        return v_base + (v_peak - v_base) * g

    tau = S.gust_tau
    lag_v = {lv: [] for lv in tau}
    for lv, tc in tau.items():
        y = v_base
        h = 0.05
        series = {}
        t = 0.0
        k = 0
        while t <= D + 1e-9:
            y += (vgust(t) - y) * (1 - math.exp(-h / tc))
            series[round(t, 3)] = y
            t += h
        lag_v[lv] = series
    keys = []
    for t in times:
        bones = {}
        for id in S.rig.order[1:]:
            lv = S.levels[id]
            vt = lag_v[lv][min(lag_v[lv], key=lambda x: abs(x - t))]
            bones[id] = {"angleDeg": round(lean_deg(S, id, vt, getattr(S, "breeze_droop", 0.0)), 2)}
        keys.append({"t": t, "pose": base_pose, "ease": "smooth", "bones": bones})
    tracks = []
    for m in S.masses:
        ks = [{"t": t, "v": round(max(vgust(t), 0.1) ** 1.4 / v_base ** 1.4, 3)} for t in [0, 1.2, 2.5, 3.6, 5, 8]]
        tracks.append({"target": "load:wind-" + m.id, "ease": "smooth", "keys": ks})
    ks = [{"t": t, "v": round(max(vgust(t), 0.1) ** 2 / v_base ** 2, 3)} for t in [0, 1.2, 2.5, 3.6, 5, 8]]
    tracks.append({"target": "load:wind-trunk", "ease": "smooth", "keys": ks})
    return {"id": "gust", "title": T("Gust", "Windböe"),
            "description": T(S.gust_desc_en % (v_base, v_peak), S.gust_desc_de % (v_base, v_peak)),
            "durationS": D, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [],
            "phases": [{"id": "rise", "fromS": 0, "toS": 2.5, "title": T("Gust front", "Böenfront")}, {"id": "peak", "fromS": 2.5, "toS": 3.6, "title": T("Peak", "Spitze")}, {"id": "decay", "fromS": 3.6, "toS": 8, "title": T("Decay and recovery", "Abklingen und Erholung")}],
            "events": [{"t": 2.5, "kind": "load-on", "title": T("Peak gust load", "Böenspitze")}, {"t": 5, "kind": "load-off", "title": T("Load back near the mean", "Last wieder nahe dem Mittelwert")}], "tracks": tracks}


def diurnal_clip(S, pose):
    sun = [(0, 0.0), (5.5, 0.0), (7, 0.25), (9, 0.65), (12, 1.0), (15, 0.7), (17.5, 0.2), (19, 0.0), (24, 0.0)]
    tr = [(0, 0.03), (5, 0.05), (7, 0.35), (9, 0.85), (11.5, 1.0), (14, 0.7), (16, 0.75), (18, 0.3), (20, 0.08), (24, 0.03)]
    tracks = []
    for e in S.emitters:
        tracks.append({"target": "emitter:" + e["id"], "ease": "smooth", "keys": [{"t": t, "v": round(v * e["strength"], 3)} for t, v in tr]})
    for r in S.regions:
        f = S.sun[r["id"]]
        tracks.append({"target": "region:%s:sunlitFraction" % r["id"], "ease": "smooth", "keys": [{"t": t, "v": round(v * f, 3)} for t, v in sun]})
    return {"id": "diurnal-transpiration", "title": T("Day of transpiration", "Transpirationstag"),
            "description": T("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, with stomatal closure at night and a mid-afternoon depression; leaf and bark temperatures are not authored but follow from the thermal solver.",
                            "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei Windstille. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata geschlossen, am frühen Nachmittag gibt es eine Mittagsdepression; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser."),
            "durationS": 24, "fps": 10, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": pose, "ease": "smooth"}, {"t": 12, "pose": pose, "ease": "smooth"}, {"t": 24, "pose": pose, "ease": "smooth"}],
            "contacts": [],
            "phases": [{"id": "night", "fromS": 0, "toS": 6, "title": T("Night", "Nacht")}, {"id": "morning", "fromS": 6, "toS": 12, "title": T("Morning to noon", "Morgen bis Mittag")}, {"id": "afternoon", "fromS": 12, "toS": 18, "title": T("Afternoon", "Nachmittag")}, {"id": "evening", "fromS": 18, "toS": 24, "title": T("Evening and night", "Abend und Nacht")}],
            "events": [{"t": 6, "kind": "thermal", "title": T("Sunrise", "Sonnenaufgang")}, {"t": 12, "kind": "thermal", "title": T("Solar noon", "Sonnenhöchststand")}, {"t": 18, "kind": "thermal", "title": T("Sunset", "Sonnenuntergang")}], "tracks": tracks}


def compute_depth(S):
    S.depth = {}
    for id in S.rig.order:
        p = S.rig.bones[id].parent
        S.depth[id] = 0 if p is None else S.depth[p] + 1


def regions_json(S, areas):
    out = []
    for r in S.regions:
        d = {"id": r["id"], "title": r["title"], "kind": r["kind"], "model": r["model"], "material": r["material"], "areaM2": round(r["area"] if "area" in r else areas[r["id"]], 2),
             "characteristicLengthM": r["len"], "geometry": r["geom"], "skyViewFactor": r["sky"], "layers": [], "heatCapacityJM2K": r["hc"]}
        if r["model"] == "leaf":
            d["leaf"] = {"stomatalConductanceMaxMS": r["gs"], "leafAreaIndex": r["lai"], "photosynthesis": "c3"}
        d["source"] = "authored-estimate"
        d["reference"] = r["ref"]
        out.append(d)
    return out


def emitters_json(S):
    ems, anchors = [], []
    for e in S.emitters:
        ids = []
        for i, (bid, pt) in enumerate(e["anchors"]):
            aid = "%s-%d" % (e["aid"], i + 1)
            b = S.rig.bones[bid]
            lx, ly = b.to_local(pt)
            anchors.append({"id": aid, "bone": bid, "x": round(lx, 3), "y": round(ly, 3), "kind": "emitter", "title": T("%s, anchor %d" % (e["title"]["en"], i + 1), "%s, Anker %d" % (e["title"]["de"], i + 1))})
            ids.append(aid)
        first = S.rig.bones[e["anchors"][0][0]]
        ems.append({"id": e["id"], "kind": "transpiration", "title": e["title"], "anchor": ids[0], "anchors": ids, "directionDeg": round(90 - first.ang, 1), "diameterM": e["diam"], "region": e["region"], "source": "authored-estimate", "reference": "monteith-unsworth-2013"})
    return ems, anchors


def decisions_texts(d):
    return {k: T(*d[k]) for k in ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]}


def assemble(S):
    compute_depth(S)
    attach_masses(S.rig, S.masses, S.levels)
    compute_depth(S)
    finish_masses(S)
    areas = region_areas(S)
    ems, tp_anchors = emitters_json(S)
    S.emitters_json = ems
    rep, bounds = S.make_reps(S)
    xs0, ys0, xs1, ys1 = bounds
    dims = {"heightM": round(ys1 - ys0, 2), "widthM": round(xs1 - xs0, 2), "depthM": S.depth_m, "massKg": 0}
    dims["massKg"] = int(round(sum(b.kg for b in S.rig.bones.values() if b.kg)))
    root = S.rig.bones[S.rig.order[0]]
    anchors = [{"id": "base-support", "bone": root.id, "x": 0, "y": 0, "kind": "support", "title": T("Root plate anchorage", "Verankerung des Wurzeltellers"), "support": "fixed", "normalDeg": 90, "footprintLengthM": S.footprint, "footprintWidthM": S.footprint, "frictionCoefficient": 0.6}] + tp_anchors
    poses = [pose_json(S, p) for p in S.poses]
    clips = [sway_clip(S, S.breeze_pose, S.breeze_v10), gust_clip(S, S.breeze_pose, S.breeze_v10, S.gust_peak), diurnal_clip(S, S.default_pose)]
    thermal = {"environment": S.environment, "regions": regions_json(S, areas), "emitters": ems, "passengers": [], "assumptions": S.thermal_assumptions, "operating": []}
    structure = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["base-support"], "assumptions": S.structure_assumptions, "clinical": False}
    asset = {"id": S.id, "category": "vegetation", "title": S.title, "summary": S.summary, "tags": S.tags,
             "provenance": {"author": "Semio staffage vegetation content worker NW-V1", "method": "handcrafted", "created": "2026-10-29", "references": S.references, "notes": S.notes},
             "dimensions": dims, "materials": S.materials, "rig": {"kind": "botanical", "root": root.id, "bones": S.rig.json()},
             "anchors": anchors, "representations": rep, "symbols": S.make_symbols(S, bounds), "poses": poses, "clips": clips, "thermal": thermal, "structure": structure,
             "presentation": {"defaultPose": S.default_pose, "defaultClip": "breeze-sway", "viewpoints": ["elevation"], "defaultScale": S.default_scale},
             "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    return doc, areas


def snow_extra(kg_m2, ref="en-1991-1-3"):
    def f(S):
        out = []
        for m in S.masses:
            if m.kind == "epi":
                continue
            b = S.rig.bones[m.id]
            area = math.pi * m.rx * min(m.rx, 1.6 * m.ry) * 0.8
            out.append({"id": "snow-" + m.id, "kind": "snow", "title": T("Snow on %s" % m.title["en"], "Schnee auf %s" % m.title["de"]), "bone": m.id, "x": round(b.com[0], 3), "y": round(b.com[1], 3), "fxN": 0, "fyN": -round(kg_m2 * 9.81 * area, 0), "source": "authored-estimate", "reference": ref})
        return out
    return f


def fruit_extra(kg_by_mass, ref):
    def f(S):
        out = []
        for m in S.masses:
            kg = kg_by_mass.get(m.id, 0.0)
            if kg <= 0:
                continue
            b = S.rig.bones[m.id]
            out.append({"id": "fruit-" + m.id, "kind": "carried", "title": T("Fruit on %s" % m.title["en"], "Früchte an %s" % m.title["de"]), "bone": m.id, "x": round(b.com[0], 3), "y": round(b.com[1], 3), "fxN": 0, "fyN": -round(kg * 9.81, 1), "source": "authored-estimate", "reference": ref})
        return out
    return f
