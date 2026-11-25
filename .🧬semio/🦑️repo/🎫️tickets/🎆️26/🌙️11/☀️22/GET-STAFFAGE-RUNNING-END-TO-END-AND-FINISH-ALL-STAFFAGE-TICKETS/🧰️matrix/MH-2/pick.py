"""🧺️ Picks the ops of an asset from the proposals of `../MR/suggest.ts` by what the gate reports: `python3 pick.py <gate.log> <asset.json> <suggest.json> > ops/<id>.json` keeps the proposals whose bone has a hard finding at that scale on the axis the op's view takes part in (front: height and lateral, plan: forward and lateral), decides the proposals marked for review (plan of the neck or a hand: `plan-revolve`, plan of a foot: `extrude`, plan of another upright part: `plan-prism`, front of a foot stays `extrude`), adds by the same rules an op for a per-bone finding `suggest.ts` has no proposal for (a part drawn as another kind of solid: limbs `plan-revolve` with `rebuild`, feet `extrude`, upright parts `plan-prism`, fronts `front-sweep`), and lists on stderr the hard findings nothing covers."""
import json, re, sys, collections
log, doc, sug = sys.argv[1:4]
a = json.load(open(doc))["asset"]
ops = json.load(open(sug))
hard = collections.defaultdict(set)
scale = code = None
for line in open(log):
    m = re.match(r"\S+ 1:(\d+) (HARD|twin|reported) (\S+) ", line)
    if m:
        scale, code = m.group(1), m.group(3) if m.group(2) == "HARD" else None
        continue
    m = re.match(r"    (\S+) ×(\d+) (\d+) mm @ (.*)$", line)
    if m and code: hard[(scale, m.group(1))].add(code.split("-")[-1] if code.startswith("sync-bone") else code)
VIEWS = {"front": {"height", "lateral"}, "plan": {"forward", "lateral"}}
def bone_of(op, view):
    for s in a["representations"][op["scale"]]["geometry"][view]:
        if s["id"] == op["id"] and ("bone" not in op or s["bone"] == op["bone"]): return s["bone"]
out, covered = [], set()
for op in ops:
    views = op.get("to") or [op.get("view") or ("plan" if op["op"].startswith("plan") else "front")]
    keep = []
    for view in views:
        bone = bone_of(op, view) or bone_of(op, "elevation")
        axes = hard.get((op["scale"], bone), set()) & VIEWS[view]
        if axes: keep.append(view); covered.add((op["scale"], bone, view))
    if not keep: continue
    op = dict(op); bone = bone_of(op, "elevation")
    if "to" in op: op["to"] = keep
    if op.get("review") and op["op"] == "plan-revolve" and re.search("foot|toe|shoe", bone): op.update(op="extrude", view="plan")
    elif op.get("review") and op["op"] == "plan-revolve" and not re.search("neck|hand", bone): op["op"] = "plan-prism"
    if re.search("foot|toe|shoe", bone) and op["op"] in ("front-sweep", "plan-revolve", "plan-prism"): op.update(op="extrude", view="front" if op["op"] == "front-sweep" else "plan")
    op.pop("why", None); op.pop("review", None)
    out.append(op)
def rule(sc, bone, view):
    geo = a["representations"][sc]["geometry"]
    own = [s for s in geo[view] if s["bone"] == bone and s.get("role") != "ground"]
    side = [s for s in geo["elevation"] if s["bone"] == bone and s.get("role") != "ground"]
    if len(own) != 1 or len(side) != 1 or own[0]["id"] != side[0]["id"]: return None
    shape, twin, base = own[0], side[0], {"scale": sc, "id": own[0]["id"]}
    kind = (shape.get("solid") or {}).get("kind"), (shape.get("solid") or {}).get("sweep")
    if (twin.get("solid") or {}).get("kind") == "revolved": return dict(base, op="copy", **{"from": "elevation", "to": [view]})
    if re.search("foot|toe|shoe", bone): return dict(base, op="extrude", view=view)
    if view == "front": return dict(base, op="front-sweep") if kind[0] is None or kind == ("extruded", "v") else None
    if re.search(LIMB + "|neck|hand", bone): return dict(base, op="plan-revolve", **({} if kind[0] in (None, "revolved") else {"rebuild": True}))
    return dict(base, op="plan-prism") if kind[0] is None or kind == ("extruded", "u") else None
LIMB = "(^|-)(arm|forearm|upper-arm|thigh|shank|leg|calf|finger|tail)(-|$)"
extra = []
for (sc, key), axes in sorted(hard.items()):
    for ax in sorted(axes):
        views = [v for v in VIEWS if ax in VIEWS[v]]
        if any((sc, key, v) in covered for v in views): continue
        op = rule(sc, key, views[0]) if len(views) == 1 and ax in ("height", "forward") else None
        if op is None: print("uncovered 1:%s %s %s" % (sc, key, ax), file=sys.stderr)
        else: extra.append(op); covered.add((sc, key, views[0]))

print("[\n" + ",\n".join("  " + json.dumps(o) for o in out + extra) + "\n]")
