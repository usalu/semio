"""🥚️ Marks the ops of the shapes `which.ts` names with `"hull": true` (the convex hull of the outline is swept): python3 hull.py <ops.json> <which.jsonl>; prints how many ops it marked."""
import json, sys

ops = json.load(open(sys.argv[1], encoding="utf8"))
marked = 0
for line in open(sys.argv[2], encoding="utf8"):
    if not line.startswith("{"):
        continue
    named = json.loads(line)
    for op in ops:
        if op.get("scale") == named["scale"] and op.get("view") == named["view"] and op.get("id") == named["id"] and op.get("bone", named.get("bone")) == named.get("bone") and not op.get("hull"):
            op["hull"] = True
            marked += 1
json.dump(ops, open(sys.argv[1], "w", encoding="utf8"), indent=1, ensure_ascii=False)
print(marked)
