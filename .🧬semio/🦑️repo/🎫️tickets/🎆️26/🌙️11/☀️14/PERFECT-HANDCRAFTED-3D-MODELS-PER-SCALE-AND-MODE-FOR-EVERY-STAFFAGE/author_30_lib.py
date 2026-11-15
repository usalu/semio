import json, math, os, subprocess, sys, copy, glob
from author_08_lib import T, SCALES, FEATURE, bun, r5, load_asset, loft, capsule, ellipsoid, box, shrink, clamp_feature, rotations, quat_basis, norm, sub, add, mul, dot, cross
GEN = os.path.join(T, "🗑️generated", "author-30")
os.makedirs(GEN, exist_ok=True)


def src_file(cat, slug):
    return os.path.join(T, "volumes", cat, slug + ".json")


def load(cat, slug):
    return json.load(open(src_file(cat, slug)))


def save_lists(cat, slug, scale, **lists):
    d = load(cat, slug)
    v = d["volumes"][str(scale)]
    for k, l in lists.items():
        v[k] = l
    p = src_file(cat, slug)
    tmp = p + ".a30.tmp"
    json.dump(d, open(tmp, "w"), ensure_ascii=False, separators=(",", ":"))
    os.replace(tmp, p)


def run_ts(script, *args):
    r = subprocess.run([bun(), script, *args], cwd=T, capture_output=True, text=True)
    return r.stdout + r.stderr


def check(cat, slug, scale):
    cmd = [bun(), "volumes-check.ts", f"{cat}/{slug}", "--json"] + (["--scale", str(scale)] if scale else [])
    out = os.path.join(GEN, f"chk-{slug}.json")
    with open(out, "w") as fh:
        subprocess.run(cmd, cwd=T, stdout=fh, stderr=subprocess.DEVNULL, text=True)
    return json.load(open(out))


def fails(cat, slug, scale, width=330):
    r = check(cat, slug, scale)
    by = {}
    for m in r["measurements"]:
        if m["ok"] or m["name"].startswith("info"):
            continue
        by.setdefault(m["rule"].replace("volumes-", ""), []).append(f"{m['name']}={round(m['value'], 3)}/{round(m['limit'], 3) if m['limit'] < 1e6 else ''}" + (f"[{m['note'][:70]}]" if m.get("note") else ""))
    n = sum(1 for m in r["measurements"] if not m["ok"])
    out = [f"{cat}/{slug} 1:{scale} fails {n}"]
    for k, v in by.items():
        out.append("  " + k + " " + "; ".join(v)[:width * 3])
    return "\n".join(out)


def sec(u, rx, ry=None, e=2.0, at=None, rot=None):
    s = {"at": at or [r5(u), 0.0, 0.0], "rx": r5(rx), "ry": r5(rx if ry is None else ry), "exponent": e}
    if rot:
        s["rotation"] = rot
    return s


def open_loft(sid, bone, mat, tone, role, sections, cap_start=False, cap_end=True, **extra):
    d = {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone, "kind": "loft", "sections": sections, "capStart": cap_start, "capEnd": cap_end}
    d.update(extra)
    return d


def revolve(sid, bone, mat, tone, role, profile, axis=(1, 0, 0), origin=(0, 0, 0), **extra):
    d = {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone, "kind": "revolve", "profile": [[r5(a), r5(b)] for a, b in profile], "axis": list(axis), "origin": list(origin)}
    d.update(extra)
    return d


def circle_profile(ax, rad, r, n=10):
    return [[ax + r * math.sin(2 * math.pi * i / n), rad + r * math.cos(2 * math.pi * i / n)] for i in range(n)]


def station_loft(sid, bone, mat, tone, role, st, e=2.3, caps=(True, True), vshift=0.0, **extra):
    secs = []
    q = quat_basis([0, 0, 1], [0, -1, 0], [1, 0, 0])
    for u, lo, hi, hw in st:
        secs.append({"at": [r5(u), r5((lo + hi) / 2 - vshift), 0.0], "rx": r5(hw), "ry": r5((hi - lo) / 2), "exponent": e, "rotation": q})
    return {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone, "kind": "loft", "sections": secs, "capStart": caps[0], "capEnd": caps[1], **extra}


def shoe_pair(mat, tone, scale=50, side=("l", "r"), clip=None, prefix=""):
    out = []
    foot = {50: [(-0.060, 0.004, 0.1485, 0.040), (-0.050, 0.0, 0.2004, 0.047), (-0.015, 0.0, 0.2004, 0.050), (0.030, 0.0, 0.2004, 0.052), (0.0508, 0.0, 0.2004, 0.053), (0.056, 0.0, 0.1222, 0.054), (0.086, 0.0, 0.0954, 0.055), (0.127, 0.0, 0.0679, 0.055)],
            100: [(-0.062, 0.006, 0.075, 0.042), (-0.052, 0.0, 0.1103, 0.048), (0.032, 0.0, 0.1103, 0.052), (0.127, 0.0, 0.0637, 0.055)],
            200: [(-0.062, 0.006, 0.075, 0.042), (-0.052, 0.0, 0.1103, 0.048), (0.052, 0.0, 0.101, 0.052), (0.127, 0.0, 0.0637, 0.055)],
            500: [(-0.062, 0.01, 0.075, 0.040), (-0.05, 0.0, 0.1103, 0.048), (0.127, 0.0, 0.0637, 0.054)]}[scale]
    toe = {50: [(0.0, 0.0, 0.0679, 0.055), (0.0391, 0.0, 0.045, 0.052), (0.0624, 0.0, 0.0356, 0.046), (0.0803, 0.0, 0.0274, 0.038), (0.0918, 0.0, 0.0192, 0.028), (0.0958, 0.0, 0.0108, 0.012)],
           100: [(0.0, 0.0, 0.0637, 0.055), (0.0554, 0.0, 0.0385, 0.046), (0.0905, 0.0, 0.0173, 0.03), (0.0955, 0.0, 0.01, 0.015)],
           200: [(0.0, 0.0, 0.0637, 0.055), (0.0574, 0.0, 0.0385, 0.046), (0.0905, 0.0, 0.0173, 0.03)],
           500: [(0.0, 0.0, 0.076, 0.054), (0.0905, 0.0, 0.03, 0.034)]}[scale]
    if clip is not None:
        foot = [(u, lo, min(hi, clip), hw) for u, lo, hi, hw in foot]
    for s in side:
        out.append(station_loft(f"{prefix}foot-{s}", f"foot-{s}", mat, tone, "body", foot, vshift=0.0782))
        out.append(station_loft(f"{prefix}toe-{s}", f"toe-{s}", mat, tone, "body", toe, vshift=0.0))
    return out


def nested_real(a, scale, rid, solids, N=True):
    """Real thermal solids for region rid: the outer layer is the solid, inner layers are shrunk copies (1:50 only)."""
    R = next(r for r in a["thermal"]["regions"] if r["id"] == rid)
    layers = R.get("layers") or []
    out = []
    for s in solids:
        n = len(layers)
        if scale == 50 and n > 1:
            for i, Lr in enumerate(layers):
                off = sum((layers[j].get("thicknessM") or 0.001) for j in range(i + 1, n))
                c = clamp_feature(shrink(s, off), FEATURE["50"]) if off > 0 else copy.deepcopy(s)
                c.update(id=f"th-{s['id']}-{Lr['id']}", region=rid, layer=Lr["id"])
                out.append(c)
        else:
            c = copy.deepcopy(s)
            c.update(id=f"th-{s['id']}", region=rid)
            if scale == 50 and n == 1:
                c["layer"] = layers[0]["id"]
            out.append(c)
    return out


def air_material(a):
    ids = [m["id"] for m in a["materials"]]
    return "still-air" if "still-air" in ids else ids[0]


def anchor_of(a, aid):
    return next(x for x in a["anchors"] if x["id"] == aid)


def emitters(a, scale, w_of=None, region_default=None, plume_dx=0.0):
    ft = FEATURE[str(scale)]
    out = []
    for e in a["thermal"].get("emitters", []):
        an = anchor_of(a, e["anchor"])
        d = e["diameterM"]
        region = e.get("region") or (region_default.get("toddler") if isinstance(region_default, dict) and e["id"].startswith("toddler") else (region_default.get("parent") if isinstance(region_default, dict) else region_default))
        r = max(d / 2, ft / 2 + 0.0015)
        w = (w_of or {}).get(e["anchor"], 0.0)
        if e["kind"] == "plume":
            half = min(max(d / 2, ft), max(0.06, ft))
            s = ellipsoid("th-" + e["id"], an["bone"], air_material(a), 0.9, "detail", [an["x"] - ft * 0.55 + plume_dx, an["y"], w], [ft / 2 + 0.0015, half, half])
        else:
            s = ellipsoid("th-" + e["id"], an["bone"], air_material(a), 0.9, "detail", [an["x"], an["y"], w], [r, r, r])
        s["region"] = region
        s["tags"] = [e["id"], "emitter"]
        out.append(s)
    return out


def thermal_from_map(a, scale, body, regmap, extra=None, hidden=None):
    """regmap: region id -> list of (body solid id, 'nest' | 'outer'); extra: list of real solids already carrying region (and layer) ids."""
    byid = {b["id"]: b for b in body}
    out = []
    ft = FEATURE[str(scale)]
    for R in a["thermal"]["regions"]:
        rid = R["id"]
        layers = R.get("layers") or []
        for sid, mode in regmap.get(rid, []):
            s = byid[sid]
            if scale == 50 and len(layers) > 1:
                n = len(layers)
                for i, Lr in enumerate(layers):
                    if i < n - 1 and mode != "nest":
                        continue
                    if i == n - 1:
                        out.append({"id": f"th-{sid}-{Lr['id']}", "of": sid, "region": rid, "layer": Lr["id"], **({"sectionFaces": s["sectionFaces"]} if s.get("sectionFaces") else {})})
                    else:
                        off = sum((layers[j].get("thicknessM") or 0.001) for j in range(i + 1, n))
                        c = clamp_feature(shrink(s, off), ft)
                        c.update(id=f"th-{sid}-{Lr['id']}", region=rid, layer=Lr["id"], material=Lr.get("material", s["material"]))
                        out.append(c)
            else:
                t = {"id": f"th-{sid}", "of": sid, "region": rid}
                if s.get("sectionFaces"):
                    t["sectionFaces"] = s["sectionFaces"]
                if scale == 50 and len(layers) == 1:
                    t["layer"] = layers[0]["id"]
                out.append(t)
    out += (extra or [])
    return out


from author_08_lib import span_at, urange, simplify, rdp

Q_U = quat_basis([0, 0, 1], [0, -1, 0], [1, 0, 0])


class Prof30:
    def __init__(self, slug, scale, pose=False):
        self.d = json.load(open(os.path.join(GEN, f"{slug}.{'pp' if pose else 'p'}{scale}.json")))
        self.f = self.d["bones"]

    def shapes(self, view, bone, include=None, exclude=(), roles=("body",), filled_only=True):
        out = []
        for s in self.d[view]:
            if s["bone"] != bone or s["role"] not in roles or s["id"] in exclude:
                continue
            if include is not None and s["id"] not in include:
                continue
            if filled_only and not (s["filled"] or s["closed"]):
                continue
            out.append(s)
        return out

    def local(self, view, bone, p):
        f = self.f[bone]
        o, u, v, w = f["o"], f["u"], f["v"], f["w"]
        P = {"elevation": [p[0], p[1], o[2]], "front": [o[0], p[1], p[0]], "plan": [p[0], o[1], p[1]]}[view]
        d = sub(P, o)
        return dot(d, u), dot(d, v), dot(d, w)

    def rings(self, view, bone, **kw):
        return [([self.local(view, bone, p) for p in ring], True) for s in self.shapes(view, bone, **kw) for ring in s["rings"]]


def rows_of(P, bone, inc_el=None, inc_w=None, wview=None, nst=48, trim=0.0, exc=(), uspan=None, wpad=0.0, wfallback=True, wclip=None, wcarry=False, wonly=False, stroke_half=None):
    el = P.rings("elevation", bone, include=inc_el, exclude=exc)
    if not el and stroke_half is not None:
        el = P.rings("elevation", bone, include=inc_el, exclude=exc, filled_only=False)
    only_w = False
    if not el:
        f0 = P.f[bone]
        wv0 = wview or ("front" if abs(f0["u"][1]) >= 0.5 else "plan")
        el = P.rings(wv0, bone, include=inc_w, exclude=exc)
        only_w = bool(el)
    if not el:
        return None
    a0, a1 = urange(el)
    if uspan:
        a0, a1 = max(a0, uspan[0]), min(a1, uspan[1])
    f = P.f[bone]
    wv = wview or ("front" if abs(f["u"][1]) >= 0.5 else "plan")
    wr = P.rings(wv, bone, include=inc_w, exclude=exc)
    if not wr and stroke_half is not None:
        wr = P.rings(wv, bone, include=inc_w, exclude=exc, filled_only=False)
    if not wr and wfallback:
        wr = P.rings("plan" if wv == "front" else "front", bone, include=inc_w, exclude=exc)
    rows = []
    for i in range(nst):
        a = a0 + trim + (a1 - a0 - 2 * trim) * i / (nst - 1)
        sv = span_at(el, a, 2 if only_w else 1)
        if sv is None:
            continue
        if only_w:
            c0, h0 = (sv[0] + sv[1]) / 2, (sv[1] - sv[0]) / 2
            sv = (-h0, h0)
        sw = span_at(wr, a, 2) if wr else None
        if sw is None and wcarry and wr:
            sw = "carry"
        if sw is None:
            h = (sv[1] - sv[0]) / 2
            sw = (-h, h)
        if wclip is not None:
            sw = (max(sw[0], -wclip), min(sw[1], wclip))
        if stroke_half is not None:
            sv = (min(sv[0], (sv[0] + sv[1]) / 2 - stroke_half), max(sv[1], (sv[0] + sv[1]) / 2 + stroke_half))
        rows.append([a, (sv[0] + sv[1]) / 2, sw if sw == "carry" else (sw[0] + sw[1]) / 2, "carry" if sw == "carry" else (sw[1] - sw[0]) / 2 + wpad, (sv[1] - sv[0]) / 2])
    if rows and any(r[2] == "carry" for r in rows):
        known = [r for r in rows if r[2] != "carry"]
        if not known:
            return None
        for i, r in enumerate(rows):
            if r[2] == "carry":
                nb = min(known, key=lambda k: abs(k[0] - r[0]))
                r[2], r[3] = nb[2], nb[3]
    return rows or None


def loft_rows(sid, bone, mat, tone, role, rows, tol, maxn, e=2.3, caps=(True, True), minr=0.0, pad=0.0, ends=None, **extra):
    st = simplify(rows, tol, maxn)
    if len(st) == 1:
        st = [st[0], st[0]]
    secs = []
    for a, vc, wc, rx, ry in st:
        secs.append({"at": [r5(a), r5(vc), r5(wc)], "rx": r5(max(rx + pad, minr)), "ry": r5(max(ry + pad, minr)), "exponent": e, "rotation": Q_U})
    return {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone, "kind": "loft", "sections": secs, "capStart": caps[0], "capEnd": caps[1], **extra}


from author_08_tail import member_solid, joint_solid, mass_solid, main_solid


def member_bones(cat, slug, scale):
    out = run_ts("author-08-measure.ts", f"{cat}/{slug}")
    return json.loads(out)[str(scale)]["members"]


def person_structure(a, scale, body, members, prop_bones, main, k=0.45, kk=None, joints=(), masses=(), mass_cfg=None):
    sc = str(scale)
    cfg = {"prop_bones": list(prop_bones), "main": main}
    out = []
    for b in members:
        if b in prop_bones:
            continue
        out += member_solid(a, sc, b, body, (kk or {}).get(b, k), cfg)
    ms = {s["member"]: s for s in out if s["part"] == "member" and "kind" in s}
    for b in joints:
        if b in ms:
            out.append(joint_solid(a, sc, b, ms[b], 0.55))
    for b in masses:
        out.append(mass_solid(a, sc, b, body, cfg))
    return out


def areas(cat, slug, scale, mod=None, rules="volumes-thermal-coverage", names=None):
    d = load(cat, slug)
    if mod:
        mod(d["volumes"][str(scale)])
    tmp = os.path.join(GEN, f"tmp-{slug}.json")
    json.dump(d, open(tmp, "w"), ensure_ascii=False, separators=(",", ":"))
    r = subprocess.run([bun(), "volumes-check.ts", f"{cat}/{slug}", "--json", "--scale", str(scale), "--rules", rules, "--source", tmp], cwd=T, capture_output=True, text=True)
    out = {}
    a = load_asset(cat, slug)
    decl = {x["id"]: x["areaM2"] for x in a["thermal"]["regions"]}
    for m in json.loads(r.stdout)["measurements"]:
        if m["name"].startswith("areaDeviation:"):
            rid = m["name"].split(":")[1]
            if rid == "Total":
                continue
            out[rid] = round(decl[rid] * (1 + m["value"]) , 4) if False else m["value"]
    return out


def inflate(s, pad):
    c = copy.deepcopy(s)
    k = c["kind"]
    if k == "loft":
        for q in c["sections"]:
            q["rx"] = r5(q["rx"] + pad)
            q["ry"] = r5(q["ry"] + pad)
    elif k == "ellipsoid":
        c["radii"] = [r5(x + pad) for x in c["radii"]]
    elif k == "capsule":
        c["r0"] = r5(c["r0"] + pad)
        c["r1"] = r5(c["r1"] + pad)
    return c


def full(cat, slug):
    r = check(cat, slug, None)
    by = {}
    for m in r["measurements"]:
        if m["ok"] or m["name"].startswith("info"):
            continue
        by.setdefault((m.get("scale"), m["rule"].replace("volumes-", "")), []).append(f"{m['name']}={round(m['value'], 3)}/{round(m['limit'], 3) if m['limit'] < 1e6 else ''}")
    lines = [f"{k[0]} {k[1]}: " + "; ".join(v) for k, v in sorted(by.items(), key=lambda x: str(x[0]))]
    return sum(1 for m in r["measurements"] if not m["ok"]), lines


def frames_of(slug):
    return json.load(open(os.path.join(GEN, f"{slug}.frames.json")))


def to_local(fr, bone, p):
    f = fr[bone]
    d = sub(p, f["o"])
    return [r5(dot(d, f["u"])), r5(dot(d, f["v"])), r5(dot(d, f["w"]))]


def missing_regions(cat, slug, scale):
    a = load_asset(cat, slug)
    d = load(cat, slug)["volumes"][str(scale)]
    have = {t.get("region") for t in d["thermal"]}
    return [r["id"] for r in a["thermal"]["regions"] if r["id"] not in have]
