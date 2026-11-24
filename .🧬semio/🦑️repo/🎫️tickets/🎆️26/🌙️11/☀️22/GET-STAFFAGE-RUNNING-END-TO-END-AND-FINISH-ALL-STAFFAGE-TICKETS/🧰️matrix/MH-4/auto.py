"""🤖️ First draft of the ops of an asset from a gate log: `python3 auto.py <asset.json> <gate log>… > ops/<id>.json`. For every bone a hard `sync-bone-*` line names at a scale, the shape that is alone on the bone gets the op its construction and the kind of the bone call for (the recipe of `multiview-MR.md` 4.1 with the ops of `./fit.ts`): a limb (thigh, shank, arm, forearm, neck, the dog's leg bones and tail) `front-sweep` and `plan-revolve`, or `copy` of a revolved elevation twin; a foot, toe or paw `extrude` in front and plan over the lateral range it had (`"taper": false`); a hand whose elevation is a solid `copy` to front and plan, a flat one `extrude` like a foot; only the views a finding names are touched (`sync-bone-height` the front, `sync-bone-forward` the plan, `sync-bone-lateral` both); a trunk part (pelvis, lumbar, torso, belly, head) `prism` in the views where it is flat. A flat elevation shape on a spread or twisted bone gets `elevation-solid` first. The draft is read, corrected by hand and proven with `try.sh`; bones it has no rule for are printed to stderr."""
import json, re, sys, collections
asset = json.load(open(sys.argv[1]))["asset"]
named = collections.defaultdict(lambda: collections.defaultdict(set))
for path in sys.argv[2:]:
    scale = None
    hard = False
    for line in open(path):
        if not line.startswith("    "):
            m = re.match(r"\S+ 1:(\d+) HARD (sync-bone-\w+)", line)
            hard = m is not None
            scale, code = (m.group(1), m.group(2)) if m else (None, None)
        elif hard:
            named[scale][line.split()[0]].update({"sync-bone-height": ["front"], "sync-bone-forward": ["plan"], "sync-bone-lateral": ["front", "plan"]}[code])
parents = {b["id"]: b.get("parent") for b in asset["rig"]["bones"]}
spread = set()
def visit(n):
    if isinstance(n, list):
        for x in n: visit(x)
    elif isinstance(n, dict):
        for k, v in n.items():
            if k == "bones" and isinstance(v, dict):
                for b, o in v.items():
                    if isinstance(o, dict) and any(abs(o.get(f, 0) or 0) >= 1 for f in ("abductionDeg", "twistDeg")): spread.add(b)
            visit(v)
visit(asset["poses"]); visit(asset.get("clips"))
def spreads(b):
    while b is not None:
        if b in spread: return True
        b = parents.get(b)
    return False
LIMB = re.compile(r"(^|-)(thigh|shank|arm|forearm|upper-arm|neck|humerus|metatarsus|metacarpus|tail|scapula)(-|$)")
FOOT = re.compile(r"(^|-)(foot|toe|paw)(-|$)")
HAND = re.compile(r"(^|-)hand(-|$)")
TRUNK = re.compile(r"(^|-)(pelvis|lumbar|torso|belly|head|thorax)(-|$)")
ops = []
for scale, bones in sorted(named.items(), key=lambda kv: int(kv[0])):
    g = asset["representations"][scale]["geometry"]
    for bone in sorted(bones):
        shapes = {v: [s for s in g[v] if s["bone"] == bone and s.get("role") != "ground"] for v in ("elevation", "front", "plan")}
        ids = set(s["id"] for v in shapes for s in shapes[v] if len(shapes[v]) == 1)
        e = shapes["elevation"][0] if len(shapes["elevation"]) == 1 else None
        if e is None or e["id"] not in ids:
            print(f"1:{scale} {bone}: no single elevation twin, by hand", file=sys.stderr); continue
        f = next((s for s in shapes["front"] if s["id"] == e["id"]), None) if len(shapes["front"]) == 1 else None
        p = next((s for s in shapes["plan"] if s["id"] == e["id"]), None) if len(shapes["plan"]) == 1 else None
        base = {"scale": scale, "id": e["id"]}
        if "front" not in bones[bone]: f = None
        if "plan" not in bones[bone]: p = None
        kind = lambda s: None if s is None else ("flat" if "solid" not in s else ("rev" if s["solid"]["kind"] == "revolved" else "sw" + s["solid"].get("sweep", "?")))
        why = f"{bone}: {kind(e)}/{kind(f)}/{kind(p)}"
        if kind(e) == "rev":
            to = [v for v, s in (("front", f), ("plan", p)) if s is not None and json.dumps([s.get("points"), s.get("solid")]) != json.dumps([e.get("points"), e.get("solid")])]
            if to: ops.append({"op": "copy", **base, "from": "elevation", "to": to, "why": why + ": one revolved body"})
            continue
        if HAND.search(bone):
            if kind(e) != "flat":
                to = [v for v, s in (("front", f), ("plan", p)) if s is not None]
                ops.append({"op": "copy", **base, "from": "elevation", "to": to, "why": why + ": the body of the elevation in every view"})
            else:
                for v, x in (("front", f), ("plan", p)):
                    if kind(x) == "flat": ops.append({"op": "extrude", **base, "view": v, "taper": False, "why": why + ": a hand that swings with the arm, the elevation outline swept laterally"})
                    elif x is not None: print(f"1:{scale} {bone} {v}: {kind(x)}, by hand", file=sys.stderr)
            continue
        if kind(e) == "flat" and spreads(bone): ops.append({"op": "elevation-solid", **base, "why": why + ": poses spread or twist the bone"})
        if FOOT.search(bone):
            for v, s in (("front", f), ("plan", p)):
                if kind(s) == "flat": ops.append({"op": "extrude", **base, "view": v, "taper": False, "why": why + ": a foot that rolls, the elevation outline swept laterally"})
                elif s is not None: print(f"1:{scale} {bone} {v}: {kind(s)}, by hand", file=sys.stderr)
        elif LIMB.search(bone):
            twin = next((x for x in g["plan"] if x["bone"] == bone and x["id"] == e["id"]), None)
            if kind(f) in ("flat", "swv"): ops.append({"op": "front-sweep", **base, "why": why + ": prism along v fitted to the elevation outline"})
            elif kind(f) == "rev" and kind(twin) == "rev":
                ops.append({"op": "plan-revolve", **base, "range": "stretch", "why": why + ": the revolved plan body stretched to the length of the elevation outline"})
                ops.append({"op": "copy", **base, "from": "plan", "to": ["front"], "why": why + ": the same revolved body in the front"})
                p = None
            elif f is not None: print(f"1:{scale} {bone} front: {kind(f)}, by hand", file=sys.stderr)
            if kind(p) in ("flat", "rev"): ops.append({"op": "plan-revolve", **base, "why": why + ": revolved about the bone axis"})
            elif p is not None: print(f"1:{scale} {bone} plan: {kind(p)}, by hand", file=sys.stderr)
        elif TRUNK.search(bone):
            for v, s in (("front", f), ("plan", p)):
                if kind(s) == "flat": ops.append({"op": "prism", **base, "view": v, "why": why + ": prism that keeps the outline"})
                elif s is not None: print(f"1:{scale} {bone} {v}: {kind(s)}, by hand", file=sys.stderr)
        else: print(f"1:{scale} {bone}: no rule ({why}), by hand", file=sys.stderr)
print(json.dumps(ops, indent=1))
