import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

GA, RU = "steel-galvanised", "rubber"
A = lib.Asset("playground-swing", GA, 0.4, "frame-galv")
ZS = (("l", 0.56), ("r", -0.56))
ang = math.radians(60.5)
legs = (("left", [0, 0], [math.cos(ang), math.sin(ang)]), ("right", [2.6, 0], [-math.cos(ang), math.sin(ang)]))
for side, z in ZS:
    for nm, o, u in legs:
        p0 = [o[0] + u[0] * 0.017, o[1] + u[1] * 0.017, z]
        p1 = [o[0] + u[0] * 2.64, o[1] + u[1] * 2.64, z]
        A.loft(f"leg-{nm}-{side}", f"leg-{nm}", [dict(c=p0, rx=0.03, ry=0.03), dict(c=p1, rx=0.03, ry=0.03)])
        c0 = [o[0] + u[0] * 2.3, o[1] + u[1] * 2.3, z]
        A.rev(f"leg-{nm}-clamp-{side}", f"leg-{nm}", c0, [[0, 0], [0, 0.04], [0.1, 0.04], [0.1, 0]], local_axis=[1, 0, 0], tone=0.3, role="detail", maxDenominator=100)
        A.box(f"foot-{nm}-{side}", "ground", [o[0], 0.006, z * 0.982], [0.18, 0.012, 0.1], corner=0.002, tone=0.25)
    A.cap(f"brace-{side}", "brace", [0.34, 0.6, z], [2.26, 0.6, z], 0.02, tone=0.45)
    for k, x in enumerate((0.345, 2.255)):
        A.box(f"brace-clamp-{side}{k + 1}", "brace", [x, 0.6, z], [0.05, 0.062, 0.064], corner=0.005, tone=0.3, role="detail", maxDenominator=100)

A.rev("beam", "beam", [1.3, 2.3, -0.6], [[0, 0], [0, 0.05], [1.2, 0.05], [1.2, 0]], axis="z", tone=0.38)
for side, z in (("l", 0.23), ("r", -0.23)):
    A.box(f"beam-hanger-{side}", f"chain-{'left' if side == 'l' else 'right'}", [1.3, 2.225, z], [0.03, 0.05, 0.012], tone=0.2, role="detail", maxDenominator=100, region="chain-steel")
    A.cap(f"chain-{side}", f"chain-{'left' if side == 'l' else 'right'}", [1.3, 2.3, z], [1.3, 0.4, z], 0.0035, tone=0.3, region="chain-steel")
    bone = f"chain-{'left' if side == 'l' else 'right'}"
    A.ell(f"chain-{side}-link-round", bone, [1.3, 2.225, z], [0.0075, 0.05, 0.0055], tone=0.2, role="detail", region="chain-steel", maxDenominator=50, repeat={"count": 9, "translation": [0.2, 0, 0]})
    A.ell(f"chain-{side}-link-edge", bone, [1.3, 2.125, z], [0.0035, 0.05, 0.01], tone=0.25, role="detail", region="chain-steel", maxDenominator=50, repeat={"count": 9, "translation": [0.2, 0, 0]})
    A.ell(f"shackle-{side}", "seat", [1.3, 0.375, z], [0.011, 0.027, 0.011], tone=0.2, role="detail", region="chain-steel")
A.box("seat", "seat", [1.3, 0.331, 0], [0.42, 0.058, 0.52], corner=0.025, material=RU, tone=0.25, region="seat-rubber")
A.save([
    ("Two A-frames (legs of 60 mm tube cut flat at the ground, 0.1 m clamps, 0.02 m braces with clamps, 0.18 x 0.1 m foot plates) at z +-0.56 carry a 0.1 m beam along z; legs are lofts so the feet end flat, the beam is a revolve about z.",
     "Zwei A-Rahmen (Beine aus 60 mm Rohr, am Boden flach abgeschnitten, 0,1 m Klemmen, Streben 0,02 m mit Klemmen, Fussplatten 0,18 x 0,1 m) bei z +-0,56 tragen einen 0,1 m Querbalken in z; die Beine sind Lofts, damit die Füsse flach enden, der Balken ist ein Revolve um z."),
    ("Each chain is a thin rod plus two repeated families of elongated links (one broad in z, one broad in x) that touch each other, so the chain reads as the continuous 0.02 m strip of the front view; the swivel eye follows the chain bone, whose stub stays inside the beam in every pose. The seat is a 0.42 x 0.52 m rounded slab (plan and front agree on 0.52 m, the extrusion hint), hung from shackles inside its edge.",
     "Jede Kette ist ein dünner Stab plus zwei wiederholte Familien lang gestreckter, sich berührender Glieder (eines breit in z, eines breit in x), sodass die Kette als durchgehender 0,02 m Streifen der Vorderansicht erscheint; das Wirbelauge folgt dem Kettenknochen, dessen Stummel in jeder Pose im Balken bleibt. Die Sitzfläche ist eine abgerundete Platte 0,42 x 0,52 m (Draufsicht und Vorderansicht stimmen auf 0,52 m überein, der Extrusionshinweis), an Schäkeln innerhalb ihres Randes aufgehängt."),
])
