"""📐️ Generator of `surveyor-with-total-station`: a 1.66 m, 63 kg surveyor (female, 31) in hi-vis vest, hard hat and safety boots, standing behind a total station on an aluminium tripod and sighting through the eyepiece.

Body: the catalog figure `worker-carrying-box` without its carton, knee pads and cargo pockets, scaled to the stature, with de Leva's female segment masses (lib). Tripod and instrument: a static object on fixed bones placed from the pelvis (the legs stand the same in every pose, so the tripod feet stay on the ground), stated per scale as bone-local parts (tool.py). Poses: trunk, neck and head solved by `ik.ts` to bring the eye to the eyepiece, the hands onto the drive knobs and the keyboard.
Usage: python3 surveyor.py
"""
import json
import math
import sys

import lib
import tool

SLUG = "surveyor-with-total-station"
STATURE = 1.66
MASS = 63.0
AGE = 31
S = STATURE / 1.741
HP = 0.9406 * S
CX = 0.40
HEAD_U = 1.32
FOOT_R = 0.52
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]
TRIPOD_FEET = ["tripod-foot-a", "tripod-foot-b", "tripod-foot-c"]
LEGS = {"a": 0.0, "b": 120.0, "c": -120.0}


def leg_rest(azimuth_deg):
    """The rest placement (angle, abduction) and length of a tripod leg from the head (u = HEAD_U on the vertical tripod bone) to its foot on the ground at FOOT_R from the centre, `azimuth` from ahead towards the left."""
    t = math.radians(azimuth_deg)
    dx, dz = FOOT_R * math.cos(t), FOOT_R * math.sin(t)
    dy = -(HEAD_U - 0.01)
    n = math.sqrt(dx * dx + dy * dy + dz * dz)
    du, dv, dw = dy / n, -dx / n, dz / n
    p = math.asin(dw)
    a = math.atan2(dv / math.cos(p), du / math.cos(p))
    return round(math.degrees(a), 4), round(math.degrees(p), 4), round(n, 4)


def materials(a):
    a["materials"] += [
        {"id": "anodised-aluminium", "title": lib.tx("Anodised aluminium tripod legs", "Stativbeine aus eloxiertem Aluminium"), "emissivity": 0.8, "solarAbsorptivity": 0.6, "densityKgM3": 2700, "conductivityWmK": 160, "source": "literature", "reference": "ashrae-2021"},
        {"id": "tripod-steel", "title": lib.tx("Steel shoes and clamps", "Stahlschuhe und Klemmen"), "emissivity": 0.7, "solarAbsorptivity": 0.65, "source": "literature", "reference": "ashrae-2021"},
        {"id": "instrument-housing", "title": lib.tx("Instrument housing (polycarbonate and cast aluminium)", "Instrumentengehäuse (Polycarbonat und Aluminiumguss)"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "source": "authored-estimate"},
        {"id": "display-glass", "title": lib.tx("Display and keyboard", "Anzeige und Tastatur"), "emissivity": 0.9, "solarAbsorptivity": 0.85, "source": "authored-estimate"},
    ]


def regions(a):
    a["thermal"]["operating"] = [{"id": "instrument", "title": lib.tx("Total station", "Tachymeter"), "levels": [{"v": 0, "title": lib.tx("Off", "Aus")}, {"v": 0.5, "title": lib.tx("Standby, display on", "Bereitschaft, Anzeige an")}, {"v": 1, "title": lib.tx("Measuring with EDM and motor drives", "Messen mit EDM und Motorantrieben")}]}]
    a["thermal"]["regions"] += [
        {"id": "instrument", "title": lib.tx("Total station housing, display and telescope", "Gehäuse, Anzeige und Fernrohr des Tachymeters"), "kind": "surface", "model": "operating", "material": "instrument-housing", "areaM2": 0.2, "characteristicLengthM": 0.18, "geometry": "vertical-plate", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 15000, "operating": {"key": "instrument", "heatW": 6}, "ratedPowerW": 7, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "iso-17123-3"},
        {"id": "tripod", "title": lib.tx("Aluminium tripod", "Aluminiumstativ"), "kind": "surface", "model": "passive", "material": "anodised-aluminium", "areaM2": 0.36, "characteristicLengthM": 0.04, "geometry": "cylinder", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 5000, "source": "authored-estimate", "reference": "ashrae-2021"},
    ]


def parts(scale):
    """🔭️ Tripod and instrument of one scale. Tripod bone: u up from the ground centre, v backwards (towards the surveyor), w to the left; legs on their own bones from the head down to the feet; instrument on its bone on the head (u up from the tribrach base)."""
    T, I = "tripod", "instrument"
    out = []
    legs = {k: leg_rest(z)[2] for k, z in LEGS.items()}
    r_leg = {"50": 0.017, "100": 0.017, "200": 0.02, "500": 0.03}[scale]
    for k, n in legs.items():
        if scale in ("50", "100"):
            out.append(tool.revolved("leg-" + k, "tripod-leg-" + k, [(0.03, 0.022), (0.55, 0.02)], T, "anodised-aluminium", 0.55, 6.5))
            out.append(tool.revolved("leg-" + k + "-lower", "tripod-leg-" + k, [(0.53, 0.014), (n - 0.06, 0.014)], T, "anodised-aluminium", 0.55, 6.4, slender=True))
            out.append(tool.revolved("shoe-" + k, "tripod-leg-" + k, [(n - 0.08, 0.016), (n - 0.02, 0.012), (n, 0.003)], T, "tripod-steel", 0.2, 6.45))
        else:
            out.append(tool.revolved("leg-" + k, "tripod-leg-" + k, [(0.03, r_leg), (n - 0.2, r_leg), (n, 0.006)], T, "anodised-aluminium", 0.55, 6.5, slender=True))
    if scale != "500":
        out.append(tool.revolved("tripod-head", T, [(HEAD_U - 0.05, 0.075), (HEAD_U, 0.075)], T, "anodised-aluminium", 0.5, 6.6))
    if scale == "50":
        out += [
            tool.revolved("tribrach", I, [(0.0, 0.07), (0.035, 0.068), (0.045, 0.06)], I, "instrument-housing", 0.3, 6.7),
            tool.plate("alidade", I, [(0.04, -0.075), (0.04, 0.09), (0.11, 0.09), (0.13, 0.075), (0.13, -0.075)], -0.085, 0.085, I, "instrument-housing", 0.85, 6.8),
            tool.plate("display", I, [(0.055, 0.09), (0.055, 0.1), (0.12, 0.1), (0.12, 0.09)], -0.06, 0.06, I, "display-glass", 0.15, 6.85),
            tool.plate("standard-l", I, [(0.12, -0.05), (0.12, 0.05), (0.27, 0.04), (0.27, -0.04)], 0.055, 0.08, I, "instrument-housing", 0.85, 6.9),
            tool.plate("standard-r", I, [(0.12, -0.05), (0.12, 0.05), (0.27, 0.04), (0.27, -0.04)], -0.08, -0.055, I, "instrument-housing", 0.85, 6.9),
            tool.plate("telescope", I, [(0.155, -0.10), (0.155, 0.075), (0.165, 0.09), (0.215, 0.09), (0.225, 0.075), (0.225, -0.10)], -0.04, 0.04, I, "instrument-housing", 0.7, 6.95),
            tool.plate("eyepiece", I, [(0.175, 0.09), (0.175, 0.125), (0.205, 0.125), (0.205, 0.09)], -0.016, 0.016, I, "display-glass", 0.1, 6.97),
            tool.plate("drive-knob-r", I, [(0.17, -0.02), (0.17, 0.02), (0.21, 0.02), (0.21, -0.02)], -0.11, -0.08, I, "display-glass", 0.1, 6.98),
            tool.plate("handle", I, [(0.27, -0.06), (0.27, 0.06), (0.30, 0.05), (0.30, -0.05)], -0.015, 0.015, I, "instrument-housing", 0.85, 6.99),
        ]
    elif scale == "100":
        out += [
            tool.revolved("tribrach", I, [(0.0, 0.07), (0.045, 0.065)], I, "instrument-housing", 0.3, 6.7),
            tool.plate("alidade", I, [(0.04, -0.075), (0.04, 0.1), (0.13, 0.1), (0.13, -0.075)], -0.085, 0.085, I, "instrument-housing", 0.85, 6.8),
            tool.plate("standards", I, [(0.12, -0.05), (0.12, 0.05), (0.30, 0.04), (0.30, -0.04)], -0.08, 0.08, I, "instrument-housing", 0.85, 6.9),
            tool.plate("telescope", I, [(0.155, -0.10), (0.155, 0.125), (0.225, 0.125), (0.225, -0.10)], -0.04, 0.04, I, "instrument-housing", 0.7, 6.95),
        ]
    elif scale == "200":
        out += [
            tool.plate("instrument-body", I, [(0.0, -0.075), (0.0, 0.1), (0.13, 0.1), (0.13, 0.05), (0.30, 0.04), (0.30, -0.04), (0.13, -0.05), (0.13, -0.075)], -0.085, 0.085, I, "instrument-housing", 0.85, 6.8),
            tool.plate("telescope", I, [(0.155, -0.10), (0.155, 0.125), (0.225, 0.125), (0.225, -0.10)], -0.04, 0.04, I, "instrument-housing", 0.7, 6.95),
        ]
    else:
        out += [tool.plate("instrument-body", I, [(-0.06, -0.10), (-0.06, 0.125), (0.30, 0.125), (0.30, -0.10)], -0.085, 0.085, I, "instrument-housing", 0.85, 6.8)]
    return out


ZT = -0.031 * S


def build():
    doc = lib.load_base()
    a = doc["asset"]
    a["id"] = SLUG
    lib.strip_box(a)
    lib.drop_parts(a, ["knee-pad-l", "knee-pad-r", "cargo-pocket-l", "cargo-pocket-r"])
    lib.scale_figure(a, S)
    for sd in a["volumes"]["500"]["structure"]:
        if sd["id"] in ("joint-shoulder-l", "joint-shoulder-r"):
            sd["radii"] = [lib.r5(r * 1.02) for r in sd["radii"]]
    lib.set_masses(a, MASS, "female")
    lib.set_body(a, MASS, STATURE, AGE, "female")
    a["rig"]["bones"].append({"id": "tripod", "parent": "pelvis", "title": lib.tx("Tripod standing in front of the surveyor", "Stativ vor der vermessenden Person"), "x": lib.r5(-HP), "y": -CX, "z": lib.r5(ZT), "angleDeg": 0, "lengthM": HEAD_U, "joint": {"type": "fixed"}, "mass": {"kg": 5.0, "comX": 0.85, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "iso-12858-2"}})
    for k, az in LEGS.items():
        ang, ab, n = leg_rest(az)
        a["rig"]["bones"].append({"id": "tripod-leg-" + k, "parent": "tripod", "title": lib.tx("Tripod leg %s" % k.upper(), "Stativbein %s" % k.upper()), "x": HEAD_U - 0.01, "y": 0, "angleDeg": ang, "abductionDeg": ab, "lengthM": n, "joint": {"type": "fixed"}})
        a["anchors"].append({"id": "tripod-foot-" + k, "bone": "tripod-leg-" + k, "x": n, "y": 0, "kind": "contact", "title": lib.tx("Tripod shoe %s on the ground" % k.upper(), "Stativschuh %s am Boden" % k.upper()), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.02, "frictionCoefficient": 0.6})
    a["rig"]["bones"].append({"id": "instrument", "parent": "tripod", "title": lib.tx("Total station on its tribrach", "Tachymeter auf dem Dreifuß"), "x": HEAD_U, "y": 0, "angleDeg": 0, "lengthM": 0.30, "joint": {"type": "fixed"}, "mass": {"kg": 5.3, "comX": 0.15, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "iso-17123-3"}})
    a["anchors"] += [
        {"id": "eyepiece", "bone": "instrument", "x": 0.19, "y": 0.125, "kind": "attachment", "title": lib.tx("Telescope eyepiece", "Okular des Fernrohrs")},
        {"id": "instrument-cog", "bone": "instrument", "x": 0.15, "y": 0, "kind": "load", "title": lib.tx("Centre of mass of the instrument", "Schwerpunkt des Instruments")},
    ]
    materials(a)
    regions(a)
    rest_z = {"tripod": ZT, "instrument": ZT, **{"tripod-leg-" + k: ZT for k in LEGS}}
    head500 = [{"id": "st-tripod-head", "kind": "capsule", "bone": "tripod", "material": "anodised-aluminium", "role": "body", "tone": 0.5, "p0": [HEAD_U - 0.04, 0, 0], "p1": [HEAD_U - 0.04, 0, 0.001], "r0": 0.04, "r1": 0.04, "member": "tripod", "part": "member"}]
    for sc in lib.SCALES:
        tool.emit(a, sc, parts(sc), rest_z, mass_marks=head500 if sc == "500" else ())
    return doc


EYE_R = [0.0214 * S, -0.1003 * S, -0.031 * S]
TIP = [0.12 * S, 0, 0]
ARM_R0 = {"upper-arm-r": {"angleDeg": 45, "abductionDeg": -10, "twistDeg": -20}, "forearm-r": {"angleDeg": 60, "twistDeg": 40}, "hand-r": {"angleDeg": 0, "abductionDeg": 0}}
ARM_L0 = {"upper-arm-l": {"angleDeg": 35, "abductionDeg": 10, "twistDeg": 20}, "forearm-l": {"angleDeg": 60, "twistDeg": -40}, "hand-l": {"angleDeg": 0, "abductionDeg": 0}}
HANG_L = {"upper-arm-l": {"angleDeg": 4, "abductionDeg": 3}, "forearm-l": {"angleDeg": 12}, "hand-l": {"angleDeg": 5}}
ARM_VARS = lambda side: [["upper-arm-" + side, "angleDeg"], ["upper-arm-" + side, "abductionDeg", -50, 50], ["upper-arm-" + side, "twistDeg", -90, 90], ["forearm-" + side, "angleDeg"], ["forearm-" + side, "twistDeg", -120, 120], ["hand-" + side, "angleDeg"], ["hand-" + side, "abductionDeg", -35, 35]]
INSTR = lambda u, v, w: {"bone": "instrument", "p": [u, v, w]}


def solve_poses(path):
    pr = []
    trunk = [["torso", "angleDeg", -60, 10], ["neck", "angleDeg"], ["head", "angleDeg"]]
    pr.append({"id": "sighting", "contacts": FEET, "base": {"torso": {"angleDeg": -25}, "neck": {"angleDeg": 10}, "head": {"angleDeg": 10}, **ARM_R0, **ARM_L0},
               "vars": trunk + ARM_VARS("r") + ARM_VARS("l"),
               "targets": [{"type": "coincide", "a": {"bone": "head", "p": EYE_R}, "b": INSTR(0.19, 0.145, 0), "w": 3},
                           {"type": "dir", "bone": "head", "axis": "v", "world": [-1, 0.0, 0], "w": 0.1},
                           {"type": "coincide", "a": {"bone": "hand-r", "p": TIP}, "b": INSTR(0.19, 0.0, -0.11), "w": 2},
                           {"type": "coincide", "a": {"bone": "hand-l", "p": TIP}, "b": INSTR(0.09, 0.02, 0.095), "w": 2},
                           {"type": "reg", "w": 0.002}]})
    pr.append({"id": "reading-display", "contacts": FEET, "base": {"torso": {"angleDeg": -12}, "neck": {"angleDeg": 15}, "head": {"angleDeg": 15}, **ARM_R0, **HANG_L},
               "vars": [["torso", "angleDeg", -22, 0], ["neck", "angleDeg"], ["head", "angleDeg"]] + ARM_VARS("r"),
               "targets": [{"type": "dir", "bone": "head", "axis": "v", "world": [-math.cos(math.radians(38)), -math.sin(math.radians(38)), 0], "w": 1},
                           {"type": "point", "bone": "head", "p": EYE_R, "world": [None, None, None], "w": 0},
                           {"type": "coincide", "a": {"bone": "hand-r", "p": TIP}, "b": INSTR(0.075, 0.108, -0.03), "w": 2},
                           {"type": "reg", "w": 0.004}]})
    pr.append({"id": "standing-by", "contacts": FEET, "base": {"torso": {"angleDeg": 0}, "neck": {"angleDeg": -1}, "head": {"angleDeg": 1}, **ARM_R0, **HANG_L},
               "vars": ARM_VARS("r"),
               "targets": [{"type": "coincide", "a": {"bone": "hand-r", "p": [0.075 * S, 0, 0]}, "b": INSTR(0.29, 0.0, 0.0), "w": 2},
                           {"type": "reg", "w": 0.004}]})
    return lib.run_ik({"asset": path, "problems": pr}, "surveyor")


ENV = {"airTempC": 12, "meanRadiantTempC": 14, "skyTempC": 2, "groundTempC": 11, "relativeHumidity": 0.7, "windMS": 2.0, "windDirectionDeg": 180, "solarWM2": 120, "diffuseFraction": 0.8, "pressurePa": 101325}


def finish(doc, sol):
    import texts_surveyor as X
    a = doc["asset"]
    lib.finish_common(doc, X, ENV, 1.6, "sighting", "measuring-cycle")
    contacts = FEET + TRIPOD_FEET
    regs = {"gloves": {"contact": {"fraction": 0.15, "tempC": 12}}, "shoes": {"contact": {"fraction": 0.3, "tempC": 11}}, "tripod": {"contact": {"fraction": 0.02, "tempC": 11}}}
    spec = {"sighting": (1.8, "concentrating", 1), "reading-display": (1.7, "light-work", 1), "standing-by": (1.5, "rest", 0.5)}
    a["poses"] = [lib.make_pose(a, pid, X.POSES[pid][0], X.POSES[pid][1], sol[pid]["bones"], contacts, met, br, regs, {"plume": 0.6}, {"instrument": op}) for pid, (met, br, op) in spec.items()]
    a["clips"] = X.clips(lib, contacts, sol)


if __name__ == "__main__":
    doc = build()
    path = lib.save_draft(doc, SLUG)
    sol = solve_poses(path)
    print(json.dumps({k: (v["cost"], v["atLimit"]) for k, v in sol.items()}))
    finish(doc, sol)
    path = lib.set_dims(doc, SLUG, MASS + 5.0 + 5.3)
    if "--write" in sys.argv:
        print(lib.write_asset(doc, SLUG))
