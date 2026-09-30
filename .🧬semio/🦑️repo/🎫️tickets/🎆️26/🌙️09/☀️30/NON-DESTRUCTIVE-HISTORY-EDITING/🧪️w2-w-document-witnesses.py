"""📨️ W2-W-document: authors the payload-only wire witnesses (design §11) —
`<owner>/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json` — for every in-scope leaf the
`schema mutation-payloads` lint reports `unwitnessed`. A witness is the aggregate wire of one editable op of the leaf.

Two sources:
  * `generate`: a schema-first instance of the leaf payload schema (all `$ref`s resolved across the repo's `$id` documents,
    every member of the top two object levels and required members below, the non-null branch of a nullable, the tag-matching or simplest branch of a union) with
    per-leaf overrides for values that carry meaning (`OVERRIDES`), wrapped in the aggregate layout;
  * `borrow`: an op or snapshot taken verbatim from a committed sibling fixture (semio subset snapshots, subset ops).
Each witness is then checked by the lint (schema) and by the crate's derive-emitted `semio_payload_law_*` (Rust decode).

  python3 🧪️w2-w-document-witnesses.py <plan.json>      # plan: [{"owner", "leaf", "layout", "variant", "payload" | "generate"}]
"""
import json, os, pathlib, sys

REPO = pathlib.Path("/Users/ueli/Documents/semio")
SKIP = {"node_modules", "dist", "target", ".git", ".🧬semio", "🤖️generated"}


def index_documents():
    documents = {}
    for root in ("✏️s", "🧰️framework"):
        for directory, names, files in os.walk(REPO / root):
            names[:] = [name for name in names if name not in SKIP and not name.startswith(".")]
            if "🔣️.json" not in files:
                continue
            path = pathlib.Path(directory) / "🔣️.json"
            text = path.read_text()
            if '"$id"' not in text:
                continue
            try:
                document = json.loads(text)
            except ValueError:
                continue
            if isinstance(document, dict) and isinstance(document.get("$id"), str):
                documents[document["$id"]] = document
    return documents


class Generator:
    def __init__(self, documents, optional_depth=2):
        self.documents = documents
        self.optional_depth = optional_depth

    def resolve(self, reference, base):
        target, _, pointer = reference.partition("#")
        document = self.documents[target] if target else base
        node = document
        for segment in [part for part in pointer.split("/") if part]:
            node = node[segment.replace("~1", "/").replace("~0", "~")]
        return node, document

    def instance(self, schema, base, key="value", depth=0):
        if depth > 40:
            raise RecursionError(f"schema recursion at {key}")
        if "$ref" in schema:
            node, document = self.resolve(schema["$ref"], base)
            return self.instance(node, document, key, depth + 1)
        if "const" in schema:
            return schema["const"]
        if "enum" in schema:
            return schema["enum"][0]
        if "allOf" in schema:
            merged = {}
            for part in schema["allOf"]:
                value = self.instance(part, base, key, depth + 1)
                if isinstance(value, dict):
                    merged.update(value)
            local = {k: v for k, v in schema.items() if k != "allOf"}
            if local.get("properties") or local.get("required"):
                merged.update(self.instance(local, base, key, depth + 1))
            return merged
        for union in ("oneOf", "anyOf"):
            if union in schema:
                branches = [branch for branch in schema[union] if branch.get("type") != "null"]
                return self.instance(min(branches, key=lambda branch: self.weight(branch, base)), base, key, depth + 1)
        kind = schema.get("type")
        if isinstance(kind, list):
            kind = next(item for item in kind if item != "null")
        if kind == "object" or "properties" in schema:
            properties = schema.get("properties", {})
            members = list(properties) if depth <= self.optional_depth else schema.get("required", [])
            return {name: self.instance(properties.get(name, {}), base, name, depth + 1) for name in members}
        if kind == "array":
            item = schema.get("items", {})
            if isinstance(item, list):
                return [self.instance(entry, base, key, depth + 1) for entry in item]
            return [self.instance(item, base, key, depth + 1) for _ in range(schema.get("minItems", 0))]
        if kind == "string":
            return key
        if kind in ("integer", "number"):
            low = schema.get("minimum", 0)
            return low + 1 if schema.get("exclusiveMinimum") == low else low
        if kind == "boolean":
            return False
        return None

    def weight(self, branch, base, depth=0):
        if "$ref" in branch and depth < 8:
            node, document = self.resolve(branch["$ref"], base)
            return self.weight(node, document, depth + 1)
        return len(json.dumps(branch)) + 1000 * len(branch.get("required", []))


def wrap(layout, variant, payload):
    if layout == "internal":
        return payload
    if layout == "adjacent":
        return {"mutation": variant, "payload": payload}
    if layout == "external":
        return {variant: payload}
    raise ValueError(layout)


def main(plan_path):
    plan = json.loads(pathlib.Path(plan_path).read_text())
    generator = Generator(index_documents()) if any("generate" in entry for entry in plan) else None
    for entry in plan:
        leaf = REPO / entry["leaf"]
        if "payload" in entry:
            payload = entry["payload"]
        else:
            schema = json.loads((leaf / "🧬️schema" / "🔣️.json").read_text())
            payload = generator.instance(schema, schema)
            payload.update(entry.get("generate") or {})
        target = REPO / entry["owner"] / "🧫️fixtures" / "🧬️mutations" / leaf.name / "🧾️wire-witness" / "🦠️mutation" / "🔣️.json"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(json.dumps(wrap(entry["layout"], entry.get("variant"), payload), ensure_ascii=False, indent=2) + "\n")
        print("witness", target.relative_to(REPO))


if __name__ == "__main__":
    main(sys.argv[1])
