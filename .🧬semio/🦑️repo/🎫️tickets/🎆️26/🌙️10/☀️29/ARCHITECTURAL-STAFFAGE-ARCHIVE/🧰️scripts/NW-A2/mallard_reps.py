"""mallard-duck: the four representation decisions. All outlines are hand-entered points in the local frame of their bone."""
from nw import *


def rep50():
    d = Draw(None, local=True)
    hat = [hatch("flank-fine", ("Fine flank lines (vermiculation)", "Feine Flankenlinien (Wellenzeichnung)"), "lines", 12, 0.6, 0.1)]
    d.curve("far-wing", "wing-f", [(-0.02, -0.02), (0.04, -0.04), (0.12, -0.043), (0.2, -0.03), (0.29, 0.0), (0.322, 0.012), (0.28, 0.03), (0.2, 0.046), (0.1, 0.05), (0.03, 0.04), (-0.02, 0.015)], 1.0, 0.55, region="wing-plumage", corners=(5,))
    d.curve("far-thigh", "thigh-f", [(-0.025, -0.03), (0.02, -0.035), (0.06, -0.02), (0.064, 0.02), (0.02, 0.034), (-0.025, 0.03)], 1.1, 0.4, region="body-plumage")
    d.curve("far-shank", "shank-f", [(-0.02, -0.0105), (0.05, -0.0105), (0.098, -0.0105), (0.108, 0.0), (0.098, 0.0105), (0.05, 0.0105), (-0.02, 0.0105), (-0.026, 0.0)], 1.2, 0.45, region="legs-skin", corners=(3,))
    d.curve("far-tarsus", "tarsus-f", [(-0.016, -0.0105), (0.05, -0.0105), (0.06, 0.0), (0.05, 0.0105), (-0.016, 0.0105), (-0.022, 0.0)], 1.2, 0.45, region="legs-skin", corners=(2,))
    d.curve("far-foot", "foot-f", [(-0.016, 0.0105), (0.025, 0.0105), (0.06, 0.006), (0.088, 0.0), (0.088, -0.006), (0.03, -0.0105), (-0.016, -0.0105), (-0.022, 0.0)], 1.2, 0.5, region="web-feet", corners=(3, 4))
    d.curve("tail", "tail", [(-0.03, -0.024), (0.02, -0.032), (0.06, -0.023), (0.092, -0.008), (0.098, 0.004), (0.07, 0.018), (0.03, 0.028), (-0.03, 0.028)], 2.0, 0.5, region="tail-plumage", corners=(3, 4))
    d.curve("body", "trunk", [(-0.02, 0.015), (0.0, 0.045), (0.05, 0.075), (0.12, 0.092), (0.2, 0.093), (0.27, 0.08), (0.325, 0.05), (0.35, 0.01), (0.34, -0.035), (0.30, -0.075), (0.23, -0.095), (0.14, -0.098), (0.06, -0.08), (0.0, -0.045)], 3.0, 0.3, region="body-plumage", hatch="flank-fine")
    d.curve("breast", "trunk", [(0.262, 0.08), (0.318, 0.056), (0.347, 0.01), (0.336, -0.045), (0.292, -0.084), (0.25, -0.092), (0.285, -0.03), (0.276, 0.03)], 3.1, 0.62, region="body-plumage")
    d.curve("near-thigh", "thigh-n", [(-0.02, -0.022), (0.02, -0.028), (0.058, -0.012), (0.06, 0.012), (0.02, 0.026), (-0.02, 0.022)], 3.5, 0.4, region="body-plumage")
    d.curve("near-wing", "wing-n", [(-0.02, -0.02), (0.04, -0.04), (0.12, -0.043), (0.2, -0.03), (0.29, 0.0), (0.322, 0.012), (0.28, 0.03), (0.2, 0.046), (0.1, 0.05), (0.03, 0.04), (-0.02, 0.015)], 4.0, 0.45, region="wing-plumage", corners=(5,))
    d.curve("speculum", "wing-n", [(0.13, 0.008), (0.19, 0.01), (0.225, 0.018), (0.2, 0.034), (0.14, 0.036)], 4.1, 0.85, region="wing-plumage")
    d.line("primary-lines", "wing-n", [(0.17, -0.02), (0.22, -0.018), (0.25, -0.01)], 4.2, 0.2, role="detail", region="wing-plumage")
    d.curve("near-shank", "shank-n", [(-0.02, -0.0105), (0.05, -0.0105), (0.098, -0.0105), (0.108, 0.0), (0.098, 0.0105), (0.05, 0.0105), (-0.02, 0.0105), (-0.026, 0.0)], 4.5, 0.55, region="legs-skin", corners=(3,))
    d.curve("near-tarsus", "tarsus-n", [(-0.016, -0.0105), (0.05, -0.0105), (0.06, 0.0), (0.05, 0.0105), (-0.016, 0.0105), (-0.022, 0.0)], 4.5, 0.55, region="legs-skin", corners=(2,))
    d.curve("near-foot", "foot-n", [(-0.016, 0.0105), (0.025, 0.0105), (0.06, 0.006), (0.088, 0.0), (0.088, -0.006), (0.03, -0.0105), (-0.016, -0.0105), (-0.022, 0.0)], 4.5, 0.6, region="web-feet", corners=(3, 4))
    d.line("toe-line", "foot-n", [(0.0, -0.002), (0.074, -0.005)], 4.6, 0.2, role="detail", region="web-feet")
    d.curve("neck-1", "neck-1", [(-0.045, 0.0), (-0.035, -0.038), (0.01, -0.034), (0.075, -0.024), (0.085, 0.0), (0.075, 0.024), (0.01, 0.036), (-0.035, 0.04)], 3.2, 0.5, region="neck-plumage")
    d.curve("neck-2", "neck-2", [(-0.012, -0.024), (0.035, -0.022), (0.078, -0.02), (0.08, 0.0), (0.078, 0.02), (0.035, 0.022), (-0.012, 0.024)], 3.3, 0.5, region="neck-plumage")
    d.curve("neck-3", "neck-3", [(-0.012, -0.022), (0.03, -0.02), (0.066, -0.024), (0.07, 0.0), (0.068, 0.026), (0.03, 0.02), (-0.012, 0.022)], 3.4, 0.6, region="neck-plumage")
    d.curve("neck-ring", "neck-2", [(0.024, -0.018), (0.04, -0.018), (0.042, 0.0), (0.04, 0.018), (0.024, 0.018), (0.022, 0.0)], 3.6, 0.04, region="neck-plumage")
    d.curve("head", "head", [(-0.025, -0.022), (-0.02, 0.02), (0.0, 0.032), (0.03, 0.034), (0.052, 0.022), (0.058, 0.004), (0.056, -0.012), (0.04, -0.024), (0.01, -0.03)], 4.0, 0.78, region="head-plumage")
    d.curve("bill", "head", [(0.045, 0.018), (0.08, 0.014), (0.1, 0.008), (0.109, 0.0), (0.104, -0.007), (0.08, -0.013), (0.045, -0.016)], 4.1, 0.35, region="bill", corners=(3,))
    d.line("bill-line", "head", [(0.05, 0.0), (0.098, 0.0)], 4.2, 0.2, role="detail", region="bill")
    dec = decisions(
        ("Full articulation 0.52 m x 0.43 m = 10.4 x 8.6 mm: drake with dark head, white neck ring, grey flank hatch, chestnut breast, wing with speculum, orange legs.", "Volle Gliederung 0,52 m x 0,43 m = 10,4 x 8,6 mm: Erpel mit dunklem Kopf, weißem Halsring, grauer Flankenschraffur, kastanienbrauner Brust, Flügel mit Spiegel, orangen Beinen."),
        ("Both wings, both legs in shank, tarsus and webbed foot, thigh pads, three neck pieces, head, bill with gape line, neck ring, speculum, one primary line.", "Beide Flügel, beide Beine in Unterschenkel, Lauf und Schwimmfuß, Schenkelpolster, drei Halsstücke, Kopf, Schnabel mit Mundspalte, Halsring, Spiegel, eine Schwingenlinie."),
        ("The breast patch is drawn over the body in its own tone; the thigh pads share the body region.", "Der Brustfleck liegt in eigenem Ton über dem Körper; die Schenkelpolster teilen die Körperregion."),
        ("The eye (0.008 m = 0.16 mm; in the 0.07 m head it cannot keep the 0.15 mm ink gap to bill and neck outline), the nostril (0.003 m), individual feathers, toes beyond one toe line, tongue and lamellae.", "Das Auge (0,008 m = 0,16 mm; im 0,07 m großen Kopf hält es den Tintenabstand von 0,15 mm zu Schnabel und Halskontur nicht), das Nasenloch (0,003 m), einzelne Federn, Zehen über eine Zehenlinie hinaus, Zunge und Lamellen."),
        ("Shank and tarsus (0.013 m = 0.26 mm) and the web (0.017 m = 0.34 mm) are filled at true width, above the 0.2 mm feature limit.", "Unterschenkel und Lauf (0,013 m = 0,26 mm) und Schwimmhaut (0,017 m = 0,34 mm) sind in wahrer Breite gefüllt, über der 0,2-mm-Grenze."),
        ("One hatch: 12 degree lines, 0.6 mm spacing, 0.1 mm pen, on the grey flank as material indication of the fine vermiculation.", "Eine Schraffur: Linien unter 12 Grad, 0,6 mm Abstand, 0,1-mm-Stift, auf der grauen Flanke als Materialhinweis für die feine Wellenzeichnung."),
        ("0.25 mm silhouette, 0.13 mm details, 0.1 mm hatch; tone dark for the head, light for the neck ring and speculum.", "0,25-mm-Kontur, 0,13-mm-Details, 0,1-mm-Schraffur; Ton dunkel für den Kopf, hell für Halsring und Spiegel."),
        ("Every thermal region has its own shape (the merged thigh pads and the breast patch use the body region, neck ring and neck pieces the neck region), so a thermal overlay shows plumage, bare skin and bill separately.", "Jede thermische Region hat eine eigene Form (Schenkelpolster und Brustfleck nutzen die Körperregion, Halsring und Halsstücke die Halsregion); ein Wärmebild zeigt so Gefieder, nackte Haut und Schnabel getrennt."))
    return {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75},
            "pens": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35}, "hatches": hat,
            "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep100():
    d = Draw(None, local=True)
    d.curve("tail", "tail", [(-0.03, -0.026), (0.03, -0.034), (0.078, -0.022), (0.1, 0.0), (0.06, 0.022), (-0.03, 0.03)], 2.0, 0.5, region="tail-plumage", corners=(3,))
    d.curve("body", "trunk", [(-0.02, 0.02), (0.05, 0.078), (0.14, 0.094), (0.25, 0.084), (0.34, 0.035), (0.345, -0.03), (0.28, -0.085), (0.15, -0.1), (0.05, -0.075), (0.0, -0.04)], 3.0, 0.3, region="body-plumage")
    d.curve("far-wing", "wing-f", [(-0.02, -0.04), (0.1, -0.065), (0.2, -0.051), (0.324, -0.003), (0.2, 0.035), (0.03, 0.031)], 1.0, 0.55, region="wing-plumage", corners=(3,))
    d.curve("near-wing", "wing-n", [(-0.02, -0.04), (0.1, -0.065), (0.2, -0.051), (0.324, -0.003), (0.2, 0.035), (0.03, 0.031)], 4.0, 0.45, region="wing-plumage", corners=(3,))
    d.line("wing-line", "wing-n", [(0.04, -0.003), (0.17, -0.019), (0.3, -0.004)], 4.1, 0.2, role="detail", region="wing-plumage")
    d.curve("neck-lower", "neck-1", [(0.0, -0.03), (0.075, -0.027), (0.1, -0.026), (0.105, 0.0), (0.1, 0.026), (0.075, 0.028), (0.0, 0.034), (-0.006, 0.0)], 3.2, 0.5, region="neck-plumage")
    d.curve("neck-upper", "neck-3", [(-0.1, -0.026), (0.03, -0.026), (0.075, -0.026), (0.08, 0.0), (0.075, 0.028), (0.03, 0.028), (-0.1, 0.026), (-0.108, 0.0)], 3.4, 0.55, region="neck-plumage")
    d.line("near-leg", "shank-n", [(0.0, 0.0), (0.085, 0.0), (0.1082, 0.0385), (0.0514, 0.0976)], 4.5, 0.55, role="body", regions=["legs-skin", "web-feet"], pen=0.2)
    d.curve("head", "head", [(-0.05, -0.022), (-0.048, 0.024), (0.03, 0.038), (0.058, 0.024), (0.064, 0.0), (0.04, -0.026), (-0.01, -0.034)], 4.0, 0.78, region="head-plumage")
    d.line("bill", "head", [(-0.02, 0.0), (0.105, -0.002)], 4.1, 0.35, role="body", region="bill", pen=0.28)
    dec = decisions(
        ("Articulated outline 0.60 x 0.46 m = 6.0 x 4.6 mm: body, tail, two wings, a two-piece neck (lower neck, upper neck), head and a bill stroke; the near leg as one polyline.", "Gegliederter Umriss 0,60 x 0,46 m = 6,0 x 4,6 mm: Körper, Schwanz, zwei Flügel, ein zweiteiliger Hals (Unter- und Oberhals), Kopf und ein Schnabelstrich; das nahe Bein als ein Linienzug."),
        ("Near wing with one feather line, far wing, head separate from the neck, the bill as the darkest stroke, the near leg.", "Naher Flügel mit einer Federlinie, ferner Flügel, Kopf vom Hals getrennt, der Schnabel als dunkelster Strich, das nahe Bein."),
        ("Neck pieces 1-3 merge into two overlapping pieces (the lower neck covers neck piece 2, the upper neck its rear half); neck ring, breast patch, thigh pad and speculum merge into the neck, body and wing tones; thigh, shank, tarsus and foot are one polyline carrying both leg regions.", "Die Halsstücke 1-3 gehen in zwei überlappende Stücke auf (der Unterhals deckt Halsstück 2, der Oberhals dessen hintere Hälfte); Halsring, Brustfleck, Schenkelpolster und Spiegel gehen in den Hals-, Körper- und Flügeltönen auf; Oberschenkel, Unterschenkel, Lauf und Fuß sind ein Linienzug, der beide Beinregionen trägt."),
        ("The far leg (any outline of it in the water poses would come within 0.15 mm of the wing and neck outlines), eye (0.008 m = 0.08 mm), nostril, toe lines, individual feathers.", "Das ferne Bein (jede Kontur davon käme in den Wasserposen näher als 0,15 mm an Flügel- und Halskontur), Auge (0,008 m = 0,08 mm), Nasenloch, Zehenlinien, einzelne Federn."),
        ("Leg 0.016 m real is a 0.2 mm stroke (0.02 m), 0.004 m wider than real; the bill (0.016-0.03 m) a 0.28 mm stroke = 0.028 m, inside its real thickness range. The polyline is bound to the shank in the standing configuration: the foot angle of the water poses is not articulated at 1:100 (it is at 1:50).", "Das 0,016 m dicke Bein ist ein 0,2-mm-Strich (0,02 m), 0,004 m breiter als real; der Schnabel (0,016-0,03 m) ein 0,28-mm-Strich = 0,028 m, innerhalb seiner realen Dicke. Der Linienzug ist an den Unterschenkel in der Stehkonfiguration gebunden: der Fußwinkel der Wasserposen ist bei 1:100 nicht gegliedert (bei 1:50 schon)."),
        ("None: the 0.4 mm minimum hatch spacing equals 0.04 m, which would cover the whole flank.", "Keine: der Mindestabstand von 0,4 mm entspricht 0,04 m und würde die ganze Flanke füllen."),
        ("0.18 mm silhouette, 0.13 mm feather line; tones: head darkest, body mid.", "0,18-mm-Kontur, 0,13-mm-Federlinie; Töne: Kopf am dunkelsten, Körper mittel."),
        ("Each thermal region has a shape; the bill and leg strokes carry their regions so bare-skin heat loss stays visible as lines.", "Jede thermische Region hat eine Form; die Schnabel- und Beinstriche tragen ihre Regionen, sodass der Wärmeverlust der nackten Haut als Linien sichtbar bleibt."))
    return {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150},
            "pens": {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [],
            "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep200():
    d = Draw(None, local=True)
    d.curve("tail", "tail", [(-0.03, -0.034), (0.04, -0.04), (0.086, -0.02), (0.086, 0.02), (0.04, 0.034), (-0.03, 0.034)], 2.0, 0.5, region="tail-plumage")
    d.curve("body", "trunk", [(-0.02, 0.025), (0.06, 0.08), (0.15, 0.095), (0.26, 0.085), (0.345, 0.03), (0.33, -0.05), (0.25, -0.092), (0.12, -0.1), (0.03, -0.065)], 3.0, 0.3, region="body-plumage")
    d.line("wing", "wing-n", [(0.06, -0.088), (0.12, -0.09), (0.25, -0.075), (0.31, -0.033), (0.2, 0.005), (0.04, -0.005)], 4.0, 0.5, role="body", region="wing-plumage", pen=0.15, smooth=True)
    d.curve("neck-lower", "neck-1", [(0.0, -0.034), (0.075, -0.032), (0.1, -0.03), (0.108, 0.0), (0.1, 0.03), (0.075, 0.032), (0.0, 0.036), (-0.006, 0.0)], 3.2, 0.5, region="neck-plumage")
    d.curve("neck-upper", "neck-3", [(-0.1, -0.03), (0.03, -0.03), (0.075, -0.03), (0.082, 0.0), (0.075, 0.032), (0.03, 0.032), (-0.1, 0.03), (-0.108, 0.0)], 3.4, 0.55, region="neck-plumage")
    d.line("near-leg", "shank-n", [(0.0, 0.0), (0.085, 0.0), (0.1082, 0.0385), (0.0514, 0.0976)], 4.5, 0.55, role="body", regions=["legs-skin", "web-feet"], pen=0.16)
    d.line("near-leg", "shank-n", [(0.0, 0.0), (0.085, 0.0), (0.1082, 0.0385), (0.0514, 0.0976)], 4.5, 0.55, role="body", regions=["legs-skin", "web-feet"], pen=0.16)
    d.curve("head", "head", [(-0.06, -0.026), (-0.058, 0.026), (0.03, 0.04), (0.062, 0.026), (0.068, 0.0), (0.04, -0.03), (-0.02, -0.038)], 4.0, 0.75, region="head-plumage")
    d.line("bill", "head", [(-0.02, 0.0), (0.108, -0.002)], 4.1, 0.35, role="body", region="bill", pen=0.2)
    dec = decisions(
        ("Composed profile 0.60 x 0.46 m = 3.0 x 2.3 mm: filled body, blunt wedge tail, a two-piece filled neck, head, wing as one edge stroke, bill stroke and the near leg as one polyline.", "Zusammengesetztes Profil 0,60 x 0,46 m = 3,0 x 2,3 mm: gefüllter Körper, stumpfer Keilschwanz, zweiteiliger gefüllter Hals, Kopf, Flügel als eine Randlinie, Schnabelstrich und das nahe Bein als ein Linienzug."),
        ("The long neck S, the heavy rounded body, the tail, the line of the folded wing and the single leg as the identifying masses.", "Der lange Halsschwung, der schwere runde Körper, der Schwanz, die Linie des angelegten Flügels und das einzelne Bein als kennzeichnende Massen."),
        ("The neck is two overlapping pieces (0.1 m overlap); thigh, shank, tarsus and webbed foot are one polyline carrying both leg regions; the tail (0.07 m thick instead of the real 0.05) is widened to the 0.04 m feature limit; the wing edge stroke is moved 0.045 m up the flank so that no pose brings it within 0.15 mm of the leg.", "Der Hals besteht aus zwei überlappenden Stücken (0,1 m Überlappung); Oberschenkel, Unterschenkel, Lauf und Schwimmfuß sind ein Linienzug, der beide Beinregionen trägt; der Schwanz (0,07 m statt real 0,05 m dick) ist auf die Merkmalsgrenze von 0,04 m verbreitert; die Flügelrandlinie ist 0,045 m an der Flanke hochgesetzt, damit sie in keiner Pose näher als 0,15 mm ans Bein kommt."),
        ("Far wing and far leg (every outline of it comes within 0.15 mm of the near leg or the wing in some pose), neck ring, speculum, eye, feather lines.", "Ferner Flügel und fernes Bein (jede Kontur davon kommt in irgendeiner Pose näher als 0,15 mm an das nahe Bein oder den Flügel), Halsring, Spiegel, Auge, Federlinien."),
        ("Bill 0.016-0.03 m is a 0.2 mm stroke (0.04 m); the leg 0.016 m is a 0.16 mm stroke (0.032 m) bound to the shank in the standing configuration, so the foot angle of the water poses is not articulated at 1:200.", "Der 0,016-0,03 m dicke Schnabel ist ein 0,2-mm-Strich (0,04 m); das 0,016 m dicke Bein ein 0,16-mm-Strich (0,032 m), an den Unterschenkel in der Stehkonfiguration gebunden, sodass der Fußwinkel der Wasserposen bei 1:200 nicht gegliedert ist."),
        ("None.", "Keine."),
        ("0.15 mm outline, no detail pen; tone separates the dark head from the lighter body.", "0,15-mm-Kontur, kein Detailstift; der Ton trennt den dunklen Kopf vom helleren Körper."),
        ("Every region is carried by a shape or stroke; the wing region by its edge stroke, the leg and foot regions by the leg polyline, bill by the bill stroke.", "Jede Region wird von einer Form oder einem Strich getragen; die Flügelregion von ihrer Randlinie, die Bein- und Fußregionen vom Beinlinienzug, der Schnabel vom Schnabelstrich."))
    return {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350},
            "pens": {"body": 0.15, "detail": None, "texture": None, "glazing": None, "ground": 0.15}, "hatches": [],
            "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep500():
    return {"status": "unavailable", "denominator": 500,
            "reason": tx("A mallard is 0.52 m x 0.43 m = 1.0 x 0.9 mm on paper at 1:500. The neck (0.045 m = 0.09 mm), the head (0.05 m = 0.1 mm) and the legs (0.013 m = 0.03 mm) are below the 0.2 mm feature width of the office laser profile, so the bird would print as a blob without neck and head.",
                         "Eine Stockente misst 0,52 m x 0,43 m = 1,0 x 0,9 mm auf dem Papier bei 1:500. Hals (0,045 m = 0,09 mm), Kopf (0,05 m = 0,1 mm) und Beine (0,013 m = 0,03 mm) liegen unter der Merkmalsbreite von 0,2 mm des Büro-Laser-Profils; der Vogel würde als Fleck ohne Hals und Kopf drucken."),
            "limitingFeature": tx("Neck and head (0.045 m = 0.09 mm)", "Hals und Kopf (0,045 m = 0,09 mm)"), "limitingSizeM": 0.045,
            "alternatives": [{"kind": "scale", "denominator": 200}]}
