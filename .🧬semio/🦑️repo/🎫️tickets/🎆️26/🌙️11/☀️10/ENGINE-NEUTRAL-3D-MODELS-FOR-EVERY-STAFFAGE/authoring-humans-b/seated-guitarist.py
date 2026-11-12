import math
from lib import Builder, auto, R
b = Builder("seated-guitarist")
auto(b, over={"eye": dict(front=["eye-l", "eye-r"]), "soundhole": None, "bridge": None,
    "head": dict(front=["head"], plan=["face"], xmax=0.108, n=9, exp=2.2), "hair": dict(front=["hair"], plan=["head"], n=6),
    "shirt": dict(front=["torso"], plan=["trunk-l", "trunk-r"], latscale=0.85, n=9, exp=2.5),
    "upper-arm-l": dict(front=["upper-arm-l"], plan=["upper-arm-l"]), "upper-arm-r": dict(front=["upper-arm-r"], plan=["upper-arm-r"]),
    "boot-l": dict(front=["shoe-l"], plan=["shoe-heel-l"], exp=2.6), "boot-r": dict(front=["shoe-r"], plan=["shoe-heel-r"], exp=2.6),
    "thumb-l": dict(lat=(0.21, 0.23), n=3), "thumb-r": dict(lat=(-0.23, -0.21), n=3),
    "guitar-body": None, "guitar-neck": dict(front=["guitar-neck"], plan=["guitar-neck"], n=6, exp=3.0), "guitar-head": dict(front=["guitar-head"], plan=["guitar-head"], n=5, exp=3.0),
    "tuner-0": None, "tuner-1": None, "tuner-2": None, "stool": None},
    bones=set(__import__("lib").ORGANIC) | {"guitar"})
b.ell("nose", "head", (0.113, 1.335, 0), (0.017, 0.017, 0.011), material="skin", role="detail", tone=0.25, region=b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0, 1.17, 0), (0.1, 0.04, 0.2), src="shirt", maxden_ids=["shirt"])
b.ell("pelvis", "pelvis", (0, 0.697, 0), (0.125, 0.06, 0.19), src="thigh-l", maxden_ids=["thigh-l"])
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"shoulder-{s}", f"upper-arm-{s}", (0, 1.165, z * 0.218), (0.054, 0.03, 0.05), src=f"upper-arm-{s}", maxden_ids=[f"upper-arm-{s}"])
    b.ell(f"knee-{s}", f"shank-{s}", (0, 0.22, z * 0.105), (0.07, 0.072, 0.076), src=f"shank-{s}", maxden_ids=[f"shank-{s}"])
b.extrude("guitar-body", "guitar", "guitar-body", (-0.055, 0.055), tol=0.002)
b.blob("soundhole", "guitar", "soundhole", lat=(-0.058, -0.052), maxden_ids=["soundhole"])
pts = [b.loc("guitar", [p[0], p[1], 0]) for p in b.rings("elevation", "bridge")[0]]
us = [p[0] for p in pts]; vs = [p[1] for p in pts]
b.boxl("bridge", "guitar", ((max(us) + min(us)) / 2, (max(vs) + min(vs)) / 2, R(b.wloc("guitar", -0.0545))), (max(us) - min(us), max(vs) - min(vs), 0.011), src="bridge", maxden_ids=["bridge"])
for i, (x, y, z) in enumerate(((0.77, 1.269, -0.03), (0.798, 1.262, 0.0), (0.824, 1.257, 0.03))):
    b.ell(f"tuner-{i}", "guitar", (x, y, z), (0.007, 0.007, 0.01), src=f"tuner-{i}", maxden_ids=[f"tuner-{i}"])
M = b.meta("stool")
b.add("stool-seat", "stool", "revolve", {"profile": [[0.52, 0.198], [0.527, 0.205], [0.553, 0.205], [0.56, 0.198]], "axis": [0, 1, 0], "origin": [0, 0, 0]}, M, ["stool"])
for i, th in enumerate((90, 210, 330)):
    sn, cs = math.sin(math.radians(th - 90)), math.cos(math.radians(th - 90))
    b.capsule(f"stool-leg-{i}", "stool", (0.14 * sn, 0.52, 0.14 * cs), (0.21 * sn, 0.015, 0.21 * cs), 0.022, 0.02, mt=M, maxden_ids=["stool"])
rr = 0.14 + 0.07 * (0.52 - 0.25) / 0.505
ring = [(rr * math.sin(math.radians(t - 90)), 0.25, rr * math.cos(math.radians(t - 90))) for t in (90, 210, 330)]
b.polyline("stool-rung", "stool", ring + [ring[0]], 0.012, mt=M, maxden_ids=["stool"])
b.dec("The guitar is drawn face-on in the side view (and 0.11 m thick in the front view), so the body is an extrusion of its hourglass outline over 0.11 m, the neck and the headstock are lofts (48 and 80 mm thick as the front and plan views draw them), the sound hole a flat ellipsoid and the bridge a thin plate on the face that points to the right of the player, the three tuners ellipsoids at the positions of the side view (the plan draws them in the reverse order; they are paired by their ahead position).",
      "Die Gitarre ist in der Seitenansicht von vorn gezeichnet (und in der Vorderansicht 0,11 m dick), daher ist der Korpus eine Extrusion seiner Sanduhrkontur über 0,11 m, Hals und Kopfplatte sind Lofts (48 und 80 mm dick, wie Vorderansicht und Grundriss sie zeichnen), das Schallloch ein flaches Ellipsoid und der Steg eine dünne Platte auf der Decke, die nach rechts vom Spieler zeigt, die drei Mechaniken Ellipsoide an den Orten der Seitenansicht (der Grundriss zeichnet sie in umgekehrter Reihenfolge; sie werden über ihre Lage nach vorn zugeordnet).")
b.dec("The stool is a revolved seat disc (0.41 m, 40 mm thick) on three splayed round legs at 120 degrees (ring radius 0.14 m at the seat, foot circle 0.21 m: between the plan polylines (0.12 to 0.19 m) and the front outline (legs out to 0.21 m)) joined by a triangular rung of three round bars at 0.25 m. In the rest pose the rig leaves the body standing through the stool, as the 1:50 drawing does; the poses seat the figure. The shirt is 85 % of the front width; ball ellipsoids at the shoulders and knees, a yoke and a hip ellipsoid close the joints.",
      "Der Hocker besteht aus einer rotierten Sitzscheibe (0,41 m, 40 mm dick) auf drei gespreizten runden Beinen im Winkel von 120 Grad (Ring 0,14 m an der Sitzfläche, Fusskreis 0,21 m: zwischen den Grundrisspolylinien (0,12 bis 0,19 m) und der Vorderansichtskontur (Beine bis 0,21 m)), verbunden durch einen Dreiecksteg aus drei runden Stäben auf 0,25 m. In der Ruhehaltung lässt das Rig den Körper durch den Hocker stehen, wie die Zeichnung 1:50 es zeigt; die Posen setzen die Figur hin. Das Hemd ist 85 % der Breite der Vorderansicht; Kugel-Ellipsoide an Schultern und Knien, ein Joch und ein Hüftellipsoid schliessen die Gelenke.")
b.save()
