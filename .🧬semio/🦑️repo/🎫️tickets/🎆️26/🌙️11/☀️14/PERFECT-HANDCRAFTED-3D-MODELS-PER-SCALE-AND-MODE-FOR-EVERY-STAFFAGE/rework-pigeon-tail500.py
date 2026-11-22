"""🪶 Pigeon second round, 1:500 structure: the 26 mm pygostyle lump is left out and the central rectrix shaft runs from the tail joint, so the coarsest skeleton is simpler than 1:200."""
import json, os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/animals/pigeon.json")
text = open(SOURCE, encoding="utf-8").read()
source = json.loads(text)
structure = source["volumes"]["500"]["structure"]
source["volumes"]["500"]["structure"] = [s for s in structure if s["id"] != "pygostyle"]
shaft = next(s for s in structure if s["id"] == "rectrix-shaft")
shaft["sections"][0]["at"] = [-0.006, 0, 0]
open(SOURCE, "w", encoding="utf-8").write(json.dumps(source, ensure_ascii=False) + ("\n" if text.endswith("\n") else ""))
print("1:500 structure", len(structure), "->", len(source["volumes"]["500"]["structure"]))
