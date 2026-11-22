"""🔗 Closes the load path of the coarse labrador skeletons: ear roots on the skull and the first caudal bar into the sacrum at 1:200, the mandible as one pen-wide bar along the muzzle at 1:500.

Usage: python3 rework-p1-labrador-dog-joins.py
"""
import json

SOURCE = "volumes/animals/labrador-dog.json"


def find(entries, identifier):
    return next(entry for entry in entries if entry["id"] == identifier)


def main():
    document = json.load(open(SOURCE))
    two = document["volumes"]["200"]["structure"]
    skull = find(two, "skull")
    for side, sign in (("n", -1), ("f", 1)):
        two.append({"id": "ear-root-" + side, "kind": "capsule", "bone": "head", "material": skull["material"], "role": "body", "tone": skull["tone"], "p0": [0.04, 0.03, sign * 0.03], "p1": [0.04, 0.04, sign * 0.06], "r0": 0.0152, "r1": 0.0152, "member": "head", "part": "member"})
    find(two, "caudal-1")["p0"] = [-0.04, 0.007, 0]
    five = document["volumes"]["500"]["structure"]
    skull = find(five, "skull")
    five.insert(five.index(skull) + 1, {"id": "mandible", "kind": "capsule", "bone": "jaw", "material": skull["material"], "role": "body", "tone": skull["tone"], "p0": [0.0, 0.003, 0], "p1": [0.17, 0.017, 0], "r0": 0.0376, "r1": 0.0376, "member": "jaw", "part": "member"})
    json.dump(document, open(SOURCE, "w"), ensure_ascii=False)


main()
