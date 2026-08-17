#!/usr/bin/env python3
"""Handcraft a small IANA-style TSV (no quoting, tab-separated) file:
header row + 4 data rows, one row documents the "no quoting" edge case in
a value that itself contains characters adjacent to tabs conceptually
(we keep the file itself strictly valid TSV: no literal embedded tabs or
newlines inside a field, since IANA TSV has no escape mechanism for them --
that constraint is what NOTES.md documents)."""
import sys

OUT = sys.argv[1]

HEADER = ["id", "name", "qty", "unit_price", "note"]
ROWS = [
    ["1", "Oak Panel", "12", "18.50", "in stock"],
    ["2", "Steel Bracket L-90", "48", "2.05", "backordered"],
    ["3", "Glass Pane 4mm", "6", "44.99", "fragile;handle with care"],
    ["4", "Cable Tie 200mm", "500", "0.03", "bag of 100 -> qty is bags"],
    ["5", "Weathering Test\\tSample", "1", "0.00", "literal backslash-t, NOT a real tab -- see NOTES.md"],
]

def main():
    lines = ["\t".join(HEADER)] + ["\t".join(r) for r in ROWS]
    text = "\n".join(lines) + "\n"  # trailing newline present
    with open(OUT, "w", newline="") as fh:
        fh.write(text)
    return {
        "rows": len(ROWS),
        "columns": len(HEADER),
        "trailing_newline": True,
        "line_ending": "\\n (LF)",
        "total_bytes": len(text.encode("utf-8")),
    }

if __name__ == "__main__":
    print(main())
