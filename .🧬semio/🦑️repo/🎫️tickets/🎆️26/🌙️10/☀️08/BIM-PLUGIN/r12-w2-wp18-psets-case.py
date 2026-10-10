"""🏷️ WP-18: writes the committed snapshot of the oracle case `🏷️effective-properties/🏗️psets`.

The case is the zoning house of `💡️inferences/🏘️zones/🏡️zoning` with property set templates, properties on a type and on instances, and two
classification systems added by hand. Run from the repo root: `python .🧬semio/.../r12-w2-wp18-psets-case.py`. Idempotent. The expectation is
never written here; the oracle `🐍️.py` of `🧪️tests/🏷️infer-bim-1-psets` writes it.
"""
import json
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
S = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/💡️inferences/"
SOURCE = S + "🏘️zones/🏡️zoning/📸️snapshot/🔣️.json"
TARGET = S + "🏷️effective-properties/🏗️psets/📸️snapshot/🔣️.json"


def value(kind, number):
    return {kind: {"value": number}}


def definition(name, kind, **over):
    return {"name": name, "kind": kind, "required": False, "allowed": [], **over}


snapshot = json.load(open(SOURCE, encoding="utf8"))
snapshot["property_templates"] = {
    "pt-wall": {
        "name": "Pset_WallCommon",
        "applies_to": ["Wall", "WallType"],
        "properties": [
            definition("FireRating", "Text", allowed=[value("Text", "REI 30"), value("Text", "REI 60"), value("Text", "REI 90")]),
            definition("ThermalTransmittance", "Real", unit="W/(m2.K)", minimum=0, maximum=5, default_value=value("Real", 0.35)),
            definition("IsExternal", "Boolean", default_value=value("Boolean", False)),
        ],
    },
    "pt-acoustic": {
        "name": "Pset_WallAcoustics",
        "applies_to": ["Wall"],
        "properties": [definition("AcousticRating", "Integer", unit="dB", required=True, minimum=20, maximum=80)],
    },
    "pt-column": {
        "name": "Pset_ColumnCommon",
        "applies_to": ["Column", "ColumnType"],
        "properties": [definition("Height", "Length", unit="m", minimum=0), definition("SectionArea", "Area", unit="m2", minimum=0)],
    },
    "pt-space": {
        "name": "Semio_Energy",
        "applies_to": ["Space"],
        "properties": [definition("TargetTemperature", "Real", unit="degC", minimum=5, maximum=30, default_value=value("Real", 20))],
    },
}
snapshot["properties"] = {
    "wt-300": {"Pset_WallCommon": {"FireRating": {"Text": {"value": "REI 60"}}, "ThermalTransmittance": value("Real", 0.28)}},
    "w-south": {
        "Pset_WallCommon": {"ThermalTransmittance": value("Real", 6.5), "IsExternal": value("Boolean", True)},
        "Pset_WallAcoustics": {"AcousticRating": value("Integer", 45)},
    },
    "w-east": {
        "Pset_WallCommon": {"FireRating": {"Text": {"value": "REI 20"}}},
        "Pset_WallAcoustics": {"AcousticRating": value("Real", 50)},
        "Project_Notes": {"Reviewer": {"Text": {"value": "UL"}}},
    },
    "w-north": {"Pset_WallAcoustics": {"AcousticRating": value("Integer", 10)}},
    "w-west": {"Pset_WallAcoustics": {"AcousticRating": value("Integer", 55)}},
    "ct-400": {"Pset_ColumnCommon": {"SectionArea": value("Area", 0.16)}},
    "c-east": {"Pset_ColumnCommon": {"Height": value("Length", 3.0)}},
    "sp-west": {"Semio_Energy": {"TargetTemperature": value("Real", 21.5)}},
    "sp-up": {"Semio_Energy": {"TargetTemperature": value("Real", 35)}},
}
snapshot["classification_systems"] = {
    "cs-din276": {
        "name": "DIN 276",
        "edition": "2018-12",
        "entries": [
            {"code": "300", "title": "Bauwerk - Baukonstruktionen"},
            {"code": "330", "title": "Aussenwaende", "parent": "300"},
            {"code": "340", "title": "Innenwaende", "parent": "300"},
        ],
    },
    "cs-uniclass": {
        "name": "Uniclass 2015 Ss",
        "edition": "v1.30",
        "entries": [{"code": "Ss_25", "title": "Wall and barrier systems"}, {"code": "Ss_25_10", "title": "Wall systems", "parent": "Ss_25"}],
    },
}
snapshot["classifications"] = {
    "w-south": {"cs-din276": "330", "cs-uniclass": "Ss_25_10"},
    "w-part": {"cs-din276": "340"},
    "w-east": {"cs-din276": "999"},
    "w-north": {"cs-gone": "X"},
}
os.makedirs(os.path.dirname(TARGET), exist_ok=True)
text = json.dumps(snapshot, indent=2, ensure_ascii=False) + "\n"
try:
    os.remove(TARGET)
except OSError:
    pass
with open(TARGET, "w", encoding="utf8", newline="") as handle:
    handle.write(text)
print("wrote", len(text), "bytes", len(TARGET), "path chars")
