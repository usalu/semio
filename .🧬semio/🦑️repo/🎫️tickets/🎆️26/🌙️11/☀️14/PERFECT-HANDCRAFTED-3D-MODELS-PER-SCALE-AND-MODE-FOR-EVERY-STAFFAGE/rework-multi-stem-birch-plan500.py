"""🗺️ Multi-stem birch 1:500: the plan drawing redrawn as the true view of the reworked body (side-shoot sleeves back in), and the body text of the volumes source. `--write` writes the catalog file, otherwise only the candidate document."""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "vegetation", "multi-stem-birch.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/multi-stem-birch/🔣️.json"
OUTLINES = os.path.join(HERE, "🗑️generated", "r3-project2d", "vegetation", "multi-stem-birch.s500.plan.json")
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-multi-stem-birch", "🔣️.json")
BODY_500 = {
    "en": "Body at 1:500, modelled for a smallest feature of 75 mm: the stool, three stems as plain tapering lofts of two sections per bone and their leaders; the six boned branches and the forty smaller ones at their true section as slender solids, because they carry the crown and a bough must not hang in the air; the crown as the boughs of the twenty-seven main branches, each one broad sleeve, one or two side-shoot sleeves of five sections at least 0.1 m across, and broad hanging tresses that start in the sleeve that bears them, so that the weeping outline and the open, airy crown with the stems showing still read.",
    "de": "Körper bei 1:500, für ein kleinstes Merkmal von 75 mm modelliert: der Stock, drei Stämme als schlichte, sich verjüngende Lofts mit zwei Querschnitten je Knochen und ihre Leittriebe; die sechs Äste an Knochen und die vierzig kleineren mit ihrem wahren Querschnitt als schlanke Körper, weil sie die Krone tragen und keine Zweigpartie in der Luft hängen darf; die Krone als die Zweigpartien der siebenundzwanzig Hauptäste, je eine breite Laubhülle, ein oder zwei Seitentriebhüllen aus fünf Querschnitten von mindestens 0,1 m Breite und breite Hängestränge, die in der Hülle beginnen, die sie trägt, sodass der hängende Umriss und die offene, luftige Krone mit sichtbaren Stämmen lesbar bleiben.",
}


def dump(doc, path):
    with open(path, "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")


def array_range(text, keys):
    """📍 Start and end of the array value reached through `keys` in the text of a JSON document, each key searched after the one before."""
    at = 0
    for key in keys:
        at = re.compile(r'"' + key + r'"\s*:\s*').search(text, at).end()
    depth, k, quoted = 0, at, False
    while True:
        c = text[k]
        if quoted:
            k += 1 if c == "\\" else 0
            quoted = c != '"'
        elif c == '"':
            quoted = True
        elif c == "[":
            depth += 1
        elif c == "]":
            depth -= 1
            if depth == 0:
                return at, k + 1
        k += 1


def splice_plan(text, shapes):
    """✂️ Replaces only the 1:500 plan array in the catalog text, in the compact style found there; every other byte stays."""
    start, end = array_range(text, ["representations", "500", "geometry", "plan"])
    return text[:start] + json.dumps(shapes, ensure_ascii=False, separators=(",", ":")) + text[end:]


def main():
    source = json.load(open(SOURCE))
    source["volumes"]["500"]["decisions"][0] = BODY_500
    json.dump(source, open(SOURCE, "w"), ensure_ascii=False, separators=(",", ":"))
    text = open(CATALOG).read()
    doc = json.loads(text)
    plan = doc["asset"]["representations"]["500"]["geometry"]["plan"]
    outlines = {b["replaces"]: b["shapes"] for b in json.load(open(OUTLINES))["bones"] if b["replaces"]}
    redrawn = []
    for shape in plan:
        if shape["id"] in outlines:
            redrawn.extend(outlines[shape["id"]])
        elif not any(shape["id"].startswith(k + "-part-") for k in outlines):
            redrawn.append(shape)
    print("[DEBUG] plan shapes", len(plan), "->", len(redrawn), "redrawn bones", len(outlines))
    doc["asset"]["representations"]["500"]["geometry"]["plan"] = redrawn
    if "--write" in sys.argv:
        result = splice_plan(text, redrawn)
        assert json.loads(result) == doc
        open(CATALOG, "w", encoding="utf-8").write(result)
        return
    for scale, volume in source["volumes"].items():
        doc["asset"]["volumes"][scale] = volume
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    dump(doc, CANDIDATE)


if __name__ == "__main__":
    main()
