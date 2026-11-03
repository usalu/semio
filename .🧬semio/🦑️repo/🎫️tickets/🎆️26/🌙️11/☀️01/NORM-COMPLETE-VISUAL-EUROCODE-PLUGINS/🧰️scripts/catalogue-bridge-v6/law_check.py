"""Python twin of the remaining clamping behaviour of the case forms (counts, angles, ignored roles, single pile or fastener spacing, retaining-wall base floor) against the demo documents: reports every (case, role) whose 1.25 scaling cannot follow exactly. The clamps of seepage head, soil-block water and rod, anchor height, opening hole, well radius and level, sheet-wall prop, footing and pile strata were removed from the builders in BRIDGE v6. Run with python3 -I law_check.py."""
import json, os, sys, pathlib
ARTS = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
UNIT = {"m": 1.0, "mm": 1e-3, "cm": 1e-2}
def find(root, parent, name="🔣️.json"):
    for dp, _, fs in os.walk(root):
        if "🗑️" in dp or "node_modules" in dp: continue
        if name in fs and os.path.basename(dp) == parent: yield os.path.join(dp, name)
def stored(root, parent, tail=None):
    return next((p for p in find(root, parent) if tail is None or tail in p), None)
bad = []
for art in sorted(ARTS.iterdir()):
    if not art.name.endswith(tuple(f"en{n}" for n in (1990, 1991, 1992, 1995, 1996, 1997))): continue
    geo = stored(art, "📐️geometry"); demo = stored(art, "📚️demo-document"); voc = stored(art, "📚️inputs")
    if not (geo and demo and voc): print("skip", art.name, bool(geo), bool(demo), bool(voc)); continue
    G = {r["id"]: r for r in json.load(open(geo))["entries"]} if isinstance(json.load(open(geo)).get("entries"), list) else json.load(open(geo))["entries"]
    V = json.load(open(voc))["inputs"]
    unit = {}
    for r in V: unit.setdefault(r["key"], {})[r.get("scope", "")] = r.get("unit", "")
    def u(entry, key):
        d = unit.get(key, {})
        for sc, un in d.items():
            if sc and entry in [s.strip() for s in sc.split(",")]: return un
        return d.get("", next(iter(d.values()), ""))
    doc = json.load(open(demo))
    for c in doc["cases"]:
        g = G.get(c["entryId"])
        if not g: continue
        N = {n["id"]: n["value"] for n in c["numbers"]}
        form = g["form"]; aux = g.get("aux", {})
        def m(key):
            if not key or key not in N: return None
            f = UNIT.get(u(c["entryId"], key))
            return N[key] * f if f else None
        def count(role, d=1.0):
            k = aux.get(role); return d if not k or k not in N else min(max(round(N[k]), 1), 12)
        def rep(role, why): bad.append((art.name, c["entryId"], form, role, why))
        for role, key in [("width", g["width"]), ("length", g["length"]), ("height", g["height"])] + list(aux.items()):
            if not key or key not in N: continue
            v = m(key)
            if role in ("count", "rows", "columns"):
                n = min(max(N[key], 1), 12)
                if round(n * 1.25) == round(n) or round(min(max(N[key] * 1.25, 1), 12)) == round(n): rep(role, f"count {N[key]} cannot change by 1.25 (round/clamp)")
                continue
            if role in ("angle", "ratio"):
                if role == "angle" and u(c["entryId"], key) in ("°", "deg"):
                    import math; r = math.radians(N[key])
                    if not (0.05 <= r * 1.25 <= 1.5) or r < 0.05 or r > 1.5: rep(role, f"angle {N[key]} deg clamps (0.05..1.5 rad)")
                continue
            if v is None: 
                if role not in ("layers",): continue
            if form in ("Gauge", "Symbol") and role in ("width", "length", "height"): rep(role, "ignored by the form"); continue
            if form in ("Cylinder", "Pile", "Well") and role == "length": rep(role, "ignored by the form"); continue
            if v is not None and not (0.01 <= v <= 40): continue
            if False:
                if form == "Footing":
                    b = m(g["width"]) or 2.0; t = m(g["height"]) or 0.5; emb = m(aux.get("embed")) or 1.0
                    depth = emb + t + 1.5 * max(b, 2.0)
                else:
                    depth = 1.25 * (m(g["height"]) or 8.0)
                ks = [k for k in N if k.rstrip("0123456789") == key.rstrip("0123456789") and k[len(key.rstrip("0123456789")):].isdigit()]
                rest = sum(m(k) or 0 for k in ks if k != key)
                if (v + rest < depth) != (1.25 * v + rest < depth): rep(role, f"top-up layer appears/disappears: sum {v+rest:.2f} depth {depth:.2f}")
            if form == "Pile" and role == "spacing":
                if count("rows") * count("columns") <= 1: rep(role, "single pile has no spacing")
            if form == "Fastener" and role == "spacing":
                if count("count", 2.0) <= 1: rep(role, "single fastener has no spacing")
            if form == "RetainingWall" and role == "base" and v is not None:
                t = m(g["width"]) or 0.4
                if v <= t + 0.2: rep(role, "base at floor t+0.2")
for b in bad: print(*b)
print(len(bad), "findings")
