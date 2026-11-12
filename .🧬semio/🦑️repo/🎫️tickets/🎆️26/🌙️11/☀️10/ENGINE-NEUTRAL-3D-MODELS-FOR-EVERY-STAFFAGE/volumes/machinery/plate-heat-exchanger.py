import importlib.util, os
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("plate-heat-exchanger")
GS = dict(material="galvanised-steel", role="body")
AL = dict(material="aluminium-plate", role="body")

V.box("frame-base", "frame", [0, 0.01, 0], [0.5, 0.02, 0.5], tone=0.35, region="frame", **GS)
V.ext("frame-top", "frame", "plan", [(-0.25, -0.25), (0.25, -0.25), (0.25, 0.25), (-0.25, 0.25)], 0.58, 0.6,
      holes=[[(-0.2, -0.2), (-0.2, 0.2), (0.2, 0.2), (0.2, -0.2)]], tone=0.35, region="frame", **GS)
for sx, sz, n in [(-1, -1, "rear-left"), (1, -1, "rear-right"), (-1, 1, "front-left"), (1, 1, "front-right")]:
    V.box(f"frame-post-{n}", "frame", [sx * 0.225, 0.3, sz * 0.225], [0.05, 0.56, 0.05], tone=0.35, region="frame", **GS)
V.box("plate-pack", "plates", [0, 0.0515, 0], [0.44, 0.043, 0.4], tone=0.7, region="plates", rep=dict(count=12, step=[0, 0.045, 0]), **AL)
for side, x0, x1, w0, w1 in [("left", -0.31, -0.25, -0.25, -0.22), ("right", 0.25, 0.31, 0.22, 0.25)]:
    V.ext(f"end-wall-{side}", "frame", "front", [(0.02, -0.2), (0.58, -0.2), (0.58, 0.2), (0.02, 0.2)], w0, w1,
          holes=[[(0.41, -0.14), (0.54, -0.14), (0.54, 0.14), (0.41, 0.14)], [(0.07, -0.14), (0.2, -0.14), (0.2, 0.14), (0.07, 0.14)]], tone=0.35, region="frame", **GS)
for side, x0, x1 in [("left", -0.31, -0.25), ("right", 0.25, 0.31)]:
    for lvl, y0, y1 in [("upper", 0.4, 0.55), ("lower", 0.06, 0.21)]:
        V.ext(f"collar-{lvl}-{side}", "ports", "front", [(y0, -0.15), (y1, -0.15), (y1, 0.15), (y0, 0.15)], x0, x1,
              holes=[[(y0 + 0.01, -0.14), (y1 - 0.01, -0.14), (y1 - 0.01, 0.14), (y0 + 0.01, 0.14)]], tone=0.4, region="ports", **GS)
V.box("damper", "damper", [0, 0.607, 0], [0.4, 0.014, 0.4], tone=0.3, region="damper", **GS)
V.cap("damper-hinge", "damper", [-0.2, 0.61, -0.2], [-0.2, 0.61, 0.2], 0.01, tone=0.1, region="damper", maxDenominator=50, material="galvanised-steel", role="detail")
V.decide("The plate pack is 12 stacked slabs with 2 mm gaps rather than one block, so it reads as a pack of plates; the 1:50 drawing shows it as a block.",
         "Das Plattenpaket besteht aus 12 gestapelten Platten mit 2 mm Spalt statt aus einem Block, damit es als Plattenpaket lesbar ist; die 1:50-Zeichnung zeigt es als Block.")
V.decide("The casing is open on the long sides: base plate, two end walls with port openings, top ring with a vent opening under the damper and four square corner posts of 50 mm. The posts fill the strip between plate pack and frame outline seen from the side and the front.",
         "Das Gehaeuse ist an den Laengsseiten offen: Bodenplatte, zwei Stirnwaende mit Anschlussoeffnungen, Deckring mit Lueftungsoeffnung unter der Klappe und vier Eckpfosten 50 mm. Die Pfosten fuellen den Streifen zwischen Plattenpaket und Rahmenumriss in Seiten- und Vorderansicht.")
V.decide("The four port collars are hollow rectangular stubs (10 mm wall, 150 mm by 300 mm duct opening) on the left and right faces; the damper is one plate on its own bone with its hinge barrel on the same bone, so both swing together about the hinge axis.",
         "Die vier Anschlusskragen sind hohle Rechteckstutzen (10 mm Wand, Kanaloeffnung 150 mm mal 300 mm) an der linken und rechten Seite; die Klappe ist eine Platte auf eigenem Knochen mit dem Scharnierrohr auf demselben Knochen, beide schwenken zusammen um die Scharnierachse.")
V.write(os.path.join(here, "plate-heat-exchanger.json"))
