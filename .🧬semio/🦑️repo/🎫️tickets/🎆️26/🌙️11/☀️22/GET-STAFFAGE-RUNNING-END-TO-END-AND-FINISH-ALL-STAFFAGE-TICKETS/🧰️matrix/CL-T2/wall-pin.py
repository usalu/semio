"""📌️ Moves the pin of the bonded brick wall in `🧪️tests/🧊️model-bounds` to the two alternating bond bands of 1:200 (`bond-band-a`, `bond-band-b`: two solids, six copies, one span, the form of the 1:100 line), reading the file right before the write; a second call changes nothing. `python3 wall-pin.py [--dry]`"""
import sys
from pathlib import Path

path = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-bounds/🟦️.ts")
swaps = [
    ("  200: { parts: /^bond-band$/, count: 1, copies: 6, spans: 1, absent: [/^plaster$/, /^render$/, /^bed-joint$/, /^course-/], words: /the wall is six bond bands of 167 mm/ },", "  200: { parts: /^bond-band-[ab]$/, count: 2, copies: 6, spans: 1, absent: [/^plaster$/, /^render$/, /^bed-joint$/, /^course-/], words: /the wall is six bond bands of 167 mm/ },"),
    ("absent: [/^plaster$/, /^render$/, /^bed-joint$/, /^course-/, /^bond-band$/], words: /the wall is one homogeneous 400 mm body/", "absent: [/^plaster$/, /^render$/, /^bed-joint$/, /^course-/, /^bond-band/], words: /the wall is one homogeneous 400 mm body/"),
    ("twelve courses in two repeated bands that touch, six bond bands that touch, one wall", "twelve courses in two repeated bands that touch, six bond bands in two repeated bands that touch, one wall"),
]
text = path.read_text(encoding="utf-8")
changed = 0
for old, new in swaps:
    if new in text:
        continue
    if old not in text:
        sys.exit(f"wall pin: text to replace is not there: {old[:80]}")
    text = text.replace(old, new)
    changed += 1
if changed and "--dry" not in sys.argv:
    path.write_text(text, encoding="utf-8")
print(f"wall pin: {changed} lines {'would change' if '--dry' in sys.argv else 'changed'}")
