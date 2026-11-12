"""Targeted shape fixes in catalog assets (idempotent), through catalog_edit: 
 - ellipse shapes that still carry the rect members x, y, w, h (chalkboard): the four stale members are removed;
 - rect shapes of zero width or height (a line): an open two-point poly of the same segment, unfilled (playground-swing, excavator);
 - shapes without tone (tower-server): tone 0.3 like the neighbouring detail lines;
 - `noJoin` (vegetation): a marker of an earlier authoring script that no code reads: removed.
Usage: catalog-fix-shapes.py <category-folder/slug> <fix>... where fix in stale-rect, zero-rect, tone, nojoin."""
import re, sys
sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from catalog_edit import S, edit
spec, fixes = sys.argv[1], set(sys.argv[2:])
path = S + spec + "/🔣️.json"
total = 0
if "stale-rect" in fixes:
    n = edit(path, lambda t: '"kind": "ellipse"' in t and '"w":' in t, lambda o: {k: v for k, v in o.items() if k not in ("x", "y", "w", "h")} if o.get("kind") == "ellipse" and all(k in o for k in ("x", "y", "w", "h")) else None)
    print(spec, "stale rect members removed from", n, "ellipses"); total += n
if "zero-rect" in fixes:
    def f(o):
        if o.get("kind") != "rect" or not (o.get("w") == 0 or o.get("h") == 0) or "bone" not in o: return None
        x, y, w, h = o["x"], o["y"], o["w"], o["h"]
        new = {k: v for k, v in o.items() if k not in ("kind", "x", "y", "w", "h", "r", "filled")}
        new = {**{k: v for k, v in new.items() if k in ("id", "bone", "role", "z")}, "filled": False, **{k: v for k, v in new.items() if k not in ("id", "bone", "role", "z")}, "kind": "poly", "points": [[x, y], [x + w, y + h]], "closed": False}
        return new
    n = edit(path, lambda t: '"kind": "rect"' in t and re.search(r'"(w|h)": 0[,\n]', t) is not None, f)
    print(spec, "zero-size rects turned into lines:", n); total += n
if "tone" in fixes:
    n = edit(path, lambda t: '"bone"' in t and '"role"' in t and '"tone"' not in t and re.search(r'"kind": "(path|ellipse|rect|poly|capsule)"', t) is not None, lambda o: {**o, "tone": 0.3} if "tone" not in o and "bone" in o and "role" in o else None)
    print(spec, "tone added to", n); total += n
if "nojoin" in fixes:
    n = edit(path, lambda t: '"noJoin"' in t, lambda o: {k: v for k, v in o.items() if k != "noJoin"} if "noJoin" in o else None)
    print(spec, "noJoin removed from", n); total += n
