"""🌱 Round 3, furniture/operating-table: the root corrections outside `volumes` and the drawn shapes: rig (tilt pin 40 mm under the frame, bones that run along their parts: both columns upright, the arm bone along the arm), castor anchors, overall width over the accessory rails, references, notes and the texts that name measures.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_operating_table_kit", os.path.join(HERE, "r3-operating-table-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

DEPTH = 2 * K.RAIL_Z1
AREAS = {"table-frame": 2.64, "table-column": 0.75, "table-base": 1.06, "light-housing": 0.5, "stand": 0.96}

BONES = {
    "base": {"lengthM": K.BASE_HALF_X},
    "column": {"angleDeg": 90, "lengthM": K.r6(K.PIVOT_Y - K.PLINTH_Y1), "comX": 0.3, "comY": 0},
    "top": {"x": K.r6(K.PIVOT_Y - K.PLINTH_Y1), "y": 0, "angleDeg": -90, "lengthM": 1.0, "comY": 0.11},
    "stand-base": {"lengthM": 0.28, "comY": 0.1},
    "stand-column": {"y": K.BOX_Y1, "angleDeg": 90, "lengthM": K.r6(K.ARM_Y - K.BOX_Y1), "comX": 0.9, "comY": 0},
    "stand-arm": {"x": K.r6(K.ARM_Y - K.BOX_Y1), "y": 0, "angleDeg": 90, "lengthM": K.ARM_LEN, "comX": 0.4},
    "light-head": {"x": K.ARM_LEN, "y": K.r6(K.ARM_Y - K.HEAD_Y), "angleDeg": 180, "lengthM": K.HEAD_R, "comY": 0.02},
}

REFERENCES = [
    {"id": "schmitz-opx-mobilis", "citation": "Schmitz u. Soehne, mobile operating tables OPX mobilis, catalogue 60: table top 540 mm wide, height 690 to 1050 mm, base plate 1120 x 580 mm, 2130 mm long with head and leg sections (manufacturer data, read 2026-11-20).", "url": "https://cdn.shopify.com/s/files/1/0044/7319/2566/files/schmitz_kat60_en.pdf"},
    {"id": "medifa-6000", "citation": "medifa, operating table 6000, brochure 2018: height 690 to 1050 mm, floor footprint 940 x 540 mm, table weight about 200 kg (manufacturer data, read 2026-11-20).", "url": "https://rynamedical.co.uk/s/BROCHURE-6000-Medifa-Operating-Table-2018.pdf"},
    {"id": "mach-led150fp", "citation": "Dr. Mach, operating light LED 150 FP on a mobile stand, as listed in the MSF catalogue: stand height 1.88 m, spring arm 0.805 m, light head height 0.9 to 2.08 m (read 2026-11-20).", "url": "https://unicat.msf.org/cat/product/65939"},
]

SUMMARY = {
    "en": "A mobile operating table 2.0 m long with a five-part padded top 0.5 m wide (0.56 m over the accessory rails) at 0.90 m on a telescopic column and an H-shaped castor chassis, and a mobile surgical light whose 0.8 m spring arm carries a 120 W lamp head of 0.5 m over the table; the top tilts to head down, the lamp head is the only warm surface.",
    "de": "Ein fahrbarer Operationstisch von 2,0 m Länge mit einer fünfteiligen gepolsterten Platte von 0,5 m Breite (0,56 m über die Zubehörschienen) auf 0,90 m, auf einer Teleskopsäule und einem H-förmigen Rollenfahrgestell, und eine fahrbare Operationsleuchte, deren 0,8 m langer Federarm einen 120-W-Leuchtenkopf von 0,5 m über den Tisch trägt; die Platte kippt in Kopftieflage, der Leuchtenkopf ist die einzige warme Fläche.",
}

NOTES = {
    "en": "Mobile operating table and mobile surgical light, head end at x = -1.0 m. Table: top surface at 0.90 m (adjustable 0.7 to 1.05 m on real columns, drawn at 0.90 m), 2.0 m long, pads 0.5 m wide and 80 mm thick on 40 mm stainless section frames, accessory rails 25 x 10 mm at 0.56 m overall width; five parts: head plate 0.265 x 0.30 m on two rods, back 0.475 m, seat 0.47 m, two leg plates 0.735 m. The seat section hangs with two saddle cheeks on the tilt pin of the column head at 0.74 m, 40 mm under the frame, and tilts by up to 25 degrees. Telescopic column of three stainless shrouds (0.30 x 0.26 m at the foot) on an H-shaped chassis 0.94 x 0.54 m (footprint of the medifa 6000) with four twin castors of 100 mm at x = +-0.40 m, z = +-0.21 m. Light: stand with two legs 0.48 m long on twin castors of 75 mm, ballast box, a 50 mm tube to the bearing head at 1.96 m, a 0.8 m spring arm at 1.92 m (Dr. Mach LED 150 FP: stand 1.88 m, arm 0.805 m), a half yoke of 25 mm tube and a lamp head of 0.5 m diameter whose lens is 1.66 m above the floor (0.76 m working distance, inside the 0.7 to 1.4 m of IEC 60601-2-41 lights), over the leg plates at x = 0.37 m; the stand column stands 0.17 m beyond the foot end. Masses (table 210 kg, stand 38 kg) are rounded from this product class (authored estimate). The origin is on the floor under the table column. The light takes 120 W at level 1, which ends entirely as heat in the room: 90 W at the housing and 30 W at the lens, a split that is an authored estimate.",
    "de": "Fahrbarer Operationstisch und fahrbare Operationsleuchte, Kopfende bei x = -1,0 m. Tisch: Oberfläche auf 0,90 m (an echten Säulen 0,7 bis 1,05 m verstellbar, gezeichnet bei 0,90 m), 2,0 m lang, Polster 0,5 m breit und 80 mm stark auf 40 mm hohen Edelstahl-Segmentrahmen, Zubehörschienen 25 x 10 mm bei 0,56 m Gesamtbreite; fünf Teile: Kopfplatte 0,265 x 0,30 m auf zwei Stäben, Rücken 0,475 m, Sitz 0,47 m, zwei Beinplatten 0,735 m. Das Sitzsegment hängt mit zwei Sattelwangen am Kippbolzen des Säulenkopfes auf 0,74 m, 40 mm unter dem Rahmen, und kippt bis 25 Grad. Teleskopsäule aus drei Edelstahlhüllen (am Fuß 0,30 x 0,26 m) auf einem H-förmigen Fahrgestell 0,94 x 0,54 m (Standfläche des medifa 6000) mit vier Doppelrollen von 100 mm bei x = +-0,40 m, z = +-0,21 m. Leuchte: Ständer mit zwei 0,48 m langen Holmen auf Doppelrollen von 75 mm, Ballastkasten, ein 50-mm-Rohr bis zum Lagerkopf auf 1,96 m, ein 0,8 m langer Federarm auf 1,92 m (Dr. Mach LED 150 FP: Ständer 1,88 m, Arm 0,805 m), ein Halbbügel aus 25-mm-Rohr und ein Leuchtenkopf von 0,5 m Durchmesser, dessen Linse 1,66 m über dem Boden liegt (0,76 m Arbeitsabstand, innerhalb der 0,7 bis 1,4 m von Leuchten nach IEC 60601-2-41), über den Beinplatten bei x = 0,37 m; die Ständersäule steht 0,17 m hinter dem Fußende. Massen (Tisch 210 kg, Ständer 38 kg) sind aus dieser Produktklasse gerundet (Schätzwert). Der Ursprung liegt auf dem Boden unter der Tischsäule. Die Leuchte nimmt auf Stufe 1 120 W auf, die vollständig als Wärme im Raum enden: 90 W am Gehäuse und 30 W an der Linse, eine Aufteilung als Schätzwert.",
}

SUPPORT_ASSUMPTION = {
    "en": "A rigid assembly standing on four table castors (x = +-0.40 m, z = +-0.21 m) and four stand castors (x = 1.17 m +- 0.22 m, z = +-0.195 m); the locked castors carry the weight, the tilted top shifts the centre of mass inside the chassis footprint.",
    "de": "Eine starre Baugruppe auf vier Tischrollen (x = +-0,40 m, z = +-0,21 m) und vier Ständerrollen (x = 1,17 m +- 0,22 m, z = +-0,195 m); die arretierten Rollen tragen das Gewicht, die gekippte Platte verschiebt den Schwerpunkt innerhalb der Aufstandsfläche des Fahrgestells.",
}

STAND_ASSUMPTION = {
    "en": "The light stand stands beyond the foot end of the table (column at 1.17 m, the table top ends at 1.0 m), so neither the level nor the tilted top can touch the column; the 0.8 m spring arm reaches back over the leg plates.",
    "de": "Der Leuchtenständer steht hinter dem Fußende des Tisches (Säule bei 1,17 m, die Tischplatte endet bei 1,0 m), sodass weder die ebene noch die gekippte Platte die Säule berühren kann; der 0,8 m lange Federarm reicht zurück über die Beinplatten.",
}

COVER_TITLE = {"en": "Stainless steel shroud 2 mm", "de": "Edelstahlhülle 2 mm"}


def apply(asset):
    """🧰 Applies every root correction to the asset in memory."""
    asset["summary"] = dict(SUMMARY)
    asset["dimensions"]["depthM"] = DEPTH
    for bone in asset["rig"]["bones"]:
        for key, value in BONES[bone["id"]].items():
            if key in ("comX", "comY"):
                bone["mass"][key] = value
            else:
                bone[key] = value
    for anchor in asset["anchors"]:
        if anchor["id"].startswith("castor-table-"):
            anchor["x"] = K.CASTOR_X if "front" in anchor["id"] else -K.CASTOR_X
            anchor["z"] = K.CASTOR_Z if anchor["id"].endswith("left") else -K.CASTOR_Z
        if anchor["id"] == "light-top":
            anchor["y"] = 0.075
    known = {r["id"] for r in asset["provenance"]["references"]}
    asset["provenance"]["references"] += [r for r in REFERENCES if r["id"] not in known]
    asset["provenance"]["notes"] = dict(NOTES)
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "table-column":
            region["layers"][0]["title"] = dict(COVER_TITLE)
    asset["structure"]["assumptions"] = [dict(SUPPORT_ASSUMPTION), dict(STAND_ASSUMPTION)]
