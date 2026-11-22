"""🧳️ HW-09 generator of `traveller-with-rolling-suitcase`: an adult in a camel overcoat with a shoulder bag walking and pulling a
two-wheeled cabin trolley (IATA cabin size 55 × 40 × 23 cm, 8.5 kg) behind the left hand.

Body: the walking commuter (rig with fingers, coat, scarf, shoulder bag, heel-to-toe walk keys), scaled by 1.02 to a 1.82 m, 76 kg adult,
the coat re-toned and its cloth re-specified as a light camel wool. Suitcase: authored here per scale (shell, wheel housings, two wheels,
feet, telescopic handle with two tubes and a grip), hung from the pelvis through case-lift / case-slide prismatic bones, tilting about
its axle (bone `suitcase`) with the wheels on their own bone (`case-wheels`). Poses and clips: poses_traveller.ts.
Run: python3 build_traveller.py
"""
import copy
import math
from hw09lib import load, write, subtree, take, scale_part, merge_refs, uniq, T_, SCALES, VIEWS, LISTS

SLUG = "traveller-with-rolling-suitcase"
K = 1.02
MASS = 76.0
ZC = 0.2254
AXLE = (-0.62, 0.0363)
R = 0.03896

com = load("👤️humans", "walking-commuter")
person = take(com, subtree(com, ["pelvis"]))
pmass = sum(b["mass"]["kg"] for b in person["bones"] if "mass" in b)
scale_part(person, K, (MASS - 0.0) / pmass)
for s in SCALES:
    for v in VIEWS:
        for x in person["shapes"][s][v]:
            if x.get("region") == "coat" and "tone" in x:
                x["tone"] = round(min(0.85, x["tone"] + 0.3), 3)
for m in person["materials"]:
    if m["id"] == "wool-coat":
        m["title"] = T_("Camel wool overcoat cloth", "Kamelfarbener Mantelstoff aus Wolle")
        m["solarAbsorptivity"] = 0.55
pelvis = next(b for b in person["bones"] if b["id"] == "pelvis")
PY = pelvis["y"]


def rrect(u0, u1, v0, v1, r, n=4):
    """⬜️ A rounded rectangle in (u, v) with n segments per corner."""
    pts = []
    for cu, cv, a0 in ((u1 - r, v0 + r, -90), (u1 - r, v1 - r, 0), (u0 + r, v1 - r, 90), (u0 + r, v0 + r, 180)):
        for i in range(n + 1):
            a = math.radians(a0 + 90 * i / n)
            pts.append([round(cu + r * math.cos(a), 5), round(cv + r * math.sin(a), 5)])
    return pts


def rect(u0, u1, v0, v1):
    return [[u0, v0], [u1, v0], [u1, v1], [u0, v1]]


def ext(z0, z1):
    return {"kind": "extruded", "sweep": "w", "z0": round(ZC + z0, 5), "z1": round(ZC + z1, 5)}


SH, HW = "suitcase-shell", "handle-and-wheels"


def poly(id_, bone, pts, region, material, tone, z, w, role="body", filled=True, closed=True, solid=True, **kw):
    out = {"kind": "poly", "id": id_, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "points": pts, "closed": closed, "region": region, "material": material, **kw}
    if solid:
        out["solid"] = ext(*w)
    return out


def ell(id_, bone, r, region, material, tone, z, w, role="body"):
    return {"kind": "ellipse", "id": id_, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, "cx": 0, "cy": 0, "rx": r, "ry": r, "region": region, "material": material, "solid": ext(*w)}


WIDTH = 0.195
DESIGN = {
    "50": dict(corner=0.035, tube=0.008, grip=0.016, wheel_w=0.012, feet=True, housings=True, two_wheels=True, two_tubes=True, hub=True, zip=True),
    "100": dict(corner=0.03, tube=0.008, grip=0.016, wheel_w=0.012, feet=True, housings=True, two_wheels=True, two_tubes=True, hub=False, zip=False),
    "200": dict(corner=0.02, tube=0.008, grip=0.016, wheel_w=0.015, feet=False, housings=False, two_wheels=True, two_tubes=True, hub=False, zip=False),
    "500": dict(corner=0.0, tube=0.0375, grip=0.0, wheel_w=0.19, feet=False, housings=False, two_wheels=False, two_tubes=False, hub=False, zip=False),
}


def suitcase_shapes(s):
    d = DESIGN[s]
    shell = rrect(0.0, 0.55, 0.012, 0.242, d["corner"]) if d["corner"] else rect(0.0, 0.55, 0.012, 0.242)
    if not d["housings"]:
        shell = rect(-0.005, 0.55, 0.0, 0.242) if s == "500" else rrect(-0.005, 0.55, 0.0, 0.242, d["corner"])
    views = {}
    for v in VIEWS:
        out = [poly("case-shell", "suitcase", shell, SH, "polycarbonate", 0.62, 60, (-WIDTH, WIDTH))]
        if d["zip"] and v == "elevation":
            out.append({"kind": "poly", "id": "case-zip", "bone": "suitcase", "role": "detail", "z": 61, "filled": False, "tone": 0.4, "points": [[0.02, 0.127], [0.53, 0.127]], "closed": False, "region": SH, "material": "polycarbonate"})
        if d["housings"]:
            for side, sg in (("l", 1), ("r", -1)):
                out.append(poly(f"case-housing-{side}", "suitcase", rrect(-0.012, 0.06, 0.0, 0.065, 0.012), HW, "nylon-wheel", 0.3, 62, (sg * 0.148, sg * WIDTH) if sg > 0 else (-WIDTH, -0.148)))
        if d["feet"]:
            for side, sg in (("l", 1), ("r", -1)):
                out.append(poly(f"case-foot-{side}", "suitcase", rect(-R, 0.002, 0.19, 0.225), HW, "nylon-wheel", 0.25, 63, (0.15, 0.18) if sg > 0 else (-0.18, -0.15)))
        if d["two_tubes"]:
            for side, sg in (("l", 1), ("r", -1)):
                out.append(poly(f"handle-tube-{side}", "suitcase", rect(0.55, 0.99, 0.0, 2 * d["tube"]), HW, "aluminium-tube", 0.5, 64, (0.11, 0.126) if sg > 0 else (-0.126, -0.11)))
            out.append(poly("handle-grip", "suitcase", rrect(0.97, 1.006, -0.008, 0.032, 0.008), HW, "nylon-wheel", 0.2, 65, (-0.135, 0.135)))
        else:
            out.append(poly("handle", "suitcase", rect(0.55, 1.006, 0.0, 0.075), HW, "aluminium-tube", 0.4, 64, (-0.0375, 0.0375)))
        if d["two_wheels"]:
            for side, sg in (("l", 1), ("r", -1)):
                c = sg * 0.172
                out.append(ell(f"wheel-{side}", "case-wheels", R, HW, "nylon-wheel", 0.12, 66, (c - d["wheel_w"], c + d["wheel_w"])))
                if d["hub"] and v == "elevation":
                    out.append({"kind": "ellipse", "id": f"hub-{side}", "bone": "case-wheels", "role": "detail", "z": 67, "filled": True, "tone": 0.6, "cx": 0, "cy": 0, "rx": 0.012, "ry": 0.012, "region": HW, "material": "aluminium-tube", "solid": ext(c - d["wheel_w"] - 0.001, c + d["wheel_w"] + 0.001)})
        else:
            out.append(ell("wheels", "case-wheels", R, HW, "nylon-wheel", 0.12, 66, (-d["wheel_w"], d["wheel_w"])))
        views[v] = out
    return views


def revolve_wheel(id_, c, w, r=R):
    return {"id": id_, "kind": "revolve", "bone": "case-wheels", "material": "nylon-wheel", "role": "body", "tone": 0.12, "profile": [[-w, 0], [-w, r * 0.8], [-w * 0.6, r], [w * 0.6, r], [w, r * 0.8], [w, 0]], "axis": [0, 0, 1], "origin": [0, 0, c]}


def suitcase_solids(s):
    d = DESIGN[s]
    body = []
    if s == "500":
        body.append({"id": "case-shell", "kind": "box", "bone": "suitcase", "material": "polycarbonate", "role": "body", "tone": 0.62, "size": [0.555, 0.242, 0.39], "transform": {"translation": [0.2725, 0.121, 0]}})
    else:
        lo = 0.0 if d["housings"] else -0.005
        body.append({"id": "case-shell", "kind": "box", "bone": "suitcase", "material": "polycarbonate", "role": "body", "tone": 0.62, "size": [round(0.55 - lo, 4), 0.23 if d["housings"] else 0.242, 0.39], "cornerRadius": max(d["corner"] * 0.6, 0.008), "transform": {"translation": [round((0.55 + lo) / 2, 4), 0.127 if d["housings"] else 0.121, 0]}})
    if d["housings"]:
        for side, sg in (("l", 1), ("r", -1)):
            body.append({"id": f"case-housing-{side}", "kind": "box", "bone": "suitcase", "material": "nylon-wheel", "role": "detail", "tone": 0.3, "size": [0.072, 0.065, 0.047], "cornerRadius": 0.008, "transform": {"translation": [0.024, 0.0325, sg * 0.1715]}})
    if d["feet"]:
        for side, sg in (("l", 1), ("r", -1)):
            body.append({"id": f"case-foot-{side}", "kind": "box", "bone": "suitcase", "material": "nylon-wheel", "role": "detail", "tone": 0.25, "size": [round(R + 0.004, 4), 0.035, 0.03], "transform": {"translation": [round((-R + 0.002) / 2, 4) + 0.001, 0.2075, sg * 0.165]}})
    if d["two_tubes"]:
        for side, sg in (("l", 1), ("r", -1)):
            body.append({"id": f"handle-tube-{side}", "kind": "capsule", "bone": "suitcase", "material": "aluminium-tube", "role": "body", "tone": 0.5, "p0": [0.5, d["tube"], sg * 0.118], "p1": [0.985, d["tube"], sg * 0.118], "r0": d["tube"], "r1": d["tube"], "slender": True})
        body.append({"id": "handle-grip", "kind": "capsule", "bone": "suitcase", "material": "nylon-wheel", "role": "body", "tone": 0.2, "p0": [0.988, 0.012, -0.12], "p1": [0.988, 0.012, 0.12], "r0": 0.018, "r1": 0.018})
    else:
        body.append({"id": "handle", "kind": "box", "bone": "suitcase", "material": "aluminium-tube", "role": "body", "tone": 0.4, "size": [0.47, 0.075, 0.075], "transform": {"translation": [0.771, 0.0375, 0]}})
    if d["two_wheels"]:
        body += [revolve_wheel("wheel-l", 0.172, d["wheel_w"]), revolve_wheel("wheel-r", -0.172, d["wheel_w"])]
    else:
        body.append(revolve_wheel("wheels", 0, d["wheel_w"]))
    thermal = [{"id": f"th-{x['id']}", "of": x["id"], "region": SH if x["id"] == "case-shell" else HW} for x in body]
    structure = [{"id": f"st-{x['id']}", "of": x["id"], "member": "case-wheels" if x["bone"] == "case-wheels" else "suitcase", "part": "member"} for x in body if x["role"] == "body"]
    structure.append({"id": "case-contents", "kind": "box", "bone": "suitcase", "material": "polycarbonate", "role": "detail", "tone": 0.2, "size": [0.3, 0.12, 0.25] if s in ("50", "100") else [0.3, 0.12, 0.25], "transform": {"translation": [0.28, 0.127, 0]}, "member": "suitcase", "part": "mass"})
    structure.append({"id": "case-axle", "kind": "revolve", "bone": "case-wheels", "material": "aluminium-tube", "role": "detail", "tone": 0.4, "profile": [[0, max(0.006, {"50": 0.006, "100": 0.0076, "200": 0.0152, "500": R}[s])], [0.4, max(0.006, {"50": 0.006, "100": 0.0076, "200": 0.0152, "500": R}[s])]], "axis": [0, 0, 1], "origin": [0, 0, -0.2], "member": "case-wheels", "part": "joint"})
    return {"body": body, "thermal": thermal, "structure": structure}


case_bones = [
    {"id": "case-lift", "parent": "pelvis", "title": T_("Suitcase height travel", "Höhenausgleich des Koffers"), "x": 0, "y": 0, "angleDeg": -90, "lengthM": 0.01, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.3, "maxM": 0.3}},
    {"id": "case-slide", "parent": "case-lift", "title": T_("Suitcase trailing distance", "Nachlaufabstand des Koffers"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.01, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.6, "maxM": 1.3}},
    {"id": "suitcase", "parent": "case-slide", "title": T_("Cabin trolley shell with telescopic handle, tilting about its axle", "Kabinentrolley mit Teleskopgriff, um die Achse kippend"), "x": AXLE[0], "y": round(AXLE[1] - PY, 5), "z": ZC, "angleDeg": 90, "lengthM": 1.006, "joint": {"type": "revolute", "minDeg": -70, "maxDeg": 10}, "mass": {"kg": 8.5, "comX": 0.3, "comY": 0.125, "distribution": "point", "source": "authored-estimate", "reference": "iata-cabin-2023"}},
    {"id": "case-wheels", "parent": "suitcase", "title": T_("Wheel pair on the axle", "Radpaar auf der Achse"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": R, "joint": {"type": "revolute"}, "mass": {"kg": 0.25, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"}},
]
anchors = person["anchors"] + [
    {"id": "grip-l", "bone": "hand-l", "x": round(0.075 * K, 5), "y": -0.004, "kind": "attachment", "title": T_("Left hand on the handle grip", "Linke Hand am Koffergriff")},
    {"id": "case-wheel-patch", "bone": "case-slide", "x": AXLE[0], "y": round(AXLE[1] - PY - R, 5), "z": ZC, "kind": "contact", "title": T_("Suitcase wheel patches", "Aufstandsflächen der Kofferräder"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.01, "footprintWidthM": 0.024, "frictionCoefficient": 0.03},
    {"id": "case-foot", "bone": "suitcase", "x": round(-R, 5), "y": 0.2075, "kind": "contact", "title": T_("Suitcase feet", "Kofferfüße"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.035, "footprintWidthM": 0.06, "frictionCoefficient": 0.5},
    {"id": "case-handle", "bone": "suitcase", "x": 0.988, "y": 0.012, "kind": "attachment", "title": T_("Handle grip", "Koffergriff")},
]
MATS = [
    {"id": "polycarbonate", "title": T_("Polycarbonate shell", "Polycarbonatschale"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 1200, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "elasticModulusPa": 2.4e9, "strengthPa": 6.5e7, "source": "literature", "reference": "incropera-2011"},
    {"id": "aluminium-tube", "title": T_("Anodised aluminium handle tube", "Eloxiertes Aluminiumrohr des Griffs"), "emissivity": 0.8, "solarAbsorptivity": 0.5, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 900, "elasticModulusPa": 7e10, "strengthPa": 2.1e8, "source": "literature", "reference": "incropera-2011"},
    {"id": "nylon-wheel", "title": T_("Nylon and polyurethane wheels, grip and housings", "Räder, Griff und Gehäuse aus Nylon und Polyurethan"), "emissivity": 0.92, "solarAbsorptivity": 0.85, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 1700, "source": "authored-estimate", "reference": "incropera-2011"},
]
REGIONS = [
    {"id": SH, "title": T_("Polycarbonate suitcase shell", "Polycarbonat-Kofferschale"), "kind": "surface", "model": "passive", "material": "polycarbonate", "areaM2": 0.86, "characteristicLengthM": 0.4, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [{"id": "shell", "title": T_("Shell 2.5 mm over packed clothing", "Schale 2,5 mm über gepackter Kleidung"), "kind": "polymer", "material": "polycarbonate", "thicknessM": 0.0025, "resistanceM2KW": 0.0125, "heatCapacityJM2K": 3600, "source": "authored-estimate", "reference": "incropera-2011"}], "heatCapacityJM2K": 9000, "source": "authored-estimate", "reference": "incropera-2011"},
    {"id": HW, "title": T_("Telescopic handle, grip, wheels and housings", "Teleskopgriff, Griff, Räder und Gehäuse"), "kind": "surface", "model": "passive", "material": "aluminium-tube", "areaM2": 0.13, "characteristicLengthM": 0.03, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 2500, "source": "authored-estimate", "reference": "incropera-2011"},
]

bones = person["bones"] + case_bones
asset = {
    "id": SLUG, "category": "humans",
    "title": T_("Traveller with Rolling Suitcase", "Reisende Person mit Rollkoffer"),
    "summary": T_("", ""),
    "tags": ["human", "adult", "traveller", "suitcase", "trolley", "walking", "gait", "wheels"],
    "provenance": {}, "dimensions": {},
    "materials": uniq(person["materials"] + MATS),
    "rig": {"kind": "anatomical", "root": "pelvis", "bones": bones},
    "volumes": {}, "anchors": anchors,
    "representations": {}, "symbols": [], "poses": [], "clips": [], "thermal": {}, "structure": {},
    "presentation": {"defaultPose": "ready-to-walk", "defaultClip": "pull-walk", "defaultScale": 100},
    "exports": com["exports"], "completion": "authored",
}
for s in SCALES:
    rep = copy.deepcopy(com["representations"][s])
    case = suitcase_shapes(s)
    rep["geometry"] = {v: person["shapes"][s][v] + case[v] for v in VIEWS}
    rep["hatches"] = person["hatches"].get(s, [])
    asset["representations"][s] = rep
    sol = suitcase_solids(s)
    asset["volumes"][s] = {"status": "authored", "denominator": int(s), **{l: person["solids"][s][l] + sol[l] for l in LISTS}, "decisions": []}

import traveller_texts
traveller_texts.finish(asset, com, person, REGIONS, K, MASS)
write(SLUG, asset)
print("written", SLUG, len(bones), "bones")
