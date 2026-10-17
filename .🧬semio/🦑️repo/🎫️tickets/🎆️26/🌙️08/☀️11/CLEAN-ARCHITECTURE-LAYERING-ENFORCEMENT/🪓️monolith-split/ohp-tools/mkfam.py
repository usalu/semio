"""📇 Writes ohp-tools/families/<name>.json from repo-relative directory prefixes: every indexed crate whose manifest lies under a prefix (excludes optional --skip substrings).
usage: mkfam.py <name> <prefix>... [--skip SUBSTR]..."""
import json, os, sys
F = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.dirname(F) + "/🗑️generated"
a = sys.argv[1:]
name = a[0]
skips = [a[i + 1] for i, x in enumerate(a) if x == "--skip"]
prefixes = [x for i, x in enumerate(a[1:], 1) if x != "--skip" and a[i - 1] != "--skip"]
idx = json.load(open(GEN + "/ohp-manifest-index.json", encoding="utf-8"))
crates = sorted(n for n, rel in idx.items() if any(rel.startswith(p.rstrip("/") + "/") for p in prefixes) and not any(s in rel for s in skips))
json.dump({"name": name, "crates": crates, "note": "ORACLE-HARVEST-PLUGINS: " + ", ".join(prefixes)}, open(F + f"/families/{name}.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(name, len(crates), "crates")
