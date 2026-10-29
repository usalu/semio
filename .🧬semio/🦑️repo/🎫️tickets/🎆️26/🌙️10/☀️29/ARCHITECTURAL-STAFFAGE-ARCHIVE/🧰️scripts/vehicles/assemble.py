import json, sys
V = sys.argv[1]; out = sys.argv[2]
ids = ["city-car", "city-bus", "excavator", "tower-crane", "forklift", "cargo-bike"]
doc = {"$schema": "../../🧬️schema/🔣️.json#/$defs/CategoryDocument", "version": "semio.staffage/v1", "category": "vehicles", "assets": [json.load(open(f"{V}/src/{i}.json")) for i in ids]}
open(out, "w").write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
