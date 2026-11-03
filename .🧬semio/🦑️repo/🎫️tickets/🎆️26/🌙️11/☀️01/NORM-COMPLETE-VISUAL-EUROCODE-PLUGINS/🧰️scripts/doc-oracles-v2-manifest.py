"""🧾 Appends one sentence to the rationale of `enXXXX-1-compliance-independent` in an artifact's oracle manifest: python -I doc-oracles-v2-manifest.py enXXXX "sentence" [host-package ...]"""
import json
import pathlib
import sys

code, sentence, packages = sys.argv[1], sys.argv[2], sys.argv[3:]
root = next(pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*" + code))
path = root / "🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json"
data = json.loads(path.read_text(encoding="utf8"))
entry = next(o for o in data["oracles"] if o["id"] == f"{code}-1-compliance-independent")
if sentence not in entry["rationale"]:
    entry["rationale"] = entry["rationale"].rstrip() + " " + sentence
for package in packages:
    hosts = entry.setdefault("subjectHostPackages", [])
    if package not in hosts:
        hosts.append(package)
path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
print(path)
