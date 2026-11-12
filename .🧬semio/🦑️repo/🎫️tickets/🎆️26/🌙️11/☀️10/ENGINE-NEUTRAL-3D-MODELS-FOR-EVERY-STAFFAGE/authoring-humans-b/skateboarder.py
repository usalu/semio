from lib import Builder, auto, ORGANIC
b = Builder("skateboarder")
auto(b, over={"deck": None, "wheel-f": None, "wheel-r": None, "eye": dict(front=["eye-l", "eye-r"]),
    "head": dict(front=["head"], plan=["face"], xmax=0.103, n=9, exp=2.2), "beanie": dict(front=["hair"], plan=["head"], n=7, exp=2.2),
    "hood": dict(front=["hood"], plan=["hood"], n=5, exp=2.2),
    "hoodie": dict(front=["torso"], plan=["trunk-l", "trunk-r"], latscale=0.9, n=9, exp=2.5),
    "upper-arm-l": dict(front=["upper-arm-l"], plan=["upper-arm-l"]), "upper-arm-r": dict(front=["upper-arm-r"], plan=["upper-arm-r"]),
    "shoe-l": dict(front=["shoe-l"], plan=["shoe-heel-l"], exp=2.6), "shoe-r": dict(front=["shoe-r"], plan=["shoe-heel-r"], exp=2.6)})
b.ell("nose", "head", (0.107, 1.405, 0), (0.017, 0.016, 0.011), material="skin", role="detail", tone=0.25, region=b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0, 1.215, 0), (0.09, 0.04, 0.135), src="hoodie", maxden_ids=["hoodie"])
b.ell("pelvis", "pelvis", (0, 0.733, 0), (0.1, 0.058, 0.12), src="thigh-l", maxden_ids=["thigh-l"])
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"shoulder-{s}", f"upper-arm-{s}", (0, 1.221, z * 0.11), (0.05, 0.04, 0.04), src=f"upper-arm-{s}", maxden_ids=[f"upper-arm-{s}"])
    b.ell(f"elbow-{s}", f"forearm-{s}", (0, 0.907, z * 0.11), (0.05, 0.036, 0.036), src=f"forearm-{s}", maxden_ids=[f"forearm-{s}"])
    b.ell(f"fist-{s}", f"hand-{s}", (0.005, 0.634, z * 0.11), (0.03, 0.031, 0.031), src=f"hand-{s}", maxden_ids=[f"hand-{s}"])
    b.ell(f"knee-{s}", f"shank-{s}", (0, 0.25, z * 0.045), (0.05, 0.048, 0.051), src=f"shank-{s}", maxden_ids=[f"shank-{s}"])
b.organic("deck", "board", "deck", front=["deck"], plan=["deck"], ymin=0.085, n=13, exp=6.0, ends=0.9)
for sid, x in (("truck-f", 0.1875), ("truck-r", -0.1875)):
    b.box(sid, "board", (x, 0.06, 0), (0.075, 0.05, 0.18), corner=0.01, src="truck-front", maxden_ids=["truck-front"], mt=b.meta("truck-front", material="polyurethane", role="body", region=None))
for w, x in (("wheel-f", 0.1875), ("wheel-r", -0.1875)):
    b.add(w, w, "revolve", {"profile": [[-0.02, 0.022], [-0.0175, 0.0265], [0.0175, 0.0265], [0.02, 0.022]], "axis": [0, 0, 1], "origin": [0, 0, 0.105]}, b.meta(w), [w], repeat={"count": 2, "translation": [0, 0, -0.21]})
    b.capsule(f"axle-{w[-1]}", w, (x, 0.027, -0.105), (x, 0.027, 0.105), 0.006, mt=b.meta("truck-front", material="polyurethane", role="detail", region=None), maxden_ids=["truck-front"])
b.dec("The deck is a loft along its length that follows the kicktail curve of the side outline (from 28 mm thick at the middle to the tails 0.188 m above the ground), 0.20 m wide and with a superelliptic rounded-rectangle section; the two trucks are rounded hanger boxes with an axle bar each, the wheels revolves about the lateral axis (53 mm diameter, 40 mm wide, edges rounded) with one solid and a repeat for the pair. The side outline merges deck and trucks into one path; the loft is clipped at the deck underside (0.085 m) and the trucks are modelled separately.",
      "Das Deck ist ein Loft über seine Länge, der der Kicktail-Kurve der Seitenkontur folgt (in der Mitte 28 mm dick, die Enden 0,188 m über dem Boden), 0,20 m breit mit abgerundetem superelliptischem Rechteckschnitt; die beiden Achsen sind abgerundete Achsquader mit je einem Achsstab, die Rollen Rotationskörper um die Querachse (53 mm Durchmesser, 40 mm breit, Kanten gerundet), ein Körper mit Wiederholung für das Paar. Die Seitenkontur verschmilzt Deck und Achsen zu einem Pfad; der Loft ist an der Decksunterseite (0,085 m) abgeschnitten und die Achsen sind getrennt modelliert.")
b.dec("In the rest pose the rig hangs the legs straight down through the board (feet 0.27 m below the ground plane), exactly as the 1:50 drawing shows; the 3D body follows that drawing and the poses (cruising, pushing, crouch, ollie) bend the legs onto the board. Torso, hood and beanie are lofts, the hoodie is 90 % of the front width, ball ellipsoids at shoulder, elbow, fist and knee and the hip and yoke ellipsoids close the joints the front view draws as round parts.",
      "In der Ruhehaltung hängt das Rig die Beine gerade durch das Brett nach unten (Füsse 0,27 m unter der Bodenebene), genau wie die Zeichnung 1:50 es zeigt; der 3D-Körper folgt dieser Zeichnung, und die Posen (Cruisen, Anschieben, Hocke, Ollie) biegen die Beine auf das Brett. Rumpf, Kapuze und Mütze sind Lofts, der Pullover ist 90 % der Breite der Vorderansicht, Kugel-Ellipsoide an Schulter, Ellbogen, Faust und Knie sowie Hüft- und Jochellipsoid schliessen die Gelenke, die die Vorderansicht als runde Teile zeichnet.")
b.save()
