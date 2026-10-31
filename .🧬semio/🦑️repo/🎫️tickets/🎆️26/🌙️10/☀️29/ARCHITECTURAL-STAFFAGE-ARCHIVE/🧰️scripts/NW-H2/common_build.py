"""Shared assembly helpers for the four NW-H2 assets: clip builder, pose records, standard physiology blocks."""
from lib import *

ENV_OUTDOOR = {"airTempC": 18, "meanRadiantTempC": 22, "skyTempC": 8, "groundTempC": 26, "relativeHumidity": 0.5, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 600, "diffuseFraction": 0.2, "pressurePa": 101325}


def pose_record(pid, en, de, den, dee, fig, contacts, thermal, ground="auto", key=False, root_y=None, loads=None):
    root = {}
    if abs(fig.root["angleDeg"]) > 1e-9:
        root["angleDeg"] = round(fig.root["angleDeg"], 3)
    if root_y is not None:
        root["y"] = round(root_y, 4)
    p = {"id": pid, "title": tx(en, de), "description": tx(den, dee)}
    if key:
        p["key"] = True
    p["bones"] = fig.pose_dict()
    if root:
        p["root"] = root
    p["ground"] = ground
    p["contacts"] = contacts
    p["thermal"] = thermal
    p["loads"] = loads or []
    return p


def clip_record(cid, en, de, den, dee, dur, loop, keys, contacts, phases, events, tracks, root_motion="none", advance=None, fps=30):
    c = {"id": cid, "title": tx(en, de), "description": tx(den, dee), "durationS": dur, "fps": fps, "loop": loop, "rootMotion": root_motion}
    if advance is not None:
        c["advanceM"] = advance
    c["keys"] = [{"t": t, "pose": p, "ease": e} for t, p, e in keys]
    c["contacts"] = [{"anchor": a, "fromS": f, "toS": t} for a, f, t in contacts]
    c["phases"] = [dict({"id": i, "fromS": f, "toS": t, "title": tx(*ti)}, **({"ballistic": True} if b else {})) for i, f, t, ti, b in phases]
    c["events"] = [dict({"t": t, "kind": k, "title": tx(*ti)}, **({"target": tg} if tg else {})) for t, k, ti, tg in events]
    c["tracks"] = [{"target": tg, "ease": e, "keys": [{"t": t, "v": v} for t, v in ks]} for tg, e, ks in tracks]
    return c


def physiology(mass, h, male, age, ref="gagge-1971", sweating=True):
    return {"species": "human", "massKg": mass, "skinAreaM2": round(dubois(h, mass), 3), "basalMetabolismW": round(mifflin_w(male, mass, h, age), 1), "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": sweating, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": ref}


def nose_mouth_emitters(nose_dir=-150, mouth_dir=-100):
    return [
        {"id": "nose-breath", "kind": "breath", "title": tx("Nostril jet", "Nasenstrahl"), "anchor": "nostrils", "directionDeg": nose_dir, "diameterM": 0.0095, "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "mouth-breath", "kind": "breath", "title": tx("Mouth jet", "Mundstrahl"), "anchor": "mouth", "directionDeg": mouth_dir, "diameterM": 0.0124, "source": "authored-estimate", "reference": "gupta-2010"},
    ]


def plume_emitter(region_id, d=0.35):
    return {"id": "plume", "kind": "plume", "title": tx("Rising warm air above the body", "Aufsteigende Warmluft über dem Körper"), "anchor": "crown", "directionDeg": 0, "diameterM": d, "region": region_id, "source": "authored-estimate", "reference": "incropera-2011"}


def asset_doc(a):
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": a}


def structure(texts):
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [tx(*t) for t in texts], "clinical": False}


def rep_authored(den, vr, pens, hatches, shapes, dec):
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {"status": "authored", "denominator": den, "validRange": {"minDenominator": vr[0], "maxDenominator": vr[1]}, "pens": pens, "hatches": hatches, "geometry": {"elevation": shapes}, "decisions": {k: tx(*dec[k]) for k in keys}}
