"""🖨️ Round 5 root fixes of humans/child-on-kick-scooter found by the print-suitability check of `r3-validate.ts` (2026-11-20): the 1:50 spoon brake drawn seated on the deck top at its hinge (it hung 3 mm above the deck, a 0.07 mm gap at 1:20), and the valid ranges of 1:200 and 1:500 narrowed where the 68 mm scooter base and the 145 mm wheel fall below the 0.2 mm laser minimum (1:200 up to 1:330, 1:500 from 1:330 to its nominal 1:500, beyond which the 30 mm wheel seen edge-on in the front view falls under the laser minimum in the pushing poses).

Usage: python3 r5-child-on-kick-scooter-root.py [--write]   (without --write: the candidate `🗑️generated/r5-child-on-kick-scooter/🔣️.json` only)
"""
import glob
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
CAT = glob.glob(f"{S}/*/child-on-kick-scooter/🔣️.json")[0]
OUT = os.path.join(T, "🗑️generated", "r5-child-on-kick-scooter", "🔣️.json")

EDITS = [(r'("maxDenominator": )350(\n)', r'\g<1>330\2'), (r'("minDenominator": )350(,\n\s*"maxDenominator": )1000\b', r'\g<1>330\g<2>500'), (r'(\[\s*0\.0937,\s*)0\.088(\s*\])', r'\g<1>0.0845\2'), (r'(\[\s*0\.0852,\s*)0\.0925(\s*\])', r'\g<1>0.0845\2')]

text = open(CAT, encoding="utf-8").read()
for pattern, repl in EDITS:
    text, n = re.subn(pattern, repl, text)
    if n != 1:
        raise SystemExit(f"{pattern}: {n} matches")
json.loads(text)
open(CAT if "--write" in sys.argv else OUT, "w", encoding="utf-8").write(text)
print(CAT if "--write" in sys.argv else OUT)
