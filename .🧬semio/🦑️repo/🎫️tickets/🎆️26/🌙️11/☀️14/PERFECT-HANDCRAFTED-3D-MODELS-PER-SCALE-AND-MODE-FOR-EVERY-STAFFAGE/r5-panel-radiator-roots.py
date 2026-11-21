"""🔥 Round 5 panel radiator root fix (2D 1:200): the H-block with its unions and the two pipes are drawn as one connection column from the floor to the radiator (front 100 x 110 mm, elevation 110 x 36 mm), because the pipes below the block were 0.19 mm at 1:350, the end of the 1:200 band, and no gap to the next band is allowed; the shape `pipes` is removed and the 1:200 decisions say so. Targeted text replacements in the 1:200 representation only; candidate under `🗑️generated/r5-panel-radiator/🔣️.json` first.

Also the 1:100 front gains the pressed channels as a texture of every second seam between the header lines (decisions `retained` and `omitted` en/de).

Usage: python3 r5-panel-radiator-roots.py [--apply]

@see ./r5-brief.md
@see ./r3-components-panel-radiator.md
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
PATH = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/panel-radiator/🔣️.json"
CAND = os.path.join(T, "🗑️generated", "r5-panel-radiator", "🔣️.json")
SEAMS100 = [round(-0.5 + 2 * k / 30, 5) for k in range(1, 15)]
CHANNELS100 = {
    "id": "panel-channels",
    "bone": "radiator",
    "role": "texture",
    "z": 7,
    "filled": False,
    "tone": 0.5,
    "kind": "path",
    "d": " ".join("M %g 0.15 L %g 0.64" % (w, w) for w in SEAMS100) + " M -0.5 0.15 L 0.5 0.15 M -0.5 0.64 L 0.5 0.64",
}
TEXTS100 = [
    ("Panel body, centre split, grille, two bracket lines, thermostat head, H-block, pipes.", "Panel body, centre split, grille, two bracket lines, thermostat head, H-block, pipes. Front: the pressed channels as a vertical texture of every second seam (66.7 mm, 0.67 mm on paper, so the 0.1 mm lines keep their gap down to 1:150) between the two header lines."),
    ("Paneelkörper, Mittelteilung, Gitter, zwei Konsollinien, Thermostatkopf, Hahnblock, Rohre.", "Paneelkörper, Mittelteilung, Gitter, zwei Konsollinien, Thermostatkopf, Hahnblock, Rohre. Vorderansicht: die gepressten Kanäle als senkrechte Textur jeder zweiten Naht (66,7 mm, 0,67 mm auf Papier, so dass die 0,1-mm-Linien ihren Abstand bis 1:150 behalten) zwischen den beiden Sammelkanallinien."),
    ("Fin corrugation, grille slots, front channels, plugs.", "Fin corrugation, grille slots, every other channel seam, plugs."),
    ("Lamellenwellung, Gitterschlitze, Frontkanäle, Stopfen.", "Lamellenwellung, Gitterschlitze, jede zweite Kanalnaht, Stopfen."),
]
TEXTS = [
    ("Front: the grille lip is in the upper band; the two pipes are one column, the H-block takes in its unions.", "Front: the grille lip is in the upper band; the H-block with its unions and the two pipes are one connection column from the floor to the radiator (the pipes alone would be 0.19 mm at 1:350)."),
    ("Vorderansicht: die Gitterlippe liegt im oberen Band; die beiden Rohre sind eine Säule, der Hahnblock schließt die Verschraubungen ein.", "Vorderansicht: die Gitterlippe liegt im oberen Band; Hahnblock mit Verschraubungen und die beiden Rohre sind eine Anschlusssäule vom Boden bis zum Heizkörper (die Rohre allein messen bei 1:350 nur 0,19 mm)."),
]


def shape_span(text, start, view, sid):
    """🔎 The text span of shape `sid` in `view` after `start`."""
    v = text.index('"' + view + '"', start)
    i = text.index('"id": "' + sid + '"', v)
    return i, text.index("}", i)


def patched(text):
    """✂️ The file with the 1:200 H-block and pipes drawn as one connection column."""
    rep = text.index('"200": {', text.index('"representations"'))
    edits = [
        ("elevation", "connection-block", "-0.06,", "-0.11,", 2),
        ("front", "connection-block", '"y": 0.05,', '"y": 0.0,', 1),
        ("front", "connection-block", '"h": 0.06\n', '"h": 0.11\n', 1),
    ]
    for view, sid, old, new, n in edits:
        a, b = shape_span(text, rep, view, sid)
        seg = text[a:b]
        assert seg.count(old) == n, (view, sid, old, seg.count(old))
        text = text[:a] + seg.replace(old, new) + text[b:]
    a, b = shape_span(text, rep, "front", "pipes")
    a = text.rindex("{", 0, a)
    a = text.rindex(",", 0, a)
    text = text[:a] + text[b + 1:]
    for old, new in TEXTS:
        assert text.count(old) == 1, old
        text = text.replace(old, new)
    geo = json.loads(text)["asset"]["representations"]["200"]["geometry"]
    print({v: [s for s in geo[v] if s["id"] in ("connection-block", "pipes")] for v in ("elevation", "front")})
    return text


def add_channels100(text):
    """〰️ The 1:100 front gains the channel texture (every second seam) after the radiator outline, and the 1:100 decisions say so."""
    rep = text.index('"100": {', text.index('"representations"'))
    a, b = shape_span(text, rep, "front", "radiator-outline")
    end = text.index("},\n", b - 1) + 3
    body = json.dumps(CHANNELS100, indent=2, ensure_ascii=False).split("\n")
    block = "\n".join("            " + line for line in body) + ",\n"
    text = text[:end] + block + text[end:]
    for old, new in TEXTS100:
        assert text.count(old) == 1, old
        text = text.replace(old, new)
    ids = [x["id"] for x in json.loads(text)["asset"]["representations"]["100"]["geometry"]["front"]]
    assert ids.count("panel-channels") == 1, ids
    return text


def full(text):
    """🧰 Every round-5 root fix that is not yet in the file: the 1:200 connection column, the 1:100 channel texture."""
    rep = json.loads(text)["asset"]["representations"]
    if any(x["id"] == "pipes" for x in rep["200"]["geometry"]["front"]):
        text = patched(text)
    if not any(x["id"] == "panel-channels" for x in rep["100"]["geometry"]["front"]):
        text = add_channels100(text)
    return text


def main():
    with open(PATH, encoding="utf-8") as f:
        out = full(f.read())
    os.makedirs(os.path.dirname(CAND), exist_ok=True)
    with open(CAND, "w", encoding="utf-8") as f:
        f.write(out)
    print("candidate", CAND)
    if "--apply" in sys.argv:
        with open(PATH, encoding="utf-8") as f:
            out = full(f.read())
        with open(PATH, "w", encoding="utf-8") as f:
            f.write(out)
        print("applied", PATH)


if __name__ == "__main__":
    main()
