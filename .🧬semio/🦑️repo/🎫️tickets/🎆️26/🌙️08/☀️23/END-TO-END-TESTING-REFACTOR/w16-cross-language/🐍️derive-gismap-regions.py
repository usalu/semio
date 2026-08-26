#!/usr/bin/env python3
"""🗺️ Derives the `mutate-gismap-1` case fixture ONCE from the committed real Liège example.

Provenance, in full:

* Input  `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`
  — the artifact's own committed demo document: two real positions (Institut de Botanique,
  Lycée Block 3000) and two real routes (Holz Fassade, Stahl Canopy) with their true WGS84
  coordinates, and an EMPTY `regions` collection.
* Output `…/🧪️tests/mutate-gismap-1/🧫️fixtures/🗺️liege-with-derived-regions.dsl.semio`
  — the same document with three regions ADDED. Nothing is removed, edited or invented: every
  number in the added regions is computed from coordinates already in the input.
    - `region-holz-fassade-envelope`  = axis-aligned bounding box of route 1's polyline
    - `region-stahl-canopy-envelope`  = axis-aligned bounding box of route 2's polyline
    - `region-campus-envelope`        = axis-aligned bounding box of BOTH positions

Why it exists: three of the twelve declared kinds (`delete-region`, `replace-region-data`,
`reorder-regions`) address an EXISTING region, and the committed example carries none. Running them
against a document with no regions would exercise only the rejection path and would make
`inverse-` vacuous. This is the "derive a complex artifact once from real committed content and
record its provenance" route, taken deliberately and recorded here rather than in a comment.
"""

# region 🔖️Imports
import json
import os
import sys

# endregion 🔖️Imports


# region 🔖️Carrier
def read_carrier(path):
    """📖️ Splits a `gis.gismap.dsl v1` document into its banner and its `key=value` lines."""
    text = open(path, encoding="utf-8").read()
    lines = text.split("\n")
    banner, body = lines[0], lines[1:]
    fields = []
    for line in body:
        if not line:
            continue
        key, _, value = line.partition("=")
        fields.append((key, value))
    return banner, fields, text


def hex_json(value):
    """#⃣ The carrier's collection encoding: hex of the UTF-8 bytes of compact JSON."""
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode("utf-8").hex()


def json_hex(payload):
    """🔎️ The inverse of :func:`hex_json`."""
    return json.loads(bytes.fromhex(payload).decode("utf-8"))


# endregion 🔖️Carrier


# region 🔖️Derivation
def envelope(points, identifier, label):
    """📐️ The axis-aligned bounding box of a real polyline, as a closed five-point ring."""
    lons = [point[0] for point in points]
    lats = [point[1] for point in points]
    west, east, south, north = min(lons), max(lons), min(lats), max(lats)
    ring = [[west, south], [east, south], [east, north], [west, north], [west, south]]
    return {"id": identifier, "data": {"id": identifier, "label": label, "points": ring}}


def derive(source, target):
    """🧬️ Writes the derived document, leaving every committed field untouched but `regions`."""
    banner, fields, _ = read_carrier(source)
    table = dict(fields)
    routes = json_hex(table["routes"])
    positions = json_hex(table["positions"])
    if json_hex(table["regions"]) != []:
        raise SystemExit("the committed example is expected to carry an empty regions collection")
    corners = [[point["data"]["lon"], point["data"]["lat"]] for point in positions]
    regions = [
        envelope(routes[0]["data"]["points"], "region-holz-fassade-envelope", "Holz Fassade Envelope"),
        envelope(routes[1]["data"]["points"], "region-stahl-canopy-envelope", "Stahl Canopy Envelope"),
        envelope(corners, "region-campus-envelope", "Campus Envelope"),
    ]
    table["regions"] = hex_json(regions)
    rendered = banner + "\n" + "\n".join("%s=%s" % (key, table[key]) for key, _ in fields) + "\n"
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(rendered)
    print("wrote %s (%d bytes, %d region(s))" % (target, len(rendered.encode("utf-8")), len(regions)))


# endregion 🔖️Derivation


if __name__ == "__main__":
    derive(sys.argv[1], sys.argv[2])
