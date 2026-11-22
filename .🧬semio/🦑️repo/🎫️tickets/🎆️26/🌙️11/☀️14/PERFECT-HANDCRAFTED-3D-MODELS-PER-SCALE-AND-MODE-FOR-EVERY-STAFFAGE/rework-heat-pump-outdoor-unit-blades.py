"""🌀 machinery/heat-pump-outdoor-unit: the three sickle blades of the axial fan are pressed aluminium sheet and are declared as such at 1:100, 1:200 and 1:500.

A blade is 100 to 168 mm in chord, 0.26 m long and a few millimetres thick in reality; it keeps its sheet section at every scale instead of being thickened to 15, 30 or 75 mm, which would close the fan opening. One-shot on `volumes/machinery/heat-pump-outdoor-unit.json`; publish with `volumes-insert.py`.
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "machinery", "heat-pump-outdoor-unit.json")
SENTENCE = {
    "en": "The three fan blades are pressed aluminium sheet and keep their sheet section at this scale; thickened to the smallest feature they would close the fan opening.",
    "de": "Die drei Ventilatorflügel sind gepresstes Aluminiumblech und behalten in diesem Maßstab ihren Blechquerschnitt; auf das kleinste Merkmal verdickt würden sie die Ventilatoröffnung schließen.",
}


def main():
    """▶️ Declares the blade loft a sheet at the three coarse scales and appends one decision each."""
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    for scale in ("100", "200", "500"):
        volumes = document["volumes"][scale]
        blades = next(solid for solid in volumes["body"] if solid["id"] == "fan-blades")
        blades["sheet"] = True
        if SENTENCE not in volumes["decisions"]:
            volumes["decisions"].append(SENTENCE)
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=1)
    print("applied")


if __name__ == "__main__":
    main()
