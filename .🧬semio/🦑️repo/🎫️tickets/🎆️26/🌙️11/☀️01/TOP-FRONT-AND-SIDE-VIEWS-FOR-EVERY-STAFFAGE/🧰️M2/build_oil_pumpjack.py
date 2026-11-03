import sys, copy
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️M2")
from lib import *

A = "oil-pumpjack"
doc = load(A)
CO, PS, CI, MS, ST = "concrete", "painted-steel", "cast-iron", "motor-steel", "painted-steel"
reps = doc["asset"]["representations"]


def elev(scale):
    return {s["id"]: s for s in reps[str(scale)]["geometry"]["elevation"]}


def ext(e, sid, z0, z1, z, suffix="", new=None):
    s = copy.deepcopy(e[sid])
    s["id"] = (new or sid) + suffix
    s["z"] = z
    s["solid"] = {"kind": "extruded", "z0": z0, "z1": z1}
    return s


MOVERS = {
    "beam": [(-0.18, 0.18, "")],
    "horsehead": [(-0.35, 0.35, "")],
    "horsehead-web": [(-0.2, 0.2, "")],
    "centre-bearing": [(-0.3, 0.3, "")],
    "tail-pin": [(-0.3, 0.3, "")],
    "pitman": [(0.5, 0.65, "-l"), (-0.65, -0.5, "-r")],
    "pitman-eye": [(0.5, 0.65, "-l"), (-0.65, -0.5, "-r")],
    "carrier": [(-0.15, 0.15, "")],
    "counterweight": [(0.45, 0.7, "-l"), (-0.7, -0.45, "-r")],
    "crank-arm": [(0.45, 0.7, "-l"), (-0.7, -0.45, "-r")],
    "crank-pin": [(0.45, 0.7, "-l"), (-0.7, -0.45, "-r")],
    "crank-hub": [(-0.8, 0.8, "")],
}
ZF = {"beam": 3.5, "horsehead": 3.45, "horsehead-web": 3.46, "centre-bearing": 3.52, "tail-pin": 3.55, "pitman": 5.4, "pitman-eye": 5.45, "carrier": 3.3,
      "counterweight": 5.7, "crank-arm": 5.75, "crank-pin": 5.8, "crank-hub": 5.85}
ZP = {"beam": 3.5, "horsehead": 3.55, "horsehead-web": 3.6, "centre-bearing": 3.65, "tail-pin": 3.65, "pitman": 3.4, "pitman-eye": 3.45, "carrier": 3.3,
      "counterweight": 3.7, "crank-arm": 3.75, "crank-pin": 3.8, "crank-hub": 3.85}


def movers(scale, view):
    e = elev(scale)
    out = []
    for sid, rows in MOVERS.items():
        if sid in e:
            for z0, z1, suf in rows:
                out.append(ext(e, sid, z0, z1, (ZF if view == "front" else ZP)[sid], suf))
    return out


def front(scale):
    e = elev(scale)
    s = [R("slab", "base", 3, 0.82, -1.2, 0, 2.4, 0.3, "base", CO),
         R("skid", "skid", 3.05, 0.5, -1.1, 0.3, 2.2, 0.25, "base", ST)]
    if scale == 50:
        s += [R("wellhead-base", "base", 3.1, 0.55, -0.17, 0, 0.34, 0.5, "wellhead", ST), R("wellhead-valve", "base", 3.15, 0.7, -0.1, 0.5, 0.2, 0.5, "wellhead", ST),
              R("stuffing-box", "base", 3.2, 0.45, -0.08, 1, 0.16, 0.28, "wellhead", ST), R("flowline", "base", 3.25, 0.55, -0.06, 0.62, 0.12, 0.12, "wellhead", ST)]
    elif scale == 100:
        s += [R("wellhead-base", "base", 3.1, 0.55, -0.17, 0, 0.34, 0.5, "wellhead", ST), R("wellhead-top", "base", 3.15, 0.6, -0.1, 0.5, 0.2, 0.78, "wellhead", ST)]
    else:
        s += [R("wellhead", "base", 3.1, 0.6, -0.15, 0, 0.3, 1.28, "wellhead", ST)]
    s += [R("polished-rod", "base", 3.2, 0.9, -0.025, 1.28, 0.05, 3.12, "steel", ST),
          P("leg-left", "post", 3.3, 0.62, [(0.8, 0.55), (0.95, 0.55), (0.22, 4.15), (0.1, 4.15)], "steel", ST),
          P("leg-right", "post", 3.3, 0.62, [(-0.8, 0.55), (-0.95, 0.55), (-0.22, 4.15), (-0.1, 4.15)], "steel", ST),
          R("brace", "post", 3.35, 0.55, -0.62, 1.6, 1.24, 0.16, "steel", ST)] if "brace" in e else \
        [R("polished-rod", "base", 3.2, 0.9, -0.025, 1.28, 0.05, 3.12, "steel", ST),
         P("leg-left", "post", 3.3, 0.62, [(0.8, 0.55), (0.95, 0.55), (0.22, 4.15), (0.1, 4.15)], "steel", ST),
         P("leg-right", "post", 3.3, 0.62, [(-0.8, 0.55), (-0.95, 0.55), (-0.22, 4.15), (-0.1, 4.15)], "steel", ST)]
    if scale == 50:
        s.append(E("saddle", "post", 3.45, 0.3, 0, 4.28, 0.28, 0.16, "steel", ST))
    s += movers(scale, "front")
    s += [R("gearbox", "gearbox", 3.5, 0.62, -0.6, 0.7, 1.2, 1.3, "gearbox", CI), R("pedestal", "gearbox", 3.5, 0.5, -0.55, 0.55, 1.1, 0.15, "gearbox", CI),
          R("belt-guard", "gearbox", 7.5, 0.45, -0.95, 0.7, 0.35, 0.9, "steel", ST),
          R("motor", "motor", 8, 0.68, -0.4, 0.65, 0.8, 0.7, "motor", MS), R("motor-feet", "motor", 7.9, 0.5, -0.45, 0.55, 0.9, 0.1, "motor", MS),
          R("motor-cowl", "motor", 8.1, 0.5, -0.3, 0.75, 0.6, 0.5, "motor", MS)]
    if scale == 50:
        s += [R("terminal-box", "motor", 8.2, 0.5, -0.15, 1.35, 0.3, 0.2, "motor", MS), R("motor-sheave", "motor", 8.3, 0.3, -0.85, 0.8, 0.15, 0.4, "steel", ST)]
    return s


def plan(scale):
    e = elev(scale)
    s = [R("slab", "base", 3, 0.82, -1.9, -1.2, 7.8, 2.4, "base", CO), R("skid", "skid", 3.05, 0.5, -1.6, -1.0, 7.2, 2.0, "base", ST)]
    if scale == 50:
        s += [E("wellhead-base", "base", 3.1, 0.55, -3, 0, 0.17, 0.17, "wellhead", ST), E("wellhead-valve", "base", 3.15, 0.7, -3, 0, 0.1, 0.1, "wellhead", ST),
              E("stuffing-box", "base", 3.2, 0.45, -3, 0, 0.08, 0.08, "wellhead", ST), R("flowline", "base", 3.1, 0.55, -2.83, -0.06, 0.83, 0.12, "wellhead", ST)]
    elif scale == 100:
        s += [E("wellhead-base", "base", 3.1, 0.55, -3, 0, 0.17, 0.17, "wellhead", ST), E("wellhead-top", "base", 3.15, 0.6, -3, 0, 0.1, 0.1, "wellhead", ST)]
    else:
        s += [E("wellhead", "base", 3.1, 0.6, -3, 0, 0.17, 0.17, "wellhead", ST)]
    s += [E("polished-rod", "base", 3.2, 0.9, -3, 0, 0.04, 0.04, "steel", ST)]
    for nm, x0, y0, x1, y1 in (("leg-fl", -1.5, 0.85, -0.08, 0.2), ("leg-fr", -1.5, -0.85, -0.08, -0.2), ("leg-rl", 1.5, 0.85, 0.08, 0.2), ("leg-rr", 1.5, -0.85, 0.08, -0.2)):
        s.append(C(nm, "post", 3.3, 0.62, x0, y0, x1, y1, 0.08, 0.08, "steel", ST))
    s += [R("gearbox", "gearbox", 3.2, 0.62, 2.0, -0.6, 1.4, 1.2, "gearbox", CI), R("pedestal", "gearbox", 3.15, 0.5, 2.15, -0.55, 1.1, 1.1, "gearbox", CI),
          R("belt-guard", "gearbox", 3.15, 0.45, 3.4, -0.95, 1.05, 0.35, "steel", ST),
          R("motor", "motor", 3.2, 0.68, 4.35, -0.4, 1.1, 0.8, "motor", MS), R("motor-feet", "motor", 3.15, 0.5, 4.4, -0.45, 1.0, 0.9, "motor", MS),
          R("motor-cowl", "motor", 3.2, 0.5, 5.45, -0.3, 0.2, 0.6, "motor", MS)]
    if scale == 50:
        s += [R("terminal-box", "motor", 3.25, 0.5, 4.75, -0.15, 0.3, 0.3, "motor", MS), R("motor-sheave", "motor", 3.3, 0.3, 4.05, -0.85, 0.4, 0.15, "steel", ST)]
    s += movers(scale, "plan")
    return s


TXT = {
    50: ("front: slab, skid, wellhead column, polished rod and the A-frame legs with brace; the beam, horsehead, bearings, both pitmans, both crank arms with counterweights and pins, the hub and the carrier bar are solids extruded across their lateral width from their elevation profiles, so the stroke raises and lowers them as in the side view; motor, belt guard and sheave stand in front. Top: slab, skid, round wellhead parts, four splayed legs, gearbox, motor and the same extruded solids, which show the beam and horsehead shortening as they tilt.",
         "vorn: Platte, Rahmen, Bohrlochkopf, Polierstange und die Beine des A-Bocks mit Strebe; Balancier, Pferdekopf, Lager, beide Schubstangen, beide Kurbelarme mit Gegengewichten und Bolzen, die Nabe und der Gestängeträger sind aus ihren Aufrissprofilen über ihre Querbreite extrudierte Körper, sodass der Hub sie wie in der Seitenansicht hebt und senkt; Motor, Riemenschutz und Scheibe stehen davor. Oben: Platte, Rahmen, runde Bohrlochkopfteile, vier gespreizte Beine, Getriebe, Motor und dieselben extrudierten Körper, an denen sich Balancier und Pferdekopf beim Kippen verkürzen."),
    100: ("as at 1:50 with merged wellhead parts and without the saddle, web, bearings and lamps; the moving parts are the same extruded solids.",
          "wie bei 1:50 mit zusammengefassten Bohrlochkopfteilen und ohne Sattel, Steg und Lager; die bewegten Teile sind dieselben extrudierten Körper."),
    200: ("main masses: slab, skid, one wellhead block, rod, legs, gearbox, motor and the extruded beam, horsehead, pitmans, crank arms with counterweights and carrier.",
          "Hauptmassen: Platte, Rahmen, ein Bohrlochkopfblock, Stange, Beine, Getriebe, Motor und der extrudierte Balancier, Pferdekopf, Schubstangen, Kurbelarme mit Gegengewichten und Gestängeträger."),
    500: ("composed silhouette: the same masses as 1:200 with the thinner polished rod; the moving solids keep their elevation profiles.",
          "zusammengesetzte Silhouette: dieselben Massen wie bei 1:200 mit der dünneren Polierstange; die bewegten Körper behalten ihre Aufrissprofile."),
}
for sc in (50, 100, 200, 500):
    put(doc, sc, front(sc), plan(sc), *TXT[sc])
save(A, doc)
print("ok")
