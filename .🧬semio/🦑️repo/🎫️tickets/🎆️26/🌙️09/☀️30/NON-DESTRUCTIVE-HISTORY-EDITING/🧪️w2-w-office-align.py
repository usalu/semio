"""🧮️ W2-W-office: re-aligns every Examples/data table of the given `🥒️.feature` files in place.

Usage: python3 🧪️w2-w-office-align.py <feature> [<feature> ...]
Cells are split on `|` exactly like the harness does, trimmed, and padded to their column's widest cell.
"""
import sys


def align(lines):
    out, block = [], []

    def flush():
        if not block:
            return
        indent = block[0][: len(block[0]) - len(block[0].lstrip())]
        rows = [[cell.strip() for cell in line.strip()[1:-1].split("|")] for line in block]
        widths = [max(len(row[i]) for row in rows if i < len(row)) for i in range(max(len(row) for row in rows))]
        for row in rows:
            out.append(indent + "| " + " | ".join(cell.ljust(widths[i]) for i, cell in enumerate(row)) + " |\n")
        block.clear()

    for line in lines:
        stripped = line.strip()
        if stripped.startswith("|") and stripped.endswith("|"):
            block.append(line.rstrip("\n"))
        else:
            flush()
            out.append(line)
    flush()
    return out


for path in sys.argv[1:]:
    with open(path, encoding="utf-8") as handle:
        lines = handle.readlines()
    with open(path, "w", encoding="utf-8") as handle:
        handle.writelines(align(lines))
