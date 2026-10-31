"""red-squirrel representations (hand-entered, bone-local outlines)"""
from nw import *

RUMP = [(-0.035, 0.0), (-0.03, 0.05), (0.02, 0.068), (0.07, 0.052), (0.115, 0.042), (0.115, -0.042), (0.07, -0.056), (0.0, -0.062), (-0.03, -0.04)]
CHEST = [(-0.03, 0.042), (0.04, 0.05), (0.1, 0.044), (0.125, 0.0), (0.1, -0.046), (0.04, -0.052), (-0.03, -0.044)]
SKULL = [(-0.02, 0.018), (-0.005, 0.034), (0.03, 0.036), (0.055, 0.02), (0.067, 0.0), (0.06, -0.016), (0.03, -0.028), (0.0, -0.03), (-0.02, -0.01)]


def rep50():
    d = Draw(None, local=True)
    for s, z, t in (("far", 1.0, 0.5), ("near", 4.0, 0.35)):
        b = "f" if s == "far" else "n"
        d.curve(f"{s}-arm", f"arm-{b}", [(-0.012, 0.0125), (0.05, 0.011), (0.054, 0.0), (0.05, -0.011), (-0.012, -0.0125)], z, t, region="arms-fur", corners=(2,))
        d.curve(f"{s}-forearm", f"forearm-{b}", [(-0.012, 0.0115), (0.05, 0.01), (0.054, 0.0), (0.05, -0.01), (-0.012, -0.0115)], z, t, region="arms-fur", corners=(2,))
        d.curve(f"{s}-hand", f"hand-{b}", [(-0.012, 0.0085), (0.034, 0.0075), (0.042, 0.0), (0.034, -0.0075), (-0.012, -0.0085)], z + 0.1, t + 0.15, region="hands-skin", corners=(2,))
        d.curve(f"{s}-thigh", f"thigh-{b}", [(-0.025, 0.03), (0.04, 0.036), (0.08, 0.016), (0.075, -0.016), (0.03, -0.032), (-0.025, -0.026)], z + 0.2, t, region="legs-fur")
        d.curve(f"{s}-shank", f"shank-{b}", [(-0.012, 0.0145), (0.07, 0.011), (0.082, 0.0), (0.07, -0.011), (-0.012, -0.0145)], z + 0.2, t, region="legs-fur", corners=(2,))
        d.curve(f"{s}-foot", f"foot-{b}", [(-0.012, 0.0115), (0.04, 0.0105), (0.066, 0.005), (0.07, -0.003), (0.066, -0.011), (0.0, -0.0115), (-0.012, -0.004)], z + 0.2, t + 0.2, region="feet-skin", corners=(3,))
    d.curve("tail-1", "tail-1", [(-0.025, 0.03), (0.05, 0.05), (0.105, 0.044), (0.105, -0.044), (0.05, -0.05), (-0.025, -0.03)], 0.5, 0.45, region="tail-fur")
    d.curve("tail-2", "tail-2", [(-0.03, 0.045), (0.05, 0.058), (0.115, 0.042), (0.13, 0.0), (0.115, -0.042), (0.05, -0.058), (-0.03, -0.045)], 0.4, 0.45, region="tail-fur")
    d.curve("rump", "spine-r", RUMP, 3.0, 0.38, region="fur-back")
    d.curve("chest", "spine-f", CHEST, 3.1, 0.4, region="fur-back")
    d.curve("belly-patch", "spine-f", [(0.0, -0.03), (0.06, -0.052), (0.115, -0.04), (0.118, -0.012), (0.06, -0.03)], 3.2, 0.05, region="fur-belly")
    d.curve("neck", "neck", [(-0.025, 0.03), (0.045, 0.025), (0.045, -0.025), (-0.025, -0.03)], 3.3, 0.38, region="head-fur")
    d.curve("head", "head", SKULL, 4.5, 0.4, regions=["head-fur", "nose"])
    d.curve("ear", "head", [(-0.004, 0.028), (-0.012, 0.062), (0.004, 0.082), (0.022, 0.04)], 4.4, 0.3, region="ears", corners=(2,))
    dec = decisions(
        ("Full articulation 0.21 x 0.27 m = 4.2 x 5.4 mm: red-brown squirrel with a white chest patch, bushy two-part tail, ear tuft, hands and hind feet in bare-skin tone.", "Volle Gliederung 0,21 x 0,27 m = 4,2 x 5,4 mm: rotbraunes Eichhörnchen mit weißem Brustfleck, buschigem zweiteiligem Schwanz, Ohrpinsel, Händen und Hinterfüßen in nackter Hauttönung."),
        ("Both fore limbs in upper arm, forearm and hand, both hind limbs in thigh, shank and foot, rump and chest, neck, skull, ear, white belly patch.", "Beide Vorderglieder in Ober-, Unterarm und Hand, beide Hinterglieder in Ober-, Unterschenkel und Fuß, Becken und Brust, Hals, Schädel, Ohr, weißer Bauchfleck."),
        ("The nose merges into the skull shape (regions head-fur and nose); whiskers, claws and fur are not drawn.", "Die Nase geht in der Schädelform auf (Regionen head-fur und nose); Schnurrhaare, Krallen und Fell sind nicht gezeichnet."),
        ("Eye (0.008 m = 0.16 mm) and the tail hair line (both would lie within 0.15 mm of neighbouring outlines in some pose), ear tuft hairs, toes, individual hairs, whiskers.", "Auge (0,008 m = 0,16 mm) und die Schwanzhaarlinie (beide läge in irgendeiner Pose näher als 0,15 mm an Nachbarkonturen), Haare des Ohrpinsels, Zehen, einzelne Haare, Schnurrhaare."),
        ("Hands 0.015 m, shank 0.022 m and foot 0.022 m are filled at true width (0.3-0.44 mm), above the 0.2 mm limit.", "Hände 0,015 m, Unterschenkel 0,022 m und Fuß 0,022 m sind in wahrer Breite gefüllt (0,3-0,44 mm), über der 0,2-mm-Grenze."),
        ("None: the 0.4 mm minimum hatch spacing equals 0.02 m, a third of the body width.", "Keine: der Mindestabstand von 0,4 mm entspricht 0,02 m, einem Drittel der Körperbreite."),
        ("0.25 mm silhouette, no detail pen; tones: back mid, tail darker, white patch lightest.", "0,25-mm-Kontur, kein Detailstift; Töne: Rücken mittel, Schwanz dunkler, weißer Fleck am hellsten."),
        ("Every region has its own shape except the nose, which is listed on the skull shape: fur, bare hands and feet and ears are separate in the thermal overlay.", "Jede Region hat eine eigene Form außer der Nase, die an der Schädelform geführt wird: Fell, nackte Hände und Füße sowie Ohren sind im Wärmebild getrennt."))
    return {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [], "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep100():
    d = Draw(None, local=True)
    d.curve("tail-1", "tail-1", [(-0.03, 0.032), (0.05, 0.052), (0.106, 0.046), (0.106, -0.046), (0.05, -0.052), (-0.03, -0.032)], 0.5, 0.45, region="tail-fur")
    d.curve("tail-2", "tail-2", [(-0.06, 0.05), (0.05, 0.06), (0.118, 0.044), (0.135, 0.0), (0.118, -0.044), (0.05, -0.06), (-0.06, -0.05)], 0.4, 0.45, region="tail-fur")
    d.curve("rump", "spine-r", [(-0.04, 0.0), (-0.035, 0.054), (0.02, 0.07), (0.075, 0.054), (0.12, 0.044), (0.12, -0.044), (0.07, -0.058), (0.0, -0.064), (-0.035, -0.042)], 3.0, 0.38, regions=["fur-back", "legs-fur", "feet-skin"])
    d.curve("chest", "spine-f", [(-0.06, 0.044), (0.04, 0.052), (0.1, 0.046), (0.13, 0.0), (0.1, -0.048), (0.04, -0.054), (-0.06, -0.046)], 3.1, 0.4, regions=["fur-back", "fur-belly", "arms-fur", "hands-skin"])
    d.curve("neck", "neck", [(-0.05, 0.03), (0.05, 0.026), (0.05, -0.026), (-0.05, -0.03)], 3.3, 0.38, region="head-fur")
    d.curve("head", "head", [(-0.05, 0.02), (-0.03, 0.036), (0.03, 0.038), (0.056, 0.022), (0.072, 0.0), (0.06, -0.018), (0.03, -0.03), (-0.03, -0.03)], 4.5, 0.4, regions=["head-fur", "nose"], corners=(4,))
    d.curve("ear", "head", [(-0.01, 0.026), (-0.016, 0.066), (0.006, 0.086), (0.026, 0.04)], 4.4, 0.3, region="ears", corners=(2,))
    dec = decisions(
        ("Composed outline 0.21 x 0.275 m = 2.1 x 2.75 mm: two-part bushy tail, rump, chest, neck, head and ear as overlapping filled masses; no limbs.", "Zusammengesetzter Umriss 0,21 x 0,275 m = 2,1 x 2,75 mm: zweiteiliger buschiger Schwanz, Becken, Brust, Hals, Kopf und Ohr als überlappende gefüllte Massen; keine Glieder."),
        ("The bushy tail, the ear tuft and the upright haunch-and-chest block that identify a squirrel; the sitting posture.", "Der buschige Schwanz, der Ohrpinsel und der aufrechte Block aus Keule und Brust, die ein Eichhörnchen kennzeichnen; die Sitzhaltung."),
        ("Fore limbs and hands are merged into the chest shape (regions fur-back, fur-belly, arms-fur, hands-skin), hind limbs and feet into the rump (fur-back, legs-fur, feet-skin), nose into the head; neck and tail pieces overlap their neighbours by 0.05-0.06 m.", "Vorderglieder und Hände sind in die Brustform aufgenommen (Regionen fur-back, fur-belly, arms-fur, hands-skin), Hinterglieder und Füße in das Becken (fur-back, legs-fur, feet-skin), die Nase in den Kopf; Hals- und Schwanzstücke überlappen ihre Nachbarn um 0,05-0,06 m."),
        ("All limbs, hands and feet (0.016-0.024 m = 0.16-0.24 mm; with four limbs in a 2 mm figure every limb outline comes within 0.15 mm of a neighbouring outline in some pose), eye, whiskers, claws.", "Alle Glieder, Hände und Füße (0,016-0,024 m = 0,16-0,24 mm; bei vier Gliedern in einer 2-mm-Figur kommt jede Gliedkontur in irgendeiner Pose näher als 0,15 mm an eine Nachbarkontur), Auge, Schnurrhaare, Krallen."),
        ("No thin element is drawn; the narrowest filled part is the neck (0.05 m = 0.5 mm).", "Kein dünnes Element ist gezeichnet; der schmalste gefüllte Teil ist der Hals (0,05 m = 0,5 mm)."),
        ("None.", "Keine."),
        ("0.18 mm silhouette, no detail pen; tones back mid, tail dark, ear light.", "0,18-mm-Kontur, kein Detailstift; Töne Rücken mittel, Schwanz dunkel, Ohr hell."),
        ("Merged shapes list their regions; the thermal overlay shows the area-weighted value of limbs and body together.", "Zusammengeführte Formen listen ihre Regionen; das Wärmebild zeigt den flächengewichteten Wert von Gliedern und Körper zusammen."))
    return {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [], "geometry": {"elevation": d.shapes}, "decisions": dec}


def rep200():
    return {"status": "unavailable", "denominator": 200,
            "reason": tx("A sitting red squirrel is 0.215 m x 0.27 m = 1.1 x 1.35 mm on paper at 1:200. Its limbs (0.016-0.024 m = 0.08-0.12 mm), ear tuft, eye and nose are below the 0.2 mm feature width, and the filled masses of tail, body and head that remain are too close together for the 0.15 mm ink gap in some pose (running, climbing). Without limbs the animal would not show how it sits, runs or climbs.",
                         "Ein sitzendes Eichhörnchen misst 0,215 m x 0,27 m = 1,1 x 1,35 mm auf dem Papier bei 1:200. Seine Glieder (0,016-0,024 m = 0,08-0,12 mm), Ohrpinsel, Auge und Nase liegen unter der Merkmalsbreite von 0,2 mm, und die verbleibenden gefüllten Massen von Schwanz, Körper und Kopf liegen in manchen Posen (Laufen, Klettern) zu dicht für den Tintenabstand von 0,15 mm. Ohne Glieder zeigte das Tier nicht, wie es sitzt, läuft oder klettert."),
            "limitingFeature": tx("Limbs and head details (0.016-0.024 m = 0.08-0.12 mm)", "Glieder und Kopfdetails (0,016-0,024 m = 0,08-0,12 mm)"), "limitingSizeM": 0.02,
            "alternatives": [{"kind": "scale", "denominator": 100}, {"kind": "asset", "asset": "domestic-cat"}]}


def rep500():
    return {"status": "unavailable", "denominator": 500,
            "reason": tx("A sitting red squirrel is 0.21 m x 0.27 m = 0.4 x 0.5 mm on paper at 1:500: body depth 0.09 m = 0.18 mm, head 0.05 m = 0.1 mm, limbs 0.016 m = 0.03 mm and the whole animal covers a quarter of a square millimetre. Nothing identifiable can print below the 0.2 mm feature width of the office laser profile.",
                         "Ein sitzendes Eichhörnchen misst 0,21 m x 0,27 m = 0,4 x 0,5 mm auf dem Papier bei 1:500: Körpertiefe 0,09 m = 0,18 mm, Kopf 0,05 m = 0,1 mm, Glieder 0,016 m = 0,03 mm; das ganze Tier bedeckt ein Viertel Quadratmillimeter. Unter der Merkmalsbreite von 0,2 mm des Büro-Laser-Profils ist nichts Erkennbares druckbar."),
            "limitingFeature": tx("Head and limbs (0.06 m = 0.12 mm, 0.02 m = 0.04 mm)", "Kopf und Glieder (0,06 m = 0,12 mm, 0,02 m = 0,04 mm)"), "limitingSizeM": 0.06,
            "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "asset", "asset": "domestic-cat"}]}
