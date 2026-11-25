"""🗒️ Proposes plan entries for `solve.ts` from a gate log: `python3 planof.py <gate.log> <doc.json> [--fixed <ops.json>]` prints, for every bone a hard `sync-bone-*` finding names at a scale, the entry `{ "op": "volume", scale, id, "as": "auto" }` with the id of the single shape the bone draws (the ops of `--fixed` come first, each marked `"fixed": true`), and on stderr the hard findings that are no bone (whole-figure sync, anchors) for the fixer to plan by hand."""
import json, re, sys
log, doc = sys.argv[1], sys.argv[2]
a = json.load(open(doc))["asset"]
out = []
if "--fixed" in sys.argv: out += [{**op, "fixed": True} for op in json.load(open(sys.argv[sys.argv.index("--fixed") + 1]))]
scale, hard, seen = None, False, set()
for line in open(log):
    m = re.match(r"\S+ 1:(\d+) (HARD|reported|twin) (\S+)", line)
    if m: scale, hard, code = m[1], m[2] == "HARD", m[3]; continue
    m = re.match(r"    (\S+) ×(\d+) (\d+) mm", line)
    if not (m and hard): continue
    key = m[1]
    if not code.startswith("sync-bone-"): print(f"by hand: 1:{scale} {code} {key} ×{m[2]} {m[3]} mm", file=sys.stderr); continue
    ids = {view: [s["id"] for s in a["representations"][scale]["geometry"][view] if s.get("bone") == key and s.get("role") != "ground"] for view in ("elevation", "front", "plan")}
    one = {i for l in ids.values() if len(l) == 1 for i in l}
    if len(one) != 1: print(f"by hand: 1:{scale} bone {key} draws {ids}", file=sys.stderr); continue
    if (scale, key) in seen: continue
    seen.add((scale, key)); out.append({"op": "volume", "scale": scale, "id": one.pop(), "as": "auto"})
print("[\n" + ",\n".join("  " + json.dumps(o) for o in out) + "\n]")
