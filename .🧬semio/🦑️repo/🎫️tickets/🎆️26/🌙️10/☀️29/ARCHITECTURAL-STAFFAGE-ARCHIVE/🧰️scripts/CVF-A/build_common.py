# [scratch CV-A] assembles a v2 animal asset from the v1 rig/anchors/poses/clips plus hand-authored thermal data and representations
import copy, json
from cva import *

def T_(en, de): return {"en": en, "de": de}
VAP_IM = 0.8  # fur/plumage is mostly still air: Lewis relation with i_m 0.8 (authored-estimate)

REFS = {
 "merck-resting-respiratory-rates": ("Reece, W. O. (ed.) (2004). Dukes' Physiology of Domestic Animals, 12th ed., Cornell University Press; table 'Resting Respiratory Rates' as reproduced in the Merck Veterinary Manual.", "https://www.merckvetmanual.com/multimedia/table/resting-respiratory-rates"),
 "kleiber-1932": ("Kleiber, M. (1932). Body size and metabolism. Hilgardia 6(11), 315-353. doi:10.3733/hilg.v06n11p315; 70 kcal per kg^0.75 per day = 3.39 W per kg^0.75 (Kleiber 1961, The Fire of Life).", "https://doi.org/10.3733/hilg.v06n11p315"),
 "stahl-1967": ("Stahl, W. R. (1967). Scaling of respiratory variables in mammals. Journal of Applied Physiology 22(3), 453-460. Allometric guidance only; the values used here are authored estimates.", None),
 "schmidt-nielsen-1970": ("Schmidt-Nielsen, K., Hainsworth, F. R., Murrish, D. E. (1970). Counter-current heat exchange in the respiratory passages: effect on water and heat balance. Respiration Physiology 9, 263-276. Guidance for cooled exhaled air.", None),
 "crawford-1962": ("Crawford, E. C. Jr. (1962). Mechanical aspects of panting in dogs. Journal of Applied Physiology 17(2), 249-251. Guidance for panting rate and small panting tidal volume; values here are authored estimates.", None),
 "craven-2010": ("Craven, B. A., Paterson, E. G., Settles, G. S. (2010). The fluid dynamics of canine olfaction: unique nasal airflow patterns as an explanation of macrosmia. Journal of the Royal Society Interface 7, 933-943. Guidance for sniffing frequency (4-7 Hz); values are authored estimates.", "https://doi.org/10.1098/rsif.2009.0490"),
 "bramble-carrier-1983": ("Bramble, D. M., Carrier, D. R. (1983). Running and breathing in mammals. Science 219, 251-256. Guidance for locomotor-respiratory coupling at the gallop; not used for the walking patterns.", None),
 "hoyt-taylor-1981": ("Hoyt, D. F., Taylor, C. R. (1981). Gait and the energetics of locomotion in horses. Nature 292, 239-240. Guidance for the cost of walking.", None),
 "lasiewski-dawson-1967": ("Lasiewski, R. C., Dawson, W. R. (1967). A re-examination of the relation between standard metabolic rate and body weight in birds. The Condor 69, 13-23. Guidance for non-passerine basal metabolism.", None),
 "cigr-2002": ("CIGR (2002). Climatization of Animal Houses: Heat and Moisture Production at Animal and House Levels. Commission Internationale du Genie Rural, 4th report of the working group. Guidance for heat production of cattle.", None),
}

def pattern(id, title, rate, vt, frac, route, ref, src="authored-estimate"):
    i, p, e, q = frac
    assert abs(i+p+e+q-1) < 1e-9, id
    return {"id": id, "title": title, "rateBpm": rate, "tidalVolumeL": vt, "inhaleFraction": i, "pauseAfterInhaleFraction": p, "exhaleFraction": e, "pauseAfterExhaleFraction": q, "route": route, "source": src, "reference": ref}

def region(id, title, kind, material, area, L, geom, sky, layers, cap, seg, meta, ref, src="authored-estimate"):
    """layers: (id, title, kind, material, thicknessM, conductivity W/mK, heatCapacity J/m2K, ref)"""
    ls = []
    for (lid, lt, lk, mat, th, k, c, lref) in layers:
        r = round(th/k, 4)
        ls.append({"id": lid, "title": lt, "kind": lk, "material": mat, "thicknessM": th, "resistanceM2KW": r,
                   "vapourResistanceM2PaW": round(r/(16.5e-3*VAP_IM), 2), "heatCapacityJM2K": c, "source": "authored-estimate", "reference": lref})
    return {"id": id, "title": title, "kind": kind, "model": "body", "material": material, "areaM2": area, "characteristicLengthM": L, "geometry": geom,
            "skyViewFactor": sky, "layers": ls, "heatCapacityJM2K": cap, "body": {"segment": seg, "metabolicShare": meta, "skinFraction": 0},
            "source": src, "reference": ref}

def passive(id, title, kind, material, area, L, geom, sky, cap, ref, src="authored-estimate", layers=()):
    r = region(id, title, kind, material, area, L, geom, sky, list(layers), cap, "tail", 0, ref, src)
    r["model"] = "passive"; r.pop("body"); return r

def finish_regions(regs):
    bodies = [r for r in regs if "body" in r]
    tot = sum(r["areaM2"] for r in bodies); ms = sum(r["body"]["metabolicShare"] for r in bodies)
    assert abs(ms-1) < 1e-6, ("metabolic share", ms)
    for r in bodies: r["body"]["skinFraction"] = round(r["areaM2"]/tot, 5)
    d = round(1-sum(r["body"]["skinFraction"] for r in bodies), 5); bodies[0]["body"]["skinFraction"] = round(bodies[0]["body"]["skinFraction"]+d, 5)
    return regs, sum(r["areaM2"] for r in regs)

def anchor_local(w, bone, x, y):
    return [round(v, 4) for v in app(inv(w[bone]), x, y)]

def emitter_anchor(w, id, bone, x, y, title):
    lx, ly = anchor_local(w, bone, x, y)
    return {"id": id, "bone": bone, "x": lx, "y": ly, "kind": "emitter", "title": title}

def bind_pose(p, cfg):
    rule = cfg["rule"](p); regs = cfg["regions"]; out = {}
    contact = {}
    for a in p["contacts"]:
        if a in cfg["contact_map"]:
            rid, f = cfg["contact_map"][a]; contact[rid] = min(1.0, contact.get(rid, 0)+f)
    for r in regs:
        rid = r["id"]; cf = round(contact.get(rid, 0), 4)
        exp = rule.get("exposed", {}).get(rid, cfg["exposed"].get(rid, 1.0))
        if cf: exp = min(exp, round(1-cf, 4))
        b = {"exposedFraction": exp, "sunlitFraction": round(cfg["sun"][rid]*rule.get("sun", 1.0), 3)}
        if rid in rule.get("wet", {}): b["wettedness"] = rule["wet"][rid]
        if cf: b["contact"] = {"fraction": cf, "tempC": cfg["ground_tempC"]}
        out[rid] = b
    th = {"activityMet": rule["met"], "breathing": rule["breathing"], "regions": out, "emitters": {e: rule.get("emit", {}).get(e, cfg["emit_default"][e]) for e in cfg["emit_default"]}}
    return th

def convert_clip(c, cfg):
    gone = {"emitter:" + e for e in cfg["breath_ids"]} | ({"activityMet"} if cfg.get("reset_met") else set())
    c["tracks"] = [t for t in c["tracks"] if t["target"] not in gone]
    c["events"] = [e for e in c["events"] if e["kind"] not in ("inhale", "exhale")]
    c["tracks"] += cfg.get("clip_tracks", {}).get(c["id"], [])
    return c

def collect_refs(x, acc):
    if isinstance(x, dict):
        for k, v in x.items():
            if k == "reference" and isinstance(v, str): acc.add(v)
            collect_refs(v, acc)
    elif isinstance(x, list):
        for v in x: collect_refs(v, acc)

def build(aid, cfg, reps, default_scale, dims):
    a = load_v1(aid)
    a.pop("geometry"); a["version"] = None; a.pop("version")
    refs = {r["id"]: r for r in a["provenance"]["references"]}
    for k, (cit, url) in REFS.items():
        refs[k] = {"id": k, "citation": cit} | ({"url": url} if url else {})
    for k, v in cfg.get("refs", {}).items(): refs[k] = {"id": k, "citation": v[0]} | ({"url": v[1]} if len(v) > 1 and v[1] else {})
    for k in cfg.get("drop_refs", []): refs.pop(k, None)
    regs, tot = finish_regions(cfg["regions"])
    th = a["thermal"]
    new_th = {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 20, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 0.2, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 0.15, "pressurePa": 101325},
              "baseMet": cfg["base_met"], "body": cfg["body"], "respiration": cfg["respiration"], "operating": [], "regions": regs, "emitters": cfg["emitters"], "assumptions": cfg["assumptions"], "passengers": []}
    a["thermal"] = new_th
    w = cfg["world"]
    anchors = [x for x in a["anchors"]]
    for em in cfg["new_anchors"]: anchors.append(em)
    a["anchors"] = anchors
    a["representations"] = reps
    a["symbols"] = cfg.get("symbols", [])
    for p in a["poses"]: p["thermal"] = bind_pose(p, cfg)
    a["clips"] = [convert_clip(c, cfg) for c in a["clips"]]
    a["dimensions"] = dims
    a["presentation"]["defaultScale"] = default_scale
    a["presentation"]["viewpoints"] = ["elevation"]
    a["completion"] = "authored"
    a["materials"] = cfg.get("materials") or a["materials"]
    a["provenance"]["notes"] = cfg["notes"]
    a["provenance"]["created"] = "2026-10-29"
    used = set(cfg.get('keep_refs', [])) | {r['id'] for r in load_v1(aid)['provenance']['references']} - set(cfg.get('drop_refs', [])); collect_refs(a, used)
    for r in used: assert r in refs, ("missing reference", r)
    a["provenance"]["references"] = [refs[k] for k in refs if k in used]
    a["structure"] = a["structure"]
    order = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "presentation", "exports", "completion"]
    a = {k: a[k] for k in order}
    return a

def rep_auth(den, vr, pens, shapes, dec, hatches=()):
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {"status": "authored", "denominator": den, "validRange": {"minDenominator": vr[0], "maxDenominator": vr[1]}, "pens": pens, "hatches": list(hatches),
            "geometry": {"elevation": shapes}, "decisions": {k: T_(*dec[k]) for k in keys}}

def rep_unavail(den, reason, feature, size, alts):
    return {"status": "unavailable", "denominator": den, "reason": T_(*reason), "limitingFeature": T_(*feature), "limitingSizeM": size, "alternatives": alts}
