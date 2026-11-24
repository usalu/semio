"""🔦️ Prints one compact block per asset of a catalog-scan jsonl: `python3 show.py <scan.jsonl> [id]`."""
import json, sys
for line in open(sys.argv[1]):
    r = json.loads(line)
    if len(sys.argv) > 2 and r["id"] != sys.argv[2]:
        continue
    print("==", r["id"], r["category"], "authored", r["authored"], "unavailable", r["unavailable"], "poses", r["poses"], "clips", r["clips"])
    for d in r["derived"]:
        print("   derived", d)
    for k, v in r["geometry"].items():
        if v:
            print("   geom", k, v)
    for k in ("rest", "syncClasses", "anchorClasses", "anchorIds"):
        if r.get(k):
            print("  ", k, json.dumps(r[k], ensure_ascii=False))
    for k, v in r["examples"].items():
        print("   ex", k, v)
    cue = {}
    for c in r["cue"]:
        cue.setdefault(f'{c["scale"]} {c["mode"]}', []).append(c["view"] + (":" + c["unavailable"] if "unavailable" in c else ""))
    if cue:
        print("   cue", cue)
    if r["errors"]:
        print("   errors", r["errors"])
