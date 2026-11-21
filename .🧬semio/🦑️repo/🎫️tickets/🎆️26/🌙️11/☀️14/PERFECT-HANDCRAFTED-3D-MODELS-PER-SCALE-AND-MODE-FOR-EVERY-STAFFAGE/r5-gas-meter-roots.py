"""🧮 Round 5, appliances/gas-meter: print roots of the 2D. The test-dial pointer cannot be printed at 1:100 (0.17 mm at the range end 1:150) or 1:200 (0.16 mm at the nominal scale), so its blade leaves the 1:100 and 1:200 drawings (the dial stays as the white disc); the 1:200 band ends at 1:250, where the 40 mm pins still leave a printable gap and the counter bar its 0.2 mm. The decision texts follow. Only the `representations` member of the catalog file is rewritten, in the file's own style (checked by a round trip before writing).

Usage: python3 r5-gas-meter-roots.py [--candidate <out.json>]
"""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-meter/🔣️.json"
_vi = open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read()
VI = type(sys)("vi")
exec(compile(_vi[: _vi.index("\ndef main(")], "volumes-insert.py", "exec"), VI.__dict__)

def js_numbers(text):
    """🔢 Exponent numbers as the catalog files carry them (plain decimals down to 1e-6)."""
    def repl(m):
        v = float(m.group(0))
        return ("%.15f" % v).rstrip("0").rstrip(".") if v != 0 and 1e-6 <= abs(v) < 1e21 else m.group(0)
    return re.sub(r"-?\d+(?:\.\d+)?e[+-]\d+", repl, text)


def style(value):
    """🪜 The file's two-space style at the asset's member indentation."""
    return '    "representations": ' + js_numbers(json.dumps(value, indent=2, ensure_ascii=False)).replace("\n", "\n    ")


TEXTS = {
    "100": {
        "silhouette": ("a round dial with a pointer", "a round dial", "einem runden Prüfrad mit Zeiger", "einem runden Prüfrad"),
        "retained": ("dial with pointer", "dial", "Prüfrad mit Zeiger", "Prüfrad"),
        "thinElements": ("The pointer is a tapered blade 5 mm wide (0.05 mm) and prints as a hairline; the stubs", "The pointer blade (5 mm, 0.05 mm on paper and 0.17 mm at 1:150) cannot be printed and is left out, the dial stays as the white disc; the stubs",
                         "Der Zeiger ist eine sich verjüngende Klinge von 5 mm (0,05 mm) Breite und druckt als Haarlinie; die Stutzen", "Die Zeigerklinge (5 mm, 0,05 mm auf dem Papier und 0,17 mm bei 1:150) ist nicht druckbar und entfällt, das Prüfrad bleibt als weiße Scheibe; die Stutzen"),
        "overlays": ("Shell, index and pipes stay the three drawn regions, the pointer turns with its bone.", "Shell, index and pipes stay the three drawn regions; the pointer is not drawn at this scale.",
                     "Schale, Zählwerkskopf und Rohre bleiben die drei gezeichneten Regionen, der Zeiger dreht sich mit seinem Knochen.", "Schale, Zählwerkskopf und Rohre bleiben die drei gezeichneten Regionen; der Zeiger wird in diesem Maßstab nicht gezeichnet."),
    },
    "200": {
        "silhouette": ("a round dial with a pointer", "a round dial", "einem runden Prüfrad mit Zeiger", "einem runden Prüfrad"),
        "retained": ("dial and pointer", "dial", "Prüfrad und Zeiger", "Prüfrad"),
        "thinElements": ("the pointer is a hairline.", "the pointer would be a hairline (0.16 mm at 1:200) and is left out; the band ends at 1:250, where the gap between the pins is still printable.",
                         "der Zeiger ist eine Haarlinie.", "der Zeiger wäre eine Haarlinie (0,16 mm bei 1:200) und entfällt; das Band endet bei 1:250, wo die Lücke zwischen den Zapfen noch druckbar ist."),
    },
}


def edit(asset):
    """✂️ Removes the pointer blades from 1:100 and 1:200, narrows the 1:200 band and rewrites the affected decision sentences."""
    reps = asset["representations"]
    for key in ("100", "200"):
        geo = reps[key]["geometry"]
        geo["elevation"] = [s for s in geo["elevation"] if s["id"] != "needle"]
        for field, (en0, en1, de0, de1) in TEXTS[key].items():
            text = reps[key]["decisions"][field]
            assert en0 in text["en"] and de0 in text["de"], (key, field)
            text["en"] = text["en"].replace(en0, en1)
            text["de"] = text["de"].replace(de0, de1)
    reps["200"]["validRange"]["maxDenominator"] = 250


def main():
    """💾 Candidate (with the volumes source merged) or in-place write of `representations`."""
    text = open(CATALOG, encoding="utf-8").read()
    doc = json.loads(text)
    start, end = VI.member_range(text, "    ", "representations")
    assert text[start:end] == style(doc["asset"]["representations"]), "style round trip failed"
    if any(s["id"] == "needle" for s in doc["asset"]["representations"]["200"]["geometry"]["elevation"]):
        edit(doc["asset"])
    if "--candidate" in sys.argv:
        doc["asset"]["volumes"] = json.load(open(os.path.join(HERE, "volumes", "appliances", "gas-meter.json"), encoding="utf-8"))["volumes"]
        out = sys.argv[sys.argv.index("--candidate") + 1]
        json.dump(doc, open(out, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
        print("candidate", out)
        return
    text = open(CATALOG, encoding="utf-8").read()
    start, end = VI.member_range(text, "    ", "representations")
    new = text[:start] + style(doc["asset"]["representations"]) + text[end:]
    assert json.loads(new)["asset"]["representations"] == doc["asset"]["representations"]
    open(CATALOG, "w", encoding="utf-8").write(new)
    print("written representations", CATALOG)


main()
