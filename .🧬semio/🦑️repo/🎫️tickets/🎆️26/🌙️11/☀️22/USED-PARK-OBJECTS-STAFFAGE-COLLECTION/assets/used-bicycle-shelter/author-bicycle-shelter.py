"""🚲️ Writes the asset document of the used bicycle shelter (`🔣️.json` next to this file) from the hand-chosen numbers of `author_parts.py`.

Run `python3 author-bicycle-shelter.py`. Drawings, solids, thermal and structural lists, masses and areas come from the same parts, so the
three views and the four 3D models of every scale agree by construction; traces of use below the 3D feature are drawn as 2D strokes.
"""
import json
import math
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from author_geometry import C5, S5, annulus, area_of, band, depth, hull, project, r6, t, view_geometry
from author_parts import BIKE_O, FEATURE, L_B, PANEL_T, PANEL_U1, PURLIN_U, STAND_ORIGIN_X, ZBAR, ZF, ZP, ZS, build_rig, parts_of
from author_text import ANCHOR_TEXT, MATERIALS, POSE_TEXT, REFERENCES, REGIONS, REP_DECISIONS, SUMMARY, TITLE, VOLUME_DECISIONS, CLIP_TEXT, STRUCTURE_ASSUMPTIONS, THERMAL_ASSUMPTIONS, SYMBOL_TEXT

SCALES = (50, 100, 200, 500)
PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": 0.1, "glazing": None, "ground": 0.25}, 500: {"body": 0.14, "detail": None, "texture": 0.1, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
MASS_R = {50: 0.018, 100: 0.02, 200: 0.03, 500: 0.0375}
HATCHES = {50: [{"id": "flutes", "title": t("Multiwall flutes along the slope", "Stege der Stegplatte in Gefällerichtung"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}], 100: [{"id": "flutes", "title": t("Multiwall flutes along the slope", "Stege der Stegplatte in Gefällerichtung"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.7, "penMm": 0.1}], 200: [], 500: []}


def shape(sid, bone, role, z, filled, tone, geo, region=None, material=None, **extra):
    """✏️ One authored 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": round(z, 4), "filled": filled, "tone": tone}
    s.update(geo)
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    s.update(extra)
    return s


def poly(points, closed=True):
    """🔷️ A poly geometry."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in points], "closed": closed}


def strokes(segments):
    """〰️ Several open strokes as one path."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in seg) for seg in segments)}


def role_of(part, s=50):
    """🖋️ Line role of a part's drawing: moss and litter are drawn as detail patches where the scale has a detail pen."""
    if part.material == "moss-litter" and s < 500:
        return "detail"
    return "texture" if part.flags.get("texture") else part.role


def drawings(rig, parts, lines, r, s):
    """🗺️ Elevation, front and plan shapes of scale 1:s: parts, stand bands, ground lines and the traces of use."""
    views = {"elevation": [], "front": [], "plan": []}
    groups = {}
    for p in parts:
        for v in ("elevation", "front", "plan"):
            if v in p.group:
                groups.setdefault((v, p.group[v]), []).append(p)
                continue
            if v not in p.views:
                continue
            extra = {"hatch": p.extra["hatch"]} if v == "plan" and "hatch" in p.extra else {}
            views[v].append(shape(p.id, p.bone, role_of(p, s), 5 + depth(rig, p, v) + (0.001 if role_of(p, s) != "body" else 0), True, p.tone, view_geometry(rig, p, v), p.region, p.material, **extra))
    holes = {}
    for sh in views["plan"]:
        if sh["id"].startswith("moss-"):
            holes.setdefault(sh["bone"], []).append(sh["points"])
    for sh in views["plan"]:
        if sh["id"].startswith("panel-") and sh["bone"] in holes:
            rings = [sh.pop("points")] + holes[sh["bone"]]
            sh.pop("closed")
            sh["kind"] = "path"
            sh["d"] = " ".join("M " + " L ".join(f"{a} {b}" for a, b in ring) + " Z" for ring in rings)
    for (v, gid), ps in groups.items():
        pts = []
        for p in ps:
            pts += project(rig, p.bone, p.samples(), v)
        views[v].append(shape(gid, ps[0].bone, "body", 5 + max(depth(rig, p, v) for p in ps), True, ps[0].tone, poly(hull(pts)), ps[0].region, ps[0].material))
    for b, cl in lines.items():
        views["elevation"].append(shape(f"{b}-u", b, "body", 5 + 0 - ZS[b.split("-")[1]] + 0.0, True, 0.55, poly(band(cl, r)), "bike-stands", "galv-worn"))
    views["elevation"].append(shape("ground-line", "post-b", "ground", 0.1, False, 0.5, poly([rig.local("post-b", -1.6, 0)[:2], rig.local("post-b", 1.6, 0)[:2]], False)))
    views["front"].append(shape("ground-line", "post-b", "ground", 0.1, False, 0.5, poly([(-2.9, 0), (2.9, 0)], False)))
    views["elevation"].append(shape("moss-edge", "panel-1", "texture", 9.0, False, 0.3, poly([(-0.02, PANEL_T + 0.003), (0.40 if s <= 100 else 0.42, PANEL_T + 0.003)], False), "moss"))
    if s == 500:
        views["elevation"].append(shape("bar-line", "beam-b", "texture", 9.0, False, 0.6, poly([(-0.03, 0.14), (L_B + 0.03, 0.14)], False), "glazing-bars"))
    for v, items in traces(rig, s, r).items():
        views[v] += items
    return views


def spokes(cx, n, r0, r1):
    """🛞️ Radial spoke strokes of a wheel at the rear (cx 0) or front (cx 1.10) axle in the bike bone frame."""
    return [[(cx + r0 * math.cos(2 * math.pi * k / n + 0.2), r0 * math.sin(2 * math.pi * k / n + 0.2)), (cx + r1 * math.cos(2 * math.pi * k / n + 0.2), r1 * math.sin(2 * math.pi * k / n + 0.2))] for k in range(n)]


def traces(rig, s, r):
    """🩹️ Traces of use below the 3D feature, drawn as detail and texture strokes (graffiti, sticker residue, rust, wear, moss, the depot tag)."""
    e, f, p = [], [], []
    tz = 9.0
    if s == 50:
        e.append(shape("graffiti-a", "post-a", "texture", tz, False, 0.2, poly([(1.05, -0.035), (1.12, 0.03), (1.18, -0.03), (1.26, 0.035), (1.33, -0.025), (1.40, 0.03), (1.46, -0.01)], False), "steel-frame"))
        e.append(shape("sticker-a", "post-a", "detail", tz, False, 0.3, poly([(1.58, -0.035), (1.66, -0.035), (1.66, 0.025), (1.58, 0.025)], True), "steel-frame"))
        e.append(shape("rust-creep-a", "post-a", "texture", tz, False, 0.3, strokes([[(0.016, -0.045), (0.07, -0.045)], [(0.016, 0.04), (0.055, 0.04)]]), "steel-frame"))
        e.append(shape("wear-top-1", "stand-1", "detail", tz, False, 0.8, poly([(0.80 - 0.004, -0.20), (0.80 - 0.004, -0.50)], False), "bike-stands"))
        e.append(shape("depot-tag", "bike", "detail", tz, False, 0.3, poly([(0.80, 0.56), (0.86, 0.56), (0.86, 0.63), (0.80, 0.63)], True), "bike-frame"))
        e.append(shape("flat-tyre", "bike", "detail", tz, False, 0.2, poly([(-0.10, -0.338), (0.0, -0.343), (0.10, -0.338)], False), "bike-rubber"))
        e.append(shape("spokes-rear", "bike", "texture", tz, False, 0.5, strokes(spokes(0.0, 16, 0.035, 0.288)), "bike-frame"))
        e.append(shape("spokes-front", "bike", "texture", tz, False, 0.5, strokes(spokes(1.10, 16, 0.03, 0.288)), "bike-frame"))
        e.append(shape("litter-edge", "gutter", "texture", tz, False, 0.3, poly([(-0.055, 0.005), (0.055, 0.005)], False), "moss"))
        f.append(shape("sticker-b", "post-b", "detail", tz, False, 0.3, poly([(-0.035, 1.52), (0.03, 1.52), (0.03, 1.60), (-0.035, 1.60)], True), "steel-frame"))
        f.append(shape("graffiti-b", "post-b", "texture", tz, False, 0.2, poly([(-0.04, 1.00), (0.035, 1.08), (-0.03, 1.14), (0.04, 1.22), (-0.025, 1.30), (0.02, 1.35)], False), "steel-frame"))
        for fr, z in ZF.items():
            f.append(shape(f"rust-plate-{fr}", f"post-{fr}", "texture", tz, False, 0.3, strokes([[(z - 0.12, 0.004), (z - 0.05, 0.004)], [(z + 0.06, 0.009), (z + 0.12, 0.009)]]), "base-fixings"))
        for k, z in enumerate((-2.2, -1.6, -0.9, -0.3, 1.0, 1.9, 2.3)):
            pn = "1" if z < -1.25 else "2" if z < 0 else "3" if z < 1.25 else "4"
            if pn == "3":
                continue
            y0 = rig.world(f"panel-{pn}", PANEL_U1, 0)[1]
            f.append(shape(f"algae-{k + 1}", f"panel-{pn}", "texture", tz, False, 0.3, poly([(z, y0 - 0.002), (z + 0.01, y0 - 0.06)], False), "roof-old"))
        for i, z in enumerate(ZBAR):
            for j, u in enumerate(PURLIN_U):
                x = rig.world("beam-b", u, 0.151)[0]
                p.append(shape(f"screw-{i + 1}{j + 1}", "beam-b", "detail", tz, True, 0.25, {"kind": "ellipse", "cx": r6(x), "cy": r6(z), "rx": 0.008, "ry": 0.008}, "glazing-bars"))
    elif s == 100:
        e.append(shape("graffiti-a", "post-a", "texture", tz, False, 0.2, poly([(1.05, -0.03), (1.18, 0.03), (1.30, -0.03), (1.45, 0.02)], False), "steel-frame"))
        e.append(shape("flat-tyre", "bike", "detail", tz, False, 0.2, poly([(-0.10, -0.336), (0.10, -0.336)], False), "bike-rubber"))
        e.append(shape("spokes-rear", "bike", "texture", tz, False, 0.5, strokes(spokes(0.0, 8, 0.035, 0.29)), "bike-frame"))
        e.append(shape("spokes-front", "bike", "texture", tz, False, 0.5, strokes(spokes(1.10, 8, 0.03, 0.29)), "bike-frame"))
        f.append(shape("graffiti-b", "post-b", "texture", tz, False, 0.2, poly([(-0.04, 1.00), (0.035, 1.12), (-0.03, 1.24), (0.02, 1.35)], False), "steel-frame"))
    elif s == 200:
        e.append(shape("spokes-rear", "bike", "texture", tz, False, 0.5, strokes(spokes(0.0, 4, 0.02, 0.29)), "bike-frame"))
        e.append(shape("spokes-front", "bike", "texture", tz, False, 0.5, strokes(spokes(1.10, 4, 0.02, 0.29)), "bike-frame"))
    else:
        e.append(shape("spokes-rear", "bike", "texture", tz, False, 0.5, strokes(spokes(0.0, 4, 0.0, 0.276)), "bike-frame"))
        e.append(shape("spokes-front", "bike", "texture", tz, False, 0.5, strokes(spokes(1.10, 4, 0.0, 0.276)), "bike-frame"))
    return {"elevation": e, "front": f, "plan": p}


def volume(rig, parts, s):
    """🧊️ The authored volume of scale 1:s: body solids, thermal references, structure references with joints and mass markers."""
    body = [p.solid() for p in parts]
    thermal = [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in parts]
    structure = [{"id": f"{'m' if p.struct == 'member' else 'j'}-{p.id}", "of": p.id, "member": p.bone, "part": p.struct} for p in parts if p.struct]
    jr = 0.033 if s == 50 else max(0.04, FEATURE[s] / 2 + 0.001)
    for f in ZF:
        structure.append({"id": f"j-head-{f}", "kind": "ellipsoid", "bone": f"post-{f}", "material": "galv-weathered", "role": "detail", "tone": 0.3, "radii": [jr, jr, jr], "transform": {"translation": [2.30, 0, 0]}, "member": f"beam-{f}", "part": "joint"})
        for end, u in (("foot", 0.012 if s == 50 else 0.0), ("head", rig.bones[f"brace-{f}"]["L"])):
            structure.append({"id": f"j-brace-{end}-{f}", "kind": "ellipsoid", "bone": f"brace-{f}", "material": "galv-weathered", "role": "detail", "tone": 0.3, "radii": [jr, jr, jr], "transform": {"translation": [r6(u), 0, 0]}, "member": f"brace-{f}", "part": "joint"})
    for bid in rig.order:
        b = rig.bones[bid]
        if b["kg"] > 0:
            mr = MASS_R[s] if not (s == 50 and bid.startswith("panel")) else 0.008
            at = MARKER_AT.get(bid, (b["mu"] / b["kg"], b["mv"] / b["kg"]))
            structure.append({"id": f"mass-{bid}", "kind": "ellipsoid", "bone": bid, "material": "galv-weathered", "role": "detail", "tone": 0.3, "radii": [mr, mr, mr], "transform": {"translation": [r6(at[0]), r6(at[1]), 0]}, "member": bid, "part": "mass"})
    return {"status": "authored", "denominator": s, "body": body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[s]}


def masses(rig, parts):
    """⚖️ Bone masses and centres from the 1:50 parts (section x density, catalogue-typical bike parts)."""
    for p in parts:
        if p.kg <= 0:
            continue
        if "repeat" in p.geo:
            cu, cv = p.geo["repeat"]["origin"][0], p.geo["repeat"]["origin"][1]
        else:
            sm = p.samples()
            cu, cv = sum(a for a, _, _ in sm) / len(sm), sum(b for _, b, _ in sm) / len(sm)
        b = rig.bones[p.bone]
        b["kg"] += p.kg
        b["mu"] += p.kg * cu
        b["mv"] += p.kg * cv


def extent(rig, parts):
    """📦️ World bounding box of the default pose (rest) of the 1:50 body."""
    pts = []
    for p in parts:
        pts += [rig.world(p.bone, *q) for q in p.samples()]
    lo = [min(q[i] for q in pts) for i in range(3)]
    hi = [max(q[i] for q in pts) for i in range(3)]
    return lo, hi


def region_areas(parts):
    """📐️ First estimate of each region's outer area from the 1:50 parts (sum of part areas, overlaps not removed)."""
    out = {}
    for p in parts:
        out[p.region] = out.get(p.region, 0.0) + area_of(p)
    return out


def anchors(rig):
    """📍️ Contacts at the base plates, stand legs, tyres and kickstand; lifting points of the replacement sheet; load points."""
    out = []
    for f in ZF:
        out.append({"id": f"base-{f}", "bone": f"post-{f}", "x": 0, "y": 0, "kind": "contact", "title": ANCHOR_TEXT["base"](f), "support": "fixed", "normalDeg": 90, "footprintLengthM": 0.25, "footprintWidthM": 0.25, "frictionCoefficient": 0.6})
    for k in ZS:
        for leg, v in (("rear", 0.0), ("front", -(0.425 - 0.02415 - STAND_ORIGIN_X))):
            out.append({"id": f"stand-{k}-{leg}", "bone": f"stand-{k}", "x": 0, "y": r6(v), "kind": "contact", "title": ANCHOR_TEXT["leg"](k, leg), "support": "fixed", "normalDeg": 90, "footprintLengthM": 0.0483, "footprintWidthM": 0.0483, "frictionCoefficient": 0.6})
    out.append({"id": "bike-rear-tyre", "bone": "bike", "x": 0, "y": -0.351, "kind": "contact", "title": ANCHOR_TEXT["tyre-rear"], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.06, "footprintWidthM": 0.035, "frictionCoefficient": 0.7})
    out.append({"id": "bike-front-tyre", "bone": "bike", "x": 1.10, "y": -0.351, "kind": "contact", "title": ANCHOR_TEXT["tyre-front"], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.05, "footprintWidthM": 0.035, "frictionCoefficient": 0.7})
    out.append({"id": "bike-kickstand", "bone": "bike", "x": 0.29, "y": -0.351, "z": 0.18, "kind": "contact", "title": ANCHOR_TEXT["kickstand"], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.02, "frictionCoefficient": 0.5})
    for side, z in (("l", 0.5), ("r", -0.5)):
        out.append({"id": f"lift-{side}", "bone": "panel-3", "x": r6(L_B), "y": PANEL_T, "z": z, "kind": "load", "title": ANCHOR_TEXT["lift"](side)})
    for k in ("1", "3"):
        out.append({"id": f"lean-{k}", "bone": f"stand-{k}", "x": 0.80, "y": r6(-(0.05 - STAND_ORIGIN_X)), "kind": "load", "title": ANCHOR_TEXT["lean"](k)})
    out.append({"id": "lean-post-b", "bone": "post-b", "x": 1.20, "y": -0.05, "kind": "load", "title": ANCHOR_TEXT["lean-post"]})
    return out


CONTACTS = ["base-a", "base-b", "base-c"] + [f"stand-{k}-{leg}" for k in ZS for leg in ("rear", "front")] + ["bike-rear-tyre", "bike-front-tyre", "bike-kickstand"]
SUN = {"in-service": {"steel-frame": (1.0, 0.35), "base-fixings": (0.75, 0.3), "roof-old": (1.0, 1.0), "roof-new": (1.0, 1.0), "moss": (1.0, 0.6), "glazing-bars": (1.0, 1.0), "rainwater-goods": (1.0, 0.5), "bike-stands": (0.98, 0.25), "bike-frame": (1.0, 0.2), "bike-rubber": (0.97, 0.2)}}
SUN["maintenance"] = dict(SUN["in-service"], **{"roof-new": (1.0, 1.0), "bike-stands": (0.98, 0.35)})
SUN["in-use"] = dict(SUN["in-service"], **{"bike-stands": (0.97, 0.2), "bike-frame": (1.0, 0.15)})
SUN["winter-snow"] = {k: (v[0], 0.0) for k, v in SUN["in-service"].items()}
SUN["winter-snow"].update({"roof-old": (0.1, 0.0), "roof-new": (0.1, 0.0), "moss": (0.1, 0.0), "glazing-bars": (0.3, 0.0)})


def binding(pose):
    """🌬️ Thermal binding of a pose: exposed and sunlit shares per region, contact of plates and tyres."""
    out = {}
    for reg, (ex, sun) in SUN[pose].items():
        b = {"exposedFraction": ex, "sunlitFraction": sun}
        if reg == "base-fixings":
            b["contact"] = {"fraction": 0.25, "tempC": 31 if pose != "winter-snow" else 1}
        if reg == "bike-rubber":
            b["contact"] = {"fraction": 0.03, "tempC": 34 if pose != "winter-snow" else 1}
        if reg == "bike-stands":
            b["contact"] = {"fraction": 0.02, "tempC": 30 if pose != "winter-snow" else 1}
        if pose == "winter-snow" and reg in ("roof-old", "roof-new", "moss", "glazing-bars"):
            b["contact"] = {"fraction": 0.9 if reg != "glazing-bars" else 0.7, "tempC": 0}
        out[reg] = b
    return {"regions": out, "emitters": {}}


HOLD_N = 17.7


def poses():
    """🧍️ In service (default), maintenance (replacement sheet slid out), in use (two visiting bikes, a sheltering visitor) and winter snow."""
    snow = []
    for f, trib in (("a", 1.3), ("b", 2.4), ("c", 1.3)):
        snow.append({"id": f"snow-{f}", "kind": "snow", "title": POSE_TEXT["snow-load"](f, trib), "distributed": {"bone": f"beam-{f}", "fromM": 0, "toM": r6(L_B), "wxNPerM": 0, "wyNPerM": r6(-680 * trib * C5)}, "fxN": 0, "fyN": 0, "source": "literature", "reference": "din-en-1991-1-3"})
    use = [
        {"id": "lean-bike-1", "kind": "occupant", "title": POSE_TEXT["lean-bike-1"], "anchor": "lean-1", "fxN": 0, "fyN": -25, "fzN": -40, "source": "authored-estimate", "reference": "din-79008-1"},
        {"id": "lean-bike-3", "kind": "occupant", "title": POSE_TEXT["lean-bike-3"], "anchor": "lean-3", "fxN": 0, "fyN": -25, "fzN": 40, "source": "authored-estimate", "reference": "din-79008-1"},
        {"id": "lock-pull", "kind": "occupant", "title": POSE_TEXT["lock-pull"], "anchor": "lean-3", "fxN": 120, "fyN": 0, "fzN": 0, "source": "authored-estimate", "reference": "din-79008-1"},
        {"id": "lean-visitor", "kind": "occupant", "title": POSE_TEXT["lean-visitor"], "anchor": "lean-post-b", "fxN": -150, "fyN": -60, "source": "authored-estimate", "reference": "din-en-1991-1-1"},
    ]
    hold = [{"id": f"hold-sheet-{side}", "kind": "equipment", "title": POSE_TEXT["hold-sheet"](side), "anchor": f"lift-{side}", "fxN": 0, "fyN": HOLD_N, "source": "authored-estimate"} for side in ("l", "r")]
    out = []
    for pid, bones, contacts, loads, key in (("in-service", {}, CONTACTS, [], None), ("maintenance", {"panel-3": {"dx": 1.0}}, CONTACTS, hold, None), ("in-use", {}, CONTACTS, use, None), ("winter-snow", {}, CONTACTS, snow, True)):
        p = {"id": pid, "title": POSE_TEXT[pid][0], "description": POSE_TEXT[pid][1]}
        if key:
            p["key"] = True
        p.update({"bones": bones, "ground": "authored", "contacts": contacts, "thermal": binding(pid), "loads": loads})
        out.append(p)
    return out


def clips():
    """🎞️ Sliding the replacement sheet out (maintenance) and an afternoon of parking under the roof (life in use)."""
    service = [{"anchor": a, "fromS": 0, "toS": 30} for a in CONTACTS]
    ct = CLIP_TEXT
    m = {"id": "sheet-slide-out", "title": ct["m"][0], "description": ct["m"][1], "durationS": 30, "fps": 10, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "linear"}, {"t": 6, "pose": "in-service", "ease": "smooth"}, {"t": 26, "pose": "maintenance", "ease": "linear"}, {"t": 30, "pose": "maintenance", "ease": "linear"}],
         "contacts": service,
         "phases": [{"id": "unclamping", "fromS": 0, "toS": 4, "title": ct["ph-unclamp"]}, {"id": "taking-hold", "fromS": 4, "toS": 6, "title": ct["ph-hold"]}, {"id": "sliding", "fromS": 6, "toS": 26, "title": ct["ph-slide"]}, {"id": "held-out", "fromS": 26, "toS": 30, "title": ct["ph-held"]}],
         "events": [{"t": 6, "kind": "load-on", "title": ct["ev-grip"], "target": "hold-sheet-l"}, {"t": 6, "kind": "motion", "title": ct["ev-start"]}, {"t": 26, "kind": "motion", "title": ct["ev-stop"]}],
         "tracks": []}
    use = [{"anchor": a, "fromS": 0, "toS": 20} for a in CONTACTS]
    u = {"id": "parking-afternoon", "title": ct["u"][0], "description": ct["u"][1], "durationS": 20, "fps": 25, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-use", "ease": "linear"}, {"t": 20, "pose": "in-use", "ease": "linear"}],
         "contacts": use,
         "phases": [{"id": "arrival-1", "fromS": 0, "toS": 7, "title": ct["ph-a1"]}, {"id": "arrival-3", "fromS": 7, "toS": 14, "title": ct["ph-a3"]}, {"id": "departure", "fromS": 14, "toS": 20, "title": ct["ph-dep"]}],
         "events": [{"t": 2, "kind": "load-on", "title": ct["ev-b1-on"], "target": "lean-bike-1"}, {"t": 8, "kind": "load-on", "title": ct["ev-b3-on"], "target": "lean-bike-3"}, {"t": 9, "kind": "load-on", "title": ct["ev-lock"], "target": "lock-pull"}, {"t": 11, "kind": "load-off", "title": ct["ev-lock-off"], "target": "lock-pull"}, {"t": 12, "kind": "load-on", "title": ct["ev-visitor"], "target": "lean-visitor"}, {"t": 17, "kind": "load-off", "title": ct["ev-visitor-off"], "target": "lean-visitor"}, {"t": 18, "kind": "load-off", "title": ct["ev-b1-off"], "target": "lean-bike-1"}],
         "tracks": [{"target": "load:lean-bike-1", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 2, "v": 0}, {"t": 2.6, "v": 1}, {"t": 17.5, "v": 1}, {"t": 18.2, "v": 0}, {"t": 20, "v": 0}]},
                    {"target": "load:lean-bike-3", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 8, "v": 0}, {"t": 8.6, "v": 1}, {"t": 20, "v": 1}]},
                    {"target": "load:lock-pull", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 9, "v": 0}, {"t": 9.3, "v": 1}, {"t": 10.7, "v": 1}, {"t": 11, "v": 0}, {"t": 20, "v": 0}]},
                    {"target": "load:lean-visitor", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 12, "v": 0}, {"t": 12.8, "v": 1}, {"t": 16.4, "v": 1}, {"t": 17, "v": 0}, {"t": 20, "v": 0}]}]}
    return [m, u]


def symbol(rig, lo, hi):
    """🔣️ The true-extent symbol for site plans at 1:200 and 1:500."""
    y0, y1 = rig.world("panel-1", -0.03, 0)[1], rig.world("panel-1", PANEL_U1, PANEL_T)[1]
    roof_e = [(lo[0], y0 - 0.12), (hi[0], y1 - 0.12), (hi[0], hi[1]), (lo[0], y0 + 0.02)]
    g = {"elevation": [shape("roof", "ground", "body", 3, True, 0.7, poly(roof_e)), shape("post", "ground", "body", 2, True, 0.55, poly([(-0.85, 0), (-0.75, 0), (-0.75, 2.30), (-0.85, 2.30)])), shape("stand", "ground", "body", 2, True, 0.55, poly([(-0.325, 0), (0.425, 0), (0.425, 0.80), (-0.325, 0.80)]))],
         "front": [shape("roof", "ground", "body", 3, True, 0.7, poly([(lo[2], y0 - 0.12), (hi[2], y0 - 0.12), (hi[2], hi[1]), (lo[2], hi[1])]))] + [shape(f"post-{f}", "ground", "body", 2, True, 0.55, poly([(z - 0.05, 0), (z + 0.05, 0), (z + 0.05, y0), (z - 0.05, y0)])) for f, z in ZF.items()],
         "plan": [shape("roof", "ground", "body", 3, True, 0.7, poly([(lo[0], lo[2]), (hi[0], lo[2]), (hi[0], hi[2]), (lo[0], hi[2])]))]}
    return [{"id": "used-bicycle-shelter-true-extent", "title": SYMBOL_TEXT["title"], "description": SYMBOL_TEXT["description"], "semantics": "true-extent", "units": "model-m", "denominators": [500, 200], "pens": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [], "geometry": g, "dimensionalNote": SYMBOL_TEXT["note"]}]


def thermal(areas, heat):
    """🌡️ Thermal model: ten passive regions with weathered material data; the shelter draws no power."""
    regs = []
    for rid, meta in REGIONS.items():
        r = {"id": rid, "title": meta["title"], "kind": meta["kind"], "model": "passive", "material": meta["material"], "areaM2": round(areas[rid], 3), "characteristicLengthM": meta["L"], "geometry": meta["geometry"], "skyViewFactor": meta["sky"], "layers": [], "heatCapacityJM2K": round(heat[rid] / areas[rid]), "source": "authored-estimate", "reference": meta["ref"]}
        if rid == "steel-frame":
            r.update({"ratedPowerW": 0, "ratedPowerBasis": "passive"})
        regs.append(r)
    env = {"airTempC": 28, "meanRadiantTempC": 30, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325, "skyTempC": 14, "groundTempC": 38}
    return {"environment": env, "referenceEnvironment": "summer-hot-sun", "operating": [], "passengers": [], "regions": regs, "emitters": [], "assumptions": THERMAL_ASSUMPTIONS}


AREA_OVERRIDE = {}
MARKER_AT = {**{f"stand-{k}": (0.40, -0.35085) for k in ZS}, "bike": (0.369, 0.114)}
SPECIFIC_HEAT = {"galv-weathered": 460, "steel-rusted": 460, "pc-yellowed": 1200, "pc-clear": 1200, "moss-litter": 2500, "alu-weathered": 900, "galv-sheet": 460, "galv-worn": 460, "bike-steel": 460, "rubber-weathered": 1600}


def main():
    """🚀️ Builds and writes the document."""
    rig = build_rig()
    per = {s: parts_of(rig, s) for s in SCALES}
    p50 = per[50][0]
    masses(rig, p50)
    lo, hi = extent(rig, p50)
    areas = region_areas(p50)
    areas.update(AREA_OVERRIDE)
    heat = {}
    for p in p50:
        heat[p.region] = heat.get(p.region, 0.0) + p.kg * SPECIFIC_HEAT[p.material]
    total = sum(b["kg"] for b in rig.bones.values())
    reps, vols = {}, {}
    for s in SCALES:
        parts, lines, r = per[s]
        lo_, hi_ = RANGES[s]
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": lo_, "maxDenominator": hi_}, "pens": PENS[s], "hatches": HATCHES[s], "geometry": drawings(rig, parts, lines, r, s), "decisions": REP_DECISIONS[s]}
        vols[str(s)] = volume(rig, parts, s)
    asset = {
        "id": "used-bicycle-shelter", "category": "objects", "title": TITLE, "summary": SUMMARY,
        "tags": ["used", "park", "public-space", "cycling", "shelter", "bicycle-parking", "sheffield-stand", "galvanised-steel", "polycarbonate", "street-furniture", "roof"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES},
        "dimensions": {"heightM": r6(hi[1] - max(lo[1], 0.0)), "widthM": r6(hi[0] - lo[0]), "depthM": r6(hi[2] - lo[2]), "massKg": round(total, 2)},
        "materials": MATERIALS,
        "rig": {"kind": "structural", "root": "ground", "bones": rig.json("shelter-catalogue-typical")},
        "anchors": anchors(rig),
        "representations": reps,
        "symbols": symbol(rig, lo, hi),
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal(areas, heat),
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": STRUCTURE_ASSUMPTIONS, "clinical": False},
        "volumes": vols,
        "presentation": {"defaultPose": "in-service", "defaultClip": "parking-afternoon", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print("[DEBUG] extent", [round(x, 4) for x in lo], [round(x, 4) for x in hi], "mass", round(total, 2))
    print("[DEBUG] bone masses", {k: round(b["kg"], 2) for k, b in rig.bones.items() if b["kg"]})
    print("[DEBUG] areas", {k: round(v, 3) for k, v in areas.items()})
    for s in SCALES:
        v = vols[str(s)]
        print("[DEBUG] scale", s, "body", len(v["body"]), "thermal", len(v["thermal"]), "structure", len(v["structure"]))


if __name__ == "__main__":
    main()


if __name__ == "__main__":
    # the silhouette repair of batch SO, the last step: ops/used-bicycle-shelter.json of the umbrella ticket of 2026-11-22
    import sys as _sys
    from pathlib import Path as _Path
    _sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️silhouette/SO")
    from lib import repair_file as _repair_file
    _repair_file(_Path(__file__).resolve().parent / "🔣️.json", "used-bicycle-shelter")
