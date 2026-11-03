p = "tied_canopy.py"
s = open(p, encoding="utf8").read()
s = s.replace('''SHEETS = [(0.1, 1.38), (1.44, 2.72), (2.78, 4.06), (4.12, 5.4)]
''', '''SHEETS = [(0.1, 1.38), (1.44, 2.72), (2.78, 4.06), (4.12, 5.4)]
OLD_Y, BEAM_Y, WALL_H, LIFT = 0.12, 2.33, 5.35, 2.21


def lifted(shapes):
    for s in shapes:
        if s["id"] in ("wall", "wall-core", "wall-face"):
            s["y"], s["h"] = 0.0, WALL_H
        elif s["kind"] == "rect":
            s["y"] = r5(s["y"] + LIFT)
        elif s["kind"] == "poly":
            s["points"] = [[x, r5(y + LIFT)] for x, y in s["points"]]
        elif s["kind"] == "capsule":
            s["y0"], s["y1"] = r5(s["y0"] + LIFT), r5(s["y1"] + LIFT)
        elif s["kind"] == "ellipse":
            s["cy"] = r5(s["cy"] + LIFT)
    return shapes


def zigzag(points, lo):
    hi, step = points[-1][1], (points[-1][1] - points[0][1]) / (len(points) - 1)
    n = round((hi - lo) / step)
    xs = [points[0][0], points[1][0]]
    return [[xs[k % 2], r5(lo + (hi - lo) * k / n)] for k in range(n + 1)]


def raise_canopy(asset):
    asset["rig"]["bones"][0]["y"] = BEAM_Y
    asset["dimensions"]["heightM"] = WALL_H
    asset["summary"]["en"] = asset["summary"]["en"].replace("pinned to an externally insulated concrete wall", "pinned at door-head height (beam underside 2.25 m) to an externally insulated concrete wall")
    asset["summary"]["de"] = asset["summary"]["de"].replace("an einer außengedämmten Betonwand gelagert", "in Türsturzhöhe (Trägerunterkante 2,25 m) an einer außengedämmten Betonwand gelagert")
    assert "2.25" in asset["summary"]["en"] and "2,25" in asset["summary"]["de"]
    wall = next(r for r in asset["thermal"]["regions"] if r["id"] == "wall-face")
    wall["areaM2"] = round(wall["areaM2"] + LIFT, 2)
    for rep in asset["representations"].values():
        if rep.get("status") != "authored":
            continue
        for s in rep["geometry"]["elevation"]:
            if s["id"] == "wall-core" or s["id"] == "wall-insulation" or s["id"] == "wall-render" or s["id"] == "wall" or s["id"] == "wall-outer":
                s["y"], s["h"] = -BEAM_Y, WALL_H
            elif "zigzag" in s["id"]:
                s["points"] = zigzag(s["points"], -BEAM_Y + 0.30)
            elif "rebar" in s["id"]:
                s["points"] = [[x, (-BEAM_Y + 0.30 if y < 1 else y)] for x, y in s["points"]]
''')
s = s.replace('''for sc in (500, 200, 100, 50):
    install(A, sc, front(sc), plan(sc), doc, *EN[sc])''', '''raise_canopy(doc["asset"])
for sc in (500, 200, 100, 50):
    install(A, sc, lifted(front(sc)), plan(sc), doc, *(EN[sc][0] + LEVEL[0], EN[sc][1] + LEVEL[1]))''')
s = s.replace("EN = {", '''LEVEL = (" The canopy hangs at door-head height, beam underside 2.25 m above the ground, so the wall rises to 5.35 m and the strut climbs 2.9 m to its anchor 5.2 m above the ground; the front shows all of it from the ground up.",
         " Das Vordach hängt in Türsturzhöhe, Trägerunterkante 2,25 m über dem Boden, daher steigt die Wand auf 5,35 m und der Stab klettert 2,9 m zu seinem Anschluss 5,2 m über dem Boden; die Vorderansicht zeigt alles vom Boden an.")
EN = {''', 1)
open(p, "w", encoding="utf8").write(s)
