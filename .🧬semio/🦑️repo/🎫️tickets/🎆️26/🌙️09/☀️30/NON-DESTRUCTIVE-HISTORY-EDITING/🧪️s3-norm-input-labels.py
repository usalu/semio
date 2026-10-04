#!/usr/bin/env python3
"""🏷️ S3-NORM: schema-first UI labels for the DIN V 18599 and VDI 3805 mutation inputs the `schema mutation-inputs` gate flags.

The labels live once, on the snapshot `$defs` the leaves `$ref` (the reader merges `x-semio-ui` along the `$ref` chain):
DIN V 18599 `EnvelopeElement.kind` / `.adjacency`, `ThermalZone.usageProfile`; VDI 3805 `VdiQuantityKind` option labels.

Usage: python3 🧪️s3-norm-input-labels.py [--check]
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
NORM = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json"

DIN18599_PROPERTIES = {
    ("EnvelopeElement", "kind"): {"label": {"en": "Element type", "de": "Bauteilart"}},
    ("EnvelopeElement", "adjacency"): {
        "label": {"en": "Adjacent to", "de": "Angrenzung"},
        "description": {
            "en": "What the element borders; sets its temperature correction factor F_x (DIN V 18599-2).",
            "de": "Woran das Bauteil grenzt; bestimmt seinen Temperatur-Korrekturfaktor F_x (DIN V 18599-2).",
        },
    },
    ("ThermalZone", "usageProfile"): {
        "label": {"en": "Usage profile", "de": "Nutzungsprofil"},
        "description": {"en": "Standard usage profile of DIN V 18599-10.", "de": "Standard-Nutzungsprofil nach DIN V 18599-10."},
    },
}

VDI_QUANTITY_KINDS = {
    "dimensionless": ("Dimensionless", "Dimensionslos"),
    "length": ("Length", "Länge"),
    "area": ("Area", "Fläche"),
    "volume": ("Volume", "Volumen"),
    "mass": ("Mass", "Masse"),
    "time": ("Time", "Zeit"),
    "temperature": ("Temperature", "Temperatur"),
    "force": ("Force", "Kraft"),
    "pressure": ("Pressure", "Druck"),
    "stress": ("Stress", "Spannung"),
    "moment": ("Moment", "Moment"),
    "energy": ("Energy", "Energie"),
    "power": ("Power", "Leistung"),
    "thermalConductivity": ("Thermal conductivity", "Wärmeleitfähigkeit"),
    "thermalResistance": ("Thermal resistance", "Wärmedurchlasswiderstand"),
    "heatTransferCoefficient": ("Heat transfer coefficient", "Wärmeübergangskoeffizient"),
    "airPermeability": ("Air permeability", "Luftdurchlässigkeit"),
    "ventilationRate": ("Ventilation rate", "Luftwechselrate"),
    "acceleration": ("Acceleration", "Beschleunigung"),
}


def rewrite(path: Path, edit, check: bool) -> int:
    text = path.read_text()
    document = json.loads(text)
    edit(document)
    out = json.dumps(document, indent=2, ensure_ascii=False) + "\n"
    if out == text:
        return 0
    if not check:
        path.write_text(out)
    return 1


def din18599(document: dict) -> None:
    for (record, field), ui in DIN18599_PROPERTIES.items():
        node = document["$defs"][record]["properties"][field]
        node["x-semio-ui"] = {**node.get("x-semio-ui", {}), **ui}


def vdi3805(document: dict) -> None:
    node = document["$defs"]["VdiQuantityKind"]
    assert node["enum"] == list(VDI_QUANTITY_KINDS), "VdiQuantityKind enum drifted"
    node["x-semio-ui"] = {"options": {value: {"en": en, "de": de} for value, (en, de) in VDI_QUANTITY_KINDS.items()}}


def main() -> int:
    check = "--check" in sys.argv
    pending = rewrite(NORM / "⚡️din18599" / SUBSET, din18599, check) + rewrite(NORM / "🏭️vdi3805" / SUBSET, vdi3805, check)
    print(f"{'pending' if check else 'rewritten'}={pending}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
