"""🏭 Round 3, furniture/height-adjustable-desk: builds the candidate asset document (root fixes, the 2D of every scale when the drawing module provides it, volumes) into `🗑️generated/r3-height-adjustable-desk/🔣️.json` from the catalog file as it is now.

Usage: python3 r3-height-adjustable-desk-build.py [--keep-2d]
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/height-adjustable-desk/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r3-height-adjustable-desk")


def load(name):
    """📥 Imports a sibling script of this ticket as a module."""
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, f"r3-height-adjustable-desk-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


M = load("model")
K = M.K
T = K.text

AREAS = {"steel-desk": 1.6, "monitor-housing": 0.65, "controls": 0.08}
REFERENCES = [
    {"id": "uplift-v2-commercial-f600", "citation": "UPLIFT Desk (2023). UPLIFT V2-Commercial Standing Desk F600 (T-Frame), specification sheet SP-F600T-1.0: T-feet 27.6 in (0.70 m) long, three-stage columns, frame width 41.3 to 72.2 in, height range 21.6 to 47.7 in, stability crossbar."},
    {"id": "flexispot-e7", "citation": "FlexiSpot (2024). E7 / E7 Pro dual-motor standing desk frame, product data: three-stage rectangular columns 3.76 x 2.58 in (95 x 66 mm), frame net weight about 30 kg, motor in each column head."},
    {"id": "linak-dl21", "citation": "LINAK A/S (2024). DESKLINE DL21 lifting column, product data: three-stage column with a 60 x 90 mm profile."},
    {"id": "brateck-3-stage", "citation": "Brateck (2024). Three-stage electric sit-stand desk frame, product data: nested column profiles 40 x 80, 46 x 86 and 52.6 x 92.6 mm."},
]


def roots(a):
    """🌱 Root fixes outside `representations` and `volumes`."""
    m = K.masses()
    a["summary"] = T("A dual-motor sit-stand desk: two pressed T-feet each carry a three-tube telescoping column (two prismatic stages, identical chains left and right) with the motor housing at its head, side rails and a crossbar under a laminated 1.6 x 0.8 m top; a 27-inch monitor on its stand and a closed laptop stand on it. Front elevation, length along x.",
                     "Ein Sitz-Steh-Schreibtisch mit zwei Motoren: zwei gepresste T-Füße tragen je eine dreiteilige Teleskopsäule (zwei Schubstufen, links und rechts identische Ketten) mit dem Motorgehäuse am Kopf, Seitenzargen und ein Querträger unter einer laminierten Platte 1,6 x 0,8 m; ein 27-Zoll-Monitor auf seinem Ständer und ein geschlossener Laptop stehen darauf. Vorderansicht, Länge entlang x.")
    refs = a["provenance"]["references"]
    for ref in REFERENCES:
        if not any(x["id"] == ref["id"] for x in refs):
            refs.append(ref)
    a["provenance"]["notes"] = T("Front elevation along the 1.6 m top, seen from the user's side (the user sits at -z, screen and handset face him). Heights are the top surface (0.72, 0.935, 1.15 m) with the board 25 mm thick. Frame after current dual-motor frames (UPLIFT V2 F600, FlexiSpot E7, LINAK DL21, Brateck): 0.70 m T-feet, three nested rectangular tubes per column (64 x 94, 56 x 86, 48 x 78 mm), motor housing at the column head bolted to a 60 x 25 mm side rail, 50 x 30 mm crossbar; frame mass 31.3 kg from tube walls x 7850 kg/m3 plus 2.5 kg per drive. Equipment loads are authored estimates (27-inch monitor 8 kg, 15.6-inch laptop 2 kg) and both are drawn and modelled. Only the left column carries the top in the tree; the right column follows the same travel by pose.",
                                 "Vorderansicht entlang der 1,6-m-Platte, von der Nutzerseite gesehen (der Nutzer sitzt bei -z, Bildschirm und Bedienteil zeigen zu ihm). Höhen sind die Plattenoberkante (0,72; 0,935; 1,15 m), Plattenstärke 25 mm. Gestell nach aktuellen Zwei-Motor-Gestellen (UPLIFT V2 F600, FlexiSpot E7, LINAK DL21, Brateck): 0,70-m-T-Füße, drei ineinander laufende Rechteckrohre je Säule (64 x 94, 56 x 86, 48 x 78 mm), Motorgehäuse am Säulenkopf an eine Seitenzarge 60 x 25 mm geschraubt, Querträger 50 x 30 mm; Gestellmasse 31,3 kg aus Rohrwänden x 7850 kg/m3 plus 2,5 kg je Antrieb. Gerätelasten sind Schätzungen (27-Zoll-Monitor 8 kg, 15,6-Zoll-Laptop 2 kg); beide sind gezeichnet und modelliert. Nur die linke Säule trägt die Platte im Baum; die rechte folgt per Pose demselben Hub.")
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    for side, title, stitle, ctitle in (("left", T("Left T-foot with the outer column tube", "Linker T-Fuß mit äußerem Säulenrohr"), T("Left middle telescope stage", "Linke mittlere Teleskopstufe"), T("Left inner column with the motor housing", "Linke Innensäule mit Motorgehäuse")),
                                        ("right", T("Right T-foot with the outer column tube", "Rechter T-Fuß mit äußerem Säulenrohr"), T("Right middle telescope stage", "Rechte mittlere Teleskopstufe"), T("Right inner column with the motor housing", "Rechte Innensäule mit Motorgehäuse"))):
        foot, stage, column = bones[f"foot-{side}"], bones[f"stage-{side}"], bones[f"column-{side}"]
        foot["title"], stage["title"], column["title"] = title, stitle, ctitle
        foot["lengthM"] = K.FOOT_W
        kg, cu, cv = m["foot"]
        foot["mass"] = {"kg": round(kg, 2), "comX": cu, "comY": round(cv, 3), "distribution": "point", "source": "literature", "reference": "en-1993-1-1"}
        stage["x"], stage["y"], stage["lengthM"] = K.FOOT_U, K.STAGE_Y, K.STAGE_LEN
        kg, cu, cv = m["stage"]
        stage["mass"] = {"kg": round(kg, 2), "comX": round(cu, 3), "comY": 0, "distribution": "uniform", "source": "literature", "reference": "en-1993-1-1"}
        column["x"], column["lengthM"] = K.COLUMN_U0, K.COLUMN_LEN
        kg, cu, cv = m["column"]
        column["mass"] = {"kg": round(kg, 2), "comX": round(cu, 3), "comY": 0, "distribution": "point", "source": "literature", "reference": "en-1993-1-1"}
    bones["foot-left"]["x"] = K.FOOT_JOINT_X[0]
    bones["foot-right"]["x"] = round(K.FOOT_JOINT_X[1] - K.FOOT_JOINT_X[0], 4)
    top = bones["top"]
    top["x"], top["y"], top["lengthM"] = K.COLUMN_LEN, K.COLUMN_X[0], K.BOARD[0]
    top["title"] = T("Desk top with side rails, crossbar, cable tray and controls", "Tischplatte mit Seitenzargen, Querträger, Kabelwanne und Steuerung")
    kg, cu, cv = m["top"]
    top["mass"] = {"kg": round(kg, 2), "comX": round(cu, 3), "comY": round(cv, 4), "distribution": "point", "source": "literature", "reference": "ashby-2011"}
    a["dimensions"]["massKg"] = round(sum(b.get("mass", {}).get("kg", 0) for b in a["rig"]["bones"]), 1)
    for anchor in a["anchors"]:
        if anchor["id"].startswith("contact-foot-"):
            anchor["x"] = K.FOOT_U
            anchor["footprintLengthM"] = K.FOOT_W
            anchor["footprintWidthM"] = K.FOOT_L
    for pose in a["poses"]:
        for load in pose["loads"]:
            load["x"] = K.MON_X if load["id"] == "monitor" else K.LAP_X
            if load["id"] == "laptop":
                load["title"] = T("Closed 15.6-inch laptop (2 kg)", "Geschlossener 15,6-Zoll-Laptop (2 kg)")
            else:
                load["title"] = T("27-inch monitor (8 kg) on its stand", "27-Zoll-Monitor (8 kg) auf seinem Ständer")
        reg = pose["thermal"]["regions"]
        if "controls" not in reg:
            items = list(reg.items())
            at = [k for k, _ in items].index("drive-right") + 1
            items.insert(at, ("controls", {"exposedFraction": 1.0, "sunlitFraction": 0.0}))
            pose["thermal"]["regions"] = dict(items)
    regions = a["thermal"]["regions"]
    if not any(x["id"] == "controls" for x in regions):
        at = next(i for i, x in enumerate(regions) if x["id"] == "drive-right") + 1
        regions.insert(at, {"id": "controls", "title": T("Control box and handset", "Steuerkasten und Bedienteil"), "kind": "surface", "model": "operating", "material": "equipment-housing", "areaM2": AREAS["controls"], "characteristicLengthM": 0.085, "geometry": "horizontal-plate-down", "skyViewFactor": 0.2,
                                "layers": [{"id": "housing", "title": T("Plastic housing 2.5 mm with electronics", "Kunststoffgehäuse 2,5 mm mit Elektronik"), "kind": "polymer", "material": "equipment-housing", "thicknessM": 0.0025, "resistanceM2KW": 0.01, "source": "authored-estimate", "heatCapacityJM2K": 6000}],
                                "heatCapacityJM2K": 6000, "operating": {"key": "lift-drive", "heatW": 6}, "ratedPowerW": 160, "ratedPowerBasis": "electrical-input", "source": "authored-estimate"})
    for x in regions:
        if x["id"] in AREAS:
            x["areaM2"] = AREAS[x["id"]]
        if x["id"] == "steel-desk":
            x["title"] = T("Steel feet, column tubes, side rails, crossbar and cable tray", "Stahlfüße, Säulenrohre, Seitenzargen, Querträger und Kabelwanne")
        if x["id"] in ("drive-left", "drive-right"):
            side = "Left" if x["id"] == "drive-left" else "Right"
            x["title"] = T(f"{side} motor housing at the column head", ("Linkes" if side == "Left" else "Rechtes") + " Motorgehäuse am Säulenkopf")
    ta = a["thermal"]["assumptions"]
    ta[0] = T("Indoor reference case: no sun, 22 degC air and radiant temperature; no authored temperatures. Operating regions release heat into their own surface: the drives and the control box only while lifting (level 1), the monitor while on.", "Innenraum-Referenzfall: keine Sonne, 22 degC Luft- und Strahlungstemperatur; keine festgelegten Temperaturen. Betriebsregionen geben Wärme an ihre eigene Oberfläche ab: die Antriebe und der Steuerkasten nur beim Heben (Stufe 1), der Monitor im Betrieb.")
    ta[3] = T("Heat capacity of the motor housings at the column heads is a 0.8 kg steel-and-copper motor mass over 0.05 m2 (7.2 kJ/m2K); the control box (160 W switched-mode supply and motor control, about 6 W loss while lifting) carries 0.5 kg over 0.08 m2 (6 kJ/m2K); other capacities are rho*cp*min(thickness, sqrt(alpha*3600 s)) with ISO 10456 values.", "Die Wärmekapazität der Motorgehäuse an den Säulenköpfen ist eine Motormasse von 0,8 kg Stahl und Kupfer auf 0,05 m2 (7,2 kJ/m2K); der Steuerkasten (160-W-Schaltnetzteil und Motorsteuerung, etwa 6 W Verlust beim Heben) trägt 0,5 kg auf 0,08 m2 (6 kJ/m2K); die übrigen Kapazitäten sind rho*cp*min(Dicke, sqrt(alpha*3600 s)) mit ISO-10456-Werten.")
    a["structure"]["assumptions"] = [T("Rigid desk on two foot contacts (the glide pairs of each 0.70 m T-foot): each column is three nested steel tubes sliding in plastic bushings, the inner tube carries the motor housing, which is bolted to a side rail under the board; side rails, the crossbar between the motor housings and the board form the rigid top that carries its own weight, the monitor and the laptop in equilibrium with the feet.",
                                       "Starrer Tisch auf zwei Fußkontakten (den Gleiterpaaren jedes 0,70-m-T-Fußes): jede Säule besteht aus drei ineinander in Kunststoffbuchsen gleitenden Stahlrohren, das Innenrohr trägt das Motorgehäuse, das an eine Seitenzarge unter der Platte geschraubt ist; Seitenzargen, der Querträger zwischen den Motorgehäusen und die Platte bilden die starre Platte, die ihr Eigengewicht, den Monitor und den Laptop im Gleichgewicht mit den Füßen trägt.")]


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    a = doc["asset"]
    roots(a)
    if "--keep-2d" not in sys.argv:
        D = load("draw")
        for key, rep in a["representations"].items():
            rep["geometry"] = D.geometry(int(key))
            rep["decisions"] = D.DECISIONS[int(key)]
    a["volumes"] = M.volumes()
    os.makedirs(OUT, exist_ok=True)
    json.dump(doc, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print(os.path.join(OUT, "🔣️.json"))


if __name__ == "__main__":
    main()
