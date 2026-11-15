"""🩹 Author 64: gives regionless emitter solids the emitter's declared region and renames the undefined region face-skin to face (seated-guitarist). Only `region` fields change; atomic per file, skipped when touched in the last 20 minutes."""
import glob, json, os, sys, time
T = os.path.dirname(os.path.abspath(__file__))
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
for key in sys.argv[1:]:
    cat, slug = key.split("/")
    path = f"{T}/volumes/{key}.json"
    if (time.time() - os.path.getmtime(path)) / 60 < 20 and "--force-age" not in sys.argv:
        print("SKIP", key); continue
    raw = open(path).read()
    src = json.loads(raw)
    asset = json.load(open(glob.glob(f"{CAT}/*{cat}/{slug}/*.json")[0]))["asset"]
    regions = {r["id"] for r in asset["thermal"]["regions"]}
    emit = {e["id"]: e.get("region") for e in asset["thermal"].get("emitters", [])}
    changed = 0
    for s, v in src["volumes"].items():
        if v.get("status") != "authored":
            continue
        for x in v["thermal"]:
            r = x.get("region")
            if r is None or r not in regions:
                tags = x.get("tags") or []
                target = None
                for t in tags:
                    if emit.get(t):
                        target = emit[t]
                if r == "face-skin" and "face" in regions:
                    target = "face"
                if target:
                    x["region"] = target
                    changed += 1
                else:
                    print("UNRESOLVED", key, s, x.get("id"), r)
    out = json.dumps(src, ensure_ascii=False, separators=(",", ":")) + ("\n" if raw.endswith("\n") else "")
    assert open(path).read() == raw
    open(path, "w").write(out)
    print("FIXED", key, changed)
