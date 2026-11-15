import json, copy, math
from author_08_person import *
from author_08_tail import *


def seed_keep(slug, ids):
    seed = seed_of(slug)
    out, reg = [], {}
    by = {s["id"]: s for s in seed["volume"]["solids"]}
    for i in ids:
        s = copy.deepcopy(by[i])
        r = s.pop("region", None) or (s.pop("regions", None) or [None])[0]
        s.pop("regions", None)
        s.pop("maxDenominator", None)
        out.append(s)
        if r:
            reg[i] = r
    return out, reg


def draft_props(slug, scale, bones):
    d = json.load(open(f"{T}/🗑️generated/volumes-tools/draft/humans/{slug}.s{scale}.body.json"))["solids"]
    P = Prof(slug, scale)
    seed = seed_of(slug)
    out, reg = [], {}
    tubes = {}
    for s in seed["volume"]["solids"]:
        if s["bone"] in bones and s["kind"] == "capsule" and s["role"] == "body" and s.get("maxDenominator", 500) >= int(scale) and slender_ratio(s) >= 12:
            tubes.setdefault(s["bone"], []).append(s)
    for bone, lst in tubes.items():
        for s in lst:
            c = copy.deepcopy(s)
            rg = c.pop("region", None) or (c.pop("regions", None) or [None])[0]
            c.pop("regions", None); c.pop("maxDenominator", None)
            if slender_ratio(c) >= 22:
                c["slender"] = True
            out.append(c)
            if rg:
                reg[c["id"]] = rg
    for s in d:
        if s["bone"] not in bones:
            continue
        if s["bone"] in tubes and s["kind"] == "extrude":
            xs = [p[0] for p in s["profile"]]; ys = [p[1] for p in s["profile"]]
            if min(max(xs) - min(xs), max(ys) - min(ys)) < 2.5 * FEATURE[scale]:
                continue
        r = None
        for v in ("elevation", "front", "plan"):
            for sh in P.d[v]:
                if sh["bone"] == s["bone"] and sh.get("region") and (sh["id"] == s["id"] or s["id"].startswith(sh["id"])):
                    r = sh["region"]
        if r is None:
            at = seed_attr(seed, s["bone"])
            r = at[2] if at else None
        s = copy.deepcopy(s)
        for k in ("region", "regions", "maxDenominator"):
            s.pop(k, None)
        out.append(s)
        if r:
            reg[s["id"]] = r
    return out, reg


def build_scale(slug, scale, cfg):
    sol, reg = person_body(slug, scale, cfg["person"])
    k, r = seed_keep(slug, cfg.get("keep", {}).get(scale, []))
    sol += k
    reg.update(r)
    if scale != "50" and cfg.get("prop_bones"):
        ds, dr = draft_props(slug, scale, cfg["prop_bones"])
        sol += ds
        reg.update(dr)
    props = cfg["props"](scale, Prof(slug, scale))
    sol += props[0]
    reg.update(props[1])
    for pid in cfg.get("drop", {}).get(scale, []):
        sol = [s for s in sol if s["id"] != pid]
    seen = set()
    gen = {s["id"] for s in sol if s.get("_g")}
    out = []
    for s in sol:
        if s["id"] in seen:
            continue
        seen.add(s["id"])
        out.append(s)
    sol = [clamp_feature(s, FEATURE[scale]) for s in out]
    return sol, reg


def structure_for(a, scale, body, cfg, meas_members):
    ft = FEATURE[scale]
    out = []
    mk = cfg.get("member_k", {})
    for b in meas_members:
        if b in cfg.get("member_custom", {}):
            out += cfg["member_custom"][b](a, scale, body)
            continue
        out += member_solid(a, scale, b, body, mk.get(b, cfg.get("k", 0.5)), cfg)
    ms = {s["member"]: s for s in out if s["part"] == "member" and "kind" in s}
    jb = cfg.get("joints", {}).get(scale, [])
    for b in jb:
        if b in ms:
            out.append(joint_solid(a, scale, b, ms[b]))
    total = sum((x.get("mass") or {}).get("kg", 0) for x in a["rig"]["bones"])
    for b in cfg.get("masses", {}).get(scale, []):
        out.append(mass_solid(a, scale, b, body, cfg))
    sup = set(a["structure"].get("supports", []))
    for an in a["anchors"]:
        if an["id"] in sup or (an.get("kind") == "contact" and scale in cfg.get("contact_pads", [])):
            out.append(support_solid(a, scale, an, cfg))
    out += cfg.get("struct_extra", lambda a, s, body: [])(a, scale, body)
    return out


def run(slug, cfg, scales=SCALES, tune=False):
    cat = "humans"
    a = load_asset(cat, slug)
    src = load_src(cat, slug)
    meas0 = None
    for sc in scales:
        body, reg = build_scale(slug, sc, cfg)
        v = src["volumes"][sc]
        v["body"] = body
        src["volumes"][sc]["structure"] = [{"id": "tmp", "bone": body[0]["bone"], "material": body[0]["material"], "role": "body", "tone": 0.5, "kind": "ellipsoid", "radii": [0.1, 0.1, 0.1], "member": body[0]["bone"], "part": "mass"}]
        json.dump(src, open(f"{GEN}/tune-{slug}.json", "w"))
        r = subprocess.run([bun(), "author-08-measure.ts", f"{cat}/{slug}", f"{GEN}/tune-{slug}.json"], cwd=T, capture_output=True, text=True)
        members = json.loads(r.stdout)[sc]["members"]
        v["structure"] = structure_for(a, sc, body, cfg, members)
        make = lambda fac: thermal_list(a, sc, body, reg, cfg, fac)
        fac = tune_areas(cat, slug, src, sc, make, a["thermal"]["regions"]) if tune else {}
        v["thermal"] = make(fac)
        v["decisions"] = cfg["decisions"](sc, fac)
        print(sc, "body", len(body), "thermal", len(v["thermal"]), "structure", len(v["structure"]), {k: round(x, 2) for k, x in fac.items()})
    save_src(cat, slug, src)
    return src


def decisions_std(sc, fac, desc):
    ft = FEATURE[sc] * 1000
    tol = ft * 0.5
    mx = MAXN[sc]
    scaled = ", ".join(f"{k} x{v:.2f}" for k, v in sorted(fac.items()) if abs(v - 1) > 0.02) or "none"
    lay = {
        "50": ("The thermal list models every multi-layer region as nested shells, one per layer, shrunk inwards by the thickness of the layers outside it; the outer layer references the body solid where the surface is identical.", "Die Thermalliste modelliert jede mehrschichtige Region als ineinander liegende Schalen, eine je Schicht, um die Dicke der aussenliegenden Schichten nach innen verkleinert; die äusserste Schicht verweist auf den Körper, wo die Fläche identisch ist."),
    }.get(sc, ("The thermal list is one surface solid per part and region (layers are not modelled below 1:50); parts below the smallest feature of %d mm are enlarged to it." % ft, "Die Thermalliste hat einen Flächenkörper je Teil und Region (Schichten werden unterhalb 1:50 nicht modelliert); Teile unter dem kleinsten Merkmal von %d mm werden darauf vergrössert." % ft))
    en = [
        f"Body at 1:{sc}: every bone drawn at this scale is a loft whose sections are sampled from this scale's own elevation, front and plan outlines and reduced with a tolerance of {tol:.1f} mm to at most {mx} sections; {desc['en'][sc]}",
        lay[0] + f" Emitters are small solids at their anchors; region surfaces were fitted to areaM2 by scaling the solids of: {scaled}.",
        "Structure: a member solid (the load-bearing core, about half the limb section) along every load-bearing bone, joint solids at the main rig joints, mass solids at the centres of the heavier point masses and support solids at the structure.supports anchors; coarser scales simplify sections and merge joints but keep every member.",
    ]
    de = [
        f"Körper bei 1:{sc}: jeder in diesem Massstab gezeichnete Knochen ist ein Loft, dessen Schnitte aus den eigenen Seiten-, Vorder- und Draufsichtumrissen dieses Massstabs entnommen und mit {tol:.1f} mm Toleranz auf höchstens {mx} Schnitte reduziert wurden; {desc['de'][sc]}",
        lay[1] + f" Emitter sind kleine Körper an ihren Ankern; die Regionsflächen wurden durch Skalieren der Körper an areaM2 angepasst: {scaled}.",
        "Tragwerk: ein Stabkörper (tragender Kern, etwa halber Gliedquerschnitt) entlang jedes tragenden Knochens, Gelenkkörper an den wichtigen Gelenken, Massenkörper an den Schwerpunkten der schweren Punktmassen und Auflagerkörper an den Ankern von structure.supports; gröbere Massstäbe vereinfachen Querschnitte und verschmelzen Gelenke, behalten aber jeden Stab.",
    ]
    return [{"en": e, "de": d} for e, d in zip(en, de)]


def min_dim_of(s):
    k = s["kind"]
    if k == "ellipsoid":
        return 2 * min(s["radii"])
    if k == "capsule":
        return 2 * min(s["r0"], s["r1"])
    if k == "box":
        return min(s["size"])
    if k == "loft":
        return 2 * min(min(c["rx"], c["ry"]) for c in s["sections"])
    return 1.0


def slender_ratio(s):
    if s["kind"] == "capsule":
        L = math.dist(s["p0"], s["p1"]) + s["r0"] + s["r1"]
        return L / (2 * max(s["r0"], s["r1"]))
    return 0


def auto_cfg(slug, prop_bones, person_over=None, extra_keep=None, main_replace=None, exclude_ids=(), masses_props=True, k=0.45, desc=None):
    a = load_asset("humans", slug)
    seed = seed_of(slug)
    sol = seed["volume"]["solids"]
    per = lambda b: is_person(b) and b not in prop_bones
    replaced = set()
    for b in a["rig"]["bones"]:
        if not per(b["id"]):
            continue
        cand = [s for s in sol if s["bone"] == b["id"] and s["role"] == "body" and s["kind"] == "loft"]
        if cand:
            replaced.add(main_solid(cand, b["id"])["id"])
    if main_replace:
        replaced |= set(main_replace)
    auto_parts = {}
    for b in a["rig"]["bones"]:
        bid = b["id"]
        if not per(bid):
            continue
        groups = {}
        for sc in SCALES:
            try:
                PP = profiles(slug, sc)
            except Exception:
                continue
            for v in ("elevation", "front", "plan"):
                for sh in PP[v]:
                    if sh["bone"] == bid and sh["role"] == "body" and sh.get("region") and (sh["filled"] or v == "elevation"):
                        groups.setdefault(sh["region"], set()).add(sh["id"])
        if len(groups) < 2:
            continue
        at = seed_attr(seed, bid)
        main_r = at[2] if at else None
        if main_r not in groups:
            continue
        parts = []
        for r, ids in groups.items():
            ss = [s for s in sol if s["bone"] == bid and (s.get("region") or (s.get("regions") or [None])[0]) == r and s["role"] == "body"]
            if r != main_r and not ss:
                continue
            mat, tone = (ss[0]["material"], ss[0]["tone"]) if ss else (at[0], at[1])
            parts.append(dict(id=bid if r == main_r else f"{bid}-{r}", mat=mat, tone=tone, region=r, include=sorted(ids), maxn=None if r == main_r else 6))
            if r != main_r:
                for s in ss:
                    if s["kind"] == "loft":
                        replaced.add(s["id"])
        if len(parts) > 1:
            auto_parts[bid] = parts
    keep = {}
    for sc in SCALES:
        ids = []
        for s in sol:
            if s["id"] in replaced or s["id"] in exclude_ids:
                continue
            md = s.get("maxDenominator", 500)
            if md < int(sc):
                continue
            if s["bone"] in prop_bones and sc != "50":
                continue
            if s["bone"] not in prop_bones and sc != "50":
                if min_dim_of(s) < FEATURE[sc] * 0.9 and s["role"] == "detail":
                    continue
            ids.append(s["id"])
        keep[sc] = ids
    for sc, ids in (extra_keep or {}).items():
        keep[sc] = keep[sc] + ids
    heavy = [b["id"] for b in a["rig"]["bones"] if per(b["id"]) and (b.get("mass") or {}).get("kg", 0) >= 7]
    props_m = [b["id"] for b in a["rig"]["bones"] if b["id"] in prop_bones and (b.get("mass") or {}).get("kg", 0) >= 1] if masses_props else []
    jb = [b["id"] for b in a["rig"]["bones"] if per(b["id"]) and any(b["id"].startswith(p) for p in ("arm", "upper-arm", "forearm", "thigh", "shank"))]
    head_reg = (seed_attr(seed, "head") or (0, 0, "face-skin"))[2]
    cfg = {
        "person": {"is_person": per, "parts": {**auto_parts, **(person_over or {})}},
        "keep": keep,
        "props": lambda scale, P: ([], {}),
        "extra": {},
        "emitter_default": head_reg,
        "k": k,
        "member_k": {"torso": 0.5, "head": 0.5},
        "joints": {"50": jb + ["torso", "head"], "100": jb, "200": [], "500": []},
        "masses": {"50": heavy + props_m, "100": heavy + props_m, "200": [h for h in heavy if h.startswith(("torso", "thigh"))] + props_m, "500": ["torso"] + props_m},
        "contact_pads": [],
        "support_mat": a["materials"][0]["id"],
        "main": {},
        "prop_bones": prop_bones,
        "decisions": lambda sc, fac: decisions_std(sc, fac, desc),
    }
    return cfg
