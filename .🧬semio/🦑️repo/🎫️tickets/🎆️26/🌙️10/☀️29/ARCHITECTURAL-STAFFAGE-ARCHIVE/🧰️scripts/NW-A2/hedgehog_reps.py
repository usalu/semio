"""hedgehog representations (bone-local hand-entered outlines)"""
from nw import *


def teeth(pts, idxs, amp, k=1):
    """insert one spine tooth (hand-chosen amplitude) outward on the edges i -> i+1 listed in idxs"""
    out = []
    n = len(pts)
    for i, p in enumerate(pts):
        out.append(p)
        if i in idxs:
            q = pts[(i+1) % n]
            mx, my = (p[0]+q[0])/2, (p[1]+q[1])/2
            dx, dy = q[0]-p[0], q[1]-p[1]
            L = math.hypot(dx, dy) or 1
            nx, ny = dy/L, -dx/L
            out.append((mx + nx*amp, my + ny*amp))
    return out


DOME50 = [(-0.03, -0.008), (-0.036, 0.016), (-0.016, 0.042), (0.03, 0.058), (0.08, 0.063), (0.12, 0.054), (0.148, 0.034), (0.158, 0.005), (0.15, -0.018), (0.12, -0.028), (0.06, -0.033), (0.0, -0.028)]
SKIRT = [(-0.015, 0.02), (0.03, 0.026), (0.06, 0.02), (0.078, 0.0), (0.06, -0.02), (0.03, -0.026), (-0.015, -0.02)]


def rep50():
    d = Draw(None, local=True)
    hat = [hatch("spine-lines", ("Spine direction lines", "Stachelrichtungslinien"), "lines", 72, 0.45, 0.1)]
    d.curve("head", "head", [(-0.022, 0.02), (0.0, 0.03), (0.03, 0.03), (0.05, 0.012), (0.064, -0.004), (0.056, -0.014), (0.03, -0.022), (0.0, -0.025), (-0.022, -0.01)], 2.0, 0.45, regions=["face-fur", "nose"], corners=(4,))
    d.curve("neck", "neck", [(-0.025, 0.021), (0.035, 0.019), (0.035, -0.019), (-0.025, -0.021)], 1.9, 0.45, region="face-fur")
    d.curve("belly", "trunk", [(0.03, -0.028), (0.08, -0.036), (0.14, -0.028), (0.134, -0.052), (0.08, -0.06), (0.03, -0.054)], 3.0, 0.25, regions=["belly-fur", "limbs-fur", "paws-skin"])
    d.poly("dome", "trunk", teeth(DOME50, {1, 2, 3, 4, 5, 6, 7, 8}, 0.008), 3.5, 0.35, region="spines-dome", hatch="spine-lines")
    d.poly("skirt-front", "coat-f", teeth(SKIRT, {1, 2, 3, 4}, 0.007), 4.0, 0.3, region="spines-skirt", hatch="spine-lines")
    d.poly("skirt-rear", "coat-r", teeth(SKIRT, {1, 2, 3, 4}, 0.007), 4.0, 0.3, region="spines-skirt", hatch="spine-lines")
    dec = decisions(
        ("Full articulation, 0.285 x 0.13 m = 5.7 x 2.6 mm (foraging) and a closed ball of 0.14 m = 2.8 mm: spiny dome and both skirts with serrated outlines (tooth height 0.008 m), pointed head with snout, belly fur strip; the spine fringe hangs to 0.012 m above the ground and hides the legs, as on the real animal.", "Volle Gliederung, 0,285 x 0,13 m = 5,7 x 2,6 mm (Suchen) und eine geschlossene Kugel von 0,14 m = 2,8 mm: Stachelkuppel und beide Säume mit gezackter Kontur (Zahnhöhe 0,008 m), spitzer Kopf mit Schnauze, Bauchfellstreifen; der Stachelsaum hängt bis 0,012 m über den Boden und verdeckt die Beine, wie beim echten Tier."),
        ("The serrated spine edge, the hatch of spine direction lines on dome and skirts, the pointed snout, the belly fur strip below the spines.", "Die gezackte Stachelkante, die Schraffur der Stachelrichtung auf Kuppel und Säumen, die spitze Schnauze, der Bauchfellstreifen unter den Stacheln."),
        ("Limbs and paws (regions limbs-fur and paws-skin) and the nose are merged into the belly strip and the head shape; the neck merges into the face region.", "Gliedmaßen und Pfoten (Regionen limbs-fur und paws-skin) und die Nase sind in den Bauchfellstreifen und die Kopfform aufgenommen; der Hals geht in die Gesichtsregion auf."),
        ("All four limbs and paws (0.019-0.025 m wide: in the foraging, alert and rolled poses their outlines come within 0.15 mm of the spine skirts or dome in some pose), eye and nose dot (0.011 m), claws, ears, individual spines.", "Alle vier Gliedmaßen und Pfoten (0,019-0,025 m breit: in den Posen Suchen, Wachsam und Eingerollt kommen ihre Konturen in irgendeiner Pose näher als 0,15 mm an die Stachelsäume oder die Kuppel), Auge und Nasenpunkt (0,011 m), Krallen, Ohren, einzelne Stacheln."),
        ("The serration teeth (0.008 m) are filled spikes, not individual spines; the neck 0.04 m is the narrowest filled part (0.8 mm).", "Die Zacken (0,008 m) sind gefüllte Spitzen, keine einzelnen Stacheln; der 0,04 m breite Hals ist der schmalste gefüllte Teil (0,8 mm)."),
        ("One hatch: lines at 72 degrees, 0.45 mm spacing (0.0225 m), 0.1 mm pen, on dome and skirts as material indication of the spine direction.", "Eine Schraffur: Linien unter 72 Grad, 0,45 mm Abstand (0,0225 m), 0,1-mm-Stift, auf Kuppel und Säumen als Materialhinweis auf die Stachelrichtung."),
        ("0.25 mm silhouette, 0.1 mm hatch; spines mid-dark, belly light. The head is drawn behind the skirts so the rolled ball shows spines only.", "0,25-mm-Kontur, 0,1-mm-Schraffur; Stacheln mitteldunkel, Bauch hell. Der Kopf liegt hinter den Säumen, sodass die eingerollte Kugel nur Stacheln zeigt."),
        ("Dome, skirts, belly strip and head are separate shapes, so the thermal overlay shows the insulating spine layer against the bare belly; the limb and paw regions are painted with the belly strip value (merged).", "Kuppel, Säume, Bauchstreifen und Kopf sind eigene Formen, sodass das Wärmebild die dämmende Stachelschicht gegen den nackten Bauch zeigt; die Gliedmaßen- und Pfotenregionen werden mit dem Wert des Bauchstreifens gefärbt (zusammengeführt)."))
    return {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.35}, "hatches": hat, "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep100():
    d = Draw(None, local=True)
    hat = [hatch("spine-lines", ("Spine direction lines", "Stachelrichtungslinien"), "lines", 72, 0.5, 0.1)]
    d.curve("neck", "neck", [(-0.03, 0.024), (0.04, 0.022), (0.04, -0.022), (-0.03, -0.024)], 1.9, 0.45, region="face-fur")
    d.curve("head", "head", [(-0.03, 0.022), (0.0, 0.033), (0.035, 0.032), (0.055, 0.012), (0.068, -0.004), (0.058, -0.016), (0.03, -0.024), (-0.02, -0.026)], 2.0, 0.45, regions=["face-fur", "nose"], corners=(4,))
    d.curve("dome", "trunk", [(-0.032, -0.01), (-0.036, 0.02), (0.0, 0.05), (0.07, 0.064), (0.125, 0.052), (0.158, 0.012), (0.15, -0.034), (0.12, -0.056), (0.06, -0.062), (0.0, -0.05)], 3.5, 0.35, regions=["spines-dome", "belly-fur", "limbs-fur", "paws-skin"], hatch="spine-lines")
    d.curve("skirt-front", "coat-f", [(-0.02, 0.022), (0.03, 0.028), (0.06, 0.022), (0.08, 0.0), (0.06, -0.022), (0.03, -0.028), (-0.02, -0.022)], 4.0, 0.3, region="spines-skirt", hatch="spine-lines")
    d.curve("skirt-rear", "coat-r", [(-0.02, 0.022), (0.03, 0.028), (0.06, 0.022), (0.08, 0.0), (0.06, -0.022), (0.03, -0.028), (-0.02, -0.022)], 4.0, 0.3, region="spines-skirt", hatch="spine-lines")
    dec = decisions(
        ("Composed outline 0.285 x 0.13 m = 2.85 x 1.3 mm: smooth spiny dome with belly strip, two skirts and a pointed head with neck; closed ball 1.4 mm.", "Zusammengesetzter Umriss 0,285 x 0,13 m = 2,85 x 1,3 mm: glatte Stachelkuppel mit Bauchstreifen, zwei Säume und ein spitzer Kopf mit Hals; geschlossene Kugel 1,4 mm."),
        ("The two skirts as separate pieces so the roll-up articulates; spine direction hatch on dome and skirts; the pointed snout; the fringe reaching 0.012 m above the ground.", "Die zwei Säume als getrennte Stücke, damit das Einrollen gegliedert ist; Schraffur der Stachelrichtung auf Kuppel und Säumen; die spitze Schnauze; der Saum, der bis 0,012 m über den Boden reicht."),
        ("Belly strip, limbs and paws merge into the dome (regions spines-dome, belly-fur, limbs-fur, paws-skin), nose into the head; the neck overlaps head and trunk by 0.04 m.", "Bauchstreifen, Gliedmaßen und Pfoten gehen in der Kuppel auf (Regionen spines-dome, belly-fur, limbs-fur, paws-skin), die Nase im Kopf; der Hals überlappt Kopf und Rumpf um 0,04 m."),
        ("All limbs and paws (0.02-0.025 m: below the 0.2 mm limit or within 0.15 mm of the skirts), serrated spine edge, eye, nose dot, ears, claws.", "Alle Gliedmaßen und Pfoten (0,02-0,025 m: unter der 0,2-mm-Grenze oder näher als 0,15 mm an den Säumen), gezackte Stachelkante, Auge, Nasenpunkt, Ohren, Krallen."),
        ("No thin element is drawn; the narrowest filled part is the neck (0.05 m = 0.5 mm).", "Kein dünnes Element ist gezeichnet; der schmalste gefüllte Teil ist der Hals (0,05 m = 0,5 mm)."),
        ("One hatch: lines at 72 degrees, 0.5 mm spacing (0.05 m), 0.1 mm pen, as the spine direction on dome and skirts.", "Eine Schraffur: Linien unter 72 Grad, 0,5 mm Abstand (0,05 m), 0,1-mm-Stift, als Stachelrichtung auf Kuppel und Säumen."),
        ("0.18 mm silhouette; head and neck drawn behind the skirts.", "0,18-mm-Kontur; Kopf und Hals hinter den Säumen."),
        ("Merged shapes list their regions, so the thermal overlay paints the dome with the area-weighted value of spines, belly, limbs and paws.", "Zusammengeführte Formen listen ihre Regionen, sodass das Wärmebild die Kuppel mit dem flächengewichteten Wert von Stacheln, Bauch, Gliedmaßen und Pfoten färbt."))
    return {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}, "hatches": hat, "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep200():
    return {"status": "unavailable", "denominator": 200,
            "reason": tx("A foraging hedgehog is 0.285 m x 0.13 m = 1.4 x 0.65 mm on paper at 1:200, the rolled ball 0.14 m = 0.7 mm. The snout (0.03 m = 0.15 mm), the paws and limbs (0.02 m = 0.1 mm) and the spine edge are below the 0.2 mm feature width, and the dome, the two skirts and the head leave no room for the 0.15 mm ink gap between neighbouring outlines in the foraging, alert and rolled poses.",
                         "Ein Nahrung suchender Igel misst 0,285 m x 0,13 m = 1,4 x 0,65 mm auf dem Papier bei 1:200, die eingerollte Kugel 0,14 m = 0,7 mm. Schnauze (0,03 m = 0,15 mm), Pfoten und Beine (0,02 m = 0,1 mm) und die Stachelkante liegen unter der Merkmalsbreite von 0,2 mm, und Kuppel, beide Säume und Kopf lassen in den Posen Suchen, Wachsam und Eingerollt keinen Platz für den Tintenabstand von 0,15 mm zwischen benachbarten Konturen."),
            "limitingFeature": tx("Snout, limbs and spine edge (0.02-0.03 m = 0.1-0.15 mm)", "Schnauze, Beine und Stachelkante (0,02-0,03 m = 0,1-0,15 mm)"), "limitingSizeM": 0.03,
            "alternatives": [{"kind": "scale", "denominator": 100}, {"kind": "asset", "asset": "red-squirrel"}]}


def rep500():
    return {"status": "unavailable", "denominator": 500,
            "reason": tx("A foraging hedgehog is 0.27 m x 0.14 m = 0.54 x 0.28 mm on paper at 1:500, the rolled ball 0.14 m = 0.28 mm: the snout (0.03 m = 0.06 mm), limbs (0.02 m = 0.04 mm) and the spine edge cannot print below the 0.2 mm feature width of the office laser profile, so only a speck would remain.",
                         "Ein Nahrung suchender Igel misst 0,27 m x 0,14 m = 0,54 x 0,28 mm auf dem Papier bei 1:500, die eingerollte Kugel 0,14 m = 0,28 mm: Schnauze (0,03 m = 0,06 mm), Glieder (0,02 m = 0,04 mm) und Stachelkante sind unter der Merkmalsbreite von 0,2 mm des Büro-Laser-Profils nicht druckbar, es bliebe nur ein Fleck."),
            "limitingFeature": tx("Snout, limbs and spine edge (0.02-0.03 m = 0.04-0.06 mm)", "Schnauze, Glieder und Stachelkante (0,02-0,03 m = 0,04-0,06 mm)"), "limitingSizeM": 0.03,
            "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "asset", "asset": "red-squirrel"}]}
