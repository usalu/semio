"""🛵 Hand-stated corrections of vehicles/motor-scooter (second round, pass 1): helmet-bin collar and fuel cap under the seat at 1:50, the raked front of the rear body under the seat nose at 1:500, decisions. Usage: python3 rework-09-motor-scooter-model.py"""
import json

PATH = "volumes/vehicles/motor-scooter.json"
ALONG = [0.5, 0.5, 0.5, 0.5]


def put(items, solid, after=None):
    for i, s in enumerate(items):
        if s["id"] == solid["id"]:
            items[i] = solid
            return
    at = next((i + 1 for i, s in enumerate(items) if s["id"] == after), len(items))
    items.insert(at, solid)


def section(x, y, rx, ry, exponent):
    return {"at": [x, y, 0], "rx": rx, "ry": ry, "exponent": exponent, "rotation": ALONG}


def sentence(decisions, index, en, de):
    d = decisions[index]
    if en not in d["en"]:
        d["en"] = d["en"].rstrip() + " " + en
        d["de"] = d["de"].rstrip() + " " + de


doc = json.load(open(PATH, encoding="utf-8"))
v = doc["volumes"]

b50, t50 = v["50"]["body"], v["50"]["thermal"]
put(b50, {
    "id": "bin-rim", "kind": "loft", "bone": "frame", "material": "rubber", "role": "detail", "tone": 0.15,
    "sections": [
        section(-0.200, 0.714, 0.012, 0.070, 3),
        section(-0.225, 0.716, 0.013, 0.112, 4),
        section(-0.300, 0.717, 0.013, 0.124, 4),
        section(-0.440, 0.717, 0.013, 0.126, 4),
        section(-0.535, 0.717, 0.013, 0.116, 4),
        section(-0.565, 0.716, 0.012, 0.074, 3),
    ],
    "capStart": True, "capEnd": True,
}, after="tail")
put(b50, {
    "id": "fuel-cap", "kind": "revolve", "bone": "frame", "material": "alloy", "role": "detail", "tone": 0.7,
    "profile": [[0, 0], [0, 0.034], [0.012, 0.034], [0.017, 0.027], [0.017, 0]],
    "axis": [0, 1, 0], "origin": [-0.635, 0.712, 0],
}, after="bin-rim")
put(t50, {"id": "th-bin-rim", "of": "bin-rim", "region": "body-panels", "layer": "paint"}, after="th-tail")
put(t50, {"id": "th-fuel-cap", "of": "fuel-cap", "region": "body-panels", "layer": "paint"}, after="th-bin-rim")
sentence(v["50"]["decisions"], 0,
         "Under the hinged seat the rear body carries the raised collar of the helmet bin and the fuel filler cap behind it; both are hidden by the closed seat and show when the seat is lifted.",
         "Unter der klappbaren Sitzbank trägt die Heckkarosserie den erhabenen Kragen des Helmfachs und dahinter den Tankdeckel; beide sind bei geschlossener Sitzbank verdeckt und zeigen sich bei angehobener Sitzbank.")

body500 = next(s for s in v["500"]["body"] if s["id"] == "body")
if not any(abs(q["at"][0] + 0.17) < 1e-9 for q in body500["sections"]):
    body500["sections"].insert(2, section(-0.17, 0.525, 0.195, 0.172, 3))
sentence(v["500"]["decisions"], 0,
         "The rear body keeps its raked front bulkhead up to the seat hinge, so the seat nose lies on the body as at the finer scales.",
         "Die Heckkarosserie behält ihr geneigtes Frontschott bis zum Sitzscharnier, sodass die Sitznase wie in den feineren Maßstäben auf der Karosserie aufliegt.")

v["100"]["decisions"][1] = {
    "en": "Thermal: every region is the surface of its own parts by reference; the painted panels and the painted steering parts are their paint film, the outer layer that exchanges heat, and the steel wall behind it is not a separate solid at this scale; the exhaust jet leaves from an outlet patch on the tail pipe mouth.",
    "de": "Thermik: Jede Region ist per Verweis die Oberfläche ihrer eigenen Teile; die lackierten Bleche und die lackierten Lenkungsteile sind ihre Lackschicht, die äußere, wärmetauschende Schicht, die Stahlwand dahinter ist in diesem Maßstab kein eigener Körper; der Abgasstrahl tritt aus einem Auslassfleck an der Endrohrmündung aus.",
}

json.dump(doc, open(PATH, "w", encoding="utf-8"), ensure_ascii=False)
print("[DEBUG] written", PATH, {k: [len(v[k][l]) for l in ("body", "thermal", "structure")] for k in v})
