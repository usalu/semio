"""✂️ Rework 05, parent carrying a toddler: takes out of the drawings the shapes that are below the printable feature of their scale in a view (their solids stay in the model; the drawing decisions name them as omitted).

Usage: python3 rework-05-parent-carrying-toddler-finish.py  (reads and writes `🗑️generated/r3-parent-carrying-toddler/🔣️.json`)
@see ./rework-05-draw2d.py
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-parent-carrying-toddler", "🔣️.json")
OMIT = {
    "100": {"elevation": ["coat-button-1", "coat-button-2"], "front": ["toddler-nose"], "plan": ["coat-button-1", "coat-button-2"]},
    "500": {
        "elevation": ["boot-toe-l", "boot-toe-r", "toddler-neck", "toddler-pompom"],
        "front": ["boot-toe-l", "boot-toe-r", "toddler-boot-l", "toddler-boot-r", "toddler-forearm-l", "toddler-forearm-r", "toddler-mitten-l", "toddler-mitten-r", "toddler-neck", "toddler-pompom"],
        "plan": ["boot-shaft-l", "boot-shaft-r", "toddler-mitten-l", "toddler-mitten-r", "toddler-neck", "toddler-pompom"],
    },
}


def main():
    """▶️ Removes the omitted shapes of every listed scale and view."""
    with open(CANDIDATE, encoding="utf8") as f:
        doc = json.load(f)
    for scale, views in OMIT.items():
        geometry = doc["asset"]["representations"][scale]["geometry"]
        for view, ids in views.items():
            geometry[view] = [x for x in geometry[view] if x["id"] not in ids]
    with open(CANDIDATE, "w", encoding="utf8") as f:
        f.write(json.dumps(doc, ensure_ascii=False) + "\n")


if __name__ == "__main__":
    main()
