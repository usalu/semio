"""🧾 Writes `manifest.json` of a staged batch from its `files/` and `base/` and a rel map: `python3 -I manifest.py <batch-dir> <file>=<repo-relative path> ...` (a file absent from base/ is new: base null)."""
import hashlib, json, os, sys
batch = sys.argv[1]
rows = {}
for pair in sys.argv[2:]:
    name, rel = pair.split("=", 1)
    sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest() if os.path.exists(p) else None
    rows[rel] = {"file": name, "base": sha(os.path.join(batch, "base", name)), "new": sha(os.path.join(batch, "files", name))}
json.dump(rows, open(os.path.join(batch, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(json.dumps(rows, ensure_ascii=False, indent=1))
