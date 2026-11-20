"""🏭 Round 3, furniture/bar-stool: builds the candidate asset document (root fixes, 2D of every scale, volumes) into `🗑️generated/r3-bar-stool/🔣️.json` and the volumes source `🗑️generated/r3-bar-stool/volumes.json` from the catalog file as it is now.

Usage: python3 r3-bar-stool-build.py
"""
import copy
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/bar-stool/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r3-bar-stool")
_spec = importlib.util.spec_from_file_location("draw", os.path.join(HERE, "r3-bar-stool-draw.py"))
D = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(D)
M, K = D.M, D.K
T = K.text

SEAT_MOUNT_AREA = 0.21

REP_DECISIONS = {
    "50": {
        "silhouette": T("Stool 8.5 x 17.4 mm, the true views of the 1:50 model: seat cushion with its top round and piping seam, the swivel plate and the pressed seat pan below it, four splayed 25 mm tubes in rubber glides, the footrest ring welded outside the legs at 0.27 m, the bent flat back bar rising behind the cushion into the curved back pad. Front view: the same parts seen from ahead; the cushion, swivel and backrest are revolved and extruded solids of their side profiles so they turn with the swivel. Plan: ring, leg feet and glides outside the cushion disc, the piping circle, the back pad on the rear edge.", "Hocker 8,5 x 17,4 mm, die wahren Ansichten des 1:50-Modells: Sitzpolster mit Rundung und Kedernaht, darunter Drehteller und gepresste Sitzpfanne, vier gespreizte 25-mm-Rohre in Gummigleitern, der außen an die Beine geschweißte Fußring auf 0,27 m, der gebogene Flachstahlbügel hinter dem Polster hinauf ins gebogene Rückenpolster. Vorderansicht: dieselben Teile von vorn; Polster, Drehteller und Lehne sind Rotations- und Extrusionskörper ihrer Seitenprofile und drehen mit. Grundriss: Ring, Beinenden und Gleiter außerhalb der Polsterscheibe, der Kederkreis, das Rückenpolster am hinteren Rand."),
        "retained": T("Cushion 54 mm with 20 mm top round and 6 mm piping, swivel plate 12 mm, seat pan 20 mm rim, tubes 25 mm, glides 30 x 22 mm, back bar 40 x 8 mm, back pad 26 mm.", "Polster 54 mm mit 20-mm-Rundung und 6-mm-Keder, Drehteller 12 mm, Sitzpfanne mit 20-mm-Rand, Rohre 25 mm, Gleiter 30 x 22 mm, Rückenbügel 40 x 8 mm, Rückenpolster 26 mm."),
        "merged": T("Front and rear legs coincide in the side view, the two side legs in the front view; the plywood board lies hidden inside the cushion.", "Vorder- und Hinterbein fallen in der Seitenansicht zusammen, die beiden Seitenbeine in der Vorderansicht; die Sperrholzplatte liegt verdeckt im Polster."),
        "omitted": T("Welds, screws, staples, the ball race groove of the swivel.", "Schweißnähte, Schrauben, Tackerklammern, die Kugelrille des Drehtellers."),
        "thinElements": T("Tubes at their real 25 mm (0.5 mm); in plan the footrest ring is a 0.5 mm pen circle, the real tube; the piping is a 0.13 mm detail stroke.", "Rohre in echten 25 mm (0,5 mm); im Grundriss ist der Fußring ein Kreis mit 0,5-mm-Stift, das echte Rohr; der Keder ist ein Detailstrich von 0,13 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("Cushion and back pad 0.3, glides 0.15, swivel 0.35, pan 0.55, chrome 0.7.", "Polster 0,3, Gleiter 0,15, Drehteller 0,35, Pfanne 0,55, Chrom 0,7."),
        "overlays": T("One shape per part; glides and back bar belong to the chromed-frame region, pan and swivel to the seat mount.", "Eine Form je Teil; Gleiter und Rückenbügel gehören zur Region des Chromgestells, Pfanne und Drehteller zur Sitzaufnahme."),
    },
    "100": {
        "silhouette": T("Stool 4.3 x 8.7 mm, the true views of the 1:100 model: cushion with a plain top round, swivel and pan as two bands, four tube legs to the floor, the footrest ring, the back bar and the curved back pad. Front and plan as at 1:50 without glides and piping.", "Hocker 4,3 x 8,7 mm, die wahren Ansichten des 1:100-Modells: Polster mit einfacher Rundung, Drehteller und Pfanne als zwei Bänder, vier Rohrbeine bis zum Boden, der Fußring, der Rückenbügel und das gebogene Rückenpolster. Vorderansicht und Grundriss wie bei 1:50 ohne Gleiter und Keder."),
        "retained": T("Cushion, swivel 15 mm, pan 17 mm, tubes 25 mm, ring, back bar 15 mm, back pad.", "Polster, Drehteller 15 mm, Pfanne 17 mm, Rohre 25 mm, Ring, Rückenbügel 15 mm, Rückenpolster."),
        "merged": T("Glides merge into the leg ends; the pan rim becomes a solid disc.", "Die Gleiter gehen in den Beinenden auf; der Pfannenrand wird eine volle Scheibe."),
        "omitted": T("Piping, crown of the cushion, welds, screws.", "Keder, Wölbung des Polsters, Schweißnähte, Schrauben."),
        "thinElements": T("Tubes at their real 25 mm (0.25 mm); the back bar drawn at the 15 mm feature.", "Rohre in echten 25 mm (0,25 mm); der Rückenbügel im 15-mm-Merkmal."),
        "hatching": T("None.", "Keine."),
        "styles": T("Cushion and back pad 0.3, swivel 0.35, pan 0.55, chrome 0.7.", "Polster 0,3, Drehteller 0,35, Pfanne 0,55, Chrom 0,7."),
        "overlays": T("One shape per part.", "Eine Form je Teil."),
    },
    "200": {
        "silhouette": T("Stool 2.1 x 4.4 mm: cushion disc on the mount band, three legs and the ring as true 25 mm tubes, the back bar and the back pad. Front view the same; plan: cushion disc, back pad on the rear edge and the ring disc, the leg feet hidden under the ring.", "Hocker 2,1 x 4,4 mm: Polsterscheibe auf dem Aufnahmeband, drei Beine und der Ring als echte 25-mm-Rohre, Rückenbügel und Rückenpolster. Vorderansicht ebenso; Grundriss: Polsterscheibe, Rückenpolster am hinteren Rand und die Ringscheibe, die Beinenden unter dem Ring verborgen."),
        "retained": T("Cushion, mount, legs, ring, back bar, back pad.", "Polster, Aufnahme, Beine, Ring, Rückenbügel, Rückenpolster."),
        "merged": T("Pan and swivel into one 32 mm mount band.", "Pfanne und Drehteller zu einem 32-mm-Aufnahmeband."),
        "omitted": T("Glides, piping, roundings under 30 mm.", "Gleiter, Keder, Rundungen unter 30 mm."),
        "thinElements": T("Legs and ring at their real 25 mm (C13), read as lines through the 0.25 mm outline pen; back bar and pad at the 30 mm feature.", "Beine und Ring in echten 25 mm (C13), durch den 0,25-mm-Umrissstift als Linien lesbar; Rückenbügel und -polster im 30-mm-Merkmal."),
        "hatching": T("None.", "Keine."),
        "styles": T("Cushion and pad 0.3, mount 0.55, back bar 0.7, tubes 0.3.", "Polster 0,3, Aufnahme 0,55, Rückenbügel 0,7, Rohre 0,3."),
        "overlays": T("One shape per part.", "Eine Form je Teil."),
    },
    "500": {
        "silhouette": T("Stool sign 0.85 x 1.74 mm: seat block on the swivel axis, the curved back panel at its rear, three legs as body-pen strokes and the ring as a true 25 mm tube. Front view the same; plan: seat disc, back panel on the rear edge and the ring disc, the leg feet hidden under the ring.", "Hockerzeichen 0,85 x 1,74 mm: Sitzblock auf der Drehachse, die gebogene Rückenplatte an seiner Rückseite, drei Beine als Körperstriche und der Ring als echtes 25-mm-Rohr. Vorderansicht ebenso; Grundriss: Sitzscheibe, Rückenplatte am hinteren Rand und die Ringscheibe, die Beinenden unter dem Ring verborgen."),
        "retained": T("Seat, back, legs, ring.", "Sitz, Lehne, Beine, Ring."),
        "merged": T("Cushion, board, swivel and pan into the seat block; bar and pad into the back panel.", "Polster, Platte, Drehteller und Pfanne zum Sitzblock; Bügel und Polster zur Rückenplatte."),
        "omitted": T("Glides and every detail under 75 mm.", "Gleiter und jedes Detail unter 75 mm."),
        "thinElements": T("Legs as 0.18 mm body-pen strokes (real 25 mm tubes, slender, C24); the ring at its real 25 mm, read as a line through its outline pen.", "Beine als Körperstriche von 0,18 mm (echte 25-mm-Rohre, schlank, C24); der Ring in echten 25 mm, durch seinen Umrissstift als Linie lesbar."),
        "hatching": T("None.", "Keine."),
        "styles": T("Seat and back 0.35, tubes and strokes 0.3.", "Sitz und Lehne 0,35, Rohre und Striche 0,3."),
        "overlays": T("The seat block carries the seat-pad and seat-mount regions.", "Der Sitzblock trägt die Regionen Sitzpolster und Sitzaufnahme."),
    },
}


def roots(a):
    """🌱 Root fixes outside `representations` and `volumes`."""
    frame_kg, frame_y = K.frame_mass_centre()
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    bones["frame"]["mass"]["kg"] = 5.0
    bones["frame"]["mass"]["comY"] = round(frame_y, 2)
    bones["frame"]["title"] = T("Steel frame: four splayed legs, footrest ring and pressed seat pan", "Stahlgestell: vier gespreizte Beine, Fußring und gepresste Sitzpfanne")
    bones["seat"]["title"] = T("Swivel seat: ball-bearing plate and plywood seat board (turns about the vertical seat axis)", "Drehsitz: Kugeldrehteller und Sperrholz-Sitzplatte (dreht um die senkrechte Sitzachse)")
    bones["seat"]["mass"]["comX"] = 0.004
    bones["pad"]["title"] = T("Upholstered seat cushion (foam and cover, sinks under load)", "Gepolstertes Sitzkissen (Schaum und Bezug, sinkt unter Last ein)")
    bones["pad"]["mass"]["comY"] = 0.012
    back = bones["backrest"]
    back["x"], back["y"], back["lengthM"] = -0.004, 0.14, 0.176
    back["title"] = T("Low backrest: bent flat bar screwed under the seat board, curved pad on a 0.30 m radius", "Niedrige Lehne: gebogener Flachstahl unter der Sitzplatte verschraubt, gebogenes Polster auf 0,30 m Radius")
    back["mass"]["comX"], back["mass"]["comY"] = 0.11, 0.045
    a["dimensions"]["massKg"] = round(sum(b.get("mass", {}).get("kg", 0) for b in a["rig"]["bones"]), 2)
    for anchor in a["anchors"]:
        if anchor["id"] in ("foot-xn", "foot-xp"):
            anchor["x"] = K.FOOT_R if anchor["id"] == "foot-xp" else -K.FOOT_R
        if anchor["id"] in ("foot-zn", "foot-zp"):
            anchor["z"] = K.FOOT_R if anchor["id"] == "foot-zp" else -K.FOOT_R
        if anchor["id"] == "ring-front":
            anchor["x"], anchor["y"] = K.RING_R, K.RING_Y + K.TUBE_R
            anchor["title"] = T("Footrest point on top of the ring tube, front", "Fußraststelle oben auf dem Ringrohr, vorn")
        if anchor["id"] == "seat-top":
            anchor["y"] = round(K.SEAT_TOP - K.PAD_JOINT_Y, 4)
    if not any(m["id"] == "rubber" for m in a["materials"]):
        a["materials"].append({"id": "rubber", "title": T("Black rubber glide", "Schwarzer Gummigleiter"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 1200, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"})
    for m in a["materials"]:
        if m["id"] == "birch-plywood":
            m["title"] = T("Birch plywood seat board 12 mm", "Birkensperrholz-Sitzplatte 12 mm")
    regions = a["thermal"]["regions"]
    for r in regions:
        if r["id"] == "seat-plate":
            r["title"] = T("Plywood seat board (inside the upholstery)", "Sperrholz-Sitzplatte (im Polster)")
            r["hidden"] = T("The 12 mm plywood board lies inside the upholstery: foam on top, the leatherette cover wrapped round its edge and stapled underneath, so no view shows it.", "Die 12-mm-Sperrholzplatte liegt im Polster: oben Schaum, der Kunstlederbezug um ihre Kante geschlagen und unten getackert, daher zeigt sie keine Ansicht.")
        if r["id"] == "footrest-ring":
            r["areaM2"] = 0.099
        if r["id"] == "legs":
            r["title"] = T("Chromed legs, glides and back bar", "Verchromte Beine, Gleiter und Rückenbügel")
    if not any(r["id"] == "seat-mount" for r in regions):
        at = next(i for i, r in enumerate(regions) if r["id"] == "seat-plate") + 1
        cap = round(7850 * 450 * K.PAN_T)
        regions.insert(at, {"id": "seat-mount", "title": T("Seat pan and swivel plate", "Sitzpfanne und Drehteller"), "kind": "surface", "model": "passive", "material": "chrome-steel", "areaM2": SEAT_MOUNT_AREA, "characteristicLengthM": 0.32, "geometry": "horizontal-plate-down", "skyViewFactor": 0.2, "layers": [{"id": "mount-steel", "title": T("Pressed steel 2 mm", "Pressstahl 2 mm"), "kind": "metal", "material": "chrome-steel", "thicknessM": K.PAN_T, "resistanceM2KW": round(K.PAN_T / 50, 6), "heatCapacityJM2K": cap, "source": "literature", "reference": "iso-10456-2007"}], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "iso-10456-2007"})
    for r in regions:
        if r["id"] == "seat-mount":
            r["areaM2"] = SEAT_MOUNT_AREA
    for pose in a["poses"]:
        reg = pose["thermal"]["regions"]
        if "seat-mount" not in reg:
            items = list(reg.items())
            at = [k for k, _ in items].index("seat-plate") + 1
            items.insert(at, ("seat-mount", {"exposedFraction": 1, "sunlitFraction": 0}))
            pose["thermal"]["regions"] = dict(items)
    sa = a["thermal"]["assumptions"]
    sa[1] = T("Seat: 1.5 mm PU leatherette (authored-estimate) over 40 mm HR foam (0.045 W/mK) on a 12 mm birch plywood board; the cover is wrapped round the board edge and stapled underneath, so the board is hidden inside the cushion. Below it a 2 mm pressed steel pan and the swivel plate form the seat mount.", "Sitz: 1,5 mm PU-Kunstleder (Schätzwert) auf 40 mm HR-Schaum (0,045 W/mK) über einer 12-mm-Birkensperrholzplatte; der Bezug ist um die Plattenkante geschlagen und unten getackert, die Platte liegt also verdeckt im Polster. Darunter bilden eine gepresste 2-mm-Stahlpfanne und der Drehteller die Sitzaufnahme.")
    st = a["structure"]["assumptions"]
    st[0] = T("The stool is a welded steel frame on four foot contacts, one under the rubber glide of each leg (x or z = +-0.1975 m): four 25 x 1.5 mm tubes welded into the rim of a pressed seat pan and a footrest ring welded to their outside at 0.27 m; the seat (swivel plate and plywood board) turns on the pan about the vertical axis, the foam cushion sits on a prismatic joint and the backrest is screwed under the board.", "Der Hocker ist ein geschweißtes Stahlgestell auf vier Fußkontakten, je einem unter dem Gummigleiter jedes Beins (x oder z = +-0,1975 m): vier Rohre 25 x 1,5 mm, in den Rand einer gepressten Sitzpfanne geschweißt, und ein außen auf 0,27 m angeschweißter Fußring; der Sitz (Drehteller und Sperrholzplatte) dreht auf der Pfanne um die Hochachse, das Schaumpolster sitzt auf einem Schiebegelenk und die Lehne ist unter der Platte verschraubt.")
    st[1] = T("The swivel is a rotation about the vertical axis (axial twist of the vertical seat bone). Cushion, board and swivel are revolved solids and look the same at every azimuth; the backrest is a bent 40 x 8 mm flat bar rising 10 mm behind the cushion into a pad bent on a 0.30 m radius, 0.30 m wide, so it runs round the seat circle as the seat turns.", "Die Drehung ist eine Rotation um die Hochachse (axiale Verdrehung des senkrechten Sitzknochens). Polster, Platte und Drehteller sind Rotationskörper und sehen bei jedem Winkel gleich aus; die Lehne ist ein gebogener Flachstahl 40 x 8 mm, der 10 mm hinter dem Polster in ein auf 0,30 m Radius gebogenes Polster von 0,30 m Breite übergeht und beim Drehen auf dem Sitzkreis läuft.")
    a["provenance"]["notes"] = T("Front elevation 0.425 x 0.87 m (seat top 0.752 m, footrest 0.27 m, foot circle 0.395 m). Construction after typical market swivel bar stools: 25 x 1.5 mm chromed tubes welded into a pressed seat pan, ball-bearing swivel plate, 12 mm plywood board with 40 mm foam and wrapped leatherette, bent flat-bar backrest. The swivel is shown through the backrest running round the seat circle. Masses from tube length x 0.87 kg/m, sheet 2 mm x 7850 kg/m3 and nominal volumes.", "Vorderansicht 0,425 x 0,87 m (Sitzoberkante 0,752 m, Fußring 0,27 m, Fußkreis 0,395 m). Konstruktion nach marktüblichen Drehbarhockern: verchromte Rohre 25 x 1,5 mm in eine gepresste Sitzpfanne geschweißt, Kugeldrehteller, 12-mm-Sperrholzplatte mit 40 mm Schaum und umgeschlagenem Kunstleder, Lehne aus gebogenem Flachstahl. Die Drehung zeigt sich an der Lehne, die auf dem Sitzkreis läuft. Massen aus Rohrlänge x 0,87 kg/m, Blech 2 mm x 7850 kg/m3 und Nennvolumen.")
    a["dimensions"]["widthM"] = 0.425
    a["dimensions"]["depthM"] = 0.425


def symbol(a):
    """🔣 The true-extent pictogram: seat at its real height, backrest above it, legs and ring at their real places."""
    sym = a["symbols"][0]
    g = sym["geometry"]
    seat = {"id": "seat", "bone": "ground", "role": "body", "z": 3, "filled": True, "tone": 0.85, "kind": "rect", "x": -0.18, "y": 0.698, "w": 0.36, "h": 0.054, "r": 0.012}
    def legs(prefix):
        return [{"id": f"leg-{prefix[0]}", "bone": "ground", "role": "body", "z": 2, "filled": True, "tone": 0.85, "kind": "capsule", "x0": -0.146, "y0": 0.68, "x1": -0.1975, "y1": 0.0125, "r0": 0.0125, "r1": 0.0125},
                {"id": f"leg-{prefix[1]}", "bone": "ground", "role": "body", "z": 2, "filled": True, "tone": 0.85, "kind": "capsule", "x0": 0.146, "y0": 0.68, "x1": 0.1975, "y1": 0.0125, "r0": 0.0125, "r1": 0.0125}]
    ring = {"id": "foot-ring", "bone": "ground", "role": "detail", "z": 2.5, "filled": False, "tone": 0.5, "kind": "poly", "points": [[-0.2125, 0.27], [0.2125, 0.27]], "closed": False}
    g["elevation"] = [seat, {"id": "back", "bone": "ground", "role": "body", "z": 3, "filled": True, "tone": 0.85, "kind": "rect", "x": -0.212, "y": 0.78, "w": 0.06, "h": 0.09, "r": 0.012}] + legs(("near", "far")) + [ring]
    g["front"] = [dict(seat), {"id": "back", "bone": "ground", "role": "body", "z": 2.8, "filled": True, "tone": 0.85, "kind": "rect", "x": -0.15, "y": 0.78, "w": 0.3, "h": 0.09, "r": 0.012}] + legs(("left", "right")) + [dict(ring)]
    g["plan"] = [{"id": "plan-ring", "bone": "ground", "role": "detail", "z": 1, "filled": False, "tone": 0.5, "kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.2125, "ry": 0.2125}, {"id": "plan-seat", "bone": "ground", "role": "body", "z": 2, "filled": True, "tone": 0.85, "kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.18, "ry": 0.18}, {"id": "plan-back", "bone": "ground", "role": "body", "z": 3, "filled": True, "tone": 0.85, "kind": "rect", "x": -0.212, "y": -0.15, "w": 0.07, "h": 0.3, "r": 0.012}]
    sym["description"] = T("The bar stool as a simple pictogram in all three views: round seat of 0.36 m diameter at 0.70 to 0.75 m with the low backrest above its rear edge, four splayed legs with a foot ring, leg footprint 0.42 m.", "Der Barhocker als einfaches Piktogramm in allen drei Ansichten: runder Sitz von 0,36 m Durchmesser auf 0,70 bis 0,75 m mit der niedrigen Lehne über seinem hinteren Rand, vier gespreizte Beine mit Fußring, Standfläche 0,42 m.")


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    a = doc["asset"]
    roots(a)
    symbol(a)
    for key, rep in a["representations"].items():
        rep["geometry"] = D.geometry(int(key))
        rep["decisions"] = REP_DECISIONS[key]
    a["volumes"] = M.volumes()
    os.makedirs(OUT, exist_ok=True)
    json.dump(doc, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    json.dump({"volumes": a["volumes"]}, open(os.path.join(OUT, "volumes.json"), "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print(os.path.join(OUT, "🔣️.json"))


if __name__ == "__main__":
    main()
