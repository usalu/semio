"""🧑‍🍳️ Shared body steps of the HW-02 hospitality figures: the generic standing adult re-dressed (regions, materials, layers, tones),
scaled to the person, with de Leva masses, DuBois / Mifflin physiology and per-region skin and metabolic shares.

Each figure script (`waiter.py`, `barista.py`, `receptionist.py`) calls `dress(...)` with its own garment table and then adds its own
garments, carried objects, poses, clips and texts. See ../../📋️brief.md.
"""
import copy, math
import lib
from lib import T, SCALES, VIEWS

CLO = 0.155


def layer(lid, en, de, material, thickness, clo, cap, ref="iso-9920-2007", kind="clothing"):
    """🧵️ One clothing layer: insulation `clo` (ISO 9920, 1 clo = 0.155 m²K/W) and vapour resistance R/(0.0165 K/Pa · 0.38)."""
    r = round(clo * CLO, 5)
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": thickness, "resistanceM2KW": r,
            "vapourResistanceM2PaW": round(r / (0.0165 * 0.38), 2), "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": ref}


def dress(a, garments, tones, layer_map, keep_hatch=()):
    """👔️ Re-binds the base regions to the figure's garments.

    garments: {new_region: (old_region, bone_predicate)}; layer_map: {(new_region, old_layer): new_layer};
    tones: [(predicate(shape_or_solid), tone)] applied in order; hatches other than keep_hatch are dropped from the shapes and the representations.
    """
    for new, (old, pred) in garments.items():
        lib.set_region(a, old, new, pred)
    for sc in SCALES:
        vol = a["volumes"][sc]
        for s in vol["thermal"]:
            key = (s["region"], s.get("layer"))
            if key in layer_map:
                s["layer"] = layer_map[key]
    for pred, tone in tones:
        lib.retone(a, pred, tone)
    for sc in SCALES:
        rep = a["representations"][sc]
        rep["hatches"] = [h for h in rep["hatches"] if h["id"] in keep_hatch]
        for view in VIEWS:
            for s in rep["geometry"][view]:
                if s.get("hatch") and s["hatch"] not in keep_hatch:
                    s.pop("hatch")


def drop_thermal_solids(a, ids):
    for sc in SCALES:
        vol = a["volumes"][sc]
        vol["thermal"] = [s for s in vol["thermal"] if s["id"] not in ids]


def physiology(a, sex, stature, body_kg, age, ref_body="mifflin-1990"):
    """🫀️ thermal.body for the person: DuBois skin area, Mifflin-St Jeor basal metabolism, Gagge two-node defaults."""
    b = a["thermal"]["body"]
    b["massKg"] = body_kg
    b["skinAreaM2"] = round(lib.dubois(body_kg, stature), 3)
    b["basalMetabolismW"] = round(lib.mifflin_w(sex, body_kg, stature, age), 1)
    b["source"] = "authored-estimate"
    b["reference"] = ref_body
    return b["skinAreaM2"]


def region(rid, en, de, kind, model, material, area, length, geometry, sky, layers, cap, segment=None, share=None, skin=None, ref="iso-9920-2007", **extra):
    r = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": area, "characteristicLengthM": length,
         "geometry": geometry, "skyViewFactor": sky, "layers": layers, "heatCapacityJM2K": cap}
    if segment:
        r["body"] = {"segment": segment, "metabolicShare": share, "skinFraction": skin}
    r.update(extra)
    r["source"] = "authored-estimate"
    r["reference"] = ref
    return r


def finger_pose(side, flex=1.0, spread=1.0, thumb=None):
    """🖐️ Finger offsets of one hand from the base's relaxed hand, flexions scaled by `flex` and spreads by `spread` (mirrored for the right hand)."""
    sg = 1 if side == "l" else -1
    out = {}
    for f, ab in (("index", 7), ("middle", 8), ("ring", 9), ("little", 10)):
        out[f"{f}-proximal-{side}"] = {"angleDeg": 0, "abductionDeg": round(-sg * ab * spread, 1), "twistDeg": 90 * sg}
        out[f"{f}-middle-{side}"] = {"angleDeg": round({"index": 15, "middle": 17, "ring": 19, "little": 21}[f] * flex, 1)}
        out[f"{f}-distal-{side}"] = {"angleDeg": round({"index": 10, "middle": 11, "ring": 12, "little": 12}[f] * flex, 1)}
    t = thumb or (0, 9.8, -1, 16, -1.6, 6)
    out[f"thumb-metacarpal-{side}"] = {"angleDeg": t[0], "abductionDeg": round(-sg * t[1], 1), "twistDeg": 90 * sg}
    out[f"thumb-proximal-{side}"] = {"angleDeg": t[2], "abductionDeg": round(-sg * t[3], 1)}
    out[f"thumb-distal-{side}"] = {"angleDeg": t[4], "abductionDeg": round(-sg * t[5], 1)}
    return out


def calibrate_areas(doc, regions, scale="50", factor=1.0):
    """📏️ Sets `areaM2` of the given regions to the outer area that the thermal solids of `scale` show to the air (the measured quantity of
    `volumes-thermal-coverage`), so the areas are those of the modelled surfaces; returns {region: (old, new)}."""
    path = lib.write_tmp(doc, "t-areas.json")
    out = lib.bun("measure.ts", path, "volumes-thermal-coverage", "outerAreaM2").stdout
    got = {}
    for line in out.splitlines():
        parts = line.split()
        names = [p for p in parts if p.startswith("info:outerAreaM2:")]
        if names and f"1:{scale}" in parts:
            got[names[0].split(":", 2)[2]] = float(parts[parts.index(names[0]) + 1])
    res = {}
    for r in doc["asset"]["thermal"]["regions"]:
        if r["id"] in regions and r["id"] in got:
            res[r["id"]] = (r["areaM2"], round(got[r["id"]] * factor, 4))
            r["areaM2"] = round(got[r["id"]] * factor, 4)
    return res


def drop_hem_and_hood_collar(a):
    """✂️ Without the sweatshirt the rib hem band (drawn on the trunk bone, modelled on the pelvis) and the raised hood collar at the nape
    go: the garment ends at the trunk outline and the back of the neck follows the trunk loft (1:50 and 1:100 elevation outlines)."""
    lib.remove_shapes(a, lambda sc, view, s: s["id"] == "hem-rib")
    lib.remove_solids(a, lambda sc, s: s["id"] in ("hem-rib", "th-hem-rib"))
    for sc in ("50", "100"):
        for s in lib.shapes(a, sc, "elevation"):
            if s["id"] == "hoodie":
                pts = s["points"]
                i = next(n for n, p in enumerate(pts) if p[0] > 0.32 and p[1] > 0.05)
                j = next(n for n, p in enumerate(pts) if n > i and p[0] < 0.285 and p[1] > 0.05)
                s["points"] = pts[:i] + [[0.302, 0.04], [0.293, 0.058], [0.278, 0.07]] + pts[j + 1:]


TORSO_REST = {"o": (-0.030, 1.168), "u": (0.032, 0.999), "v": (-0.999, 0.032)}


def _se(e, t):
    c, s = math.cos(t), math.sin(t)
    return math.copysign(abs(c) ** (2 / e), c), math.copysign(abs(s) ** (2 / e), s)


def torso_from_loft(a, scales=("200", "500"), shape_ids=("hoodie", "sil-body")):
    """🫁️ Redraws the trunk outline of the side elevation, the front elevation and the plan of the given scales from the trunk loft of the
    same scale (base coordinates, before scaling): side = section centre ± half depth, front = ± half width, plan = the envelope of the
    sections seen from above in the rest frame of the trunk, so the coarse drawings and the coarse body are the same trunk without the hood."""
    for sc in scales:
        loft = lib.solid(a, sc, "body", "hoodie")
        secs = [(x["at"][0], x["at"][1], x["rx"], x["ry"], x.get("exponent", 2)) for x in loft["sections"]]
        secs = [s for s in secs if s[2] > 0.5 * max(t[2] for t in secs)] if sc != "500" else secs
        elev = [[lib.r5(u), lib.r5(c - ry)] for u, c, rx, ry, e in secs] + [[lib.r5(u), lib.r5(c + ry)] for u, c, rx, ry, e in reversed(secs)]
        front = [[lib.r5(u), lib.r5(-rx)] for u, c, rx, ry, e in secs] + [[lib.r5(u), lib.r5(rx)] for u, c, rx, ry, e in reversed(secs)]
        (ox, oy), (ux, uy), (vx, vy) = TORSO_REST["o"], TORSO_REST["u"], TORSO_REST["v"]
        cx = sum(ox + ux * u + vx * c for u, c, *_ in secs) / len(secs)
        plan = []
        for k in range(48):
            th = 2 * math.pi * k / 48
            dx, dz = math.cos(th), math.sin(th)
            best = 0.0
            for u, c, rx, ry, e in secs:
                sx = ox + ux * u + vx * c
                lo, hi = 0.0, 0.6
                inside = lambda t: abs((cx + dx * t - sx) / ry) ** e + abs((dz * t) / rx) ** e <= 1
                if not inside(0.0):
                    continue
                for _ in range(40):
                    mid = (lo + hi) / 2
                    lo, hi = (mid, hi) if inside(mid) else (lo, mid)
                best = max(best, lo)
            plan.append([lib.r5(cx + dx * best), lib.r5(dz * best)])
        for view, pts in (("elevation", elev), ("front", front), ("plan", plan)):
            for s in lib.shapes(a, sc, view):
                if s["id"] in shape_ids and s["bone"] == "torso":
                    s["points"] = pts
