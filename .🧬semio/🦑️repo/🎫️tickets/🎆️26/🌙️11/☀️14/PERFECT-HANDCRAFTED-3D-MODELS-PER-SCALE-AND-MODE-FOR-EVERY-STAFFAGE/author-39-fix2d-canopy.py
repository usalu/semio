import os, copy
__here = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(__here, "author-39-lib.py")).read())
exec(open(os.path.join(__here, "author-39-canopy-geom.py")).read())
r4 = lambda p: [[round(a, 4), round(b, 4)] for a, b in p]
BEAM_V0, BEAM_V1 = -0.06, 0.1744


def mutate(reps):
    for sc in ("50", "100"):
        g = reps[sc]["geometry"]
        for s in g["elevation"]:
            if s["id"] == "clevis-beam":
                for k in ("x", "y", "w", "h", "r"):
                    s.pop(k, None)
                s.update({"kind": "poly", "points": r4(CHEEK_BEAM), "closed": True})
            if s["id"] == "clevis-wall":
                for k in ("x", "y", "w", "h", "r"):
                    s.pop(k, None)
                s.update({"kind": "poly", "points": r4(CHEEK_WALL), "closed": True})
            if s["id"] == "pin-wall":
                s["cx"], s["cy"] = round(P_WALL[0], 4), round(P_WALL[1], 4)
        for s in g["front"]:
            if s["id"] == "clevis-beam":
                s["y"], s["h"], s["x"], s["w"] = round(2.33 + BEAM_V0, 4), round(BEAM_V1 - BEAM_V0, 4), -0.05, 0.1
            if s["id"] == "clevis-wall":
                s["y"], s["h"] = 5.11, 0.2
            if s["id"] == "pin-wall":
                s["points"] = [[-0.04, round(2.33 + P_WALL[1], 4)], [0.04, round(2.33 + P_WALL[1], 4)]]
        for s in g["plan"]:
            if s["id"] == "clevis-beam":
                x0 = min(p[0] for p in CHEEK_BEAM)
                s["x"], s["w"], s["y"], s["h"] = round(x0, 4), round(4.06 - x0, 4), -0.05, 0.1
                s.pop("r", None)
            if s["id"] == "clevis-wall":
                s["x"], s["w"] = 0, round(max(p[0] for p in CHEEK_WALL), 4)
            if s["id"] == "pin-wall":
                s["points"] = [[round(P_WALL[0], 4), -0.04], [round(P_WALL[0], 4), 0.04]]
    g = reps["50"]["geometry"]
    for view in ("elevation", "front", "plan"):
        ref = [s for s in g[view] if s["id"] == "pin-beam"][0]
        n = copy.deepcopy(ref)
        n["id"] = "pin-beam-2"
        if view == "elevation":
            n["cx"], n["cy"], n["rx"], n["ry"] = round(P2[0], 4), round(P2[1], 4), 0.01, 0.01
        elif view == "front":
            n["points"] = [[-0.05, round(2.33 + P2[1], 4)], [0.05, round(2.33 + P2[1], 4)]]
        else:
            n["points"] = [[round(P2[0], 4), -0.05], [round(P2[0], 4), 0.05]]
        g[view].append(n)


patch_representations("tied-canopy", mutate)
