"""🪟 Round 5 glass balustrade root fix (2D 1:100): the two clamp bars of the end view meet on the glass axis, so the 21.5 mm slot (0.29 mm at 1:75) no longer leaves two unprintable 0.04 mm gaps beside the 0.2 mm glass stroke; the 1:100 decisions say so. Targeted text replacements, file re-read right before writing; candidate under `🗑️generated/r5-glass-balustrade/🔣️.json` first.

Usage: python3 r5-glass-balustrade-roots.py [--apply]

@see ./r5-brief.md
@see ./r3-components-glass-balustrade.md
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
PATH = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/glass-balustrade/🔣️.json"
CAND = os.path.join(T, "🗑️generated", "r5-glass-balustrade", "🔣️.json")
TEXTS = [
    ("Base plate (3 x 0.15 mm), clamp bars (0.25 x 1.1 mm),", "Base plate (3 x 0.15 mm), clamp bars (0.36 x 1.1 mm, meeting on the glass axis),"),
    ("Fußplatte (3 x 0,15 mm), Klemmleisten (0,25 x 1,1 mm),", "Fußplatte (3 x 0,15 mm), Klemmleisten (0,36 x 1,1 mm, auf der Glasachse aneinanderstoßend),"),
    ("No hatch: the clamp bars are 0.25 mm wide, the glass 0.2 mm.", "No hatch: the clamp bars are 0.36 mm wide, the glass 0.2 mm."),
    ("Keine Schraffur: die Klemmleisten sind 0,25 mm breit, das Glas 0,2 mm.", "Keine Schraffur: die Klemmleisten sind 0,36 mm breit, das Glas 0,2 mm."),
    ("each elastomer strip into its clamp bar, bolt head, shank and nut into one rod.", "each elastomer strip into its clamp bar, bolt head, shank and nut into one rod; the two clamp bars meet on the glass axis with the glass stroke over their joint, because the 21.5 mm slot (0.29 mm at 1:75) would leave two unprintable gaps beside the stroke."),
    ("jeder Elastomerstreifen in seiner Klemmleiste, Schraubenkopf, Schaft und Mutter in einem Stab.", "jeder Elastomerstreifen in seiner Klemmleiste, Schraubenkopf, Schaft und Mutter in einem Stab; die beiden Klemmleisten stoßen auf der Glasachse aneinander, der Glasstrich liegt über ihrer Fuge, weil der Schlitz von 21,5 mm (0,29 mm bei 1:75) neben dem Strich zwei nicht druckbare Spalte ließe."),
]


def block(text, start, sid):
    """🔎 The span of the shape object `sid` after `start`."""
    i = text.index('"id": "' + sid + '"', start)
    return i, text.index('"closed"', i)


def patched(text):
    """✂️ The file with the 1:100 elevation clamp bars meeting on the axis and the decisions updated."""
    rep = text.index('"100": {', text.index('"representations"'))
    elev = text.index('"elevation"', rep)
    for sid, old, new in (("clamp-bar-fixed", "-0.01075\n", "0.0\n"), ("clamp-bar-clamp", "0.01075\n", "0.0\n")):
        a, b = block(text, elev, sid)
        seg = text[a:b]
        assert seg.count(old) == 2, (sid, seg.count(old))
        text = text[:a] + seg.replace(old, new) + text[b:]
    for old, new in TEXTS:
        assert text.count(old) == 1, old
        text = text.replace(old, new)
    geo = json.loads(text)["asset"]["representations"]["100"]["geometry"]["elevation"]
    bars = {s["id"]: s["points"] for s in geo if s["id"].startswith("clamp-bar")}
    assert max(p[1] for p in bars["clamp-bar-fixed"]) == 0.0 and min(p[1] for p in bars["clamp-bar-clamp"]) == 0.0
    return text


def main():
    with open(PATH, encoding="utf-8") as f:
        out = patched(f.read())
    os.makedirs(os.path.dirname(CAND), exist_ok=True)
    with open(CAND, "w", encoding="utf-8") as f:
        f.write(out)
    print("candidate", CAND)
    if "--apply" in sys.argv:
        with open(PATH, encoding="utf-8") as f:
            out = patched(f.read())
        with open(PATH, "w", encoding="utf-8") as f:
            f.write(out)
        print("applied", PATH)


if __name__ == "__main__":
    main()
