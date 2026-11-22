"""⛏️ Generator of `construction-worker-with-shovel`: a 1.72 m, 78 kg construction worker (male, 42) in hi-vis vest, hard hat, gloves and safety boots, digging with a 1.21 m round-point shovel with D-grip.

Body: the catalog figure `worker-carrying-box` without its carton, scaled to the stature (lib). Shovel: stated here per scale as bone-local parts (tool.py). Poses: legs, trunk and head by hand, both arms and the shovel joint solved by `ik.ts` onto the D-grip, the shaft and the blade tip on the ground.
Usage: python3 shovel.py [--write]  (without --write only the draft in 🗑️generated/HW-04/draft is written)
"""
import json
import math
import sys
import sys

import lib
import tool

SLUG = "construction-worker-with-shovel"
STATURE = 1.72
MASS = 78.0
AGE = 42
S = STATURE / 1.741
TOOL_KG = 2.4
L_TIP = 1.21
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]


def materials(a):
    a["materials"] += [
        {"id": "ash-wood", "title": lib.tx("Ash wood shaft, oiled", "Eschenholzstiel, geölt"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 690, "conductivityWmK": 0.16, "source": "literature", "reference": "ashrae-2021"},
        {"id": "pp-grip", "title": lib.tx("Polypropylene D-grip", "D-Griff aus Polypropylen"), "emissivity": 0.92, "solarAbsorptivity": 0.8, "densityKgM3": 905, "conductivityWmK": 0.22, "source": "authored-estimate"},
        {"id": "blade-steel", "title": lib.tx("Hardened steel blade, powder-coated", "Gehärtetes Stahlblatt, pulverbeschichtet"), "emissivity": 0.88, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 45, "source": "literature", "reference": "ashrae-2021"},
    ]


def regions(a):
    a["thermal"]["regions"] += [
        {"id": "shovel-handle", "title": lib.tx("Shovel shaft and D-grip", "Schaufelstiel und D-Griff"), "kind": "surface", "model": "passive", "material": "ash-wood", "areaM2": 0.13, "characteristicLengthM": 0.038, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 9000, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "ashrae-2021"},
        {"id": "shovel-blade", "title": lib.tx("Steel blade of the shovel", "Stahlblatt der Schaufel"), "kind": "surface", "model": "passive", "material": "blade-steel", "areaM2": 0.13, "characteristicLengthM": 0.25, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 9400, "source": "authored-estimate", "reference": "ashrae-2021"},
    ]


def parts(scale):
    """🪏️ The shovel of one scale (u along the shaft from the D-grip bar in the right fist, v across the blade, w its thickness)."""
    b = "shovel"
    H, P, BL = "shovel-handle", "pp-grip", "shovel-blade"
    blade = [(1.035, -0.118), (1.035, 0.118), (1.12, 0.117), (1.17, 0.108), (1.215, 0.087), (1.26, 0.052), (1.29, 0.018), (1.30, 0.0), (1.29, -0.018), (1.26, -0.052), (1.215, -0.087), (1.17, -0.108), (1.12, -0.117)]
    blade = [(u - 0.09, v) for u, v in blade]
    t = 0.0055
    below = lambda z: {"elevation": z, "front": z, "plan": round(3.2 + (z - 7.6) / 10, 3)}
    if scale == "50":
        return [
            tool.plate("grip-bar", b, [(-0.017, -0.062), (0.017, -0.062), (0.017, 0.062), (-0.017, 0.062)], -0.016, 0.016, H, P, 0.15, below(7.9)),
            tool.plate("grip-leg-a", b, [(0.005, -0.062), (0.03, -0.062), (0.11, -0.022), (0.095, -0.012)], -0.011, 0.011, H, P, 0.15, below(7.8)),
            tool.plate("grip-leg-b", b, [(0.005, 0.062), (0.03, 0.062), (0.11, 0.022), (0.095, 0.012)], -0.011, 0.011, H, P, 0.15, below(7.8)),
            tool.revolved("grip-socket", b, [(0.09, 0.021), (0.15, 0.021)], H, P, 0.15, below(7.85)),
            tool.revolved("shaft", b, [(0.10, 0.019), (0.88, 0.019)], H, "ash-wood", 0.7, below(7.7), slender=True),
            tool.revolved("blade-socket", b, [(0.86, 0.021), (0.93, 0.024), (0.965, 0.02)], BL, "blade-steel", 0.35, below(7.75)),
            tool.plate("blade", b, blade, -t, t, BL, "blade-steel", 0.35, below(7.6), sheet=True),
        ]
    if scale == "100":
        return [
            tool.plate("grip", b, [(-0.017, -0.062), (0.017, -0.062), (0.017, -0.03), (0.12, -0.021), (0.12, 0.021), (0.017, 0.03), (0.017, 0.062), (-0.017, 0.062)], -0.016, 0.016, H, P, 0.15, below(7.9)),
            tool.revolved("shaft", b, [(0.11, 0.019), (0.88, 0.019)], H, "ash-wood", 0.7, below(7.7), slender=True),
            tool.revolved("blade-socket", b, [(0.86, 0.022), (0.965, 0.022)], BL, "blade-steel", 0.35, below(7.75)),
            tool.plate("blade", b, blade, -t, t, BL, "blade-steel", 0.35, below(7.6), sheet=True),
        ]
    if scale == "200":
        return [
            tool.plate("grip", b, [(-0.018, -0.062), (0.018, -0.062), (0.018, 0.062), (-0.018, 0.062)], -0.017, 0.017, H, P, 0.15, below(7.9)),
            tool.revolved("shaft", b, [(0.0, 0.02), (0.95, 0.02)], H, "ash-wood", 0.7, below(7.7)),
            tool.plate("blade", b, blade, -t, t, BL, "blade-steel", 0.35, below(7.6), sheet=True),
        ]
    return [
        tool.revolved("shaft", b, [(-0.02, 0.038), (0.95, 0.038)], H, "ash-wood", 0.7, below(7.7)),
        tool.plate("blade", b, [(0.945, -0.118), (0.945, 0.118), (1.07, 0.105), (1.16, 0.05), (1.21, 0.0), (1.16, -0.05), (1.07, -0.105)], -t, t, BL, "blade-steel", 0.35, below(7.6), sheet=True),
    ]


def build():
    doc = lib.load_base()
    a = doc["asset"]
    a["id"] = SLUG
    lib.strip_box(a)
    lib.scale_figure(a, S)
    lib.set_masses(a, MASS, "male")
    lib.set_body(a, MASS, STATURE, AGE, "male")
    grip = next(x for x in a["anchors"] if x["id"] == "grip-r")["x"]
    a["rig"]["bones"].append({"id": "shovel", "parent": "hand-r", "title": lib.tx("Round-point shovel with D-grip", "Spitzschaufel mit D-Griff"), "x": grip, "y": 0, "angleDeg": 0, "abductionDeg": 90, "lengthM": 0.015, "joint": {"type": "revolute", "minDeg": -25, "maxDeg": 25}, "mass": {"kg": TOOL_KG, "comX": 0.78, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "din-20127"}})
    a["anchors"] += [
        {"id": "blade-tip", "bone": "shovel", "x": L_TIP, "y": 0, "kind": "contact", "title": lib.tx("Blade tip in the soil", "Blattspitze im Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.08, "frictionCoefficient": 0.7},
        {"id": "shovel-cog", "bone": "shovel", "x": 0.78, "y": 0, "kind": "load", "title": lib.tx("Centre of mass of the shovel", "Schwerpunkt der Schaufel")},
        {"id": "blade-load", "bone": "shovel", "x": 1.07, "y": 0, "kind": "load", "title": lib.tx("Soil on the blade", "Erde auf dem Blatt")},
    ]
    materials(a)
    regions(a)
    rest_z = {"shovel": -0.18 * S}
    for sc in lib.SCALES:
        tool.emit(a, sc, parts(sc), rest_z)
    return doc


ARM_R0 = {"upper-arm-r": {"angleDeg": 27.951, "abductionDeg": -4.745, "twistDeg": -15.662}, "forearm-r": {"angleDeg": 30, "twistDeg": 15.496}, "hand-r": {"angleDeg": -10, "abductionDeg": 7.182}}
ARM_L0 = {"upper-arm-l": {"angleDeg": 40, "abductionDeg": 4.745, "twistDeg": 15.662}, "forearm-l": {"angleDeg": 30, "twistDeg": -15.496}, "hand-l": {"angleDeg": -10, "abductionDeg": -7.182}}
ARM_VARS = lambda side: [["upper-arm-" + side, "angleDeg"], ["upper-arm-" + side, "abductionDeg", -60, 60], ["upper-arm-" + side, "twistDeg", -90, 90], ["forearm-" + side, "angleDeg"], ["forearm-" + side, "twistDeg", -120, 120], ["hand-" + side, "angleDeg"], ["hand-" + side, "abductionDeg", -40, 40]]

# id: (body offsets, contacts, tip world, shaft angle below the horizontal towards the blade, left grip u along the shaft, left grip v, blade lateral sign)
POSES = {
    "driving-blade": ({"pelvis": {"angleDeg": -8}, "torso": {"angleDeg": -16}, "neck": {"angleDeg": 8}, "head": {"angleDeg": 8}, "thigh-l": {"angleDeg": 22}, "shank-l": {"angleDeg": -28}, "foot-l": {"angleDeg": 6}, "thigh-r": {"angleDeg": -10}, "shank-r": {"angleDeg": -12}, "foot-r": {"angleDeg": 22}}, FEET + ["blade-tip"], [None, 0.0, -0.03], 62, 0.42, 0.0),
    "prying-soil": ({"pelvis": {"angleDeg": -15}, "torso": {"angleDeg": -20}, "neck": {"angleDeg": 10}, "head": {"angleDeg": 10}, "thigh-l": {"angleDeg": 35}, "shank-l": {"angleDeg": -50}, "foot-l": {"angleDeg": 15}, "thigh-r": {"angleDeg": 0}, "shank-r": {"angleDeg": -30}, "foot-r": {"angleDeg": 30}}, FEET + ["blade-tip"], [None, 0.0, -0.03], 45, 0.32, 0.0),
    "lifting-soil": ({"pelvis": {"angleDeg": -8}, "torso": {"angleDeg": -10}, "neck": {"angleDeg": 4}, "head": {"angleDeg": 6}, "thigh-l": {"angleDeg": 30}, "shank-l": {"angleDeg": -40}, "foot-l": {"angleDeg": 10}, "thigh-r": {"angleDeg": 4}, "shank-r": {"angleDeg": -24}, "foot-r": {"angleDeg": 20}}, FEET, [None, 1.0, -0.06], -7, 0.62, 0.0),
    "resting-on-shovel": ({"torso": {"angleDeg": 3}, "neck": {"angleDeg": -2}, "head": {"angleDeg": -1}}, FEET + ["blade-tip"], [0.42, 0.0, -0.26], 84, 0.0, 0.045),
}


def solve_poses(a, path):
    problems = []
    for (pid, (body, contacts, tip, gr, ul, vl, *_)) in POSES.items():
      for sd in (1, -1):
       for sa in (1, -1):
        base = {**body, **ARM_R0, **ARM_L0, "shovel": {"angleDeg": 0}}
        targets = [
            {"type": "point", "bone": "shovel", "p": [L_TIP, 0, 0], "world": tip, "w": 3},
            {"type": "dir", "bone": "shovel", "axis": "u", "world": [math.cos(math.radians(gr)), -math.sin(math.radians(gr)), 0], "w": 2},
            {"type": "dir", "bone": "shovel", "axis": "v", "world": [0, 0, sd], "w": 0.5},
            {"type": "coincide", "a": {"bone": "hand-l", "p": [0.072 * S, 0, 0]}, "b": {"bone": "shovel", "p": [ul, vl, 0]}, "w": 3},
            {"type": "align", "a": {"bone": "hand-l", "axis": "v"}, "b": {"bone": "shovel", "axis": "u" if vl == 0 else "v"}, "sign": sa, "w": 0.4},
            {"type": "reg", "w": 0.002},
        ]
        problems.append({"id": "%s|%d|%d" % (pid, sd, sa), "restarts": 10, "contacts": [c for c in contacts if c != "blade-tip"], "base": base, "vars": ARM_VARS("r") + ARM_VARS("l") + [["shovel", "angleDeg"]], "targets": targets})
    sol = lib.run_ik({"asset": path, "problems": problems}, "shovel")
    best = {}
    for k, v in sol.items():
        pid = k.split("|")[0]
        if pid not in best or v["cost"] < best[pid]["cost"]:
            best[pid] = {**v, "variant": k}
    return best


ENV = {"airTempC": 15, "meanRadiantTempC": 15, "skyTempC": 8, "groundTempC": 13, "relativeHumidity": 0.6, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 150, "diffuseFraction": 1, "pressurePa": 101325}


def finish(doc, sol):
    import texts_shovel as X
    a = doc["asset"]
    lib.finish_common(doc, X, ENV, 2.0, "driving-blade", "dig-cycle")
    regs = {"gloves": {"contact": {"fraction": 0.5, "tempC": 15}}, "shoes": {"contact": {"fraction": 0.3, "tempC": 13}}, "shovel-handle": {"contact": {"fraction": 0.2, "tempC": 20}}, "head-skin": {"wettedness": 0.2}}
    soil = [{"id": "soil", "kind": "carried", "title": lib.tx("About 7 kg of moist soil on the blade", "Etwa 7 kg feuchte Erde auf dem Blatt"), "anchor": "blade-load", "fxN": 0, "fyN": -68.65, "source": "authored-estimate"}]
    spec = {"driving-blade": (5.5, "digging", 0, True, []), "prying-soil": (6.0, "digging", 0, True, []), "lifting-soil": (6.5, "lifting", 40, False, soil), "resting-on-shovel": (2.0, "recovering", 0, True, [])}
    sol["driving-blade"]["bones"].update({"neck": {"angleDeg": 2}, "head": {"angleDeg": 3}})
    a["poses"] = []
    for pid, (met, br, ext, tip, loads) in spec.items():
        a["poses"].append(lib.make_pose(a, pid, X.POSES[pid][0], X.POSES[pid][1], sol[pid]["bones"], FEET + (["blade-tip"] if tip else []), met, br, regs, {"plume": 0.8}, {}, external=ext, loads=loads))
    a["clips"] = X.clips(lib, FEET + ["blade-tip"], {p["id"]: p for p in a["poses"]})


if __name__ == "__main__":
    doc = build()
    path = lib.save_draft(doc, SLUG)
    sol = solve_poses(doc["asset"], path)
    print(json.dumps({k: v["atLimit"] for k, v in sol.items()}))
    finish(doc, sol)
    path = lib.set_dims(doc, SLUG, MASS + TOOL_KG)
    if "--write" in sys.argv:
        print(lib.write_asset(doc, SLUG))
