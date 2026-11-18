"""✏️ Value edits of the three-views spread repairs on top of mvlib: set one number of a shape's points or fields, byte-stable."""
import sys, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib")
sp = mv.sp
def pt(d, scale, view, sid, i, j, value):
    node = d.shape(scale, view, sid)
    p = sp.child(node, "points")
    d.setv(sp.child(sp.child(p, i), j), value)
def fld(d, scale, view, sid, key, value):
    d.setv(sp.child(d.shape(scale, view, sid), key), value)
def shift(d, scale, view, sid, dx, dy):
    shapes = d.val("asset", "representations", str(scale), "geometry", view)
    for i, p in enumerate([x for x in shapes if x["id"] == sid][0]["points"]):
        pt(d, scale, view, sid, i, 0, round(p[0] + dx, 5))
        pt(d, scale, view, sid, i, 1, round(p[1] + dy, 5))
def head_raw(d, rev, scale, view, sid):
    import subprocess
    t = subprocess.run(["git", "-C", "/Users/ueli/Documents/semio", "show", f"{rev}:{d.path[len('/Users/ueli/Documents/semio/'):]}"], capture_output=True, text=True, encoding="utf8").stdout
    r = sp.parse(t)
    import json
    for it in sp.walk(r, ["asset", "representations", str(scale), "geometry", view])["items"]:
        for k, v in it["pairs"]:
            if k == "id" and json.loads(t[v["s"]:v["e"]]) == sid: return t[it["s"]:it["e"]]
    raise KeyError(sid)
def restore_shape(d, rev, scale, view, sid):
    d.rep(d.shape(scale, view, sid), head_raw(d, rev, scale, view, sid))
def drop(d, scale, view, sid):
    d.delete_item(d.shape(scale, view, sid))
def reinsert(d, rev, scale, view, sid):
    import json, subprocess
    t = subprocess.run(["git", "-C", "/Users/ueli/Documents/semio", "show", f"{rev}:{d.path[len('/Users/ueli/Documents/semio/'):]}"], capture_output=True, text=True, encoding="utf8").stdout
    items = json.loads(t)["asset"]["representations"][str(scale)]["geometry"][view]
    ids = [s["id"] for s in items]
    i = ids.index(sid)
    cur = {s["id"] for s in d.val("asset", "representations", str(scale), "geometry", view)}
    j = i - 1
    while ids[j] not in cur: j -= 1
    d.insert_after(d.shape(scale, view, ids[j]), items[i])
