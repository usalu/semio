"""🪚 Chisel helpers of author 07: write hand-chosen loft section tables into a volumes source.

A table row is (u, v, w, a, b[, exponent]): centre in the bone frame, a = half extent in the sagittal plane (the loft `rx` of the limb rotation), b = lateral half extent (`ry`).
Rows are authored for the near side (w < 0); the far twin is the mirror in w.
"""
import json

LIMB = [0.5, 0.5, 0.5, 0.5]


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def save(path, document):
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")


def sections(rows, rotation=LIMB, exponent=2.2, mirror=False):
    out = []
    for row in rows:
        u, v, w, a, b = row[:5]
        out.append({"at": [u, v, -w if mirror else w], "rx": a, "ry": b, "exponent": row[5] if len(row) > 5 else exponent, "rotation": list(rotation)})
    return out


def solid(listing, ident):
    for entry in listing:
        if entry.get("id") == ident:
            return entry
    raise SystemExit("missing solid " + ident)


def has(listing, ident):
    return any(entry.get("id") == ident for entry in listing)


def reloft(listing, ident, rows, exponent=2.2, mirror=False, rotation=None):
    entry = solid(listing, ident)
    keep = rotation or entry["sections"][0]["rotation"]
    entry["sections"] = sections(rows, keep, exponent, mirror)
    entry["capStart"] = True
    entry["capEnd"] = True
    return entry


def pair(listing, name, rows, exponent=2.2, rotation=None):
    reloft(listing, "near-" + name, rows, exponent, False, rotation)
    reloft(listing, "far-" + name, rows, exponent, True, rotation)


def insert_after(listing, after, entry):
    for index, existing in enumerate(listing):
        if existing.get("id") == after:
            listing.insert(index + 1, entry)
            return entry
    raise SystemExit("missing solid " + after)


def remove(listing, ident):
    for index, existing in enumerate(listing):
        if existing.get("id") == ident:
            del listing[index]
            return
    raise SystemExit("missing solid " + ident)
