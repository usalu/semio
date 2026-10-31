"""Shared builders for the thermal, pose, anchor and structural blocks of the NW-V2 plants (numbers stay in the asset scripts)."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]


def leaf_region(id, title, mat, area, length, geom, svf, hc, gs, lai, ptype, ref, source="authored-estimate", kind="foliage"):
    return {"id": id, "title": title, "kind": kind, "model": "leaf", "material": mat, "areaM2": area, "characteristicLengthM": length, "geometry": geom, "skyViewFactor": svf, "layers": [], "heatCapacityJM2K": hc,
            "leaf": {"stomatalConductanceMaxMS": gs, "leafAreaIndex": lai, "photosynthesis": ptype}, "source": source, "reference": ref}


def passive_region(id, title, kind, mat, area, length, geom, svf, hc, ref, source="authored-estimate", layers=None):
    return {"id": id, "title": title, "kind": kind, "model": "passive", "material": mat, "areaM2": area, "characteristicLengthM": length, "geometry": geom, "skyViewFactor": svf, "layers": layers or [], "heatCapacityJM2K": hc, "source": source, "reference": ref}


def transpiration(id, title, anchors, region, direction, diameter, ref="monteith-unsworth-2013"):
    return {"id": id, "kind": "transpiration", "title": title, "anchor": anchors[0], "anchors": anchors, "directionDeg": r4(direction), "diameterM": diameter, "region": region, "source": "authored-estimate", "reference": ref}


def bind(regions, emitters, wet=None):
    out = {}
    for rid, v in regions.items():
        d = {"exposedFraction": v[0], "sunlitFraction": v[1]}
        if len(v) > 2 and v[2] is not None:
            d["wettedness"] = v[2]
        out[rid] = d
    return {"regions": out, "emitters": emitters}


def pose(id, title, desc, bones, thermal, loads):
    return {"id": id, "title": title, "description": desc, "bones": bones, "ground": "authored", "contacts": [], "thermal": thermal, "loads": loads}


def wind_load(id, title, bone, x, y, cf, area, v, ref="en-1991-1-4"):
    return {"id": id, "kind": "wind", "title": title, "bone": bone, "x": r4(x), "y": r4(y), "fxN": wind_force(cf, area, v), "fyN": 0, "source": "authored-estimate", "reference": ref}


def vertical_load(id, kind, title, bone, x, y, fy, ref):
    return {"id": id, "kind": kind, "title": title, "bone": bone, "x": r4(x), "y": r4(y), "fxN": 0, "fyN": round(fy, 3), "source": "authored-estimate", "reference": ref}


def structure(support_ids, text_en, text_de):
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": support_ids, "assumptions": [T(text_en, text_de)], "clinical": False}


def presentation(pose_id, clip_id, scale):
    return {"defaultPose": pose_id, "defaultClip": clip_id, "defaultScale": scale, "viewpoints": ["elevation", "plan"]}


def pens(body, detail, texture, glazing, ground):
    return {"body": body, "detail": detail, "texture": texture, "glazing": glazing, "ground": ground}


def rep(denominator, lo, hi, pens_, hatches, elevation, decisions, plan=None):
    g = {"elevation": elevation}
    if plan:
        g["plan"] = plan
    return {"status": "authored", "denominator": denominator, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": pens_, "hatches": hatches, "geometry": g, "decisions": decisions}


def decisions(sil, ret, mer, omi, thin, hat, sty, ove):
    return {"silhouette": sil, "retained": ret, "merged": mer, "omitted": omi, "thinElements": thin, "hatching": hat, "styles": sty, "overlays": ove}


def unavailable(denominator, reason, feature, size, alternatives):
    return {"status": "unavailable", "denominator": denominator, "reason": reason, "limitingFeature": feature, "limitingSizeM": size, "alternatives": alternatives}


def anchors_support(bone, title, fl, fw, x=0, y=0, id="base-support", normal=90):
    return {"id": id, "bone": bone, "x": x, "y": y, "kind": "support", "title": title, "support": "fixed", "normalDeg": normal, "footprintLengthM": fl, "footprintWidthM": fw, "frictionCoefficient": 0.6}


def emitter_anchor(id, bone, x, y, title):
    return {"id": id, "bone": bone, "x": r4(x), "y": r4(y), "kind": "emitter", "title": title}


def up_in_bone(bones, bone):
    a = bones.w[bone][2]
    d = (90 - a + 180) % 360 - 180
    return d


def validate_text(doc):
    def walk(o, path):
        if isinstance(o, dict):
            ks = set(o.keys())
            if ks and ks <= {"en", "de"}:
                assert ks == {"en", "de"} and o["en"] and o["de"], path
            else:
                for k, v in o.items():
                    walk(v, path + "." + k)
        elif isinstance(o, list):
            for i, v in enumerate(o):
                walk(v, path + f"[{i}]")
    walk(doc, "$")
