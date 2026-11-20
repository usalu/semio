"""🌱️ Root fixes of vehicles/vintage-car-1929 outside `volumes`: rig, dimensions, anchor, texts and the redrawn 2D shapes. Writes the candidate document (root fixes plus the volumes source) to 🗑️generated/r3-vintage-car-1929/🔣️.json; with --apply writes the catalog file (re-read right before, only the named members replaced, every other byte kept). Usage: python3 r3-vintage-car-1929-root.py [--apply]"""
import os, sys, re
from decimal import Decimal

HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "r3-vintage-car-1929-kit.py")).read())
exec(open(os.path.join(HERE, "r3-vintage-car-1929-2d.py")).read())
exec(open(os.path.join(HERE, "r3-vintage-car-1929-decisions.py"), encoding="utf-8").read())
PATH = CATALOG_V + SLUG + "/🔣️.json"
OUT = os.path.join(HERE, "🗑️generated", "r3-vintage-car-1929")


def num(v):
    s = repr(v)
    if "e" in s or "E" in s:
        s = format(Decimal(s), "f")
    return s[:-2] if s.endswith(".0") else s


def dump(v, depth):
    pad = " " * depth
    if isinstance(v, dict):
        if not v:
            return "{}"
        return "{\n" + ",\n".join(f"{pad} {json.dumps(k, ensure_ascii=False)}: {dump(x, depth + 1)}" for k, x in v.items()) + "\n" + pad + "}"
    if isinstance(v, list):
        if not v:
            return "[]"
        return "[\n" + ",\n".join(f"{pad} {dump(x, depth + 1)}" for x in v) + "\n" + pad + "]"
    if isinstance(v, float):
        return num(v)
    return json.dumps(v, ensure_ascii=False)


def span(text, key):
    a = text.index(f'\n  "{key}": ') + 1
    m = re.compile(r'\n  "[A-Za-z$]+": |\n }\n').search(text, a)
    b = m.start()
    if text[b - 1] == ",":
        b -= 1
    return a, b


def member(text, key, value=None):
    a, b = span(text, key)
    if value is None:
        return text[a:b]
    return text[:a] + f'  "{key}": ' + dump(value, 2) + text[b:]


text = open(PATH, encoding="utf-8").read()
asset = json.loads(text)["asset"]
KEYS = ["summary", "provenance", "dimensions", "rig", "anchors", "representations", "thermal", "structure"]
for k in KEYS:
    assert k == "representations" or member(text, k) == f'  "{k}": ' + dump(asset[k], 2), k

asset["summary"]["en"] = asset["summary"]["en"].replace("(4.4 m, 1.7 m wide", "(4.45 m long, 1.7 m wide").replace("side-mounted spare wheels", "a side-mounted spare wheel")
asset["summary"]["de"] = asset["summary"]["de"].replace("(4,4 m, 1,7 m breit", "(4,45 m lang, 1,7 m breit").replace("seitlich montierte Reserveräder", "ein seitlich montiertes Reserverad")
refs = asset["provenance"]["references"]
REF = {"id": "voisin-c11-c14-data", "citation": "Published data of the six-cylinder Voisin C11 (1926 to 1929) and C14 (1928 to 1932) saloons: wheelbase 3213 mm (C11) and 3220 mm (C14), track 1422 mm (C11), 1400 mm front and 1350 mm rear (C14), overall 4450 x 1630 x 1750 mm (C14, 1931), 2327 cc sleeve-valve six, 32 x 6 tyres (conceptcarz.com vehicle profiles of the Voisin C11 and C14, supercars.net Voisin C14, vintagetyres.com Voisin C14 fitment; retrieved 2026-11-19). Used for the wheelbase of 3.20 m, the overall length of 4.45 m and the radiator standing over the front axle."}
if not any(r["id"] == REF["id"] for r in refs):
    refs.insert(1, REF)
n = asset["provenance"]["notes"]
n["en"] = "Wheelbase (3.20 m) and overall length (4.45 m) follow the published data of the six-cylinder Voisin C11 and C14 saloons, so that the radiator stands over the front axle as on every car of the period; width (1.70 m over the wings), height (1.70 m), track (1.34 m) and tyre size (5.00-19, overall radius 0.37 m) follow the class data of the C7/C11 family; the body lines are an original drawing of that type, not a copy of a particular car. Wheels roll consistently: the clip poses satisfy theta = -x / r with r = 0.37 m. The plan view has no wheels because fenders and running boards cover them from above. The drawings of every scale are the orthographic views of that scale's handcrafted 3D body. Thermal numbers are scaled-down authored estimates of a 40 kW petrol engine, tagged authored-estimate."
n["de"] = "Radstand (3,20 m) und Gesamtlänge (4,45 m) folgen den veröffentlichten Daten der sechszylindrigen Voisin-Salonwagen C11 und C14, sodass der Kühler wie bei jedem Wagen der Zeit über der Vorderachse steht; Breite (1,70 m über die Kotflügel), Höhe (1,70 m), Spurweite (1,34 m) und Reifengröße (5.00-19, Außenradius 0,37 m) folgen den Klassendaten der C7/C11-Familie; die Karosserielinien sind eine eigene Zeichnung dieses Typs, keine Kopie eines bestimmten Wagens. Die Räder rollen stimmig: die Clip-Posen erfüllen theta = -x / r mit r = 0,37 m. Der Grundriss zeigt keine Räder, da Kotflügel und Trittbretter sie von oben verdecken. Die Zeichnungen jedes Maßstabs sind die orthografischen Ansichten des handgebauten 3D-Körpers dieses Maßstabs. Die thermischen Zahlen sind verkleinerte Schätzungen eines Benzinmotors von 40 kW und als authored-estimate gekennzeichnet."
asset["dimensions"]["widthM"] = 4.45
for b in asset["rig"]["bones"]:
    if b["id"] == "chassis":
        b["lengthM"] = 4.45
        b["title"] = {"en": "Body, frame, fenders and spare wheel", "de": "Aufbau, Rahmen, Kotflügel und Reserverad"}
    if b["id"] == "axle-f":
        b["x"] = XF
for a in asset["anchors"]:
    if a["id"] == "tailpipe":
        a["x"], a["y"], a["z"] = -2.126, 0.42, -0.35
for r in asset["thermal"]["regions"]:
    if r["id"] == "radiator":
        r["areaM2"] = 0.75
st = asset["structure"]["assumptions"][0]
st["en"] = re.sub(r"static front share about \d+ %", "static front share about 44 %", st["en"])
st["de"] = re.sub(r"statischer Vorderachsanteil etwa \d+ %", "statischer Vorderachsanteil etwa 44 %", st["de"])
for den in (50, 100, 200, 500):
    rep = asset["representations"][str(den)]
    for view in ("elevation", "front", "plan"):
        fn = globals().get(f"{view}{den}")
        if fn:
            rep["geometry"][view] = fn(rep["geometry"][view])
    dec = globals().get(f"decisions2d{den}")
    if dec:
        dec(rep["decisions"])
new = text
for k in KEYS:
    new = member(new, k, asset[k])
doc = json.loads(new)
assert doc["asset"]["volumes"] == json.loads(text)["asset"]["volumes"]
os.makedirs(OUT, exist_ok=True)
cand = member(new, "volumes", json.load(open(os.path.join(HERE, "volumes", "vehicles", SLUG + ".json"), encoding="utf-8"))["volumes"])
json.loads(cand)
open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8").write(cand)
print("candidate", os.path.join(OUT, "🔣️.json"), len(text), "->", len(new))
if "--apply" in sys.argv:
    now = open(PATH, encoding="utf-8").read()
    for k in KEYS:
        now = member(now, k, asset[k])
    json.loads(now)
    open(PATH, "w", encoding="utf-8").write(now)
    print("applied", PATH)
