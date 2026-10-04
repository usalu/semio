"""🥞️ The shared home-grid vectors of §17: the card layer's `columns` counts give way to `layouts` — per viewport width
the layout of the overview and the fr weights of the live grid's columns and rows the card layer shares (`null` rows:
the list below tablets, one column). The description says the same. Exact anchors; the rest of the file is untouched."""

import pathlib
import re
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🧫️fixtures/🏠️home-grid/🔣️.json")
text = path.read_text(encoding="utf-8")
OLD_SENTENCE = "Columns of the card layer by viewport width at the design system's breakpoints: one up to the mobile maximum, two up to the tablet maximum with the leaderboard spanning the row, three from there with the leaderboard in the larger centre column."
NEW_SENTENCE = "At rest the overview is a live grid of every page and the card layer shares its tracks, each card in its page's cell; by viewport width at the design system's breakpoints: a list of sections in one column up to the mobile maximum, two equal columns over five equal rows up to the tablet maximum (the leaderboard alone on its row), three columns weighted 1 : 1.5 : 1 over three rows weighted 1 : 1.4 : 1 from there (the leaderboard in the larger centre cell)."
if text.count(OLD_SENTENCE) != 1:
    sys.exit("description anchor")
text = text.replace(OLD_SENTENCE, NEW_SENTENCE)
block = re.search(r'  "columns": \[\n(?:.|\n)*?\n  \],\n', text)
if block is None or text.count('  "columns": [\n') != 1:
    sys.exit("columns block")
LAYOUTS = [
    (320, "list", "[1]", "null"),
    (375, "list", "[1]", "null"),
    (767, "list", "[1]", "null"),
    (768, "tablet", "[1, 1]", "[1, 1, 1, 1, 1]"),
    (1023, "tablet", "[1, 1]", "[1, 1, 1, 1, 1]"),
    (1024, "desktop", "[1, 1.5, 1]", "[1, 1.4, 1]"),
    (1440, "desktop", "[1, 1.5, 1]", "[1, 1.4, 1]"),
]
entries = ",\n".join(f'    {{\n      "width": {width},\n      "layout": "{layout}",\n      "columns": {columns},\n      "rows": {rows}\n    }}' for width, layout, columns, rows in LAYOUTS)
text = text[: block.start()] + f'  "layouts": [\n{entries}\n  ],\n' + text[block.end():]
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[live grid fixture] done")
