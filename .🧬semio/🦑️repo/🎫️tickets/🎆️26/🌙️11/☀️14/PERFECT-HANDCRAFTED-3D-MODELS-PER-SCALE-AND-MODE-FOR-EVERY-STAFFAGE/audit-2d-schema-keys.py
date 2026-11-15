import json, sys
S = sys.argv[1]
schema = json.load(open(S, encoding="utf-8"))
d = schema["$defs"]
def keys(node, depth=0):
    out = []
    if isinstance(node, dict):
        if "properties" in node:
            out.append(sorted(node["properties"].keys()))
            out.append(("required", node.get("required")))
        for k in ("oneOf", "anyOf"):
            if k in node:
                for i, sub in enumerate(node[k]):
                    out.append((k, i, keys(sub, depth + 1)))
    return out
for name in sys.argv[2:]:
    print("==", name)
    print(json.dumps(keys(d[name]), ensure_ascii=False, indent=None))
