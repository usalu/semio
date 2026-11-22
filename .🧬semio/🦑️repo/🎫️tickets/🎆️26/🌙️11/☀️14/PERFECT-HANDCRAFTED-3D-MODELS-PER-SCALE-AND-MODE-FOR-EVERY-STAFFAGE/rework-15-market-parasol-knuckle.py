"""⛱️ Rework 15 market parasol: the mast tilts in its knuckle, not at its foot. The whole 2.1 m mast used to lean out of the 58 kg base at the foot clevis; a real crank parasol stands on a fixed lower mast that carries the winch, and only the upper mast with crown, runner and canopy tilts in the knuckle collar above the crank and below the furled runner.

Hand-stated numbers (metres along the mast from the foot pin, which is 0.26 m above ground): knuckle pin at 0.81 (1.07 m above ground) inside the existing collar 0.755..0.865; winch box and crank moved down from 1.12 to 0.66 (0.92 m above ground, under the knuckle; the furled runner at 1.044 no longer sits on the winch); lower mast o48, upper mast o40.
Usage: python3 rework-15-market-parasol-knuckle.py [--apply]
"""
import copy
import sys

from rework_15_kit import Kit, SCALES, rnd, text

kit = Kit("objects/market-parasol")
K, DOWN, OUT = 0.81, -0.46, 0.004
done = any(b["id"] == "mast-upper" for b in kit.asset["rig"]["bones"])
if done:
    raise SystemExit("already applied")

mast = kit.bone("mast")
mast.update({"lengthM": K, "joint": {"type": "fixed"}, "title": text("Lower mast with foot pin, winch and crank", "Unterer Mast mit Fußbolzen, Winde und Kurbel")})
mast["mass"].update({"kg": 1.4, "comX": 0.405})
upper = copy.deepcopy(mast)
upper.update({"id": "mast-upper", "parent": "mast", "x": K, "y": 0, "angleDeg": 0, "lengthM": 1.28, "joint": {"type": "revolute", "minDeg": -10, "maxDeg": 10},
              "title": text("Upper mast above the tilt knuckle", "Oberer Mast über dem Knickgelenk")})
upper["mass"].update({"kg": 1.8, "comX": 0.64})
kit.add_bone("mast", upper)
for id in ("crown", "runner"):
    bone = kit.bone(id)
    bone["parent"], bone["x"] = "mast-upper", round(bone["x"] - K, 6)
for pose in kit.asset["poses"]:
    if "mast" in pose["bones"]:
        pose["bones"]["mast-upper"] = pose["bones"].pop("mast")
for clip in kit.asset["clips"]:
    for key in clip["keys"]:
        if "mast" in key.get("bones", {}):
            key["bones"]["mast-upper"] = key["bones"].pop("mast")


def shifted(profile, du, dv=0.0):
    return [[p[0] + du, p[1] + dv] for p in profile]


for scale in SCALES:
    v = kit.volumes[scale]
    regions = {s["of"]: s for s in v["thermal"] if "of" in s}
    for name in ("body", "structure"):
        for s in list(v[name]):
            if s.get("bone") != "mast":
                continue
            if s["id"] in ("mast-upper", "member-mast-upper"):
                s["bone"], s["profile"] = "mast-upper", shifted(s["profile"], -K)
                if "member" in s:
                    s["member"] = "mast-upper"
            elif s["id"] in ("mast", "member-mast"):
                top = copy.deepcopy(s)
                top.update({"id": s["id"] + "-upper", "bone": "mast-upper", "profile": [[min(p[0], 2.07 if scale != "500" else 2.06) - K if p[0] > 1 else -0.02, p[1]] for p in s["profile"]]})
                s["profile"] = [[K + 0.02 if p[0] > 1 else p[0], p[1]] for p in s["profile"]]
                if "member" in top:
                    top["member"] = "mast-upper"
                    v[name].insert(v[name].index(s) + 1, rnd(top))
                else:
                    ref = regions.get(s["id"])
                    kit.put(scale, top, region=ref and ref["region"], member=None, after=s["id"])
                    if any(x.get("of") == s["id"] for x in v["structure"]):
                        v["structure"].append({"id": "st-" + top["id"], "of": top["id"], "member": "mast-upper", "part": "member"})
            elif s["id"] == "mast-winch" and s["kind"] == "box":
                s["transform"]["translation"] = rnd([s["transform"]["translation"][0] + DOWN, s["transform"]["translation"][1] + OUT, 0])
            elif s["id"] in ("mast-winch", "mast-crank"):
                s["profile"] = rnd(shifted(s["profile"], DOWN, OUT), 6)
            elif s["id"] == "mast-crank-knob":
                s["p0"], s["p1"] = rnd([s["p0"][0] + DOWN, s["p0"][1] + OUT, 0]), rnd([s["p1"][0] + DOWN, s["p1"][1] + OUT, 0])
            elif s["id"] == "mass-mast":
                top = copy.deepcopy(s)
                s["transform"]["translation"] = [0.405, 0, 0]
                top.update({"id": "mass-mast-upper", "bone": "mast-upper", "member": "mast-upper"})
                top["transform"]["translation"] = [0.64, 0, 0]
                v[name].insert(v[name].index(s) + 1, top)
    if scale == "50":
        kit.put(scale, {"id": "mast-knuckle-pin", "kind": "capsule", "bone": "mast-upper", "material": "steel-galvanised" if "steel-galvanised" in kit.asset["materials"] else kit.solid("50", "mast-pin")["material"],
                        "role": "detail", "tone": 0.3, "p0": [0, 0, -0.036], "p1": [0, 0, 0.036], "r0": 0.006, "r1": 0.006}, region=regions["mast-pin"]["region"], member="mast-upper", part="joint", after="mast-collar-knob")

    for s in list(kit.shapes(scale, "elevation")):
        if s["bone"] != "mast":
            continue
        if s["id"] == "mast":
            top = copy.deepcopy(s)
            pts = s["points"]
            if len(pts) == 2:
                top["points"], s["points"] = [[0, 0], [2.09 - K, 0]], [pts[0], [K, 0]]
            else:
                r = pts[-1][1] if len(pts) == 4 else 0.02
                top["points"] = [[0, -r], [2.09 - K, -r], [2.09 - K, r], [0, r]]
                low = pts[0][1]
                s["points"] = [[pts[0][0], low], [K, low], [K, -low], [pts[0][0], -low]]
            top.update({"id": "mast-upper", "bone": "mast-upper"})
            kit.put_shape(scale, "elevation", top, after="mast")
        elif s["id"] in ("mast-winch", "mast-crank-knob") and s["kind"] == "rect":
            s["x"], s["y"] = round(s["x"] + DOWN, 5), round(s["y"] + OUT, 5)
        elif s["id"] in ("mast-winch", "mast-crank"):
            s["points"] = rnd(shifted(s["points"], DOWN, OUT))
    for s in list(kit.shapes(scale, "front")):
        if s["bone"] == "mast" and s["id"] == "mast":
            top = copy.deepcopy(s)
            pts = s["points"]
            split = 0.26 + K
            if len(pts) == 2:
                top["points"], s["points"] = [[0, split], pts[1]], [pts[0], [0, split]]
            else:
                r = min(abs(p[0]) for p in pts)
                tube = {"50": 0.024}.get(scale, 0.022)
                top["points"] = [[-r, split], [r, split], [r, 2.35], [-r, 2.35]]
                if scale == "200":
                    s["points"] = [[-tube, 0.26], [tube, 0.26], [tube, split], [-tube, split]]
                else:
                    s["points"] = [[-0.023, 0.236], [0.023, 0.236], [0.023, 0.31], [tube, 0.31], [tube, split], [-tube, split], [-tube, 0.31], [-0.023, 0.31]]
            s.pop("followTip", None)
            top.update({"id": "mast-upper", "bone": "mast-upper"})
            kit.put_shape(scale, "front", top, after="mast")
    kit.decide(scale, "The lower mast stands fixed on the base and carries winch and crank at 0.92 m; the upper mast with crown, runner and canopy tilts up to 10 degrees in the knuckle at 1.07 m.",
               "Der untere Mast steht fest auf dem Sockel und trägt Winde und Kurbel auf 0,92 m; der obere Mast mit Krone, Schieber und Schirmdach neigt sich im Knickgelenk auf 1,07 m um bis zu 10 Grad.")
kit.finish(sys.argv)
