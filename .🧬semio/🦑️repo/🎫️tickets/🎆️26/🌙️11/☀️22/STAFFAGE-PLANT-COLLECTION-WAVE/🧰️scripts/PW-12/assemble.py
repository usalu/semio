"""🧩️ PW-12 assembly: physics (masses, loads, thermal regions, emitters), poses, clips, representations, volumes, symbols and the
manifest row of one shrub from its species description. The NW-V1 helpers `lean_deg`, `sway_clip`, `gust_clip` and `diurnal_clip`
are imported from their place (see kit.py); everything species-specific comes from the species file.

@see ./kit.py"""
import json
import math
import sys
from types import SimpleNamespace

from kit import T, D2R, wind_force, nw, SCALES, RANGES, PENS, HATCHES, EXPORTS, Lobe, Cap, build_rig, representation, world_boxes, extents, write, manifest_row, r, scallop

RHO_AIR = 1.25


def cap_area(c):
    L = math.dist(c.p0, c.p1)
    return math.pi * (c.r0 + c.r1) * math.hypot(L, c.r0 - c.r1)


def cap_volume(c):
    L = math.dist(c.p0, c.p1)
    return math.pi / 3 * L * (c.r0 ** 2 + c.r0 * c.r1 + c.r1 ** 2)


def bark_area(sp):
    """🪵️ Outer bark of the modelled stems at 1:50: the stem capsules (frustum mantles and their rounded ends) and the mantle and top of the root collar."""
    fine = sp["scales"]["50"]
    caps = [c for c in fine["caps"] if c.region == "stem-bark"]
    r0, r1 = fine["collar"][0], fine["collar"][1]
    h = sp["collar"]["h"]
    collar = math.pi * (r0 + r1) * math.hypot(h, r0 - r1) + math.pi * r1 * r1
    return r(sum(cap_area(c) + 2 * math.pi * (c.r0 ** 2 + c.r1 ** 2) for c in caps) + collar, 2)


def frontal(lb):
    return math.pi * lb.rad[2] * lb.rad[1]


def masses(sp, rig):
    """⚖️ Green wood of every stem segment and the root collar; leaves, twigs and flowers of the foliage bones (from the 1:50 lobes)."""
    fine = sp["scales"]["50"]
    caps = {}
    for c in fine["caps"]:
        caps.setdefault(c.bone, []).append(c)
    for id in rig.order:
        b = rig.bones[id]
        if b.kind == "wood":
            vol = sum(cap_volume(c) for c in caps.get(id, []) if c.region == "stem-bark")
            b.kg = vol * sp["density"]
            b.com = (b.L / 2, 0.0)
        elif b.kind == "root":
            b.kg = sp["collar"]["kg"]
            b.com = (b.L / 2, 0.0)
            b.dist = "uniform"
    leaf = {}
    for lb in fine["lobes"]:
        if lb.region in sp["leaf_area"]:
            leaf.setdefault(lb.region, []).append(lb)
    share = {}
    for reg, lbs in leaf.items():
        tot = sum(frontal(lb) for lb in lbs)
        for lb in lbs:
            share[lb.id] = sp["leaf_area"][reg] * frontal(lb) / tot
    sp["_leaf_of"] = {}
    flowers = [lb for lb in fine["lobes"] if lb.region == "flowers"] + [c for c in fine["caps"] if c.region == "flowers"]
    for id in rig.order:
        b = rig.bones[id]
        if b.kind != "foliage":
            continue
        own = [lb for lb in fine["lobes"] if lb.bone == id and lb.region != "flowers"]
        fl = [f for f in flowers if f.bone == id]
        la = sum(share.get(lb.id, 0.0) for lb in own)
        sp["_leaf_of"][id] = la
        kg = la * sp["leaf_kg_m2"] * sp["twig_factor"] + len(fl) * sp["flower_kg"] * sp["flowers_per_drawn"]
        wsum = sum(frontal(lb) for lb in own) or 1.0
        cx = sum(b.local(lb.c)[0] * frontal(lb) for lb in own) / wsum if own else 0.0
        cy = sum(b.local(lb.c)[1] * frontal(lb) for lb in own) / wsum if own else 0.0
        b.kg, b.com = kg, (cx, cy)
        b.source, b.ref = "authored-estimate", sp["leaf_ref"]


def wind_loads(sp, rig, v10, cd, factor=1.0):
    out = []
    fine = sp["scales"]["50"]
    for id in rig.order:
        b = rig.bones[id]
        if b.kind != "foliage":
            continue
        own = [lb for lb in fine["lobes"] if lb.bone == id]
        area = sum(frontal(lb) for lb in own) * sp["porosity"]
        zc = sum(lb.c[1] for lb in own) / max(1, len(own))
        fx = wind_force(area, zc, v10, cd=cd) * factor
        out.append({"id": "wind-" + id, "kind": "wind", "title": T("Drag on %s" % b.title["en"].lower(), "Widerstand an %s" % lower_de(b.title["de"])), "bone": id, "x": r(b.com[0], 3), "y": r(b.com[1], 3), "fxN": r(fx, 1), "fyN": 0, "source": "literature", "reference": "en-1991-1-4"})
    for id in rig.order:
        b = rig.bones[id]
        if b.kind != "wood" or b.level != "stem":
            continue
        v = v10 * (max(b.o[1] + b.L / 2, 1.0) / 10) ** 0.22
        wx = 0.5 * RHO_AIR * 1.0 * b.d0 * v * v * factor
        lid = "wind-trunk" if id == sp["trunk"] else "wind-" + id
        out.append({"id": lid, "kind": "wind", "title": T("Drag on %s" % b.title["en"].lower(), "Widerstand an %s" % lower_de(b.title["de"])), "fxN": 0, "fyN": 0, "distributed": {"bone": id, "fromM": 0, "toM": r(b.L, 3), "wxNPerM": r(wx, 3), "wyNPerM": 0}, "source": "literature", "reference": "en-1991-1-4"})
    return out


def lower_de(s):
    return s


def bindings(sp, sun=1.0, emit=1.0, wet=None, sunlit=None):
    regs = {}
    for rg in sp["regions"]:
        f = sunlit.get(rg["id"], rg["sun"] * sun) if sunlit else rg["sun"] * sun
        regs[rg["id"]] = {"exposedFraction": 1, "sunlitFraction": r(min(1.0, f), 3)}
        if wet is not None and rg["id"] in wet:
            regs[rg["id"]]["wettedness"] = wet[rg["id"]]
    ems = {e["id"]: r(min(1.0, e["strength"] * emit), 3) for e in sp["emitters"]}
    return {"regions": regs, "emitters": ems}


def pose(sp, rig, S, p):
    """🧍️ One named pose: wind lean per level from the NW-V1 law, an optional authored droop, thermal binding and loads."""
    bones = {}
    for id in rig.order[1:]:
        a = nw.lean_deg(S, id, p.get("v10", 0.0))
        off = p.get("offsets", {})
        off = off(sp, rig) if callable(off) else off
        a += off.get(id, 0.0)
        a = round(a, 2)
        if a != 0.0:
            bones[id] = {"angleDeg": a}
    loads = wind_loads(sp, rig, p.get("v10", 0.0), p.get("cd", sp["cd"]))
    extra = p.get("extra", [])
    for ex in (extra(sp, rig) if callable(extra) else extra):
        loads.append(ex)
    th = bindings(sp, p.get("sun", 1.0), p.get("emit", 1.0), p.get("wet"), p.get("sunlit"))
    return {"id": p["id"], "title": p["title"], "description": p["desc"], "bones": bones, "ground": "authored", "contacts": [], "thermal": th, "loads": loads}


def carried(sp, rig, kind, per_bone_kg, title, ref):
    out = []
    for id, kg in per_bone_kg.items():
        b = rig.bones[id]
        if kg <= 0:
            continue
        out.append({"id": "%s-%s" % (kind, id), "kind": "carried" if kind != "snow" else "snow", "title": T("%s on %s" % (title[0], b.title["en"].lower()), "%s an %s" % (title[1], b.title["de"])), "bone": id, "x": r(b.com[0], 3), "y": r(b.com[1], 3), "fxN": 0, "fyN": -r(kg * 9.80665, 1), "source": "authored-estimate", "reference": ref})
    return out


def symbol(sp, rig, dims, box):
    """🔣️ The plan symbol at true extent: crown outline and stem clump in all three views (outlines in model metres)."""
    x0, x1, y0, y1, z0, z1 = box
    cb = sp["symbol_crown_base"]
    cx, cz = (x0 + x1) / 2, (z0 + z1) / 2
    rx, rz, ry = (x1 - x0) / 2, (z1 - z0) / 2, (y1 - cb) / 2
    cy = cb + ry
    root = rig.bones["root-plate"]
    k = sp["symbol_k"]
    el = scallop(cx, cy, rx, ry, k, 0.0)
    stem = sp["symbol_stem"]
    st_el = [(-stem[0], 0), (stem[0], 0), (stem[1], cb + 0.1), (-stem[1], cb + 0.1)]
    pens = {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}
    base = {"bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "closed": True}
    geo = {
        "elevation": [{"id": "crown", **base, "points": [[r(x), r(y)] for x, y in el]}, {"id": "stems", **base, "role": "detail", "points": [[r(x), r(y)] for x, y in st_el]}],
        "front": [{"id": "crown", **base, "points": [[r(x), r(y)] for x, y in scallop(cz, cy, rz, ry, k, 0.0)]}, {"id": "stems", **base, "role": "detail", "points": [[r(-stem[0]), 0], [r(stem[0]), 0], [r(stem[1]), r(cb + 0.1)], [r(-stem[1]), r(cb + 0.1)]]}],
        "plan": [{"id": "crown", **base, "points": [[r(x), r(y)] for x, y in scallop(cx, cz, rx / 1.04, rz / 1.04, 8, 0.10, math.pi / 2)]}, {"id": "stem-clump", **base, "role": "detail", "filled": True, "tone": 0.3, "points": [[r(stem[0] * math.cos(a)), r(stem[0] * math.sin(a))] for a in [2 * math.pi * i / 16 for i in range(16)]]}],
    }
    w, d, h = dims["widthM"], dims["depthM"], dims["heightM"]
    note = T("A conventional sign of the shrub drawn at its true extent in model metres in all three views (%s m ahead, %s m across and %s m high): the outline is the crown, the stem clump is drawn at its real position and spread. It is a symbol and is never used silently for a pictorial scale." % (fmt(w), fmt(d), fmt(h)),
             "Ein konventionelles Zeichen des Strauchs in seiner wahren Ausdehnung in Modellmetern in allen drei Ansichten (%s m in Blickrichtung, %s m quer und %s m hoch): Der Umriss ist die Krone, der Stammhorst ist in wirklicher Lage und Spreizung gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet." % (fmt(w, True), fmt(d, True), fmt(h, True)))
    return [{"id": sp["id"] + "-symbol", "title": T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
             "description": T("A conventional plan sign of the shrub for site and landscape plans: outline of the true crown extent and the stem clump. It is a separate symbol and is never used silently for a pictorial scale.",
                              "Ein konventionelles Grundrisszeichen des Strauchs für Lage- und Freiraumpläne: Umriss der wahren Kronenausdehnung und der Stammhorst. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
             "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": pens, "hatches": [], "geometry": geo, "dimensionalNote": note}]


def fmt(v, de=False):
    s = ("%.2f" % v)
    return s.replace(".", ",") if de else s


def build(sp):
    rig = build_rig(sp)
    masses(sp, rig)
    S = SimpleNamespace()
    S.rig = rig
    S.levels = {id: rig.bones[id].level for id in rig.order}
    S.lean_ref = sp["lean_ref"]
    S.droop_levels = {}
    S.sway_amp = sp["sway_amp"]
    S.sway_lag = sp["sway_lag"]
    S.sway_period = sp["sway_period"]
    S.gust_tau = sp["gust_tau"]
    S.sway_desc_en, S.sway_desc_de = sp["sway_desc"]
    S.gust_desc_en, S.gust_desc_de = sp["gust_desc"]
    S.emitters = sp["emitters"]
    S.regions = sp["regions"]
    S.sun = {rg["id"]: rg["sun"] for rg in sp["regions"]}
    S.masses = [SimpleNamespace(id=id) for id in rig.order if rig.bones[id].kind == "foliage"]
    nw.compute_depth(S)
    for id in rig.order:
        S.levels.setdefault(id, "root")
    poses = [pose(sp, rig, S, p) for p in sp["poses"]]
    sway = nw.sway_clip(S, sp["breeze_pose"], sp["breeze_v10"])
    sway["id"] = "wind-sway"
    sway["title"] = T("Wind sway", "Schwanken im Wind")
    gust = nw.gust_clip(S, sp["breeze_pose"], sp["breeze_v10"], sp["gust_peak"])
    trunk = [t for t in gust["tracks"] if t["target"] == "load:wind-trunk"][0]
    for id in rig.order:
        b = rig.bones[id]
        if b.kind == "wood" and b.level == "stem" and id != sp["trunk"]:
            gust["tracks"].append({"target": "load:wind-" + id, "ease": "smooth", "keys": json.loads(json.dumps(trunk["keys"]))})
    diurnal = nw.diurnal_clip(S, sp["default_pose"])
    clips = [sway, gust, diurnal]
    for c in clips:
        for k in c["keys"]:
            if "bones" in k:
                k["bones"] = {b: o for b, o in k["bones"].items() if b in rig.bones and b != "root-plate"}

    lo50, hi50 = drawn_extent(sp, "50", crest=False)
    for scale in SCALES[1:]:
        fit_extents(sp, scale, lo50, hi50)
    reps, vols, boxes = {}, {}, None
    for scale in SCALES:
        geo, body, thermal, structure = representation(sp, rig, scale)
        D = sp["decisions"][scale]
        reps[scale] = {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": RANGES[scale][0], "maxDenominator": RANGES[scale][1]}, "pens": PENS[scale], "hatches": HATCHES[scale],
                       "geometry": geo, "decisions": {k: T(*D[k]) for k in ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]}}
        vols[scale] = {"status": "authored", "denominator": int(scale), "body": body, "thermal": thermal, "structure": structure, "decisions": [T(*t) for t in sp["volume_decisions"][scale]]}
    lo, hi = drawn_extent(sp, "50", crest=False)
    box = (lo[0], hi[0], lo[1], hi[1], lo[2], hi[2])
    dims = {"heightM": r(box[3] - max(0.0, box[2]), 3), "widthM": r(box[1] - box[0], 3), "depthM": r(box[5] - box[4], 3), "massKg": r(sum(b.kg for b in rig.bones.values() if b.kg), 1)}
    dims.update(sp.get("measured_dims", {}))
    regions = []
    for rg in sp["regions"]:
        d = {k: rg[k] for k in ["id", "title", "kind", "model", "material"]}
        d["areaM2"] = rg["area"] if "area" in rg else bark_area(sp)
        if rg.get("exchange"):
            d["areaBasis"] = "exchange"
        d.update({"characteristicLengthM": rg["len"], "geometry": rg["geom"], "skyViewFactor": rg["sky"], "layers": [], "heatCapacityJM2K": rg["hc"]})
        if rg["model"] == "leaf":
            d["leaf"] = {"stomatalConductanceMaxMS": rg["gs"], "leafAreaIndex": rg["lai"], "photosynthesis": "c3"}
        d["source"] = rg.get("source", "authored-estimate")
        d["reference"] = rg["ref"]
        regions.append(d)
    anchors = [{"id": "base-support", "bone": "root-plate", "x": 0, "y": 0, "kind": "support", "title": T("Root crown anchorage", "Verankerung des Wurzelstocks"), "support": "fixed", "normalDeg": 90, "footprintLengthM": sp["footprint"], "footprintWidthM": sp["footprint"], "frictionCoefficient": 0.6}]
    ems = []
    for e in sp["emitters"]:
        ids = []
        for i, (bid, p) in enumerate(e["anchors"]):
            b = rig.bones[bid]
            u, v, w = b.local(p)
            aid = "%s-%d" % (e["aid"], i + 1)
            anchors.append({"id": aid, "bone": bid, "x": r(u, 3), "y": r(v, 3), "z": r(w, 3), "kind": "emitter", "title": T("%s, anchor %d" % (e["title"]["en"], i + 1), "%s, Anker %d" % (e["title"]["de"], i + 1))})
            ids.append(aid)
        first = rig.bones[e["anchors"][0][0]]
        ems.append({"id": e["id"], "kind": "transpiration", "title": e["title"], "anchor": ids[0], "anchors": ids, "directionDeg": r(90 - first.ang, 1), "diameterM": e["diam"], "region": e["region"], "source": "authored-estimate", "reference": "monteith-unsworth-2013"})
    thermal = {"environment": sp["environment"], "regions": regions, "emitters": ems, "passengers": [], "assumptions": [T(*a) for a in sp["thermal_assumptions"]], "operating": []}
    structure = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["base-support"], "assumptions": [T(*a) for a in sp["structure_assumptions"]], "clinical": False}
    asset = {"id": sp["id"], "category": "vegetation", "title": sp["title"], "summary": sp["summary"], "tags": sp["tags"],
             "provenance": {"author": "Semio staffage plant collection worker PW-12", "method": "handcrafted", "created": "2026-11-22", "references": sp["references"], "notes": sp["notes"]},
             "dimensions": dims, "materials": sp["materials"], "rig": {"kind": "botanical", "root": "root-plate", "bones": rig.json()}, "anchors": anchors,
             "representations": reps, "symbols": symbol(sp, rig, dims, box), "poses": poses, "clips": clips, "thermal": thermal, "structure": structure, "volumes": vols,
             "presentation": {"defaultPose": sp["default_pose"], "defaultClip": "wind-sway", "defaultScale": sp["default_scale"]}, "exports": EXPORTS, "completion": "authored"}
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    path = write(doc, sp["id"])
    manifest_row(asset, sp["distinction"])
    return doc, path, rig


def fit_extents(sp, scale, lo50, hi50, eps=0.004):
    """📐️ Brings the outermost primitives of a coarser scale onto the extremes of 1:50 (the `dimensions`): a lobe that defines an extreme grows
    or shrinks on that side only (its centre moves by half the change), a flower or stem end that defines it moves with it, so every scale has
    the physical extent of the plant while its composition stays as authored."""
    S = sp["scales"][scale]
    for _ in range(3):
        lo, hi = drawn_extent(sp, scale, crest=False)
        for k in range(3):
            for side, want, have in ((1, hi50[k], hi[k]), (-1, lo50[k], lo[k])):
                if k == 1 and side < 0:
                    continue
                d = (want - have) * side
                if abs(d) < 1e-4:
                    continue
                for lb in S["lobes"]:
                    edge = lb.c[k] + side * lb.rad[k]
                    if abs(edge - have) <= eps:
                        rad = list(lb.rad)
                        c = list(lb.c)
                        rad[k] = max(0.02, rad[k] + d / 2)
                        c[k] += side * d / 2
                        lb.rad, lb.c = tuple(rad), tuple(c)
                for cp in S.get("caps", []):
                    if cp.role == "thermal-only":
                        continue
                    for which, rr in (("p0", cp.r0), ("p1", cp.r1)):
                        p = list(getattr(cp, which))
                        if abs(p[k] + side * rr - have) <= eps and not (k == 1 and p[1] < 0.5):
                            p0, p1 = list(cp.p0), list(cp.p1)
                            p0[k] += side * d
                            p1[k] += side * d
                            cp.p0, cp.p1 = tuple(p0), tuple(p1)
                            break


def drawn_extent(sp, scale, crest=True):
    """📏️ World extent of the drawn primitives of a scale (scallop crests included), to compare every scale with `dimensions`."""
    S = sp["scales"][scale]
    lo, hi = [9e9] * 3, [-9e9] * 3
    def grow(c, rad):
        for k in range(3):
            lo[k] = min(lo[k], c[k] - rad[k])
            hi[k] = max(hi[k], c[k] + rad[k])
    rc = S["collar"]
    grow((0, (rc[2] if len(rc) > 2 else sp["collar"]["h"]) / 2, 0), (rc[0], (rc[2] if len(rc) > 2 else sp["collar"]["h"]) / 2, rc[0]))
    for c in S.get("caps", []):
        if c.role == "thermal-only":
            continue
        grow(c.p0, (c.r0,) * 3)
        grow(c.p1, (c.r1,) * 3)
    for lb in S["lobes"]:
        f = 1 + 0.4 * lb.amp if (lb.k and crest) else 1
        grow(lb.c, tuple(x * f for x in lb.rad))
    return lo, hi


def view_extents(doc, rig, scale):
    """📐️ Per-view extents of the shipped 2D shapes of a scale at rest in world metres: elevation (x, y), front (z, y), plan (x, z)."""
    geo = doc["asset"]["representations"][scale]["geometry"]
    out = {}
    for view, (ia, ib) in (("elevation", (0, 1)), ("front", (2, 1)), ("plan", (0, 2))):
        xs, ys = [], []
        for s in geo[view]:
            if s["role"] == "ground":
                continue
            b = rig.bones[s["bone"]]
            if s["kind"] == "poly":
                pts = s["points"]
                rr = [0.0] * len(pts)
            else:
                pts = [(s["x0"], s["y0"]), (s["x1"], s["y1"])]
                rr = [s["r0"], s["r1"]]
            for (px, py), rad in zip(pts, rr):
                if view == "elevation":
                    wx, wy, _ = b.world(px, py)
                else:
                    wx, wy = px, py
                xs += [wx - rad, wx + rad]
                ys += [wy - rad, wy + rad]
        out[view] = (max(xs) - min(xs), (max(ys) - max(0.0, min(ys))) if view != "plan" else (max(ys) - min(ys)))
    return out


def report(doc, rig, sp=None):
    a = doc["asset"]
    if sp is not None:
        d = a["dimensions"]
        want = {"elevation": (d["widthM"], d["heightM"]), "front": (d["depthM"], d["heightM"]), "plan": (d["widthM"], d["depthM"])}
        for s in SCALES:
            e = view_extents(doc, rig, s)
            dev = {v: tuple(round((e[v][k] - want[v][k]) / max(0.015 / want[v][k], 0.02) / want[v][k], 2) for k in (0, 1)) for v in e}
            lo, hi = drawn_extent(sp, s, crest=False)
            print("[DEBUG] extent 1:%s 2D deviation in tolerances %s | 3D x %.3f y %.3f z %.3f" % (s, dev, hi[0] - lo[0], hi[1] - max(0, lo[1]), hi[2] - lo[2]))
    print("[DEBUG] dims", a["dimensions"])
    print("[DEBUG] regions", [(x["id"], x["areaM2"]) for x in a["thermal"]["regions"]])
    for s in SCALES:
        v = a["volumes"][s]
        print("[DEBUG] scale", s, "body", len(v["body"]), "thermal", len(v["thermal"]), "structure", len(v["structure"]), "shapes", {k: len(x) for k, x in a["representations"][s]["geometry"].items()})
