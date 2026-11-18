"""Tower crane 1:50: the generated solid shapes keep their long members (at least four times as long as wide) as filled quadrilaterals and draw the short and end-on members as pen strokes of their width (a short filled strip is no strip for the slender volume bars and the bar would follow the next wide strip), the strokes as extra shapes next to the quads."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
for view in ("elevation", "front", "plan"):
    ids = [x["id"] for x in d.val("asset", "representations", "50", "geometry", view) if x["id"].endswith("-stroke") and x.get("filled")]
    solids = [i[:-len("-stroke")] for i in ids]
    if not ids: continue
    res = lib.strokes(50, view, "^(" + "|".join(solids) + ")$", "qc2", "quadcopies")
    per = {}
    for r in res: per.setdefault(r["id"].split("#")[0], []).append(lib.classify(r))
    for i, sid in zip(ids, solids):
        items = per.get(sid, [])
        strips = [x[1] for x in items if x[0] == "strip"]
        pens = {}
        for x in items:
            if x[0] == "pen": pens.setdefault(round(x[2], 3), []).append(x[1])
        n = d.shape(50, view, i); o = d.obj(n)
        base = {k: v for k, v in o.items() if k not in ("penMm", "d", "filled")}
        shapes = []
        if strips: shapes.append(dict(base, filled=True, d=" ".join(strips)))
        for w, ps in sorted(pens.items()):
            shapes.append(dict(base, id=base["id"] if not shapes else base["id"] + "-dots%s" % ("" if len(pens) == 1 else str(w)), filled=False, d=" ".join(ps), penMm=round(w * 1000 / 50, 4)))
        d.replace_obj(n, shapes[0])
        for s in reversed(shapes[1:]): d.insert_after(n, s)
        print(view, sid, len(strips), {k: len(v) for k, v in pens.items()})
d.commit()
