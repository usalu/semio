import importlib.util, os, json, sys
R = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/"
spec = importlib.util.spec_from_file_location("e", R + "sil-edit.py"); e = importlib.util.module_from_spec(spec); sys.path.insert(0, R); spec.loader.exec_module(e)
def sections(d, scale, sid, fn):
    n = e.V(d, scale, sid); secs = e.s.child(n, "sections")
    for i, it in enumerate(secs["items"]):
        cur = json.loads(d.t[it["s"]:it["e"]])
        for k, v in fn(i, cur).items():
            d.setv(e.s.child(it, k), v)
def run(slug, f):
    d = e.doc(slug); f(d); d.commit()
