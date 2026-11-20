"""🌱 Vacuum cleaner roots: the candidate asset document with the corrected rig, anchors, poses, dimensions, thermal areas, provenance, 2D drawings and the authored volumes, written to `🗑️generated/r3-vacuum-cleaner/🔣️.json` for validation before it goes into the catalog.

@see ./r3-vacuum-cleaner-poses.py
@see ./r3-vacuum-cleaner-draw.py
@see ./r3-vacuum-cleaner-apply.py
"""
import copy
import importlib.util
import json
from pathlib import Path


def _load(name, file):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(file))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit", "r3-vacuum-cleaner-kit.py")
D = _load("draw", "r3-vacuum-cleaner-draw.py")
T = K.text

AREAS = {"lid": 0.17, "hose": 0.18, "grip": 0.03, "brush": 0.014}
REFERENCES = [
    {"id": "miele-complete-c3", "citation": "Miele Complete C3 canister vacuum cleaner, technical data as listed by retailers (thebrick.com, read 2026-11-19): 49.6 cm long, 28.4 cm wide, 22.7 cm high, operating radius up to 12 m."},
    {"id": "otto-blue-vac-power", "citation": "Canister vacuum cleaner Blue Vac Power, product data sheet (d.otto.de, read 2026-11-19): appliance 46.9 x 31.5 x 25.5 cm, suction hose 1.70 m, telescopic tube 0.90 m."},
    {"id": "miele-sbd-285-3", "citation": "Miele AllTeQ combination floor brush SBD 285-3 for 35 mm tubes, retailer data (read 2026-11-19): about 285 mm working width, about 30 mm head height."},
]
NOTES = T(
    "Side view of a typical canister vacuum cleaner in use (no single product): canister 0.50 m long, 0.25 m wide (0.30 m over the rear wheels) and 0.28 m high to the carry handle, as the class of Miele Complete C3 (0.496 x 0.284 x 0.227 m) and comparable canisters (0.47 x 0.32 x 0.26 m); rear wheels 0.12 m, a twin swivel caster 0.05 m; a 1.5 m corrugated hose of 36 mm (retail hoses 1.5 to 1.8 m); a 0.2 m handle with air slide on a hose swivel, a telescopic wand of 35 and 32 mm tubes and the 0.14 m elbow of the floor nozzle, together 0.86 m from the handle socket to the nozzle pivot; a combination floor nozzle 0.285 m wide, 83 mm deep and 30 mm high with rubber end caps and two bristle strips. In use the handle is held at about 0.97 m. The rated input of 700 W is below the 900 W limit of Regulation (EU) 666/2013. The whole 700 W ends up in the room: 110 W warm the shell, 40 W the lid, 7 W the grille and 543 W the exhaust air (0.030 m3/s at 15 K rise); the split is an authored estimate. Intake air and dust are not modelled.",
    "Seitenansicht eines typischen Bodenstaubsaugers im Gebrauch (kein einzelnes Produkt): Kessel 0,50 m lang, 0,25 m breit (0,30 m über die Hinterräder) und 0,28 m hoch bis zum Tragegriff, wie die Klasse des Miele Complete C3 (0,496 x 0,284 x 0,227 m) und vergleichbarer Geräte (0,47 x 0,32 x 0,26 m); Hinterräder 0,12 m, eine Doppel-Lenkrolle 0,05 m; ein gewellter Schlauch von 1,5 m und 36 mm (handelsübliche Schläuche 1,5 bis 1,8 m); ein 0,2 m langer Handgriff mit Nebenluftschieber auf einem Schlauchdrehgelenk, ein Teleskoprohr aus Rohren von 35 und 32 mm und der 0,14 m lange Krümmer der Bodendüse, zusammen 0,86 m vom Griffstutzen bis zum Düsengelenk; eine Kombi-Bodendüse von 0,285 m Breite, 83 mm Tiefe und 30 mm Höhe mit Gummi-Endkappen und zwei Borstenleisten. Im Gebrauch wird der Griff in etwa 0,97 m Höhe gehalten. Die Nennaufnahme von 700 W liegt unter der Grenze von 900 W der Verordnung (EU) 666/2013. Die gesamten 700 W landen im Raum: 110 W erwärmen die Schale, 40 W den Deckel, 7 W das Gitter und 543 W die Abluft (0,030 m3/s bei 15 K Anstieg); die Aufteilung ist ein Schätzwert. Ansaugluft und Staub sind nicht modelliert.",
)


def rig(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    b = bones["body"]
    b["mass"]["comX"], b["mass"]["comY"] = -0.05, 0.10
    bones["wheel"]["x"] = K.WHEEL[0]
    for i in range(K.HOSE_N):
        h = bones[f"hose-{i + 1}"]
        if i > 0:
            h["x"] = K.HOSE_SEG
        h["lengthM"] = K.HOSE_SEG
        h["mass"]["comX"] = K.HOSE_SEG / 2
    g = bones["grip"]
    g["x"], g["lengthM"] = K.HOSE_SEG, K.GRIP_L
    g["title"] = T("Handle with air slide on the hose swivel", "Handgriff mit Nebenluftschieber am Schlauchdrehgelenk")
    g["mass"]["comX"] = 0.10
    w = bones["wand"]
    w["x"], w["lengthM"] = K.WAND_X, K.WAND_L
    w["title"] = T("Telescopic wand with nozzle elbow", "Teleskoprohr mit Düsenkrümmer")
    w["mass"]["comX"] = 0.40
    n = bones["nozzle"]
    n["x"], n["lengthM"] = K.WAND_L, 0.05
    n["mass"]["comX"], n["mass"]["comY"] = 0.005, -0.022


def anchors(asset):
    a = {x["id"]: x for x in asset["anchors"]}
    a["wheel-contact"]["footprintWidthM"] = 0.05
    a["caster-contact"]["footprintWidthM"] = 0.024
    nz = a["nozzle-contact"]
    nz["x"], nz["y"], nz["footprintLengthM"], nz["footprintWidthM"] = 0.0045, K.NOZZLE_FLOOR, 0.083, 0.285
    ex = a["exhaust-port"]
    ex["x"], ex["y"] = -0.258, 0.121


def poses(asset):
    solved = K.load(K.GEN / "poses.json")
    for p in asset["poses"]:
        for bone, v in solved[p["id"]].items():
            p["bones"][bone] = v


def thermal(asset):
    for r in asset["thermal"]["regions"]:
        if r["id"] in AREAS:
            r["areaM2"] = AREAS[r["id"]]


def provenance(asset):
    pv = asset["provenance"]
    have = {r["id"] for r in pv["references"]}
    pv["references"] += [r for r in REFERENCES if r["id"] not in have]
    pv["notes"] = NOTES


def representations(asset):
    for sc in K.SCALES:
        rep = asset["representations"][str(sc)]
        rep["geometry"] = D.geometry(sc)
        rep["decisions"] = D.DECISIONS[sc]


def build():
    doc = K.load(K.CATALOG)
    a = doc["asset"]
    a["dimensions"]["heightM"] = 0.99
    a["dimensions"]["widthM"] = 1.35
    a["dimensions"]["depthM"] = 0.30
    rig(a)
    anchors(a)
    poses(a)
    thermal(a)
    provenance(a)
    representations(a)
    a["volumes"] = K.load(K.SOURCE)["volumes"]
    return doc


def main():
    doc = build()
    K.GEN.mkdir(parents=True, exist_ok=True)
    out = K.GEN / "🔣️.json"
    out.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(out)


if __name__ == "__main__":
    main()
