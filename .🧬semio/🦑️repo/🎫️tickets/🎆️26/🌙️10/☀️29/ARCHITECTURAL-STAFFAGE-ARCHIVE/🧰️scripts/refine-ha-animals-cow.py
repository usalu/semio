import importlib.util, json, re
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/"
spec = importlib.util.spec_from_file_location("lib", T + "🧰️scripts/refine-ha-animals-lib.py")
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
D = lib.Doc(); A = "dairy-cow"
D.edit(A, "anchors", lambda o: o["id"] in ("contact-hn", "contact-hf", "contact-fn", "contact-ff"), lambda o: {**o, "y": -0.052})
def narrow(o):
    if o["id"] == "udder":
        pts = re.findall(r"-?\d+\.?\d*", o["d"]); v = list(map(float, pts)); out = []
        toks = o["d"].split(); res = []; ys = False
        for t in toks:
            if re.match(r"^[MLCZ]$", t): res.append(t); ys = False; continue
            x = float(t)
            res.append(f"{x:g}" if not ys else f"{round(x * 0.42, 4):g}"); ys = not ys
        o["d"] = " ".join(res)
    elif o["id"] == "teat-front": o["y0"] = o["y1"] = 0.05
    elif o["id"] == "teat-rear": o["y0"] = o["y1"] = -0.075
    return o
D.edit(A, "geometry", lambda o: o["id"] in ("udder", "teat-front", "teat-rear"), narrow)
D.edit(A, "poses", lambda o: o["id"] == "lying-resting", lambda o: {**o, "bones": {**o["bones"], "tail-1": {"angleDeg": 30}, "tail-2": {"angleDeg": 40}, "tail-3": {"angleDeg": 40}}})
D.save()
