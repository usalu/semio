import json, sys
sys.stdout.reconfigure(encoding="utf8")
ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
def load(cat, aid):
    return json.load(open(ROOT + cat + "/" + aid + "/🔣️.json", encoding="utf8"))["asset"]
if __name__ == "__main__":
    cat, aid = sys.argv[1], sys.argv[2]
    what = sys.argv[3]
    a = load(cat, aid)
    if what == "rig":
        for b in a["rig"]["bones"]:
            print(json.dumps(b, ensure_ascii=False))
        print("dims", a["dimensions"], "presentation", json.dumps(a["presentation"], ensure_ascii=False))
        for p in a["poses"]:
            print("pose", p["id"], json.dumps(p["bones"], ensure_ascii=False)[:600])
        for c in a["clips"]:
            print("clip", c["id"], c.get("durationS"), json.dumps(c, ensure_ascii=False)[:500])
    elif what == "rep":
        sc = sys.argv[4]
        r = a["representations"][sc]
        g = r["geometry"]
        print("keys", list(g.keys()), "pens", r["pens"], "hatches", r["hatches"])
        view = sys.argv[5]
        for s in g[view]["shapes"] if isinstance(g[view], dict) and "shapes" in g[view] else g[view]:
            print(json.dumps(s, ensure_ascii=False, separators=(",", ":")))
    elif what == "dec":
        sc = sys.argv[4]
        print(json.dumps(a["representations"][sc]["decisions"], ensure_ascii=False, indent=1))
    elif what == "geomshape":
        sc = sys.argv[4]
        g = a["representations"][sc]["geometry"]["elevation"]
        print(type(g), (list(g.keys()) if isinstance(g, dict) else len(g)))
