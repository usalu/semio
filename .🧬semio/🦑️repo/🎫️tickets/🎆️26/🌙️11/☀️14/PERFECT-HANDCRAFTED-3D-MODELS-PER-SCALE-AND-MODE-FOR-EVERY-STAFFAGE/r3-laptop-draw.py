"""✏ Round 3, appliances/laptop: the 2D drawings of every authored scale as the true orthographic views of that scale's body (ruling E1), drawn by hand from the same measures as the solids.

Frames: elevation bone-local (base u ahead from the rear edge and v up; lid u along the lid from the hinge axis and v towards its back); front x = world z and y = world height; plan x = world x ahead and y = world z. Every lid shape of the front and the plan is its side profile with an extruded span, so the product projects it with the lid angle of the pose.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_laptop_" + name, os.path.join(HERE, f"r3-laptop-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")
M = _load("model")

UNDERSIDE = {"region": "underside", "material": "alu-anodised"}
FEET = {"region": "underside", "material": "rubber-foot"}
DECK = {"region": "deck", "material": "alu-anodised"}
CHASSIS = {"regions": ["deck", "underside"], "material": "alu-anodised"}
LID = {"region": "lid", "material": "alu-anodised"}
GLASS = {"region": "lid", "material": "display-glass"}
BARREL = {"region": "lid", "material": "keycap-plastic"}
X0 = K.BASE_X0
W = K.HALF_W


def n(value):
    """🔢 A coordinate as written in the file."""
    return round(value + 0.0, 5)


def shape(sid, bone, z, tone, geometry, tags, role="body", filled=True, span=None):
    """🧾 One authored shape; `span` makes it the side profile of a part extruded over that lateral half width."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(tags)
    if span is not None:
        out["solid"] = {"kind": "extruded", "sweep": "w", "z0": -span, "z1": span}
    return out


def rect(x0, y0, x1, y1, r=0.0):
    """▭ A rectangle between two corners, optionally rounded."""
    out = {"kind": "rect", "x": n(min(x0, x1)), "y": n(min(y0, y1)), "w": n(abs(x1 - x0)), "h": n(abs(y1 - y0))}
    if r:
        out["r"] = r
    return out


def poly(points, closed=True):
    """🔷 A polygon or an open polyline."""
    return {"kind": "poly", "points": [[n(x), n(y)] for x, y in points], "closed": closed}


def circle(cx, cy, r):
    """⚪ A circle."""
    return {"kind": "ellipse", "cx": n(cx), "cy": n(cy), "rx": r, "ry": r}


def hull_side(rows):
    """📐 The side outline (u, v) of a hull from its section rows (v, half width, half depth)."""
    front = [(K.CENTRE_U + ru, v) for v, _, ru in rows]
    rear = [(K.CENTRE_U - ru, v) for v, _, ru in rows]
    return front + rear[::-1]


def hull_front(rows, top=None):
    """🗼 The front outline (z, y) of a hull from its section rows, optionally carried up to `top`."""
    right = [(rw, v) for v, rw, _ in rows] + ([(rows[-1][1], top)] if top is not None else [])
    return right + [(-z, y) for z, y in right[::-1]]


def lid_side(rows):
    """📐 The side profile (u, v) of the lid shell from its section rows."""
    top = [(K.LID_CENTRE_U + ru, v) for v, _, ru in rows]
    foot = [(K.LID_CENTRE_U - ru, v) for v, _, ru in rows]
    return top + foot[::-1]


def lid_with_barrel():
    """📐 The 1:100 side profile of the lid with the hinge barrel as its round foot."""
    return lid_side(M.LID_100) + [(-0.0032, -0.0032), (0.0, -K.BARREL_R), (0.0032, -0.0032), (0.0043, K.LID_FACE_V)]


def key_rows():
    """⌨ The six key rows seen from above."""
    half = K.KEY_UNITS * K.KEY_PITCH / 2 - K.KEY_GAP / 2
    return [shape(f"plan-key-row-{k + 1}", "base", 3.2, 0.3, rect(X0 + u0, -half, X0 + u1, half), {}, role="detail") for k, (u0, u1) in enumerate(K.KEY_ROWS)]


def scale50():
    """🖊 1:50."""
    glass = rect(K.GLASS[0], K.GLASS_V[0], K.GLASS[1], K.GLASS_V[1])
    screen = rect(K.SCREEN[0], K.SCREEN_V[0], K.SCREEN[1], K.GLASS_V[1])
    lid = poly(lid_side(M.LID_50))
    barrel = circle(0, 0, K.BARREL_R)
    elevation = [
        shape("foot-front", "base", 3, 0.15, rect(K.FEET_U[1] - K.FOOT_R, 0, K.FEET_U[1] + K.FOOT_R, K.FOOT_H), FEET),
        shape("foot-rear", "base", 3, 0.15, rect(K.FEET_U[0] - K.FOOT_R, 0, K.FEET_U[0] + K.FOOT_R, K.FOOT_H), FEET),
        shape("base-lower", "base", 3, 0.5, poly(hull_side(M.TUB_50)), UNDERSIDE),
        shape("base-upper", "base", 3.1, 0.62, rect(K.LEDGE_U, K.SEAM_V, K.CENTRE_U + M.DECK_RU, K.DECK_V), DECK),
        shape("lid-shell", "lid", 3.4, 0.55, lid, LID),
        shape("hinge", "lid", 3.6, 0.3, barrel, BARREL),
        shape("port-strip", "base", 3.8, 0.1, poly([(0.0238, 0.0069), (0.0468, 0.0069)], closed=False), {}, role="detail", filled=False),
    ]
    front = [
        shape("hinge", "lid", 2.8, 0.3, barrel, BARREL, span=K.BARREL_W),
        shape("lid-shell", "lid", 2.9, 0.55, lid, LID, span=W),
        shape("display", "lid", 2.95, 0.1, glass, GLASS, role="glazing", span=K.GLASS[2]),
        shape("screen", "lid", 2.97, 0.3, screen, GLASS, role="glazing", span=K.SCREEN[2]),
        shape("foot-left", "base", 3, 0.15, rect(K.FEET_W - K.FOOT_R, 0, K.FEET_W + K.FOOT_R, K.FOOT_H), FEET),
        shape("foot-right", "base", 3, 0.15, rect(-K.FEET_W - K.FOOT_R, 0, -K.FEET_W + K.FOOT_R, K.FOOT_H), FEET),
        shape("base", "base", 3, 0.6, poly(hull_front(M.TUB_50, K.DECK_V)), CHASSIS),
    ]
    plan = [
        shape("plan-deck", "base", 3, 0.62, rect(X0, -W, X0 + K.DEPTH, W, 0.009), DECK),
        shape("plan-hinge", "lid", 3.15, 0.3, barrel, BARREL, span=K.BARREL_W),
        *key_rows(),
        shape("plan-trackpad", "base", 3.2, 0.55, rect(X0 + K.PAD[0], -K.PAD[2], X0 + K.PAD[1], K.PAD[2], 0.002), {}, role="detail", filled=False),
        shape("plan-lid-shell", "lid", 3.4, 0.55, lid, LID, span=W),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def scale100():
    """🖊 1:100."""
    lower = [row for row in M.BASE_100 if row[0] < K.SEAM_V] + [(K.SEAM_V, W, K.CENTRE_U)]
    glass = rect(0.012, K.GLASS_V[0], 0.209, 0.0015)
    lid = poly(lid_with_barrel())
    elevation = [
        shape("base-lower", "base", 3, 0.5, poly(hull_side(lower)), UNDERSIDE),
        shape("base-upper", "base", 3.1, 0.62, rect(0, K.SEAM_V, K.DEPTH, K.DECK_V), DECK),
        shape("lid-shell", "lid", 3.4, 0.5, lid, LID),
    ]
    front = [
        shape("lid-shell", "lid", 2.9, 0.5, lid, LID, span=W),
        shape("display", "lid", 2.95, 0.1, glass, GLASS, role="glazing", span=0.152),
        shape("base-lower", "base", 3, 0.5, poly(hull_front(lower)), UNDERSIDE),
        shape("base-upper", "base", 3.1, 0.62, rect(-W, K.SEAM_V, W, K.DECK_V), DECK),
    ]
    plan = [
        shape("plan-deck", "base", 3, 0.62, rect(X0, -W, X0 + K.DEPTH, W, 0.009), DECK),
        shape("plan-keys", "base", 3.2, 0.3, rect(X0 + 0.019, -0.138, X0 + 0.119, 0.138), {}, role="detail"),
        shape("plan-trackpad", "base", 3.2, 0.55, rect(X0 + K.PAD[0], -K.PAD[2], X0 + K.PAD[1], K.PAD[2]), {}, role="detail", filled=False),
        shape("plan-lid-shell", "lid", 3.4, 0.5, lid, LID, span=W),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def scale200():
    """🖊 1:200."""
    lid = rect(K.LID_U0, -0.002, K.LID_U1, K.LID_BACK_V)
    elevation = [
        shape("base", "base", 3, 0.55, poly(hull_side(M.BASE_200)), CHASSIS),
        shape("lid", "lid", 3.4, 0.5, lid, LID),
    ]
    front = [
        shape("lid", "lid", 2.9, 0.5, lid, LID, span=W),
        shape("base", "base", 3, 0.55, poly(hull_front(M.BASE_200)), CHASSIS),
    ]
    plan = [
        shape("plan-base", "base", 3, 0.55, rect(X0, -W, X0 + K.DEPTH, W, 0.009), CHASSIS),
        shape("plan-lid", "lid", 3.4, 0.5, lid, LID, span=W),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def text(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": {
        "silhouette": text(
            "A 0.273 m by 0.221 m side view, 5.5 mm by 4.4 mm on paper: the chassis as a 10.5 mm slab with a boat-shaped bilge on two rubber feet, its keyboard deck ending at a ledge before the rear edge, on which the 9 mm hinge barrel sits, and the lid rising from the barrel, leaning back by 15 degrees. Front view: the 0.313 m wide chassis on its two front feet with the bilge at both sides and the lid behind it with the dark glass and the lit panel on its face. Top view: the chassis with rounded corners, six key rows and the trackpad, and the lid seen from above as a band behind the hinge barrel.",
            "Eine Seitenansicht von 0,273 m mal 0,221 m, auf Papier 5,5 mm mal 4,4 mm: das Chassis als 10,5-mm-Platte mit bootsförmiger Kimm auf zwei Gummifüßen, dessen Tastaturdeck vor der Hinterkante an einem Absatz endet, auf dem die 9-mm-Scharnierwalze sitzt, und der Deckel, der aus der Walze aufsteigt und sich um 15 Grad nach hinten lehnt. Vorderansicht: das 0,313 m breite Chassis auf seinen zwei vorderen Füßen mit der Kimm an beiden Seiten und dahinter der Deckel mit dem dunklen Glas und dem leuchtenden Panel auf seiner Fläche. Draufsicht: das Chassis mit gerundeten Ecken, sechs Tastenreihen und Touchpad sowie der Deckel von oben als Band hinter der Scharnierwalze.",
        ),
        "retained": text(
            "Overall height 0.221 m with the lid open at 105 degrees, chassis depth 0.218 m, deck 12.5 mm above the desk, lid thickness 6.5 mm, the feet, the bilge, the parting line of the two covers, the hinge ledge and barrel, the glass with its lit panel, the key rows and the trackpad.",
            "Gesamthöhe 0,221 m bei 105 Grad geöffnetem Deckel, Chassistiefe 0,218 m, Deck 12,5 mm über dem Tisch, Deckeldicke 6,5 mm, die Füße, die Kimm, die Trennfuge der zwei Schalen, Scharnierabsatz und Walze, das Glas mit seinem leuchtenden Panel, die Tastenreihen und das Touchpad.",
        ),
        "merged": text(
            "The bottom cover and the keyboard deck are two stacked shapes; lid shell and rim are one profile; each key row is one bar; the two ports of the right side are one stroke.",
            "Bodenschale und Tastaturdeck sind zwei gestapelte Formen; Deckelschale und Rand sind ein Profil; jede Tastenreihe ist ein Balken; die zwei Buchsen der rechten Seite sind ein Strich.",
        ),
        "omitted": text(
            "The keys and the trackpad in the side and front views, where they lie below the deck plane; the exhaust grille under the barrel, the speaker fields, the camera and the single key caps. The top view draws the lid in its aluminium tone in every pose: the paint order within one bone is that of the closed lid, whose back hides the display.",
            "Tasten und Touchpad in Seiten- und Vorderansicht, wo sie unter der Deckebene liegen; das Abluftgitter unter der Walze, die Lautsprecherfelder, die Kamera und die einzelnen Tastenkappen. Die Draufsicht zeichnet den Deckel in jeder Pose in seinem Aluminiumton: die Malreihenfolge innerhalb eines Knochens ist die des geschlossenen Deckels, dessen Rücken das Display verdeckt.",
        ),
        "thinElements": text(
            "The 2.1 mm cover glass and the 2.3 mm panel stack are side profiles inside the lid that only the front view projects; the 2 mm feet stay real and rely on the pen weight to reach the feature width.",
            "Das 2,1 mm dicke Deckglas und der 2,3 mm dicke Panelaufbau sind Seitenprofile im Deckel, die nur die Vorderansicht projiziert; die 2 mm hohen Füße bleiben real und erreichen die Mindestbreite durch die Strichstärke.",
        ),
        "hatching": text("None; the key rows are six detail bars.", "Keine; die Tastenreihen sind sechs Detailbalken."),
        "styles": text(
            "Bottom cover 0.5, deck 0.62, lid 0.55, glass 0.1 dark with the lit panel 0.3, barrel 0.3, feet 0.15; body pen 0.18 mm, detail 0.13 mm, glazing 0.18 mm, ground 0.18 mm so the 12.5 mm chassis is not swallowed by the desk line.",
            "Bodenschale 0,5, Deck 0,62, Deckel 0,55, Glas 0,1 dunkel mit dem leuchtenden Panel 0,3, Walze 0,3, Füße 0,15; Körperstift 0,18 mm, Detail 0,13 mm, Glas 0,18 mm, Boden 0,18 mm, damit das 12,5 mm hohe Chassis nicht von der Tischlinie verschluckt wird.",
        ),
        "overlays": text(
            "The keyboard deck, the underside and the lid are separate regions, so the thermal view paints the hot underside and deck apart from the cooler lid.",
            "Tastaturdeck, Unterseite und Deckel sind getrennte Regionen, sodass die Thermalansicht die heiße Unterseite und das Deck vom kühleren Deckel trennt.",
        ),
    },
    "100": {
        "silhouette": text(
            "An L-shaped side sign 2.7 mm by 2.2 mm: the chassis as a hull with its bilge standing on the desk and the lid leaning back from the round hinge foot at its rear edge. Front view: the chassis and the lid behind it with its dark glass. Top view: the chassis with one key field and the trackpad outline, and the lid seen from above as a band.",
            "Ein L-förmiges Seitenzeichen von 2,7 mm mal 2,2 mm: das Chassis als Rumpf mit Kimm, der auf dem Tisch steht, und der Deckel, der sich vom runden Scharnierfuß an dessen Hinterkante nach hinten lehnt. Vorderansicht: das Chassis und dahinter der Deckel mit seinem dunklen Glas. Draufsicht: das Chassis mit einem Tastenfeld und dem Umriss des Touchpads sowie der Deckel von oben als Band.",
        ),
        "retained": text(
            "Overall height 0.221 m, chassis depth 0.218 m, the bilge, the parting line of the two covers, the lid thickness and the round hinge foot; the glass, the key field and the trackpad.",
            "Gesamthöhe 0,221 m, Chassistiefe 0,218 m, die Kimm, die Trennfuge der zwei Schalen, die Deckeldicke und der runde Scharnierfuß; das Glas, das Tastenfeld und das Touchpad.",
        ),
        "merged": text(
            "The feet and their 2 mm air gap join the bottom cover, the hinge barrel joins the lid profile, the lit panel joins the glass, the key rows join one key field.",
            "Die Füße und ihr 2-mm-Luftspalt gehen in der Bodenschale auf, die Scharnierwalze im Deckelprofil, das leuchtende Panel im Glas, die Tastenreihen in einem Tastenfeld.",
        ),
        "omitted": text(
            "The hinge ledge, the keyboard well, ports, grille, speaker fields and camera; the display face in the top view.",
            "Scharnierabsatz, Tastaturmulde, Buchsen, Gitter, Lautsprecherfelder und Kamera; die Displayfläche in der Draufsicht.",
        ),
        "thinElements": text(
            "The 12.5 mm chassis is 0.125 mm of paper; the 0.18 mm pen outline lifts it to the feature width, so the two covers read as one dark block with a lighter deck line.",
            "Das 12,5 mm hohe Chassis sind 0,125 mm Papier; die 0,18-mm-Umrisslinie hebt es auf die Mindestbreite, sodass die zwei Schalen als ein dunkler Block mit hellerer Decklinie lesen.",
        ),
        "hatching": text("None.", "Keine."),
        "styles": text(
            "Bottom cover 0.5, deck 0.62, lid 0.5, glass 0.1, key field 0.3; body pen 0.18 mm, glazing pen 0.18 mm for the display stack, detail pen 0.13 mm for the key field and the trackpad.",
            "Bodenschale 0,5, Deck 0,62, Deckel 0,5, Glas 0,1, Tastenfeld 0,3; Körperstift 0,18 mm, Glasstift 0,18 mm für den Displayaufbau, Detailstift 0,13 mm für Tastenfeld und Touchpad.",
        ),
        "overlays": text(
            "Deck, underside and lid keep their own regions, so the heat of the underside and the keyboard deck reads separately from the lid.",
            "Deck, Unterseite und Deckel behalten ihre eigenen Regionen, sodass die Wärme von Unterseite und Tastaturdeck getrennt vom Deckel lesbar bleibt.",
        ),
    },
    "200": {
        "silhouette": text(
            "Two bars, 1.4 mm by 1.1 mm on paper: the chassis bar and the lid bar leaning back with its foot in the rear edge of the chassis. Front view: the chassis block and the lid plate raised with the lid angle. Top view: the chassis and the lid seen from above behind its rear edge.",
            "Zwei Balken, auf Papier 1,4 mm mal 1,1 mm: der Chassisbalken und der Deckelbalken, der sich mit seinem Fuß in der Hinterkante des Chassis nach hinten lehnt. Vorderansicht: der Chassisblock und die mit dem Deckelwinkel gehobene Deckelplatte. Draufsicht: das Chassis und der Deckel von oben hinter dessen Hinterkante.",
        ),
        "retained": text(
            "Overall height 0.221 m, depth 0.218 m, the real thicknesses of 12.5 and 6.5 mm, the lid angle and the hinge at the rear edge of the chassis.",
            "Gesamthöhe 0,221 m, Tiefe 0,218 m, die realen Dicken von 12,5 und 6,5 mm, der Deckelwinkel und das Scharnier an der Hinterkante des Chassis.",
        ),
        "merged": text(
            "Deck and underside merge into one chassis bar; hinge, shell, rim and glass merge into one lid bar.",
            "Deck und Unterseite gehen in einem Chassisbalken auf; Scharnier, Schale, Rand und Glas in einem Deckelbalken.",
        ),
        "omitted": text(
            "Feet, bilge, key field, trackpad, glass and every port. Front and top view carry no detail beyond the two plates.",
            "Füße, Kimm, Tastenfeld, Touchpad, Glas und alle Buchsen. Vorder- und Draufsicht tragen keine Einzelheiten außer den zwei Platten.",
        ),
        "thinElements": text(
            "The 6.5 mm lid is 0.03 mm of paper and the 12.5 mm chassis 0.06 mm; both keep their real thickness and rely on the 0.18 mm pen outline to reach the feature width.",
            "Der 6,5 mm dicke Deckel sind 0,03 mm Papier und das 12,5 mm hohe Chassis 0,06 mm; beide behalten ihre reale Dicke und erreichen die Mindestbreite durch die 0,18-mm-Umrisslinie.",
        ),
        "hatching": text("None.", "Keine."),
        "styles": text("Chassis 0.55, lid 0.5; one body pen of 0.18 mm.", "Chassis 0,55, Deckel 0,5; ein Körperstift von 0,18 mm."),
        "overlays": text(
            "The chassis bar lists deck and underside as merged regions; the lid bar is the lid region.",
            "Der Chassisbalken führt Deck und Unterseite als zusammengeführte Regionen; der Deckelbalken ist die Deckelregion.",
        ),
    },
}


def apply(asset):
    """🖼 Writes the redrawn geometry and the decisions of every authored scale into the asset in memory; 1:100 gains the glazing pen of its display."""
    asset["representations"]["100"]["pens"] = dict(asset["representations"]["100"]["pens"], glazing=0.18)
    for scale, draw in (("50", scale50), ("100", scale100), ("200", scale200)):
        asset["representations"][scale]["geometry"] = draw()
        asset["representations"][scale]["decisions"] = DECISIONS[scale]
