#!/usr/bin/env python3
"""🧱️ Inventories every fixed-size slot-table construction in the tree and, when a frame-size table
from `🐍️wgpu-frame-sizes.py` is supplied, joins each site's owning type to its measured `sub sp`
prologue reservation (Rust v0 symbols are length-prefixed, so the type name is read back exactly).

Two shapes are collected:
  * `boxed`  — `Box::new([…; N])` / `Box::new(*::array::from_fn(…))`: a heap destination with a stack
    source, so the whole array is still materialised in the caller's frame at `opt-level = 0`.
  * `inline` — `*::array::from_fn(…)` assigned straight into an array field: the array IS the struct,
    so the frame cost is paid on construction AND on every by-value move of the owner.

Instrument only — nothing here ships.
Usage: boxed-slot-sites.py <repo-root> [frames.tsv] > sites.tsv
"""
import os, re, sys

ROOTS = ("🧰️framework", "✏️s", "🌎️hub")
BOXED = re.compile(r"Box::new\(\s*(?:\[|(?:std|core)::array::from_fn)")
INLINE = re.compile(r"(?<!Box::new\()(?:std|core)::array::from_fn\(")
OWNER = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|impl(?:<[^>]*>)?)\s+(?:[A-Za-z_:<>, ]*?\bfor\s+)?([A-Za-z_][A-Za-z0-9_]*)")

def v0_identifiers(symbol):
    """🔤️ Reads a Rust v0 mangled symbol's length-prefixed identifiers, resuming the scan after each
    one — `re.findall` cannot, because its greedy tail swallows the next prefix."""
    names, index, end = [], 0, len(symbol)
    while index < end:
        if not symbol[index].isdigit() or (index and symbol[index - 1].isdigit()):
            index += 1
            continue
        digits = index
        while digits < end and symbol[digits].isdigit():
            digits += 1
        length = int(symbol[index:digits])
        if digits < end and symbol[digits].isalpha() and digits + length <= end:
            names.append(symbol[digits : digits + length])
            index = digits + length
        else:
            index = digits
    return [name for name in names if len(name) > 3]


root = sys.argv[1]
frames = {}
if len(sys.argv) > 2:
    for line in open(sys.argv[2], encoding="utf-8", errors="replace"):
        size, _, name = line.rstrip("\n").partition("\t")
        for ident in v0_identifiers(name):
            frames[ident] = max(frames.get(ident, 0), int(size))

rows = []
for base in ROOTS:
    for folder, _, files in os.walk(os.path.join(root, base)):
        if "🗑️generated" in folder or "node_modules" in folder:
            continue
        for name in files:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(folder, name)
            owner = ""
            for number, line in enumerate(open(path, encoding="utf-8", errors="replace"), 1):
                found = OWNER.match(line)
                if found:
                    owner = found.group(1)
                shape = "boxed" if BOXED.search(line) else ("inline" if INLINE.search(line) else None)
                if shape:
                    rows.append((frames.get(owner, 0), shape, owner, os.path.relpath(path, root), number, line.strip()[:140]))

rows.sort(reverse=True)
print("measuredFrameBytes\tshape\towner\tfile\tline\tsource")
for row in rows:
    print("\t".join(str(cell) for cell in row))
print(f"# {len(rows)} sites", file=sys.stderr)
