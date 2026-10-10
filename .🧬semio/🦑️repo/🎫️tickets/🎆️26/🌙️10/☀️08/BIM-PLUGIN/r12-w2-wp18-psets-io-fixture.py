"""Writes the IFC psets fixture snapshot from the committed IFC house: classification tables with parents, several systems per element and per type,
type-level property sets and the property set templates of the IFC4 library.

Usage: python r12-w2-wp18-psets-io-fixture.py <house snapshot json> <psets snapshot json>
"""

import json
import sys
from pathlib import Path

NEWLINE = chr(10)


def text(value):
    return {"Text": {"value": value}}


def flag(value):
    return {"Boolean": {"value": value}}


def real(value):
    return {"Real": {"value": value}}


def length(value):
    return {"Length": {"value": value}}


def integer(value):
    return {"Integer": {"value": value}}


def definition(name, kind, required=False, **extra):
    row = {"name": name, "kind": kind, "required": required, "allowed": []}
    row.update(extra)
    return row


def main(source, target):
    model = json.loads(Path(source).read_text(encoding="utf-8"))
    model["project"]["name"] = "IFC Psets"
    model["project"]["description"] = "Classification and property set fixture"
    model["classification_systems"] = {
        "cs-uniclass-2015": {
            "name": "Uniclass 2015",
            "edition": "",
            "entries": [
                {"code": "EF", "title": "Elements and functions"},
                {"code": "EF_25", "title": "Wall and barrier elements", "parent": "EF"},
                {"code": "EF_25_10", "title": "Walls", "parent": "EF_25"},
                {"code": "EF_30", "title": "Roof, floor and paving elements", "parent": "EF"},
            ],
        },
        "cs-din-276": {
            "name": "DIN 276",
            "edition": "2018-12",
            "source": "https://www.din.de",
            "entries": [
                {"code": "300", "title": "Building construction"},
                {"code": "330", "title": "Exterior walls", "parent": "300"},
                {"code": "331", "title": "Load-bearing exterior walls", "parent": "330"},
                {"code": "340", "title": "Interior walls", "parent": "300"},
            ],
        },
        "cs-omniclass": {"name": "Omniclass", "edition": "", "entries": [{"code": "21-02 10 10", "title": "Columns"}]},
    }
    model["classifications"] = {
        "w-south": {"cs-uniclass-2015": "EF_25_10", "cs-din-276": "331"},
        "w-east": {"cs-uniclass-2015": "EF_25_10"},
        "c-1": {"cs-omniclass": "21-02 10 10"},
        "wt-300": {"cs-din-276": "330", "cs-uniclass-2015": "EF_25"},
        "dr-180": {"cs-din-276": "300"},
        "st-ground": {"cs-din-276": "300"},
    }
    model["properties"]["wt-300"] = {
        "Pset_WallCommon": {"FireRating": text("EI90"), "IsExternal": flag(True), "ThermalTransmittance": real(0.2)},
        "Custom": {"Reference": text("WT-300")},
    }
    model["properties"]["dr-180"] = {"Pset_DoorCommon": {"FireExit": flag(False), "Width": length(1.8), "Leaves": integer(2)}}
    model["properties"]["wnd-120"] = {"Pset_WindowCommon": {"Reference": text("W-120"), "ThermalTransmittance": real(1.1)}}
    model["properties"]["ct-rect"] = {"Pset_ColumnCommon": {"LoadBearing": flag(True)}}
    model["property_templates"] = {
        "pt-wall": {
            "name": "Pset_WallCommon",
            "applies_to": ["Wall", "WallType"],
            "properties": [
                definition("FireRating", "Text", allowed=[text("EI30"), text("EI60"), text("EI90")]),
                definition("ThermalTransmittance", "Real", True, unit="W/(m2.K)", minimum=0, maximum=5, default_value=real(0.35)),
                definition("IsExternal", "Boolean", default_value=flag(False)),
            ],
        },
        "pt-door": {
            "name": "Pset_DoorCommon",
            "applies_to": ["Door", "DoorType"],
            "properties": [
                definition("FireExit", "Boolean"),
                definition("Width", "Length", True, description="Clear width; keep = and ; apart", minimum=0.6, maximum=3),
                definition("Leaves", "Integer", allowed=[integer(1), integer(2)], default_value=integer(1)),
            ],
        },
        "pt-window-type": {"name": "Pset_WindowTypeCommon", "applies_to": ["WindowType"], "properties": [definition("Reference", "Text", description="Catalogue reference")]},
        "pt-space": {
            "name": "Pset_SpaceCommon",
            "applies_to": ["Space"],
            "properties": [definition("NetPlannedArea", "Area", minimum=0), definition("Category", "Text", default_value=text("Office"))],
        },
    }
    Path(target).parent.mkdir(parents=True, exist_ok=True)
    path = Path(target)
    if path.exists():
        path.unlink()
    path.write_text(json.dumps(model, indent=2, ensure_ascii=False) + NEWLINE, encoding="utf-8", newline=NEWLINE)
    print("wrote %d bytes" % path.stat().st_size)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
