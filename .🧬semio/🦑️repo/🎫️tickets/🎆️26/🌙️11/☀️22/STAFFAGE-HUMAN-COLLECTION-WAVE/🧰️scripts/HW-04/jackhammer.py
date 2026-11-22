"""🔨️ Generator of `worker-with-jackhammer`: a 1.80 m, 92 kg road worker (male, 38) in hi-vis vest, hard hat with helmet-mounted ear defenders, gloves and safety boots, breaking concrete with a 30 kg class electric demolition hammer (2000 W nameplate) on a pointed chisel.

Body: the catalog figure `worker-carrying-box` without its carton, scaled to the stature (lib). Breaker: stated here per scale as bone-local parts (tool.py) on a bone carried by the right fist on the T-handle; both arms and the breaker joint solved by `ik.ts` so that both fists hold the T-handle and the chisel point stands on the ground.
Usage: python3 jackhammer.py
"""
import json
import math
import sys

import lib
import tool

SLUG = "worker-with-jackhammer"
STATURE = 1.80
MASS = 92.0
AGE = 38
S = STATURE / 1.741
TOOL_KG = 31.0
D = 0.26
L_TIP = 0.96
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]


def materials(a):
    a["materials"] += [
        {"id": "breaker-housing", "title": lib.tx("Glass-fibre polyamide motor housing", "Motorgehäuse aus glasfaserverstärktem Polyamid"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 1350, "conductivityWmK": 0.3, "source": "authored-estimate"},
        {"id": "cast-aluminium", "title": lib.tx("Cast aluminium gear and cylinder housing", "Getriebe- und Zylindergehäuse aus Aluminiumguss"), "emissivity": 0.6, "solarAbsorptivity": 0.5, "densityKgM3": 2700, "conductivityWmK": 120, "source": "literature", "reference": "ashrae-2021"},
        {"id": "tool-steel", "title": lib.tx("Hexagon chisel steel (28 mm)", "Sechskant-Meißelstahl (28 mm)"), "emissivity": 0.7, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 45, "source": "literature", "reference": "ashrae-2021"},
        {"id": "grip-rubber", "title": lib.tx("Rubber grips on the T-handle", "Gummigriffe am T-Griff"), "emissivity": 0.94, "solarAbsorptivity": 0.9, "source": "authored-estimate"},
        {"id": "earmuff-abs", "title": lib.tx("ABS ear-defender cups with foam cushions", "ABS-Kapselgehörschützer mit Schaumpolstern"), "emissivity": 0.92, "solarAbsorptivity": 0.5, "source": "authored-estimate", "reference": "en-352-3"},
    ]


def regions(a):
    a["thermal"]["operating"] = [{"id": "breaker", "title": lib.tx("Demolition hammer", "Abbruchhammer"), "levels": [{"v": 0, "title": lib.tx("Off", "Aus")}, {"v": 0.3, "title": lib.tx("Idling without load", "Leerlauf ohne Last")}, {"v": 1, "title": lib.tx("Breaking at full impact", "Brechen mit voller Schlagleistung")}]}]
    a["thermal"]["regions"] += [
        {"id": "breaker-housing", "title": lib.tx("Motor, gear and cylinder housing of the breaker", "Motor-, Getriebe- und Zylindergehäuse des Abbruchhammers"), "kind": "surface", "model": "operating", "material": "breaker-housing", "areaM2": 0.48, "characteristicLengthM": 0.2, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 30000, "operating": {"key": "breaker", "heatW": 350}, "ratedPowerW": 2000, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "en-62841-2-6"},
        {"id": "breaker-chisel", "title": lib.tx("Pointed chisel", "Spitzmeißel"), "kind": "surface", "model": "passive", "material": "tool-steel", "areaM2": 0.035, "characteristicLengthM": 0.03, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 12000, "source": "authored-estimate", "reference": "ashrae-2021"},
        {"id": "breaker-grips", "title": lib.tx("T-handle with rubber grips", "T-Griff mit Gummigriffen"), "kind": "surface", "model": "passive", "material": "grip-rubber", "areaM2": 0.07, "characteristicLengthM": 0.04, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 6000, "source": "authored-estimate", "reference": "ashrae-2021"},
    ]


def parts(scale):
    """🔨️ The breaker of one scale: u from the top of the T-handle down the tool axis, v along the T-bar (right grip at v = -D, left grip at v = +D), w across."""
    b = "breaker"
    HS, CH, GR = "breaker-housing", "breaker-chisel", "breaker-grips"
    bar = lambda pid, r, v0, v1, region, material, tone, z: tool.plate(pid, b, [(-r, v0), (r, v0), (r, v1), (-r, v1)], -r, r, region, material, tone, z)
    motor = [(0.0, -0.11), (0.0, 0.11), (0.05, 0.09), (0.33, 0.09), (0.33, -0.09), (0.05, -0.09)]
    if scale == "50":
        return [
            bar("t-bar", 0.016, -D + 0.06, D - 0.06, HS, "cast-aluminium", 0.45, 7.5),
            bar("grip-r", 0.021, -D - 0.07, -D + 0.065, GR, "grip-rubber", 0.12, 7.9),
            bar("grip-l", 0.021, D - 0.065, D + 0.07, GR, "grip-rubber", 0.12, 7.9),
            tool.plate("handle-posts", b, [(0.0, -0.13), (0.0, 0.13), (0.07, 0.08), (0.07, -0.08)], -0.03, 0.03, HS, "cast-aluminium", 0.45, 7.4),
            tool.plate("motor", b, [(0.05, -0.08), (0.06, -0.09), (0.31, -0.09), (0.33, -0.075), (0.33, 0.075), (0.31, 0.09), (0.06, 0.09), (0.05, 0.08)], -0.11, 0.11, HS, "breaker-housing", 0.55, 7.3),
            tool.revolved("cylinder", b, [(0.32, 0.068), (0.45, 0.065), (0.56, 0.055)], HS, "cast-aluminium", 0.4, 7.2),
            tool.revolved("chuck", b, [(0.55, 0.048), (0.60, 0.046), (0.64, 0.036)], HS, "cast-aluminium", 0.3, 7.25),
            tool.revolved("chisel", b, [(0.63, 0.016), (0.90, 0.015), (0.96, 0.003)], CH, "tool-steel", 0.2, 7.1),
        ]
    if scale == "100":
        return [
            bar("t-bar", 0.018, -D - 0.07, D + 0.07, GR, "grip-rubber", 0.15, 7.9),
            tool.plate("motor", b, motor, -0.11, 0.11, HS, "breaker-housing", 0.55, 7.3),
            tool.revolved("cylinder", b, [(0.32, 0.068), (0.45, 0.065), (0.56, 0.055), (0.64, 0.04)], HS, "cast-aluminium", 0.4, 7.2),
            tool.revolved("chisel", b, [(0.63, 0.016), (0.90, 0.015), (0.96, 0.004)], CH, "tool-steel", 0.2, 7.1),
        ]
    if scale == "200":
        return [
            bar("t-bar", 0.02, -D - 0.07, D + 0.07, GR, "grip-rubber", 0.15, 7.9),
            tool.plate("motor", b, motor, -0.11, 0.11, HS, "breaker-housing", 0.55, 7.3),
            tool.revolved("cylinder", b, [(0.32, 0.068), (0.56, 0.058), (0.64, 0.042)], HS, "cast-aluminium", 0.4, 7.2),
            tool.revolved("chisel", b, [(0.63, 0.02), (0.96, 0.02)], CH, "tool-steel", 0.2, 7.1),
        ]
    return [
        bar("t-bar", 0.03, -D - 0.07, D + 0.07, GR, "grip-rubber", 0.15, 7.9),
        tool.plate("body", b, [(0.0, -0.11), (0.0, 0.11), (0.33, 0.09), (0.40, 0.07), (0.62, 0.05), (0.62, -0.05), (0.40, -0.07), (0.33, -0.09)], -0.1, 0.1, HS, "breaker-housing", 0.5, 7.3),
        tool.revolved("chisel", b, [(0.60, 0.038), (0.96, 0.038)], CH, "tool-steel", 0.2, 7.1),
    ]


def ear_defenders(a):
    """🎧️ Helmet-mounted ear defenders (EN 352-3): a cup over each ear on an arm clipped into the helmet slot, in all views and scales (at 1:500 merged into the helmet)."""
    a["thermal"]["regions"].append({"id": "ear-defenders", "title": lib.tx("Helmet-mounted ear defenders", "Am Helm befestigter Kapselgehörschutz"), "kind": "surface", "model": "passive", "material": "earmuff-abs", "areaM2": 0.05, "characteristicLengthM": 0.08, "geometry": "sphere", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 4000, "source": "authored-estimate", "reference": "en-352-3"})
    hy, hx = 0.7014 * S, -0.0150 * S
    cu, cv, cw = 0.0, 0.002 * S, 0.0925 * S
    ru, rv, rw = 0.052, 0.041, 0.021
    for sc in lib.SCALES:
        g = a["representations"][sc]["geometry"]
        if sc == "500":
            for view in g:
                for sh in g[view]:
                    if sh["id"] == "helmet":
                        sh.pop("region", None)
                        sh["regions"] = ["helmet", "ear-defenders"]
            band = {"id": "ear-defenders", "kind": "ellipsoid", "bone": "head", "material": "earmuff-abs", "role": "body", "tone": 0.25, "radii": [0.05, 0.04, lib.r5(cw + 0.024)], "transform": {"translation": [cu, cv, 0]}}
            lib.add_solids(a, sc, "body", [band])
            lib.add_solids(a, sc, "thermal", [{"id": "th-ear-defenders", "of": "ear-defenders", "region": "ear-defenders"}])
            continue
        n = 20 if sc in ("50", "100") else 12
        base = {"bone": "head", "role": "body", "filled": True, "tone": 0.25, "region": "ear-defenders", "material": "earmuff-abs", "kind": "poly", "closed": True}
        g["elevation"].append({**base, "id": "ear-cup", "z": 5.45, "points": lib.ring(lib.ellipse(cu, cv, ru, rv, n))})
        for side, sg in (("l", 1), ("r", -1)):
            g["front"].append({**base, "id": "ear-cup-" + side, "z": 5.45, "points": lib.ring(lib.ellipse(sg * (cw + 0.004), hy + cu, rw, ru, n))})
            g["plan"].append({**base, "id": "ear-cup-" + side, "z": 5.45, "points": lib.ring(lib.ellipse(hx, sg * (cw + 0.004), rv, rw, n))})
        if sc in ("50", "100"):
            arm = {**base, "role": "detail", "tone": 0.3}
            g["elevation"].append({**arm, "id": "ear-arm", "z": 5.46, "points": lib.ring([(0.04, -0.008), (0.15 * S, -0.008), (0.15 * S, 0.008), (0.04, 0.008)])})
            for side, sg in (("l", 1), ("r", -1)):
                g["front"].append({**arm, "id": "ear-arm-" + side, "z": 5.46, "points": lib.ring([(sg * 0.105, hy + 0.04), (sg * 0.123, hy + 0.04), (sg * 0.128, hy + 0.15 * S), (sg * 0.11, hy + 0.15 * S)])})
        cups = []
        for side, sg in (("l", 1), ("r", -1)):
            cups.append({"id": "ear-cup-" + side, "kind": "ellipsoid", "bone": "head", "material": "earmuff-abs", "role": "body", "tone": 0.25, "radii": [ru, rv, rw], "transform": {"translation": [cu, cv, lib.r5(sg * (cw + 0.004))]}})
            if sc in ("50", "100"):
                cups.append({"id": "ear-arm-" + side, "kind": "capsule", "bone": "head", "material": "earmuff-abs", "role": "detail", "tone": 0.3, "p0": [0.045, 0, lib.r5(sg * 0.112)], "p1": [lib.r5(0.15 * S), 0, lib.r5(sg * 0.12)], "r0": 0.008, "r1": 0.008})
        lib.add_solids(a, sc, "body", cups)
        lib.add_solids(a, sc, "thermal", [{"id": "th-" + c["id"], "of": c["id"], "region": "ear-defenders"} for c in cups])


def build():
    doc = lib.load_base()
    a = doc["asset"]
    a["id"] = SLUG
    lib.strip_box(a)
    lib.scale_figure(a, S)
    lib.set_masses(a, MASS, "male")
    lib.set_body(a, MASS, STATURE, AGE, "male")
    grip = next(x for x in a["anchors"] if x["id"] == "grip-r")["x"]
    a["rig"]["bones"].append({"id": "breaker", "parent": "hand-r", "title": lib.tx("Electric demolition hammer on its chisel", "Elektro-Abbruchhammer auf seinem Meißel"), "x": grip, "y": D, "angleDeg": 0, "abductionDeg": 90, "lengthM": 0.03, "joint": {"type": "revolute", "minDeg": -25, "maxDeg": 25}, "mass": {"kg": TOOL_KG, "comX": 0.30, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "en-62841-2-6"}})
    a["anchors"] += [
        {"id": "chisel-tip", "bone": "breaker", "x": L_TIP, "y": 0, "kind": "contact", "title": lib.tx("Chisel point on the concrete", "Meißelspitze auf dem Beton"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.02, "frictionCoefficient": 0.5},
        {"id": "breaker-cog", "bone": "breaker", "x": 0.30, "y": 0, "kind": "load", "title": lib.tx("Centre of mass of the breaker", "Schwerpunkt des Abbruchhammers")},
    ]
    materials(a)
    regions(a)
    ear_defenders(a)
    rest_z = {"breaker": lib.r5(-0.18 * S)}
    for sc in lib.SCALES:
        tool.emit(a, sc, parts(sc), rest_z)
    return doc


ARM_R0 = {"upper-arm-r": {"angleDeg": 30, "abductionDeg": -5, "twistDeg": -20}, "forearm-r": {"angleDeg": 30, "twistDeg": 60}, "hand-r": {"angleDeg": 0, "abductionDeg": 0}}
ARM_L0 = {"upper-arm-l": {"angleDeg": 30, "abductionDeg": 5, "twistDeg": 20}, "forearm-l": {"angleDeg": 30, "twistDeg": -60}, "hand-l": {"angleDeg": 0, "abductionDeg": 0}}
ARM_VARS = lambda side: [["upper-arm-" + side, "angleDeg"], ["upper-arm-" + side, "abductionDeg", -60, 60], ["upper-arm-" + side, "twistDeg", -90, 90], ["forearm-" + side, "angleDeg"], ["forearm-" + side, "twistDeg", -120, 120], ["hand-" + side, "angleDeg"], ["hand-" + side, "abductionDeg", -35, 35]]

# id: (body offsets, contacts, chisel tip, tilt of the tool top towards the operator in degrees)
STANCE = {"thigh-l": {"angleDeg": 14}, "shank-l": {"angleDeg": -22}, "foot-l": {"angleDeg": 8}, "thigh-r": {"angleDeg": -6}, "shank-r": {"angleDeg": -16}, "foot-r": {"angleDeg": 22}}
POSES = {
    "breaking": ({**STANCE, "pelvis": {"angleDeg": -6}, "torso": {"angleDeg": -10}, "neck": {"angleDeg": 10}, "head": {"angleDeg": 10}}, FEET + ["chisel-tip"], [0.50, 0.0, 0.0], 5),
    "pressing-down": ({"thigh-l": {"angleDeg": 22}, "shank-l": {"angleDeg": -34}, "foot-l": {"angleDeg": 12}, "thigh-r": {"angleDeg": 0}, "shank-r": {"angleDeg": -24}, "foot-r": {"angleDeg": 24}, "pelvis": {"angleDeg": -10}, "torso": {"angleDeg": -18}, "neck": {"angleDeg": 14}, "head": {"angleDeg": 12}}, FEET + ["chisel-tip"], [0.48, 0.0, 0.0], 9),
    "rocking-out": ({**STANCE, "pelvis": {"angleDeg": -2}, "torso": {"angleDeg": -2}, "neck": {"angleDeg": 8}, "head": {"angleDeg": 10}}, FEET + ["chisel-tip"], [0.56, 0.0, 0.0], 20),
    "standing-ready": ({"torso": {"angleDeg": 2}, "neck": {"angleDeg": 2}, "head": {"angleDeg": 4}}, FEET + ["chisel-tip"], [0.42, 0.0, 0.0], 0),
}


def solve_poses(path):
    problems = []
    for pid, (body, contacts, tip, tilt) in POSES.items():
        for sd in (1, -1):
            for sa in (1, -1):
                t = math.radians(tilt)
                targets = [
                    {"type": "point", "bone": "breaker", "p": [L_TIP, 0, 0], "world": tip, "w": 3},
                    {"type": "dir", "bone": "breaker", "axis": "u", "world": [math.sin(t), -math.cos(t), 0], "w": 2},
                    {"type": "dir", "bone": "breaker", "axis": "v", "world": [0, 0, sd], "w": 1},
                    {"type": "coincide", "a": {"bone": "hand-l", "p": [0.072 * S, 0, 0]}, "b": {"bone": "breaker", "p": [0, D, 0]}, "w": 3},
                    {"type": "align", "a": {"bone": "hand-l", "axis": "v"}, "b": {"bone": "breaker", "axis": "v"}, "sign": sa, "w": 0.4},
                    {"type": "reg", "w": 0.002},
                ]
                problems.append({"id": "%s|%d|%d" % (pid, sd, sa), "contacts": FEET, "base": {**body, **ARM_R0, **ARM_L0, "breaker": {"angleDeg": 0}}, "vars": ARM_VARS("r") + ARM_VARS("l") + [["breaker", "angleDeg"]], "targets": targets})
    sol = lib.run_ik({"asset": path, "problems": problems}, "jackhammer")
    best = {}
    for k, v in sol.items():
        pid = k.split("|")[0]
        if pid not in best or v["cost"] < best[pid]["cost"]:
            best[pid] = {**v, "variant": k}
    return best


ENV = {"airTempC": 25, "meanRadiantTempC": 32, "skyTempC": 10, "groundTempC": 35, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 600, "diffuseFraction": 0.2, "pressurePa": 101325}


def finish(doc, sol):
    import texts_jackhammer as X
    a = doc["asset"]
    lib.finish_common(doc, X, ENV, 2.0, "breaking", "hammering")
    contacts = FEET + ["chisel-tip"]
    regs = {"gloves": {"contact": {"fraction": 0.5, "tempC": 30}}, "shoes": {"contact": {"fraction": 0.3, "tempC": 35}}, "breaker-grips": {"contact": {"fraction": 0.4, "tempC": 33}}, "breaker-chisel": {"contact": {"fraction": 0.05, "tempC": 35}}, "head-skin": {"wettedness": 0.25}}
    spec = {"breaking": (6.3, "breaking", 1, 120), "pressing-down": (6.5, "breaking", 1, 120), "rocking-out": (6.0, "straining", 0.3, 60), "standing-ready": (1.8, "rest", 0.3, 0)}
    a["poses"] = [lib.make_pose(a, pid, X.POSES[pid][0], X.POSES[pid][1], sol[pid]["bones"], contacts, met, br, regs, {"plume": 0.8}, {"breaker": op}, external=ext, sun=0.5) for pid, (met, br, op, ext) in spec.items()]
    a["clips"] = X.clips(lib, contacts, {k: {"bones": p["bones"]} for k, p in ((p["id"], p) for p in a["poses"])})


if __name__ == "__main__":
    doc = build()
    path = lib.save_draft(doc, SLUG)
    sol = solve_poses(path)
    print(json.dumps({k: (v["variant"], v["cost"], v["atLimit"]) for k, v in sol.items()}))
    finish(doc, sol)
    path = lib.set_dims(doc, SLUG, MASS + TOOL_KG)
    if "--write" in sys.argv:
        print(lib.write_asset(doc, SLUG))
