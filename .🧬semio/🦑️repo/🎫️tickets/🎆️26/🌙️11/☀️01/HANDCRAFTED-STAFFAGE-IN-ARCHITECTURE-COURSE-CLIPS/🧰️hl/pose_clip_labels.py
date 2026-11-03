import json, os, sys

ROOT = r"C:\git\semio"
CAT = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")
MANIFEST = os.path.join(CAT, "🔣️.json")
VIS = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "⚡️energy", "🏗️authoring", "🎨️visuals", "🧍️staffage", "🔣️.json")

with open(MANIFEST, encoding="utf-8") as fh:
    manifest = json.load(fh)
entries = {e["id"]: e for e in manifest["entries"]}

out = []
for aid, e in entries.items():
    cat = e["category"]
    path = None
    for root, _, files in os.walk(CAT):
        if os.path.basename(root) == aid and "🔣️.json" in files:
            path = os.path.join(root, "🔣️.json")
    if path is None:
        continue
    with open(path, encoding="utf-8") as fh:
        asset = json.load(fh)["asset"]
    t = asset.get("title", {})
    if t.get("en") != e["title"].get("en") or t.get("de") != e["title"].get("de"):
        out.append(f"TITLE MISMATCH {aid}: manifest={e['title']} asset={t}")
    for kind in ("poses", "clips"):
        asset_items = {x["id"]: x for x in asset.get(kind, [])}
        for mid in e.get(kind, []):
            if mid not in asset_items:
                out.append(f"MISSING {kind} {aid}.{mid} in asset file")
                continue
            lab = asset_items[mid].get("title")
            if not isinstance(lab, dict) or not lab.get("en") or not lab.get("de"):
                out.append(f"NO BILINGUAL TITLE {kind} {aid}.{mid}: {lab}")
        for aid_item in asset_items:
            if aid_item not in e.get(kind, []):
                out.append(f"EXTRA in asset not manifest {kind} {aid}.{aid_item}")
    for kind in ("poses", "clips"):
        for x in asset.get(kind, []):
            for fld in ("title", "description"):
                v = x.get(fld)
                if v is not None and (not isinstance(v, dict) or not v.get("en") or not v.get("de")):
                    out.append(f"NO BILINGUAL {fld} {kind} {aid}.{x['id']}")
    for fld in ("summary", "title"):
        v = asset.get(fld)
        if not isinstance(v, dict) or not v.get("en") or not v.get("de"):
            out.append(f"NO BILINGUAL {fld} {aid}")

for line in out:
    print(line)
print("issues", len(out))
