import json, sys
S = sys.argv[1]
d = json.load(open(S, encoding="utf-8"))["$defs"]
LIM = int(sys.argv[2])
def brief(v):
    if isinstance(v, dict):
        o = {}
        for k in ("type", "enum", "const", "$ref", "minimum", "exclusiveMinimum", "maximum", "minItems", "maxItems", "default"):
            if k in v: o[k] = v[k]
        if "description" in v: o["description"] = v["description"][:LIM]
        return o
    return v
def dump(name, node, indent=""):
    print(f"{indent}{name}: {json.dumps(brief(node), ensure_ascii=False)[:1500]}")
    if isinstance(node, dict):
        if "properties" in node:
            print(f"{indent}  required={node.get('required')}")
            for k, v in node["properties"].items():
                dump(k, v, indent + "  ")
        if "items" in node and isinstance(node["items"], dict) and "properties" in node["items"]:
            print(f"{indent}  items.required={node['items'].get('required')}")
            for k, v in node["items"]["properties"].items():
                dump("items." + k, v, indent + "  ")
for name in sys.argv[3:]:
    print("=====", name)
    n = d[name]
    if "description" in n: print("desc:", n["description"][:LIM])
    if "properties" in n or "items" in n or "enum" in n or "oneOf" in n:
        if "oneOf" in n:
            for i, sub in enumerate(n["oneOf"]):
                print(f"-- oneOf[{i}] keys:", list(sub.get("properties", {}).keys()), "required:", sub.get("required"))
        else:
            print("required:", n.get("required"))
            for k, v in n.get("properties", {}).items(): dump(k, v, "  ")
