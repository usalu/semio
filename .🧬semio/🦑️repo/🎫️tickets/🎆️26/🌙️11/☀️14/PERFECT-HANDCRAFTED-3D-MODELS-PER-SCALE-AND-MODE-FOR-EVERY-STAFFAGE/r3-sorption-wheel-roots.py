"""🌱 Round 3, machinery/sorption-wheel: the root corrections outside `volumes` (materials, masses, anchor, thermal regions, pose bindings, texts) and the 2D drawings of 1:50, 1:100 and 1:200 drawn by hand as the true views of each scale's body; writes the candidate document `🗑️generated/r3-sorption-wheel/🔣️.json` (catalog asset + roots + the volumes source).

Usage: python3 r3-sorption-wheel-roots.py
"""
import copy
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def load(name, file):
    """📦 Import a sibling script by file name."""
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


G = load("sw_geo", "r3-sorption-wheel-geo.py")
X = load("sw_texts", "r3-sorption-wheel-texts.py")
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/sorption-wheel/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "machinery", "sorption-wheel.json")
OUT = os.path.join(HERE, "🗑️generated", "r3-sorption-wheel", "🔣️.json")


def shape(id, bone, role, z, tone, region, material, geom, filled=True, **kw):
    """✏️ One 2D shape with the members in the catalog's order."""
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    if isinstance(region, list):
        out["regions"] = region
    else:
        out["region"] = region
    out["material"] = material
    out.update(kw)
    return out


def poly(points, closed=True):
    """📐 Poly geometry."""
    return {"kind": "poly", "points": [G.pt(x, y) for x, y in points], "closed": closed}


def rect(x0, y0, x1, y1):
    """▭ Rect geometry between two corners."""
    return {"kind": "rect", "x": G.r6(x0), "y": G.r6(y0), "w": G.r6(x1 - x0), "h": G.r6(y1 - y0)}


def ellipse(cx, cy, r):
    """⚪ Circle geometry."""
    return {"kind": "ellipse", "cx": cx, "cy": cy, "rx": r, "ry": r}


def path(rings):
    """〰️ Path geometry of closed rings."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{G.r6(x):g} {G.r6(y):g}" for x, y in ring) + " Z" for ring in rings)}


def honeycomb():
    """🐝 The honeycomb symbol of the matrix face in the rotor frame: pointed hexagons of 0.02 m side between the hub and the flange edge."""
    side, rings = 0.02, []
    dx, dy = side * math.sqrt(3), 1.5 * side
    for j in range(-10, 11):
        for i in range(-9, 10):
            cx, cy = i * dx + (dx / 2 if j % 2 else 0.0), j * dy
            ring = [[cx + side * math.cos(math.radians(30 + 60 * k)), cy + side * math.sin(math.radians(30 + 60 * k))] for k in range(6)]
            if all(G.R_HUB + 0.008 < math.hypot(x, y) < G.R_FLANGE - 0.006 for x, y in ring):
                rings.append(ring)
    return path(rings)


def extruded(z0, z1):
    """🧊 The solid hint of a part prismatic across the depth."""
    return {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}


def motor_box(s):
    """⚙️ x and y range of the gearmotor outline in the elevation."""
    y = G.motor_y(s)
    return G.MOTOR_X - G.MOTOR_HALF, y - G.MOTOR_HALF, G.MOTOR_X + G.MOTOR_HALF, y + G.MOTOR_HALF


def elevation(s):
    """🖼️ Side elevation, seen from -z at the wheel face (bone-local coordinates; casing local = world, seal and rotor about C)."""
    plate = G.SCALE[s]["plate"]
    steel, gel = G.STEEL, G.GEL
    out = [
        shape("foot-left", "casing", "body", 3.1, 0.3, "casing", steel, poly(G.foot_profile(-1, plate))),
        shape("foot-right", "casing", "body", 3.1, 0.3, "casing", steel, poly(G.foot_profile(1, plate, G.PLATE_X1))),
        shape("belt-guard", "casing", "body", 3.15, 0.4, "casing", steel, poly(G.guard_profile(s))),
        shape("disc", "rotor", "body", 3.16, 0.6, "disc", gel, ellipse(0, 0, G.R_ROTOR)),
    ]
    if s == 50:
        out.append(shape("honeycomb", "rotor", "detail", 3.17, 0.15, "disc", gel, honeycomb(), filled=False))
    if s != 200:
        for k in range(G.SPOKES):
            a = 360.0 / G.SPOKES * k
            out.append(shape(f"spoke-{k + 1}", "rotor", "detail", 3.17, 0.2, "disc", gel, poly([G.polar(G.R_HUB, a), G.polar(G.R_ROTOR - G.T_SHELL, a)], closed=False), filled=False))
    out.append(shape("regen-sector", "seal", "body", 3.18, 0.9, "regen", gel, path([G.sector(G.R_HUB, G.R_ROTOR, -G.SECTOR_DEG, G.SECTOR_DEG, 7.5)])))
    if s != 200:
        out.append(shape("hub", "rotor", "body", 3.19, 0.4, "hub", steel, ellipse(0, 0, G.R_HUB)))
    out.append(shape("casing", "casing", "body", 3.2, 0.35, "casing", steel, path([G.arc(G.R_BAND, 0.0, 355.0, 5.0, 0.0, G.CY), G.arc(G.R_FLANGE, 355.0, 0.0, 5.0, 0.0, G.CY)])))
    if s != 200:
        r = G.R_BAND - 0.0005
        left = math.degrees(math.acos(G.FOOT_X1 / r))
        strokes = [G.arc(r, G.GUARD_A0, G.GUARD_A1, 5.0, 0.0, G.CY), G.arc(r, -180.0 + left, -90.0 - math.degrees(math.asin(G.FOOT_X0 / r)), 5.0, 0.0, G.CY)]
        out.append(shape("casing-edge", "casing", "detail", 3.22, 0.15, "casing", steel, {"kind": "path", "d": " ".join("M " + " L ".join(f"{x:g} {y:g}" for x, y in ring) for ring in strokes)}, filled=False))
    out.append(shape("sector-frame", "seal", "body", 3.75, 0.3, "casing", steel, poly(G.v_profile(G.R_FLANGE))))
    out.append(shape("beam", "casing", "body", 3.8, 0.3, ["casing", "hub"] if s == 200 else "casing", steel, rect(-G.BEAM_W / 2, G.CY - G.R_FLANGE, G.BEAM_W / 2, G.CY + G.R_FLANGE)))
    x0, y0, x1, y1 = motor_box(s)
    if s == 50:
        out.append(shape("terminal-box", "casing", "body", 3.45, 0.25, "motor", G.PAINT, rect(x1, G.motor_y(s) - 0.015, 0.305, G.motor_y(s) + 0.015)))
    out.append(shape("motor", "casing", "body", 3.5, 0.2, "motor", G.PAINT, rect(x0, y0, x1, y1)))
    if s == 50:
        out.append(shape("motor-end-cap", "casing", "detail", 3.55, 0.1, "motor", G.PAINT, ellipse(G.MOTOR_X, G.motor_y(s), 0.031), filled=False))
    return out


def front(s):
    """🖼️ Front elevation, seen from ahead (+x): paper x is the lateral position (left of the asset, +z, to the right), y the height."""
    plate = G.SCALE[s]["plate"]
    steel = G.STEEL
    x0, y0, x1, y1 = motor_box(s)
    out = [shape("casing", "casing", "body", 3.2, 0.35, "casing", steel, ellipse(0, G.CY, G.R_BAND), solid=extruded(-G.HALF_DEPTH, G.HALF_DEPTH))]
    if s == 50:
        out.append(shape("foot-plate-right", "casing", "body", 3.25, 0.3, "casing", steel, rect(-G.FOOT_HALF_Z, 0.0, G.FOOT_HALF_Z, plate)))
        out.append(shape("gusset-right-near", "casing", "body", 3.25, 0.3, "casing", steel, rect(-0.09, plate, -0.082, G.ring_y(G.FOOT_X1, G.R_BAND - 0.0005))))
        out.append(shape("gusset-right-far", "casing", "body", 3.25, 0.3, "casing", steel, rect(0.082, plate, 0.09, G.ring_y(G.FOOT_X1, G.R_BAND - 0.0005))))
    else:
        out.append(shape("foot-right", "casing", "body", 3.25, 0.3, "casing", steel, rect(-G.FOOT_HALF_Z, 0.0, G.FOOT_HALF_Z, G.ring_y(G.FOOT_X1, G.R_BAND - 0.0005))))
    top = G.guard_profile(s)[0][1]
    out.append(shape("belt-guard", "casing", "body", 3.4, 0.4, "casing", steel, rect(G.GUARD_Z0, plate, G.GUARD_Z1, top)))
    if s != 200:
        foot_top = plate if s == 50 else G.ring_y(G.FOOT_X1, G.R_BAND - 0.0005)
        edges = [[[G.GUARD_Z0, plate], [G.GUARD_Z0, top], [G.GUARD_Z1, top], [G.GUARD_Z1, plate]], [[-G.FOOT_HALF_Z, 0.0], [-G.FOOT_HALF_Z, foot_top], [G.FOOT_HALF_Z, foot_top], [G.FOOT_HALF_Z, 0.0]]]
        if s == 50:
            gy = G.ring_y(G.FOOT_X1, G.R_BAND - 0.0005)
            edges += [[[-0.09, plate], [-0.09, gy], [-0.082, gy], [-0.082, plate]], [[0.082, plate], [0.082, gy], [0.09, gy], [0.09, plate]]]
        out.append(shape("part-edges", "casing", "detail", 3.42, 0.15, "casing", steel, {"kind": "path", "d": " ".join("M " + " L ".join(f"{G.r6(x):g} {G.r6(y):g}" for x, y in ring) for ring in edges)}, filled=False))
    if s == 50:
        out.append(shape("motor-end-cap", "casing", "body", 3.45, 0.2, "motor", G.PAINT, poly([[-0.079, y0 + 0.004], [-0.086, y0 + 0.004], [-0.09, y0 + 0.011], [-0.09, y1 - 0.011], [-0.086, y1 - 0.004], [-0.079, y1 - 0.004]])))
        out.append(shape("motor", "casing", "body", 3.5, 0.2, "motor", G.PAINT, rect(-0.079, y0, G.GUARD_Z0, y1)))
        out.append(shape("gearhead-joint", "casing", "detail", 3.55, 0.1, "motor", G.PAINT, poly([[-0.024, y0], [-0.024, y1]], closed=False), filled=False))
        out.append(shape("terminal-box", "casing", "body", 3.6, 0.25, "motor", G.PAINT, rect(-0.07, G.motor_y(s) - 0.015, -0.03, G.motor_y(s) + 0.015)))
    else:
        out.append(shape("motor", "casing", "body", 3.5, 0.2, "motor", G.PAINT, rect(-0.09, y0, G.GUARD_Z0, y1)))
    return out


def plan(s):
    """🖼️ Plan, seen from above: the band covers everything below it."""
    return [shape("casing", "casing", "body", 3.2, 0.35, "casing", G.STEEL, ellipse(0, G.CY, G.R_BAND), solid=extruded(-G.HALF_DEPTH, G.HALF_DEPTH))]


def replace_once(text, old, new):
    """🔁 Replace exactly one occurrence."""
    if text.count(old) != 1:
        raise SystemExit("text to replace not found exactly once: " + old[:60])
    return text.replace(old, new)


def apply_roots(asset):
    """🌱 All corrections outside `volumes`."""
    asset["summary"] = X.SUMMARY
    prov = asset["provenance"]
    have = {r["id"] for r in prov["references"]}
    prov["references"] += [r for r in X.REFERENCES if r["id"] not in have]
    for lang in ("en", "de"):
        for old, new in X.NOTES_REPLACE[lang]:
            if new not in prov["notes"][lang]:
                prov["notes"][lang] = replace_once(prov["notes"][lang], old, new)
    asset["dimensions"]["massKg"] = X.TOTAL_KG
    ids = {m["id"] for m in asset["materials"]}
    asset["materials"] += [m for m in X.MATERIALS if m["id"] not in ids]
    for bone in asset["rig"]["bones"]:
        if bone["id"] == "casing":
            bone["title"] = X.CASING_BONE_TITLE
            bone["mass"].update({"kg": X.CASING_KG, "comX": X.CASING_COM[0], "comY": X.CASING_COM[1]})
        if bone["id"] == "seal":
            bone["title"] = X.SEAL_BONE_TITLE
    for anchor in asset["anchors"]:
        if anchor["id"] == "base-contact":
            anchor["title"] = X.BASE_ANCHOR_TITLE
            anchor["footprintLengthM"] = X.FOOTPRINT_LENGTH_M
            anchor["footprintWidthM"] = X.FOOTPRINT_WIDTH_M
    thermal = asset["thermal"]
    for region in thermal["regions"]:
        if region["id"] == "casing":
            region["title"] = X.CASING_TITLE
            region["layers"][0]["title"] = X.CASING_TITLE
            region["areaM2"] = X.CASING_AREA_M2
        if region["id"] == "hub":
            region["title"] = X.HUB_TITLE
            region["layers"][0]["title"] = X.HUB_LAYER_TITLE
            region.pop("ratedPowerW", None)
            region.pop("ratedPowerBasis", None)
    if all(region["id"] != "motor" for region in thermal["regions"]):
        thermal["regions"].append(copy.deepcopy(X.MOTOR_REGION))
    else:
        thermal["regions"] = [copy.deepcopy(X.MOTOR_REGION) if region["id"] == "motor" else region for region in thermal["regions"]]
    if X.THERMAL_ASSUMPTION not in thermal["assumptions"]:
        thermal["assumptions"].append(X.THERMAL_ASSUMPTION)
    for pose in asset["poses"]:
        pose["thermal"]["regions"]["motor"] = {"sunlitFraction": 0, "exposedFraction": 0.75}
    asset["structure"]["assumptions"] = [X.STRUCTURE_ASSUMPTION]
    for s in (50, 100, 200):
        rep = asset["representations"][str(s)]
        rep["geometry"] = {"elevation": elevation(s), "front": front(s), "plan": plan(s)}
        rep["decisions"] = X.DECISIONS_2D[s]
    asset["representations"]["500"].update(X.UNAVAILABLE_500)
    return asset


def main():
    """💾 Write the candidate document."""
    document = json.load(open(CATALOG, encoding="utf-8"))
    apply_roots(document["asset"])
    document["asset"]["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    print("candidate", OUT)


if __name__ == "__main__":
    main()
