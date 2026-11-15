from author_30_lib import *

PERSON = ("pelvis", "torso", "neck", "head", "arm", "upper-arm", "forearm", "hand", "thigh", "shank", "foot", "toe")


def is_person_bone(b, extra_skip=()):
    return any(b == p or b.startswith(p + "-") for p in PERSON) and not b.startswith(("toddler", "crutch")) and b not in extra_skip


def bbox_ellipsoid(P, bone, sid, mat, tone, ft):
    pts = []
    for v in ("elevation", "front", "plan"):
        for r, _ in P.rings(v, bone):
            pts += r
    if not pts:
        return None
    lo = [min(p[k] for p in pts) for k in range(3)]
    hi = [max(p[k] for p in pts) for k in range(3)]
    c = [(a + b) / 2 for a, b in zip(lo, hi)]
    rad = [max((b - a) / 2, 0.55 * ft) for a, b in zip(lo, hi)]
    if bone == "neck":
        rad = [min(rad[0], 0.09), min(rad[1], 0.06), min(rad[2], 0.06)]
    return ellipsoid(sid, bone, mat, tone, "body", c, rad)


def groups_of(P, bone, skip=lambda r: False, skip_ids=()):
    g = {}
    for v in ("elevation", "front", "plan"):
        for s in P.d[v]:
            if s["bone"] != bone or s["role"] != "body" or not (s["filled"] or s["closed"]) or s["id"] in skip_ids:
                continue
            r = s.get("region") or (s.get("regions") or [None])[0]
            if skip(r):
                continue
            g.setdefault(r, {"elevation": [], "front": [], "plan": []})[v].append(s["id"])
    return g


def person_solids(P, scale, a, order, mats, default_region, skip=lambda r: False, tol=0.5, maxn=None, skip_bones=(), special=None, wpad=None, e=2.3, tones=None, stroke_half=None, toddler=False, skip_ids=()):
    ft = FEATURE[str(scale)]
    maxn = maxn or {50: 10, 100: 7, 200: 5, 500: 5}[scale]
    out = []
    for b in a["rig"]["bones"]:
        bone = b["id"]
        if not (is_person_bone(bone, skip_bones) or (toddler and bone.startswith("toddler"))):
            continue
        g = groups_of(P, bone, skip, skip_ids)
        if not g:
            continue
        if None in g:
            r0 = default_region.get(bone)
            g.setdefault(r0, {"elevation": [], "front": [], "plan": []})
            for v in g[None]:
                g[r0][v] += g[None][v]
            del g[None]
        regs = sorted(g, key=lambda r: order.index(r) if r in order else 99)
        for k, r in enumerate(regs):
            sid = bone if k == 0 else f"{bone}-{r}"
            el, fr, pl = g[r]["elevation"], g[r]["front"], g[r]["plan"]
            wv = "front" if abs(P.f[bone]["u"][1]) >= 0.5 else "plan"
            wid = fr if wv == "front" else pl
            if not wid:
                wid = pl if wv == "front" else fr
                wv2 = "plan" if wv == "front" else "front"
            else:
                wv2 = wv
            own_w = bool(fr or pl)
            rows = rows_of(P, bone, inc_el=el or None, inc_w=wid or None, wview=wv2, wpad=(0.0 if own_w else (wpad or {}).get(r, 0.0)), trim=0.003, stroke_half=stroke_half) if (el or wid) else None
            mat = mats.get(r, "skin")
            tone = (tones or {}).get(r, 0.5)
            if rows is None or len(rows) < 2 or (rows[-1][0] - rows[0][0]) < 0.004:
                e_ = bbox_ellipsoid(P, bone, sid, mat, tone, ft)
                if e_:
                    out.append(e_)
                continue
            if bone.startswith("toe") and not el and not P.rings("elevation", bone):
                rows = [[r[0], r[1] - 0.0, r[2], r[3], min(r[4], 0.0125)] for r in rows]
            out.append(loft_rows(sid, bone, mat, tone, "body", rows, ft * tol, maxn if k == 0 else max(2, maxn - 1), minr=0.55 * ft, e=e))
            if out[-1]["kind"] == "loft":
                out[-1] = clamp_feature(out[-1], ft)
            out[-1]["_r"] = r
    return out
