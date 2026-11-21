"""🩹 Round 5 vehicles: small root patches of a catalog asset outside `volumes`, stated by hand in a JSON patch file: `shapes` = [{scale, view, id, set: {member: value}}] replaces members of one 2D shape, `ranges` = {scale: {minDenominator?, maxDenominator?}} narrows a representation's valid range. Re-reads the catalog file and keeps every other member, its member order and its indent-1 style; the `volumes` text block stays as it is (spliced by volumes-insert.py).

Usage: python3 r5-vehicles-patch.py <category/slug> <patch.json> [--apply]   (without --apply: the candidate document with the volumes source goes to 🗑️generated/r5-<slug>/🔣️.json)"""
import glob
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))


def patch(asset, spec):
    for p in spec.get("shapes", []):
        shapes = [s for s in asset["representations"][str(p["scale"])]["geometry"][p["view"]] if s["id"] == p["id"]]
        assert len(shapes) == 1, p
        for k, v in p["set"].items():
            shapes[0][k] = v
    for scale, r in spec.get("ranges", {}).items():
        asset["representations"][scale]["validRange"].update(r)
    return asset


if __name__ == "__main__":
    name, spec_path = sys.argv[1], sys.argv[2]
    cat, slug = name.split("/")
    path = glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{slug}/🔣️.json")[0]
    spec = json.load(open(spec_path, encoding="utf8"))
    text = open(path, encoding="utf8").read()
    doc = json.loads(text)
    doc["asset"] = patch(doc["asset"], spec)
    out_dir = os.path.join(T, "🗑️generated", f"r5-{slug}")
    os.makedirs(out_dir, exist_ok=True)
    cand = json.loads(json.dumps(doc))
    cand["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", cat, slug + ".json"), encoding="utf8"))["volumes"]
    json.dump(cand, open(os.path.join(out_dir, "🔣️.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("candidate", os.path.join(out_dir, "🔣️.json"))
    if "--apply" in sys.argv:
        start = text.index('\n  "volumes": {') + 1
        end = text.index('\n  "presentation": {') + 1
        block = text[start:end]
        vol = doc["asset"]["volumes"]
        doc["asset"]["volumes"] = "@@VOLUMES@@"
        out = json.dumps(doc, ensure_ascii=False, indent=1) + "\n"
        out = out.replace('  "volumes": "@@VOLUMES@@",\n', block)
        assert json.loads(out)["asset"]["volumes"] == vol
        open(path, "w", encoding="utf8").write(out)
        print("applied", path)
