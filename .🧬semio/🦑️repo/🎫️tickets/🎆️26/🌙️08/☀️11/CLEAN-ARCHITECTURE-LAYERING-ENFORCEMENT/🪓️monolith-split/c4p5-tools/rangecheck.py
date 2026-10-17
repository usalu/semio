"""📐 C4P5 range check: every segment of a modsplit/fragsplit spec starts and ends at brace depth 0, outside literals, with no orphan doc/attribute at either seam, and every original line is claimed at most once.
usage: rangecheck.py <spec.json>   (fragment specs use `ranges` instead of `segments`)"""
import json, os, re, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rf8-tools")
from rslex import lex, multiline_literal_lines

spec = json.load(open(sys.argv[1], encoding="utf-8"))
text = open(spec["original"], encoding="utf-8").read()
ls = text.split("\n")
if ls and ls[-1] == "":
    ls.pop()
depths, inlit = lex(text)
ml = multiline_literal_lines(text)
owner = {}
bad = 0
ORPHAN = re.compile(r"^\s*(///|#\[|#!\[)")


def check(who, a, b):
    global bad
    for n in range(a, b + 1):
        if n in owner:
            print(f"CLAIMED TWICE line {n}: {owner[n]} and {who}")
            bad += 1
        owner[n] = who
    if depths[a - 1] != 0 or inlit[a - 1]:
        print(f"{who}: range {a}-{b} starts at depth {depths[a-1]} / in literal {inlit[a-1]}")
        bad += 1
    nxt = depths[b] if b < len(depths) else 0
    if nxt != 0 or (b < len(inlit) and inlit[b]):
        print(f"{who}: range {a}-{b} ends at depth {nxt}")
        bad += 1
    if a >= 2 and ORPHAN.match(ls[a - 2]):
        print(f"{who}: line {a-1} before the range is an orphan doc/attribute: {ls[a-2][:80]}")
        bad += 1
    if b < len(ls) and ls[b - 1].strip() and ORPHAN.match(ls[b - 1]):
        print(f"{who}: last line {b} is an orphan doc/attribute: {ls[b-1][:80]}")
        bad += 1


for c in spec["chunks"]:
    for s in c.get("segments", []):
        check(c["name"], *s["lines"])
        if "wrap" in s:
            for k in ("header", "close"):
                pass
    for r in c.get("ranges", []):
        check(c.get("folder", c.get("name", "?")), *r)
    for d in c.get("drop", []):
        check(c["name"] + ":drop", *d)
print(f"multiline-literal lines in original: {len(ml)}; claimed lines: {len(owner)} of {len(ls)}; problems: {bad}")
sys.exit(1 if bad else 0)
