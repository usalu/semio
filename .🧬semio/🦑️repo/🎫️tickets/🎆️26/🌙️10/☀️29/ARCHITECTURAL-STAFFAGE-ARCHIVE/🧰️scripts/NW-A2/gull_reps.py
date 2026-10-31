"""herring-gull representations: hand-entered outlines in the local frame of their bones"""
from nw import *

BODY50 = [(-0.03, 0.02), (0.0, 0.055), (0.07, 0.085), (0.17, 0.095), (0.26, 0.085), (0.32, 0.055), (0.35, 0.015), (0.345, -0.035), (0.30, -0.075), (0.2, -0.092), (0.1, -0.088), (0.03, -0.065), (-0.025, -0.02)]
SHANK50 = [(-0.02, -0.0105), (0.06, -0.0105), (0.11, -0.0105), (0.118, 0.0), (0.11, 0.0105), (0.06, 0.0105), (-0.02, 0.0105), (-0.026, 0.0)]
TARS50 = [(-0.016, -0.0105), (0.065, -0.0105), (0.075, 0.0), (0.065, 0.0105), (-0.016, 0.0105), (-0.022, 0.0)]
FOOT50 = [(-0.016, 0.0105), (0.025, 0.0105), (0.065, 0.007), (0.088, 0.0), (0.088, -0.007), (0.03, -0.0105), (-0.016, -0.0105), (-0.022, 0.0)]


def rep50():
    d = Draw(None, local=True)
    for s, z, t in (("far", 1.0, 0.45), ("near", 4.0, 0.3)):
        b = "f" if s == "far" else "n"
        d.curve(f"{s}-arm", f"arm-{b}", [(-0.03, -0.04), (0.06, -0.05), (0.15, -0.048), (0.215, -0.038), (0.228, 0.0), (0.2, 0.04), (0.1, 0.05), (0.0, 0.04), (-0.03, 0.0)], z, t, region="wing-plumage")
        d.curve(f"{s}-hand", f"hand-{b}", [(-0.03, -0.032), (0.08, -0.036), (0.17, -0.03), (0.235, -0.015), (0.243, 0.012), (0.17, 0.026), (0.08, 0.03), (-0.03, 0.028)], z + 0.1, t + 0.05, region="wing-plumage", corners=())
    d.curve("wingtip", "hand-n", [(0.15, -0.027), (0.225, -0.014), (0.239, 0.008), (0.16, 0.021)], 4.2, 0.92, region="wing-plumage")
    for s, z, t in (("far", 1.2, 0.5), ("near", 4.5, 0.45)):
        b = "f" if s == "far" else "n"
        d.curve(f"{s}-shank", f"shank-{b}", SHANK50, z, t, region="legs-skin", corners=(3,))
        d.curve(f"{s}-tarsus", f"tarsus-{b}", TARS50, z, t, region="legs-skin", corners=(2,))
        d.curve(f"{s}-foot", f"foot-{b}", FOOT50, z, t + 0.05, region="web-feet", corners=(3, 4))
    d.curve("far-thigh", "thigh-f", [(-0.02, -0.03), (0.03, -0.036), (0.07, -0.02), (0.072, 0.02), (0.03, 0.034), (-0.02, 0.03)], 1.1, 0.12, region="body-plumage")
    d.curve("tail", "tail", [(-0.03, -0.034), (0.06, -0.05), (0.14, -0.044), (0.152, 0.0), (0.14, 0.044), (0.06, 0.05), (-0.03, 0.036)], 2.0, 0.12, region="tail-plumage")
    d.curve("body", "trunk", BODY50, 3.0, 0.08, region="body-plumage")
    d.curve("mantle", "trunk", [(0.03, 0.05), (0.1, 0.083), (0.2, 0.088), (0.28, 0.07), (0.25, 0.035), (0.15, 0.02), (0.07, 0.0)], 3.1, 0.3, region="body-plumage")
    d.curve("near-thigh", "thigh-n", [(-0.02, -0.03), (0.03, -0.036), (0.07, -0.02), (0.072, 0.02), (0.03, 0.034), (-0.02, 0.03)], 3.6, 0.1, region="body-plumage")
    d.curve("neck-1", "neck-1", [(-0.05, -0.045), (0.0, -0.042), (0.08, -0.034), (0.1, 0.0), (0.08, 0.034), (0.0, 0.042), (-0.05, 0.045)], 3.2, 0.08, region="neck-plumage")
    d.curve("neck-2", "neck-2", [(-0.04, -0.034), (0.07, -0.032), (0.095, 0.0), (0.07, 0.034), (-0.04, 0.034)], 3.3, 0.08, region="neck-plumage")
    d.curve("head", "head", [(-0.05, -0.03), (-0.045, 0.03), (0.0, 0.042), (0.04, 0.04), (0.06, 0.028), (0.066, 0.005), (0.058, -0.015), (0.03, -0.03)], 4.0, 0.06, region="head-plumage")
    d.curve("bill", "head", [(0.05, 0.02), (0.095, 0.016), (0.125, 0.006), (0.129, -0.004), (0.112, -0.014), (0.095, -0.02), (0.05, -0.022)], 4.1, 0.35, region="bill", corners=(3,))
    dec = decisions(
        ("Full articulation 0.66 x 0.52 m = 13.2 x 10.4 mm: white gull with pale grey mantle, black wingtips (wings drawn in oblique projection when spread), yellow bill, pink legs.", "Volle Gliederung 0,66 x 0,52 m = 13,2 x 10,4 mm: weiße Möwe mit hellgrauem Mantel, schwarzen Flügelspitzen (gespreizte Flügel in schräger Projektion), gelbem Schnabel, rosa Beinen."),
        ("Both wings in arm and hand, black wingtip, both legs with shank, tarsus and webbed foot, thigh pads, neck in two pieces, head, hooked bill.", "Beide Flügel in Arm und Hand, schwarze Flügelspitze, beide Beine mit Unterschenkel, Lauf und Schwimmfuß, Schenkelpolster, zweiteiliger Hals, Kopf, Hakenschnabel."),
        ("The mantle and thigh pads use the body region; the black wingtip uses the wing region.", "Mantel und Schenkelpolster nutzen die Körperregion; die schwarze Flügelspitze die Flügelregion."),
        ("The eye (0.015 m) and the covert line (both would lie within 0.15 mm of the bill, neck or wing outline in some pose), individual feathers, the red spot of the bill (0.008 m = 0.16 mm), nostrils, toe lines.", "Das Auge (0,015 m) und die Deckfederlinie (beide läge in irgendeiner Pose näher als 0,15 mm an Schnabel-, Hals- oder Flügelkontur), einzelne Federn, der rote Schnabelfleck (0,008 m = 0,16 mm), Nasenlöcher, Zehenlinien."),
        ("Shank, tarsus and foot are filled 0.021 m wide (0.42 mm) instead of the real 0.012 m: 0.009 m wider, less than the 0.0125 m the outline pen adds.", "Unterschenkel, Lauf und Fuß sind 0,021 m (0,42 mm) breit gefüllt statt real 0,012 m: 0,009 m breiter, weniger als die 0,0125 m, die der Konturstift hinzufügt."),
        ("None: white plumage stays clean, the tone separates mantle and wingtip.", "Keine: das weiße Gefieder bleibt sauber, der Ton trennt Mantel und Flügelspitze."),
        ("0.25 mm silhouette, no detail pen; tones white 0.1, mantle 0.3, wingtip 0.9, legs 0.45.", "0,25-mm-Kontur, kein Detailstift; Töne weiß 0,1, Mantel 0,3, Flügelspitze 0,9, Beine 0,45."),
        ("Every thermal region has its own shape so a thermal overlay separates plumage, wing, bill and bare skin.", "Jede thermische Region hat eine eigene Form, sodass ein Wärmebild Gefieder, Flügel, Schnabel und nackte Haut trennt."))
    return {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [],
            "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep100():
    d = Draw(None, local=True)
    for s, z, t in (("far", 1.0, 0.45), ("near", 4.0, 0.3)):
        b = "f" if s == "far" else "n"
        d.curve(f"{s}-arm", f"arm-{b}", [(-0.04, -0.044), (0.1, -0.054), (0.22, -0.04), (0.234, 0.0), (0.2, 0.044), (0.05, 0.05), (-0.04, 0.0)], z, t, region="wing-plumage")
        d.curve(f"{s}-hand", f"hand-{b}", [(-0.04, -0.036), (0.12, -0.038), (0.236, -0.016), (0.248, 0.012), (0.12, 0.03), (-0.04, 0.03)], z + 0.1, t + 0.05, region="wing-plumage")
    d.curve("wingtip", "hand-n", [(0.17, -0.03), (0.23, -0.014), (0.244, 0.01), (0.17, 0.024)], 4.2, 0.92, region="wing-plumage")
    d.curve("tail", "tail", [(-0.03, -0.034), (0.1, -0.05), (0.154, -0.02), (0.154, 0.02), (0.1, 0.05), (-0.03, 0.036)], 2.0, 0.12, region="tail-plumage")
    d.curve("body", "trunk", [(-0.03, 0.02), (0.07, 0.088), (0.2, 0.096), (0.32, 0.055), (0.35, 0.0), (0.3, -0.075), (0.15, -0.095), (0.03, -0.065)], 3.0, 0.08, region="body-plumage")
    d.curve("neck-1", "neck-1", [(-0.06, -0.046), (0.08, -0.036), (0.108, 0.0), (0.08, 0.036), (-0.06, 0.046)], 3.2, 0.08, region="neck-plumage")
    d.curve("neck-2", "neck-2", [(-0.06, -0.036), (0.07, -0.034), (0.105, 0.0), (0.07, 0.034), (-0.06, 0.036)], 3.3, 0.08, region="neck-plumage")
    d.line("near-leg", "shank-n", [(-0.03, 0.0), (0.1, 0.0), (0.1389, 0.0521), (0.0863, 0.1188)], 4.5, 0.5, role="body", regions=["legs-skin", "web-feet"], pen=0.2)
    d.curve("head", "head", [(-0.06, -0.032), (-0.055, 0.032), (0.0, 0.044), (0.05, 0.036), (0.068, 0.006), (0.05, -0.02), (0.0, -0.034)], 4.0, 0.06, region="head-plumage")
    d.curve("bill", "head", [(0.05, 0.022), (0.1, 0.016), (0.13, 0.004), (0.116, -0.012), (0.098, -0.02), (0.05, -0.024)], 4.1, 0.35, region="bill", corners=(2,))
    dec = decisions(
        ("Articulated outline 0.66 x 0.52 m = 6.6 x 5.2 mm: body, tail, both wings in arm and hand, black wingtip, two neck pieces, head and bill, the near leg as one polyline.", "Gegliederter Umriss 0,66 x 0,52 m = 6,6 x 5,2 mm: Körper, Schwanz, beide Flügel in Arm und Hand, schwarze Flügelspitze, zwei Halsstücke, Kopf und Schnabel, das nahe Bein als ein Linienzug."),
        ("The wing in two pieces so gliding and landing postures articulate; bill as a separate darker piece; the near leg.", "Der Flügel in zwei Stücken, damit Gleit- und Landehaltung gegliedert sind; Schnabel als eigenes dunkleres Stück; das nahe Bein."),
        ("Mantle, covert lines, thigh pads and eye merge into the body and wing tones; the wingtip stays its own black piece.", "Mantel, Deckfederlinien, Schenkelpolster und Auge gehen in die Körper- und Flügeltöne auf; die Flügelspitze bleibt ein eigenes schwarzes Stück."),
        ("The far leg (its outline comes within 0.15 mm of the near leg, tail or body in some pose), eye (0.015 m = 0.15 mm), red bill spot, feather lines, toes.", "Das ferne Bein (seine Kontur kommt in irgendeiner Pose näher als 0,15 mm an das nahe Bein, den Schwanz oder den Körper), Auge (0,015 m = 0,15 mm), roter Schnabelfleck, Federlinien, Zehen."),
        ("Leg 0.012 m = 0.12 mm real is a 0.2 mm stroke (0.02 m); thigh, shank, tarsus and foot are one polyline bound to the shank in the standing configuration, so the foot angle of flight and landing poses is not articulated at 1:100 (it is at 1:50); wing tips are blunted to 0.025 m so the feature stays above 0.2 mm.", "Das reale Bein von 0,012 m = 0,12 mm ist ein 0,2-mm-Strich (0,02 m); Oberschenkel, Unterschenkel, Lauf und Fuß sind ein Linienzug, an den Unterschenkel in der Stehkonfiguration gebunden, sodass der Fußwinkel der Flug- und Landeposen bei 1:100 nicht gegliedert ist (bei 1:50 schon); die Flügelspitzen sind auf 0,025 m abgestumpft, damit die Merkmalsbreite über 0,2 mm bleibt."),
        ("None.", "Keine."),
        ("0.18 mm silhouette; tones white 0.1, wing 0.3, wingtip 0.92.", "0,18-mm-Kontur; Töne weiß 0,1, Flügel 0,3, Flügelspitze 0,92."),
        ("Each region has a shape; the leg polyline carries both bare-skin regions.", "Jede Region hat eine Form; der Beinlinienzug trägt beide Regionen der nackten Haut."))
    return {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [],
            "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep200():
    d = Draw(None, local=True)
    d.curve("arm", "arm-n", [(-0.05, -0.05), (0.12, -0.056), (0.236, -0.04), (0.236, 0.04), (0.12, 0.034), (-0.05, 0.05)], 4.0, 0.3, region="wing-plumage")
    d.curve("hand", "hand-n", [(-0.06, -0.04), (0.12, -0.04), (0.24, -0.026), (0.252, 0.0), (0.24, 0.026), (0.12, 0.034), (-0.06, 0.034)], 4.1, 0.55, region="wing-plumage")
    d.curve("tail", "tail", [(-0.03, -0.04), (0.1, -0.056), (0.156, -0.02), (0.156, 0.02), (0.1, 0.056), (-0.03, 0.04)], 2.0, 0.12, region="tail-plumage")
    d.curve("body", "trunk", [(-0.03, 0.02), (0.07, 0.09), (0.2, 0.098), (0.32, 0.055), (0.352, 0.0), (0.3, -0.075), (0.15, -0.096), (0.03, -0.065)], 3.0, 0.08, region="body-plumage")
    d.curve("neck-1", "neck-1", [(-0.07, -0.05), (0.06, -0.04), (0.075, 0.0), (0.06, 0.04), (-0.07, 0.05)], 3.2, 0.08, region="neck-plumage")
    d.curve("neck-2", "neck-2", [(-0.07, -0.04), (0.06, -0.038), (0.085, 0.0), (0.06, 0.038), (-0.07, 0.04)], 3.3, 0.08, region="neck-plumage")
    d.line("near-leg", "shank-n", [(-0.03, 0.0), (0.1, 0.0), (0.1389, 0.0521), (0.0863, 0.1188)], 4.5, 0.5, role="body", regions=["legs-skin", "web-feet"], pen=0.16)
    d.curve("head", "head", [(-0.07, -0.036), (-0.065, 0.036), (0.0, 0.046), (0.05, 0.038), (0.07, 0.006), (0.05, -0.022), (0.0, -0.036)], 4.0, 0.06, region="head-plumage")
    d.line("bill", "head", [(0.0, 0.0), (0.13, -0.002)], 4.1, 0.35, role="body", region="bill", pen=0.2)
    dec = decisions(
        ("Composed profile 0.66 x 0.52 m = 3.3 x 2.6 mm: body, fan tail, near wing in two blunt pieces, two-piece neck, head, bill stroke, the near leg as one polyline.", "Zusammengesetztes Profil 0,66 x 0,52 m = 3,3 x 2,6 mm: Körper, Fächerschwanz, naher Flügel in zwei stumpfen Stücken, zweiteiliger Hals, Kopf, Schnabelstrich, das nahe Bein als ein Linienzug."),
        ("The dark tipped hand against the paler arm and body, the upright neck, the heavy bill line.", "Die dunkle Hand gegen Arm und Körper, der aufrechte Hals, der kräftige Schnabelstrich."),
        ("Wingtip, mantle, eye and thigh pad merge into the hand tone; the hand tip is blunted to 0.05 m (real primaries 0.05-0.06 m wide near the tip).", "Flügelspitze, Mantel, Auge und Schenkelpolster gehen im Handton auf; die Handspitze ist auf 0,05 m abgestumpft (reale Handschwingen 0,05-0,06 m breit nahe der Spitze)."),
        ("Far wing (hidden behind the near wing in standing, a thin sliver in flight) and far leg (every outline of it comes within 0.15 mm of the near leg or tail in some pose), eye, feather lines.", "Ferner Flügel (verdeckt im Stand, im Flug nur ein schmaler Streifen) und fernes Bein (jede Kontur davon kommt in irgendeiner Pose näher als 0,15 mm an das nahe Bein oder den Schwanz), Auge, Federlinien."),
        ("Bill 0.02-0.04 m is a 0.2 mm stroke (0.04 m); leg 0.012 m is a 0.16 mm polyline stroke (0.032 m) bound to the shank in the standing configuration (no foot articulation at 1:200).", "Der 0,02-0,04 m dicke Schnabel ist ein 0,2-mm-Strich (0,04 m); das 0,012 m dicke Bein ein 0,16-mm-Linienzug (0,032 m), an den Unterschenkel in der Stehkonfiguration gebunden (keine Fußgliederung bei 1:200)."),
        ("None.", "Keine."),
        ("0.15 mm outline only, tones white 0.1 / wing 0.3-0.55.", "Nur 0,15-mm-Kontur, Töne weiß 0,1 / Flügel 0,3-0,55."),
        ("Each region is carried by a shape or stroke; the bill region appears as the bill stroke, leg and foot regions as the leg polyline.", "Jede Region wird von einer Form oder einem Strich getragen; die Schnabelregion erscheint als Schnabelstrich, Bein- und Fußregion als Beinlinienzug."))
    return {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": {"body": 0.15, "detail": None, "texture": None, "glazing": None, "ground": 0.15}, "hatches": [],
            "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep500():
    return {"status": "unavailable", "denominator": 500,
            "reason": tx("A standing herring gull is 0.66 m x 0.52 m = 1.3 x 1.0 mm on paper at 1:500; the neck (0.07 m = 0.14 mm), head (0.07 m) and legs (0.012 m = 0.02 mm) are below the 0.2 mm feature width of the office laser profile, and the spread wings of the gliding pose are foreshortened in elevation to the chord (0.18 m = 0.36 mm).",
                         "Eine stehende Silbermöwe misst 0,66 m x 0,52 m = 1,3 x 1,0 mm auf dem Papier bei 1:500; Hals (0,07 m = 0,14 mm), Kopf (0,07 m) und Beine (0,012 m = 0,02 mm) liegen unter der Merkmalsbreite von 0,2 mm des Büro-Laser-Profils, und die gespreizten Flügel der Gleitpose sind im Aufriss auf die Flügeltiefe (0,18 m = 0,36 mm) verkürzt."),
            "limitingFeature": tx("Neck and head (0.07 m = 0.14 mm)", "Hals und Kopf (0,07 m = 0,14 mm)"), "limitingSizeM": 0.07, "alternatives": [{"kind": "scale", "denominator": 200}]}
