import sys, importlib, pickle, json
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
part, cutters, _ = pickle.load(open("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/rb2/capture.pkl", "rb"))
def enc(p):
    return {"vertices": [[str(c) for c in v] for v in p["mesh"]["vertices"]], "faces": p["mesh"]["faces"]}
out = {"stock": enc(part), "cutter": enc(cutters[1]), "comparison": {"volume": 1e-12},
       "provenance": "Roof gravel weathering cell cut by GUARD-RM-2 beside RES-FIN-ROOF-GRAVEL-SL-R-705; the cutter is an exact sliver whose face 2 is wound inward."}
json.dump(out, open("C:/git/semio/" + sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=2)
