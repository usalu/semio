"""🦴 Root addition of the dairy cow: the material `bone` for the skeleton of the structure lists (density of compact bovine bone about 1900 kg/m3, emissivity and absorptivity of dry bone as authored estimates).

Usage: python3 r3-dairy-cow-material.py
"""
import r3_dairy_cow_io as io

BONE = {"id": "bone", "title": {"en": "Bone", "de": "Knochen"}, "emissivity": 0.94, "solarAbsorptivity": 0.45, "densityKgM3": 1900, "source": "authored-estimate", "reference": "dyce-2010"}


def change(asset):
    materials = [m for m in asset["materials"] if m["id"] != "bone"] + [BONE]
    return {"materials": materials}


print("written", io.write(change))
