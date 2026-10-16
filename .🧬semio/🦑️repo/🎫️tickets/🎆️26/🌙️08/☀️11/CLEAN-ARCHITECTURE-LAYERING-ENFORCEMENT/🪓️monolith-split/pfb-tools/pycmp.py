"""⚖️ Compares two host result streams on scenario, status, raw hash, projection hash and first diagnostic message."""
import json, sys
def load(path):
    rows = {}
    for line in open(path, encoding="utf-8"):
        if not line.strip():
            continue
        row = json.loads(line)
        out = row.get("output", {})
        rows[row["scenario"]] = (row["status"], out.get("rawHash"), out.get("projectionHash"), [d["message"] for d in row.get("diagnostics", [])])
    return rows
a, b = load(sys.argv[1]), load(sys.argv[2])
diff = [k for k in sorted(set(a) | set(b)) if a.get(k) != b.get(k)]
print("scenarios", len(a), len(b), "different", len(diff))
for k in diff[:10]:
    print(" ", k, a.get(k), b.get(k))
sys.exit(1 if diff else 0)
