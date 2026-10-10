#!/usr/bin/env python3
"""Registers the two oracles of `w2-wp20-energy-io` in the oracle manifest of the BIM subset (`🔮️oracles/🔣️.json`): `bim-1-lxml-gbxml` (capability `bim-1-export-gbxml`) and `bim-1-ifcopenshell-ifc-energy`
(capability `bim-1-export-ifc-energy`). The manifest is read as JSON and written back with two-space indentation (an unchanged manifest round-trips byte for byte); already registered ids are kept.
"""
import json
import os
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
S = T.parents[6] / "✏️s" / "🔌️plugins" / "🏙️bim" / "🗿️artifacts" / "🏢️model" / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any"
MANIFEST = S / "🔮️oracles" / "🔣️.json"

ROWS = [
    {
        "id": "bim-1-lxml-gbxml",
        "ecosystem": "python",
        "package": "lxml",
        "version": "6.1.3",
        "capabilities": ["bim-1-export-gbxml"],
        "comparisonProfiles": ["floating-point-v1"],
        "license": "BSD-3-Clause",
        "testOnly": True,
        "homepage": "https://lxml.de",
        "rationale": "lxml (libxml2) is a namespace-aware XML parser written by unrelated authors, shapely (GEOS) is the reference computational-geometry library and numpy evaluates the vector algebra. lxml opens the committed gbXML 7.03 exports of the BIM thermal envelope (files the subject wrote through the stdio XML writer), checks the root, units, xs:ID syntax and uniqueness and that every reference resolves, validates the document with the official XSD when it is committed, and reads every space, zone, surface, opening, construction, layer, material and window type. numpy recomputes the normal, area, azimuth and tilt of every surface from its polygon, shapely lays it into its own plane to check the rectangular geometry, the openings inside it and their positions, and the divergence theorem closes every space (outward area vectors sum to zero, the polygons enclose the stated volume, the floors add up to the stated area). The surfaces are regrouped by space, kind, boundary, neighbour and compass sector and compared with the table of the energy inference oracle. The subject reports the same table from the plan it writes.",
        "kind": "third-party-library",
        "engine": {"family": "lxml-shapely-numpy", "implementation": "lxml 6.1.3 (libxml2) XML parser with shapely 2.1.2 (GEOS) geometry and numpy vector algebra", "version": "6.1.3"},
        "productionReachable": False,
        "networkDuringExecution": False,
    },
    {
        "id": "bim-1-ifcopenshell-ifc-energy",
        "ecosystem": "python",
        "package": "ifcopenshell",
        "version": "0.8.4.post1",
        "capabilities": ["bim-1-export-ifc-energy"],
        "comparisonProfiles": ["floating-point-v1"],
        "license": "LGPL-3.0-or-later",
        "testOnly": True,
        "homepage": "https://ifcopenshell.org",
        "rationale": "IfcOpenShell is the reference open-source IFC implementation, written by unrelated authors, and ships the official property set templates of IFC 2x3 and IFC4 (ADD2) and the EXPRESS schemas with every WHERE rule. It opens the committed IFC 2x3 and IFC4 exports of the BIM house (files the subject wrote with its own Part-21 writer), reads the thermal property sets of every product with their IFC value types, checks every derived property against the standard template of its set (the property exists and has the template's primary measure type), restates the rules from the snapshot alone (kelvin set points, air changes from the clear height of the space, type values of windows and doors, ISO 6946 bounds of the layer stacks of walls, slabs and roofs, authored values kept and derived values listed) and runs the EXPRESS rules. The subject reports the same table from its document.",
        "kind": "third-party-library",
        "engine": {"family": "ifcopenshell", "implementation": "IfcOpenShell C++/Python IFC engine", "version": "0.8.4.post1"},
        "productionReachable": False,
        "networkDuringExecution": False,
    },
]


def main():
    text = MANIFEST.read_bytes().decode("utf-8")
    manifest = json.loads(text)
    known = {row["id"] for row in manifest["oracles"]}
    added = [row["id"] for row in ROWS if row["id"] not in known]
    manifest["oracles"].extend(row for row in ROWS if row["id"] not in known)
    if not added:
        print("both oracles are registered already")
        return 0
    temporary = MANIFEST.with_name(MANIFEST.name + ".tmp")
    temporary.write_bytes((json.dumps(manifest, indent=2, ensure_ascii=False) + "\n").encode("utf-8"))
    os.replace(temporary, MANIFEST)
    print("registered", ", ".join(added))
    return 0


if __name__ == "__main__":
    sys.exit(main())
