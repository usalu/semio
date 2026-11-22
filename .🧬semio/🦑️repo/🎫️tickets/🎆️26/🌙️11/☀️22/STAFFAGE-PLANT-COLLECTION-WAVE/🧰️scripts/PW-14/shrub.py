"""🌳 PW-14 shrub assembly: the scale-independent physical scenario (poses, clips, thermal bindings, loads, structure) and the asset document of a botanical shrub whose numbers come from a species script.

The species script states the rig, the primitives of every scale, regions, loads and texts; this module only formats them into `semio.staffage/v2`.
@see plant_lib.py
"""
import math
from plant_lib import T, r4, wind_force, limits_of, sway_keys, thermal_list, emitter_solid, volume, manifest_row, write_asset, write_row, feature, EXPORTS, representation, pens, ring

RHO = 1.25


def wind_at(v10, height):
    """💨 Power-law profile v(z) = v10 (z / 10)^0.22 of the archive's vegetation (EN 1991-1-4 terrain II order of magnitude)."""
    return v10 * (max(height, 0.1) / 10) ** 0.22


def drag(cd, area, v):
    return round(0.5 * RHO * cd * area * v * v, 3)


def region_bindings(regions, sunlit, wet=None):
    out = {}
    for rid in regions:
        b = {"exposedFraction": 1, "sunlitFraction": r4(sunlit[rid])}
        if wet and rid in wet:
            b["wettedness"] = wet[rid]
        out[rid] = b
    return out


def load(id, kind, title, bone, x, y, fx, fy, src, ref):
    return {"id": id, "kind": kind, "title": title, "bone": bone, "x": r4(x), "y": r4(y), "fxN": round(fx, 3), "fyN": round(fy, 3), "source": src, "reference": ref}


def pose(id, title, desc, bones, regions, emitters, loads, key=False):
    p = {"id": id, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "authored", "contacts": [], "thermal": {"regions": regions, "emitters": emitters}, "loads": loads})
    return p


def gust_clip(windy_bones, wind_loads, peak_scale_bones, title, desc):
    """🌬️ An 8 s gust over the fresh-breeze pose: load tracks 0.05 → 2.75 → 0.05 (EN 1991-1-4 peak velocity pressure with Iv 0.25), bones leaning in proportion."""
    times = [0, 1.2, 2.5, 3.6, 5, 8]
    factors = [0.05, 0.45, 2.75, 1.4, 0.5, 0.05]
    lean = [0.1, 0.4, 1.0, 0.7, 0.35, 0.1]
    keys = []
    for t, k in zip(times, lean):
        keys.append({"t": t, "pose": "windy", "ease": "smooth", "bones": {b: {"angleDeg": r4(a * k)} for b, a in peak_scale_bones.items()}})
    tracks = [{"target": f"load:{l['id']}", "ease": "smooth", "keys": [{"t": t, "v": f} for t, f in zip(times, factors)]} for l in wind_loads]
    return {"id": "gust", "title": T("Gust", "Windböe"), "description": desc, "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [],
            "phases": [{"id": "rise", "fromS": 0, "toS": 2.5, "title": T("Gust front", "Böenfront")}, {"id": "peak", "fromS": 2.5, "toS": 3.6, "title": T("Peak", "Spitze")}, {"id": "decay", "fromS": 3.6, "toS": 8, "title": T("Decay and recovery", "Abklingen und Erholung")}],
            "events": [{"t": 2.5, "kind": "load-on", "title": T("Peak gust load", "Böenspitze")}, {"t": 5, "kind": "load-off", "title": T("Load below the mean again", "Last wieder unter dem Mittelwert")}], "tracks": tracks}


def sway_clip(rig, duration, n, base, specs, desc, emitter_ids):
    keys = sway_keys("windy", duration, n, base, specs, limits_of(rig))
    tracks = [{"target": f"emitter:{e}", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": r4(duration / 2), "v": 0.92}, {"t": duration, "v": 1}]} for e in emitter_ids]
    return {"id": "wind-sway", "title": T("Wind sway", "Windschwingung"), "description": desc, "durationS": duration, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [],
            "phases": [{"id": "sway", "fromS": 0, "toS": duration, "title": T("Sway cycle", "Schwingungszyklus")}], "events": [{"t": 0, "kind": "motion", "title": T("Cycle start at mean lean", "Zyklusbeginn in mittlerer Neigung")}], "tracks": tracks}


DAY_T = [0, 5, 7, 9, 12, 15, 18, 20, 24]
DAY_SUN = [0, 0, 0.25, 0.7, 1, 0.8, 0.25, 0, 0]
DAY_TRANSPIRATION = [0.03, 0.05, 0.3, 0.7, 1, 0.9, 0.35, 0.08, 0.03]


def diurnal_clip(regions_noon, emitter_ids, desc, pose_id="calm"):
    tracks = [{"target": f"emitter:{e}", "ease": "smooth", "keys": [{"t": t, "v": v} for t, v in zip(DAY_T, DAY_TRANSPIRATION)]} for e in emitter_ids]
    tracks += [{"target": f"region:{rid}:sunlitFraction", "ease": "smooth", "keys": [{"t": t, "v": r4(v * noon)} for t, v in zip(DAY_T, DAY_SUN)]} for rid, noon in regions_noon.items()]
    return {"id": "diurnal-transpiration", "title": T("Day of transpiration", "Transpirationstag"), "description": desc, "durationS": 24, "fps": 10, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": pose_id, "ease": "smooth"}, {"t": 12, "pose": pose_id, "ease": "smooth"}, {"t": 24, "pose": pose_id, "ease": "smooth"}], "contacts": [],
            "phases": [{"id": "night", "fromS": 0, "toS": 6, "title": T("Night", "Nacht")}, {"id": "morning", "fromS": 6, "toS": 12, "title": T("Morning to noon", "Morgen bis Mittag")}, {"id": "afternoon", "fromS": 12, "toS": 18, "title": T("Afternoon", "Nachmittag")}, {"id": "evening", "fromS": 18, "toS": 24, "title": T("Evening and night", "Abend und Nacht")}],
            "events": [{"t": 6, "kind": "thermal", "title": T("Sunrise", "Sonnenaufgang")}, {"t": 12, "kind": "thermal", "title": T("Solar noon, peak transpiration", "Sonnenhöchststand, maximale Transpiration")}, {"t": 18, "kind": "thermal", "title": T("Sunset", "Sonnenuntergang")}], "tracks": tracks}


THERMAL_ASSUMPTIONS = [
    T("Leaf regions use the one-sided leaf (or bract and calyx) area as exchange area; the leaf area index is that area over the ground projection of the shrub. Maximum stomatal conductances are authored estimates for the species type guided by Jones (2014). Photosynthesis type C3. Aperture and night closure are the engine's business, not authored here.",
      "Blattregionen verwenden die einseitige Blatt- (oder Deck- und Kelchblatt-)Fläche als Austauschfläche; der Blattflächenindex ist diese Fläche bezogen auf die Bodenprojektion des Strauchs. Die maximalen Spaltöffnungsleitfähigkeiten sind Autorenschätzungen für den Artentyp, angelehnt an Jones (2014). Photosynthesetyp C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
    T("Bark and stem regions are passive. Their areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of a quarter of the stem diameter, an authored estimate guided by the Wood Handbook. Their area is the outer surface of the woody solids of the 1:50 model. No surface temperature is authored.",
      "Rinden- und Stängelregionen sind passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von einem Viertel des Stammdurchmessers, eine Autorenschätzung angelehnt an das Wood Handbook. Ihre Fläche ist die Außenfläche der Holzkörper des Modells 1:50. Es werden keine Oberflächentemperaturen vorgegeben."),
    T("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors across the shrub; the direction is the vertical in the frame of the first anchor's bone, since the release is diffuse.",
      "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf mindestens fünf Anker über den Strauch; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist."),
]


def structure_model(text):
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["base-support"], "assumptions": [T(*text)], "clinical": False}


def up_direction(rig, bone):
    """⬆️ The world vertical as `directionDeg` in the frame of a bone (emitters measure their outflow there)."""
    return r4(((90 - rig.theta(bone)) + 180) % 360 - 180)


def assemble(spec):
    """🧾 The asset document of a shrub from its species spec: dimensions from the 1:50 body, per-scale volumes (body, thermal references plus emitter markers plus extra thermal solids, structure) and representations, the manifest row."""
    r, scales = spec["rig"], spec["scales"]
    lo, hi = scales[50].extent()
    dims = {"heightM": r4(hi[1] - max(0, lo[1])), "widthM": r4(hi[0] - lo[0]), "depthM": r4(hi[2] - lo[2])}
    for d, sc in scales.items():
        a, b = sc.extent()
        print(f"[DEBUG] 1:{d} extent x {a[0]:.3f}..{b[0]:.3f} y {a[1]:.3f}..{b[1]:.3f} z {a[2]:.3f}..{b[2]:.3f}")
    anchor_by = {a["id"]: a for a in spec["anchors"]}
    reps, vols = {}, {}
    for d, sc in scales.items():
        f = feature(d)
        em = [emitter_solid(r, f"th-{a}", anchor_by[a], region, emitter, max(0.6 * f, spec["emitter_radius"][d]), material) for a, region, emitter, material in spec["emitter_anchors"]]
        thermal = thermal_list(sc, em) + spec.get("extra_thermal", {}).get(d, [])
        vols[str(d)] = volume(d, sc.body(), thermal, spec["structure"](d), spec["volume_notes"][d])
        if d in spec["drawn"]:
            reps[str(d)] = representation(d, *spec["ranges"][d], spec["pens"][d], spec.get("hatches", {}).get(d, []), sc.geometry(), spec["decisions"][d])
    reps.update(spec.get("unavailable", {}))
    asset = {"id": spec["id"], "category": "vegetation", "title": spec["title"], "summary": spec["summary"](dims), "tags": spec["tags"],
             "provenance": {"author": "Semio staffage plant worker PW-14", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in spec["refs"]], "notes": spec["notes"]},
             "dimensions": {**dims, "massKg": round(sum(b["mass"]["kg"] for b in r.bones), 3)}, "materials": spec["materials"], "rig": {"kind": "botanical", "root": r.bones[0]["id"], "bones": r.bones},
             "anchors": spec["anchors"], "poses": spec["poses"], "clips": spec["clips"],
             "thermal": {"environment": spec["environment"], "regions": spec["regions"], "emitters": spec["emitters"], "assumptions": THERMAL_ASSUMPTIONS, "operating": [], "passengers": []},
             "structure": structure_model(spec["structure_text"]), "volumes": vols, "presentation": {"defaultPose": spec["poses"][0]["id"], "defaultClip": "wind-sway", "defaultScale": 50},
             "exports": EXPORTS, "completion": "authored", "representations": {k: reps[k] for k in ("50", "100", "200", "500")}, "symbols": spec["symbols"](dims)}
    path = write_asset(asset)
    write_row(manifest_row(asset, spec["distinction"]))
    print(path, dims, asset["dimensions"]["massKg"])
    return asset


def outline_symbol(r, root, dims, side, id, title, description, note, heart=0.05, denominators=(500, 200)):
    """🔣 A true-extent symbol: a closed outline in side and front (side = [(x share of the half width, height in m)] scaled to the dimensions) and a lobed plan outline with a circle at the stem base."""
    w, h, dz = dims["widthM"] / 2, dims["heightM"], dims["depthM"] / 2
    top = max(y for _, y in side)
    el = [(x * w, y * h / top) for x, y in side]
    fr = [(x * dz, y * h / top) for x, y in side]
    plan = ring(0, 0, w, dz, 24, jitter=[1.0, 0.95, 0.99, 0.94])

    def shp(sid, view, pts, role="body", z=2):
        loc = [r.local(root, (p[0], p[1], 0))[:2] for p in pts] if view == "elevation" else pts
        return {"id": sid, "bone": root, "role": role, "z": z, "filled": False, "tone": 0.5, "kind": "poly", "points": [[r4(x), r4(y)] for x, y in loc], "closed": True}
    geo = {"elevation": [shp("crown", "elevation", el)], "front": [shp("crown", "front", fr)], "plan": [shp("crown", "plan", plan), shp("heart", "plan", ring(0, 0, heart, heart, 12), role="detail", z=3)]}
    return {"id": id, "title": title, "description": description, "semantics": "true-extent", "units": "model-m", "denominators": list(denominators), "pens": pens(0.18, 0.13, None, None, 0.25), "hatches": [], "geometry": geo, "dimensionalNote": note}
