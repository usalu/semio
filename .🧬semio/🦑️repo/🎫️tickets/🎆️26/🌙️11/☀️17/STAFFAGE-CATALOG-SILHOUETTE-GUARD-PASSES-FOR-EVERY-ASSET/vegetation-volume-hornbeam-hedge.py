import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib"); sp = mv.sp
d = mv.D("🌳️vegetation/hornbeam-hedge")
for sc in ("50", "100"):
    for n in range(1, 7):
        top = d.val("asset", "volumes", sc, "body", *[str(i) for i, it in enumerate(d.val("asset", "volumes", sc, "body")) if it["id"] == "block-%d" % n], "sections")[-1]["at"][0]
        for i, it in enumerate(d.val("asset", "volumes", sc, "body")):
            if it["id"].startswith("crown-%d" % n):
                at = it["sections"][0]["at"][0]
                new = round(min(at, top - 0.003), 5)
                print(sc, it["id"], at, "->", new, "block top", top)
                if new != at:
                    node = sp.walk(d.root, ["asset", "volumes", sc, "body", str(i), "sections", "0", "at", "0"]); d.setv(node, new)
d.commit()
