"""📝️ Appends `set` ops that replace parts of decision texts of an object to its ops file (idempotent: an op of the same path is replaced): texts.py <id> <json list of [scale, key, old en, new en, old de, new de]>; a key that is a number names an entry of the decisions of the volume, a word a decision of the representation; the old part must be in the text of the document or already replaced."""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
slug, edits = sys.argv[1], json.loads(sys.argv[2])
a = json.load(open(os.environ.get("ASSET_FILE") or f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/{slug}/🔣️.json"))["asset"]
path = f"{HERE}/ops/{slug}.json"
ops = json.load(open(path)) if os.path.exists(path) else []
texts = {}
for scale, key, old_en, new_en, old_de, new_de in edits:
    p = ["volumes" if isinstance(key, int) else "representations", scale, "decisions", key]
    t = texts.setdefault(json.dumps(p), dict(a[p[0]][scale]["decisions"][key]))
    for lang, old, new in (("en", old_en, new_en), ("de", old_de, new_de)):
        if old in t[lang]:
            t[lang] = t[lang].replace(old, new)
        elif new not in t[lang]:
            raise SystemExit(f"{slug} {scale} {key} {lang}: '{old}' not found")
for p, t in texts.items():
    op = {"op": "set", "path": json.loads(p), "value": t}
    ops = [o for o in ops if o.get("path") != op["path"]] + [op]
open(path, "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
print(f"{slug}: {len(texts)} texts")
