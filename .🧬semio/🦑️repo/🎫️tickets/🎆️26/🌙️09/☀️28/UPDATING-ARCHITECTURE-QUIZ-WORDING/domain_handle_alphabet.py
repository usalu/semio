#!/usr/bin/env python3
"""🔤️ Derives the handle alphabet of the quiz contract from the Unicode Character Database and holds the schema to it.

Run from the repository root: ``.venv/Scripts/python.exe <this file>``. The letters are every code point of the blocks
Basic Latin, Latin-1 Supplement, Latin Extended-A, Latin Extended-B and Latin Extended Additional whose general
category is ``Lu`` or ``Ll`` and that has no compatibility decomposition (NFKC leaves it unchanged). Prints the ranges
as they appear in ``$defs/Handle`` of ``🧬️schema/🔣️.json`` and in both cores, and exits non-zero when the schema
pattern accepts another set of single characters, when a letter is not NFC-stable, or when two characters of the
alphabet compose. The pattern is ECMA 262 as JSON Schema requires; Python's ``$`` also matches before a final line
feed, so a match must end at the end of the text.
"""

import json
import os
import re
import sys
import unicodedata

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..", "..", ".."))
SCHEMA = os.path.join(ROOT, "🧰️framework", "🛍️products", "❓️quiz", "🧬️schema", "🔣️.json")
BLOCKS = [(0x0000, 0x007F), (0x0080, 0x00FF), (0x0100, 0x017F), (0x0180, 0x024F), (0x1E00, 0x1EFF)]
OTHERS = "0123456789'._-"


def letters():
    """🔡️ The letter code points, ascending."""
    return [point for low, high in BLOCKS for point in range(low, high + 1) if unicodedata.category(chr(point)) in ("Lu", "Ll") and unicodedata.normalize("NFKC", chr(point)) == chr(point)]


def ranges(points):
    """📏️ Ascending code points as inclusive ranges."""
    found = []
    for point in points:
        if found and found[-1][1] == point - 1:
            found[-1][1] = point
        else:
            found.append([point, point])
    return found


def main():
    """🧪️ Prints the table and checks the schema against it."""
    table = letters()
    print("unicode %s, %d letters, ranges: %s" % (unicodedata.unidata_version, len(table), " ".join("%04X-%04X" % (low, high) for low, high in ranges(table))))
    with open(SCHEMA, "r", encoding="utf-8") as handle:
        pattern = re.compile(json.load(handle)["$defs"]["Handle"]["pattern"])
    alphabet = set(table) | {ord(character) for character in OTHERS}
    failures = 0
    for point in range(0x110000):
        if 0xD800 <= point <= 0xDFFF:
            continue
        found = pattern.search("a" + chr(point))
        accepted = found is not None and found.end() == 2
        if accepted != (point in alphabet):
            failures += 1
            print("[ERROR] U+%04X: the schema pattern %s it, the database %s it" % (point, "accepts" if accepted else "refuses", "holds" if point in alphabet else "lacks"))
    members = sorted(alphabet | {0x20})
    for point in members:
        if unicodedata.normalize("NFC", chr(point)) != chr(point) or unicodedata.combining(chr(point)) != 0:
            failures += 1
            print("[ERROR] U+%04X is not NFC-stable" % point)
    composing = sum(1 for first in members for second in members if unicodedata.normalize("NFC", chr(first) + chr(second)) != chr(first) + chr(second))
    print("pairs that compose: %d of %d" % (composing, len(members) ** 2))
    sys.exit(1 if failures or composing else 0)


if __name__ == "__main__":
    main()
