#!/usr/bin/env python3
"""📏 Print the longest source-side and ticket-side Windows paths in UTF-16 units (read-only)."""
import os

ROOT = "C:\\git\\semio\\"
GEN = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "pl")


def units(s):
    return len(s.encode("utf-16-le")) // 2


def main():
    rows = []
    for line in open(os.path.join(GEN, "src-list.txt"), encoding="utf-8"):
        xy, rel = line.rstrip("\n")[:2], line.rstrip("\n")[3:]
        full = ROOT + rel.replace("/", "\\")
        rows.append((units(full), len(full), xy, rel))
    rows.sort(reverse=True)
    for r in rows[:5]:
        print("SRC", r)
    print("SRC count", len(rows), "max units", rows[0][0] if rows else 0)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
