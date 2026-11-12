import sys, os, math
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("office-swivel-chair")
PL, FB, SP, RB = "pa-plastic", "fabric-upholstery", "steel-painted", "rubber"
R = 0.3
def bez(P, t):
    a, b, cc, d = P
    k = [(1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t * t, t ** 3]
    return [sum(k[i] * q[j] for i, q in enumerate(P)) for j in range(2)]
def at_u(P, u):
    lo, hi = 0.0, 1.0
    inc = P[3][0] > P[0][0]
    for _ in range(60):
        m = (lo + hi) / 2
        if (bez(P, m)[0] < u) == inc: lo = m
        else: hi = m
    return bez(P, (lo + hi) / 2)[1]
c.rev("hub", "base", PL, "body", 0.3, [[-0.02, 0.0]] + [[p[1], p[0]] for p in [bez([(0, -0.02), (0.03, -0.02), (0.055, -0.012), (0.055, 0.004)], t / 3) for t in range(1, 4)]] + [[0.042, 0.04]], Y, [0, 0.025, 0], region="chair-plastic")
ARMS = [("front", 0), ("side-a", 72), ("side-b", -72), ("back-a", 144), ("back-b", -144)]
for name, deg in ARMS:
    th = math.radians(deg); d = [math.cos(th), 0, math.sin(th)]; p = [-math.sin(th), 0, math.cos(th)]
    ctr = [R * d[0], 0.025, R * d[2]]
    pt = lambda s, y: [s * d[0], y, s * d[2]]
    c.loft(f"arm-{name}", f"arm-{name}", PL, "body", 0.3, [(pt(0.03, 0.043), 0.02, 0.016, 2.5), (pt(0.3, 0.033), 0.016, 0.014, 2.5)], p, Y, region="chair-plastic")
    c.rev(f"arm-{name}-yoke", f"arm-{name}", PL, "body", 0.28, [[0, 0.022], [0.012, 0.0205], [0.02, 0.014], [0.0255, 0.0]], Y, ctr, region="chair-plastic", maxD=50)
    c.rev(f"arm-{name}-stem", f"arm-{name}", SP, "detail", 0.2, [[0, 0.006], [0.012, 0.006]], Y, [ctr[0], 0.045, ctr[2]], region="chair-steel", maxD=50)
    wheel = [[-0.0125, 0.0187], [-0.0105, 0.0232], [-0.006, 0.025], [0.006, 0.025], [0.0105, 0.0232], [0.0125, 0.0187]]
    c.rev(f"castor-{name}-wheel", f"castor-{name}", RB, "body", 0.2, wheel, Z, [ctr[0], 0.025, ctr[2] - 0.0],  region="castor-rubber")
c.rev("column-sleeve", "column-sleeve", SP, "body", 0.25, [[0.02, 0.034], [0.30, 0.026]], Y, [0, 0.025, 0], region="chair-steel")
c.rev("column-collar", "column-sleeve", SP, "detail", 0.15, [[0.285, 0.034], [0.305, 0.034]], Y, [0, 0.025, 0], region="chair-steel", maxD=50)
c.cap("column", "column", SP, "body", 0.7, [0, 0.225, 0], [0, 0.455, 0], 0.0125, region="chair-steel")
c.rev("column-cone", "column", SP, "body", 0.22, [[0.37, 0.014], [0.435, 0.032]], Y, [0, 0.025, 0], region="chair-steel", maxD=50)
c.box("seat-cushion", "seat", FB, "body", 0.62, [-0.001, 0.454, 0], [0.466, 0.062, 0.48], corner=0.028, region="chair-fabric")
c.box("seat-shell", "seat", PL, "detail", 0.3, [-0.005, 0.4225, 0], [0.39, 0.009, 0.46], corner=0.0035, region="chair-plastic", maxD=50)
c.box("seat-mechanism", "seat", SP, "body", 0.22, [0, 0.404, 0], [0.2, 0.042, 0.2], corner=0.006, region="chair-steel", maxD=100)
c.bw("seat-lever", "seat", SP, "detail", 0.2, (0.09, 0.17), (0.395, 0.405), (-0.14, -0.095), region="chair-steel", maxD=50)
c.ext("armrest-post", "seat", PL, "body", 0.35, [(-0.05, 0.485), (-0.02, 0.485), (-0.025, 0.645), (-0.045, 0.645)], "xy", 0.26, 0.28, region="chair-plastic", repeat={"count": 2, "translation": [0, 0, -0.54]})
c.bw("armrest-bracket", "seat", PL, "body", 0.35, (-0.05, -0.02), (0.43, 0.49), (0.23, 0.26), region="chair-plastic", maxD=100, repeat={"count": 2, "translation": [0, 0, -0.49]})
c.cap("armrest-pad", "seat", PL, "body", 0.4, [-0.19, 0.66, 0.27], [0.115, 0.66, 0.27], 0.019, region="chair-plastic", repeat={"count": 2, "translation": [0, 0, -0.54]})
c.cap("backrest-strut", "backrest", SP, "body", 0.22, [-0.242, 0.445, 0], [-0.256, 0.685, 0], 0.011, 0.009, region="chair-steel")
F = [(0.07, -0.012), (0.2, -0.046), (0.36, -0.030), (0.6, -0.014)]
B = [(0.62, 0.028), (0.40, 0.038), (0.22, 0.044), (0.08, 0.040)]
def station(u):
    vf, vb = at_u(F, u), at_u(B, u)
    return u, (vf + vb) / 2, (vb - vf) / 2
def world(u, vc): return [-0.23 - vc, 0.445 + u, 0]
secs = []
for u in (0.08, 0.15, 0.25, 0.35, 0.45, 0.55, 0.6):
    uu, vc, h = station(u); secs.append((world(uu, vc), 0.22, h, 8))
secs.append((world(0.633, 0.007), 0.2, 0.014, 8))
c.loft("backrest-shell", "backrest", PL, "body", 0.5, secs, Z, X, region="chair-plastic")
psecs = []
for u in (0.12, 0.2, 0.3, 0.4, 0.5, 0.56):
    vf = at_u(F, u); psecs.append((world(u, vf + 0.0005), 0.2, 0.0055, 8))
c.loft("backrest-pad", "backrest", FB, "detail", 0.3, psecs, Z, X, region="chair-fabric", maxD=50)
c.dec("The five-star base is a dome hub (revolve) with five tapered arm lofts at 72 degree steps (front, two side, two back arms on their own bones), each ending in a yoke dome, a steel stem and a 50 mm castor wheel (axis along z, as the front and elevation views draw), so the footprint is the true 0.30 m radius star the plan shows.",
      "Der Fuenfstern besteht aus einer Kuppelnabe (Rotation) mit fuenf verjuengten Arm-Lofts im 72-Grad-Raster (vorderer, zwei seitliche und zwei hintere Arme auf eigenen Knochen), jeder endet in einer Gabelkuppel, einem Stahlzapfen und einem 50-mm-Rollenrad (Achse in z wie in Vorderansicht und Ansicht), sodass der Grundriss der echte Stern mit 0,30 m Radius ist.")
c.dec("The seat cushion is a 62 mm box with 28 mm rounded edges on a thin plastic shell and the mechanism box; armrests (post extrude, bracket and a round 38 mm pad, left and right via repeat) sit at z = +-0.27 outboard of the cushion, so two small plastic brackets (added, not drawn) bridge the 20 mm gap from the seat shell to the posts.",
      "Das Sitzpolster ist eine 62-mm-Box mit 28 mm gerundeten Kanten auf duenner Kunststoffschale und Mechanikkasten; die Armlehnen (Pfosten als Extrusion, Konsole und runde 38-mm-Auflage, links und rechts per repeat) sitzen bei z = +-0,27 ausserhalb des Polsters, zwei kleine Kunststoffkonsolen (ergaenzt, nicht gezeichnet) ueberbruecken deshalb die 20 mm Luecke von der Sitzschale zu den Pfosten.")
c.dec("The backrest shell is a loft of 8-sections with exponent 8 (rounded rectangle 0.44 m wide) following the curved front and back faces of the drawn lumbar curve; the fabric pad is a thin loft standing 5 mm proud on the front face. The 2D plan draws the castor wheels as 50 mm circles whereas the front view gives 25 mm width; the 3D wheels follow the front view and the elevation.",
      "Die Lehnenschale ist ein Loft aus 8 Querschnitten mit Exponent 8 (abgerundetes Rechteck, 0,44 m breit) entlang der gezeichneten gewoelbten Vorder- und Rueckflaeche der Lendenkurve; das Stoffpolster ist ein duennes Loft, das 5 mm vor der Vorderflaeche steht. Der 2D-Grundriss zeichnet die Rollenraeder als 50-mm-Kreise, die Vorderansicht gibt jedoch 25 mm Breite an; die 3D-Raeder folgen Vorderansicht und Ansicht.")
c.write()
