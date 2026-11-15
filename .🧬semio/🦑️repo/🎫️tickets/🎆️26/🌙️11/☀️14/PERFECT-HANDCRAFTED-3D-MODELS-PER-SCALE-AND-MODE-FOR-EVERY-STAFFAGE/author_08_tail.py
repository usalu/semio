import copy, json, math
from author_08_lib import *


def region_parts(R, body, reg, scale, cfg):
    parts = [b for b in body if reg.get(b["id"]) == R["id"]]
    extra = cfg.get("extra", {}).get(R["id"])
    if extra is not None:
        parts = parts + extra(scale, body)
    return parts


def thermal_list(a, scale, body, reg, cfg, fac=None):
    fac = fac or {}
    bodyids = {b["id"] for b in body}
    out = []
    ft = FEATURE[scale]
    tr = a["thermal"]
    for R in tr["regions"]:
        rid = R["id"]
        parts = region_parts(R, body, reg, scale, cfg)
        if not parts:
            parts = fallback_parts(a, scale, rid, body, cfg)
        if not parts:
            raise SystemExit(f"{a['id']} {scale}: region {rid} has no parts")
        layers = R.get("layers") or []
        f = fac.get(rid, 1.0)
        for p in parts:
            real = p["id"] not in bodyids
            big = p.get("role") == "body"
            ff = f if big else 1.0
            outer = p if (abs(ff - 1.0) < 0.02 and not real) else clamp_feature(scale_lat(p, ff), ft) if abs(ff - 1.0) >= 0.02 else copy.deepcopy(p)
            tid = "th-" + p["id"]
            if scale == "50" and len(layers) > 1:
                n = len(layers)
                for i, L in enumerate(layers):
                    if i < n - 1 and not big:
                        continue
                    off = sum((layers[j].get("thicknessM") or 0.001) for j in range(i + 1, n))
                    if i == n - 1:
                        if outer is p and not real:
                            out.append({"id": f"{tid}-{L['id']}", "of": p["id"], "region": rid, "layer": L["id"]})
                        else:
                            s = copy.deepcopy(outer)
                            s.update(id=f"{tid}-{L['id']}", region=rid, layer=L["id"])
                            out.append(s)
                    else:
                        s = clamp_feature(shrink(outer, off), ft)
                        s.update(id=f"{tid}-{L['id']}", region=rid, layer=L["id"], material=L.get("material", s["material"]))
                        out.append(s)
            else:
                lay = {"layer": layers[-1]["id"]} if (scale == "50" and len(layers) == 1) else {}
                if outer is p and not real:
                    out.append({"id": tid, "of": p["id"], "region": rid, **lay})
                else:
                    s = clamp_feature(copy.deepcopy(outer), ft)
                    s.update(id=tid, region=rid, **lay)
                    out.append(s)
    out += emitter_solids(a, scale, cfg, reg)
    out += cfg.get("hidden", lambda a, s: [])(a, scale)
    return out


def fallback_parts(a, scale, rid, body, cfg):
    """A real solid for a region that no body solid of this scale carries: copies of the main solid of the bones whose 2D shapes name the region."""
    bones = []
    for sc in SCALES:
        try:
            P = profiles(a["id"], sc)
        except Exception:
            continue
        for v in ("elevation", "front", "plan"):
            for s in P[v]:
                if s.get("region") == rid and s["bone"] not in bones:
                    bones.append(s["bone"])
    out = []
    for b in bones:
        m = main_solid(body, b)
        if m is None:
            rb = next((x for x in a["rig"]["bones"] if x["id"] == b), None)
            if rb is None:
                continue
            L = rb.get("lengthM") or 0.1
            r = max(FEATURE[scale] / 2 + 0.0015, 0.03)
            out.append(ellipsoid(f"{b}-{rid}", b, body[0]["material"], 0.6, "body", [L / 2, 0, 0], [max(L / 2, r), r, r]))
            continue
        c = copy.deepcopy(m)
        c["id"] = f"{b}-{rid}"
        out.append(c)
    return out


def anchor_of(a, aid):
    for x in a["anchors"]:
        if x["id"] == aid:
            return x
    raise SystemExit("no anchor " + aid)


def air_mat(a):
    ids = [m["id"] for m in a["materials"]]
    return "still-air" if "still-air" in ids else ids[0]


def emitter_solids(a, scale, cfg, reg):
    ft = FEATURE[scale]
    out = []
    for e in a["thermal"].get("emitters", []):
        an = anchor_of(a, e["anchor"])
        d = e["diameterM"]
        region = e.get("region") or cfg.get("emitter_region", {}).get(e["id"]) or cfg["emitter_default"]
        r = max(d / 2, ft / 2 + 0.0015)
        if e["kind"] == "plume":
            half = min(max(d / 2, ft), max(0.06, ft))
            s = ellipsoid("th-" + e["id"], an["bone"], air_mat(a), 0.9, "detail", [an["x"] - ft * 0.55, an["y"], cfg.get("anchor_w", {}).get(e["anchor"], 0.0)], [ft / 2 + 0.0015, half, half])
        else:
            s = ellipsoid("th-" + e["id"], an["bone"], air_mat(a), 0.9, "detail", [an["x"], an["y"], cfg.get("anchor_w", {}).get(e["anchor"], 0.0)], [r, r, r])
        s["region"] = region
        s["tags"] = [e["id"], "emitter"]
        out.append(s)
    return out


def tune_areas(cat, slug, src, scale, make, regions, tries=7):
    fac = {}
    limit = 0.15 if scale == "50" else 0.3
    aim = limit * 0.6
    ft = FEATURE[scale]
    for it in range(tries):
        src["volumes"][scale]["thermal"] = make(fac)
        path = f"{GEN}/tune-{slug}.json"
        json.dump(src, open(path, "w"))
        r = subprocess.run([bun(), "author-08-measure.ts", f"{cat}/{slug}", path], cwd=T, capture_output=True, text=True)
        mm = json.loads(r.stdout)[scale]
        area = mm["regionArea"]
        bb = mm["bbox"]
        out = set()
        for row in mm["thermal"]:
            if row["min"] is None:
                continue
            if any(row["min"][k] < bb["min"][k] - ft * 0.9 or row["max"][k] > bb["max"][k] + ft * 0.9 for k in range(3)):
                out.add(row["region"])
        worst = 0
        changed = False
        for R in regions:
            if R.get("areaM2") is None or R["id"] not in area:
                continue
            a0 = area[R["id"]]
            hi, lo = R["areaM2"] * (1 + aim), R["areaM2"] * (1 - aim)
            want = hi if a0 > hi else lo if a0 < lo else None
            if R["id"] in out and (want is None or want > a0):
                fac[R["id"]] = fac.get(R["id"], 1.0) * 0.96
                changed = True
                continue
            if want is not None:
                fac[R["id"]] = max(0.3, min(3.0, fac.get(R["id"], 1.0) * ((want / a0) ** 0.9)))
                changed = True
        if not changed:
            break
    return fac


def main_solid(body, bone, prefer=None):
    c = [b for b in body if b["bone"] == bone]
    if prefer:
        for b in c:
            if b["id"] in prefer:
                return b
    c = [b for b in c if b["role"] == "body"] or c
    best, bv = None, -1
    for b in c:
        k = b["kind"]
        v = 0
        if k == "loft":
            v = sum(max(s["rx"] * s["ry"], 0) for s in b["sections"]) * (b["sections"][-1]["at"][0] - b["sections"][0]["at"][0] + 0.01)
        elif k == "capsule":
            v = b["r0"] ** 2 * (math.dist(b["p0"], b["p1"]) + 0.01)
        else:
            v = 1e-6
        if v > bv:
            best, bv = b, v
    return best


def member_solid(a, scale, bone, body, k, cfg):
    ft = FEATURE[scale]
    caps = [b for b in body if b["bone"] == bone and b["role"] == "body" and not b.get("slender") or (b["bone"] == bone and b.get("slender"))]
    if bone in cfg.get("prop_bones", []) and caps:
        return [{"id": "member-" + c["id"], "of": c["id"], "member": bone, "part": "member"} for c in caps[:20]]
    m = main_solid(body, bone, cfg.get("main", {}).get(bone))
    rb = next(b for b in a["rig"]["bones"] if b["id"] == bone)
    ms = None
    if m is not None and m["kind"] == "loft":
        secs = []
        for s in m["sections"]:
            sc = copy.deepcopy(s)
            sc["rx"] = r5(max(s["rx"] * k, ft / 2 + 0.0015))
            sc["ry"] = r5(max(s["ry"] * k, ft / 2 + 0.0015))
            secs.append(sc)
        ms = {"id": "member-" + bone, "bone": bone, "material": m["material"], "role": "body", "tone": m["tone"], "kind": "loft", "sections": secs, "capStart": True, "capEnd": True}
    elif m is not None and m["kind"] == "capsule":
        r = max(m["r0"] * k, ft / 2 + 0.0015)
        ms = capsule("member-" + bone, bone, m["material"], m["tone"], "body", m["p0"], m["p1"], r, max(m["r1"] * k, r))
    else:
        L = rb.get("lengthM") or 0.2
        r = max(0.02 * k, ft / 2 + 0.0015)
        mat = (m or body[0])["material"]
        ms = capsule("member-" + bone, bone, mat, 0.5, "body", [0, 0, 0], [L, 0, 0], r)
    ms["member"] = bone
    ms["part"] = "member"
    return [ms]


def joint_solid(a, scale, bone, ms, scale_r=0.7):
    ft = FEATURE[scale]
    if ms["kind"] == "loft":
        s0 = ms["sections"][0]
        r = max(s0["rx"], s0["ry"]) * scale_r
        c = [s0["at"][0], s0["at"][1], s0["at"][2]]
    else:
        r = ms.get("r0", 0.03) * scale_r
        c = list(ms.get("p0", [0, 0, 0]))
    r = max(r, ft / 2 + 0.0015)
    j = ellipsoid("joint-" + bone, bone, ms["material"], 0.35, "detail", c, [r, r, r])
    j["member"] = bone
    j["part"] = "joint"
    return j


def mass_solid(a, scale, bone, body, cfg):
    rb = next(b for b in a["rig"]["bones"] if b["id"] == bone)
    mk = rb["mass"]["kg"]
    ft = FEATURE[scale]
    r = (3 * mk / (4 * math.pi * 1000)) ** (1 / 3)
    m = main_solid(body, bone, cfg.get("main", {}).get(bone))
    lim = 0.5 * (min(min(s["rx"], s["ry"]) for s in m["sections"]) if m is not None and m["kind"] == "loft" else r)
    if m is not None and m["kind"] == "loft":
        cx = rb["mass"]["comX"]
        best = min(m["sections"], key=lambda s: abs(s["at"][0] - cx))
        lim = 0.5 * min(best["rx"], best["ry"])
    r = max(min(r, lim), ft / 2 + 0.0015)
    s = ellipsoid("mass-" + bone, bone, m["material"] if m else "skin", 0.6, "detail", [rb["mass"]["comX"], rb["mass"].get("comY", 0), 0], [r, r, r])
    s["member"] = bone
    s["part"] = "mass"
    return s


def support_solid(a, scale, an, cfg):
    ft = FEATURE[scale]
    t = an.get("support", "contact")
    lx = max(an.get("footprintLengthM", 0.05), ft)
    ly = max(an.get("footprintWidthM", 0.05), ft)
    h = max(ft, 0.01)
    cw = cfg.get("anchor_w", {}).get(an["id"], 0.0)
    if t == "roller":
        r = max(ly / 2, ft / 2 + 0.0015)
        s = ellipsoid("support-" + an["id"], an["bone"], cfg.get("support_mat", "skin"), 0.2, "detail", [an["x"], an["y"], cw], [r, r, r])
    elif t == "pin":
        s = capsule("support-" + an["id"], an["bone"], cfg.get("support_mat", "skin"), 0.2, "detail", [an["x"] - h, an["y"], cw], [an["x"], an["y"], cw], max(ly / 2, ft / 2 + 0.0015))
    else:
        nd = an.get("normalDeg", 90) % 180
        size = [h, lx, ly] if (nd < 45 or nd > 135) else [lx, h, ly]
        s = box("support-" + an["id"], an["bone"], cfg.get("support_mat", "skin"), 0.2, "detail", [an["x"], an["y"], cw], size)
    s["member"] = an["bone"]
    s["part"] = "support"
    s["anchor"] = an["id"]
    return s
