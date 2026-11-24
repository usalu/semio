"""🗂️ The drawn shapes of a person at a scale whose id or bone contains a substring, per view: id, bone, how it is placed (flat, the solid kind with sweep or mirror), filled or stroke; and the body solids of those bones: python3 kinds.py <slug> <scale> <substring,…>"""
import json, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
slug, scale, subs = sys.argv[1], sys.argv[2], sys.argv[3].split(",")
a = json.load(open(slug if slug.endswith(".json") else f"{CAT}/{slug}/🔣️.json"))["asset"]
hit = lambda s: any(x in s["id"] or x in s["bone"] for x in subs)
for v in ["elevation", "front", "plan"]:
    out = []
    for s in a["representations"][scale]["geometry"][v]:
        if hit(s):
            so = s.get("solid")
            how = "flat" if so is None else ("rev" + ("" if so.get("mirror") else "-half") if so["kind"] == "revolved" else "ext-" + so["sweep"] + ("-tip" if "zTip" in so else "") + ("-skin" if "weights" in so else ""))
            out.append(f"{s['id']}[{s['bone']}]{how}{'' if s.get('filled', True) else '/open'}")
    print(f"{v}: " + " ".join(out))
print("body: " + " ".join(f"{s['id']}[{s['bone']}]{s['kind']}{'~' if s.get('slender') else ''}{'/w' if 'weights' in s else ''}" for s in a["volumes"][scale]["body"] if hit(s)))
