"""🏗️ Splits the coarse shoe of `components/glass-balustrade` in the structure lists of 1:200 and 1:500 by hand into its load path: the clamp upstand is the member of the shoe bone, the 300 x 15 mm base plate the fixed support."""
import json
import pathlib

SOURCE = pathlib.Path(__file__).parent / "volumes" / "components" / "glass-balustrade.json"
UPSTAND = {"id": "member-shoe", "kind": "extrude", "bone": "shoe", "material": "steel", "role": "body", "tone": 0.38, "plane": "xy", "profile": [[0.015, -0.03575], [0.125, -0.03575], [0.125, 0.03575], [0.015, 0.03575]], "from": -1.0, "to": 1.0, "member": "shoe", "part": "member"}
PLATE = {"id": "support-shoe", "kind": "extrude", "bone": "shoe", "material": "steel", "role": "body", "tone": 0.32, "plane": "yz", "profile": [[-0.15, -1.0], [0.15, -1.0], [0.15, 1.0], [-0.15, 1.0]], "from": 0, "to": 0.015, "sheet": True, "member": "shoe", "part": "support", "anchor": "base-clamp"}


def main():
    volumes = json.loads(SOURCE.read_text())
    for scale in ("200", "500"):
        structure = volumes["volumes"][scale]["structure"]
        if not any(solid.get("of") == "shoe" for solid in structure):
            raise SystemExit("already split")
        kept = [solid for solid in structure if solid.get("of") != "shoe"]
        volumes["volumes"][scale]["structure"] = kept[:2] + [dict(UPSTAND), dict(PLATE)] + kept[2:]
    SOURCE.write_text(json.dumps(volumes, ensure_ascii=False, indent=2) + "\n")


main()
