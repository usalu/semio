import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("sorption-wheel")
GS = dict(material="galvanised-steel", role="body")
CY = 0.31
circle = lambda r, n=64, rev=False: [(r * math.cos(2 * math.pi * k / n), CY + r * math.sin(2 * math.pi * k / n)) for k in range(n)]

V.ext("casing", "casing", "side", circle(0.31), -0.11, 0.11, holes=[circle(0.285)], tone=0.35, region="casing", **GS)
V.ext("stand", "casing", "side", [(-0.12, 0), (0.12, 0), (0.1, 0.03), (-0.1, 0.03)], -0.11, 0.11, tone=0.35, region="casing", **GS)
V.cyl("disc", "rotor", [0, CY, -0.09], [0, CY, 0.09], 0.285, tone=0.6, region="disc", material="silica-gel", role="body")
V.cyl("hub", "rotor", [0, CY, -0.1], [0, CY, 0.1], 0.045, tone=0.4, region="hub", maxDenominator=100, **GS)
V.cyl("shaft", "rotor", [0, CY, -0.11], [0, CY, 0.11], 0.015, tone=0.4, region="hub", maxDenominator=100, **GS)
for side, z in [("front", 0.0925), ("rear", -0.0925)]:
    V.box(f"spoke-{side}", "rotor", [0.165, CY, z], [0.24, 0.008, 0.005], tone=0.2, region="disc", maxDenominator=100, material="silica-gel", role="detail",
          rep=dict(count=8, angle=math.pi / 4, axis=[0, 0, 1], origin=[0, CY, 0]))
arc = [(0.285 * math.cos(math.radians(a)), CY + 0.285 * math.sin(math.radians(a))) for a in [-42 + 84 * k / 14 for k in range(15)]]
sector = [(0.0, CY)] + arc
V.ext("regen-seal-front", "seal", "side", sector, 0.09, 0.1, tone=0.9, region="regen", material="galvanised-steel", role="body")
V.ext("regen-seal-rear", "seal", "side", sector, -0.1, -0.09, tone=0.9, region="regen", material="galvanised-steel", role="body")
V.decide("The casing is a 64-gon ring (outer r 0.31 m, inner 0.285 m, 0.22 m deep) extruded with a hole; the rotor disc is a 0.18 m cylinder that just fits it and turns on the rotor bone together with hub, shaft and eight radial spoke ribs on both faces (one solid per face with a rotational repeat). The honeycomb texture of the elevation is not modelled.",
         "Das Gehaeuse ist ein 64-Eck-Ring (aussen r 0,31 m, innen 0,285 m, 0,22 m tief) als Extrusion mit Loch; die Rotorscheibe ist ein Zylinder von 0,18 m, der genau hineinpasst und sich auf dem Knochen rotor zusammen mit Nabe, Welle und acht radialen Speichenrippen auf beiden Seiten dreht (ein Koerper je Seite mit Rotationswiederholung). Die Wabentextur der Ansicht ist nicht modelliert.")
V.decide("The regeneration sector (84 degrees about +x, bone seal, fixed) is two 10 mm seal plates on the front and rear faces of the wheel instead of a volume overlapping the disc, so the hot zone stays a separate colourable region while the wheel turns beneath it; plates are galvanised steel although the 2D sector is tinted as silica gel.",
         "Der Regenerationssektor (84 Grad um +x, Knochen seal, fest) sind zwei 10-mm-Dichtplatten auf Vorder- und Rueckseite des Rads statt eines Volumens, das die Scheibe ueberlagert, so bleibt die heisse Zone eine eigene faerbbare Region, waehrend das Rad darunter dreht; die Platten sind verzinkter Stahl, obwohl der 2D-Sektor als Silikagel getoent ist.")
V.decide("The stand is 0.22 m deep as in the front view; the plan draws it 0.18 m deep (inside the casing outline), the front view wins because it matches the casing depth.",
         "Der Fuss ist 0,22 m tief wie in der Vorderansicht; der Grundriss zeichnet ihn 0,18 m tief (innerhalb des Gehaeuseumrisses), die Vorderansicht gilt, weil sie der Gehaeusetiefe entspricht.")
V.write(os.path.join(here, "sorption-wheel.json"))
