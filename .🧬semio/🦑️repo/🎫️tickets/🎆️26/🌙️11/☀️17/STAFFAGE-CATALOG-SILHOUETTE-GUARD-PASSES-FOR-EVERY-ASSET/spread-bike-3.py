import sys, json, subprocess; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
sp = lib.sp
lim = float(sys.argv[1])
P = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/city-bicycle/🔣️.json"
h = json.loads(subprocess.check_output(["git", "-C", "/Users/ueli/Documents/semio", "show", "3cde99d423d:" + P]))["asset"]
cur = {s["id"]: s for s in h["representations"]["200"]["geometry"]["plan"] if s["id"].startswith("sweep")}
d = lib.mv.D("🚜️vehicles/city-bicycle")
for sid, z in (("handlebar-l", 0.31), ("handlebar-r", -0.31)):
    d.setv(sp.child(sp.child(d.shape(200, "plan", sid), "to"), "z"), z)
for sid in ("sweep-l", "sweep-r"):
    d.replace_obj(d.shape(200, "plan", sid), cur[sid])
d.commit()
d = lib.mv.D("🚜️vehicles/city-bicycle")
for sid, sg in (("sweep-l", 1), ("sweep-r", -1)):
    for i, p in enumerate(d.val("asset", "representations", "200", "geometry", "plan", [k for k, s in enumerate(d.val("asset", "representations", "200", "geometry", "plan")) if s["id"] == sid][0])["points"]):
        if abs(p[1]) > lim: lib.pt(d, 200, "plan", sid, i, 1, sg * lim)
d.commit()
