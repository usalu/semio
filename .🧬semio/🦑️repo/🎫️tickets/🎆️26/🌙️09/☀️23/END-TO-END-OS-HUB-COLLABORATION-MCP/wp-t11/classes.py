"""🧮️ Counts contract breaches by id (and inventory rows by owner) for a T11 breaches-N.json capture."""
import json, sys, collections
rows = json.load(open(sys.argv[1]))
print(len(rows), "rows")
for key, count in collections.Counter(row["id"] for row in rows).most_common():
    print(f"{count:5d}  {key}")
