"""✏️ Rework 05: redraws the 2D drawings of a candidate document from its body solids, one filled outline per solid in the frame of its bone, through the product's own projection (`r3-project2d.ts` run on the candidate).

Usage: python3 rework-05-draw2d.py <slug> [scales…]  (reads and writes `🗑️generated/r3-<slug>/🔣️.json`)
Stage 1 gives every drawable body solid a placeholder shape with its id, bone, role, tone, region and material; stage 2 lets the projection draw each outline (elevation first, because front and plan take their depth from it).
@see ./r3-tools.md
"""
import importlib.util
import os
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "rework-05-kit.py"))
kit = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(kit)
VIEWS = ("elevation", "front", "plan")


def moving_out_of_plane(asset):
    """🌀 The bones that a pose or clip key twists or spreads, with all their descendants, and the bones that run laterally at rest (clavicles): their shapes must be `solid` placements."""
    hit = set()
    offsets = [p["bones"] for p in asset["poses"]] + [k.get("bones", {}) for c in asset["clips"] for k in c["keys"]]
    for bones in offsets:
        for id, o in bones.items():
            if o.get("twistDeg") or o.get("abductionDeg"):
                hit.add(id)
    grew = True
    while grew:
        grew = False
        for b in asset["rig"]["bones"]:
            if b["parent"] in hit and b["id"] not in hit:
                hit.add(b["id"])
                grew = True
    for b in asset["rig"]["bones"]:
        if b.get("abductionDeg") and not b.get("twistDeg") and b["id"].startswith("clavicle"):
            hit.add(b["id"])
    return hit


def placeholders(asset, scale, rest, order=None):
    """🧩 One shape per drawable body solid for each view: a placeholder that the projection redraws, or, on a bone that twists or spreads, the solid placement of the same stations; `rest` are the rest frames, `order(view, solid)` the paint order z (default: the list order)."""
    volume = asset["volumes"][str(scale)]
    moving = moving_out_of_plane(asset)
    region = {t["of"]: t["region"] for t in volume["thermal"] if "of" in t}
    for b in volume["body"]:
        for t in volume["thermal"]:
            if b["id"] not in region and "of" not in t and t["id"].startswith("th-" + b["id"] + "-"):
                region[b["id"]] = t["region"]
    geometry = {}
    toward = {"elevation": (0, 0, -1), "front": (1, 0, 0), "plan": (0, 1, 0)}
    for view in VIEWS:
        shapes = []
        near = sorted(range(len(volume["body"])), key=lambda i: (round(kit.reach(volume["body"][i], rest[volume["body"][i]["bone"]], toward[view]), 3), i))
        rank = {i: n for n, i in enumerate(near)}
        for i, b in enumerate(volume["body"]):
            if b.get("texture") or b.get("slender"):
                continue
            shape = {"id": b["id"], "bone": b["bone"], "role": "detail" if b.get("role") == "detail" else "body", "z": round(order(view, b) if order else 1 + rank[i] * 0.01, 3), "filled": True, "tone": b["tone"], "material": b["material"], "kind": "poly", "points": [[0, 0], [0.01, 0], [0, 0.01]], "closed": True}
            if b["id"] in region:
                shape["region"] = region[b["id"]]
            if b["bone"] in moving and b["kind"] in ("loft", "capsule", "ellipsoid", "box"):
                fields = {k: v for k, v in shape.items() if k not in ("kind", "points", "closed", "filled")}
                shape = kit.solid_shape(b, rest[b["bone"]]["o"][2], fields)
            shapes.append(shape)
        geometry[view] = shapes
    return geometry


def project(slug, category, candidate, scale):
    """📽️ Draws the three views of one scale: every placeholder is replaced by the projected outline of its solid."""
    before = kit.read_json(candidate)["asset"]["representations"][str(scale)]["geometry"]
    for view in VIEWS:
        keep = {x["id"]: x for x in before[view] if "solid" in x}
        run = subprocess.run([os.path.join(HERE, "rework-05-tool.sh"), "project2d", candidate, "%s/%s" % (category, slug), "--scale", str(scale), "--view", view, "--candidate", "--quiet"], capture_output=True, text=True)
        made = os.path.join(HERE, "🗑️generated", "r3-project2d", category, "%s.s%d.%s" % (slug, scale, view), "🔣️.json")
        if run.returncode != 0 or not os.path.exists(made):
            raise SystemExit("[DEBUG] project2d %s %s failed: %s" % (scale, view, (run.stderr or run.stdout)[-1500:]))
        drawn = kit.read_json(made)
        os.remove(made)
        shapes = drawn["asset"]["representations"][str(scale)]["geometry"][view]
        drawn["asset"]["representations"][str(scale)]["geometry"][view] = [keep.get(x["id"], x) for x in shapes]
        kit.write_json(candidate, drawn)
        print("[DEBUG] drew 1:%d %s" % (scale, view))


def main():
    """▶️ Placeholders, then projection, for the given scales (default all four)."""
    slug = sys.argv[1]
    scales = [int(x) for x in sys.argv[2:]] or [50, 100, 200, 500]
    candidate = os.path.join(HERE, "🗑️generated", "r3-" + slug, "🔣️.json")
    doc = kit.read_json(candidate)
    probe = kit.read_json(candidate)
    probe["asset"]["poses"].append({**probe["asset"]["poses"][0], "id": "rest-probe", "bones": {}})
    probe_path = os.path.join(HERE, "🗑️generated", "rework-05", "rest-probe.json")
    kit.write_json(probe_path, probe)
    rest = kit.frames_of(probe_path, ["rest-probe"])["rest-probe"]
    os.remove(probe_path)
    for s in scales:
        doc["asset"]["representations"][str(s)]["geometry"] = placeholders(doc["asset"], s, rest)
    kit.write_json(candidate, doc)
    for s in scales:
        project(slug, doc["asset"]["category"], candidate, s)


if __name__ == "__main__":
    main()
