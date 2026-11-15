import json, math
from author_08_lib import *

PERSON_BONES = ("pelvis", "torso", "neck", "head", "arm", "upper-arm", "forearm", "hand", "thigh", "shank", "foot", "toe")


def is_person(b):
    return any(b == p or b.startswith(p + "-") for p in PERSON_BONES) and not b.startswith("toddler") and not b.startswith("crutch")


def seed_of(slug):
    return json.load(open(f"{GEN}/seed-{slug}.json"))


def seed_attr(seed, bone):
    from author_08_tail import main_solid
    cand = [s for s in seed["volume"]["solids"] if s["bone"] == bone and s["role"] == "body"]
    if not cand:
        return None
    lofts = [s for s in cand if s["kind"] == "loft"] or cand
    m = main_solid(lofts, bone)
    reg = m.get("region") or (m.get("regions") or [None])[0]
    return m["material"], m["tone"], reg


def seed_part_attr(seed, sid):
    for s in seed["volume"]["solids"]:
        if s["id"] == sid:
            reg = s.get("region") or (s.get("regions") or [None])[0]
            return s["material"], s["tone"], reg, s["role"]
    return None


TOL = {"50": 0.5, "100": 0.5, "200": 0.5, "500": 0.5}
MAXN = {"50": 12, "100": 7, "200": 4, "500": 3}


def part_solid(P, scale, bone, sid, mat, tone, reg, include=None, exclude=(), exp=2.3, roles=("body",), role="body", maxn=None, tolf=None, trim=0.0, wview=None, minrad=None, cap=(True, True)):
    rows = bone_profile(P, bone, roles=roles, include=include, exclude=exclude, trim=trim, wview=wview)
    if not rows:
        return None
    ft = FEATURE[scale]
    st = simplify(rows, ft * (tolf if tolf is not None else TOL[scale]), maxn or MAXN[scale])
    mr = (minrad if minrad is not None else ft / 2) + 0.0005
    st = [[s[0], s[1], s[2], max(s[3], mr), max(s[4], mr)] for s in st]
    if len(st) == 1:
        st = [st[0], st[0]]
    return loft(sid, bone, mat, tone, role, st, exp=exp, cap=cap), reg


def person_body(slug, scale, cfg):
    """Generated body solids of every person bone with drawn shapes at this scale; returns (solids, regions)."""
    cat = "humans"
    a = load_asset(cat, slug)
    seed = seed_of(slug)
    P = Prof(slug, scale)
    out, reg = [], {}
    exps = cfg.get("exp", {})
    for b in a["rig"]["bones"]:
        bid = b["id"]
        if not cfg["is_person"](bid):
            continue
        parts = cfg.get("parts", {}).get(bid)
        if parts is None:
            at = seed_attr(seed, bid)
            if at is None:
                par = next((x["parent"] for x in a["rig"]["bones"] if x["id"] == bid), None)
                at = seed_attr(seed, par) if par else None
            if at is None:
                continue
            parts = [dict(id=bid, mat=at[0], tone=at[1], region=at[2])]
        for p in parts:
            r = part_solid(P, scale, bid, p["id"], p["mat"], p["tone"], p["region"], include=p.get("include"), exclude=p.get("exclude", ()), exp=p.get("exp", exps.get(bid, 2.3)), roles=p.get("roles", ("body",)), role=p.get("role", "body"), maxn=p.get("maxn"), tolf=p.get("tolf"), trim=p.get("trim", 0.0), wview=p.get("wview"), minrad=p.get("minrad"), cap=p.get("cap", (True, True)))
            if r is None and P.has_shapes(bid):
                r = fallback_ellipsoid(a, scale, bid, p)
            if r is None:
                continue
            out.append(r[0])
            reg[p["id"]] = p["region"]
    return out, reg


def fallback_ellipsoid(a, scale, bid, p):
    rb = next(x for x in a["rig"]["bones"] if x["id"] == bid)
    L = max(rb.get("lengthM") or 0.08, 0.05)
    m = FEATURE[scale] / 2 + 0.0015
    sz = {"pelvis": (L / 2 + 0.03, 0.1, 0.15), "toe": (L / 2, 0.03, 0.045), "neck": (L / 2, 0.05, 0.05)}
    key = next((k for k in sz if bid.startswith(k)), None)
    r = sz[key] if key else (L / 2, 0.04, 0.04)
    return ellipsoid(p["id"], bid, p["mat"], p["tone"], "body", [L / 2, 0, 0], [max(r[0], m), max(r[1], m), max(r[2], m)]), p["region"]
