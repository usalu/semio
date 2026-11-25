"""📈️ W15: the series of the coordinator's slice reports copied each minute (argv[1] = folder with slice-NNN.json, argv[2] = slice-rss.jsonl): uptime, phase, rss (sampled), guard parts, string and function counts, top 5 collections with real bytes where the report has them."""
import json, glob, sys
rss = [json.loads(l) for l in open(sys.argv[2])]
for path in sorted(glob.glob(sys.argv[1] + "/slice-[0-9]*.json")):
    try: d = json.load(open(path))
    except Exception: continue
    t = {x["type"]: x["count"] for x in d.get("heap", {}).get("types", [])}
    top = sorted(d.get("collections", []), key=lambda c: -(c.get("bytes") or c.get("estimatedBytes") or 0))[:5]
    print(int(d["uptimeSeconds"]), d["phase"]["name"][:50], d["phase"]["done"], "heap", d["memory"]["heapUsed"], "ext", d["memory"]["external"], "rss", d["memory"]["rss"], "str", t.get("string"), "obj", t.get("Object"), "fn", t.get("Function"))
    for c in top: print("     ", c["name"][:80], c["entries"], round((c.get("bytes") or c.get("estimatedBytes") or 0) / 1048576))
print("peak sampled rss", max(r["rssMB"] for r in rss) if rss else None)
