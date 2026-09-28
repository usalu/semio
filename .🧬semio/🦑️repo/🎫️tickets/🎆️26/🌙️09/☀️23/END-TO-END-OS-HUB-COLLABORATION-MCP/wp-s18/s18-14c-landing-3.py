# -*- coding: utf-8 -*-
"""📝️ S18 §14c: landing row for the csv/tsv pin after LB2 p8 (idempotent)."""
import pathlib

LANDING = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️landing.md")
ROW = "| S18 | 14c item 1 follow-up — after LB2 p8 (row-scoped DOM ids) and the chain's stdio restage (windowed structural table), the `stdio.set-cell` rendered-edit pin selects `[id$=\"/row-0/cell-0\"]` (was the scene-table id `…::framework.window.table.0.0`, now absent); law fixture follows (header row proven excluded): `🧑‍💻dev/🧫️fixtures/🧮️program-matrix.json`, `🧑‍💻dev/🧪️tests/🧮️program-matrix/🧮️reducers/🟦️.ts`; codemod `wp-s18/s18-14c-csv-pin-p8.py` | TS/JSON only (dev harness) | tsc 0 (`s18-14c-tsc-harness-4.txt`), laws 4/4 native lane (`s18-14c-law-pins-7.txt`); live **stdio/csv + stdio/tsv editors PASS** (edits [0,1,0,1], `Set Cell↶`, 0 faults — `.🧬semio/🌐hub/s14-s18-logs/matrix-s18-14c-stdio-p8b-en.txt`) | 28 19:4x |"


def main() -> None:
    text = LANDING.read_text(encoding="utf-8")
    if ROW[:70] not in text:
        LANDING.write_text(text.rstrip("\n") + "\n" + ROW + "\n", encoding="utf-8")
        print("added 1")
    else:
        print("added 0")


main()
