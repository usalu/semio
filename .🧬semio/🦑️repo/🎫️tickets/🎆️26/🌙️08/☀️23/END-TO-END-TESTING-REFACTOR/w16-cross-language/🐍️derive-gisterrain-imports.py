#!/usr/bin/env python3
"""🏔️ Derives the `mutate-gisterrain-1` case fixture ONCE from committed real content.

Provenance, in full:

* ``✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio``
  gives the `exaggeration` (1.5) and the `mesh` composed-child handle, read out of the committed
  document's own `mesh=[hex(childId),hex(target)]` member.
* ``✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio``
  gives the `importedFeaturesJson` payload: the two REAL Liège positions — Institut de Botanique
  and Lycée Block 3000 — with their true WGS84 coordinates, rendered in the `{id, lat, lon}`
  descriptor shape the committed `change-imported-features` specification vector demonstrates.
* Output ``…/🧪️tests/mutate-gisterrain-1/🧫️fixtures/🏔️liege-terrain.snapshot.json``.

Why it exists: the committed terrain example carries an EMPTY `importedFeaturesJson`, so
`change-imported-features` would replace nothing with something and its inverse would restore
emptiness — a vacuous law. Every number in the output is a coordinate already committed in this
plugin; nothing is invented.
"""

# region 🔖️Imports
import json
import os
import sys

# endregion 🔖️Imports


# region 🔖️Carrier
def members(path):
    """📖️ The `key=value` members of a `.dsl.semio` document, banner excluded."""
    table = {}
    for line in open(path, encoding="utf-8").read().splitlines()[1:]:
        key, separator, value = line.partition("=")
        if separator == "=":
            table.setdefault(key, value)
    return table


def child_handle(payload):
    """🔗️ Reads a `[hex(childId),hex(target)]` composed-child member into its committed JSON shape."""
    child, target = [bytes.fromhex(part).decode("utf-8") for part in payload.strip("[]").split(",")]
    artifact, _, dialect = target.partition("!")
    kind, _, rest = dialect.partition("@")
    standard, _, subset = rest.partition("/")
    return {"childId": child, "target": {"artifactId": artifact, "dialect": {"artifactKind": kind, "standard": standard, "subset": subset}}}


# endregion 🔖️Carrier


# region 🔖️Derivation
def derive(terrain_path, gismap_path, target):
    """🧬️ Writes the derived snapshot: real exaggeration, real mesh handle, real imported positions."""
    terrain = members(terrain_path)
    gismap = members(gismap_path)
    positions = json.loads(bytes.fromhex(gismap["positions"]).decode("utf-8"))
    if terrain["importedFeaturesJson"] != "":
        raise SystemExit("the committed terrain example is expected to carry an empty importedFeaturesJson")
    descriptor = {"positions": [{"id": feature["id"], "lat": feature["data"]["lat"], "lon": feature["data"]["lon"]} for feature in positions], "routes": [], "regions": []}
    snapshot = {
        "exaggeration": float(terrain["exaggeration"]),
        "importedFeaturesJson": json.dumps(descriptor, separators=(",", ":"), ensure_ascii=False),
        "mesh": child_handle(terrain["mesh"]),
    }
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(snapshot, ensure_ascii=False, indent=2) + "\n")
    print("wrote %s (%d imported position(s))" % (target, len(descriptor["positions"])))


# endregion 🔖️Derivation


if __name__ == "__main__":
    derive(sys.argv[1], sys.argv[2], sys.argv[3])
