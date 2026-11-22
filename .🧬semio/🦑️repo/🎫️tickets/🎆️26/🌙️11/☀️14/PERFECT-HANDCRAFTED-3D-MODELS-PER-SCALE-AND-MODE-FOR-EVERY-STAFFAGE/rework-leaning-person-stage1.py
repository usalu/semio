"""💀 Stage 1 of humans/leaning-person: skull with face and mandible, iliac wings that follow the bowed crest.

Head frame: x up from the atlanto-occipital joint, y back, z left. Loaded by rework-leaning-person-build.py.
"""
import copy
import json
import os


def cap(sid, bone, p0, p1, r0, r1=None, **flags):
    s = {"id": sid, "kind": "capsule", "bone": bone, "material": "bone", "role": "body", "tone": 0.92, "p0": p0, "p1": p1, "r0": r0, "r1": r1 or r0}
    s.update(flags)
    s.update({"member": bone, "part": "member"})
    return s


def skull(m, scale):
    out = [m.local_ball("cranium", "head", [0.066, 0.008, 0], [0.07, 0.092, 0.073])]
    if scale in ("200", "500"):
        out.append(m.barrel("face-skeleton", "head", [(0.03, -0.056, 0.05, 0.032), (-0.02, -0.06, 0.046, 0.038), (-0.06, -0.058, 0.04, 0.038), (-0.082, -0.06, 0.038, 0.0375)]))
        return out
    floor = m.HALF[scale]
    out.append(m.barrel("facial-skeleton", "head", [(0.034, -0.06, 0.048, 0.026), (0.0, -0.066, 0.042, 0.026), (-0.03, -0.066, 0.032, 0.026), (-0.046, -0.064, 0.028, 0.024)]))
    out.append(cap("brow-ridge", "head", [0.052, -0.078, -0.044], [0.052, -0.078, 0.044], max(0.008, floor)))
    out.append(cap("nasal-bone", "head", [0.044, -0.082, 0], [0.01, -0.094, 0], max(0.006, floor)))
    jaw = [(0.016, -0.004, 0.05, 0.007), (-0.02, -0.006, 0.05, 0.0075), (-0.05, -0.012, 0.046, 0.009), (-0.064, -0.05, 0.034, 0.008), (-0.076, -0.08, 0.0, 0.0095)]
    path = jaw + [(x, y, -z, r) for x, y, z, r in reversed(jaw[:-1])]
    out.append(m.local_rod("mandible", "head", [(x, y, z, max(r, floor)) for x, y, z, r in path]))
    for s, sg in (("l", 1), ("r", -1)):
        out.append(m.local_ball(f"zygomatic-{s}", "head", [0.01, -0.062, 0.05 * sg], [0.014, 0.016, 0.014]))
        out.append(cap(f"zygomatic-arch-{s}", "head", [0.014, -0.055, 0.058 * sg], [0.016, -0.006, 0.058 * sg], max(0.005, floor)))
        out.append(m.local_ball(f"mastoid-{s}", "head", [0.008, 0.03, 0.05 * sg], [0.014, 0.012, 0.012]))
    return out


def wings(m, k, scale):
    out = []
    for s, sg in (("l", 1), ("r", -1)):
        b = f"ilium-{s}"
        arc = m.side([[0.04, 0.085, 0.112], [0.012, 0.09, 0.125], [-0.025, 0.095, 0.118], [-0.058, 0.098, 0.085], [-0.078, 0.098, 0.05]], sg)
        out.append(k.plate(f"ilium-{s}", b, arc, [0.028, 0.05, 0.055, 0.045, 0.03], 0.006, [0, 0.9, 0.35 * sg]))
        body = m.side([[-0.008, 0.055, 0.1], [-0.002, 0.012, 0.082]], sg)
        out.append(k.capsule(f"ilium-body-{s}", b, *body, 0.02, 0.02))
    return out


def apply(a, m):
    fr = m.frames(a["rig"]["bones"])
    for scale in m.SCALES:
        lst = a["volumes"][scale]["structure"]
        drop = ("cranium", "face-skeleton", "mandible")
        lst[:] = [s for s in lst if not s["id"].startswith(drop)]
        lst.extend(skull(m, scale))
        if scale != "500":
            lst[:] = [s for s in lst if s["id"] not in ("ilium-l", "ilium-r")]
            lst.extend(wings(m, m.Kit(fr, m.HALF[scale]), scale))
        for s in lst:
            if s["id"].startswith("patella-"):
                s["transform"]["translation"][1] = 0.034
        if scale == "200":
            print("[DEBUG] 1:200 structure:", " ".join(s["id"] for s in lst))
        if scale in ("50", "100"):
            for sd, sg in (("l", 1), ("r", -1)):
                lst.append(m.local_ball(f"calcaneal-tuber-{sd}", f"foot-{sd}", [-0.05, -0.052, 0.004 * sg], [0.02, 0.02, 0.02]))


def reach(a, m):
    ends = {"humerus-": 0.292, "ulna-": 0.25, "radius-": 0.249}
    for s in a["volumes"]["50"]["structure"] + a["volumes"]["100"]["structure"]:
        for prefix, x in ends.items():
            if s["id"] in (prefix + "l", prefix + "r"):
                s["sections"][-1]["at"][0] = x
    for s in a["volumes"]["200"]["body"]:
        if s["id"] in ("shank-l", "shank-r"):
            s["sections"][-1]["at"][0] = max(s["sections"][-1]["at"][0], 0.405)
    anchor = next(x for x in a["anchors"] if x["id"] == "wall-shoulder")
    for scale in m.SCALES:
        ap = max(0.02, m.HALF[scale])
        pad = {"id": "bearing-wall-shoulder", "kind": "ellipsoid", "bone": "torso", "material": "bone", "role": "body", "tone": 0.7, "radii": [0.06, ap, 0.11], "transform": {"translation": [anchor["x"], round(anchor["y"] - ap, 5), 0]}, "member": "torso", "part": "member"}
        a["volumes"][scale]["structure"].append(pad)
    fine = a["volumes"]["50"]["structure"]
    for rib in [s for s in fine if s["id"] in ("rib-12-l", "rib-12-r")]:
        head = rib["sections"][0]["at"]
        fine.append(cap("costovertebral-12-" + rib["id"][-1], "torso", [0.0127, round(m.spine_back(0.1827), 4), 0], [round(c, 5) for c in head], 0.006))
    for scale in ("200", "500"):
        lst = a["volumes"][scale]["structure"]
        lst[:] = [s for s in lst if s["id"] == "rib-cage" or not s["id"].startswith(("rib-", "sternum", "costal", "cartilage"))]
        for s in a["volumes"][scale]["thermal"]:
            if s["id"] == "th-cervical-skin":
                keep = {k: s[k] for k in ("id", "region", "layer", "tags") if k in s}
                s.clear()
                s.update({"id": keep.pop("id"), "of": "neck", **keep})


def nowall(doc, path):
    twin = copy.deepcopy(doc)
    for vol in twin["asset"]["volumes"].values():
        for name in ("body", "thermal", "structure"):
            vol[name] = [s for s in vol[name] if "wall" not in s["id"] and s.get("bone") != "wall"]
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(twin, f, ensure_ascii=False)
