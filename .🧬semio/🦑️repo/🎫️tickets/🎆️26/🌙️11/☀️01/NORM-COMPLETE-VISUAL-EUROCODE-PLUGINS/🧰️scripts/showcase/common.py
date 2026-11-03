"""Shared helpers of the showcase-document generators (python3 -I). A showcase document is built from committed typed fixtures with leaf edits only, validated against the artifact's JSON schema (the language-agnostic law: every entity collection populated, ids unique, every field typed) and written next to the artifact's other fixtures."""
import copy, glob, json, pathlib, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any"


def artifact(folder):
    return next(ROOT.glob(f"{folder}/{SUB}"))


def load(path):
    return json.loads(pathlib.Path(path).read_text(encoding="utf-8"))


def defs(root):
    return root.get("$defs", {})


def validate(node, schema, root, path="$"):
    if "$ref" in schema:
        return validate(node, defs(root)[schema["$ref"].rsplit("/", 1)[1]], root, path)
    errors = []
    kind = schema.get("type")
    if "enum" in schema and node not in schema["enum"]:
        errors.append(f"{path}: {node!r} not in {schema['enum']}")
    if kind == "object":
        if not isinstance(node, dict):
            return [f"{path}: expected object"]
        props = schema.get("properties", {})
        for key in schema.get("required", []):
            if key not in node:
                errors.append(f"{path}: missing {key}")
        for key, value in node.items():
            if key not in props:
                errors.append(f"{path}: unknown member {key}")
            else:
                errors += validate(value, props[key], root, f"{path}.{key}")
    elif kind == "array":
        if not isinstance(node, list):
            return [f"{path}: expected array"]
        for i, item in enumerate(node):
            errors += validate(item, schema["items"], root, f"{path}[{i}]")
    elif kind == "string" and not isinstance(node, str):
        errors.append(f"{path}: expected string")
    elif kind == "boolean" and not isinstance(node, bool):
        errors.append(f"{path}: expected boolean")
    elif kind == "integer" and (isinstance(node, bool) or not isinstance(node, int) or node < 0):
        errors.append(f"{path}: expected non-negative integer, got {node!r}")
    elif kind == "number" and (isinstance(node, bool) or not isinstance(node, (int, float))):
        errors.append(f"{path}: expected number")
    return errors


def coerce(node, schema, root):
    """Writes every schema `number` as a float so the typed decode never sees an integer literal for a double."""
    if "$ref" in schema:
        return coerce(node, defs(root)[schema["$ref"].rsplit("/", 1)[1]], root)
    kind = schema.get("type")
    if kind == "object" and isinstance(node, dict):
        return {k: coerce(v, schema.get("properties", {}).get(k, {}), root) for k, v in node.items()}
    if kind == "array" and isinstance(node, list):
        return [coerce(v, schema["items"], root) for v in node]
    if kind == "number" and isinstance(node, int) and not isinstance(node, bool):
        return float(node)
    return node


def merge(old, new):
    """Additive union: existing values always win; keys and id-keyed rows only the new document has are added, rows keep their order."""
    if isinstance(old, dict) and isinstance(new, dict):
        return {**{k: merge(v, new[k]) if k in new else v for k, v in old.items()}, **{k: v for k, v in new.items() if k not in old}}
    if isinstance(old, list) and isinstance(new, list) and all(isinstance(r, dict) and "id" in r for r in old + new):
        have = {r["id"]: r for r in old}
        return [merge(r, next((n for n in new if n["id"] == r["id"]), r)) for r in old] + [r for r in new if r["id"] not in have]
    return old


def finish(doc, art, out_rel, expect_empty=(), compact=False, schema_rel="🧬️schema/🔣️.json"):
    """Validates the document and writes it additively (existing values win, only missing keys and id rows are added; `--check` writes nothing and exits 1 when the file would change); exits non-zero on any defect. `expect_empty` lists collections that may stay empty (none expected)."""
    schema = load(art / schema_rel)
    doc = coerce(doc, schema, schema)
    errors = validate(doc, schema, schema)
    for name, spec in schema["properties"].items():
        if spec.get("type") == "array":
            rows = doc.get(name, [])
            if not rows and name not in expect_empty:
                errors.append(f"collection {name} is empty")
            ids = [r.get("id") for r in rows if isinstance(r, dict) and "id" in r]
            if len(ids) != len(set(ids)):
                errors.append(f"collection {name} has duplicate ids")
    if errors:
        print("\n".join(errors[:60]))
        sys.exit(1)
    out = art / out_rel
    if out.exists():
        doc = merge(coerce(json.loads(out.read_text(encoding="utf-8")), schema, schema), doc)
        errors = validate(doc, schema, schema)
        if errors:
            print("\n".join(errors[:60]))
            sys.exit(1)
    text = (json.dumps(doc, separators=(",", ":"), ensure_ascii=False) if compact else json.dumps(doc, indent=2, ensure_ascii=False)) + "\n"
    changed = not out.exists() or out.read_text(encoding="utf-8") != text
    if "--check" in sys.argv:
        print(("would change " if changed else "unchanged ") + out_rel)
        sys.exit(1 if changed else 0)
    if changed:
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(text, encoding="utf-8")
    print(out_rel, {k: len(v) for k, v in doc.items() if isinstance(v, list)})


def clone(row, **leaves):
    row = copy.deepcopy(row)
    row.update(leaves)
    return row
