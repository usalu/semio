import json, sys, os
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
BYTES = {"type": "array", "items": {"type": "integer", "minimum": 0, "maximum": 255}}
def nullable(node):
    t = node.get("type")
    if isinstance(t, str) and t != "null": node["type"] = [t, "null"]
    return node
def load(rel):
    p = os.path.join(ROOT, rel); t = open(p, encoding="utf-8").read(); return p, t, json.loads(t)
def save(p, t, d):
    n = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    if n != t: open(p, "w", encoding="utf-8").write(n); print("patched", p.split("🪆️subsets/")[1])
def rename(props, old, new):
    if old in props: props[new] = props.pop(old)
def doc_blocks(d):
    for branch in d["oneOf"]:
        props = branch["properties"]
        rename(props, "styleId", "style_id"); rename(props, "imageId", "image_id")
        for k in ("style_id", "language", "width", "height"):
            if k in props: nullable(props[k])
# document snapshot
p, t, d = load("📑️document/🧬️schema/📸️snapshot/🔣️.json")
for k in ("size", "font", "color", "link"): nullable(d["$defs"]["RunStyle"]["properties"][k])
nullable(d["properties"]["styles"]["items"]["properties"]["basedOn"])
doc_blocks(d["$defs"]["DocBlock"])
save(p, t, d)
# image snapshot
p, t, d = load("🖼️image/🧬️schema/📸️snapshot/🔣️.json")
d["properties"]["frames"]["items"]["properties"]["rgba8"] = dict(BYTES)
d["properties"]["icc"] = {"anyOf": [dict(BYTES), {"type": "null"}], "x-semio-state": "artifact"}
save(p, t, d)
# video snapshot: sample payloads are Vec<u8>
p, t, d = load("🎬️video/🧬️schema/📸️snapshot/🔣️.json")
def fix_bytes(node):
    if isinstance(node, dict):
        if node.get("contentEncoding") == "hex" and node.get("type") in ("string", ["string", "null"]):
            nullable_bytes = node.get("type") == ["string", "null"]
            extra = {k: v for k, v in node.items() if k.startswith("x-")}
            node.clear(); node.update({"anyOf": [dict(BYTES), {"type": "null"}]} if nullable_bytes else dict(BYTES)); node.update(extra)
        for v in list(node.values()): fix_bytes(v)
    elif isinstance(node, list):
        for v in node: fix_bytes(v)
fix_bytes(d)
save(p, t, d)
# presentation snapshot: text-box blocks are document blocks, referenced
p, t, d = load("📽️presentation/🧬️schema/📸️snapshot/🔣️.json")
DOC_ID = json.load(open(os.path.join(ROOT, "📑️document/🧬️schema/📸️snapshot/🔣️.json"), encoding="utf-8"))["$id"]
d["definitions"]["SlideShape"]["properties"]["blocks"] = {"type": "array", "items": {"$ref": f"{DOC_ID}#/$defs/DocBlock"}}
save(p, t, d)
# image diff: frame pixels and the ICC profile are Vec<u8> on the wire
p, t, d = load("🖼️image/🧬️schema/🔺️diff/🔣️.json")
d["properties"]["icc"] = {"anyOf": [dict(BYTES), {"type": "null"}]}
def fix_rgba(node):
    if isinstance(node, dict):
        if isinstance(node.get("properties"), dict) and "rgba8" in node["properties"]: node["properties"]["rgba8"] = dict(BYTES)
        for v in node.values(): fix_rgba(v)
    elif isinstance(node, list):
        for v in node: fix_rgba(v)
fix_rgba(d)
save(p, t, d)
# video diff: sample payloads are Vec<u8>
p, t, d = load("🎬️video/🧬️schema/🔺️diff/🔣️.json")
def fix_data(node):
    if isinstance(node, dict):
        props = node.get("properties")
        if isinstance(props, dict) and isinstance(props.get("data"), dict) and props["data"].get("type") in ("string", ["string", "null"]): props["data"] = dict(BYTES)
        for v in node.values(): fix_data(v)
    elif isinstance(node, list):
        for v in node: fix_data(v)
fix_data(d)
save(p, t, d)
