"""🧲️ The proposals of `suggest.ts` that a gate log asks for: `python3 pick.py <gate.log> <suggest.json> [<suggest-all.json>]` prints as JSON the proposed ops whose scale and bone carry a hard `sync-bone-*` finding in the log (the bone of a proposal is read from the op or looked up by shape id in the findings' twin lines), and on stderr the hard findings no proposal answers. A first draft of `ops/<id>.json`, to be read and decided op by op."""
import json, re, sys
want = {}
scale = code = None
tier = None
twins = {}
for line in open(sys.argv[1]):
    m = re.match(r"^\S+ 1:(\d+) (HARD|twin|reported) (\S+) ×", line)
    if m: scale, tier, code = m.group(1), m.group(2), m.group(3); continue
    m = re.match(r"^    (\S+) ×(\d+) (\d+) mm", line)
    if not m: continue
    key = m.group(1)
    if tier == "HARD": want.setdefault((scale, key), []).append(f"{code} {m.group(3)} mm")
    if tier == "twin" and "/" in key: twins.setdefault((scale, key.split("/")[1]), key.split("/")[0])
ops = []
answered = set()
for path in sys.argv[2:]:
    for op in json.load(open(path)):
        bone = op.get("bone") or twins.get((op["scale"], op["id"]))
        if bone is None or (op["scale"], bone) not in want: continue
        if any(o["scale"] == op["scale"] and o["id"] == op["id"] and o["op"] == op["op"] for o in ops): continue
        codes = want[(op["scale"], bone)]
        fits = any(("height" in c and op["op"] in ("front-sweep", "extrude", "copy")) or ("forward" in c and op["op"] in ("plan-revolve", "copy", "extrude", "plan-sweep")) or ("lateral" in c) for c in codes)
        if not fits: continue
        ops.append({**op, "for": "; ".join(codes)})
        answered.add((op["scale"], bone))
print("[\n" + ",\n".join("  " + json.dumps(op, ensure_ascii=False) for op in ops) + "\n]")
for key, codes in sorted(want.items()):
    if key not in answered: print(f"unanswered 1:{key[0]} {key[1]}: {'; '.join(codes)}", file=sys.stderr)
