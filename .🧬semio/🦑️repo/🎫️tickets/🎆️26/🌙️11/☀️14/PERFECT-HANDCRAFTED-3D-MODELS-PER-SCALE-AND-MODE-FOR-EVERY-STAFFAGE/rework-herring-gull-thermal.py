"""🌡️ Herring gull, second round pass 1, thermal: secondaries and humeral feathers take the contour layer of the primaries so the whole wing reads as one layered region.

Usage: python3 rework-herring-gull-thermal.py   edits volumes/animals/herring-gull.json in place (publish with volumes-insert.py afterwards)
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes/animals/herring-gull.json")


def main():
    source = json.load(open(SOURCE, encoding="utf8"))
    for scale in ("50", "100", "200", "500"):
        thermal = source["volumes"][scale]["thermal"]
        layer = next(entry for entry in thermal if entry["id"] == "th-near-primary-1").get("layer")
        changed = 0
        for entry in thermal:
            parts = entry["id"].split("-")
            if layer and len(parts) == 4 and parts[2] in ("secondary", "tertial") and "layer" not in entry:
                entry["layer"] = layer
                changed += 1
        print("[DEBUG]", scale, layer, changed)
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")


if __name__ == "__main__":
    main()
