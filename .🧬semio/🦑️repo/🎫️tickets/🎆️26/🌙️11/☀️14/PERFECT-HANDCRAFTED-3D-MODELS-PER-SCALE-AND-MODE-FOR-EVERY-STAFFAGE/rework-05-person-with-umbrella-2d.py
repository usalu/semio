"""✏️ person-with-umbrella: side elevation of head and hair at 1:50 redrawn after the head was remodelled.

The outlines are the ruled projection of the new head and hair solids (`r3-project2d.ts humans/person-with-umbrella --scale 50 --view elevation --bone head --candidate`), taken over shape by shape (same id, bone, role, depth, tone, region); every other shape of the catalog document stays as it is.

Usage: python3 rework-05-person-with-umbrella-2d.py    re-reads the catalog document and writes it
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "person-with-umbrella", "🔣️.json")
RULED = os.path.join(T, "🗑️generated", "r3-project2d", "humans", "person-with-umbrella.s50.elevation", "🔣️.json")
REDRAWN = ("head", "hair")


def main():
    ruled = json.load(open(RULED, encoding="utf-8"))["asset"]["representations"]["50"]["geometry"]["elevation"]
    document = json.load(open(CATALOG, encoding="utf-8"))
    shapes = document["asset"]["representations"]["50"]["geometry"]["elevation"]
    for index, shape in enumerate(shapes):
        if shape.get("bone") == "head" and shape["id"] in REDRAWN:
            fresh = next(item for item in ruled if item["id"] == shape["id"] and item.get("bone") == "head")
            shapes[index] = fresh
            print(f"[DEBUG] elevation 1:50 {shape['id']}: {len(shape.get('points', []))} -> {len(fresh.get('points', []))} points")
    with open(CATALOG, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


if __name__ == "__main__":
    main()
