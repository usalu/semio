# [scratch] NW-A1 asset document assembly + verification report
import json, math, os
from lib import *
from build import *

PENS = {
    "500": dict(body=0.13, detail=None, texture=None, glazing=None, ground=0.18),
    "200": dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25),
    "100": dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.35),
    "50": dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.5),
}
RANGE = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}


def D8(sil, ret, mer, omi, thin, hat, sty, ove):
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, (sil, ret, mer, omi, thin, hat, sty, ove))}


def authored(den, sh, decisions, hatches=None, pens=None):
    lo, hi = RANGE[str(den)]
    return {"status": "authored", "denominator": den, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": pens or PENS[str(den)], "hatches": hatches or [], "geometry": {"elevation": sh.items}, "decisions": decisions}


def unavailable(den, reason, feature, size, alts):
    return {"status": "unavailable", "denominator": den, "reason": reason, "limitingFeature": feature, "limitingSizeM": size, "alternatives": alts}


def respiration_block(patterns, motion, exh_t, exh_rh, src_ref):
    return {"noseEmitter": "breath-nose", "mouthEmitter": "breath-mouth", "exhaledTempC": exh_t, "exhaledRelativeHumidity": exh_rh, "patterns": patterns, "motion": motion, "source": "authored-estimate", "reference": src_ref}


def pattern(id, title, rate, vt, fr, route, ref, src="authored-estimate"):
    return {"id": id, "title": title, "rateBpm": rate, "tidalVolumeL": vt, "inhaleFraction": fr[0], "pauseAfterInhaleFraction": fr[1], "exhaleFraction": fr[2], "pauseAfterExhaleFraction": fr[3], "route": route, "source": src, "reference": ref}


def write(asset, folder):
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    d = ANIMALS + "/" + folder
    os.makedirs(d, exist_ok=True)
    path = d + "/🔣️.json"
    json.dump(doc, open(path, "w"), ensure_ascii=False, separators=(",", ":"))
    return path


def verify_poses(sk, asset, rep_shapes):
    """prints extents of the standing pose per rep and lowest geometry y per pose"""
    msgs = []
    for p in asset["poses"]:
        offs = {k: v.get("angleDeg", 0.0) for k, v in p["bones"].items()}
        root = None
        if p.get("root"):
            root = (sk.hip[0], p["root"].get("y", sk.hip[1]))
        fr = sk.frames(offs, root)
        if p["ground"] == "auto" and p["contacts"]:
            ys = [sk.anchor_w(fr, c)[1] for c in p["contacts"]]
            drop = min(ys)
            fr = {k: ((o[0], o[1] - drop), a) for k, (o, a) in fr.items()}
        for den, sh in rep_shapes.items():
            rings = world_rings(sk, sh.items, fr)
            x0, x1, y0, y1 = bounds(rings)
            if p["id"] == asset["presentation"]["defaultPose"]:
                msgs.append("default %s: x %.3f..%.3f (w %.3f) y %.3f..%.3f (h %.3f)" % (den, x0, x1, x1 - x0, y0, y1, y1 - y0))
            for bid, v in p["bones"].items():
                lo, hi = sk.lim[bid]
                if not lo - 1e-6 <= v.get("angleDeg", 0) <= hi + 1e-6:
                    msgs.append("LIMIT %s %s %.1f [%s,%s]" % (p["id"], bid, v["angleDeg"], lo, hi))
            if not p.get("key") and y0 < -0.004:
                msgs.append("LOW %s %s y0 %.3f" % (p["id"], den, y0))
    return msgs
