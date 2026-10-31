"""CV-H2: assemble a v2 human asset from the v1 file plus hand-authored specs (physical scenario, representations). Conversion plumbing only."""
import sys, json, copy, math
sys.path.insert(0, '.')
from common import *

CLO = 0.155


def tx(en, de): return {"en": en, "de": de}


def layer(id_, kind, material, R, cap, en, de, thick=None, src="authored-estimate", ref="iso-9920-2007", vap_override=None):
    d = {"id": id_, "title": tx(en, de), "kind": kind, "material": material}
    if thick is not None: d["thicknessM"] = thick
    d["resistanceM2KW"] = round(R, 4); d["vapourResistanceM2PaW"] = vap_override if vap_override is not None else vap(R)
    d["heatCapacityJM2K"] = cap; d["source"] = src
    if ref: d["reference"] = ref
    return d


def region(r):
    layers = r.get("layers", [])
    cap = 13600 + sum(l.get("heatCapacityJM2K", 0) for l in layers) if r["model"] == "body" else r["cap"]
    d = {"id": r["id"], "title": tx(r["en"], r["de"]), "kind": r["kind"], "model": r["model"], "material": r["material"], "areaM2": r["area"],
         "characteristicLengthM": r["L"], "geometry": r["geom"], "skyViewFactor": r["svf"], "layers": layers, "heatCapacityJM2K": cap}
    if "seg" in r:
        d["body"] = {"segment": r["seg"], "metabolicShare": r["ms"], "skinFraction": r["sf"]}
        if r.get("subject"): d["body"]["subject"] = r["subject"]
    if r.get("hidden"): d["hidden"] = r["hidden"]
    d["source"] = r.get("src", "authored-estimate")
    d["reference"] = r.get("ref", "iso-9920-2007")
    return d


def ensure_materials(a, mats):
    have = {m["id"] for m in a["materials"]}
    for m in mats:
        if m["id"] not in have: a["materials"].append(m)


def mat(id_, en, de, eps, abs_=None, cap=None, src="authored-estimate", ref=None, **kw):
    m = {"id": id_, "title": tx(en, de), "emissivity": eps}
    if abs_ is not None: m["solarAbsorptivity"] = abs_
    if cap is not None: m["specificHeatJKgK"] = cap
    m.update(kw); m["source"] = src
    if ref: m["reference"] = ref
    return m


def add_refs(a, refs):
    have = {r["id"] for r in a["provenance"]["references"]}
    for rid, cit in refs:
        if rid not in have: a["provenance"]["references"].append({"id": rid, "citation": cit})


GAGGE = ("gagge-1971", "Gagge, A. P., Stolwijk, J. A. J. and Nishi, Y. (1971). An effective temperature scale based on a simple model of human physiological regulatory response. ASHRAE Transactions 77(1), 247-262.")
GUPTA = ("gupta-2010", "Gupta, J. K., Lin, C.-H. and Chen, Q. (2010). Characterizing exhaled airflow from breathing and talking. Indoor Air 20(1), 31-39.")


def pattern(id_, en, de, rate, vt, fr, route, src="authored-estimate", ref="gupta-2010"):
    assert abs(sum(fr)-1) < 1e-9, id_
    return {"id": id_, "title": tx(en, de), "rateBpm": rate, "tidalVolumeL": vt, "inhaleFraction": fr[0], "pauseAfterInhaleFraction": fr[1], "exhaleFraction": fr[2],
            "pauseAfterExhaleFraction": fr[3], "route": route, "source": src, "reference": ref}


def finish(c, spec, reps, symbols=()):
    """apply the spec to context c (loaded v1 asset) and return the v2 document"""
    a = c.a
    del a["geometry"]
    ensure_materials(a, spec.get("materials", []))
    add_refs(a, spec.get("refs", []))
    # anchors for respiration/plume
    for an in spec["new_anchors"]:
        bone, wx, wy = an["bone"], an["wx"], an["wy"]
        lx, ly = ap(inv(c.w[bone]), wx, wy)
        a["anchors"] = [x for x in a["anchors"] if x["id"] != an["id"]]
        a["anchors"].append({"id": an["id"], "bone": bone, "x": round(lx, 4), "y": round(ly, 4), "kind": "emitter", "title": tx(an["en"], an["de"])})
    t = a["thermal"]
    env = spec["env"]; regions = [region(r) for r in spec["regions"]]
    body = spec["body"]
    t_new = {"environment": env, "baseMet": spec.get("baseMet", 1.0), "body": body, "respiration": spec["respiration"], "passengers": spec.get("passengers", []), "operating": spec.get("operating", []),
             "regions": regions, "emitters": spec["emitters"], "assumptions": spec["assumptions"]}
    a["thermal"] = t_new
    rid = [r["id"] for r in regions]
    for subj in {r["body"].get("subject") for r in regions if "body" in r}:
        bodyr = [r for r in regions if "body" in r and r["body"].get("subject") == subj]
        assert abs(sum(r["body"]["metabolicShare"] for r in bodyr)-1) < 1e-6, ("metabolic", subj, sum(r["body"]["metabolicShare"] for r in bodyr))
        assert abs(sum(r["body"]["skinFraction"] for r in bodyr)-1) < 1e-6, ("skin", subj, sum(r["body"]["skinFraction"] for r in bodyr))
    pats = {p["id"]: i for i, p in enumerate(spec["respiration"]["patterns"])}
    nonbreath = [e["id"] for e in spec["emitters"] if e["kind"] != "breath"]
    # poses
    for p in a["poses"]:
        rule = spec["pose_rule"](p)
        regs = {}
        for r in spec["regions"]:
            ex = rule.get("ex", {}).get(r["id"], r["ex"]); su = rule.get("su", {}).get(r["id"], r["su"])
            b = {"exposedFraction": ex, "sunlitFraction": su}
            if r["id"] in rule.get("contact", {}): b["contact"] = rule["contact"][r["id"]]
            if r["id"] in rule.get("wet", {}): b["wettedness"] = rule["wet"][r["id"]]
            regs[r["id"]] = b
        th = {"activityMet": rule["met"], "externalWorkW": rule.get("work", 0), "breathing": rule["pat"], "regions": regs,
              "emitters": {e: rule.get("em", {}).get(e, spec["emit_default"].get(e, 0.5)) for e in nonbreath}}
        assert rule["pat"] in pats, rule["pat"]
        if spec.get("passenger_rule"): th["passengers"] = spec["passenger_rule"](p)
        for o in spec.get("operating_default", {}): th.setdefault("operating", {})[o] = rule.get("op", {}).get(o, spec["operating_default"][o])
        p["thermal"] = th
    for cl in a["clips"]:
        cl["tracks"] = [tr for tr in cl["tracks"] if not tr["target"].startswith("emitter:") or tr["target"].split(":")[1] in nonbreath]
        cl["events"] = [e for e in cl["events"] if e["kind"] not in ("inhale", "exhale")]
        for ex in spec["clip_tracks"].get(cl["id"], []): cl["tracks"] = [x for x in cl["tracks"] if x["target"] != ex["target"]] + [ex]
    reps_out = {}
    for k in ("500", "200", "100", "50"):
        r = reps[k]
        if r.get("unavailable"): reps_out[k] = r["unavailable"]; continue
        reps_out[k] = {"status": "authored", "denominator": int(k), "validRange": {"minDenominator": r["range"][0], "maxDenominator": r["range"][1]}, "pens": r["pens"],
                       "hatches": r["hatches"], "geometry": {"elevation": r["shapes"]}, "decisions": r["decisions"]}
    # key order: put representations/symbols after anchors
    out = {}
    for k, v in a.items():
        out[k] = v
        if k == "anchors":
            out["representations"] = reps_out; out["symbols"] = list(symbols)
    out["provenance"]["references"] = a["provenance"]["references"]
    doc = {"$schema": c.doc["$schema"], "version": c.doc["version"], "asset": out}
    # validations
    for k, r in reps_out.items():
        if r["status"] != "authored": continue
        bound = set()
        for s in r["geometry"]["elevation"]:
            if "region" in s: bound.add(s["region"])
            for x in s.get("regions", []): bound.add(x)
        miss = [x for x in rid if x not in bound]
        if miss: print("WARN", c.id, k, "regions not drawn:", miss)
    return doc


def write(doc, asset_id):
    path = H + asset_id + "/🔣️.json"
    json.dump(doc, open(path, "w"), indent=2, ensure_ascii=False); open(path, "a").write("\n")
    return path


AIR0 = 0.161*CLO


def clothed(prefix, garments):
    """ISO 9920-style stack: enclosed air layer 0.161 clo plus the garments (intrinsic clo x 0.835)"""
    ls = [layer(prefix+"-air", "air-gap", "still-air", AIR0, 0, "Air enclosed under the clothing", "Unter der Kleidung eingeschlossene Luft", 0.004)]
    for gid, kind, m, iclu, cap, en, de, th in garments:
        ls.append(layer(prefix+"-"+gid, kind, m, iclu*0.835*CLO, cap, en, de, th))
    return ls


def std_emitters(nose_dir=-150, mouth_dir=-100, plume_region="jacket", plume_d=0.2):
    return [
        {"id": "breath-nose", "kind": "breath", "title": tx("Exhaled breath from the nostrils", "Ausatemluft aus den Nasenlöchern"), "anchor": "nose", "directionDeg": nose_dir, "diameterM": 0.0095, "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "breath-mouth", "kind": "breath", "title": tx("Exhaled breath from the mouth", "Ausatemluft aus dem Mund"), "anchor": "mouth", "directionDeg": mouth_dir, "diameterM": 0.0124, "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "plume", "kind": "plume", "title": tx("Thermal plume above the body", "Thermische Fahne über dem Körper"), "anchor": "crown", "directionDeg": 0, "diameterM": plume_d, "region": plume_region, "source": "authored-estimate", "reference": "ashrae-2021"}]
