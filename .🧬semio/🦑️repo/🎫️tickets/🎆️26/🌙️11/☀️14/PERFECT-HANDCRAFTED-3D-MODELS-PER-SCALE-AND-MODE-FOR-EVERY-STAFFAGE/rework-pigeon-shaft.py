"""🧹 Pigeon second round: removes the folded-primaries shaft that still hung on the hand bone; every primary has its own rachis member on its feather bone."""
import json, os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/animals/pigeon.json")
text = open(SOURCE, encoding="utf-8").read()
source = json.loads(text)
for scale, volume in source["volumes"].items():
    before = len(volume["structure"])
    volume["structure"] = [s for s in volume["structure"] if s["id"] not in ("near-primary-shaft", "far-primary-shaft")]
    legs = [(s["id"], s["bone"], max(max(x["rx"], x["ry"]) for x in s["sections"])) for s in volume["body"] if s["id"] in ("near-leg", "near-foot", "near-tarsus", "near-toes")]
    print(scale, "structure", before, "->", len(volume["structure"]), "legs", legs)
open(SOURCE, "w", encoding="utf-8").write(json.dumps(source, ensure_ascii=False) + ("\n" if text.endswith("\n") else ""))
