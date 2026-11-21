"""🛏️ Round 5, furniture/single-bed: the round 3 bed with the mattress remodelled by hand where the sheets showed defects, written as the candidate document (🗑️generated/r5-single-bed/🔣️.json) and as the volumes source (volumes/furniture/single-bed.json).

Changes against round 3 (every station stated here):
- The three mattress parts meet in a soft quilted crease instead of square butt ends: each joint end of a part closes over its last 12 mm (1:50) or 15 mm (1:100) to a 6 / 8 mm smaller half height and a 4 / 6 mm smaller half width, so a part that sinks under a sleeper shows a rounded shoulder, not a ledge.
- The head part is 3 mm narrower on each side (half width 0.447 m) than the middle part, so that, when the head section is raised, the end that turns into the middle part lies inside it instead of sharing its side faces (no crossing seams at the fold); its piping follows at 0.444 m.
- 1:500: each mattress block is a crisp box 0.20 m high (the head block 3 mm narrower) instead of a two-station loft with 20 to 30 mm soft edges that lie below the 75 mm feature and shaded as long facets in the side view; the three blocks read as one clean mattress mass.

Usage: python3 r5-single-bed-build.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/single-bed/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r5-single-bed")

HEAD_INSET = 0.003
CREASE = {"50": [(0.0, 0.004, 0.006), (0.004, 0.0018, 0.0025), (0.008, 0.0006, 0.0008), (0.012, 0.0, 0.0)],
          "100": [(0.0, 0.006, 0.008), (0.007, 0.002, 0.003), (0.015, 0.0, 0.0)],
          "200": [],
          "500": []}


def module(name):
    """📦 Loads one of the round 3 sibling scripts."""
    spec = importlib.util.spec_from_file_location("r3_single_bed_" + name, os.path.join(HERE, f"r3-single-bed-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def patch(B, K, scale_of):
    """🪡 Replaces the mattress functions of one loaded body module by the round 5 forms."""

    def mattress_part(sid, bone, x0, x1, end, shape, inset=0.0, material=K.QUILT, tone=None):
        """🛏 One part of the mattress: superelliptic section 0.90 x 0.20 m, the free end rounded by `shape`, a joint end closed into the quilted crease; the head part 3 mm narrower each side."""
        narrow = HEAD_INSET if sid.endswith("mat-head") or sid.endswith("mat-head-core") else 0.0
        half_w, half_h = K.MAT_Z - inset - narrow, K.MAT_T / 2 - inset
        yc = K.MAT_Y0 + K.MAT_T / 2
        crease = CREASE[scale_of()]
        station = lambda x, dw, dh: (x, yc, 0, half_w - dw, half_h - dh, K.MAT_E)
        round_end = [(d + inset, dw, dh) for d, dw, dh in shape]
        crease_end = [(d, dw, dh) for d, dw, dh in crease]
        head = round_end if end == -1 else crease_end
        foot = round_end if end == 1 else crease_end
        lip = 0.003
        start = [station(x0 + d, dw, dh) for d, dw, dh in head] if head else [station(x0, 0, 0)]
        stop = [station(x1 - d, dw, dh) for d, dw, dh in reversed(foot)] if foot else [station(x1, 0, 0)]
        middle = [station(start[-1][0] + lip, 0, 0), station(stop[0][0] - lip, 0, 0)]
        return K.loft(sid, bone, material, start + middle + stop, tone=tone)

    def piping(sid, bone, x_joint, x_end, y, side, radius=0.004):
        """🧵 Half of a piped seam of an end part: along the long edge from the crease, round the corner and across the end to the centre line; on the head part 3 mm further in."""
        toward = 1 if x_end > x_joint else -1
        z_run = 0.447 - (HEAD_INSET if bone == "mat-head" else 0.0)
        x_face, corner = x_end - toward * 0.002, 0.045
        x_start = x_joint - toward * 0.012
        cx, cz = x_face - toward * corner, z_run - corner
        path = [(x_start, z_run), (cx, z_run)]
        for degrees in (30, 60):
            a = math.radians(degrees)
            path.append((cx + toward * corner * math.sin(a), cz + corner * math.cos(a)))
        path += [(x_face, cz), (x_face, 0.0)]
        return K.loft(sid, bone, K.QUILT, [(x, y, side * z, radius, radius, 2) for x, z in path], tone=0.7, role="detail")

    def piping_run(sid, bone, x0, x1, y, side, radius=0.004):
        """🧶 The piped seam along a long edge of the middle part, stopping at both creases."""
        return K.loft(sid, bone, K.QUILT, [(x0 + 0.012, y, side * 0.447, radius, radius, 2), (x1 - 0.012, y, side * 0.447, radius, radius, 2)], tone=0.7, role="detail")

    B.mattress_part, B.piping, B.piping_run = mattress_part, piping, piping_run


def body500(B, K):
    """🛰 1:500 as round 3, the three mattress blocks as crisp boxes."""
    s = [solid for solid in B.body500() if not solid["id"].startswith("mat-")]
    for sid, x0, x1 in (("mat-head", K.MAT_X0, K.HINGE_X), ("mat-mid", K.HINGE_X, K.FOOT_X), ("mat-foot", K.FOOT_X, K.MAT_X1)):
        half_w = K.MAT_Z - (HEAD_INSET if sid == "mat-head" else 0.0)
        s.append(K.box(sid, sid, K.QUILT, (x0, x1), (K.MAT_Y0, K.MAT_Y0 + K.MAT_T), (-half_w, half_w)))
    return s


def candidate():
    """🧩 The catalog document with the round 3 roots and the round 5 volumes."""
    kit, body, lists, roots, draw = (module(name) for name in ("kit", "body", "lists", "roots", "draw"))
    current = {"scale": "50"}
    for B in (body, lists.B):
        patch(B, B.K, lambda: current["scale"])
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = document["asset"]
    asset["rig"] = kit.rig(asset["rig"])
    roots.apply(asset)
    for scale in ("50", "100", "200", "500"):
        current["scale"] = scale
        solids = body500(body, kit) if scale == "500" else getattr(body, "body" + scale)()
        asset["volumes"][scale] = {"status": "authored", "denominator": int(scale), "body": solids, "thermal": lists.thermal(scale, solids), "structure": lists.structure(scale, solids), "decisions": lists.decisions(scale)}
    draw.apply(asset)
    return document


def main():
    document = candidate()
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    volumes = document["asset"]["volumes"]
    json.dump({"volumes": volumes}, open(os.path.join(HERE, "volumes", "furniture", "single-bed.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("candidate", {k: [sum(s.get("repeat", {}).get("count", 1) for s in v[l]) for l in ("body", "thermal", "structure")] for k, v in volumes.items()})


if __name__ == "__main__":
    main()
