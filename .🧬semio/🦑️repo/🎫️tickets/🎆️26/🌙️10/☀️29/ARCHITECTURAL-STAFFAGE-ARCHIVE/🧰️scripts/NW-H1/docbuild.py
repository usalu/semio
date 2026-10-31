"""NW-H1: generic assembly of a v2 human asset document from authored specs (thermal bindings, clips, validations)."""
import json, math
from nwlib import *
from human import *


def mat(id_, en, de, eps, ab=None, cap=None, src="authored-estimate", ref=None, **kw):
    m = {"id": id_, "title": tx(en, de), "emissivity": eps}
    if ab is not None: m["solarAbsorptivity"] = ab
    if cap is not None: m["specificHeatJKgK"] = cap
    m.update(kw)
    m["source"] = src
    if ref: m["reference"] = ref
    return m


def ref(id_, cit):
    return {"id": id_, "citation": cit}


def region_obj(r, A):
    """r: dict(id,en,de,kind,material,model,frac,fcl,L,geom,svf, seg,ms,sf, layers/garments, cap, ...)"""
    if r["model"] == "body":
        layers = clothed(r["id"], r["garments"]) if r.get("garments") else r.get("layers", [])
        return body_region(r["id"], r["en"], r["de"], r["kind"], r["material"], r["frac"] * A * r.get("fcl", 1.0), r["L"], r["geom"], r["svf"], r["seg"], r["ms"], r["sf"], layers, r.get("cap_extra", 0), hidden=r.get("hidden"))
    return passive_region(r["id"], r["en"], r["de"], r["kind"], r["material"], r["area"], r["L"], r["geom"], r["svf"], r["cap"], src=r.get("src", "authored-estimate"), ref=r.get("ref"), layers=r.get("layers"), hidden=r.get("hidden"))


def bind(regs, rule, nonbreath, default_em):
    out = {}
    for r in regs:
        ex = rule.get("ex", {}).get(r["id"], r["ex"])
        su = rule.get("su", {}).get(r["id"], r["su"])
        b = {"exposedFraction": ex, "sunlitFraction": su}
        if r["id"] in rule.get("contact", {}): b["contact"] = rule["contact"][r["id"]]
        if r["id"] in rule.get("wet", {}): b["wettedness"] = rule["wet"][r["id"]]
        out[r["id"]] = b
    th = {"activityMet": rule["met"], "externalWorkW": rule.get("work", 0), "breathing": rule["pat"], "regions": out,
          "emitters": {e: rule.get("em", {}).get(e, default_em.get(e, 0.5)) for e in nonbreath}}
    return th


def assemble(spec):
    h = spec["human"]
    A = dubois(h.kg, h.H)
    regs_spec = spec["regions"]
    regions = [region_obj(r, A) for r in regs_spec]
    bodyr = [r for r in regions if "body" in r]
    assert abs(sum(r["body"]["metabolicShare"] for r in bodyr) - 1) < 1e-6, ("metabolic", sum(r["body"]["metabolicShare"] for r in bodyr))
    assert abs(sum(r["body"]["skinFraction"] for r in bodyr) - 1) < 1e-6, ("skin", sum(r["body"]["skinFraction"] for r in bodyr))
    resp = spec["respiration"]
    pats = {p["id"]: i for i, p in enumerate(resp["patterns"])}
    emitters = spec["emitters"]
    nonbreath = [e["id"] for e in emitters if e["kind"] != "breath"]
    poses = []
    for p in spec["poses"]:
        th = bind(regs_spec, p["rule"], nonbreath, spec["emit_default"])
        assert p["rule"]["pat"] in pats
        poses.append(make_pose(p["id"], p["en"], p["de"], p["den"], p["dede"], p["offs"], p["root"], p["contacts"], th, p.get("loads"), key=p.get("key", False), ground=p.get("ground", "auto")))
    clips = spec["clips"]
    reps_out = {}
    for k in ("500", "200", "100", "50"):
        r = spec["reps"][k]
        if r.get("unavailable"):
            reps_out[k] = r["unavailable"]
            continue
        reps_out[k] = {"status": "authored", "denominator": int(k), "validRange": {"minDenominator": r["range"][0], "maxDenominator": r["range"][1]}, "pens": r["pens"], "hatches": r["hatches"],
                       "geometry": {"elevation": r["shapes"]}, "decisions": r["decisions"]}
        bound = set()
        for s in r["shapes"]:
            if "region" in s: bound.add(s["region"])
            for x in s.get("regions", []): bound.add(x)
        miss = [x["id"] for x in regions if x["id"] not in bound and "hidden" not in x]
        if miss: print("WARN", spec["id"], k, "regions not drawn:", miss)
    asset = {
        "id": spec["id"], "category": "humans", "title": spec["title"], "summary": spec["summary"], "tags": spec["tags"],
        "provenance": {"author": "Semio staffage content worker NW-H1", "method": "handcrafted", "created": "2026-10-29", "references": spec["refs"], "notes": spec["notes"]},
        "dimensions": spec["dims"], "materials": spec["materials"],
        "rig": {"kind": "anatomical", "root": "pelvis", "bones": spec["rig"].bones},
        "anchors": spec["anchors"],
        "representations": reps_out, "symbols": [],
        "poses": poses, "clips": clips,
        "thermal": {"environment": spec["env"], "baseMet": 1.0, "body": spec["body"], "respiration": resp, "passengers": [], "operating": [], "regions": regions, "emitters": emitters, "assumptions": spec["assumptions"]},
        "structure": spec["structure"],
        "presentation": spec["presentation"],
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


def mass_total(rig):
    return sum(b["mass"]["kg"] for b in rig.bones if "mass" in b)


def keys_from_states(states, dt, pose_id, ease="smooth"):
    out = []
    for i, offs in enumerate(states):
        out.append({"t": r4(i * dt), "pose": pose_id, "ease": ease, "bones": {k: {kk: r4(vv) for kk, vv in v.items()} for k, v in offs.items()}})
    return out


def make_clip(id_, en, de, den, dede, dur, fps, loop, rootmotion, keys, contacts, phases, events, tracks, advance=None):
    c = {"id": id_, "title": tx(en, de), "description": tx(den, dede), "durationS": dur, "fps": fps, "loop": loop, "rootMotion": rootmotion}
    if advance is not None: c["advanceM"] = round(advance, 6)
    c.update({"keys": keys, "contacts": contacts, "phases": phases, "events": events, "tracks": tracks})
    return c


def ev(t, kind, en, de, target=None):
    d = {"t": r4(t), "kind": kind, "title": tx(en, de)}
    if target: d["target"] = target
    return d


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """bilingual decisions: each arg is (en, de)"""
    return {"silhouette": tx(*silhouette), "retained": tx(*retained), "merged": tx(*merged), "omitted": tx(*omitted), "thinElements": tx(*thin), "hatching": tx(*hatching), "styles": tx(*styles), "overlays": tx(*overlays)}
