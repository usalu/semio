"""🚧 Writes the round-3 volumes source of `objects/retractable-bollard` from the hand-stated solids of the kit.

Usage: python3 r3-retractable-bollard-model.py
@see ./r3-retractable-bollard-kit.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-retractable-bollard-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)


def main():
    """💾 Writes `volumes/objects/retractable-bollard.json`."""
    out = os.path.join(HERE, "volumes", "objects", "retractable-bollard.json")
    with open(out, "w", encoding="utf-8") as f:
        json.dump({"volumes": K.volumes()}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(out)


if __name__ == "__main__":
    main()
