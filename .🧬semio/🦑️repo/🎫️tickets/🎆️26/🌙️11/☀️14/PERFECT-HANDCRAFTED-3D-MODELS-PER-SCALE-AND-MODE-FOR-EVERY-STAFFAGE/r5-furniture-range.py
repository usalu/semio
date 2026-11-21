"""🎚️ Round 5, furniture: writes a candidate asset document with changed representation valid ranges (and the volumes source merged) for `r3-validate.ts --file`, or applies the ranges to the catalog file in place with `--apply`.

Usage: python3 r5-furniture-range.py <slug> <scale>=<min>-<max> [...] [--apply]
@see ./r5-brief.md
"""
import json
import os
import re
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/{}/🔣️.json"


def ranges(specs):
    return {s.split("=")[0]: tuple(int(x) for x in s.split("=")[1].split("-")) for s in specs}


def main():
    args = [a for a in sys.argv[1:] if a != "--apply"]
    slug, wanted = args[0], ranges(args[1:])
    path = CATALOG.format(slug)
    if "--apply" in sys.argv:
        text = open(path, encoding="utf-8").read()
        reps = json.loads(text)["asset"]["representations"]
        pattern = re.compile(r'("denominator": (\d+),(?:(?!"denominator").)*?"validRange": \{\s*"minDenominator": )(\d+)(,\s*"maxDenominator": )(\d+)', re.S)

        def swap(m):
            scale = m.group(2)
            if scale not in wanted or str(reps[scale]["denominator"]) != scale:
                return m.group(0)
            lo, hi = wanted[scale]
            return f"{m.group(1)}{lo}{m.group(4)}{hi}"

        new = pattern.sub(swap, text)
        after = json.loads(new)["asset"]["representations"]
        assert all(after[s]["validRange"] == {"minDenominator": lo, "maxDenominator": hi} for s, (lo, hi) in wanted.items()), "validRange not found"
        open(path, "w", encoding="utf-8").write(new)
        print("applied", path)
        return
    doc = json.load(open(path, encoding="utf-8"))
    for scale, (lo, hi) in wanted.items():
        doc["asset"]["representations"][scale]["validRange"] = {"minDenominator": lo, "maxDenominator": hi}
    doc["asset"]["volumes"] = json.load(open(f"volumes/furniture/{slug}.json", encoding="utf-8"))["volumes"]
    out = f"🗑️generated/r5-{slug}/cand/🔣️.json"
    os.makedirs(os.path.dirname(out), exist_ok=True)
    json.dump(doc, open(out, "w", encoding="utf-8"), ensure_ascii=False)
    print(out)


main()
