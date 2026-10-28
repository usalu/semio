"""🧱 Rebuilds a staged batch of lane linuxdc from a mirror: python3 -I mkbatch.py <batch-name> <mirror-root> <live-root>. The batch rows are the files listed in the existing manifest.json; `new` is taken from the mirror, `base` from the CURRENT live file (re-run right before the gate lands the batch)."""
import hashlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
name, mirror, live = sys.argv[1], os.path.abspath(sys.argv[2]), os.path.abspath(sys.argv[3])
batch = os.path.join(HERE, "staged", name)
rels = list(json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8")))
sha = lambda data: hashlib.sha256(data).hexdigest()
rows = {}
for index, rel in enumerate(rels):
    key = "%02d%s" % (index, os.path.splitext(rel)[1])
    base = open(os.path.join(live, rel), "rb").read() if os.path.exists(os.path.join(live, rel)) else None
    new = open(os.path.join(mirror, rel), "rb").read()
    open(os.path.join(batch, "files", key), "wb").write(new)
    if base is not None: open(os.path.join(batch, "base", key), "wb").write(base)
    rows[rel] = {"file": key, "base": None if base is None else sha(base), "new": sha(new)}
json.dump(rows, open(os.path.join(batch, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(name, len(rows), "rows")
