"""👯️ For each asset of a scan jsonl: the band-copy pairs with the ids of the copied view at both bands: `python3 copies.py <scan.jsonl> <catalog dir> [id]`."""
import glob, json, sys
for line in open(sys.argv[1]):
    r = json.loads(line)
    if len(sys.argv) > 3 and r["id"] != sys.argv[3]:
        continue
    a = json.load(open(glob.glob(f"{sys.argv[2]}/*/{r['id']}/🔣️.json")[0]))["asset"]
    print("==", r["id"], a["dimensions"])
    seen = set()
    for d in r["derived"]:
        if not d["against"].startswith("1:"):
            print("   in-band", d)
            continue
        other = d["against"].split()[0][2:]
        key = (d["view"], tuple(sorted((d["scale"], other), key=int)))
        if key in seen:
            continue
        seen.add(key)
        print("   copy", d["view"], key[1], d["distance"])
    for view in sorted({k[0] for k in seen}):
        for scale, rep in a["representations"].items():
            if "geometry" not in rep:
                continue
            g = rep["geometry"]
            ids = [s["id"] + ("*" if s.get("solid") else "") + ("" if s.get("filled") else "°") for s in g[view]]
            print(f"   {view} 1:{scale} ({len(ids)}; elev {len(g['elevation'])}, front {len(g['front'])}, plan {len(g['plan'])}) pens {rep['pens']}:", " ".join(ids)[:700])
