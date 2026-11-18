"""Sets the lateral offset (third coordinate of every loft section `at`) and scales rx of solids by span edits. python3 humansa-toez.py <slug> <solidId=z,...> <rxFactor> [scales]"""
import importlib.util, json, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('s', O + 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
slug = sys.argv[1]
want = {kv.split("=")[0]: float(kv.split("=")[1]) for kv in sys.argv[2].split(",")}
factor = float(sys.argv[3])
scales = sys.argv[4].split(",") if len(sys.argv) > 4 else ["500", "200", "100", "50"]
d = s.Doc("👤️humans/" + slug)
for sc in scales:
    for it in s.walk(d.root, ["asset", "volumes", sc, "body"])["items"]:
        sid = None
        for k, v in it["pairs"]:
            if k == "id": sid = json.loads(d.t[v["s"]:v["e"]])
        if sid not in want: continue
        for k, v in it["pairs"]:
            if k == "sections":
                for sec in v["items"]:
                    at = next(vv for kk, vv in sec["pairs"] if kk == "at")
                    z = at["items"][2]
                    d.setv(z, want[sid])
                    rx = next(vv for kk, vv in sec["pairs"] if kk == "rx")
                    old = float(d.t[rx["s"]:rx["e"]])
                    d.setv(rx, round(old * factor, 5))
d.commit()
