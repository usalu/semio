"""🔭 Observed values of a snapshot schema's numeric fields in every committed JSON document of the artifact (fixtures, assets, examples): a document is a dict that holds every `required` key of a record, walked by the schema `$ref`s. Standard library only."""
import json, os

SKIP_DIRS = {"node_modules", "target", "🗑️generated", "📦️packages", "json-schema-twin", "🪶️sqlite"}


def resolve(doc, node):
    key = None
    while isinstance(node, dict) and "$ref" in node:
        key = node["$ref"].split("/")[-1]
        node = doc["$defs"].get(key, {})
    return key, (node if isinstance(node, dict) else {})


def walk_value(doc, node, value, owner, prop, out):
    key, node = resolve(doc, node)
    alt = node.get("anyOf") or node.get("oneOf")
    if alt:
        for branch in alt:
            walk_value(doc, branch, value, owner, prop, out)
        return
    kind = node.get("type")
    if kind in ("number", "integer") and isinstance(value, (int, float)) and not isinstance(value, bool):
        out.setdefault((owner, prop), []).append(value)
    elif kind == "array" and isinstance(value, list):
        for item in value:
            walk_value(doc, node.get("items", {}), item, owner, prop, out)
    elif isinstance(value, dict) and "properties" in node:
        here = key or "$root"
        for child_key, child in node["properties"].items():
            if child_key in value:
                walk_value(doc, child, value[child_key], here, child_key, out)


def record_defs(doc):
    return {name: node for name, node in doc.get("$defs", {}).items() if node.get("required")}


def find_roots(doc, data, hits):
    if isinstance(data, dict):
        if set(doc.get("required", [])) <= set(data):
            hits.append(data)
            return
        for value in data.values():
            find_roots(doc, value, hits)
    elif isinstance(data, list):
        for value in data:
            find_roots(doc, value, hits)


def observed(artifact_dir, doc):
    """{(def key, property): [values]} and the list of documents read."""
    out, used = {}, []
    for base, dirs, files in os.walk(artifact_dir):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        if "🧬️schema" in base.split(os.sep) and "🧪️tests" not in base:
            continue
        for name in files:
            if not name.endswith(".json") or name.endswith("schema.json"):
                continue
            path = os.path.join(base, name)
            try:
                if os.path.getsize(path) > 40_000_000:
                    continue
                data = json.load(open(path, encoding="utf8"))
            except Exception:
                continue
            hits = []
            find_roots(doc, data, hits)
            for root in hits:
                walk_value(doc, doc, root, "$root", "", out)
                used.append(path)
    return out, used
