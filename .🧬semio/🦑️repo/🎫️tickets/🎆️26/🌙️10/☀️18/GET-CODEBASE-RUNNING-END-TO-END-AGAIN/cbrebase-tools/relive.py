"""🔁️ Re-records whole creation rows of a flat staged batch whose live file was edited after the batch landed: files/<file> and src copy become the live bytes, manifest new = live hash. Usage: relive.py <stage-dir> <root> <file>:<src>..."""
import hashlib, json, os, shutil, sys
stage, root = sys.argv[1], sys.argv[2]
mf = os.path.join(stage, "manifest.json"); rows = json.load(open(mf, encoding="utf-8"))
shutil.copyfile(mf, mf + ".pre-relive")
for spec in sys.argv[3:]:
    name, src = spec.split(":"); rel = next(k for k, v in rows.items() if v["file"] == name)
    live = os.path.join(root, rel); data = open(live, "rb").read()
    for p in (os.path.join(stage, "files", name), os.path.join(stage, src)):
        shutil.copyfile(p, p + ".pre-relive"); open(p, "wb").write(data)
    rows[rel]["new"] = hashlib.sha256(data).hexdigest()
open(mf, "w", encoding="utf-8").write(json.dumps(rows, ensure_ascii=False, indent=1))
print("relived", len(sys.argv) - 3)
