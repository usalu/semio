"""🧮️ Census of mutation leaf descriptors by plugin and declared outcomeClasses vocabulary."""
import json, os, collections, sys
root = "/Users/ueli/Documents/semio/"
by = collections.Counter(); total = collections.Counter(); vocab = collections.Counter()
for base in ("✏️s", "🧰️framework"):
    for dirpath, dirs, files in os.walk(root + base):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "target", "dist", "🗑️generated", ".git")]
        if "🔣️.json" not in files or os.path.basename(os.path.dirname(dirpath)) != "🧬️mutations": continue
        try: d = json.load(open(os.path.join(dirpath, "🔣️.json"), encoding="utf-8"))
        except (ValueError, UnicodeDecodeError): continue
        if not (isinstance(d, dict) and isinstance(d.get("outcomeClasses"), list)): continue
        rel = dirpath[len(root):].split("/")
        plugin = "/".join(rel[:3]) if rel[0] == "✏️s" else "/".join(rel[:4])
        total[plugin] += 1
        for c in d["outcomeClasses"]: vocab[c] += 1
        if set(d["outcomeClasses"]) != {"applied"}: by[plugin] += 1
print("vocab", dict(vocab)); print("leaves", sum(total.values()), "non-applied-only", sum(by.values()))
for p in sorted(total): print(f"{total[p]:5} {by[p]:5} {p}")
