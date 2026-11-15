"""🏗️ Author 09 build driver: 1:50 repair, per-scale coarsening, silhouette fitting, thermal and structure generation, tuning, checking."""
import copy
import json
import math
import sys

from author_09_lib import *


def fit(slug, scale, ids=None, rounds=3, bounds="0.7,1.8", steps="0.12,0.06,0.03", sections=True):
    args = ["author-09-fit.ts", f"humans/{slug}", str(scale), "--write", "--rounds", str(rounds), "--bounds", bounds, "--steps", steps]
    if ids:
        args += ["--ids", ",".join(ids)]
    out = run(args)
    res = [l for l in out.splitlines() if l.startswith("fit")]
    if sections:
        out2 = run(args + ["--sections", "--steps", "0.10,0.05,0.025", "--rounds", "3"])
        res += [l for l in out2.splitlines() if l.startswith("fit")]
    return "\n".join(res)


def coarsen_all(body, scale, drop=(), absorb=True, e_limit=None):
    f = FEATURE[scale]
    out = []
    for s in body:
        if s["id"] in drop:
            continue
        s = copy.deepcopy(s)
        k = s["kind"]
        if k == "loft":
            s = coarsen_loft(s, scale)
            if e_limit:
                for q in s["sections"]:
                    q["exponent"] = min(q.get("exponent", 2), e_limit)
        elif k in ("ellipsoid", "capsule"):
            min_radius(s, 0.55 * f)
        elif k == "box":
            s["size"] = [max(x, 1.05 * f) for x in s["size"]]
        out.append(s)
    return out


def layered_thermal(asset_, body, spec, scale, factors):
    return thermal_from(asset_, body, spec, scale, factors)


def anchor(a, aid):
    return next(x for x in a["anchors"] if x["id"] == aid)


def emitters_for(a, scale, cfg):
    out = []
    for e in a["thermal"].get("emitters", []):
        eid = e["id"]
        an = anchor(a, e["anchor"])
        reg = e.get("region") or cfg["emitter_region"].get(eid) or cfg["emitter_region"]["*"]
        at = [an["x"], an["y"], an.get("z", 0.0)]
        if eid in cfg.get("plume", ["plume", "body-plume"]):
            layers = next(r for r in a["thermal"]["regions"] if r["id"] == reg).get("layers", [])
            lay = layers[0]["id"] if scale == 50 and len(layers) > 1 else None
            out.append(plume(a, eid, reg, scale, an["bone"], at, lay, cfg.get("plume_material", "skin")))
        elif eid in cfg.get("direct", {}):
            out.append(cfg["direct"][eid](a, e, an, scale))
        else:
            d = cfg.get("direction", (0, -1, 0))
            out.append(emitter(a, eid, reg, scale, an["bone"], at, d, material=cfg.get("emitter_material", "skin")))
    return out


def finish(slug, vols, cfg):
    a = asset(slug)
    for scale in (50, 100, 200, 500):
        v = vols[str(scale)]
        body = v["body"]
        ids = {s["id"] for s in body}
        spec_all = cfg["thermal"].get(scale, cfg["thermal"][50]) if isinstance(cfg["thermal"], dict) and 50 in cfg["thermal"] else cfg["thermal"]
        spec = {r: [i for i in lst if i.lstrip("=") in ids] for r, lst in spec_all.items()}
        spec = {r: l for r, l in spec.items() if l}
        extra_t = cfg.get("thermal_extra", lambda a_, body_, s_: [])(a, body, scale)

        def make(factors, spec=spec, scale=scale, body=body, extra_t=extra_t):
            return thermal_from(a, body, spec, scale, factors) + emitters_for(a, scale, cfg) + copy.deepcopy(extra_t)

        v["thermal"] = make({})
        sk = cfg.get("skip_members", ())
        ex = cfg["struct_extra"](a, body, scale) if "struct_extra" in cfg else []
        jb = cfg.get("joints", {}).get(scale)
        v["structure"] = structure_from(a, body, scale, joints=jb, masses=cfg.get("masses", True), skip=sk, extra=ex)
        v["decisions"] = cfg["decisions"][scale]
        save(slug, vols)
    return vols


def D(en, de):
    return {"en": en, "de": de}
