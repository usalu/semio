"""📏 Merges the `volumes-feature` findings of a measurement into the feature floors of one species (a design input of its build):
`python3 floors.py <species module> <measure.json>`; prints how many floors changed.

@see ./morph.py (`apply_floors`, `floors_from`)
"""
import json
import sys
from pathlib import Path

from morph import floors_from

path = Path(__file__).resolve().parent / f"{sys.argv[1]}-floors.json"
known = json.loads(path.read_text()) if path.exists() else []
issues = json.load(open(sys.argv[2]))["issues"]
merged = floors_from(issues, known)
path.write_text(json.dumps(merged, indent=1) + "\n")
print(len(merged) - len(known) + sum(1 for a, b in zip(sorted(known, key=str), sorted(merged, key=str)) if a != b))
