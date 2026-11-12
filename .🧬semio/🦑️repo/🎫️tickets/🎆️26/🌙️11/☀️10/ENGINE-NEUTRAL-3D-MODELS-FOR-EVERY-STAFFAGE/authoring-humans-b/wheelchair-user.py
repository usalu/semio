import math
from lib import Builder, auto, ORGANIC, R
b = Builder("wheelchair-user")
auto(b, over={"cuff-l": None, "cuff-r": None, "hoodie-pocket": None, "beanie-cuff": None, "finger-tips-l": None, "finger-tips-r": None, "eye": dict(front=["eye-l", "eye-r"]),
    "head": dict(front=["head"], plan=["face"], xmax=0.102, n=9, exp=2.2), "beanie": dict(front=["hair"], plan=["head"], n=7, exp=2.2),
    "hood": dict(front=["hood"], plan=["shoulders"], lat=(-0.115, 0.115), n=5, exp=2.2),
    "hoodie": dict(front=["hoodie"], plan=["trunk"], latscale=0.9, n=9, exp=2.5),
    "pelvis-jeans": dict(front=["pelvis"], plan=["waist"], n=4, exp=2.4),
    "sleeve-l": dict(front=["sleeve-l"], plan=["upper-arm-l"], n=6), "sleeve-r": dict(front=["sleeve-r"], plan=["upper-arm-r"], n=6),
    "glove-l": dict(front=["hand-l"], plan=["hand-l"], n=5), "glove-r": dict(front=["hand-r"], plan=["hand-r"], n=5),
    "glove-thumb-l": dict(lat=(0.128, 0.152)), "glove-thumb-r": dict(lat=(-0.152, -0.128)),
    "shoe-rear-l": dict(front=["shoe-l"], plan=["shoe-heel-l"], exp=2.6), "shoe-rear-r": dict(front=["shoe-r"], plan=["shoe-heel-r"], exp=2.6)}, bones=set(ORGANIC))
b.ell("nose", "head", (0.108, 1.29, 0), (0.017, 0.02, 0.011), material="skin", role="detail", tone=0.25, region=b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0, 1.125, 0), (0.1, 0.04, 0.17), src="hoodie", maxden_ids=["hoodie"])
b.box("hoodie-pocket", "torso", (0.13, 0.65, 0), (0.068, 0.016, 0.14), corner=0.005, src="hoodie-pocket", maxden_ids=["hoodie-pocket"])
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"shoulder-{s}", f"arm-{s}", (0, 1.105, z * 0.152), (0.062, 0.045, 0.052), src=f"sleeve-{s}", maxden_ids=[f"sleeve-{s}"])
    b.ell(f"knee-{s}", f"shank-{s}", (0.426, 0.585, z * 0.088), (0.065, 0.065, 0.07), src=f"shank-{s}", maxden_ids=[f"shank-{s}"])
AL = b.meta("frame-seat-rail")
b.box("frame-seat-rail", "frame", (0.173, 0.483, 0.205), (0.518, 0.056, 0.024), corner=0.01, mt=AL, repeat={"count": 2, "translation": [0, 0, -0.41]}, maxden_ids=["frame-seat-rail"])
for sid, z, r in (("frame-back-tube", 0.205, None), ("frame-hanger", 0.205, None), ("frame-lower-front", 0.205, None), ("brake-lever", 0.185, 0.009)):
    b.tube(sid, "frame", sid, z, r=r, repeat={"count": 2, "translation": [0, 0, -2 * z]})
b.tube("frame-push-handle", "frame", "frame-push-handle", 0.205, r=0.021, repeat={"count": 2, "translation": [0, 0, -0.41]})
b.tube("frame-lower-rail", "frame", "frame-lower-rail", 0.205, r=0.021, repeat={"count": 2, "translation": [0, 0, -0.41]})
b.capsule("frame-cross-brace", "frame", (0.2, 0.298, -0.205), (0.2, 0.298, 0.205), 0.0125, mt=b.meta("frame-lower-rail", role="detail"), maxden_ids=["frame-lower-rail"])
b.tube("castor-fork", "frame", "castor-fork", 0.2155, r=0.014, repeat={"count": 2, "translation": [0, 0, -0.431]})
b.box("castor-housing", "frame", (0.46, 0.22, 0.2155), (0.06, 0.06, 0.04), corner=0.008, mt=b.meta("castor-housing"), repeat={"count": 2, "translation": [0, 0, -0.431]}, maxden_ids=["castor-housing"])
b.box("footplate", "frame", (0.62, 0.0875, 0), (0.32, 0.025, 0.34), corner=0.006, maxden_ids=["footplate"])
b.organic("seat-cushion", "frame", "seat-cushion", front=["seat-cushion"], plan=["seat-cushion"], n=9, exp=4.0, ends=0.9)
b.extrude("back-sling", "frame", "back-sling", (-0.2, 0.2), tol=0.002)
MATS = {"rear-rim": "aluminium", "hand-rim": "aluminium"}
def torus(w, sid, bone, cz, pts, repeat=None, src=None, role=None):
    mt = b.meta(src or sid, role=role, material=MATS.get(sid))
    b.add(sid, bone, "revolve", {"profile": [[round(p[0], 4), round(p[1], 4)] for p in pts], "axis": [0, 0, 1], "origin": [0, 0, cz]}, mt, [src or sid], repeat=repeat)
loop = lambda cz, rr, hw, rin, rout: [(-hw, rin), (-hw, rout - 0.007), (-hw + 0.007, rout), (hw - 0.007, rout), (hw, rout - 0.007), (hw, rin), (-hw, rin)]
torus(0, "rear-tyre", "rear-wheel", 0.33, loop(0, 0, 0.0225, 0.272, 0.305), repeat={"count": 2, "translation": [0, 0, -0.66]})
torus(0, "rear-rim", "rear-wheel", 0.33, loop(0, 0, 0.012, 0.258, 0.282), repeat={"count": 2, "translation": [0, 0, -0.66]})
circ = lambda rc, rr: [(round(rr * math.sin(a * math.pi / 4), 4), round(rc + rr * math.cos(a * math.pi / 4), 4)) for a in range(9)]
torus(0, "hand-rim", "rear-wheel", 0.3, circ(0.274, 0.006), repeat={"count": 2, "translation": [0, 0, -0.6]})
b.add("rear-hub", "rear-wheel", "revolve", {"profile": [[-0.03, 0.035], [0.03, 0.035]], "axis": [0, 0, 1], "origin": [0, 0, 0.315]}, b.meta("rear-hub"), ["rear-hub"], repeat={"count": 2, "translation": [0, 0, -0.63]})
for sg, zc in (("l", 0.322), ("r", -0.322)):
    b.capsule(f"spoke-{sg}", "rear-wheel", (0.033, 0.305, zc), (0.27, 0.305, zc), 0.0025, mt=b.meta("spoke-0", material="aluminium"), role="detail", maxden_ids=["spoke-0"], repeat={"count": 12, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / 12, 5), "origin": [0, 0, 0]})
torus(0, "castor-tyre", "castor", 0.2155, loop(0, 0, 0.0175, 0.05, 0.068), repeat={"count": 2, "translation": [0, 0, -0.431]})
b.add("castor-hub", "castor", "revolve", {"profile": [[-0.02, 0.018], [0.02, 0.018]], "axis": [0, 0, 1], "origin": [0, 0, 0.2155]}, b.meta("castor-hub"), ["castor-hub"], repeat={"count": 2, "translation": [0, 0, -0.431]})
b.dec("The wheelchair is built from the 2D parts: the rear wheel is a closed-profile revolve for the tyre (a hollow ring 45 mm wide), one for the rim and one for the 12 mm hand rim, a hub cylinder and 12 spoke capsules (one solid with a 12-fold repeat about the axle) per side, one solid with a repeat for the two sides; the castor is a smaller ring with a hub; the hand-drawn wobbly circles of the side view are replaced by true circles. Frame tubes follow the principal axis of their side outlines at the lateral position of the front or plan view (0.205 m), the footplate, seat rails and castor housings are rounded boxes, the cushion and the sling a loft and an extrusion.",
      "Der Rollstuhl ist aus den 2D-Teilen gebaut: das Hinterrad ist ein Rotationskörper mit geschlossenem Profil für den Reifen (ein hohler Ring, 45 mm breit), einer für die Felge und einer für den 12-mm-Greifreifen, ein Nabenzylinder und 12 Speichenkapseln (ein Körper mit zwölffacher Wiederholung um die Achse) je Seite, je ein Körper mit Wiederholung für die zwei Seiten; die Lenkrolle ist ein kleinerer Ring mit Nabe; die von Hand gezeichneten wackligen Kreise der Seitenansicht sind durch echte Kreise ersetzt. Die Rahmenrohre folgen der Hauptachse ihrer Seitenkonturen an der seitlichen Lage der Vorder- oder Grundrissansicht (0,205 m), Fussbrett, Sitzholme und Rollengehäuse sind abgerundete Quader, Polster und Sitzfläche ein Loft und eine Extrusion.")
b.dec("The seated figure is in the rig's rest pose (thighs horizontal on the cushion, shanks down to the footplate): lofts measured on the side outline with widths from the front or plan view, the hoodie at 90 % of the front width, gloves in leather, shoulder and knee balls, a shoulder yoke and the pelvis loft close the joints; the head outline is clipped at 0.102 m and the nose is its own ellipsoid. The ring profiles use the closed-loop revolve of the kernel, which the staffage validator rejects as volume-profile-self-intersecting until it is fixed; the castor spokes, finger tips, beanie cuff and heel loop are fine detail that the 1:50 drawing shows but that is not modelled.",
      "Die sitzende Figur steht in der Ruhehaltung des Rigs (Oberschenkel waagrecht auf dem Polster, Unterschenkel hinunter zum Fussbrett): Lofts, an der Seitenkontur gemessen, mit Breiten aus Vorder- oder Grundrissansicht, der Pullover mit 90 % der Breite der Vorderansicht, Handschuhe aus Leder, Schulter- und Knieskugeln, ein Schulterjoch und das Beckenloft schliessen die Gelenke; die Kopfkontur ist bei 0,102 m abgeschnitten und die Nase ein eigenes Ellipsoid. Die Ringprofile nutzen den geschlossenen Rotationsring des Kerns, den der Staffage-Validator bis zur Korrektur als volume-profile-self-intersecting ablehnt; die Speichen der Lenkrolle, Fingerspitzen, Mützenaufschlag und Fersenschlaufe sind feine Details, die die Zeichnung 1:50 zeigt, die aber nicht modelliert sind.")
b.save()
