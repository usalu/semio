"""🌱 Round 3 root members of `furniture/folding-deck-chair` outside `volumes`: rig, anchors, dimensions, poses, clips, thermal and structural data, symbol and the 2D representations, all derived from the corrected chair of `r3_folding_deck_chair_geom.py`."""
import copy

from r3_folding_deck_chair_scales import *

AREA = {R_TIMBER: 0.90, R_BACK: 0.73, R_SEAT: 0.50}
THETA_RECLINED = theta_of_notch(NOTCHES[0])
THETA_HALF = 37.0
FOLDED_TILT = math.degrees(math.asin(2 * CROSS_HALF_V / (BACK_TOP - END_R - FOOT_RAIL_U - CROSS_HALF_U)))
HEADS = (("head-end-left", 1), ("head-end-right", -1))


def text(en, de):
    return {"en": en, "de": de}


def half_folded():
    f = frames(THETA_HALF)
    q = place(*f["back"], STRUT_PIVOT)
    return configuration(THETA_HALF, strut_world=180.0 + math.degrees(math.asin((q[1] - STRUT_H / 2) / STRUT_LEN)))


CONFIG = {
    "unfolded": REST,
    "folded": configuration(0.0, strut_world=180.0 + FOLDED_TILT, tilt=FOLDED_TILT),
    "reclined-seated": configuration(THETA_RECLINED, notch=NOTCHES[0], loaded=True),
    "sling-proof-load": configuration(THETA_REST, notch=NOTCHES[2], loaded=True),
    "half-folded-key": half_folded(),
}


def extent():
    xs, ys = [], []
    for u in (BACK_FOOT + END_R, BACK_TOP - END_R):
        p = place(*REST["back"], u)
        xs += [p[0] - END_R, p[0] + END_R]
        ys += [p[1] - END_R, p[1] + END_R]
    for u in (END_R, SEAT_LEN - END_R):
        p = place(*REST["seat"], u)
        xs += [p[0] - END_R, p[0] + END_R]
        ys += [p[1] - END_R, p[1] + END_R]
    return min(xs), max(xs), max(ys)


X_MIN, X_MAX, Y_MAX = extent()
DIMENSIONS = {"heightM": rnd(Y_MAX, 3), "widthM": rnd(X_MAX - X_MIN, 3), "depthM": 0.59, "massKg": 4.85}


def rig():
    rel = REST_REL
    seat_o = REST["seat"][0]
    def mass(bone, com):
        return {"kg": MASS[bone], "comX": com, "comY": 0.0, "distribution": "uniform", "source": "authored-estimate", "reference": "wood-handbook-2021" if bone in COM else "semio-nwf"}
    def section(h, title):
        return {"areaM2": rnd(2 * RAIL_T * h, 6), "secondMomentM4": float("%.4e" % (2 * RAIL_T * h ** 3 / 12)), "extremeFibreM": h / 2, "material": BE, "title": title}
    def canvas(bone, parent, at, chord, title):
        return {"id": bone, "parent": parent, "title": title, "x": rnd(at[0], 4), "y": rnd(at[1], 5), "angleDeg": rnd(rel[bone], 4), "lengthM": rnd(CHORD[chord], 4), "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180}, "mass": mass(bone, rnd(CHORD[chord] / 2, 4))}
    return {
        "kind": "mechanical",
        "root": "ground",
        "bones": [
            {"id": "ground", "parent": None, "title": text("Ground reference", "Bodenbezug"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.02, "joint": {"type": "fixed"}},
            {"id": "seat", "parent": "ground", "title": text("Seat frame: rear feet, notches, knee rail", "Sitzrahmen: Hinterfüsse, Rasten, Knieleiste"), "x": rnd(seat_o[0]), "y": rnd(seat_o[1]), "angleDeg": rnd(rel["seat"], 4), "lengthM": SEAT_LEN, "joint": {"type": "revolute", "minDeg": -30, "maxDeg": 10}, "mass": mass("seat", COM["seat"]), "section": section(RAIL_H, text("Two beech rails 20 x 40 mm on edge", "Zwei Buchenholme 20 x 40 mm hochkant"))},
            {"id": "back", "parent": "seat", "title": text("Back frame: front feet, pivot, head rail", "Rückenrahmen: Vorderfüsse, Drehpunkt, Kopfleiste"), "x": SEAT_PIVOT, "y": 0, "angleDeg": rnd(rel["back"], 4), "lengthM": BACK_TOP, "joint": {"type": "revolute", "minDeg": -10, "maxDeg": 80}, "mass": mass("back", COM["back"]), "section": section(RAIL_H, text("Two beech rails 20 x 40 mm on edge", "Zwei Buchenholme 20 x 40 mm hochkant"))},
            {"id": "strut", "parent": "back", "title": text("Prop frame with the notch dowel", "Stützrahmen mit Rastenstab"), "x": STRUT_PIVOT, "y": 0, "angleDeg": rnd(rel["strut"], 4), "lengthM": rnd(STRUT_LEN, 4), "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180}, "mass": mass("strut", COM["strut"]), "section": section(STRUT_H, text("Two beech arms 20 x 32 mm on edge", "Zwei Buchenarme 20 x 32 mm hochkant"))},
            canvas("canvas-a", "back", T_LOCAL, 0, text("Upper back canvas (from the head rail)", "Obere Rückenbahn (ab der Kopfleiste)")),
            canvas("canvas-a2", "canvas-a", (CHORD[0], 0.0), 1, text("Lower back canvas (to the sag)", "Untere Rückenbahn (bis zum Durchhang)")),
            canvas("canvas-b", "seat", S_LOCAL, 3, text("Front seat canvas (from the knee rail)", "Vordere Sitzbahn (ab der Knieleiste)")),
            canvas("canvas-b2", "canvas-b", (CHORD[3], 0.0), 2, text("Rear seat canvas (to the sag)", "Hintere Sitzbahn (bis zum Durchhang)")),
        ],
    }


def anchors():
    out = []
    titles = {"foot-rear-left": text("Rear left foot", "Hinterer linker Fuss"), "foot-rear-right": text("Rear right foot", "Hinterer rechter Fuss"), "foot-front-left": text("Front left foot", "Vorderer linker Fuss"), "foot-front-right": text("Front right foot", "Vorderer rechter Fuss")}
    for anchor, bone, side in FEET:
        u, v = foot_anchor(bone)
        z = Z_SEAT if bone == "seat" else Z_BACK
        out.append({"id": anchor, "bone": bone, "x": rnd(u), "y": rnd(v), "kind": "contact", "title": titles[anchor], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": RAIL_T, "frictionCoefficient": 0.5, "z": rnd(side * (z[0] + z[1]) / 2)})
    for anchor, side in HEADS:
        out.append({"id": anchor, "bone": "back", "x": BACK_TOP - END_R, "y": END_R, "kind": "contact", "title": text("Head end of the %s back rail (rests on the ground when folded)" % ("left" if side > 0 else "right"), "Kopfende des %s Rückenholms (liegt zusammengeklappt am Boden)" % ("linken" if side > 0 else "rechten")), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": RAIL_T, "frictionCoefficient": 0.5, "z": rnd(side * (Z_BACK[0] + Z_BACK[1]) / 2)})
    out.append({"id": "sling", "bone": "canvas-a2", "x": rnd(CHORD[1], 4), "y": 0, "kind": "load", "title": text("Sitting point in the sling", "Sitzpunkt im Segeltuch")})
    return out


def symbol(old):
    """🔣 The true-extent chair sign in the frame of the asset: frames as their rest outlines, the sling as its hanging line."""
    def bar(bone, u0, u1, h):
        return pts([place(*REST[bone], u, v) for u, v in ((u0, -h), (u1, -h), (u1, h), (u0, h))], 4)
    def poly(ident, z, points, filled=True, closed=True, role="body", tone=0.85):
        return {"id": ident, "bone": "ground", "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "points": points, "closed": closed}
    def box(ident, z, x, y, w, h, filled=True, role="body", tone=0.85):
        return {"id": ident, "bone": "ground", "role": role, "z": z, "filled": filled, "tone": tone, "kind": "rect", "x": rnd(x, 4), "y": rnd(y, 4), "w": rnd(w, 4), "h": rnd(h, 4)}
    def reach(bone, u0, u1, h, axis):
        vals = [place(*REST[bone], u, v)[axis] for u in (u0, u1) for v in (-h, h)]
        return min(vals), max(vals)
    line = [sling_point(SLING["t"][0] + (SLING["s"][0] - SLING["t"][0]) * i / 8) for i in range(9)]
    rails = (("back", BACK_FOOT, BACK_TOP, 0.02, Z_BACK, 2), ("seat", 0.0, SEAT_LEN, 0.02, Z_SEAT, 2.1), ("strut", -0.016, STRUT_LEN + 0.016, 0.016, Z_STRUT, 2.2))
    cross = (("top-rail", "back", TOP_RAIL_U, Z_BACK[0]), ("front-rail", "seat", FRONT_RAIL_U, Z_SEAT[0]), ("strut-bar", "strut", STRUT_LEN, Z_STRUT[1]))
    elevation = [poly(bone, z, bar(bone, u0, u1, h)) for bone, u0, u1, h, _, z in rails]
    elevation.append(poly("canvas", 1.5, pts(line, 4), filled=False, closed=False, role="detail", tone=0.5))
    front, plan = [], []
    for bone, u0, u1, h, zz, z in rails:
        y0, y1 = reach(bone, u0, u1, h, 1)
        x0, x1 = reach(bone, u0, u1, h, 0)
        for side, name in ((1, "l"), (-1, "r")):
            lo = zz[0] if side > 0 else -zz[1]
            front.append(box("%s-%s" % (bone, name), z, lo, y0, RAIL_T, y1 - y0))
            plan.append(box("%s-%s" % (bone, name), z, x0, lo, x1 - x0, RAIL_T))
    for ident, bone, u, half in cross:
        y0, y1 = reach(bone, u - 0.02, u + 0.02, 0.01, 1)
        x0, x1 = reach(bone, u - 0.02, u + 0.02, 0.01, 0)
        front.append(box(ident, 2.5, -half, y0, 2 * half, y1 - y0))
        plan.append(box(ident, 2.5, x0, -half, x1 - x0, 2 * half))
    ys = [p[1] for p in line]
    front.append(box("canvas", 1.5, -Z_CANVAS, min(ys), 2 * Z_CANVAS, max(ys) - min(ys), filled=False, role="detail", tone=0.5))
    plan.append(box("canvas", 1.5, line[0][0], -Z_CANVAS, line[-1][0] - line[0][0], 2 * Z_CANVAS, filled=False, role="detail", tone=0.5))
    out = copy.deepcopy(old)
    out["description"] = text(
        "The unfolded deck chair in all three views: the back frame from the front feet to the head rail, the seat frame from the rear feet to the knee rail, the prop frame with its dowel in the notch, and the hanging canvas sling.",
        "Der aufgeklappte Liegestuhl in allen drei Ansichten: der Rückenrahmen von den Vorderfüssen zur Kopfleiste, der Sitzrahmen von den Hinterfüssen zur Knieleiste, der Stützrahmen mit seinem Rundstab in der Raste und das hängende Segeltuch.",
    )
    out["geometry"] = {"elevation": elevation, "front": front, "plan": plan}
    out["dimensionalNote"] = text(
        "Model metres at real size in the frame of the asset (x ahead, lateral positive to the left, y up): the side view is seen from the right, the front view from ahead, the top view from above; all three show the default pose. The sling hangs between the head rail at the back and the knee rail at the front; the chair faces ahead (+x).",
        "Modellmeter in wahrer Grösse im Koordinatensystem des Objekts (x nach vorn, seitlich positiv nach links, y nach oben): die Seitenansicht blickt von rechts, die Vorderansicht von vorn, die Draufsicht von oben; alle drei zeigen die Standardhaltung. Das Tuch hängt zwischen der Kopfleiste hinten und der Knieleiste vorn; der Stuhl blickt nach vorn (+x).",
    )
    return out


POSE_TEXT = {
    "unfolded": (text("Unfolded", "Aufgeklappt"), text("The prop dowel sits in the third of four notches: back frame at 51 degrees, knee rail 0.41 m and head rail 0.90 m above the ground, the empty canvas hangs as a deep sling between them; in full sun.", "Der Stützstab liegt in der dritten von vier Rasten: Rückenrahmen bei 51 Grad, Knieleiste 0,41 m und Kopfleiste 0,90 m über dem Boden, das leere Segeltuch hängt als tiefer Sack dazwischen; in voller Sonne.")),
    "folded": (text("Folded flat", "Zusammengeklappt"), text("The prop is lifted out of the notches and the scissor collapses onto the ground: back frame, seat frame and prop lie in one plane 40 mm thick and 1.43 m long, resting on the head ends of the back rails and on the foot rail planted under the front legs; the canvas is stretched between head rail and knee rail.", "Die Stütze ist aus den Rasten gehoben und die Schere fällt auf den Boden zusammen: Rückenrahmen, Sitzrahmen und Stütze liegen in einer Ebene von 40 mm Dicke und 1,43 m Länge und ruhen auf den Kopfenden der Rückenholme und der unter die Vorderbeine gesetzten Fussleiste; das Segeltuch liegt gespannt zwischen Kopf- und Knieleiste.")),
    "reclined-seated": (text("Reclined and seated", "Zurückgelehnt, besetzt"), text("First notch (back frame at 45 degrees, head rail 0.82 m): an adult lies back in the sling, which is pulled into two straight runs from the rails to the sitting point.", "Erste Raste (Rückenrahmen bei 45 Grad, Kopfleiste 0,82 m): eine erwachsene Person liegt im Tuch, das zu zwei geraden Bahnen von den Leisten zum Sitzpunkt gespannt ist.")),
    "sling-proof-load": (text("Seat proof load", "Sitz-Prüflast"), text("Static seat test load 1600 N in the sling of the unfolded chair.", "Statische Sitz-Prüflast 1600 N im Segeltuch des aufgeklappten Stuhls.")),
    "half-folded-key": (text("Half folded (key)", "Halb zusammengeklappt (Schlüssel)"), text("Frame angle 37 degrees, the prop trails on the ground behind.", "Rahmenwinkel 37 Grad, die Stütze schleift hinten am Boden.")),
}

CLIP_TEXT = {
    "fold-chair": (text("The prop is lifted out of its notch, the scissor of back and seat frame sinks to the ground and the sling is pulled flat between the rails (4 s).", "Die Stütze wird aus der Raste gehoben, die Schere aus Rücken- und Sitzrahmen sinkt zu Boden und das Tuch wird zwischen den Leisten flach gezogen (4 s)."), text("Chair is flat", "Stuhl liegt flach")),
    "unfold-chair": (text("The frames are raised from the ground until the prop dowel drops into the third notch.", "Die Rahmen werden vom Boden angehoben, bis der Stützstab in die dritte Raste fällt."), text("Prop drops into the notch", "Stütze fällt in die Raste")),
    "sit-and-recline": (text("An adult sits in the open chair and the prop is moved to the first notch while the sling takes the load.", "Eine erwachsene Person setzt sich in den geöffneten Stuhl, die Stütze wird in die erste Raste versetzt, während das Tuch die Last aufnimmt."), None),
}


def poses(old):
    out = []
    for pose in old:
        new = copy.deepcopy(pose)
        new["title"], new["description"] = POSE_TEXT[pose["id"]]
        new["bones"] = offsets(CONFIG[pose["id"]])
        new["contacts"] = [c for c in pose["contacts"] if not c.startswith("head-end")] + ([a for a, _ in HEADS] if pose["id"] == "folded" else [])
        out.append(new)
    return out


def clips(old):
    out = []
    for clip in old:
        new = copy.deepcopy(clip)
        description, event = CLIP_TEXT[clip["id"]]
        new["description"] = description
        if event:
            new["events"][0]["title"] = event
        new["contacts"] = [span for span in clip["contacts"] if not span["anchor"].startswith("head-end")]
        if clip["id"] == "fold-chair":
            new["contacts"] += [{"anchor": a, "fromS": 3.6, "toS": 4} for a, _ in HEADS]
        if clip["id"] == "unfold-chair":
            new["contacts"] += [{"anchor": a, "fromS": 0, "toS": 0.4} for a, _ in HEADS]
        out.append(new)
    return out


def thermal(old):
    out = copy.deepcopy(old)
    for region in out["regions"]:
        region["areaM2"] = AREA[region["id"]]
        if region["id"] == R_TIMBER:
            region["characteristicLengthM"] = RAIL_H
            layer = region["layers"][0]
            layer["title"] = text("Beech 20 x 40 mm, half thickness", "Buche 20 x 40 mm, halbe Dicke")
            layer["thicknessM"], layer["resistanceM2KW"], layer["heatCapacityJM2K"] = 0.01, 0.055556, 11520
            region["heatCapacityJM2K"] = 11520
        if region["id"] == R_BACK:
            region["characteristicLengthM"] = rnd(LA, 2)
        if region["id"] == R_SEAT:
            region["characteristicLengthM"] = rnd(LB, 2)
    out["assumptions"][1] = text(
        "Beech rails 20 x 40 mm after ISO 10456 / Wood Handbook; half thickness 10 mm is the heat-storing layer. The frame area is the hand sum of the modelled joinery: 7.5 m of rail and cross rail with 0.12 m girth, the prop dowel and the end grain.",
        "Buchenholme 20 x 40 mm nach ISO 10456 / Wood Handbook; die halbe Dicke 10 mm ist die wärmespeichernde Schicht. Die Rahmenfläche ist die Handsumme der modellierten Schreinerarbeit: 7,5 m Holme und Querleisten mit 0,12 m Umfang, der Stützstab und das Hirnholz.",
    )
    return out


def structure(old):
    out = copy.deepcopy(old)
    out["assumptions"] = [
        text(
            "The chair is a scissor of two frames on four feet: the seat frame turns on its rear foot rounds, the back frame turns on the seat frame at the pivot bolts 0.565 m from the rear feet and 0.30 m from its own front feet, which slide on the ground. The prop frame hangs from the back frame 0.45 m above the pivot; its dowel in one of four notches of the seat rails (0.10, 0.145, 0.19, 0.235 m from the rear end) closes the triangle and fixes the frame angle at 66, 70, 74 or 78 degrees. The rig is the open tree ground, seat, back, prop; the notch closure is solved per pose and stored as bone offsets.",
            "Der Stuhl ist eine Schere aus zwei Rahmen auf vier Füssen: der Sitzrahmen dreht auf seinen hinteren Fussrundungen, der Rückenrahmen dreht am Sitzrahmen an den Gelenkschrauben, 0,565 m von den Hinterfüssen und 0,30 m von den eigenen Vorderfüssen, die am Boden gleiten. Der Stützrahmen hängt 0,45 m über dem Drehpunkt am Rückenrahmen; sein Rundstab in einer der vier Rasten der Sitzholme (0,10, 0,145, 0,19, 0,235 m vom hinteren Ende) schliesst das Dreieck und legt den Rahmenwinkel auf 66, 70, 74 oder 78 Grad fest. Das Rig ist der offene Baum Boden, Sitz, Rücken, Stütze; der Rastenschluss wird je Pose gelöst und als Knochen-Offsets gespeichert.",
        ),
        text(
            "The canvas is four inextensible bones (0.482, 0.290, 0.227 and 0.267 m from the head rail to the knee rail): their sum 1.265 m equals the distance of the two rails in the flat-folded chair, so the folded canvas lies taut. At rest their joints lie on the unloaded catenary at equal forward steps; an empty chair in another position hangs them on the catenary that closes the chain, a sitter pulls them into two straight runs that meet at the sitting point (the lower intersection of the two circles about the rails).",
            "Das Segeltuch besteht aus vier undehnbaren Knochen (0,482, 0,290, 0,227 und 0,267 m von der Kopfleiste zur Knieleiste): ihre Summe 1,265 m entspricht dem Abstand der zwei Leisten im flach zusammengeklappten Stuhl, das gefaltete Tuch liegt daher gespannt. In Ruhe liegen ihre Gelenke in gleichen Schritten nach vorn auf der unbelasteten Kettenlinie; ein leerer Stuhl in anderer Stellung hängt sie an die Kettenlinie, die die Kette schliesst, eine sitzende Person zieht sie zu zwei geraden Bahnen, die sich im Sitzpunkt treffen (unterer Schnittpunkt der zwei Kreise um die Leisten).",
        ),
        old["assumptions"][2],
    ]
    return out


SUMMARY = text(
    "A beech deck chair as it is built: a back frame from the front feet to the head rail, a seat frame from the rear feet to the knee rail, pivoted on each other like scissors, a prop frame whose dowel drops into one of four notches, and a striped canvas sling hanging between the two rails. It reclines notch by notch and folds flat with the canvas stretched.",
    "Ein Buchen-Liegestuhl, wie er gebaut wird: ein Rückenrahmen von den Vorderfüssen zur Kopfleiste, ein Sitzrahmen von den Hinterfüssen zur Knieleiste, scherenartig aneinander drehbar, ein Stützrahmen, dessen Rundstab in eine von vier Rasten fällt, und ein gestreiftes Segeltuch, das zwischen den zwei Leisten hängt. Er neigt sich Raste um Raste und klappt mit gespanntem Tuch flach zusammen.",
)

NOTES = text(
    "Side elevation 1.03 x 0.94 m unfolded in the third notch, 0.58 m across the prop arms (0.597 m across the dome nuts); 4.85 kg. Compared with trade data of beech deck chairs (height 0.78 to 1.06 m by notch, front rail 0.33 to 0.44 m, width 0.56 to 0.62 m, length about 1.0 m, 4 to 4.4 kg). Joint geometry: back rails 1.20 m with the pivot 0.30 m above the front feet, seat rails 1.04 m with the pivot 0.565 m from the rear feet, prop arms 0.49 m hinged 0.45 m above the pivot, canvas 0.772 + 0.493 m between the rails. Poses are solved by r3_folding_deck_chair_geom.py (ground contact, notch closure, circle intersection of the canvas) and stored as bone offsets.",
    "Seitenansicht 1,03 x 0,94 m aufgeklappt in der dritten Raste, 0,58 m über die Stützarme (0,597 m über die Hutmuttern); 4,85 kg. Verglichen mit Handelsdaten von Buchen-Liegestühlen (Höhe 0,78 bis 1,06 m je Raste, Vorderleiste 0,33 bis 0,44 m, Breite 0,56 bis 0,62 m, Länge etwa 1,0 m, 4 bis 4,4 kg). Gelenkgeometrie: Rückenholme 1,20 m mit Drehpunkt 0,30 m über den Vorderfüssen, Sitzholme 1,04 m mit Drehpunkt 0,565 m von den Hinterfüssen, Stützarme 0,49 m, 0,45 m über dem Drehpunkt angeschlagen, Segeltuch 0,772 + 0,493 m zwischen den Leisten. Posen werden von r3_folding_deck_chair_geom.py gelöst (Bodenkontakt, Rastenschluss, Kreisschnitt des Tuchs) und als Knochen-Offsets gespeichert.",
)

REP_DECISIONS = {
    "50": {
        "silhouette": text("Chair 20.5 x 18.7 mm: the true side view of the 1:50 body. Back rails and prop arms as stadiums, the seat rail with its four notches, cross rails as rounded sections behind the rails, the planted foot rail, pivot and prop bolts as circles, the sling as a closed 7.5 mm band along the hanging catenary with its laps round the rails. Front and plan are the same side profiles extruded across their true lateral spans, so every part folds with its bone.", "Stuhl 20,5 x 18,7 mm: die wahre Seitenansicht des 1:50-Körpers. Rückenholme und Stützarme als Stadien, der Sitzholm mit seinen vier Rasten, Querleisten als gerundete Schnitte hinter den Holmen, die aufgesetzte Fussleiste, Gelenk- und Stützschrauben als Kreise, das Tuch als geschlossenes 7,5-mm-Band entlang der hängenden Kettenlinie mit seinen Umschlägen um die Leisten. Vorder- und Draufsicht sind dieselben Seitenprofile, über ihre wahren Seitenlagen gezogen, so faltet jedes Teil mit seinem Knochen."),
        "retained": text("Notches, prop with dowel, pivot bolts, cross rails, canvas laps, the nine canvas stripes in front and plan.", "Rasten, Stütze mit Rundstab, Gelenkschrauben, Querleisten, Tuchumschläge, die neun Tuchstreifen in Vorder- und Draufsicht."),
        "merged": text("Left and right rails coincide in the side view.", "Linke und rechte Holme fallen in der Seitenansicht zusammen."),
        "omitted": text("Arris rounding of the rails, tacks, hems, washers.", "Kantenrundung der Holme, Nägel, Säume, Scheiben."),
        "thinElements": text("The 0.7 mm canvas is a 7.5 mm band (0.15 mm on paper).", "Das 0,7-mm-Tuch ist ein 7,5-mm-Band (0,15 mm auf dem Papier)."),
        "hatching": text("None.", "Keine."),
        "styles": text("Rails 0.55, cross rails 0.45, steel 0.2, canvas light 0.85 with stripes 0.5.", "Holme 0,55, Querleisten 0,45, Stahl 0,2, Tuch hell 0,85 mit Streifen 0,5."),
        "overlays": text("One shape per part; the dark stripes lie as texture over the canvas.", "Eine Form je Teil; die dunklen Streifen liegen als Textur über dem Tuch."),
    },
    "100": {
        "silhouette": text("Chair 10.3 x 9.4 mm: the true side view of the 1:100 body. Rails as stadiums, the seat rail with four square notches, sharp cross rail sections, plain pins at both pivots, the sling as a closed 15 mm band of the hanging catenary. Front and plan: the same profiles extruded across their lateral spans.", "Stuhl 10,3 x 9,4 mm: die wahre Seitenansicht des 1:100-Körpers. Holme als Stadien, der Sitzholm mit vier eckigen Rasten, scharfkantige Querleistenschnitte, glatte Bolzen an beiden Gelenken, das Tuch als geschlossenes 15-mm-Band der hängenden Kettenlinie. Vorder- und Draufsicht: dieselben Profile, über ihre Seitenlagen gezogen."),
        "retained": text("Real rail depth 40 mm, notches, prop and dowel, pivots, the canvas stripes in front and plan.", "Echte Holmhöhe 40 mm, Rasten, Stütze mit Rundstab, Gelenke, die Tuchstreifen in Vorder- und Draufsicht."),
        "merged": text("Left and right rails coincide in the side view.", "Linke und rechte Holme fallen in der Seitenansicht zusammen."),
        "omitted": text("Canvas laps, bolt heads and nuts, rounded notch bottoms, arrises.", "Tuchumschläge, Schraubenköpfe und Muttern, runde Rastengründe, gebrochene Kanten."),
        "thinElements": text("The canvas is a 15 mm band (0.15 mm on paper).", "Das Tuch ist ein 15-mm-Band (0,15 mm auf dem Papier)."),
        "hatching": text("None.", "Keine."),
        "styles": text("Rails 0.55, cross rails 0.45, pins 0.2, canvas 0.85 with stripes 0.5.", "Holme 0,55, Querleisten 0,45, Bolzen 0,2, Tuch 0,85 mit Streifen 0,5."),
        "overlays": text("One shape per part; stripes as texture.", "Eine Form je Teil; Streifen als Textur."),
    },
    "200": {
        "silhouette": text("Chair 5.1 x 4.7 mm: three crossing bars (back frame, seat frame, prop) 40 mm deep and the sling as a bent 30 mm band from the head rail through the sag to the knee rail. Front and plan: the bars 30 mm thick side by side, the cross rails and both canvas panels extruded across their widths.", "Stuhl 5,1 x 4,7 mm: drei sich kreuzende Stäbe (Rückenrahmen, Sitzrahmen, Stütze) von 40 mm Höhe und das Tuch als geknicktes 30-mm-Band von der Kopfleiste über den Durchhang zur Knieleiste. Vorder- und Draufsicht: die Stäbe 30 mm dick nebeneinander, die Querleisten und beide Tuchbahnen über ihre Breite gezogen."),
        "retained": text("Scissor, prop, the hanging sling with its sag.", "Schere, Stütze, das hängende Tuch mit seinem Durchhang."),
        "merged": text("Rail thickness 20 mm is drawn 30 mm; the frames move 15 mm inward each.", "Die Holmdicke 20 mm ist 30 mm gezeichnet; die Rahmen rücken je 15 mm nach innen."),
        "omitted": text("Notches, round ends, bolts, stripes.", "Rasten, runde Enden, Schrauben, Streifen."),
        "thinElements": text("The canvas is a 30 mm band (0.15 mm on paper).", "Das Tuch ist ein 30-mm-Band (0,15 mm auf dem Papier)."),
        "hatching": text("None.", "Keine."),
        "styles": text("Timber 0.3; canvas 0.8 and 0.7 from above and ahead.", "Holz 0,3; Tuch 0,8 und 0,7 von oben und vorn."),
        "overlays": text("One shape per bar and canvas panel.", "Eine Form je Stab und Tuchbahn."),
    },
    "500": {
        "silhouette": text("Chair sign 2.1 x 1.9 mm: the X of back and seat frame, the prop and the sling as two short bands. Front and plan: square 40 mm bars side by side, one cross bar per frame and the two canvas panels.", "Stuhlzeichen 2,1 x 1,9 mm: das X aus Rücken- und Sitzrahmen, die Stütze und das Tuch als zwei kurze Bänder. Vorder- und Draufsicht: quadratische 40-mm-Stäbe nebeneinander, ein Querstab je Rahmen und die zwei Tuchbahnen."),
        "retained": text("The crossing frames, the prop and the hanging sling.", "Die gekreuzten Rahmen, die Stütze und das hängende Tuch."),
        "merged": text("Rails are 40 mm square bars; the frames move inward by their extra thickness.", "Holme sind quadratische 40-mm-Stäbe; die Rahmen rücken um ihre Mehrdicke nach innen."),
        "omitted": text("Foot rail, rear rail, notches, bolts, stripes, round ends.", "Fussleiste, hintere Leiste, Rasten, Schrauben, Streifen, runde Enden."),
        "thinElements": text("Rails and canvas are 40 mm bands (0.08 mm on paper), carried by their 0.18 and 0.2 mm outlines.", "Holme und Tuch sind 40-mm-Bänder (0,08 mm auf dem Papier), getragen von ihren 0,18- und 0,2-mm-Umrissen."),
        "hatching": text("None.", "Keine."),
        "styles": text("Timber 0.3; canvas 0.75 and 0.65.", "Holz 0,3; Tuch 0,75 und 0,65."),
        "overlays": text("One shape per bar and canvas panel.", "Eine Form je Stab und Tuchbahn."),
    },
}


def representations(old, scales):
    out = {}
    for key, rep in old.items():
        new = copy.deepcopy(rep)
        sc = scales[key]
        new["geometry"] = {"elevation": sc.elevation, "front": sc.front, "plan": sc.plan}
        new["decisions"] = REP_DECISIONS[key]
        out[key] = new
    return out


def members(asset, scales):
    """🧩 Every member of the asset that round 3 rewrites, keyed by member name."""
    provenance = copy.deepcopy(asset["provenance"])
    provenance["notes"] = NOTES
    return {
        "summary": SUMMARY,
        "provenance": provenance,
        "dimensions": DIMENSIONS,
        "rig": rig(),
        "anchors": anchors(),
        "representations": representations(asset["representations"], scales),
        "symbols": [symbol(asset["symbols"][0])],
        "poses": poses(asset["poses"]),
        "clips": clips(asset["clips"]),
        "thermal": thermal(asset["thermal"]),
        "structure": structure(asset["structure"]),
    }
