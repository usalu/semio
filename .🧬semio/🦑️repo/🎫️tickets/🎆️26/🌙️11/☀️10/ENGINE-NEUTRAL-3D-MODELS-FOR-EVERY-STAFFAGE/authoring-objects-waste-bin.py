import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

HD = "hdpe-plastic"
A = lib.Asset("waste-bin", HD, 0.42, "bin-hdpe")

def rrect(hz, hx, c, cu=0.0):
    return [[-hz + c, cu - hx], [-hz, cu - hx + c], [-hz, cu + hx - c], [-hz + c, cu + hx], [hz - c, cu + hx], [hz, cu + hx - c], [hz, cu - hx + c], [hz - c, cu - hx]]

def rect_ring(sid, bone, oy, y0, y1, hx0, hz0, hx1, wall, c, cu=0.0, **kw):
    s = A.meta(sid, bone, kw)
    body = {"profile": rrect(hz0, hx0, c, cu), "holes": [rrect(hz0 - wall, hx0 - wall, max(c - wall, 0.002), cu)], "plane": "zx", "from": round(y0 - oy, 5), "to": round(y1 - oy, 5)}
    if abs(hx1 - hx0) > 1e-9:
        body["taperTo"] = [1, round(hx1 / hx0, 5)]
    return A.finish(s, "extrude", body)

def hw(y):
    return 0.2 + 0.048 * (y - 0.025) / 0.93

A.box("pedestal", "body", [0, 0.0125, 0], [0.4, 0.025, 0.4], corner=0.01, tone=0.3)
A.box("pedestal-foot", "body", [-0.175, 0.004, 0], [0.05, 0.008, 0.4], corner=0.002, tone=0.15, role="detail", maxDenominator=100, repeat={"count": 2, "translation": [0.35, 0, 0]})
A.box("floor", "body", [0, 0.03, 0], [0.4, 0.01, 0.4], corner=0.005)
rect_ring("shell", "body", 0, 0.025, 0.955, 0.2, 0.2, 0.248, 0.0105, 0.033)
for k, y in enumerate((0.15, 0.45, 0.72)):
    rect_ring(f"band-{k + 1}", "body", 0, y, y + 0.018, hw(y) + 0.002, hw(y) + 0.002, hw(y + 0.018) + 0.002, 0.006, 0.04, region="bin-ribs", tone=0.38, role="detail", maxDenominator=100)
rect_ring("rim", "body", 0, 0.94, 0.98, 0.255, 0.25, 0.255, 0.02, 0.04, tone=0.5)
rect_ring("liner-collar", "body", 0, 0.948, 0.956, 0.235, 0.235, 0.235, 0.015, 0.03, material="ldpe-film", region="liner-film", tone=0.5, role="detail", maxDenominator=100)
for sz in (1, -1):
    A.box(f"hinge-bracket-{'l' if sz > 0 else 'r'}", "body", [-0.255, 0.9575, sz * 0.19], [0.026, 0.065, 0.06], corner=0.004, tone=0.3)

LX, LY = -0.255, 0.98
LID = dict(material=HD, region="lid-hdpe")
outline = [(0, 0), (0.52, 0), (0.52, 0.006)] + lib.bez([0.52, 0.006], [0.52, 0.03], [0.45, 0.04], [0.4, 0.038], 6)[1:] + [(0.12, 0.038)] + lib.bez([0.12, 0.038], [0.05, 0.04], [0, 0.03], [0, 0.006], 6)[1:-1]
A.ext("lid", "lid", [[LX + x, LY + y] for x, y in outline], -0.25, 0.25, tone=0.55, **LID)
rect_ring("lid-skirt", "lid", LY, LY - 0.028, LY + 0.002, 0.26, 0.25, 0.26, 0.006, 0.03, cu=0.26, tone=0.45, **LID)
hd = lib.bez([0.38, 0.038], [0.39, 0.058], [0.47, 0.058], [0.48, 0.038], 8)
A.ext("lid-handle", "lid", [[LX + x, LY + y] for x, y in hd], -0.05, 0.05, tone=0.35, role="detail", **LID)
A.rev("hinge-barrel", "lid", [LX, LY + 0.004, -0.13], [[0, 0], [0, 0.014], [0.26, 0.014], [0.26, 0]], axis="z", tone=0.25, role="detail", **LID)
A.save([
    ("The bin is a hollow tapered shell (rounded-square ring extrusion with taper, 0.40 m at the foot to 0.496 m at the rim, 10.5 mm wall at the base) on a pedestal with a floor plate; the drawn 1:50 shell outline is a slight curve and is approximated by a straight taper. The three reinforcing bands are thin tapered rings 2 mm proud of the shell, shown to 1:100; the film liner is a thin collar ring folded over the inner rim.",
     "Der Behälter ist eine hohle konische Schale (Ringextrusion mit abgerundetem Quadrat und Verjüngung, 0,40 m am Fuss bis 0,496 m am Rand, 10,5 mm Wand am Boden) auf einem Sockel mit Bodenplatte; der gezeichnete Schalenumriss 1:50 ist leicht gekrümmt und wird durch eine gerade Verjüngung angenähert. Die drei Verstärkungsbänder sind dünne konische Ringe, 2 mm vor der Schale, bis 1:100; der Folienbeutel ist ein dünner, über den Innenrand geschlagener Kragenring."),
    ("The lid is an extrusion of the drawn dome outline over the full 0.5 m width with a skirt ring that overlaps the rim by 28 mm, a handle arch and a hinge barrel at the rear edge; it turns about the hinge axis (z) with the lid bone, to 100 degrees in lid-open. The two hinge brackets stand at z +-0.19 as the plan shows (the front view draws them at +-0.187, a different place).",
     "Der Deckel ist eine Extrusion des gezeichneten Wölbungsumrisses über die ganze Breite von 0,5 m mit einem Randring, der den Rand um 28 mm übergreift, einem Griffbogen und einer Scharniertrommel an der Hinterkante; er dreht mit dem Deckelknochen um die Scharnierachse (z), bis 100 Grad in lid-open. Die zwei Scharnierböcke stehen bei z +-0,19 wie die Draufsicht zeigt (die Vorderansicht zeichnet sie an einem anderen Ort)."),
])
