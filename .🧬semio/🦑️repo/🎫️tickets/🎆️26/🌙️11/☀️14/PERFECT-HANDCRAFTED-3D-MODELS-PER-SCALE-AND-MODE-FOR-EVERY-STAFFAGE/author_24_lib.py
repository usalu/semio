"""🧍️ Author 24 helpers on top of author_09: symmetrise left/right, load/save the working volumes, summaries."""
import copy
import json
import sys

sys.path.insert(0, ".")
from author_09_build import *


def load(slug):
    return json.load(open(f"{T}/volumes/humans/{slug}.json"))["volumes"]


def _mean(a, b):
    return (a + b) / 2


def sym_pair(l, r):
    k = l["kind"]
    if k == "loft" and len(l["sections"]) == len(r["sections"]):
        for p, q in zip(l["sections"], r["sections"]):
            for f in ("rx", "ry"):
                p[f] = q[f] = _mean(p[f], q[f])
            p["at"][0] = q["at"][0] = _mean(p["at"][0], q["at"][0])
            p["at"][1] = q["at"][1] = _mean(p["at"][1], q["at"][1])
            w = _mean(abs(p["at"][2]), abs(q["at"][2]))
            sp = -1 if p["at"][2] < 0 else 1
            sq = -1 if q["at"][2] < 0 else 1
            p["at"][2], q["at"][2] = sp * w, sq * w
    elif k == "ellipsoid":
        l["radii"] = r["radii"] = [_mean(a, b) for a, b in zip(l["radii"], r["radii"])]
        tl, tr = l["transform"]["translation"], r["transform"]["translation"]
        for i in (0, 1):
            tl[i] = tr[i] = _mean(tl[i], tr[i])
        w = _mean(abs(tl[2]), abs(tr[2]))
        tl[2], tr[2] = (-1 if tl[2] < 0 else 1) * w, (-1 if tr[2] < 0 else 1) * w
    elif k == "capsule":
        l["r0"] = r["r0"] = _mean(l["r0"], r["r0"])
        l["r1"] = r["r1"] = _mean(l["r1"], r["r1"])


def symmetrise(body, skip=()):
    by = {s["id"]: s for s in body}
    for i, s in by.items():
        if i.endswith("-l") and i[:-2] + "-r" in by and i not in skip:
            sym_pair(s, by[i[:-2] + "-r"])
    return body


def summary(slug):
    v = load(slug)
    return {s: {k: len(v[s].get(k, [])) for k in ("body", "thermal", "structure", "decisions")} for s in v}


def mess(slug, scale=None, rules=None, show_ok=False):
    out = []
    for m in measurements(slug, scale, rules):
        if show_ok or not m["ok"]:
            out.append(f"{m['rule']} {m['scale']} {m['name']}={m['value']}<{m['limit']} {m.get('note','')}")
    return out


def clip_members(slug, vols, floor_extent=None):
    """Re-sample every member loft inside the u-range of its bone's primary body loft (a core never leaves the body), keeping the core factor of its first section; the section stays at least half of floor_extent[scale] wide."""
    a = asset(slug)
    for sc in (50, 100, 200, 500):
        v = vols[str(sc)]
        body = v["body"]
        f = FEATURE[sc]
        for m in v["structure"]:
            if m.get("part") != "member" or m["kind"] != "loft":
                continue
            bone = m["bone"]
            p = primary(body, bone)
            if p is None or p["kind"] != "loft":
                continue
            L = next(b["lengthM"] for b in a["rig"]["bones"] if b["id"] == bone)
            us = sorted(q["at"][0] for q in p["sections"])
            lo, hi = us[0], us[-1]
            old = sorted(m["sections"], key=lambda q: q["at"][0])
            if abs(old[0]["at"][0] - lo) < 1e-6 and abs(old[-1]["at"][0] - hi) < 1e-6:
                continue
            k = 0.6
            n = len(old)
            rot = old[0].get("rotation", ROT_U)
            secs = []
            for i in range(n):
                u = lo + (hi - lo) * i / (n - 1)
                vv, w, rx, ry, e = sec_at_u(p["sections"], u)
                o = min(old, key=lambda q: abs(q["at"][0] - u))
                kx = o["rx"] / max(sec_at_u(p["sections"], o["at"][0])[2], 1e-9)
                ky = o["ry"] / max(sec_at_u(p["sections"], o["at"][0])[3], 1e-9)
                secs.append(sec(u, vv, w, max(rx * kx, 0.5008 * f), max(ry * ky, 0.5008 * f), min(e, 3.0), rot))
            m["sections"] = secs
    return vols


def fix_joints(slug, vols, k=1.5):
    """Joint balls no larger than k times the half section of their member at the joint end (a ball bigger than the member reads as a growth, not a joint)."""
    for sc in (50, 100, 200, 500):
        st = vols[str(sc)]["structure"]
        mem = {m["member"]: m for m in st if m.get("part") == "member" and m["kind"] == "loft"}
        f = FEATURE[sc]
        for j in st:
            if j.get("part") != "joint" or j["kind"] != "ellipsoid" or j["member"] not in mem:
                continue
            q = min(mem[j["member"]]["sections"], key=lambda q: q["at"][0])
            r = max(k * max(q["rx"], q["ry"]), 0.5008 * f)
            j["radii"] = [min(x, r) for x in j["radii"]]
    return vols


def shorten_emitters(vols, factor=0.6):
    for sc in (100, 200, 500):
        f = FEATURE[sc]
        for t in vols[str(sc)]["thermal"]:
            if t.get("kind") == "capsule" and "emitter" in t.get("tags", []):
                p0, p1 = t["p0"], t["p1"]
                d = [b - a for a, b in zip(p0, p1)]
                n = math.sqrt(sum(x * x for x in d)) or 1
                L = max(0.02, factor * f)
                t["p1"] = [a + x / n * L for a, x in zip(p0, d)]
    return vols
