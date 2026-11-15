"""Author 33 helpers: load/save a scale of a humans source, solid builders. Run from T."""
import json, copy, os, subprocess, glob, math
T = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
def path(slug): return f"{T}/volumes/humans/{slug}.json"
def load(slug): return json.load(open(path(slug)))
def save(slug, d):
    tmp = path(slug) + ".a33.tmp"; json.dump(d, open(tmp, "w")); os.replace(tmp, path(slug))
def bun(*a, quiet=True):
    p = subprocess.run([BUN, *a], cwd=T, capture_output=True, text=True); out = p.stdout + p.stderr
    return "\n".join(l for l in out.splitlines() if not l.startswith("[check]") and not l.startswith("absorbed"))
def sec(at, rx, ry, e=2.2, **k): return {"at": [at, 0, 0], "rx": rx, "ry": ry, "exponent": e, **k}
def loft(id, bone, material, role, secs, tone=0.5, **k):
    return {"id": id, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": secs, "capStart": True, "capEnd": True, **k}
def interp(s, u, key):
    ss = s["sections"]
    for a, b in zip(ss, ss[1:]):
        if a["at"][0] <= u <= b["at"][0]:
            t = (u - a["at"][0]) / max(1e-9, b["at"][0] - a["at"][0]); return a[key] + t * (b[key] - a[key])
    return ss[0][key] if u < ss[0]["at"][0] else ss[-1][key]
def by(solids, id): return next(s for s in solids if s["id"] == id)
D = lambda en, de: {"en": en, "de": de}

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
def primary(body, bone):
    mine = [s for s in body if s["bone"] == bone and s.get("role", "body") == "body" and not s.get("slender") and s["kind"] == "loft"]
    span = lambda s: abs(s["sections"][-1]["at"][0] - s["sections"][0]["at"][0])
    return max(mine, key=span) if mine else None
def path_at(s, u):
    ss = s["sections"]; out = [u, 0, 0]
    for k in (1, 2):
        for a, b in zip(ss, ss[1:]):
            if a["at"][0] <= u <= b["at"][0]:
                t = (u - a["at"][0]) / max(1e-9, b["at"][0] - a["at"][0]); out[k] = a["at"][k] + t * (b["at"][k] - a["at"][k]); break
        else: out[k] = ss[0]["at"][k] if u < ss[0]["at"][0] else ss[-1]["at"][k]
    return out
def ball(id, bone, member, part, centre, radii, material="skin", tone=0.4):
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": "body" if part != "mass" else "detail", "tone": tone, "radii": radii, "transform": {"translation": centre}, "member": member, "part": part}
def person_structure(slug, scale, joints=True, joint_k=0.9, skip_joints=(), mass_k=0.75):
    """kit members + hand-placed joint balls on the loft path of each child bone + compact mass ellipsoids on the path at the centre of mass."""
    out = bun("kit-structure.ts", f"humans/{slug}", "--scale", str(scale), "--no-joints", "--write")
    d = load(slug); v = d["volumes"][str(scale)]; body = v["body"]; f = FEATURE[scale]
    kit = [s for s in v["structure"]]
    members = [s for s in kit if s["part"] == "member"]
    CORE = {"torso": 0.72, "pelvis": 0.72, "head": 0.8}
    nsec = {50: 4, 100: 3}.get(scale, 2)
    bl = {b["id"]: b for b in d_asset(slug)["rig"]["bones"]}
    mine = []
    for m in members:
        bid = m["bone"]; p = primary(body, bid)
        if p is None: mine.append(m); continue
        L = bl[bid]["lengthM"]; k = next((v for kk, v in CORE.items() if bid.startswith(kk)), 0.7)
        secs = []
        for i in range(nsec):
            u0, u1 = max(0, p["sections"][0]["at"][0]), min(L, p["sections"][-1]["at"][0]); u = u0 + (u1 - u0) * i / (nsec - 1); uu = min(max(u, u0 + 0.06 * (u1 - u0)), u1 - 0.06 * (u1 - u0))
            pa = path_at(p, uu); pa[0] = u
            secs.append({"at": pa, "rx": max(0.55 * f, k * interp(p, uu, "rx")), "ry": max(0.55 * f, k * interp(p, uu, "ry")), "exponent": min(p["sections"][0].get("exponent", 2), 3), "rotation": p["sections"][0].get("rotation")})
            if secs[-1]["rotation"] is None: del secs[-1]["rotation"]
        mine.append({**{kk: vv for kk, vv in m.items() if kk not in ("sections",)}, "sections": secs})
    members = mine
    masses = [s for s in kit if s["part"] == "mass"]
    bones = {b["id"]: b for b in d_asset(slug)["rig"]["bones"]}
    new = list(members)
    if joints and scale <= 100:
        for b in bones.values():
            if not b.get("parent") or b["id"] in skip_joints: continue
            p = primary(body, b["id"])
            if p is None: continue
            L = b["lengthM"]; u = 0.04 * L
            rx, ry = interp(p, u, "rx"), interp(p, u, "ry")
            r = max(0.55 * f, joint_k * min(rx, ry) * 0.8)
            new.append(ball(f"joint-{b['id']}", b["id"], b["id"], "joint", path_at(p, u + r * 0.3), [r, r, r]))
    for m in masses:
        bid = m["bone"]; p = primary(body, bid)
        if p is None: new.append(m); continue
        u = m["transform"]["translation"][0] if scale <= 100 else m["transform"]["translation"][0]
        L = bones[bid]["lengthM"]
        rx, ry = interp(p, u, "rx") * mass_k * 0.7, interp(p, u, "ry") * mass_k * 0.7
        if scale >= 500 and bid == "pelvis": continue
        rr = [max(0.55 * f, 0.18 * L), max(0.55 * f, ry), max(0.55 * f, rx)]
        if scale >= 200:
            new.append({"id": f"mass-{bid}", "kind": "box", "bone": bid, "material": "skin", "role": "detail", "tone": 0.2, "size": [2 * x for x in rr], "transform": {"translation": path_at(p, u)}, "member": bid, "part": "mass"})
        else:
            new.append(ball(f"mass-{bid}", bid, bid, "mass", path_at(p, u), rr, tone=0.2))
    v["structure"] = new; save(slug, d)
    return out
def d_asset(slug):
    import glob
    p = glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{slug}/🔣️.json")[0]
    return json.load(open(p))["asset"]

def coarsen(slug, frm, to, absorb=0.995, drop=()):
    out = bun("kit-coarsen.ts", f"humans/{slug}", "--from", str(frm), "--to", str(to), "--absorb", str(absorb), "--apply", "--force")
    d = load(slug); v = d["volumes"][str(to)]
    v["body"] = [s for s in v["body"] if s["id"] not in drop]; save(slug, d)
    return [l for l in out.splitlines() if l.startswith("kit-coarsen")]
def thermal(slug, n, tmap=""):
    a = ["kit-thermal.ts", f"humans/{slug}", "--scale", str(n), "--write"] + (["--map", tmap] if tmap else [])
    return bun(*a)
def check(slug, n, rules=None):
    a = ["volumes-check.ts", f"humans/{slug}", "--scale", str(n), "--quiet"] + (["--rules", rules] if rules else [])
    return "\n".join(l[:900] for l in bun(*a).splitlines() if not l.startswith("ok "))

def hull(slug, scale, bone, id, material, n=9, **o):
    a = ["author-33-hull.ts", f"humans/{slug}", "--scale", str(scale), "--bone", bone, "--id", id, "--material", material, "--n", str(n)]
    for k, v in o.items(): a += [f"--{k}", str(v)]
    p = subprocess.run([BUN, *a], cwd=T, capture_output=True, text=True)
    line = [l for l in p.stdout.splitlines() if l.startswith("{")]
    if not line: raise RuntimeError(p.stdout + p.stderr)
    return json.loads(line[-1])

def shell_loft(s, t, id):
    """Loft inset by t on the cross-section; sections thinner than 2.2 t keep 45 % of their radius (tapered caps)."""
    n = copy.deepcopy(s); n["id"] = id
    for k in ("of", "region", "layer", "tags"): n.pop(k, None)
    for q in n["sections"]:
        for r in ("rx", "ry"):
            q[r] = q[r] - t if q[r] > 2.2 * t else max(q[r] * 0.45, 0.0006)
    return n
def prune_and_rebuild_shells(slug, scale, detail_ids, drop_refs=()):
    """After kit-thermal: drop layer shells of detail solids, rebuild loft shells with the real cumulative thickness, keep references."""
    d = load(slug); v = d["volumes"][str(scale)]; body = {s["id"]: s for s in v["body"]}; A = d_asset(slug)
    regs = {r["id"]: r for r in A["thermal"]["regions"]}
    out = []
    for s in v["thermal"]:
        if "of" in s:
            if s["of"] in drop_refs: continue
            out.append(s); continue
        if not s.get("layer"): out.append(s); continue
        # shell id th-<sid>-<layer>
        r = regs[s["region"]]; layers = [l["id"] for l in r["layers"]]; i = layers.index(s["layer"])
        sid = s["id"][3:-(len(s["layer"]) + 1)]
        if sid in detail_ids: continue
        b = body.get(sid)
        if b is None: out.append(s); continue
        cum = sum(l.get("thicknessM", 0.002) for l in r["layers"][i + 1:])
        if b["kind"] == "loft":
            n = shell_loft(b, cum, s["id"]); n.update(region=s["region"], layer=s["layer"], role="body", tone=s.get("tone", 0.4)); out.append(n)
        else: out.append(s)
    v["thermal"] = out; save(slug, d)
    return len(out)
