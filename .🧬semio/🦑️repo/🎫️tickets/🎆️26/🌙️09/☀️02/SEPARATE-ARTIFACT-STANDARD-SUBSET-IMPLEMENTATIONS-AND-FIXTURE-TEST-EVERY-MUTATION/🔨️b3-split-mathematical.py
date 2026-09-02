import re, sys

ORIG = "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🧪️tests/mutate-mathematical-1/🦀️.rs"
text = open(ORIG, encoding="utf-8").read()

# Extract each match arm "KIND" => Vector { ... },
arm_re = re.compile(r'        "([a-z-]+)" => Vector \{\n(.*?\n        \},\n)', re.DOTALL)
arms = {}
for m in arm_re.finditer(text):
    kind, body = m.group(1), m.group(2)
    arms[kind] = f'        "{kind}" => Vector {{\n{body}'

print("found arms:", sorted(arms.keys()))
assert len(arms) == 15, len(arms)

subsets = {
    "graph": ["change-graph-directed", "update-graph-algorithm", "replace-graph", "create-node", "delete-node", "delete-nodes", "change-node-label", "move-node", "connect-nodes", "disconnect-nodes"],
    "geometry": ["replace-points", "insert-point", "remove-point", "move-point"],
    "equation": ["change-coefficient"],
}
unobservable = {"change-graph-directed", "update-graph-algorithm", "replace-graph", "create-node", "delete-node", "delete-nodes", "change-node-label", "move-node", "connect-nodes", "disconnect-nodes", "replace-points", "remove-point", "move-point"}

import json
out = {}
for subset, kinds in subsets.items():
    body = "".join(arms[k] for k in kinds)
    # rewrite include_str! paths: from "../../🏅️standards/🔖️1/🪆️subsets/✳️<subset>/" to "../../"
    old_prefix = f"../../🏅️standards/🔖️1/🪆️subsets/✳️{subset}/"
    body2 = body.replace(old_prefix, "../../")
    # sanity: no other subset prefix should remain
    assert "🪆️subsets" not in body2, (subset, body2[:200])
    out[subset] = body2

with open("/tmp/_math_arms.json", "w", encoding="utf-8") as f:
    json.dump(out, f, ensure_ascii=False)
print("wrote arms for", list(out.keys()))
