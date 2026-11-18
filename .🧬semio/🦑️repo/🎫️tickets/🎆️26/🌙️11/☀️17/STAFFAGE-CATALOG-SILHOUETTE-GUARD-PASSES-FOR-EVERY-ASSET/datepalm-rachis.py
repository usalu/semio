"""Date palm: the revolved rachis outlines have no body in the volume and project differently from the plates; they are removed from every view."""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib")
d = mv.D("🌳️vegetation/date-palm"); a = d.val("asset"); n = 0
for sc, rep in a["representations"].items():
    for view, shapes in rep["geometry"].items():
        for s in shapes:
            if s["id"].endswith("-rachis") or s["id"].startswith("rachis-"):
                d.delete_item(d.shape(sc, view, s["id"]) if False else [it for it in d.at("asset", "representations", sc, "geometry", view)["items"] if json.loads(d.t[[v for k, v in it["pairs"] if k == "id"][0]["s"]:[v for k, v in it["pairs"] if k == "id"][0]["e"]]) == s["id"] and json.loads(d.t[[v for k, v in it["pairs"] if k == "bone"][0]["s"]:[v for k, v in it["pairs"] if k == "bone"][0]["e"]]) == s["bone"]][0]); n += 1
d.commit(); print("removed", n)
