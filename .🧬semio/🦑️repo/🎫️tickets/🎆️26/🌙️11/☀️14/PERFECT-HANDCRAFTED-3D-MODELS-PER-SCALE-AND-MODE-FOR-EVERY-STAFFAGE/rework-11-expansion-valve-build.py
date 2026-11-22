"""🧊 Rework 11: the thermostatic expansion valve gets its working parts (orifice cartridge, needle, push pin, diaphragm, superheat spring) as a hidden thermal region; the exterior stays as authored.

Usage: python3 rework-11-expansion-valve-build.py [--publish]
Frame of bone `body`: origin on the outlet axis in the valve centre, u to the outlet, v up (liquid enters from below); bone `head` points up from v 0.015 (its u = up); bone `stem` points down and back at 210 degrees.
"""
import json
import sys

from rework_11_lib import *

SLUG = "expansion-valve"
BR, SS = "brass", "stainless-steel"
OLD = pristine(SLUG)
AREA = 0.0036


def throttle(scale):
    """Orifice cartridge 9 x 16 mm with its 2 mm bore, needle, push pin under the 36 mm diaphragm, superheat spring 8 x 12 mm on the spindle."""
    cartridge = rev("th-orifice-cartridge", "body", BR, 0.5, [[-0.024, 0.001], [-0.024, 0.0045], [-0.008, 0.0045], [-0.008, 0.001], [-0.024, 0.001]], [0, 1, 0], [0, 0, 0])
    diaphragm = rev("th-diaphragm", "head", SS, 0.75, [[0.0078, 0], [0.0078, 0.018], [0.0084, 0.018], [0.0084, 0]], [1, 0, 0], [0, 0, 0], role="texture")
    coarse = {
        "100": [[-0.024, 0], [-0.024, 0.0045], [-0.008, 0.0045], [-0.008, 0]],
        "200": [[-0.024, 0], [-0.024, 0.0045], [0.003, 0.0045], [0.003, 0]],
        "500": [[-0.024, 0], [-0.016, 0.0045], [-0.005, 0.0045], [0.003, 0]],
    }
    if scale != "50":
        return [rev("th-orifice-cartridge", "body", BR, 0.5, coarse[scale], [0, 1, 0], [0, 0, 0], role="texture"), diaphragm]
    return [
        cartridge,
        rev("th-needle", "body", SS, 0.7, [[-0.009, 0], [-0.007, 0.0016], [0.003, 0.0016], [0.003, 0]], [0, 1, 0], [0, 0, 0], role="texture"),
        rev("th-push-pin", "head", SS, 0.7, [[-0.012, 0], [-0.012, 0.0012], [0.0078, 0.0012], [0.0078, 0]], [1, 0, 0], [0, 0, 0], role="texture"),
        diaphragm,
        rev("th-superheat-spring", "stem", SS, 0.6, [[0.002, 0.0025], [0.002, 0.004], [0.014, 0.004], [0.014, 0.0025], [0.002, 0.0025]], [1, 0, 0], [0, 0, 0], role="texture"),
        rev("th-spindle", "stem", BR, 0.5, [[0.014, 0], [0.014, 0.002], [0.021, 0.002], [0.021, 0]], [1, 0, 0], [0, 0, 0], role="texture"),
    ]


REGION = {
    "id": "throttle", "title": text("Orifice, needle, diaphragm and superheat spring", "Düse, Nadel, Membran und Überhitzungsfeder"), "kind": "surface", "model": "operating", "material": BR, "areaM2": AREA,
    "characteristicLengthM": 0.016, "geometry": "cylinder", "skyViewFactor": 0,
    "layers": [{"id": "throttle-skin", "title": text("Brass cartridge wall", "Messing-Düsenwand"), "kind": "metal", "material": BR, "thicknessM": 0.0035, "resistanceM2KW": 0.00003, "heatCapacityJM2K": 11000, "source": "authored-estimate", "reference": "incropera-2018"}],
    "heatCapacityJM2K": 11000, "operating": {"key": "flow", "heatW": -0.25}, "source": "authored-estimate", "reference": "incropera-2018",
    "hidden": text("Inside the brass body: the exchangeable orifice cartridge with its 2 mm bore where the liquid flashes, the needle on its seat, the push pin under the diaphragm of the element and the superheat spring on the setting spindle. The pressure drop happens here, so this is where the refrigerant turns cold; no drawing shows it, the cut 3D thermal model does.", "Im Messinggehäuse: der wechselbare Düseneinsatz mit seiner 2-mm-Bohrung, in der die Flüssigkeit entspannt, die Nadel auf ihrem Sitz, der Druckstift unter der Membran des Elements und die Überhitzungsfeder auf der Einstellspindel. Hier fällt der Druck, hier wird das Kältemittel kalt; keine Zeichnung zeigt es, das geschnittene thermische 3D-Modell schon."),
}
NOTE = {
    "50": text("The working parts are solids of the hidden thermal region throttle at true size: orifice cartridge of 9 x 16 mm, needle of 3.2 mm, push pin of 2.4 mm, diaphragm of 36 mm, superheat spring of 8 x 12 mm and its spindle; the parts thinner than 7.5 mm are declared thin sheets.", "Die Funktionsteile sind Körper der verdeckten thermischen Region throttle in wahrer Größe: Düseneinsatz 9 x 16 mm, Nadel 3,2 mm, Druckstift 2,4 mm, Membran 36 mm, Überhitzungsfeder 8 x 12 mm und ihre Spindel; die Teile unter 7,5 mm sind als dünne Bleche erklärt."),
    "x": text("Of the working parts the orifice cartridge (simplified for the scale) and the 36 mm diaphragm remain as the solids of the hidden region throttle.", "Von den Funktionsteilen bleiben der Düseneinsatz (für den Maßstab vereinfacht) und die 36-mm-Membran als Körper der verdeckten Region throttle."),
}


def main():
    document, before = load_catalog(SLUG)
    asset = document["asset"]
    asset["thermal"]["regions"] = [x for x in json.loads(json.dumps(OLD["thermal"]["regions"])) if x["id"] != "throttle"] + [REGION]
    for pose in asset["poses"]:
        pose["thermal"]["regions"]["throttle"] = {"sunlitFraction": 0, "exposedFraction": 1}
    front = json.loads(json.dumps(OLD["representations"]["50"]["geometry"]["front"]))
    find(front, "capillary")["d"] += " L 0.0252 0.0785 L 0.0243 0.0735"
    asset["representations"]["50"]["geometry"]["front"] = front
    volumes = json.loads(json.dumps(OLD["volumes"]))
    for scale in SCALES:
        volumes[scale]["thermal"] += [dict(x, region="throttle") for x in throttle(scale)]
        volumes[scale]["decisions"] = volumes[scale]["decisions"] + [NOTE["50" if scale == "50" else "x"]]
    save(SLUG, document, volumes, before, "--publish" in sys.argv)


main()
