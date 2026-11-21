"""📏 Round 5 root fix: sets `representations.<scale>.validRange.<bound>` of a catalog asset by a targeted text replacement (re-reads the file right before writing, changes only that number); writes a candidate under `🗑️generated/r5-<slug>/🔣️.json` first.

Usage: python3 r5-valid-range.py <category>/<slug> <scale> <minDenominator|maxDenominator> <value> [--apply]

@see ./r5-brief.md
"""
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
FOLDERS = {"components": "🧱️components"}


def patched(text, scale, bound, value):
    """✂️ The file text with the one bound of the scale's valid range replaced."""
    doc = json.loads(text)
    rep = doc["asset"]["representations"][scale]
    old = rep["validRange"][bound]
    start = text.index('"representations"')
    key = text.index('"' + scale + '": {', start)
    vr = text.index('"validRange"', key)
    m = re.compile('"' + bound + r'": ' + str(old) + r"\b").search(text, vr)
    out = text[: m.start()] + '"' + bound + '": ' + str(value) + text[m.end():]
    assert json.loads(out)["asset"]["representations"][scale]["validRange"][bound] == value
    return out, old


def main():
    asset, scale, bound, value = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
    cat, slug = asset.split("/")
    path = os.path.join(CATALOG, FOLDERS[cat], slug, "🔣️.json")
    with open(path, encoding="utf-8") as f:
        text = f.read()
    out, old = patched(text, scale, bound, value)
    cand = os.path.join(T, "🗑️generated", "r5-" + slug, "🔣️.json")
    os.makedirs(os.path.dirname(cand), exist_ok=True)
    with open(cand, "w", encoding="utf-8") as f:
        f.write(out)
    print("candidate", cand, scale, bound, old, "->", value)
    if "--apply" in sys.argv:
        with open(path, encoding="utf-8") as f:
            text = f.read()
        out, _ = patched(text, scale, bound, value)
        with open(path, "w", encoding="utf-8") as f:
            f.write(out)
        print("applied", path)


if __name__ == "__main__":
    main()
