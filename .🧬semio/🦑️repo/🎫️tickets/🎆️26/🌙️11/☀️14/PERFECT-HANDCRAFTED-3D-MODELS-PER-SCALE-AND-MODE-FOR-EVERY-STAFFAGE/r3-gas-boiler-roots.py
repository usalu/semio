"""🔥 Round 3, appliances/gas-boiler: the root fixes outside `volumes`.

The casing grows from 0.62 m to the 0.72 m of the 20 kW wall-hung class (Vaillant ecoTEC plus 720 x 440 x 335 mm): every part above y = 0.30 m (window, burner, flames, door top, slits, logo, collar, wall bracket) moves up by 0.10 m in the rig, the anchors and the 2D drawings of all scales; the front and plan outlines lose the corner radii the 3D hood does not have (E1); the hidden internals get their thermal regions and materials; `areaM2` of the casing and the controls follows the hand calculation of the real parts.

Usage: python3 r3-gas-boiler-roots.py            builds the candidate 🗑️generated/r3-gas-boiler/🔣️.json (volumes from the source) and validates it
       python3 r3-gas-boiler-roots.py --apply    also writes the changed members into the catalog file and the manifest height
"""
import copy
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAFFAGE = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"))
CATALOG = os.path.join(STAFFAGE, "🗂️catalog/🔌️appliances/gas-boiler/🔣️.json")
MANIFEST = os.path.join(STAFFAGE, "🗂️catalog/🔣️.json")
SOURCE = os.path.join(HERE, "volumes/appliances/gas-boiler.json")
OUT = os.path.join(HERE, "🗑️generated/r3-gas-boiler")
CANDIDATE = os.path.join(OUT, "🔣️.json")
NS = {}
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0], NS)

SPLIT = 0.30
LIFT = 0.10
OLD_HEIGHT = 0.72
NEW_HEIGHT = 0.82


def up(y):
    """⬆️ Height after the casing grew: everything above the split moves up by the lift."""
    return round(y + LIFT, 6) if y > SPLIT else y


def shift_d(d):
    """🧭 Moves the points of an SVG-like path string (M/L/Z with x y pairs) above the split."""
    tokens = d.split()
    out, i = [], 0
    while i < len(tokens):
        t = tokens[i]
        if t in ("M", "L"):
            x, y = float(tokens[i + 1]), float(tokens[i + 2])
            out += [t, fmt(x), fmt(up(y))]
            i += 3
        else:
            out.append(t)
            i += 1
    return " ".join(out)


def fmt(v):
    """🔢 A number as the drawings write it."""
    s = repr(round(v, 6))
    return s[:-2] if s.endswith(".0") else s


def shift_shape(s, world):
    """📐 Lifts one shape of the casing bone (elevation: bone frame = world; front: y is world height)."""
    if not world:
        return
    k = s["kind"]
    if k == "path":
        s["d"] = shift_d(s["d"])
    elif k == "poly":
        s["points"] = [[p[0], up(p[1])] for p in s["points"]]
    elif k == "ellipse":
        s["cy"] = up(s["cy"])
    elif k == "rect":
        y0, y1 = s["y"], s["y"] + s["h"]
        n0, n1 = up(y0), up(y1)
        s["y"], s["h"] = n0, round(n1 - n0, 6)


def rounded_lines():
    """〰️ The plan's top-edge detail: the tangent lines of the 14 mm rounded side edges of the hood, front to back."""
    return "M -0.206 -0.155 L -0.206 0.175 M 0.206 -0.155 L 0.206 0.175"


def text_swap(t, pairs):
    """🔤 Replaces stated numbers in an en/de text, every pair must hit."""
    t = dict(t)
    for lang, old, new in pairs:
        if old not in t[lang]:
            raise SystemExit(f"text not found: {old}")
        t[lang] = t[lang].replace(old, new)
    return t


def roots(asset):
    """🌱 Applies every root fix to a deep copy of the asset."""
    a = copy.deepcopy(asset)
    a["dimensions"]["heightM"] = NEW_HEIGHT
    refs = a["provenance"]["references"]
    if not any(r["id"] == "vaillant-ecotec-plus" for r in refs):
        refs.append({"id": "vaillant-ecotec-plus", "citation": "Vaillant UK, ecoTEC plus system and combi boilers, product brochure and data sheets (read 2026-11-20): casing 720 mm high, 440 mm wide, 335 mm deep; low-lift weight from 36.8 kg; concentric flue adapter 60/100 or 80/125 mm on top, connections below."})
    notes = a["provenance"]["notes"]
    notes["en"] = notes["en"].replace(
        "casing 0.44 m wide and 0.62 m high, 0.35 m deep, a concentric flue collar of 125 mm on top reaching a total height of 0.72 m",
        "casing 0.44 m wide, 0.72 m high and 0.33 m deep (Vaillant ecoTEC plus: 720 x 440 x 335 mm), 0.35 m deep over the control knobs, a concentric flue collar of 125 mm on top reaching a total height of 0.82 m")
    notes["de"] = notes["de"].replace(
        "Gehäuse 0,44 m breit und 0,62 m hoch, 0,35 m tief, oben ein konzentrischer Abgasstutzen von 125 mm bis zu einer Gesamthöhe von 0,72 m",
        "Gehäuse 0,44 m breit, 0,72 m hoch und 0,33 m tief (Vaillant ecoTEC plus: 720 x 440 x 335 mm), über den Reglerknöpfen 0,35 m tief, oben ein konzentrischer Abgasstutzen von 125 mm bis zu einer Gesamthöhe von 0,82 m")
    if "0.82 m" not in notes["en"] or "0,82 m" not in notes["de"]:
        raise SystemExit("provenance notes not updated")
    mats = a["materials"]
    have = {m["id"] for m in mats}
    for m in [
        {"id": "galvanised-steel", "title": {"en": "Galvanised steel chassis sheet", "de": "Verzinktes Stahlblech des Chassis"}, "emissivity": 0.25, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
        {"id": "stainless-steel", "title": {"en": "Stainless steel heat exchanger", "de": "Edelstahl-Wärmeübertrager"}, "emissivity": 0.4, "solarAbsorptivity": 0.5, "densityKgM3": 7900, "conductivityWmK": 15, "specificHeatJKgK": 500, "elasticModulusPa": 193000000000, "strengthPa": 205000000, "source": "literature", "reference": "incropera-2018"},
        {"id": "die-cast-aluminium", "title": {"en": "Die-cast aluminium fan housing", "de": "Lüftergehäuse aus Aluminium-Druckguss"}, "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 2700, "conductivityWmK": 120, "specificHeatJKgK": 900, "source": "literature", "reference": "incropera-2018"},
    ]:
        if m["id"] not in have:
            mats.append(m)
    for b in a["rig"]["bones"]:
        if b["id"] == "casing":
            b["lengthM"] = 0.77
            b["mass"]["comY"] = 0.42
        elif b["id"] in ("collar", "flame", "flame-tall"):
            b["y"] = up(b["y"])
    for an in a["anchors"]:
        if an["bone"] == "casing":
            an["y"] = up(an["y"])
        if an["id"] == "wall-bracket":
            an["z"] = 0.165
        if an["id"] == "flue-outlet":
            an["z"] = 0.04
    reps = a["representations"]
    for scale in ("50", "100", "200"):
        rep = reps[scale]
        for view in ("elevation", "front"):
            for s in rep["geometry"][view]:
                if s["bone"] == "casing" or (view == "front" and s["bone"] == "collar"):
                    shift_shape(s, True)
        for view in ("front", "plan"):
            for s in rep["geometry"][view]:
                if s["id"] in ("casing", "panel") and s["kind"] == "rect":
                    s.pop("r", None)
    g50 = reps["50"]["geometry"]
    for s in g50["plan"]:
        if s["id"] == "top-edge":
            s.pop("points", None)
            s.pop("closed", None)
            s["kind"] = "path"
            s["d"] = rounded_lines()
    g50["front"] = [s for s in g50["front"] if s["id"] != "door"]
    seam_front = {"id": "rear-seam", "bone": "casing", "role": "detail", "z": 3.05, "filled": False, "tone": 0.4, "kind": "poly", "closed": False, "points": [[0.155, 0.0515], [0.155, 0.7685]]}
    seam_plan = {"id": "rear-seam", "bone": "casing", "role": "detail", "z": 3.05, "filled": False, "tone": 0.4, "kind": "poly", "closed": False, "points": [[-0.2185, 0.155], [0.2185, 0.155]]}
    for view, seam in (("front", seam_front), ("plan", seam_plan)):
        if not any(s["id"] == "rear-seam" for s in g50[view]):
            idx = next(i for i, s in enumerate(g50[view]) if s["id"] == "casing") + 1
            g50[view].insert(idx, seam)
    d50 = reps["50"]["decisions"]
    d50["silhouette"] = text_swap(d50["silhouette"], [
        ("en", "A 0.44 m by 0.72 m front elevation, 8.8 mm by 14.4 mm on paper", "A 0.44 m by 0.82 m front elevation, 8.8 mm by 16.4 mm on paper"),
        ("en", "7 mm by 14.4 mm on paper", "7 mm by 16.4 mm on paper"),
        ("en", "the rounded casing top with its edge line", "the casing top with the tangent lines of its rounded side edges and the rear seam of the hood"),
        ("de", "Eine Vorderansicht von 0,44 m mal 0,72 m, auf Papier 8,8 mm mal 14,4 mm", "Eine Vorderansicht von 0,44 m mal 0,82 m, auf Papier 8,8 mm mal 16,4 mm"),
        ("de", "7 mm auf 14,4 mm", "7 mm auf 16,4 mm"),
        ("de", "die gerundete Gehäuseoberseite mit Kantenlinie", "die Gehäuseoberseite mit den Tangentenlinien ihrer gerundeten Seitenkanten und der hinteren Haubenfuge"),
    ])
    d50["retained"] = text_swap(d50["retained"], [
        ("en", "the door line,", "the door line, the rear seam of the hood,"),
        ("de", "Türlinie,", "Türlinie, hintere Haubenfuge,"),
    ])
    d50["merged"] = text_swap(d50["merged"], [
        ("en", "The wall bracket, the door seam and the casing fixings are part of the casing shape;", "The wall bracket and the casing fixings are part of the casing shape, the front edges of the pressed hood are sharp at this scale;"),
        ("de", "Wandhalter, Türfuge und Gehäusebefestigungen gehören zur Gehäuseform;", "Wandhalter und Gehäusebefestigungen gehören zur Gehäuseform, die Vorderkanten der gepressten Haube sind in diesem Maßstab scharf;"),
    ])
    d100 = reps["100"]["decisions"]
    d100["silhouette"] = text_swap(d100["silhouette"], [
        ("en", "A 4.4 mm by 7.2 mm front elevation", "A 4.4 mm by 8.2 mm front elevation"),
        ("en", "Front, 3.5 mm by 7.2 mm", "Front, 3.5 mm by 8.2 mm"),
        ("de", "Eine Vorderansicht von 4,4 mm mal 7,2 mm", "Eine Vorderansicht von 4,4 mm mal 8,2 mm"),
        ("de", "3,5 mm auf 7,2 mm", "3,5 mm auf 8,2 mm"),
    ])
    d200 = reps["200"]["decisions"]
    d200["silhouette"] = text_swap(d200["silhouette"], [
        ("en", "A 2.2 mm by 3.6 mm mark", "A 2.2 mm by 4.1 mm mark"),
        ("en", "Front, 1.75 mm by 3.6 mm", "Front, 1.75 mm by 4.1 mm"),
        ("de", "Ein Zeichen von 2,2 mm mal 3,6 mm", "Ein Zeichen von 2,2 mm mal 4,1 mm"),
        ("de", "1,75 mm auf 3,6 mm", "1,75 mm auf 4,1 mm"),
    ])
    d200["retained"] = text_swap(d200["retained"], [
        ("en", "The 0.44 m by 0.62 m casing", "The 0.44 m by 0.72 m casing"),
        ("de", "Das Gehäuse von 0,44 m mal 0,62 m", "Das Gehäuse von 0,44 m mal 0,72 m"),
    ])
    reps["500"]["reason"] = text_swap(reps["500"]["reason"], [
        ("en", "0.88 mm by 1.44 mm", "0.88 mm by 1.64 mm"),
        ("de", "0,88 mm mal 1,44 mm", "0,88 mm mal 1,64 mm"),
    ])
    th = a["thermal"]
    for r in th["regions"]:
        if r["id"] == "casing":
            r["areaM2"] = 1.26
            r["characteristicLengthM"] = 0.72
        if r["id"] == "controls":
            r["areaM2"] = 0.025
    hidden = [
        {"id": "heat-exchanger", "title": {"en": "Stainless steel heat exchanger with combustion chamber and flue collector", "de": "Edelstahl-Wärmeübertrager mit Brennkammer und Abgassammler"}, "kind": "surface", "model": "passive", "material": "stainless-steel", "areaM2": 0.35, "characteristicLengthM": 0.23, "geometry": "cylinder", "skyViewFactor": 0,
         "layers": [{"id": "wall", "title": {"en": "Stainless steel wall 1 mm", "de": "Edelstahlwand 1 mm"}, "kind": "metal", "material": "stainless-steel", "thicknessM": 0.001, "resistanceM2KW": 6.7e-05, "heatCapacityJM2K": 3950, "source": "authored-estimate", "reference": "incropera-2018"}],
         "heatCapacityJM2K": 3950, "source": "authored-estimate", "reference": "incropera-2018",
         "hidden": {"en": "The coil pack of 0.23 m diameter and 0.17 m depth with its burner door, back plate and the flue collector up to the collar sits in the upper half of the casing behind the sight glass; no drawing shows it. Its area is the envelope of the housing (0.35 m2); the wetted coil area that takes the 20 kW into the heating water is not modelled, the region is passive at room air.", "de": "Das Rohrschlangenpaket von 0,23 m Durchmesser und 0,17 m Tiefe mit Brennertür, Rückwand und dem Abgassammler bis zum Stutzen sitzt in der oberen Gehäusehälfte hinter dem Schauglas; keine Zeichnung zeigt es. Seine Fläche ist die Hülle des Gehäuses (0,35 m2); die benetzte Rohrfläche, die die 20 kW ans Heizwasser abgibt, ist nicht modelliert, die Region ist passiv bei Raumluft."}},
        {"id": "fan", "title": {"en": "Premix fan", "de": "Vormischgebläse"}, "kind": "surface", "model": "passive", "material": "die-cast-aluminium", "areaM2": 0.037, "characteristicLengthM": 0.09, "geometry": "cylinder", "skyViewFactor": 0,
         "layers": [{"id": "housing", "title": {"en": "Die-cast aluminium housing 3 mm", "de": "Aluminium-Druckgussgehäuse 3 mm"}, "kind": "metal", "material": "die-cast-aluminium", "thicknessM": 0.003, "resistanceM2KW": 2.5e-05, "heatCapacityJM2K": 7290, "source": "authored-estimate", "reference": "incropera-2018"}],
         "heatCapacityJM2K": 7290, "source": "authored-estimate", "reference": "incropera-2018",
         "hidden": {"en": "The premix fan of 90 mm diameter sits beside the heat exchanger on the left and blows the gas-air mixture through the burner door; no drawing shows it. Its 30 W of electrical input are part of the casing losses.", "de": "Das Vormischgebläse von 90 mm Durchmesser sitzt links neben dem Wärmeübertrager und bläst das Gas-Luft-Gemisch durch die Brennertür; keine Zeichnung zeigt es. Seine 30 W Leistungsaufnahme sind Teil der Gehäuseverluste."}},
        {"id": "expansion-vessel", "title": {"en": "Expansion vessel", "de": "Ausdehnungsgefäß"}, "kind": "surface", "model": "passive", "material": "painted-steel", "areaM2": 0.16, "characteristicLengthM": 0.32, "geometry": "vertical-plate", "skyViewFactor": 0,
         "layers": [{"id": "shell", "title": {"en": "Painted steel shell 1.5 mm", "de": "Lackierte Stahlschale 1,5 mm"}, "kind": "metal", "material": "painted-steel", "thicknessM": 0.0015, "resistanceM2KW": 3e-05, "heatCapacityJM2K": 5420, "source": "authored-estimate", "reference": "incropera-2018"}],
         "heatCapacityJM2K": 5420, "source": "authored-estimate", "reference": "incropera-2018",
         "hidden": {"en": "The flat round expansion vessel of 0.32 m diameter and 80 mm depth stands between the heat exchanger and the back sheet; no drawing shows it.", "de": "Das flache runde Ausdehnungsgefäß von 0,32 m Durchmesser und 80 mm Tiefe steht zwischen Wärmeübertrager und Rückwand; keine Zeichnung zeigt es."}},
        {"id": "hydraulics", "title": {"en": "Hydraulic group (pump, gas valve, plate heat exchanger, manifold, siphon, pipes)", "de": "Hydraulikgruppe (Pumpe, Gasarmatur, Plattenwärmeübertrager, Verteiler, Siphon, Rohre)"}, "kind": "surface", "model": "passive", "material": "brass", "areaM2": 0.21, "characteristicLengthM": 0.1, "geometry": "cylinder", "skyViewFactor": 0,
         "layers": [{"id": "body", "title": {"en": "Brass and composite bodies 3 mm", "de": "Messing- und Verbundkörper 3 mm"}, "kind": "metal", "material": "brass", "thicknessM": 0.003, "resistanceM2KW": 2.5e-05, "heatCapacityJM2K": 9690, "source": "authored-estimate", "reference": "incropera-2018"}],
         "heatCapacityJM2K": 9690, "source": "authored-estimate", "reference": "incropera-2018",
         "hidden": {"en": "Pump, gas valve, domestic hot-water plate heat exchanger, the connection manifold, the condensate siphon and the flow and return pipes from the heat exchanger fill the lower part of the casing behind the control panel; no drawing shows them.", "de": "Pumpe, Gasarmatur, Trinkwasser-Plattenwärmeübertrager, der Anschlussverteiler, der Kondensatsiphon und Vor- und Rücklaufrohre vom Wärmeübertrager füllen den unteren Gehäuseteil hinter dem Bedienfeld; keine Zeichnung zeigt sie."}},
    ]
    ids = {r["id"] for r in th["regions"]}
    for h in hidden:
        if h["id"] not in ids:
            th["regions"].append(h)
    note = {"en": "The heat exchanger, fan, expansion vessel and hydraulic group are hidden passive regions inside the casing: they show where the heat path runs but are not solved between water and flue gas; their surface losses are part of the casing losses.",
            "de": "Wärmeübertrager, Gebläse, Ausdehnungsgefäß und Hydraulikgruppe sind verborgene passive Regionen im Gehäuse: sie zeigen, wo der Wärmeweg verläuft, werden aber zwischen Wasser und Abgas nicht gelöst; ihre Oberflächenverluste sind Teil der Gehäuseverluste."}
    if note not in th["assumptions"]:
        th["assumptions"].append(note)
    for p in a["poses"]:
        regs = p["thermal"]["regions"]
        for h in hidden:
            regs.setdefault(h["id"], {"exposedFraction": 1, "sunlitFraction": 0})
    snote = {"en": "The load path is the galvanised chassis: the back sheet with its folded side, top and bottom edges hangs on the wall rail; the heat exchanger sits on two pairs of angle brackets, the flue collar on the flue collector of the heat exchanger and the connections on the manifold bracket of the bottom edge. The front hood is cladding and carries nothing.",
             "de": "Der Lastweg ist das verzinkte Chassis: das Rückblech mit seinen gekanteten Seiten-, Ober- und Unterkanten hängt an der Wandschiene; der Wärmeübertrager sitzt auf zwei Paar Winkelkonsolen, der Abgasstutzen auf dem Abgassammler des Wärmeübertragers und die Anschlüsse am Verteilerwinkel der Unterkante. Die Fronthaube ist Verkleidung und trägt nichts."}
    if snote not in a["structure"]["assumptions"]:
        a["structure"]["assumptions"].append(snote)
    return a


def js_numbers(text):
    """🔢 Writes exponent numbers the way the catalog files carry them (JavaScript: plain decimals down to 1e-6)."""
    def repl(m):
        v = float(m.group(0))
        if v != 0 and abs(v) >= 1e-6 and abs(v) < 1e21:
            s = ("%.15f" % v).rstrip("0").rstrip(".")
            return s
        return m.group(0)
    return re.sub(r"-?\d+(?:\.\d+)?e[+-]\d+", repl, text)


def indent2(value):
    """🪜 The two-space style with the asset's member indentation."""
    return js_numbers(json.dumps(value, indent=2, ensure_ascii=False)).replace("\n", "\n    ")


def build():
    """🏗️ Writes and validates the candidate document."""
    doc = json.load(open(CATALOG, encoding="utf-8"))
    base = doc["asset"]
    if base["rig"]["bones"][0]["lengthM"] == 0.77:
        raise SystemExit("catalog already carries the root fixes; nothing to rebuild")
    new = roots(base)
    new["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    doc["asset"] = new
    os.makedirs(OUT, exist_ok=True)
    json.dump(doc, open(CANDIDATE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    bun = sorted(os.popen("ls ~/Library/Caches/semio/runtimes/bun/*/bun").read().split())[0]
    r = subprocess.run([bun, "r3-validate.ts", "--file", CANDIDATE], cwd=HERE, capture_output=True, text=True)
    print(r.stdout[-4000:], r.stderr[-2000:])
    return doc, r.returncode


def apply(doc):
    """📝 Writes the changed members (never `volumes`) into the catalog file and the manifest height."""
    want = doc["asset"]
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = []
    for key in want:
        if key == "volumes" or have.get(key) == want[key]:
            continue
        start, end = NS["member_range"](text, "    ", key)
        old = text[start:end]
        style = indent2 if indent2(have[key]) in old else (lambda v: js_numbers(NS["dump"](v, 2)))
        text = text[:start] + "    " + json.dumps(key) + ": " + style(want[key]) + text[end:]
        changed.append(key)
    got = json.loads(text)["asset"]
    for key in want:
        if key != "volumes" and got[key] != want[key]:
            raise SystemExit(f"member {key} differs from the candidate")
    if list(got) != list(have):
        raise SystemExit("member order changed")
    open(CATALOG, "w", encoding="utf-8").write(text)
    m = open(MANIFEST, encoding="utf-8").read()
    i = m.index('"id": "gas-boiler"')
    j = m.index('"heightM": ', i)
    k = m.index(",", j)
    if m[j:k] != f'"heightM": {OLD_HEIGHT}':
        raise SystemExit("manifest height not as expected: " + m[j:k])
    m = m[:j] + f'"heightM": {NEW_HEIGHT}' + m[k:]
    json.loads(m)
    open(MANIFEST, "w", encoding="utf-8").write(m)
    print("written", CATALOG, "members:", ", ".join(changed), "; manifest heightM", NEW_HEIGHT)


if __name__ == "__main__":
    doc, code = build()
    if "--apply" in sys.argv:
        if code != 0:
            raise SystemExit("candidate INVALID: not applied")
        apply(doc)
