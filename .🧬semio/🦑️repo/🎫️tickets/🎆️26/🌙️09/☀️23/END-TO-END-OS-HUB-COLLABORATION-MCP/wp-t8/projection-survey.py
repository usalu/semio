"""Counts, over every owner manifest in the repository, how many rows agree with their leaf descriptor under the bridge's
current severity→outcome projection (info→applied) and under the repository convention (info→no-op)."""
import json, os, collections
root = "/Users/ueli/Documents/semio/"
OLD = {"applied": "applied", "warning": "applied", "info": "applied", "error": "rejected", "fatal": "rejected"}
NEW = {"applied": "applied", "warning": "applied", "info": "no-op", "error": "rejected", "fatal": "rejected"}
leaves = collections.defaultdict(list)
manifests = []
for base in ("✏️s", "🧰️framework"):
    for dirpath, dirs, files in os.walk(root + base):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "target", "dist", "🗑️generated", ".git")]
        if "🔣️.json" not in files: continue
        if os.path.basename(os.path.dirname(dirpath)) == "🧬️mutations":
            try: d = json.load(open(os.path.join(dirpath, "🔣️.json"), encoding="utf-8"))
            except (ValueError, UnicodeDecodeError): continue
            if isinstance(d, dict) and "semanticKind" in d and isinstance(d.get("outcomeClasses"), list): leaves[d["semanticKind"]].append((dirpath, d["outcomeClasses"]))
        elif dirpath.endswith("🔮️oracles"):
            try: d = json.load(open(os.path.join(dirpath, "🔣️.json"), encoding="utf-8"))
            except (ValueError, UnicodeDecodeError): continue
            for m in d.get("mutationManifests", []) if isinstance(d, dict) else []: manifests.append((os.path.dirname(dirpath), m))
counts = collections.Counter(); samples = collections.defaultdict(list)
for owner, m in manifests:
    region = owner if os.path.basename(os.path.dirname(owner)) != "🪆️subsets" else os.path.dirname(owner)
    for row in m.get("mutations", []):
        found = [c for p, c in leaves.get(row.get("id"), []) if p.startswith(region)]
        if not found: counts["no-leaf"] += 1; continue
        classes = found[0]; want = sorted(row.get("outcomes", []))
        old = sorted({OLD[c] for c in classes}); new = sorted({NEW[c] for c in classes})
        key = ("old" if old == want else "") + ("new" if new == want else "")
        counts[key or "neither"] += 1
        if len(samples[key or "neither"]) < 4: samples[key or "neither"].append((row["id"], want, classes))
print(dict(counts))
for k, v in samples.items(): print(k, v)
