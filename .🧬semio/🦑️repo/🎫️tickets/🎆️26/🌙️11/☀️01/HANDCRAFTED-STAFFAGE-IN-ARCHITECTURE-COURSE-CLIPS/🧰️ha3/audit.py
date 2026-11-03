import json, sys

SCALES = ["500", "200", "100", "50"]
VR_DEFAULT = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}


def refs_of(obj, out):
    if isinstance(obj, dict):
        for k, v in obj.items():
            if k in ("source", "reference") and isinstance(v, str):
                out.append((k, v))
            refs_of(v, out)
    elif isinstance(obj, list):
        for v in obj:
            refs_of(v, out)
    return out


def audit(path):
    f = []
    with open(path, encoding="utf-8") as fh:
        d = json.load(fh)
    a = d["asset"]
    aid = a["id"]
    rig_bones = {b["id"] for b in a.get("rig", {}).get("bones", [])}
    anchors = {x["id"] for x in a.get("anchors", [])}
    mats = {m["id"] for m in a.get("materials", [])}
    refs = {r["id"] for r in a.get("provenance", {}).get("references", [])}
    th = a.get("thermal", {})
    regs = {r["id"]: r for r in th.get("regions", [])}
    emitters = {e["id"]: e for e in th.get("emitters", [])}
    rs = th.get("respiration")
    ds = a.get("dimensions", {})

    for lang in ("en", "de"):
        if not a.get("title", {}).get(lang):
            f.append(f"title.{lang} missing")
        if not a.get("summary", {}).get(lang):
            f.append(f"summary.{lang} missing")
    if a.get("completion") != "authored":
        f.append(f"completion={a.get('completion')}")
    for r in a.get("materials", []):
        for k in ("source", "reference"):
            pass
    for k, r in regs.items():
        if r.get("material") and r["material"] not in mats:
            f.append(f"region {k} material {r['material']} not in materials")
    for k, src in refs_of(a, []):
        if k == "reference" and src not in refs and src.startswith(("gupta", "x")) is False:
            pass
    unresolved = sorted({v for k, v in refs_of(a, []) if k == "reference" and v not in refs})
    if unresolved:
        f.append(f"unresolved references: {unresolved}")
    bad_src = sorted({v for k, v in refs_of(a, []) if k == "source" and v not in ("literature", "authored-estimate", "illustrative", "measured", "catalogue", "manufacturer")})
    if bad_src:
        f.append(f"unusual source values: {bad_src}")

    # rig mass
    mass_sum = sum((b.get("mass") or {}).get("kg", 0) or 0 for b in a.get("rig", {}).get("bones", []))
    if ds.get("massKg") is not None and abs(mass_sum - ds["massKg"]) > 0.02 * max(1, ds["massKg"]):
        f.append(f"mass sum {mass_sum:.2f} vs massKg {ds['massKg']}")

    # representations
    reps = a.get("representations", {})
    drawn = {}
    for s in SCALES:
        r = reps.get(s)
        if r is None:
            f.append(f"scale {s} missing")
            continue
        if r.get("status") == "unavailable":
            for key in ("reason", "limitingFeature", "limitingSizeM", "alternatives"):
                if key not in r:
                    f.append(f"{s} unavailable lacks {key}")
            continue
        vr = r.get("validRange", {})
        den = int(s)
        if not (vr.get("minDenominator", 0) <= den <= vr.get("maxDenominator", 0)):
            f.append(f"{s} validRange {vr} excludes own denominator")
        dmin, dmax = VR_DEFAULT[s]
        pens = r.get("pens", {})
        b, dt, tx, gl, gd = pens.get("body"), pens.get("detail"), pens.get("texture"), pens.get("glazing"), pens.get("ground")
        if dt is not None and b is not None and dt > b:
            f.append(f"{s} pen hierarchy detail {dt} > body {b}")
        if tx is not None and dt is not None and tx > dt:
            f.append(f"{s} pen hierarchy texture {tx} > detail {dt}")
        if tx is not None and b is not None and tx > b:
            f.append(f"{s} pen hierarchy texture {tx} > body {b}")
        if gd is not None and b is not None and gd < b:
            f.append(f"{s} ground {gd} thinner than body {b}")
        if b is not None and b < 0.13:
            f.append(f"{s} body pen {b} below 0.13")
        if b is not None and b > 0.5:
            f.append(f"{s} body pen {b} above 0.5 (heavy at this scale)")
        names = set()
        for lst in ("elevation", "plan"):
            for sh in r.get("geometry", {}).get(lst) or []:
                names.add(sh.get("region"))
                for x in sh.get("regions") or []:
                    names.add(x)
                if sh.get("bone") and sh["bone"] not in rig_bones:
                    f.append(f"{s} {lst} shape {sh.get('id')} bone {sh['bone']} unknown")
                if sh.get("region") and sh["region"] not in regs:
                    f.append(f"{s} {lst} shape {sh.get('id')} region {sh['region']} unknown")
                role = sh.get("role")
                if role and pens and role in pens and pens.get(role) is None and role != "ground":
                    f.append(f"{s} {lst} shape {sh.get('id')} role {role} but pen is null")
                if sh.get("filled") and not sh.get("region") and not sh.get("regions") and role == "body":
                    f.append(f"{s} {lst} filled body shape {sh.get('id')} has no region")
                if sh.get("hatch") and sh["hatch"] not in [h["id"] for h in r.get("hatches", [])]:
                    f.append(f"{s} shape {sh.get('id')} hatch {sh['hatch']} undefined")
        names.discard(None)
        drawn[s] = names
        for rid, reg in regs.items():
            if reg.get("hidden"):
                continue
            if rid not in names:
                f.append(f"{s} region {rid} not drawn")
        for ds_ in ("decisions",):
            dec = r.get(ds_, {})
            for key in ("silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"):
                if not dec.get(key, {}).get("en") or not dec.get(key, {}).get("de"):
                    f.append(f"{s} decision {key} missing en/de")
        if not (dmin <= den <= dmax):
            pass
    # validRange overlaps
    prev = None
    for s in SCALES:
        r = reps.get(s, {})
        if r.get("status") == "authored":
            vr = r["validRange"]
            if prev is not None and vr["minDenominator"] < prev["maxDenominator"]:
                f.append(f"validRange {s} overlaps previous ({vr} vs {prev})")
            prev = vr
    # pose checks
    for p in a.get("poses", []):
        pid = p["id"]
        pt = p.get("thermal", {})
        for bone in p.get("bones", {}):
            if bone not in rig_bones:
                f.append(f"pose {pid} bone {bone} unknown")
        for c in p.get("contacts", []):
            if c not in anchors:
                f.append(f"pose {pid} contact {c} unknown")
        for rid in regs:
            if rid not in pt.get("regions", {}):
                f.append(f"pose {pid} missing region binding {rid}")
        for eid, e in emitters.items():
            if e.get("kind") == "breath":
                continue
            if eid not in pt.get("emitters", {}):
                f.append(f"pose {pid} missing emitter binding {eid}")
        if th.get("body") and "activityMet" not in pt:
            f.append(f"pose {pid} missing activityMet")
        if th.get("body") and "breathing" not in pt:
            f.append(f"pose {pid} missing breathing")
        for rid, rb in pt.get("regions", {}).items():
            if rid in regs and rb.get("contact") and not (0 <= rb["contact"]["fraction"] <= 1):
                f.append(f"pose {pid} contact fraction bad for {rid}")
    # clips
    for c in a.get("clips", []):
        cid = c["id"]
        for ev in c.get("events", []):
            if ev.get("kind") in ("inhale", "exhale") or ev.get("id") in ("inhale", "exhale"):
                f.append(f"clip {cid} event {ev.get('kind')} present")
        if c.get("durationS") and c.get("keys"):
            if c["keys"][-1]["t"] > c["durationS"] + 1e-9:
                f.append(f"clip {cid} key beyond duration")
        for k in c.get("keys", []):
            if k.get("pose") and k["pose"] not in [p["id"] for p in a.get("poses", [])]:
                f.append(f"clip {cid} key pose {k['pose']} unknown")
            for bn in k.get("bones", {}):
                if bn not in rig_bones:
                    f.append(f"clip {cid} bone {bn} unknown")
        for ct in c.get("contacts", []):
            if ct.get("anchor") not in anchors:
                f.append(f"clip {cid} contact anchor {ct.get('anchor')} unknown")
        for tr in c.get("tracks", []):
            t = tr.get("target", "")
            if t.startswith("region:"):
                parts = t.split(":")
                if len(parts) != 3 or parts[1] not in regs:
                    f.append(f"clip {cid} track {t} invalid")
            if t.startswith("emitter:") and t[8:] not in emitters:
                f.append(f"clip {cid} track {t} unknown emitter")
    # thermal body
    body_regs = [r for r in regs.values() if r.get("body")]
    if body_regs:
        ms = sum(r["body"].get("metabolicShare", 0) for r in body_regs if not r["body"].get("subject"))
        sf = sum(r["body"].get("skinFraction", 0) for r in body_regs if not r["body"].get("subject"))
        if abs(ms - 1) > 0.005:
            f.append(f"metabolicShare sum {ms:.3f}")
        if abs(sf - 1) > 0.005:
            f.append(f"skinFraction sum {sf:.3f}")
        area = sum(r.get("areaM2", 0) for r in body_regs if not r["body"].get("subject"))
        bd = th.get("body", {})
        if bd.get("skinAreaM2") and abs(area - bd["skinAreaM2"]) / bd["skinAreaM2"] > 0.05:
            f.append(f"region area sum {area:.3f} vs skinAreaM2 {bd['skinAreaM2']}")
        if bd.get("metWM2") not in (None, 58.15):
            f.append(f"metWM2 {bd.get('metWM2')}")
    # respiration
    if rs:
        for pat in rs.get("patterns", []):
            if pat.get("source") is None or pat.get("reference") not in refs:
                f.append(f"pattern {pat['id']} source/reference unresolved")
            if pat["inhaleFraction"] + pat["pauseAfterInhaleFraction"] + pat["exhaleFraction"] + pat["pauseAfterExhaleFraction"] > 1.0001:
                f.append(f"pattern {pat['id']} fractions sum > 1")
            if pat.get("route") not in ("nose", "mouth", "both"):
                f.append(f"pattern {pat['id']} route {pat.get('route')}")
        for key in ("noseEmitter", "mouthEmitter"):
            if rs.get(key) and rs[key] not in emitters:
                f.append(f"respiration {key} {rs[key]} unknown")
    for eid, e in emitters.items():
        if e.get("anchor") and e["anchor"] not in anchors:
            f.append(f"emitter {eid} anchor {e['anchor']} unknown")
        if e.get("region") and e["region"] not in regs:
            f.append(f"emitter {eid} region {e['region']} unknown")
    # presentation
    pr = a.get("presentation", {})
    if pr.get("defaultScale") not in [int(s) for s in SCALES]:
        f.append(f"defaultScale {pr.get('defaultScale')} not a scale")
    pose_ids = [p["id"] for p in a.get("poses", [])]
    if pr.get("defaultPose") not in pose_ids:
        f.append(f"defaultPose {pr.get('defaultPose')} unknown")
    if len([p for p in a.get("poses", []) if not p.get("key")]) < 3:
        f.append("fewer than 3 named poses")
    if len(a.get("clips", [])) < 2:
        f.append("fewer than 2 clips")
    return aid, d, f, reps, drawn


if __name__ == "__main__":
    for p in sys.argv[1:]:
        aid, d, f, reps, drawn = audit(p)
        print(f"== {aid} ({len(f)} findings)")
        for x in f:
            print("   -", x)
        pens = {s: (reps[s].get("pens") if reps.get(s, {}).get("status") == "authored" else reps.get(s, {}).get("status")) for s in SCALES if s in reps}
        print("   pens", json.dumps(pens, ensure_ascii=False))
        print("   vr", {s: (reps[s].get("validRange") if reps.get(s, {}).get("status") == "authored" else "U") for s in SCALES if s in reps})
