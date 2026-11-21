"""📏 Round 5: narrows the declared valid range of a representation band to where its thinnest drawn feature stays printable (office-laser 0.2 mm), by text surgery on the catalog file so member order and formatting stay.

@see ./r5-brief.md
@see ./r3-tools.md
"""
import json, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/{}/🔣️.json"
EDITS = {"scissor-lift": {"200": 200}, "oil-pumpjack": {"500": 580}}
for slug, bands in EDITS.items():
    path = CAT.format(slug)
    text = open(path, encoding="utf-8").read()
    doc = json.loads(text)
    for band, den in bands.items():
        vr = doc["asset"]["representations"][band]["validRange"]
        old = json.dumps(vr, indent=1).replace("\n", "\n    ")
        start = text.index('"validRange"', text.index('"denominator": %s' % band, text.index('"representations"')))
        seg_end = text.index("}", start)
        seg = text[start:seg_end]
        new = seg.replace('"maxDenominator": %d' % vr["maxDenominator"], '"maxDenominator": %d' % den)
        if new == seg:
            continue
        text = text[:start] + new + text[seg_end:]
        print(slug, band, vr["maxDenominator"], "->", den)
    check = json.loads(text)
    for band, den in bands.items():
        assert check["asset"]["representations"][band]["validRange"]["maxDenominator"] == den
    open(path, "w", encoding="utf-8").write(text)
