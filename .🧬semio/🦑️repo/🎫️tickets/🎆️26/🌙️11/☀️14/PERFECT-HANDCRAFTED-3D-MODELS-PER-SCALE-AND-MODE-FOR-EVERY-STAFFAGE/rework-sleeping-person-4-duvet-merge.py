"""🧵 Take only the re-sewn duvet (cloth bones, their pose and clip offsets, cells, webs, cell drawings) from the sewn document into a candidate.

Usage: python3 rework-sleeping-person-4-duvet-merge.py <body candidate> <sewn document> <output candidate>
Everything else (thermal regions and areas, hem supports, anchors, skeleton, decisions) stays as the body candidate has it.
"""
import json
import sys


def cloth(identity):
    return identity.startswith("quilt-")


def by_id(solids, fresh, keep):
    """🪡 Replace every solid that `keep` selects by the one of the same id from `fresh`."""
    new = {s["id"]: s for s in fresh if keep(s)}
    old = {s["id"] for s in solids if keep(s)}
    if old != set(new):
        print("[DEBUG] id sets differ", len(old), len(new), sorted(old ^ set(new))[:6])
    return [new.get(s["id"], s) if keep(s) else s for s in solids] + [s for i, s in new.items() if i not in old]


def main(body_path, sewn_path, out_path):
    document, sewn = json.load(open(body_path)), json.load(open(sewn_path))
    a, b = document["asset"], sewn["asset"]
    a["rig"]["bones"] = [x for x in a["rig"]["bones"] if not cloth(x["id"])] + [x for x in b["rig"]["bones"] if cloth(x["id"])]
    for pose in a["poses"]:
        fresh = next(p for p in b["poses"] if p["id"] == pose["id"])["bones"]
        pose["bones"] = {k: v for k, v in pose["bones"].items() if not cloth(k)} | {k: v for k, v in fresh.items() if cloth(k)}
    for clip in a["clips"]:
        if clip["id"] == "turn-over":
            clip["keys"] = next(c for c in b["clips"] if c["id"] == "turn-over")["keys"]
    cell = lambda s: s.get("material") == "duvet-cover" and s.get("kind") in ("polyhedron", "poly") and s["id"] != "pillow" and not s["id"].startswith("side-hem")
    web = lambda s: s["id"].startswith("quilt-web-")
    for scale, lists in a["volumes"].items():
        lists["body"] = by_id(lists["body"], b["volumes"][scale]["body"], cell)
        lists["structure"] = by_id(lists["structure"], b["volumes"][scale]["structure"], web)
        geometry = a["representations"][scale]["geometry"]
        for view in geometry:
            geometry[view] = by_id(geometry[view], b["representations"][scale]["geometry"][view], cell)
    json.dump(document, open(out_path, "w"), ensure_ascii=False)


main(*sys.argv[1:4])
