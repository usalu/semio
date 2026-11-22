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
    out = [m.local_ball("cranium", "head", [0.07, 0.012, 0], [0.066, 0.09, 0.072])]
    if scale in ("200", "500"):
        out.append(m.local_ball("face-skeleton", "head", [-0.034, -0.05, 0], [0.046, 0.042, 0.046]))
        return out
    floor = m.HALF[scale]
    out.append(m.local_ball("maxilla", "head", [-0.02, -0.068, 0], [0.026, 0.024, 0.03]))
    out.append(cap("brow-ridge", "head", [0.052, -0.078, -0.044], [0.052, -0.078, 0.044], max(0.008, floor)))
    out.append(cap("nasal-bone", "head", [0.044, -0.082, 0], [0.01, -0.094, 0], max(0.006, floor)))
    jaw = [(0.012, 0.0, 0.052, 0.007), (-0.02, -0.006, 0.05, 0.0075), (-0.05, -0.012, 0.046, 0.009), (-0.064, -0.05, 0.034, 0.008), (-0.076, -0.08, 0.0, 0.0095)]
    path = jaw + [(x, y, -z, r) for x, y, z, r in reversed(jaw[:-1])]
    out.append(m.local_rod("mandible", "head", [(x, y, z, max(r, floor)) for x, y, z, r in path]))
    for s, sg in (("l", 1), ("r", -1)):
        out.append(m.local_ball(f"zygomatic-{s}", "head", [0.01, -0.062, 0.05 * sg], [0.014, 0.016, 0.014]))
        out.append(cap(f"zygomatic-arch-{s}", "head", [0.014, -0.055, 0.058 * sg], [0.012, -0.004, 0.066 * sg], max(0.005, floor)))
        out.append(m.local_ball(f"mastoid-{s}", "head", [0.0, 0.022, 0.05 * sg], [0.014, 0.012, 0.01]))
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


def nowall(doc, path):
    twin = copy.deepcopy(doc)
    for vol in twin["asset"]["volumes"].values():
        for name in ("body", "thermal", "structure"):
            vol[name] = [s for s in vol[name] if "wall" not in s["id"] and s.get("bone") != "wall"]
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(twin, f, ensure_ascii=False)
