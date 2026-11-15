"""Author 55 helpers: 2D inspection, prop builders, post-kit steps. Run from T."""
import json, os, sys, math
from author_33_lib import *
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
def asset(slug): return json.load(open(f"{CAT}{slug}/🔣️.json"))["asset"]
def shapes(slug, scale, bones=None, view=None, regions=None):
    g = asset(slug)["representations"][str(scale)]["geometry"]; out = []
    for vw, lst in g.items():
        if view and vw != view: continue
        for s in lst:
            if bones and s.get("bone") not in bones: continue
            if regions and s.get("region") not in regions: continue
            out.append((vw, s))
    return out
def bbox_d(d):
    import re
    nums = [float(x) for x in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", d)]
    xs, ys = nums[0::2], nums[1::2]
    return min(xs), max(xs), min(ys), max(ys)
def describe(slug, scale, bones=None, regions=None, view=None):
    for vw, s in shapes(slug, scale, bones, view, regions):
        k = s.get("kind")
        if k == "path": b = bbox_d(s["d"]); ex = [round(x, 3) for x in b]
        else: ex = {kk: (round(v, 3) if isinstance(v, float) else v) for kk, v in s.items() if kk in ("cx", "cy", "rx", "ry", "r", "x", "y", "w", "h", "x1", "y1", "x2", "y2")}
        print(vw[:4], s["id"], s.get("bone"), s.get("role"), "f" if s.get("filled") else "o", s.get("region"), k, ex)
def strip(v, bones=(), regions=(), ids=()):
    for lst in ("body", "thermal", "structure"):
        v[lst] = [s for s in v[lst] if s["id"] not in ids and not (s.get("bone") in bones) and not (lst == "thermal" and s.get("region") in regions and "of" not in s or False)]
def skinfix(v, mat):
    for lst in ("body", "structure", "thermal"):
        for s in v[lst]:
            if s.get("material") == "skin": s["material"] = mat

def S(kind, id, bone, material, role="body", tone=0.5, **k): return {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **k}
def cap(id, bone, material, p0, p1, r0, r1=None, role="body", tone=0.5, **k): return S("capsule", id, bone, material, role, tone, p0=list(p0), p1=list(p1), r0=r0, r1=r0 if r1 is None else r1, **k)
def ell(id, bone, material, at, radii, role="body", tone=0.5, rot=None, **k):
    t = {"translation": list(at)}
    if rot: t["rotation"] = rot
    return S("ellipsoid", id, bone, material, role, tone, radii=list(radii), transform=t, **k)
def box(id, bone, material, at, size, role="body", tone=0.5, rot=None, corner=None, **k):
    t = {"translation": list(at)}
    if rot: t["rotation"] = rot
    return S("box", id, bone, material, role, tone, size=list(size), transform=t, **({"cornerRadius": corner} if corner else {}), **k)
def lofts(id, bone, material, stations, role="body", tone=0.5, e=2.0, caps=True, **k):
    """stations: [(u, rx, ry)] or [(u, rx, ry, v, w)] along bone +u; rx lateral (w), ry sagittal (v) for a path along u."""
    secs = []
    for q in stations:
        u, rx, ry = q[:3]; v = q[3] if len(q) > 3 else 0; w = q[4] if len(q) > 4 else 0
        secs.append({"at": [u, v, w], "rx": rx, "ry": ry, "exponent": e})
    return S("loft", id, bone, material, role, tone, sections=secs, capStart=caps, capEnd=caps, **k)
def revolve(id, bone, material, profile, axis="u", origin=(0, 0, 0), role="body", tone=0.5, rot=None, at=None, **k):
    s = S("revolve", id, bone, material, role, tone, profile=[list(p) for p in profile], axis={"u": [1, 0, 0], "v": [0, 1, 0], "w": [0, 0, 1]}[axis], origin=list(origin), **k)
    t = {}
    if at: t["translation"] = list(at)
    if rot: t["rotation"] = rot
    if t: s["transform"] = t
    return s
def extrude(id, bone, material, plane, profile, a, b, role="body", tone=0.5, **k):
    return S("extrude", id, bone, material, role, tone, plane=plane, profile=[list(p) for p in profile], **{"from": a, "to": b}, **k)
def quat_axis(axis, deg):
    h = math.radians(deg) / 2; s = math.sin(h); x, y, z = axis; n = math.sqrt(x*x+y*y+z*z); return [x/n*s, y/n*s, z/n*s, math.cos(h)]

def replace_props(v, bones, regions, body, thermal, structure, keep_thermal_ids=()):
    """drop old prop solids (body/structure by bone, thermal by region or of-reference) and append the new ones."""
    ids = {x["id"] for x in v["body"] if x.get("bone") in bones}
    v["body"] = [x for x in v["body"] if x.get("bone") not in bones] + body
    v["thermal"] = [x for x in v["thermal"] if not (x.get("region") in regions and not x.get("tags")) and x.get("of") not in ids] + thermal
    v["structure"] = [x for x in v["structure"] if x.get("bone") not in bones and x.get("member") not in bones and x.get("of") not in ids] + structure
def th(of, region, **k): return {"id": f"th-{of}", "of": of, "region": region, **k}
def add_decisions(v, en, de): v["decisions"] = [x for x in v["decisions"] if not x["en"].startswith("[props]")] + [{"en": "[props] " + en, "de": "[Requisiten] " + de}]
def run_build(slug):
    return bun("figure-build.ts", f"humans/{slug}", "--write")

BK = f"{T}/🗑️generated/author-55/backup"
def restore_bones(slug, d, bones):
    """copy the body solids of `bones` and their thermal solids from the author-08 backup (bones the kit was told to skip)."""
    old = json.load(open(f"{BK}/{slug}.json"))["volumes"]
    for n in ("50", "100", "200", "500"):
        o, v = old[n], d["volumes"][n]
        ids = {s["id"] for s in o["body"] if s["bone"] in bones}
        have = {s["id"] for s in v["body"]}
        v["body"] += [s for s in o["body"] if s["id"] in ids and s["id"] not in have]
        th = {s["id"] for s in v["thermal"]}
        v["thermal"] += [s for s in o["thermal"] if (s.get("bone") in bones or s.get("of") in ids) and s["id"] not in th]

def fit(slug, scale, props_bones, rounds=3, extra=()):
    """kit-fit the human solids (everything not on a prop bone) of one scale against that scale's drawings."""
    v = load(slug)["volumes"][str(scale)]
    ids = [s["id"] for s in v["body"] if s["bone"] not in props_bones]
    o = bun("kit-fit.ts", f"humans/{slug}", "--scale", str(scale), "--solid", ",".join(ids), "--rounds", str(rounds), "--write", *extra)
    return [l[:170] for l in o.splitlines() if l.startswith("kit-fit")]
def fails(slug, scale=None):
    import subprocess
    p = subprocess.run(["bash", f"{T}/figure-fails.sh", f"humans/{slug}"] + ([str(scale)] if scale else []), capture_output=True, text=True)
    return "\n".join(l[:520] for l in p.stdout.splitlines() if l.strip() and l != "FAIL:")

def dedupe(v):
    """drop loft sections that repeat the previous section's centre (a path needs a direction)."""
    for lst in ("body", "thermal", "structure"):
        for s in v[lst]:
            if s.get("kind") == "loft":
                out = []
                for q in s["sections"]:
                    if out and all(abs(a - b) < 1e-6 for a, b in zip(q["at"], out[-1]["at"])): continue
                    out.append(q)
                s["sections"] = out

def _val(slug, x, scale, rule, pat):
    import re
    f = f"{T}/🗑️generated/author-55/x-t.json"; json.dump(x, open(f, "w"))
    out = bun("volumes-check.ts", f"humans/{slug}", "--scale", str(scale), "--rules", rule, "--source", f)
    l = [l for l in out.splitlines() if l.startswith("FAIL") or l.startswith("ok")]
    return {k: float(v) for k, v in re.findall(pat, l[0])} if l else {}
def culprits(slug, scale, lst="structure", rule="volumes-structure-coverage", pat=r"(outsideBodyM:\w+)=([0-9.]+)", lim=None):
    import copy
    base = load(slug); sc = str(scale); b0 = _val(slug, base, sc, rule, pat); print("base", {k: round(v, 4) for k, v in b0.items()})
    for s in base["volumes"][sc][lst]:
        x = copy.deepcopy(base); x["volumes"][sc][lst] = [q for q in x["volumes"][sc][lst] if q["id"] != s["id"]]
        r = _val(slug, x, sc, rule, pat)
        d = {k: round(b0[k] - r.get(k, 0), 4) for k in b0 if b0[k] - r.get(k, 0) > 0.002}
        if d: print("  without", s["id"], d)

def set_area(slug, region, new, only_if=None):
    """text-level edit of `areaM2` of one thermal region of the catalog asset (re-reads right before); factor 1.5 rule is the caller's job."""
    import re
    p = f"{CAT}{slug}/🔣️.json"; L = open(p).read().split("\n")
    for i, l in enumerate(L):
        if l.strip().startswith('"areaM2"'):
            m = re.findall(r'"id": "([a-z0-9\-]+)",\n\s+"title"', "\n".join(L[max(0, i - 12):i]))
            if m and m[-1] == region:
                old = float(l.strip().split(":")[1].strip().rstrip(","))
                if only_if is not None and abs(old - only_if) > 1e-9: return f"{region}: unchanged ({old})"
                assert max(old, new) / min(old, new) <= 1.5 + 1e-9, (region, old, new)
                L[i] = l.replace(str(old), str(new), 1); open(p, "w").write("\n".join(L))
                open(f"{T}/🗑️generated/author-55/area-changes.txt", "a").write(f"{slug} {region} {old} -> {new}\n"); return f"{region}: {old} -> {new}"

def ensure_bones(slug, d, scale, spec, tone=0.45):
    """for bones named in spec = {bone: (solid id, material, thermal region)} that have no body solid at `scale`: hull loft from the scale's drawings, else a round loft along the bone."""
    v = d["volumes"][str(scale)]; A = asset(slug); L = {b["id"]: b["lengthM"] for b in A["rig"]["bones"]}; f = FEATURE[scale]
    for bone, (sid, mat, reg) in spec.items():
        if any(s["bone"] == bone for s in v["body"]): continue
        try: h = hull(slug, scale, bone, sid, mat, n=5, tone=tone)
        except Exception: h = None
        if h is None or len(h["sections"]) < 2: h = lofts(sid, bone, mat, [(0.0, 0.55 * f + 0.01, 0.55 * f + 0.01), (L[bone], 0.55 * f + 0.01, 0.55 * f + 0.01)], tone=tone)
        v["body"].append(h); v["thermal"].append(th(sid, reg))

def outer_areas(slug, scale, regions=None):
    """product outer area (self-burial only) per region against areaM2, from the validator's info measurements."""
    out = bun("volumes-check.ts", f"humans/{slug}", "--scale", str(scale), "--rules", "volumes-thermal-coverage", "--json")
    d = json.loads(out[out.find("{"):]); m = {x["name"]: x["value"] for x in d["measurements"]}
    return {k.split(":")[2]: round(v, 4) for k, v in m.items() if k.startswith("info:outerAreaM2:") and (regions is None or k.split(":")[2] in regions)}

def solo(slug, scale, limit=None, lst="structure", rule="volumes-structure-coverage", pat=r"(outsideBodyM:\w+)=([0-9.]+)"):
    """each structure solid alone: its own outside-the-body distance per view; prints those above `limit` (default 0.5 feature x 2)."""
    import copy
    base = load(slug); sc = str(scale); lim = limit if limit is not None else FEATURE[scale]
    for s in base["volumes"][sc][lst]:
        x = copy.deepcopy(base); x["volumes"][sc][lst] = [s]
        r = _val(slug, x, sc, rule, pat)
        if any(v > lim for v in r.values()): print("  ", s["id"], s.get("bone"), {k[14:]: round(v, 4) for k, v in r.items() if v > lim})
