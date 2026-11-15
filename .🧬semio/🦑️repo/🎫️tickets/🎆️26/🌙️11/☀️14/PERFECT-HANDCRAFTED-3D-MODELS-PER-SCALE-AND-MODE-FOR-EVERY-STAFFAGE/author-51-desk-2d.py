import sys, copy
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_51_edit import edit_member
def plate_rect(sh):
    cx = sh["points"][0][0]; half = abs(sh["points"][1][0] - cx) / 2; mid = (sh["points"][0][0] + sh["points"][1][0]) / 2
    for k in ("points", "closed", "penMm"): sh.pop(k, None)
    sh.update({"kind": "rect", "x": round(mid - half, 4), "y": 0.0, "w": round(2 * half, 4), "h": 0.016, "closed": True, "filled": True})
def fn(reps):
    g100 = reps["100"]["geometry"]
    for view in g100:
        for sh in g100[view]:
            if sh["id"].endswith("-plate") and sh["kind"] == "poly": plate_rect(sh)
            if sh["id"] == "laptop" and view in ("elevation", "front") and sh["kind"] == "rect":
                sh["y"] = 0.027; sh["h"] = 0.02; sh.pop("penMm", None)
    SKIP200 = {"top-cable-tray", "monitor-base", "laptop-seam"}
    SKIP500 = SKIP200 | {"monitor-neck", "monitor-panel"} | {n for n in ("l-bracket", "r-bracket")}
    for scale, skip in (("200", SKIP200), ("500", SKIP500)):
        old = reps[scale]["geometry"]; new = {}
        for view in ("elevation", "front", "plan"):
            lst = []
            for sh in g100[view]:
                if sh["id"] in skip: continue
                s = copy.deepcopy(sh)
                if s.get("kind") != "rect" or "penMm" in s: s.pop("penMm", None)
                lst.append(s)
            if scale == "500":
                blocks = [copy.deepcopy(s) for s in old[view] if s["id"] == "monitor-block"]
                for b in blocks:
                    if view == "plan": b.update({"y": 0.13, "h": 0.04})
                    lst.append(b)
                if view == "plan": lst = [s for s in lst if s["id"] != "monitor-panel"]
            new[view] = lst
        reps[scale]["geometry"] = new
    return reps
edit_member("height-adjustable-desk", "representations", fn)
