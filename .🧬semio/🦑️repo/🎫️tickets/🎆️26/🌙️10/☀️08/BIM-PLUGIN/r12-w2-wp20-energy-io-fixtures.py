#!/usr/bin/env python3
"""Writes the input snapshots of the two cases of `w2-wp20-energy-io` from the generated house (`🖼️assets/🏡️house/📸️snapshot.json`: conditions of every space, thermal data of every window and door type, authored
`ThermalTransmittance` on a wall and a window): `🧫️fixtures/🌿️gbxml/🏠️house/📸️snapshot/🔣️.json` and `🧫️fixtures/🏗️ifc/🔥️energy/📸️snapshot/🔣️.json`. The files are copies, written through `*.tmp` and `os.replace`.
The expected files (`gbxml.xml`, `energy-2x3.ifc`, `energy-4.ifc`) are written by the export tests with `BIM_BLESS=1`, the measured tables by the oracles' `write`.
"""
import os
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
S = T.parents[6] / "✏️s" / "🔌️plugins" / "🏙️bim" / "🗿️artifacts" / "🏢️model" / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any"
SOURCE = S / "🖼️assets" / "🏡️house" / "📸️snapshot.json"
TARGETS = [S / "🧫️fixtures" / "🌿️gbxml" / "🏠️house" / "📸️snapshot" / "🔣️.json", S / "🧫️fixtures" / "🏗️ifc" / "🔥️energy" / "📸️snapshot" / "🔣️.json"]


def main():
    data = SOURCE.read_bytes()
    for target in TARGETS:
        target.parent.mkdir(parents=True, exist_ok=True)
        temporary = target.with_name(target.name + ".tmp")
        temporary.write_bytes(data)
        os.replace(temporary, target)
        print("wrote", target.relative_to(S), len(data), "bytes")
    return 0


if __name__ == "__main__":
    sys.exit(main())
