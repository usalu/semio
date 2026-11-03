import json, sys
sys.stdout.reconfigure(encoding="utf8")
ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/"
def path(aid): return ROOT + aid + "/🔣️.json"
def load(aid): return json.load(open(path(aid), encoding="utf8"))
if __name__ == "__main__":
    aid, what = sys.argv[1], sys.argv[2]
    a = load(aid)["asset"]
    if what == "rig":
        for b in a["rig"]["bones"]: print(json.dumps(b, ensure_ascii=False))
        for p in a["poses"]: print("pose", p["id"], json.dumps(p["bones"], ensure_ascii=False)[:800])
        for c in a["clips"]: print("clip", c["id"], json.dumps(c, ensure_ascii=False)[:700])
    elif what == "rep":
        sc, view = sys.argv[3], sys.argv[4]
        r = a["representations"][sc]
        print("pens", r["pens"], "hatches", r["hatches"])
        g = r["geometry"][view]
        for s in (g["shapes"] if isinstance(g, dict) else g): print(json.dumps(s, ensure_ascii=False, separators=(",", ":")))
    elif what == "dec":
        print(json.dumps(a["representations"][sys.argv[3]]["decisions"], ensure_ascii=False, indent=1))
    elif what == "misc":
        for k in ("dimensions","anchors","materials","symbols"): print(k, json.dumps(a[k], ensure_ascii=False)[:1500])
