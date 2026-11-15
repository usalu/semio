import os, copy
__here = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(__here, "author-39-lib.py")).read())

path = CATALOG + "tilt-window/🔣️.json"
text = open(path).read()
if '"id": "steel"' not in text.split('"rig"')[0]:
    m = re.search(r'\n( +)"materials": ', text)
    st = m.end()
    en = _member_end(text, st)
    mats = _json.loads(text[st:en])
    mats.append({"id": "steel", "title": {"en": "Steel hinge fitting", "de": "Stahlbeschlag Scharnier"}, "emissivity": 0.8, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "elasticModulusPa": 210000000000, "source": "literature", "reference": "en-10025-2", "conductivityWmK": 50, "specificHeatJKgK": 450})
    ind = m.group(1)
    new = _json.dumps(mats, indent=2, ensure_ascii=False).replace("\n", "\n" + ind)
    text = text[:st] + new + text[en:]
    _json.loads(text)
    open(path, "w").write(text)
    print("steel material added")
text = open(path).read()
print('"en-10025-2"' in text)


def mutate(reps):
    g = reps["50"]["geometry"]
    for s in g["front"]:
        if s["id"] in ("hinge-left", "hinge-right"):
            cx = s["cx"]
            for k in ("cx", "cy", "rx", "ry"):
                s.pop(k, None)
            s.update({"kind": "rect", "x": round(cx - 0.04, 4), "y": 0.03, "w": 0.08, "h": 0.04})
    for s in g["plan"]:
        if s["id"] in ("hinge-left", "hinge-right"):
            cy = s["cy"]
            for k in ("cx", "cy", "rx", "ry"):
                s.pop(k, None)
            s.update({"kind": "rect", "x": -0.02, "y": round(cy - 0.04, 4), "w": 0.04, "h": 0.08})


patch_representations("tilt-window", mutate)
