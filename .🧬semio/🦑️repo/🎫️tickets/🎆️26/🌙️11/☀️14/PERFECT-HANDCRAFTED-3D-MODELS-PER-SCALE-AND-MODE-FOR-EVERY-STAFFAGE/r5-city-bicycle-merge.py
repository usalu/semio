"""🚲 Round 5 vehicles/city-bicycle: carries the round-5 hand edits of the 1:50 scale (brake cables in `body`, the decisions) from a fresh `r3-city-bicycle-build.py 50` into the previous volumes source, keeping every list that later passes refined there (the split frame masses of the structure lists).

Usage: python3 r5-city-bicycle-merge.py <previous source json> <fresh build json>   (writes volumes/vehicles/city-bicycle.json)"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(T, "volumes", "vehicles", "city-bicycle.json")
CABLES = ("cable-front", "cable-rear-loop", "cable-rear")

if __name__ == "__main__":
    old, new = (json.load(open(p, encoding="utf8")) for p in sys.argv[1:3])
    fifty = old["volumes"]["50"]
    fifty["body"] = [s for s in fifty["body"] if s["id"] not in CABLES] + [s for s in new["volumes"]["50"]["body"] if s["id"] in CABLES]
    fifty["decisions"] = new["volumes"]["50"]["decisions"]
    old["volumes"]["500"]["decisions"] = new["volumes"]["500"]["decisions"]
    with open(SRC, "w", encoding="utf8") as fh:
        json.dump(old, fh, ensure_ascii=False)
        fh.write("\n")
    print("1:50 body", len(fifty["body"]))
