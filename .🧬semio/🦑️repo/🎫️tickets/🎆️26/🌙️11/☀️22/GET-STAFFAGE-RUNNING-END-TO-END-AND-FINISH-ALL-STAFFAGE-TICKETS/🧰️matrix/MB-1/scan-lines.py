"""🧾️ One block per asset of a scan file: derived flags, geometry codes per scale (hard codes marked), anchor classes, cue: scan-lines.py <scan.jsonl>"""
import json, sys
SOFT = ("group-drift",)
for line in open(sys.argv[1]):
    r = json.loads(line)
    derived = [f"{d['scale']} {d['view']}~{d['against']}({d['transform']} {d['distance']})" for d in r["derived"]]
    hard = {k: {c: n for c, n in v.items() if not c.startswith(SOFT)} for k, v in r["geometry"].items()}
    hard = {k: v for k, v in hard.items() if v}
    soft = sorted({c for v in r["geometry"].values() for c in v if c.startswith(SOFT)})
    cue = sorted({f"{c['scale']} {c['mode']}" for c in r["cue"]})
    clean = not derived and not hard and not cue and not r["errors"]
    print(f"{r['id']}: {'CLEAN' if clean else 'FINDINGS'} derived {len(derived)} hard {json.dumps(hard)} soft {soft} cue {cue} errors {r['errors']}")
    for d in derived: print("   D " + d)
    for code, text in r["examples"].items():
        if not code.startswith(SOFT): print(f"   ex {code}: {text[:200]}")
    if r.get("anchorIds"): print("   anchors " + json.dumps(r["anchorIds"]))
