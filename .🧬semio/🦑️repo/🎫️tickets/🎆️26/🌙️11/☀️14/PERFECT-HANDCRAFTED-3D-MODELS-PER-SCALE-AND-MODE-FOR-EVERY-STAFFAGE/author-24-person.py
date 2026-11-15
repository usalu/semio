"""Generic person driver (author 24): python3 author-24-person.py <slug> [--only N] [--finish]
Fine-raster kit fit (cell feature/8, section shifts): cfg.KITFIT = {500: ["--shift --sections --rounds 3 --bounds 0.6,2.2"]} runs author-24-kitfit.ts after the coordinate descent.
Config module author_24_cfg_<slug_with_underscores>.py provides:
  repair50(vols)            hand repairs of the seeded 1:50 body (after the generic ones)
  post50(vols)              hand repairs after the 1:50 fit and left/right symmetrisation
  DROP {100,200,500}        ids left out of the coarser scale (derived from the previous scale's fitted body)
  special(b, s)             per-scale hand work on the coarsened list before its fit
  spec                      None or {region: [body id]}; default from the seed's 1:50 thermal references ('=id' forces a reference)
  thermal_extra(a, body, s) extra thermal solids
  skip_members, struct_extra(a, body, scale)
  DECISIONS {50,100,200,500} of D(en, de)
"""
import importlib
import os
import sys
sys.path.insert(0, ".")
from author_24_lib import *

slug = sys.argv[1]
cfg = importlib.import_module("author_24_cfg_" + slug.replace("-", "_"))
A = asset(slug)
L = {b["id"]: b["lengthM"] for b in A["rig"]["bones"]}
EXP = {"arm": 4.0, "forearm": 4.0, "upper-arm": 4.0, "thigh": 4.0, "shank": 4.0, "sleeve": 4.0}
only = int(sys.argv[sys.argv.index("--only") + 1]) if "--only" in sys.argv else None
PREV = {100: 50, 200: 100, 500: 200}


def mitten(side):
    h = L[f"hand-{side}"]
    k = h / 0.182
    r = ROT_U
    secs = [sec(-0.005 * k, 0, 0, 0.018 * k, 0.02 * k, 3, r), sec(0.11 * h, 0, 0, 0.026 * k, 0.029 * k, 3, r), sec(0.44 * h, 0.002 * k, 0, 0.0275 * k, 0.03 * k, 3, r), sec(0.77 * h, 0.002 * k, 0, 0.025 * k, 0.026 * k, 3, r), sec(0.93 * h, 0.001 * k, 0, 0.024 * k, 0.025 * k, 4.5, r), sec(h, 0.001 * k, 0, 0.021 * k, 0.0235 * k, 4.5, r)]
    return loft(f"hand-{side}", f"hand-{side}", "skin", 0.85, secs)


def generic50(vols):
    body = vols["50"]["body"]
    body[:] = [s for s in body if not s["id"].startswith("fist-")]
    have = {s["id"] for s in body}
    for s in body:
        if s["kind"] == "loft":
            pre = s["id"].rsplit("-", 1)[0]
            if pre in EXP:
                for q in s["sections"]:
                    q["exponent"] = EXP[pre]
            if s["bone"].startswith("forearm") and s["id"].startswith("forearm"):
                lim = L[s["bone"]] + 0.01
                keep = [q for q in s["sections"] if q["at"][0] <= lim]
                last = copy.deepcopy(keep[-1])
                last["at"][0] = L[s["bone"]] + 0.015
                last["rx"] *= 0.92
                last["ry"] *= 0.92
                s["sections"] = keep + [last]
    for side in ("l", "r"):
        if f"hand-{side}" not in have:
            body.append(mitten(side))


def floor(b, s):
    for x in b:
        if x["kind"] == "loft" and not x["id"].startswith(getattr(cfg, "floor_skip", ("shoe", "boot"))):
            for q in x["sections"]:
                q["rx"] = max(q["rx"], 0.5008 * FEATURE[s])
                q["ry"] = max(q["ry"], 0.5008 * FEATURE[s])


def stage(s, prev):
    vols = load(slug)
    b = coarsen_all(prev, s, cfg.DROP[s])
    cfg.special(b, s)
    vols[str(s)]["body"] = b
    save(slug, vols)
    print(fit(slug, s, **getattr(cfg, "FIT", {}).get(s, {})))
    vols = load(slug)
    symmetrise(vols[str(s)]["body"], skip=getattr(cfg, "sym_skip", ()))
    floor(vols[str(s)]["body"], s)
    save(slug, vols)
    for args in getattr(cfg, "KITFIT", {}).get(s, []):
        out = run(["author-24-kitfit.ts", f"humans/{slug}", "--scale", str(s)] + args.split() + ["--write"])
        print("\n".join(l[:200] for l in out.splitlines() if l.startswith("kit-fit")))
    vols = load(slug)
    print("\n".join(mess(slug, s, "volumes-silhouette,volumes-bounds")))
    return vols[str(s)]["body"]


def finish_all():
    V = seed(slug)
    spec = getattr(cfg, "spec", None)
    if spec is None:
        spec = {}
        for t in V["50"]["thermal"]:
            if "of" in t and t["of"] not in getattr(cfg, "thermal_skip", ()):
                spec.setdefault(getattr(cfg, "region_of", {}).get(t["of"], t["region"]), []).append(t["of"])
    jb = {200: [b for b in member_bones(A) if b.split("-")[0] in ("arm", "thigh", "shank", "head", "neck", "torso")], 500: []}
    CFG = dict(thermal={50: spec}, thermal_extra=getattr(cfg, "thermal_extra", lambda a, b, s: []), emitter_region=getattr(cfg, "emitter_region", {"*": "face-skin"}), masses=True, joints=jb, decisions=cfg.DECISIONS, skip_members=getattr(cfg, "skip_members", ()))
    if hasattr(cfg, "struct_extra"):
        CFG["struct_extra"] = cfg.struct_extra
    vols = load(slug)
    finish(slug, vols, CFG)
    vols = load(slug)
    clip_members(slug, vols)
    fix_joints(slug, vols)
    shorten_emitters(vols)
    if hasattr(cfg, "post_finish"):
        cfg.post_finish(vols)
    save(slug, vols)
    print("finished", summary(slug))


if "--finish" in sys.argv:
    finish_all()
    sys.exit()
if only:
    stage(only, load(slug)[str(PREV[only])]["body"])
    sys.exit()
vols = seed(slug)
vols = {"50": {"status": "authored", "denominator": 50, "body": vols["50"]["body"], "thermal": [], "structure": [], "decisions": []}, **{str(s): {"status": "authored", "denominator": s, "body": [], "thermal": [], "structure": [], "decisions": []} for s in (100, 200, 500)}}
generic50(vols)
cfg.repair50(vols)
save(slug, vols)
print(fit(slug, 50))
vols = load(slug)
symmetrise(vols["50"]["body"], skip=getattr(cfg, "sym_skip", ()))
cfg.post50(vols)
save(slug, vols)
print("\n".join(mess(slug, 50, "volumes-silhouette,volumes-bounds")))
prev = vols["50"]["body"]
for s in (100, 200, 500):
    prev = stage(s, prev)
