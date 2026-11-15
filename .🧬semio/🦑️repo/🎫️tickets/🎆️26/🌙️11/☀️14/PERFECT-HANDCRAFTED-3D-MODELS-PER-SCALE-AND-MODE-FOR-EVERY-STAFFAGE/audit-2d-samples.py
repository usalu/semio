import json, sys, collections
r = json.load(open(sys.argv[1], encoding="utf-8"))
ROOT = sys.argv[2]
SAMPLES = ["🧍️standing-adult", "x"]
pick = {"👤️humans": "standing-adult", "🐕️animals": "labrador-dog", "🌳️vegetation": "english-oak", "🚜️vehicles": "city-car", "🛋️furniture": "cafe-chair", "🔌️appliances": "gas-boiler"}
S = ["50", "100", "200", "500"]
VPS = ["elevation", "front", "plan"]
for cat, aid in pick.items():
    rec = next(x for x in r if x["cat"] == cat and x["id"] == aid)
    doc = json.load(open(f"{ROOT}/{rec['path']}", encoding="utf-8"))
    a = doc["asset"] if "asset" in doc else doc
    print(f"## {cat} / {aid}  (dims h={a['dimensions']['heightM']} w={a['dimensions']['widthM']} d={a['dimensions']['depthM']} m)")
    print("bones:", len(a["rig"]["bones"]), " materials:", len(a["materials"]), " thermal regions:", len(a["thermal"]["regions"]))
    prev = None
    for s in S:
        sc = a["representations"][s]
        if sc["status"] == "unavailable":
            print(f"- 1:{s} UNAVAILABLE cause={sc['cause']} limitingFeature={sc['limitingFeature']['en']} size={sc['limitingSizeM']} m; alternatives={[ (al['kind'], al.get('denominator', al.get('symbol', al.get('asset')))) for al in sc['alternatives']]}")
            print(f"    reason: {sc['reason']['en']}")
            continue
        g = sc["geometry"]
        shapes = [(vp, sh) for vp in VPS for sh in g[vp]]
        roles = collections.Counter(sh["role"] for _, sh in shapes)
        kinds = collections.Counter(sh["kind"] for _, sh in shapes)
        solids = sum(1 for _, sh in shapes if "solid" in sh)
        regs = sorted({sh.get("region") for _, sh in shapes if sh.get("region")} | {y for _, sh in shapes for y in sh.get("regions", [])})
        ids = {sh["id"] for _, sh in shapes}
        sig = lambda sh: json.dumps({k: v for k, v in sh.items() if k != "id"}, sort_keys=True)
        cnt = collections.Counter(sig(sh) for _, sh in shapes)
        inherited = "-"
        if prev is not None:
            pids = prev["ids"]
            pshapes = prev["sigs"]
            common_sig = sum(min(cnt[k], pshapes[k]) for k in cnt)
            inherited = f"{common_sig}/{len(shapes)} geometry-identical to 1:{prev['s']} (id-set overlap {len(ids & pids)}; only-here {len(ids - pids)}; dropped-from-coarser {len(pids - ids)})"
        pens = sc["pens"]
        print(f"- 1:{s} authored validRange={sc['validRange']['minDenominator']}..{sc['validRange']['maxDenominator']} shapes={len(shapes)} per-vp={ {vp: len(g[vp]) for vp in VPS} } roles={dict(roles)} kinds={dict(kinds)} solid-hints={solids} regions={len(regs)} hatches={[h['id']+':'+h['pattern'] for h in sc['hatches']]} sections={sorted((sc.get('sections') or {}).keys())} pens(body/detail/texture/glazing/ground)={pens['body']}/{pens['detail']}/{pens['texture']}/{pens['glazing']}/{pens['ground']}")
        print(f"    vs coarser: {inherited}")
        print(f"    omitted decision: {sc['decisions']['omitted']['en'][:400]}")
        print(f"    merged decision: {sc['decisions']['merged']['en'][:250]}")
        print(f"    shape ids by vp: " + "; ".join(f"{vp}: " + ",".join(sh['id'] for vv, sh in shapes if vv == vp) for vp in VPS)[:1400])
        prev = {"s": s, "ids": ids, "sigs": cnt}
    print()
