#!/usr/bin/env python3
"""Generates the mutate-semio-model case fixtures.

Base state is a faithful transcription of the REAL committed example artifact
`🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio`
(byte-identical to `demo_semio_model_snapshot()` per that subset's own fixture_honesty_law).

Apply semantics mirror `🪆️subsets/✳️model/🧬️schema/🔺️diff/🦀️component.rs`:
  apply_named = retain(not removed) -> patch modified in place -> push added at the END.
"""
import copy, json, os, sys

OUT = sys.argv[1]

IDENT = {
    "translation": {"x": 0.0, "y": 0.0, "z": 0.0},
    "rotation": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0},
    "scale": {"x": 1.0, "y": 1.0, "z": 1.0},
}


def xform(x=0.0, y=0.0, z=0.0):
    t = copy.deepcopy(IDENT)
    t["translation"] = {"x": x, "y": y, "z": z}
    return t


BASE = {
    "schema": "stdio.semio.model",
    "spatial": [
        {"id": "site-1", "kind": "site", "name": "Site One", "parentId": None, "placement": xform()},
        {"id": "storey-1", "kind": "storey", "name": "Ground Floor", "parentId": "site-1", "placement": xform(z=3.0)},
    ],
    "elements": [
        {
            "id": "wall-1",
            "class": {"kind": "wall"},
            "placement": xform(),
            "geometry": {"kind": "brep", "brep_id": "brep-1"},
            "spatialId": "storey-1",
            "psets": [
                {
                    "name": "Pset_WallCommon",
                    "properties": [
                        {"key": "IsExternal", "value": {"kind": "boolean", "value": True}},
                        {"key": "FireRating", "value": {"kind": "text", "value": "REI60"}},
                        {"key": "ThermalTransmittance", "value": {"kind": "number", "value": 0.24}},
                    ],
                }
            ],
        }
    ],
    "relations": [{"id": "rel-1", "kind": {"kind": "containedIn"}, "from": "wall-1", "to": "storey-1"}],
}

OFFICE = {"id": "space-1", "kind": "space", "name": "Office 1", "parentId": "storey-1", "placement": xform()}
DOOR = {"id": "door-1", "class": {"kind": "door"}, "placement": xform(x=1.2), "geometry": {"kind": "none"}, "spatialId": "storey-1", "psets": []}
REL2 = {"id": "rel-2", "kind": {"kind": "aggregates"}, "from": "site-1", "to": "storey-1"}

NEW_PSETS = [
    {
        "name": "Pset_WallCommon",
        "properties": [
            {"key": "IsExternal", "value": {"kind": "boolean", "value": False}},
            {"key": "FireRating", "value": {"kind": "text", "value": "REI90"}},
        ],
    }
]


def apply_named(items, removed=(), modified=(), added=()):
    out = [i for i in items if i["id"] not in removed]
    for key, patch in modified:
        for item in out:
            if item["id"] == key:
                item.update(copy.deepcopy(patch))
    return out + [copy.deepcopy(a) for a in added]


def with_spatial(base, **kw):
    s = copy.deepcopy(base)
    s["spatial"] = apply_named(s["spatial"], **kw)
    return s


def with_elements(base, **kw):
    s = copy.deepcopy(base)
    s["elements"] = apply_named(s["elements"], **kw)
    return s


def with_relations(base, **kw):
    s = copy.deepcopy(base)
    s["relations"] = apply_named(s["relations"], **kw)
    return s


SET_SNAPSHOT_TARGET = with_spatial(BASE, modified=[("storey-1", {"name": "First Floor", "placement": xform(z=6.0)})])
AFTER_INSERT_SPATIAL = with_spatial(BASE, added=[OFFICE])
AFTER_INSERT_ELEMENT = with_elements(BASE, added=[DOOR])

CASES = [
    ("no-mutation", BASE, {"mutation": "noMutation"}, BASE),
    ("set-snapshot", BASE, {"mutation": "setSnapshot", "snapshot": SET_SNAPSHOT_TARGET}, SET_SNAPSHOT_TARGET),
    ("insert-spatial-node", BASE, {"mutation": "insertSpatialNode", "node": OFFICE}, AFTER_INSERT_SPATIAL),
    ("remove-spatial-node", AFTER_INSERT_SPATIAL, {"mutation": "removeSpatialNode", "id": "space-1"}, BASE),
    (
        "set-spatial-node",
        BASE,
        {"mutation": "setSpatialNode", "id": "storey-1", "kind": None, "name": "First Floor", "placement": xform(z=6.0)},
        with_spatial(BASE, modified=[("storey-1", {"name": "First Floor", "placement": xform(z=6.0)})]),
    ),
    ("insert-element", BASE, {"mutation": "insertElement", "element": DOOR}, AFTER_INSERT_ELEMENT),
    ("remove-element", AFTER_INSERT_ELEMENT, {"mutation": "removeElement", "id": "door-1"}, BASE),
    (
        "set-element",
        BASE,
        {"mutation": "setElement", "id": "wall-1", "class": None, "placement": xform(x=3.5), "geometry": None, "psets": NEW_PSETS},
        with_elements(BASE, modified=[("wall-1", {"placement": xform(x=3.5), "psets": NEW_PSETS})]),
    ),
    ("insert-relation", BASE, {"mutation": "insertRelation", "relation": REL2}, with_relations(BASE, added=[REL2])),
    ("remove-relation", BASE, {"mutation": "removeRelation", "id": "rel-1"}, with_relations(BASE, removed={"rel-1"})),
    (
        "set-relation",
        BASE,
        {"mutation": "setRelation", "id": "rel-1", "kind": {"kind": "connectsTo"}, "from": None, "to": None},
        with_relations(BASE, modified=[("rel-1", {"kind": {"kind": "connectsTo"}})]),
    ),
]


def write(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


for kind, before, mutation, after in CASES:
    write(os.path.join(OUT, kind, "⬅️before.json"), before)
    write(os.path.join(OUT, kind, "🦠️mutation.json"), mutation)
    write(os.path.join(OUT, kind, "➡️after.json"), after)

print(f"model: {len(CASES)} kinds, {len(CASES) * 3} files -> {OUT}")
