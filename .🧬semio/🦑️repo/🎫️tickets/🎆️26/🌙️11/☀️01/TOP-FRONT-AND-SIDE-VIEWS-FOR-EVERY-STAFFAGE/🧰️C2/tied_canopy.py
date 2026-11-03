from lib import *

A = "tied-canopy"
doc = load(A)
B = "canopy"
SHEETS = [(0.1, 1.38), (1.44, 2.72), (2.78, 4.06), (4.12, 5.4)]
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


def front(sc):
    if sc == 500:
        return [R("wall", B, "body", 1, -0.5, 0, 1.0, 3.14, tone=0.6, regions=["wall-face", "wall-plate"], material="concrete"),
                P("beam", B, "body", 3, [(-0.5, 0.2), (-0.041, 0.2), (-0.041, 0.04), (0.041, 0.04), (0.041, 0.2), (0.5, 0.2), (0.5, 0.22), (-0.5, 0.22)], tone=0.4, regions=["canopy-steel", "canopy-glass"], material="steel"),
                line("strut", B, "body", 5, [(0, 0.2), (0, 3.0)], tone=0.45, region="rod-steel", material="steel", pen=0.13)]
    if sc == 200:
        return [R("wall-core", B, "body", 1, -0.5, 0, 1.0, 3.14, tone=0.6, regions=["wall-face", "wall-plate"], material="concrete"),
                R("beam", B, "body", 3, -0.041, 0.04, 0.082, 0.16, tone=0.4, region="canopy-steel", material="steel"),
                R("glass-roof", B, "glazing", 3.5, -0.5, 0.2, 1.0, 0.02, tone=0.92, region="canopy-glass", material="glass", pen=0.1),
                line("strut", B, "body", 5, [(0, 0.12), (0, 3.0)], tone=0.45, region="rod-steel", material="steel", pen=0.3)]
    s = [R("wall-face", B, "body", 1, -0.5, 0, 1.0, 3.14, tone=0.55 if sc == 50 else 0.6, region="wall-face", material="render" if sc == 50 else "concrete"),
         R("pad-beam", B, "body", 1.4, -0.1, 0, 0.2, 0.24, tone=0.7, region="wall-plate", material="anchor-pad"),
         R("pad-strut", B, "body", 1.4, -0.1, 2.88, 0.2, 0.24, tone=0.7, region="wall-plate", material="anchor-pad"),
         R("plate-beam", B, "body", 1.5, -0.08, 0.02, 0.16, 0.2, tone=0.3, region="wall-plate", material="steel"),
         R("plate-strut", B, "body", 1.5, -0.08, 2.9, 0.16, 0.2, tone=0.3, region="wall-plate", material="steel"),
         R("beam", B, "body", 3, -0.041, 0.04, 0.082, 0.16, tone=0.4, region="canopy-steel", material="steel"),
         R("glass-0", B, "glazing", 3.5, -0.5, 0.2, 1.0, 0.02, tone=0.92, region="canopy-glass", material="glass"),
         C("strut-tube", B, "body", 5, 0, 0.12, 0, 3.0, 0.03, tone=0.45, region="rod-steel", material="steel"),
         R("clevis-beam", B, "body", 4.5, -0.05, 0, 0.1, 0.24, tone=0.3, region="rod-steel", material="steel", r=0.01),
         R("clevis-wall", B, "body", 4.5, -0.04, 2.88, 0.08, 0.24, tone=0.3, region="rod-steel", material="steel", r=0.01)]
    if sc == 100:
        s = [x for x in s if x["id"] not in ("plate-beam", "plate-strut")]
        return s
    s += [line("beam-flange-top", B, "detail", 3.1, [(-0.041, 0.1926), (0.041, 0.1926)], region="canopy-steel"),
          line("beam-flange-bottom", B, "detail", 3.1, [(-0.041, 0.0474), (0.041, 0.0474)], region="canopy-steel"),
          line("pin-beam", B, "detail", 5.3, [(-0.05, 0.12), (0.05, 0.12)], tone=0.1, region="rod-steel"),
          line("pin-wall", B, "detail", 5.3, [(-0.04, 3.0), (0.04, 3.0)], tone=0.1, region="rod-steel"),
          line("strut-axis", B, "detail", 5.1, [(0, 0.15), (0, 2.97)], tone=0.3, region="rod-steel")]
    return s


def plan(sc):
    if sc == 500:
        return [R("wall", B, "body", 1, -0.5, -0.5, 0.5, 1.0, tone=0.6, regions=["wall-face", "wall-plate"], material="concrete"),
                R("roof", B, "body", 2, 0.1, -0.5, 5.3, 1.0, tone=0.85, regions=["canopy-glass"], material="glass"),
                R("beam", B, "body", 3, 0, -0.041, 5.5, 0.082, tone=0.4, region="canopy-steel", material="steel"),
                line("strut", B, "body", 5, [(0, 0), (4.0, 0)], tone=0.45, region="rod-steel", material="steel", pen=0.13)]
    if sc == 200:
        return [R("wall-core", B, "body", 1, -0.5, -0.5, 0.3, 1.0, tone=0.6, region="wall-face", material="concrete"),
                R("wall-outer", B, "body", 1, -0.2, -0.5, 0.2, 1.0, tone=0.9, regions=["wall-face", "wall-plate"], material="mineral-wool"),
                R("glass-roof", B, "glazing", 2, 0.1, -0.5, 5.3, 1.0, tone=0.92, region="canopy-glass", material="glass", pen=0.1),
                R("beam", B, "body", 3, 0, -0.041, 5.5, 0.082, tone=0.4, region="canopy-steel", material="steel"),
                line("strut", B, "body", 5, [(0, 0), (4.0, 0)], tone=0.45, region="rod-steel", material="steel", pen=0.3)]
    s = [R("wall-core", B, "body", 1, -0.5, -0.5, 0.3, 1.0, tone=0.6, region="wall-face", material="concrete", hatch="concrete"),
         R("wall-insulation", B, "body", 1, -0.2, -0.5, 0.2, 1.0, tone=0.9, region="wall-face", material="mineral-wool")]
    s += [R("glass-%d" % i, B, "glazing", 2, a, -0.5, b - a, 1.0, tone=0.92, region="canopy-glass", material="glass") for i, (a, b) in enumerate(SHEETS)]
    s += [R("beam", B, "body", 3, 0, -0.041, 5.5, 0.082, tone=0.4, region="canopy-steel", material="steel")]
    if sc == 100:
        s += [C("strut-tube", B, "body", 5, 0, 0, 4.0, 0, 0.03, tone=0.45, region="rod-steel", material="steel"),
              R("clevis-beam", B, "body", 5.2, 3.94, -0.04, 0.12, 0.08, tone=0.3, region="rod-steel", material="steel", pen=0.1)]
        return s
    s[0:2] = [R("wall-core", B, "body", 1, -0.5, -0.5, 0.3, 1.0, tone=0.6, region="wall-face", material="concrete", hatch="concrete"),
              R("wall-insulation", B, "body", 1, -0.2, -0.5, 0.2, 1.0, tone=0.9, region="wall-face", material="mineral-wool"),
              R("wall-render", B, "body", 1.05, -0.02, -0.5, 0.02, 1.0, tone=0.55, region="wall-face", material="render")]
    s += [line("beam-web", B, "detail", 3.1, [(0.02, 0), (5.5, 0)], region="canopy-steel"),
          C("strut-tube", B, "body", 5, 0, 0, 4.0, 0, 0.03, tone=0.45, region="rod-steel", material="steel"),
          line("strut-axis", B, "detail", 5.1, [(0.1, 0), (3.9, 0)], region="rod-steel"),
          R("clevis-beam", B, "body", 5.2, 3.94, -0.04, 0.12, 0.08, tone=0.3, region="rod-steel", material="steel", r=0.01),
          R("clevis-wall", B, "body", 5.2, -0.04, -0.04, 0.08, 0.08, tone=0.3, region="rod-steel", material="steel", r=0.01),
          line("pin-beam", B, "detail", 5.3, [(4.0, -0.04), (4.0, 0.04)], tone=0.1, region="rod-steel"),
          line("pin-wall", B, "detail", 5.3, [(0, -0.04), (0, 0.04)], tone=0.1, region="rod-steel")]
    return s


LEVEL = (" The canopy hangs at door-head height, beam underside 2.25 m above the ground, so the wall rises to 5.35 m and the strut climbs 2.9 m to its anchor 5.2 m above the ground; the front shows all of it from the ground up.",
         " Das Vordach hängt in Türsturzhöhe, Trägerunterkante 2,25 m über dem Boden, daher steigt die Wand auf 5,35 m und der Stab klettert 2,9 m zu seinem Anschluss 5,2 m über dem Boden; die Vorderansicht zeigt alles vom Boden an.")
EN = {
    500: ("Front and plan: the front is the wall seen from the open side with the canopy as a T (beam under the roof strip) and the strut as a line; the plan is the wall top, the glass roof, the beam seen through it and the strut line, from the wall to the beam.",
          "Vorder- und Draufsicht: Die Vorderansicht ist die Wand von der offenen Seite mit dem Vordach als T (Träger unter dem Dachstreifen) und dem Stab als Linie; die Draufsicht ist die Wandkrone, das Glasdach, der hindurch sichtbare Träger und die Stablinie von der Wand zum Träger."),
    200: ("Front and plan: the front shows the wall face, the beam end (IPE 160), the roof as an edge-on glass strip and the strut as a line; the plan shows the wall (concrete and insulation), the glass roof, the beam seen through it and the strut line.",
          "Vorder- und Draufsicht: Die Vorderansicht zeigt die Wandfläche, das Trägerende (IPE 160), das Dach als Glasstreifen in Kantenansicht und den Stab als Linie; die Draufsicht zeigt die Wand (Beton und Dämmung), das Glasdach, den hindurch sichtbaren Träger und die Stablinie."),
    100: ("Front and plan: the front adds the wall pads, the end of the beam, the 60 mm tube strut and both clevises; the plan draws the wall top with concrete stipple, the four glass sheets with their joints, the beam under them and the strut tube above them with the beam clevis.",
          "Vorder- und Draufsicht: Die Vorderansicht ergänzt die Wandplatten, das Trägerende, den Rohrstab 60 mm und beide Gabeln; die Draufsicht zeichnet die Wandkrone mit Betonpunktraster, die vier Glasscheiben mit ihren Fugen, den Träger darunter und das Stabrohr darüber mit der Trägergabel."),
    50: ("Front and plan: the front shows the render face, the anchor pads with steel plates, the beam end with flange lines, the glass edge, the 60 mm tube with axis line, both clevises and pins; the plan shows the wall top (concrete stipple, insulation, render), the four glass sheets, the beam with its web line seen through the glass, the tube strut with axis, clevises and the pin lines (pin axes run across the strut).",
         "Vorder- und Draufsicht: Die Vorderansicht zeigt die Putzfläche, die Ankerplatten mit Stahlplatten, das Trägerende mit Flanschlinien, die Glaskante, das Rohr 60 mm mit Achslinie sowie beide Gabeln und Bolzen; die Draufsicht zeigt die Wandkrone (Betonpunktraster, Dämmung, Putz), die vier Glasscheiben, den Träger mit seiner durch das Glas sichtbaren Steglinie, den Rohrstab mit Achse, die Gabeln und die Bolzenlinien (die Bolzenachsen verlaufen quer zum Stab)."),
}
raise_canopy(doc["asset"])
for sc in (500, 200, 100, 50):
    install(A, sc, lifted(front(sc)), plan(sc), doc, *(EN[sc][0] + LEVEL[0], EN[sc][1] + LEVEL[1]))
finish(A, doc)
print("ok", A)
