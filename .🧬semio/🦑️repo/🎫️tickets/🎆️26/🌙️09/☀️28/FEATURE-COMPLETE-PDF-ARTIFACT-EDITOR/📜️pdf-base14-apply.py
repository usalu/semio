"""🧷 Applies the regenerated Base 14 overlays to the shared worktree with hash guards."""
import base64, hashlib, json, os, sys

D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "pdf-base14")
sha = lambda b: hashlib.sha256(b).hexdigest()
rows = json.load(open(os.path.join(D, "symbolic-code-overlay.json"))) + json.load(open(os.path.join(D, "font-package-overlay.json")))
assets = json.load(open(os.path.join(D, "font-asset-overlay.json")))
for path, before, after in rows:
    assert len(path.encode()) <= 256, path
    if before is None:
        assert not os.path.exists(path), path
    else:
        current = open(path, "rb").read()
        if sha(current) == sha(after.encode()):
            continue
        assert sha(current) == before, "changed: " + path
for path, before, after in rows:
    if before is not None and sha(open(path, "rb").read()) == sha(after.encode()):
        continue
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(after)
    print("[DEBUG] wrote", path[-80:])
for row in assets:
    data = base64.b64decode(row["base64"])
    assert sha(data) == row["sha256"]
    os.makedirs(os.path.dirname(row["path"]), exist_ok=True)
    if not os.path.exists(row["path"]):
        open(row["path"], "wb").write(data)
        print("[DEBUG] asset", row["path"][-60:])
