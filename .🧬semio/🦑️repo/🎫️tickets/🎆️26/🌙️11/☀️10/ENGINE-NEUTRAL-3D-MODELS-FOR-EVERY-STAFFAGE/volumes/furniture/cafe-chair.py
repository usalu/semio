import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("cafe-chair")
BE, ST = "beech-timber", "steel-tube"
def fr(bone): return c.fr[bone]["frame"]
def lp(bone, foot, ztop, t):
    f = fr(bone); u = f["u"]; zf = foot[2]
    return [foot[0] + t * u[0], foot[1] + t * u[1], zf + (ztop - zf) * t / 0.42]
LEGS = {"rear": ([0.05, 0, 0.23], 0.18), "front": ([0.42, 0, 0.2395], 0.19)}
for name, (foot, ztop) in LEGS.items():
    bone = "leg-" + name
    for tag, s in (("l", 1), ("r", -1)):
        F = [foot[0], foot[1], foot[2] * s]
        c.cap(f"leg-{name}-tube-{tag}", bone, ST, "body", 0.35, lp(bone, F, ztop * s, 0.0095), lp(bone, F, ztop * s, 0.42), 0.0095, 0.0125, region="steel-tube")
        c.rev(f"leg-foot-{name}-{tag}", bone, ST, "detail", 0.15, [[0, 0.0115], [0.016, 0.0115]], Y, F, region="steel-tube", maxD=50)
        if name == "rear":
            u = fr(bone)["u"]; v = fr(bone)["v"]
            c.ell(f"leg-rear-knuckle-{tag}", bone, ST, "detail", 0.3, lp(bone, F, ztop * s, 0.42), [0.012, 0.014, 0.0125], axes=[u, v, Z], region="steel-tube", maxD=50)
for tag, s in (("l", 1), ("r", -1)):
    a = lp("leg-rear", [0.05, 0, 0.23 * s], 0.18 * s, 0.12 / fr("leg-rear")["u"][1])
    b = lp("leg-front", [0.42, 0, 0.2395 * s], 0.19 * s, 0.12 / fr("leg-front")["u"][1])
    c.cap(f"stretcher-tube-{tag}", "stretcher", ST, "body", 0.35, a, b, 0.007, region="steel-tube")
    for end, p in (("a", a), ("b", b)):
        c.ell(f"stretcher-joint-{end}-{tag}", "stretcher", ST, "detail", 0.25, p, [0.0085, 0.0095, 0.0085], region="steel-tube", maxD=50)
c.box("seat-pad", "seat", BE, "body", 0.62, [0.244, 0.446, 0], [0.428, 0.028, 0.43], corner=0.013, region="seat-beech")
c.box("seat-apron-side", "seat", BE, "body", 0.45, [0.24, 0.416, -0.1725], [0.36, 0.036, 0.015], corner=0.004, region="seat-beech", repeat={"count": 2, "translation": [0, 0, 0.345]})
c.box("seat-apron-end", "seat", BE, "body", 0.45, [0.0675, 0.416, 0], [0.015, 0.036, 0.33], corner=0.004, region="seat-beech", repeat={"count": 2, "translation": [0.345, 0, 0]})
b = fr("back")
c.cap("back-post", "back", BE, "body", 0.42, [0.066, 0.46, -0.19], [0.066 + 0.405 * b["u"][0], 0.46 + 0.405 * b["u"][1], -0.19], 0.013, 0.009, region="back-beech", repeat={"count": 2, "translation": [0, 0, 0.38]})
c.ell("back-finial", "back", BE, "body", 0.38, [0.066 + 0.412 * b["u"][0], 0.46 + 0.412 * b["u"][1], -0.19], [0.012, 0.011, 0.012], axes=[b["u"], b["v"], Z], region="back-beech", repeat={"count": 2, "translation": [0, 0, 0.38]})
def on(bone, u0, u1, v0, v1, w):
    f = fr(bone); uc, vc = (u0 + u1) / 2, (v0 + v1) / 2
    ctr = [f["o"][i] + uc * f["u"][i] + vc * f["v"][i] for i in range(3)]
    return ctr, [u1 - u0, v1 - v0, 2 * w], [f["u"], f["v"], Z]
ctr, size, ax = on("back", 0.215, 0.385, -0.02, 0.008, 0.185)
c.box("back-panel", "back", BE, "body", 0.52, ctr, size, axes=ax, corner=0.009, region="back-beech")
ctr, size, ax = on("back", 0.11, 0.17, -0.017, 0.003, 0.185)
c.box("back-lower-rail", "back", BE, "body", 0.5, ctr, size, axes=ax, corner=0.006, region="back-beech")
c.dec("The four steel legs splay both ways: 2 degrees fore/aft (the elevation shows one line per pair) and 0.05 m inward from foot to knuckle laterally as the front and plan views draw, so each leg is its own capsule tapering from 9.5 mm to 12.5 mm radius.",
      "Die vier Stahlbeine spreizen sich in beide Richtungen: 2 Grad vor/zurueck (die Ansicht zeigt eine Linie je Paar) und 0,05 m seitlich nach innen vom Fuss zum Knie, wie Vorder- und Draufsicht es zeichnen; jedes Bein ist eine eigene Kapsel, die von 9,5 mm auf 12,5 mm Radius zunimmt.")
c.dec("The seat is a 28 mm rounded slab on a hollow four-rail apron instead of a solid block; the legs end in the apron corners and the stretchers connect the rear and front leg centres at 0.12 m height.",
      "Der Sitz ist eine 28 mm starke, gerundete Platte auf einer hohlen Vier-Zargen-Zarge statt eines Vollblocks; die Beine enden in den Zargenecken und die Streben verbinden die Beinmitten hinten und vorn in 0,12 m Hoehe.")
c.dec("The back (5 degrees tilted bone) has two post capsules with finials, a 37 cm panel and a lower rail; the seat pad and the back panel follow the lateral extrusion hints z0/z1 (the live catalog lacks the mandatory sweep and is evaluated as sweep v, which draws them as slivers in the plan).",
      "Die Lehne (um 5 Grad geneigter Knochen) hat zwei Pfostenkapseln mit Endkugeln, eine 37 cm breite Fuellung und eine untere Traverse; Sitzpolster und Lehnenfuellung folgen den seitlichen Extrusionshinweisen z0/z1 (im Katalog fehlt noch das verpflichtende sweep, es wird als sweep v ausgewertet und im Grundriss als schmaler Streifen gezeichnet).")
c.write()
