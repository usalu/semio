import sys, json, subprocess; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
sp = lib.sp
P = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/city-bicycle/🔣️.json"
h = json.loads(subprocess.check_output(["git", "-C", "/Users/ueli/Documents/semio", "show", "395993ad470:" + P]))["asset"]
old = {s["id"]: s for s in h["representations"]["200"]["geometry"]["plan"] if s["id"].startswith("sweep")}
d = lib.mv.D("🚜️vehicles/city-bicycle")
for sid, z in (("handlebar-l", 0.26), ("handlebar-r", -0.26)):
    d.setv(sp.child(sp.child(d.shape(200, "plan", sid), "to"), "z"), z)
for sid in ("sweep-l", "sweep-r"):
    d.replace_obj(d.shape(200, "plan", sid), old[sid])
d.commit()
