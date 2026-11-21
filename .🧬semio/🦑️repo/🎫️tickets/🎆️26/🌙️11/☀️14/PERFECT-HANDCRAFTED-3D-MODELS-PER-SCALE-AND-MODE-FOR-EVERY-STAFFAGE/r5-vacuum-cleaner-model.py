"""🌀 Round 5 vacuum cleaner: keeps the hand-stated lists of `r3-vacuum-cleaner-model.py` and rebuilds the hose of every scale so it bends instead of kinking. Each 0.15 m hose segment keeps a rigid middle on its own bone (0.025 to 0.125 m) and every joint between two segments gets two bend pieces of 25 mm, blended over both bones with weights 2/3 : 1/3 and 1/3 : 2/3, so a 40 degree joint turns in three steps of about 13 degrees; the rounded ends of the pieces overlap, so the hose stays closed in every pose and clip. At 1:50 each piece carries one corrugation rib at the 25 mm pitch of the rigid middles. Thermal follows the body (references, or its own true-size skins at 1:100 and 1:500); structure keeps one wall member per hose bone with the joints at the bone joints. Writes `volumes/appliances/vacuum-cleaner.json`.

Usage: python3 r5-vacuum-cleaner-model.py
"""
import importlib.util
import json
import os
from pathlib import Path
from runpy import run_path

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("model3", os.path.join(HERE, "r3-vacuum-cleaner-model.py"))
M = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(M)
K = M.K
SEG = K.HOSE_SEG
N = K.HOSE_N
RIGID = (0.025, 0.125)
BEND = ((0.125, 0.15, 2.0 / 3.0), (0.15, 0.175, 1.0 / 3.0))
RIB = [(-0.0015, 0.0172), (-0.0015, 0.0195), (0.0015, 0.0195), (0.0015, 0.0172), (-0.0015, 0.0172)]
PITCH = 0.025


def span(i):
    """📏 The rigid middle of hose segment i (1-based): the first starts at the socket cuff, the last runs into the end fitting."""
    return (0.0 if i == 1 else RIGID[0], SEG if i == N else RIGID[1])


def weights(i, w):
    return [{"bone": f"hose-{i}", "weight": round(w, 6)}, {"bone": f"hose-{i + 1}", "weight": round(1.0 - w, 6)}]


def tube(sid, i, a, b, r, w=None, **extra):
    s = K.capsule(sid, f"hose-{i}", M.HOSE, (a, 0, 0), (b, 0, 0), r, tone=0.35, **extra)
    if w is not None:
        s["weights"] = weights(i, w)
    return s


def rib(sid, i, at, count, w=None):
    s = K.revolve(sid, f"hose-{i}", M.HOSE, [(at + u, r) for u, r in RIB], (1, 0, 0), role="texture", tone=0.15, texture=True, repeat={"count": count, "translation": [PITCH, 0, 0]} if count > 1 else None)
    if s.get("repeat") is None:
        s.pop("repeat", None)
    if w is not None:
        s["weights"] = weights(i, w)
    return s


def hose(r, ribs):
    """🐍 The bending hose of one scale: rigid middles, two weighted bend pieces per joint, at 1:50 the corrugation ribs on both."""
    out = []
    for i in range(1, N + 1):
        a, b = span(i)
        out.append(tube(f"hose-{i}-tube", i, a, b, r))
        if ribs:
            first = 0.0125 if i == 1 else 0.0375
            last = 0.0875 if i == N else 0.1125
            out.append(rib(f"hose-{i}-rib", i, first, int(round((last - first) / PITCH)) + 1))
        if i < N:
            for k, (p0, p1, w) in enumerate(BEND):
                out.append(tube(f"hose-{i}-bend-{'ab'[k]}", i, p0, p1, r, w))
                if ribs:
                    out.append(rib(f"hose-{i}-bend-{'ab'[k]}-rib", i, (p0 + p1) / 2, 1, w))
    return out


def replace(solids, prefix_ids, new):
    """🔁 The list with the old hose solids replaced by the new ones at the place of the first old one."""
    at = next(k for k, s in enumerate(solids) if s["id"] in prefix_ids)
    rest = [s for s in solids if s["id"] not in prefix_ids]
    return rest[:at] + new + rest[at:]


def thermal_hose(thermal, body_hose, r):
    """🌡️ The hose region of one scale: references to the body hose where the body is the true hose, else its own true-size skins."""
    old = {f"th-hose-{i}" for i in range(1, N + 1)}
    first = next(s for s in thermal if s["id"] in old)
    if "of" in first:
        new = [K.ref(f"th-{s['id']}", s["id"], region="hose", layer="hose") for s in body_hose if not s.get("texture")]
    else:
        new = [K.thermal(dict(s, id=f"th-{s['id']}"), "hose", "hose") for s in hose(r, False)]
    return replace(thermal, old, new)


def main():
    vol = M.volumes()
    for den, r in (("50", K.HOSE_R), ("100", K.HOSE_R), ("200", K.HOSE_R), ("500", 0.03)):
        v = vol[den]
        old = {f"hose-{i}-tube" for i in range(1, N + 1)} | {f"hose-{i}-rib" for i in range(1, N + 1)}
        new = K.r5(hose(r, den == "50"))
        v["body"] = replace(v["body"], old, new)
        v["thermal"] = K.r5(thermal_hose(v["thermal"], new, K.HOSE_R))
    catalog = next(Path(HERE).parents[6].glob("🎓*/*/*staffage/🗂*/*/vacuum-cleaner/🔣️.json"))
    asset = json.loads(catalog.read_text(encoding="utf-8"))["asset"]
    asset["volumes"] = vol
    run_path(os.path.join(HERE,"digital-coarse-parts.py"))["author"](asset)
    K.SOURCE.write_text(json.dumps({"volumes": vol}, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    for den, v in vol.items():
        print(den, {l: len(v[l]) for l in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
