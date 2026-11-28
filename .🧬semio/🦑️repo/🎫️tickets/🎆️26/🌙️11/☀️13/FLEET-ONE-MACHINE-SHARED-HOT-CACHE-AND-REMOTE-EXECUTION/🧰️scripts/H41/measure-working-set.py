"""Working set of the compiler-build image `r` per worker, from the fleet store scans: content bytes (scan bytes less the backing file), the declared image size and the largest members. Usage: measure-working-set.py [store-root]. Scans are partial (bounded at ~38 s), so every figure is a lower bound."""
import json
import os
import sys

root = sys.argv[1] if len(sys.argv) > 1 else "/Users/Shared/semio-store/v1"
GB = 1e9
print("| origin | content GB | declared image GB | free in image GB | partial | largest member |")
print("|---|---|---|---|---|---|")
rows = []
for origin in sorted(os.listdir(os.path.join(root, "origins"))):
    try:
        scan = json.load(open(os.path.join(root, "origins", origin, "partitions.json"), encoding="utf8"))
    except Exception:
        continue
    for partition in scan["partitions"]:
        if partition["name"] != "r" or not partition.get("capacityBytes"):
            continue
        backing = next((member["bytes"] for member in partition["top"] if member["path"] == "fixed.dmg"), 0)
        content = max(0, partition["bytes"] - backing)
        top = next((member for member in partition["top"] if member["path"] != "fixed.dmg"), None)
        rows.append((origin, content, partition["capacityBytes"], partition["partial"], top))
for origin, content, capacity, partial, top in rows:
    print(f"| {origin} | {content / GB:.1f} | {capacity / GB:.1f} | {(capacity - content) / GB:.1f} | {partial} | {top['path'][:60] + ' ' + format(top['bytes'] / GB, '.1f') + ' GB' if top else '-'} |")
if rows:
    values = sorted(row[1] for row in rows)
    print(f"\nworkers {len(values)}, median {values[len(values) // 2] / GB:.1f} GB, maximum {values[-1] / GB:.1f} GB")
