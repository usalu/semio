"""Date palm: the 1:50 frond plugs are removed; the plates and the volume ribbons share their profile and overlap at the joints like the volume."""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib")
d = mv.D("🌳️vegetation/date-palm"); a = d.val("asset"); n = 0
def idof(it): return json.loads(d.t[[v for k, v in it["pairs"] if k == "id"][0]["s"]:[v for k, v in it["pairs"] if k == "id"][0]["e"]])
for sc, rep in a["representations"].items():
    for view, shapes in rep["geometry"].items():
        for it in d.at("asset", "representations", sc, "geometry", view)["items"]:
            if "-blade-plug" in idof(it): d.delete_item(it); n += 1
d.commit(); print("removed", n)
