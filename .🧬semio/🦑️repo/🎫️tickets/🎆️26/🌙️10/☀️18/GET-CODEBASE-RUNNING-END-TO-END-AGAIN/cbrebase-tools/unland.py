"""↩️ Derives the base of a landed flat-batch row as the live file minus the lines the batch added (new vs the pre-rebase base): unland.py <stage-dir> <file> <root>. Re-records manifest base."""
import hashlib, json, os, sys
stage, name, root = sys.argv[1:4]
mf = os.path.join(stage, "manifest.json"); rows = json.load(open(mf, encoding="utf-8"))
rel = next(k for k, v in rows.items() if v["file"] == name); row = rows[rel]
old_base = open(os.path.join(stage, "base", name + ".pre-rebase"), "rb").read().split(b"\n")
old_new = open(os.path.join(stage, "files", name + ".pre-rebase"), "rb").read().split(b"\n")
added = [l for l in old_new if l not in set(old_base)]
live = open(os.path.join(root, rel), "rb").read().split(b"\n")
out = []; pending = list(added)
for l in live:
    if pending and l == pending[0]: pending.pop(0); continue
    out.append(l)
assert not pending, pending
data = b"\n".join(out)
open(os.path.join(stage, "base", name), "wb").write(data)
row["base"] = hashlib.sha256(data).hexdigest()
open(mf, "w", encoding="utf-8").write(json.dumps(rows, ensure_ascii=False, indent=1))
print("unlanded", name, len(added), "lines")
