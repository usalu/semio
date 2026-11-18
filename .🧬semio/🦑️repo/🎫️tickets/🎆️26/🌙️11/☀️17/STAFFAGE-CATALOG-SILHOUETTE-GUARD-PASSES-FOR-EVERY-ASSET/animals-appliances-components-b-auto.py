"""Retraces every body shape of an animal from the volume solids of its bone: python3 auto.py <slug> <scales,> <views,> [extra "bone=bone+bone;..."] [skip ids,] [only ids,]"""
import sys, json, shutil, importlib.util, os
H = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/"
s = importlib.util.spec_from_file_location("r", H + "animals-appliances-components-b-retrace.py"); r = importlib.util.module_from_spec(s); s.loader.exec_module(r)
slug, scales, views = sys.argv[1:4]
extra = {}
if len(sys.argv) > 4 and sys.argv[4]:
    for it in sys.argv[4].split(";"):
        k, v = it.split("="); extra[k] = v.split("+")
skip = set(sys.argv[5].split(",")) if len(sys.argv) > 5 and sys.argv[5] else set()
only = set(sys.argv[6].split(",")) if len(sys.argv) > 6 and sys.argv[6] else set()
clip = set(sys.argv[7].split(",")) if len(sys.argv) > 7 and sys.argv[7] else set()
d = r.l.doc(slug)
shutil.copy(d.path, H + "🗑️generated/aac-b-snap-" + slug + ".json")
for sc in scales.split(","):
    for view in views.split(","):
        shapes = d.obj(d.at("asset", "representations", sc, "geometry", view))
        jobs = []
        for sh in shapes:
            if sh.get("role") != "body" or sh.get("kind") not in ("poly", "ellipse", "capsule", "path") or "bone" not in sh or sh.get("filled") is not True or sh.get("closed", True) is not True: continue
            if sh["id"] in clip:
                b = sh["bone"]
                jobs.append({"id": sh["id"], "sel": "&" + ",".join(["bone:" + b] + ["bone:" + x for x in extra.get(b, [])])}); continue
            if sh["id"] in skip or (only and sh["id"] not in only): continue
            b = sh["bone"]
            jobs.append({"id": sh["id"], "sel": ",".join(["bone:" + b] + ["bone:" + x for x in extra.get(b, [])])})
        res = r.trace_batch(slug, sc, view, jobs)
        for j in jobs:
            x = res.get(j["id"], {})
            if "err" in x: print("skip", sc, view, j["id"], x["err"]); continue
            r.apply_trace(d, sc, view, j["id"], x)
            print(sc, view, j["id"], x["info"].replace("[DEBUG] ", "")[:100])
d.commit()
