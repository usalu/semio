"""Wave 2 features v3 (input script): idempotent edits of an artifact's `🔮️oracles/🔣️.json` — subject host packages and capabilities. Usage: python -I wave2-features-v3-manifest.py <manifest> host <package> <relative crate dir> <rationale> | cap <oracle id> <capability>"""
import json
import sys

path, kind = sys.argv[1], sys.argv[2]
document = json.load(open(path, encoding="utf8"))
if kind == "host":
    package, directory, rationale = sys.argv[3:6]
    if all(entry["package"] != package for entry in document["subjectHostPackages"]):
        document["subjectHostPackages"].append({"implementation": "rust", "package": package, "path": directory, "rationale": rationale})
else:
    oracle = next(entry for entry in document["oracles"] if entry["id"] == sys.argv[3])
    if sys.argv[4] not in oracle["capabilities"]:
        oracle["capabilities"].append(sys.argv[4])
open(path, "w", encoding="utf8").write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
