#!/usr/bin/env python3
"""Schema-first bridge of the field-adder and collection tools: a field, a record or a collection is written into the JSON Schema (the single source, in the layout the file already has) and every twin is produced by `repo ./📜️script.ts schema twins generate`: TypeScript wire type and reader, protobuf, GraphQL and the Rust structs (mutation leaf payloads and the annotated records of artifact, snapshot and diff roots). No tool edits a twin or a generated struct by hand.

A type spec is one of the strings f64 f32 u8 u16 u32 u64 usize i32 i64 String bool, or an object:
  {"enum": ["a", "b"], "rust": "path::Type"}          string enum (rust path optional)
  {"ref": "Name", "rust": "path::Type"}                reference to a $defs record or enum of the same document
  {"rows": "Row", "fields": {"id": "String", ...}}     id-keyed row collection: array of the $defs record `Row`, which is created from `fields` (each field spec again)
  {"nullable": <spec>}                                 Option<T> (optional and nullable on the wire)
Rust annotations of a property: `dsl` ({"unit": "m"} / {"table": true}), `state` ("artifact"), `doc` (docstring lines), `rust` ({"attributes": [...], "type": "...", "name": "..."}).
Usage: `schema_twins.py compact|indent FILE...` sets the layout of schema files; `schema_twins.py FILE FIELD SPEC` adds a field. As a library: add_field(json_path, name, spec, after=None, container=None, required=True, ui=None, state=None, dsl=None, doc=None, rust=None); add_record(json_path, key, fields, like=None, ...); regenerate(under).
"""
import json, os, subprocess, sys

INTEGERS = {"u8": (0, 255), "u16": (0, 65535), "u32": (0, 4294967295), "u64": (0, None), "usize": (0, None), "i32": (-2147483648, 2147483647), "i64": (None, None)}


def dump_indent(data):
    return json.dumps(data, indent=2, ensure_ascii=False) + "\n"


def dump_compact(data):
    """One line per top-level key and per definition (the canonical layout of the diff and mutation-aggregate schemas)."""
    c = lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False)
    rows = []
    for key, value in data.items():
        if key in ("$defs", "definitions") and isinstance(value, dict):
            body = ",\n".join(f"    {c(name)}:{c(node)}" for name, node in value.items())
            rows.append(f"  {c(key)}:{{\n{body}\n  }}")
        else:
            rows.append(f"  {c(key)}:{c(value)}")
    return "{\n" + ",\n".join(rows) + "\n}\n"


def layout_of(text):
    data = json.loads(text)
    return "compact" if dump_compact(data) == text and dump_indent(data) != text else "indent"


def edit_schema(path, mutate, layout=None):
    """Reads a schema document, lets `mutate(data)` change it (return False for no change) and writes it back in the layout it had (`layout` "compact" or "indent" states it: the diff and mutation-aggregate schemas are compact, the final `schema twins generate` normalises the layout to the canonical one of `serializeSchema`, so no separate compaction step exists); a concurrent change between the read and the write aborts."""
    with open(path, encoding="utf8") as handle:
        text = handle.read()
    data = json.loads(text)
    if mutate(data) is False:
        return False
    rendered = dump_compact(data) if (layout or layout_of(text)) == "compact" else dump_indent(data)
    with open(path, encoding="utf8") as handle:
        if handle.read() != text:
            raise RuntimeError(f"{path} changed while it was edited; run again")
    if rendered == text:
        return False
    with open(path, "w", encoding="utf8") as handle:
        handle.write(rendered)
    return True


def node_of(spec, defs):
    """JSON Schema node of a type spec; records needed by `rows` are added to `defs`."""
    if isinstance(spec, str):
        if spec in ("f64", "f32"): return {"type": "number"}
        if spec == "String": return {"type": "string"}
        if spec == "bool": return {"type": "boolean"}
        if spec in INTEGERS:
            lo, hi = INTEGERS[spec]
            node = {"type": "integer"}
            if lo is not None: node["minimum"] = lo
            if hi is not None: node["maximum"] = hi
            return node
        raise ValueError(f"unknown field type {spec!r}")
    if "nullable" in spec:
        inner = node_of(spec["nullable"], defs)
        inner["type"] = [inner["type"], "null"] if isinstance(inner.get("type"), str) else inner.get("type")
        return inner
    if "enum" in spec:
        node = {"type": "string", "enum": list(spec["enum"])}
    elif "ref" in spec:
        node = {"$ref": "#/$defs/" + spec["ref"]}
    elif "rows" in spec:
        row = spec["rows"]
        if row not in defs:
            props = {name: node_of(field, defs) for name, field in spec["fields"].items()}
            defs[row] = {"title": row, "type": "object", "additionalProperties": False, "required": list(props), "properties": props}
        node = {"type": "array", "items": {"$ref": "#/$defs/" + row}}
    else:
        raise ValueError(f"unknown field spec {spec!r}")
    if "rust" in spec:
        node["x-semio-rust"] = {"type": spec["rust"]}
    return node


def annotate(node, ui=None, state=None, dsl=None, doc=None, rust=None):
    """Puts the UI, state and Rust annotations on a property node."""
    if ui: node["x-semio-ui"] = ui
    if state: node["x-semio-state"] = state
    if dsl: node["x-semio-dsl"] = dsl
    note = {**node.get("x-semio-rust", {}), **(rust or {})}
    if doc: note["doc"] = [doc] if isinstance(doc, str) else list(doc)
    if note: node["x-semio-rust"] = note
    return node


def defs_of(data):
    return data.setdefault("$defs", {}) if "definitions" not in data else data["definitions"]


def insert_after(mapping, name, value, after):
    ordered = {}
    for key, item in mapping.items():
        ordered[key] = item
        if key == after: ordered[name] = value
    if name not in ordered: ordered[name] = value
    return ordered


def add_property(path, name, node, after=None, container=None, required=True, layout=None):
    """Writes one property node into the schema document (root, or the `$defs` record `container`); False when it is already there."""
    def mutate(data):
        holder = data if container is None else defs_of(data)[container]
        if name in holder.get("properties", {}):
            return False
        holder["properties"] = insert_after(holder.get("properties", {}), name, node, after)
        if required:
            req = holder.setdefault("required", [])
            req.insert(req.index(after) + 1 if after in req else len(req), name)
    return edit_schema(path, mutate, layout)


def add_field(json_path, name, spec, after=None, container=None, required=True, ui=None, state=None, dsl=None, doc=None, rust=None):
    """Writes one typed property (with its annotations) into the schema document; records a `rows` spec needs are created beside it."""
    created = {}
    node = annotate(node_of(spec, created), ui, state, dsl, doc, rust)
    nullable = isinstance(spec, dict) and "nullable" in spec

    def mutate(data):
        defs = defs_of(data)
        for key, record in created.items(): defs.setdefault(key, record)
        holder = data if container is None else defs[container]
        if name in holder.get("properties", {}):
            return False
        holder["properties"] = insert_after(holder.get("properties", {}), name, node, after)
        if required and not nullable:
            req = holder.setdefault("required", [])
            req.insert(req.index(after) + 1 if after in req else len(req), name)
    return edit_schema(json_path, mutate)


def add_record(json_path, key, fields, like=None, doc=None, after=None, formats=None):
    """Creates the `$defs` record `key` from `fields` ({name: {"node": <json node>, "dsl": .., "state": .., "doc": ..}}). With `like` (another record of the document) the record copies that record's Rust annotations (derive, attributes, artifactSchema, dsl keyword) and `x-semio-formats`, so the generator emits its struct next to the sibling's."""
    def mutate(data):
        defs = defs_of(data)
        if key in defs:
            return False
        sibling = defs.get(like, {}) if like else {}
        props = {name: annotate(dict(field["node"]), field.get("ui"), field.get("state"), field.get("dsl"), field.get("doc"), field.get("rust")) for name, field in fields.items()}
        record = {"title": key, "type": "object", "additionalProperties": False, "required": [name for name, field in fields.items() if field.get("required", True)], "properties": props}
        note = {k: v for k, v in sibling.get("x-semio-rust", {}).items() if k in ("derive", "attributes", "artifactSchema")}
        if note:
            if doc: note["doc"] = [doc] if isinstance(doc, str) else list(doc)
            record["x-semio-rust"] = note
        if "x-semio-dsl" in sibling: record["x-semio-dsl"] = sibling["x-semio-dsl"]
        if formats or "x-semio-formats" in sibling: record["x-semio-formats"] = formats or sibling["x-semio-formats"]
        ordered = insert_after(dict(defs), key, record, after)
        defs.clear()
        defs.update(ordered)
    return edit_schema(json_path, mutate)


def regenerate(under, languages=None):
    """Runs the twin generator for a scope (a path, relative to the repository root or absolute); exits non-zero on failure."""
    command = ["repo", "./📜️script.ts", "schema", "twins", "generate", "--under", under]
    if languages: command += ["--only", ",".join(languages)]
    root = os.environ.get("SEMIO_ROOT") or os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
    subprocess.run(["nice", "-n", "19"] + command if os.name != "nt" else command, cwd=root, check=True)


if __name__ == "__main__":
    if len(sys.argv) >= 3 and sys.argv[1] in ("compact", "indent"):
        for target in sys.argv[2:]:
            print(("rewrote " if edit_schema(target, lambda data: None if data else None, sys.argv[1]) else "unchanged ") + target)
        sys.exit(0)
    if len(sys.argv) < 4:
        sys.exit(__doc__)
    path, field, raw = sys.argv[1:4]
    spec = json.loads(raw) if raw.startswith(("{", '"')) else raw
    print("added" if add_field(path, field, spec) else "present")
