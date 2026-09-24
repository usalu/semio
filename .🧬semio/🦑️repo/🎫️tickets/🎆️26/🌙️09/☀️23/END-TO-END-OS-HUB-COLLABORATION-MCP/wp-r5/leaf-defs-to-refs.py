import json, os, sys, glob
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
apply = len(sys.argv) > 1 and sys.argv[1] == "apply"
total = 0
for subset in sorted(os.listdir(ROOT)):
    snap_path = os.path.join(ROOT, subset, "🧬️schema/📸️snapshot/🔣️.json")
    if subset == "✉️base" or not os.path.exists(snap_path): continue
    snap = json.load(open(snap_path, encoding="utf-8"))
    sid, stitle, sdefs = snap["$id"], snap.get("title"), set(snap.get("$defs", {}).keys())
    for leaf in glob.glob(os.path.join(ROOT, subset, "🧬️schema/🧬️mutations/*/🧬️schema/🔣️.json")):
        t = open(leaf, encoding="utf-8").read(); d = json.loads(t)
        defs = d.get("$defs", {}); changed = []
        for k in list(defs):
            if k == stitle: defs[k] = {"$ref": sid}; changed.append(k)
            elif k in sdefs: defs[k] = {"$ref": f"{sid}#/$defs/{k}"}; changed.append(k)
        if changed:
            total += len(changed)
            print(subset, leaf.split("🧬️mutations/")[1].split("/")[0], changed)
            if apply: open(leaf, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
print("defs replaced", total)
