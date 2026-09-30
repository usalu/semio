#!/usr/bin/env python3
"""⚖️ W2-S norm parity: rebuilds every norm mutation leaf payload schema, the snapshot-facet `$defs` its records reference and every
norm aggregate schema from the Rust wire shape (value_derive rules), keeping the W2-R `x-semio-ui` annotations.

    python3 🧪️w2-s-norm-parity.py report            # what would change, labels still missing
    python3 🧪️w2-s-norm-parity.py apply [--only <artifact>]

Rules (plan "W2-S parity brief"): leaf root = `payload_value()` exactly as Rust emits it (every norm leaf struct is camelCase);
internally tagged aggregates pin `<tag>: {const}` in each leaf and are a `oneOf` of leaf `$ref`s; externally tagged ones wrap each
leaf `$ref` in its variant key; `Option` fields are nullable and not required; records and enums live once in the artifact's
snapshot facet `$defs`.
"""
import copy
import importlib.util
import json
import os
import re
import sys
from collections import OrderedDict

REPO = "/Users/ueli/Documents/semio"
TICKET = os.path.dirname(os.path.abspath(__file__))
NORM = "✏️s/🔌️plugins/📕️norm"
ARTIFACTS = f"{NORM}/🗿️artifacts"
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any"
ID_BASE = "https://json.schemas.assets.semio-tech.com/"
DRAFT7 = "http://json-schema.org/draft-07/schema#"
ANNOTATIONS = ("x-semio-ui", "description", "title")
BOUNDS = ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "minLength", "maxLength", "pattern", "minItems", "maxItems", "uniqueItems", "format", "default", "examples")


def load_module(name, filename):
    spec = importlib.util.spec_from_file_location(name, os.path.join(TICKET, filename))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


rust = load_module("w2s_norm_rust", "🧪️w2-s-norm-rust.py")
labels = load_module("w2s_norm_labels", "🧪️w2-s-norm-labels.py")


def read_json(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return json.load(handle, object_pairs_hook=OrderedDict)


def dump_json(value, trailing):
    return json.dumps(value, indent=2, ensure_ascii=False) + ("\n" if trailing else "")


def write_json(path, value):
    absolute = os.path.join(REPO, path)
    trailing = True
    if os.path.exists(absolute):
        with open(absolute, encoding="utf-8") as handle:
            trailing = handle.read().endswith("\n")
    text = dump_json(value, trailing)
    if os.path.exists(absolute):
        with open(absolute, encoding="utf-8") as handle:
            if handle.read() == text:
                return False
    with open(absolute, "w", encoding="utf-8") as handle:
        handle.write(text)
    return True


#region 🔖️TypeExpressions
PRIMITIVE_INT_UNSIGNED = {"u8", "u16", "u32", "u64", "u128", "usize"}
PRIMITIVE_INT_SIGNED = {"i8", "i16", "i32", "i64", "i128", "isize"}
PRIMITIVE_FLOAT = {"f32", "f64"}
PRIMITIVE_STRING = {"String", "str", "char", "PathBuf"}
WRAPPERS = {"Box", "Rc", "Arc", "Cow"}
SEQUENCES = {"Vec", "VecDeque", "BTreeSet", "HashSet", "SmallVec"}
MAPS = {"BTreeMap", "HashMap", "IndexMap"}


def parse_type(text):
    """🌳️ A Rust type expression as `(name, [args])`; arrays are `("[;]", [T, N])`, tuples `("()", [...])`."""
    text = text.strip().rstrip(",").strip()
    text = re.sub(r"^&\s*(?:'\w+\s+)?(?:mut\s+)?", "", text)
    if text.startswith("(") and text.endswith(")"):
        parts = rust.split_top(text[1:-1])
        return ("()", [parse_type(part) for part in parts if part.strip()])
    if text.startswith("[") and text.endswith("]"):
        inner = text[1:-1]
        parts = rust.split_top(inner, ";")
        if len(parts) == 2:
            return ("[;]", [parse_type(parts[0]), parts[1].strip()])
        return ("[]", [parse_type(inner)])
    match = re.match(r"^((?:[\w]+::)*)(\w+)\s*(<(.*)>)?$", text, flags=re.S)
    if not match:
        raise ValueError(f"unparsed type {text!r}")
    args = [parse_type(part) for part in rust.split_top(match.group(4)) if not part.strip().startswith("'")] if match.group(4) else []
    return (match.group(2), args, match.group(1))


def type_name(tree):
    return tree[0]
#endregion 🔖️TypeExpressions


class Leaf:
    def __init__(self, artifact, directory, descriptor):
        self.artifact, self.directory, self.descriptor = artifact, directory, descriptor
        self.kind = descriptor["semanticKind"]
        self.variant = descriptor["aggregateVariant"]
        self.schema_path = f"{directory}/{descriptor.get('payloadSchema', '🧬️schema/🔣️.json')}"
        self.rust_file, self.struct = None, None


class Artifact:
    """🗿️ One norm artifact (or the results window config): crate registry, aggregate, leaves, snapshot facet."""

    def __init__(self, key, root, mutations, scope, snapshot_path, snapshot_id, registry_roots):
        self.key, self.root, self.mutations, self.scope = key, root, mutations, scope
        self.snapshot_path, self.snapshot_id = snapshot_path, snapshot_id
        self.registry = rust.Registry([os.path.join(REPO, path) for path in registry_roots])
        aggregates = [item for item in rust.parse_items(os.path.join(REPO, mutations, "🦀️.rs")) if item.kind == "enum" and "Mutations" in item.derives]
        self.aggregate = aggregates[0]
        self.layout = self.aggregate.value
        self.leaves = []
        for entry in sorted(os.listdir(os.path.join(REPO, mutations))):
            directory = f"{mutations}/{entry}"
            descriptor_path = os.path.join(REPO, directory, "🔣️.json")
            if not os.path.isdir(os.path.join(REPO, directory)) or not os.path.exists(descriptor_path):
                continue
            descriptor = read_json(f"{directory}/🔣️.json")
            if "semanticKind" not in descriptor:
                continue
            leaf = Leaf(self, directory, descriptor)
            variant = next((v for v in self.aggregate.variants if v["ident"] == leaf.variant), None)
            if variant is None:
                raise KeyError(f"{directory}: aggregate {self.aggregate.name} has no variant {leaf.variant}")
            struct_name = variant["fields"][0].split("::")[-1] if variant["kind"] == "tuple" else None
            for candidate in ("🦀️.rs", "🦠️mutation/🦀️.rs"):
                path = f"{directory}/{candidate}"
                if os.path.exists(os.path.join(REPO, path)):
                    found = [item for item in rust.parse_items(os.path.join(REPO, path)) if item.name == struct_name]
                    if found:
                        leaf.rust_file, leaf.struct = path, found[0]
                        break
            if leaf.struct is None:
                raise KeyError(f"{directory}: no struct {struct_name}")
            leaf.wire = rust.variant_wire(variant["ident"], variant["value"].get("rename"), self.layout.get("rename_all"))
            self.leaves.append(leaf)

    def leaf_id(self, leaf):
        return f"{ID_BASE}{self.scope}/mutation/{leaf.kind}/schema.json"

    def aggregate_id(self):
        return f"{ID_BASE}{self.scope}/mutations.json"


def artifacts():
    found = []
    for entry in sorted(os.listdir(os.path.join(REPO, ARTIFACTS))):
        root = f"{ARTIFACTS}/{entry}"
        if not os.path.isdir(os.path.join(REPO, root)):
            continue
        key = re.sub(r"^[^\w]+", "", entry)
        subset = f"{root}/{SUBSET}"
        snapshot_path = f"{subset}/🧬️schema/📸️snapshot/🔣️.json"
        found.append(Artifact(key, root, f"{subset}/🧬️schema/🧬️mutations", f"s/norm/{key}", snapshot_path, labels.SNAPSHOT_IDS.get(key, f"{ID_BASE}s/norm/{key}/snapshot.json"), [root, f"{NORM}/⚖️compliance"]))
    results = f"{NORM}/🪟️results"
    found.append(Artifact("results", results, f"{results}/🎚️config/🧬️schema/🧬️mutations", "app/norm/results/window-config", None, None, [results, f"{NORM}/⚖️compliance"]))
    return found


#region 🔖️SchemaBuilder
class Builder:
    """🧱️ JSON Schema for Rust types of one artifact: primitives inline, named records/enums as snapshot-facet `$defs`."""

    def __init__(self, artifact):
        self.artifact = artifact
        self.defs = OrderedDict()
        self.pending = []
        self.problems = []

    def ref(self, name, local):
        if name not in self.defs and name not in self.pending:
            self.pending.append(name)
        return OrderedDict([("$ref", f"#/$defs/{name}" if local else f"{self.artifact.snapshot_id}#/$defs/{name}")])

    def schema(self, text, local=False, hint=None):
        return self.tree(parse_type(text), local, hint)

    def tree(self, tree, local, hint):
        name, args = tree[0], tree[1]
        if name in ("()",):
            items = [self.tree(arg, local, hint) for arg in args]
            return OrderedDict([("type", "array"), ("items", items), ("minItems", len(items)), ("maxItems", len(items))])
        if name == "[;]":
            count = int(args[1]) if args[1].isdigit() else None
            node = OrderedDict([("type", "array"), ("items", self.tree(args[0], local, hint))])
            if count is not None:
                node["minItems"] = count
                node["maxItems"] = count
            return node
        if name == "[]":
            return OrderedDict([("type", "array"), ("items", self.tree(args[0], local, hint))])
        if name in PRIMITIVE_FLOAT:
            return OrderedDict([("type", "number")])
        if name in PRIMITIVE_INT_UNSIGNED:
            return OrderedDict([("type", "integer"), ("minimum", 0)])
        if name in PRIMITIVE_INT_SIGNED:
            return OrderedDict([("type", "integer")])
        if name == "bool":
            return OrderedDict([("type", "boolean")])
        if name in PRIMITIVE_STRING:
            return OrderedDict([("type", "string")])
        if name == "Option":
            return nullable(self.tree(args[0], local, hint))
        if name in WRAPPERS:
            return self.tree(args[0], local, hint)
        if name in SEQUENCES:
            node = OrderedDict([("type", "array"), ("items", self.tree(args[0], local, hint))])
            if name in ("BTreeSet", "HashSet"):
                node["uniqueItems"] = True
            return node
        if name in MAPS:
            return OrderedDict([("type", "object"), ("additionalProperties", self.tree(args[1], local, hint))])
        if name in ("DslValue", "Value"):
            self.problems.append(f"opaque value type {name}")
            return OrderedDict()
        item = self.artifact.registry.lookup(name, hint)
        if item is None:
            self.problems.append(f"unknown type {name}")
            return OrderedDict([("type", "object")])
        if item.kind == "alias":
            return self.schema(item.target, local, hint)
        if item.kind == "tuple" and len(item.fields) == 1:
            return self.schema(item.fields[0]["type"], local, hint)
        return self.ref(name, local)

    def definition(self, name):
        """📐️ The `$defs` entry of one named record or enum, fields in declaration order, nested types as local refs."""
        item = self.artifact.registry.lookup(name)
        value = item.value
        if item.kind == "struct":
            node = OrderedDict([("title", name), ("type", "object"), ("additionalProperties", False)])
            if value.get("transparent") and len(item.fields) == 1:
                return self.schema(item.fields[0]["type"], True)
            required, properties = [], OrderedDict()
            for field in item.fields:
                fvalue = field["value"]
                if fvalue.get("skip") or fvalue.get("skip_serializing"):
                    continue
                if fvalue.get("flatten") or fvalue.get("with") or fvalue.get("serialize_with"):
                    self.problems.append(f"{name}.{field['ident']} uses unsupported attribute {fvalue}")
                wire = rust.field_wire(field["ident"], fvalue.get("rename"), value.get("rename_all"))
                properties[wire] = self.schema(field["type"], True, os.path.dirname(item.path))
                optional = field["type"].startswith("Option<") and not fvalue.get("required")
                if not optional and "default" not in fvalue and not value.get("default"):
                    required.append(wire)
            node["required"] = required
            node["properties"] = properties
            return node
        if item.kind == "enum":
            rename_all = value.get("rename_all")
            tag, content = value.get("tag"), value.get("content")
            units = [v for v in item.variants if v["kind"] == "unit"]
            if len(units) == len(item.variants) and not tag:
                return OrderedDict([("title", name), ("type", "string"), ("enum", [rust.variant_wire(v["ident"], v["value"].get("rename"), rename_all) for v in item.variants])])
            branches = []
            for variant in item.variants:
                wire = rust.variant_wire(variant["ident"], variant["value"].get("rename"), rename_all)
                field_case = variant["value"].get("rename_all") or value.get("rename_all_fields")
                if variant["kind"] == "named":
                    payload = OrderedDict([("type", "object"), ("additionalProperties", False)])
                    required, properties = [], OrderedDict()
                    for field in variant["fields"]:
                        fwire = rust.field_wire(field["ident"], field["value"].get("rename"), field_case)
                        properties[fwire] = self.schema(field["type"], True)
                        if not field["type"].startswith("Option<") and "default" not in field["value"]:
                            required.append(fwire)
                    payload["required"], payload["properties"] = required, properties
                elif variant["kind"] == "tuple":
                    if len(variant["fields"]) != 1:
                        self.problems.append(f"{name}::{variant['ident']} has several tuple fields")
                    inner = self.artifact.registry.lookup(parse_type(variant["fields"][0])[0]) if tag and not content else None
                    payload = self.definition(inner.name) if inner is not None and inner.kind == "struct" else self.schema(variant["fields"][0], True)
                    if inner is not None and inner.kind == "struct":
                        payload.pop("title", None)
                else:
                    payload = None
                if tag and content:
                    branch = OrderedDict([("type", "object"), ("additionalProperties", False), ("required", [tag] + ([content] if payload is not None else [])), ("properties", OrderedDict([(tag, OrderedDict([("const", wire)]))] + ([(content, payload)] if payload is not None else [])))])
                elif tag:
                    if payload is None:
                        branch = OrderedDict([("type", "object"), ("additionalProperties", False), ("required", [tag]), ("properties", OrderedDict([(tag, OrderedDict([("const", wire)]))]))])
                    elif "properties" in payload:
                        branch = copy.deepcopy(payload)
                        branch["required"] = [tag] + branch["required"]
                        branch["properties"] = OrderedDict([(tag, OrderedDict([("const", wire)]))] + list(branch["properties"].items()))
                    else:
                        self.problems.append(f"{name}::{variant['ident']} internally tagged newtype over a non-record")
                        branch = OrderedDict([("type", "object"), ("required", [tag]), ("properties", OrderedDict([(tag, OrderedDict([("const", wire)]))]))])
                elif payload is None:
                    branch = OrderedDict([("const", wire)])
                else:
                    branch = OrderedDict([("type", "object"), ("additionalProperties", False), ("required", [wire]), ("properties", OrderedDict([(wire, payload)]))])
                branch = OrderedDict([("title", variant["ident"])] + list(branch.items()))
                branches.append(branch)
            return OrderedDict([("title", name), ("oneOf", branches)])
        self.problems.append(f"{name}: unsupported item kind {item.kind}")
        return OrderedDict([("title", name)])

    def drain(self):
        while self.pending:
            name = self.pending.pop(0)
            if name in self.defs:
                continue
            self.defs[name] = None
            self.defs[name] = self.definition(name)
        return self.defs


def nullable(node):
    if "type" in node and isinstance(node["type"], str) and "$ref" not in node and "oneOf" not in node:
        out = OrderedDict(node)
        out["type"] = [node["type"], "null"]
        return out
    return OrderedDict([("oneOf", [node, OrderedDict([("type", "null")])])])
#endregion 🔖️SchemaBuilder


def leaf_fields(leaf):
    """🧾️ Wire fields of a leaf struct under the camelCase rule every norm leaf follows."""
    out = []
    for field in leaf.struct.fields:
        value = field["value"]
        if value.get("skip"):
            continue
        wire = rust.field_wire(field["ident"], value.get("rename"), "camelCase")
        optional = field["type"].startswith("Option<") and not value.get("required")
        out.append((field, wire, not optional and "default" not in value and not leaf.struct.value.get("default")))
    return out


#region 🔖️Merge
NUMERIC = {"number", "integer"}
NUMBER_UI = {"unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource"}


def normalized(text):
    return re.sub(r"[^a-z0-9]", "", str(text).lower())


def node_types(node):
    kind = node.get("type")
    return {kind} if isinstance(kind, str) else set(kind or [])


def target_node(node, documents):
    """🎯️ The node a property resolves to (through `$ref` and a nullable `oneOf`), for widget/option sanity checks."""
    for _ in range(8):
        if "$ref" in node:
            document, _, pointer = node["$ref"].partition("#")
            owner = documents.get(document)
            if owner is None:
                return node
            current = owner
            for segment in pointer.split("/")[1:]:
                current = current.get(segment, {}) if isinstance(current, dict) else {}
            node = current
            continue
        branches = node.get("oneOf") or node.get("anyOf")
        if branches:
            concrete = [b for b in branches if not (b.get("type") == "null" and len(b) == 1)]
            if len(concrete) == 1 and len(concrete) < len(branches):
                node = concrete[0]
                continue
        return node
    return node


def sane_ui(ui, resolved):
    """🧽️ Keeps the `x-semio-ui` keys the resolved value can take: number facets only on numbers, options only over its enum
    (re-keyed case-insensitively onto the Rust wire spelling), no scalar widget on a record."""
    ui = OrderedDict(ui)
    types = node_types(resolved)
    enum = resolved.get("enum")
    record = "object" in types or "properties" in resolved or "oneOf" in resolved
    array = "array" in types
    if not (types & NUMERIC) and not (array and resolved.get("items", {}).get("type") in NUMERIC):
        for key in NUMBER_UI:
            ui.pop(key, None)
    if "options" in ui:
        if not enum:
            ui.pop("options")
        else:
            by = {normalized(value): value for value in enum}
            ui["options"] = OrderedDict((by[normalized(key)], label) for key, label in ui["options"].items() if normalized(key) in by)
            if not ui["options"]:
                ui.pop("options")
    widget = ui.get("widget")
    if widget is not None:
        fits = {
            "slider": bool(types & NUMERIC), "stepper": bool(types & NUMERIC), "dial": bool(types & NUMERIC),
            "toggle": "boolean" in types, "select": bool(enum), "segmented": bool(enum),
            "text": "string" in types and not enum, "multiline": "string" in types and not enum,
            "reference": "string" in types or "integer" in types or (array and resolved.get("items", {}).get("type") in ("string", "integer")),
            "vector": array, "hidden": True,
        }.get(widget, False)
        if record or not fits:
            ui.pop("widget")
    if record and ui.get("role") == "target":
        ui["role"] = "value"
    if "ref" in ui and not ("string" in types or "integer" in types or array):
        ui.pop("ref")
    return ui


def merge_property(generated, previous, documents):
    """🧬️ The Rust-derived structure with the previous node's annotations (`x-semio-ui`, description, title) and hard bounds."""
    merged = copy.deepcopy(generated)
    if not previous:
        return merged
    resolved = target_node(merged, documents)
    if "$ref" in merged or ("oneOf" in merged and "type" not in merged):
        pass
    else:
        types = node_types(merged)
        for key in BOUNDS:
            if key not in previous or key in ("default", "examples"):
                continue
            if key in ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum") and not (types & NUMERIC):
                continue
            if key in ("minLength", "maxLength", "pattern", "format") and "string" not in types:
                continue
            if key in ("minItems", "maxItems", "uniqueItems") and "array" not in types:
                continue
            if key == "minimum" and "minimum" in merged:
                merged[key] = max(merged[key], previous[key])
            else:
                merged.setdefault(key, previous[key])
        if "array" in types and isinstance(merged.get("items"), dict) and isinstance(previous.get("items"), dict):
            inner_types = node_types(merged["items"])
            for key in ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"):
                if key in previous["items"] and inner_types & NUMERIC:
                    merged["items"].setdefault(key, previous["items"][key])
    for key in ANNOTATIONS:
        if key in previous:
            merged[key] = copy.deepcopy(previous[key]) if key != "x-semio-ui" else sane_ui(previous[key], resolved)
    if merged.get("title") and "$ref" in merged:
        merged.pop("title")
    return order_node(merged)


KEY_ORDER = ("$schema", "$id", "$ref", "title", "description", "type", "const", "enum", "format", "minimum", "exclusiveMinimum", "maximum", "exclusiveMaximum", "minLength", "maxLength", "pattern", "additionalProperties", "required", "properties", "items", "minItems", "maxItems", "uniqueItems", "oneOf", "anyOf", "allOf", "$defs", "x-semio-ui")


def order_node(node):
    ordered = OrderedDict((key, node[key]) for key in KEY_ORDER if key in node)
    for key, value in node.items():
        if key not in ordered:
            ordered[key] = value
    return ordered
#endregion 🔖️Merge


#region 🔖️Leaves
def previous_property(properties, wire, ident):
    for name in (wire, ident, rust.field_wire(ident, None, "camelCase")):
        if name in properties:
            return properties[name]
    by = {normalized(name): node for name, node in properties.items()}
    return by.get(normalized(wire))


def build_leaf(artifact, leaf, builder, documents):
    old = read_json(leaf.schema_path) if os.path.exists(os.path.join(REPO, leaf.schema_path)) else OrderedDict()
    old_properties = old.get("properties", OrderedDict()) if isinstance(old, dict) else OrderedDict()
    tag = artifact.layout.get("tag")
    hint = os.path.dirname(os.path.join(REPO, leaf.rust_file))
    required, properties = [], OrderedDict()
    if tag:
        required.append(tag)
        properties[tag] = OrderedDict([("const", leaf.wire)])
    extra = labels.LEAVES.get(artifact.key, {}).get(leaf.kind, {})
    for field, wire, is_required in leaf_fields(leaf):
        generated = builder.schema(field["type"], False, hint)
        previous = previous_property(old_properties, wire, field["ident"])
        if wire in extra:
            previous = OrderedDict(previous or {})
            previous["x-semio-ui"] = extra[wire]
        properties[wire] = merge_property(generated, previous, documents)
        if is_required:
            required.append(wire)
    schema = OrderedDict([("$schema", DRAFT7), ("$id", artifact.leaf_id(leaf)), ("title", leaf.struct.name)])
    if isinstance(old, dict) and old.get("description"):
        schema["description"] = old["description"]
    schema["type"] = "object"
    schema["additionalProperties"] = False
    schema["required"] = required
    schema["properties"] = properties
    if isinstance(old, dict) and "x-semio-ui" in old:
        schema["x-semio-ui"] = old["x-semio-ui"]
    return schema
#endregion 🔖️Leaves


#region 🔖️Definitions
def rewrite_definitions(node):
    """🔁️ `definitions` → `$defs` in one document, local `#/definitions/` refs included."""
    if isinstance(node, dict):
        out = OrderedDict()
        for key, value in node.items():
            if key == "$ref" and isinstance(value, str) and value.startswith("#/definitions/"):
                out[key] = "#/$defs/" + value[len("#/definitions/") :]
            elif key == "definitions":
                out["$defs"] = rewrite_definitions(value)
            else:
                out[key] = rewrite_definitions(value)
        return out
    if isinstance(node, list):
        return [rewrite_definitions(item) for item in node]
    return node


def merge_definition(generated, previous, label_table, options_table, documents, name):
    merged = copy.deepcopy(generated)
    previous = previous or OrderedDict()
    for key in ("description", "x-semio-ui"):
        if key in previous:
            merged[key] = copy.deepcopy(previous[key])
    if "enum" in merged:
        ui = OrderedDict(merged.get("x-semio-ui", OrderedDict()))
        if options_table:
            ui["options"] = OrderedDict((value, {"en": options_table[value][0], "de": options_table[value][1]}) for value in merged["enum"] if value in options_table)
        if ui:
            merged["x-semio-ui"] = sane_ui(ui, merged)
    if "properties" in merged:
        old_properties = previous.get("properties", OrderedDict())
        for wire, node in list(merged["properties"].items()):
            prior = old_properties.get(wire)
            if wire in (label_table or {}):
                prior = OrderedDict(prior or {})
                prior["x-semio-ui"] = label_table[wire]
            merged["properties"][wire] = merge_property(node, prior, documents)
            for key, bound in labels.DEF_BOUNDS.get(documents.get("__artifact__", ""), {}).get(name, {}).get(wire, {}).items():
                if isinstance(bound, dict):
                    merged["properties"][wire][key] = order_node(OrderedDict(list(merged["properties"][wire][key].items()) + list(bound.items())))
                else:
                    merged["properties"][wire][key] = bound
            merged["properties"][wire] = order_node(merged["properties"][wire])
    if "oneOf" in merged:
        for branch in merged["oneOf"]:
            variant = branch.get("title")
            ui = (options_table or {}).get(variant)
            if ui and "x-semio-ui" not in branch:
                branch["x-semio-ui"] = {"label": {"en": ui[0], "de": ui[1]}}
            for wire, node in list(branch.get("properties", {}).items()):
                key = f"{variant}.{wire}"
                if key in (label_table or {}):
                    branch["properties"][wire] = merge_property(node, OrderedDict([("x-semio-ui", label_table[key])]), documents)
    return order_node(merged)


W2R = load_module("w2r_norm_annotate", "🧪️w2-r-norm-annotate-inputs.py")
HARVEST_KEYS = ("widget", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "options")


def harvest_ui(annotation):
    """🌾️ The record-portable part of a W2-R leaf-input annotation: labels, units and references, never bounds, grouping or role."""
    ui = OrderedDict((key, copy.deepcopy(annotation[key])) for key in HARVEST_KEYS if key in annotation)
    if "ref" in ui:
        ui["role"] = "value"
    return ui


def harvest(artifact, builder):
    """🌾️ `{record: {field: x-semio-ui}}` for the snapshot-facet `$defs` from the W2-R rollout tables: its nested-record rows
    (`names`, `paths`, `suffixes`) and every `change-*` leaf input whose value is one record field."""
    tables = next((value for key, value in W2R.ARTIFACTS.items() if re.sub(r"^[^\w]+", "", key) == artifact.key), {})
    out = OrderedDict()
    fields_of = {name: list((node or {}).get("properties", {}).keys()) for name, node in builder.defs.items()}
    owners = {}
    for name, fields in fields_of.items():
        for field in fields:
            owners.setdefault(field, []).append(name)
    for field, annotation in tables.get("names", {}).items():
        for name in owners.get(field, []):
            out.setdefault(name, OrderedDict()).setdefault(field, harvest_ui(annotation))
    for leaf_kind, rows in tables.get("leaves", {}).items():
        words = leaf_kind.split("-")
        for input_name, annotation in rows.items():
            if annotation.get("role") == "target" or input_name in ("index",):
                continue
            stem = input_name[3:] if input_name.startswith("new") and input_name != "newValue" else None
            candidates = []
            if stem:
                field = stem[:1].lower() + stem[1:]
                candidates = [(name, field) for name in owners.get(field, [])] + [(name, key) for key, names in owners.items() for name in names if normalized(key) == normalized(field) and key != field]
            else:
                for size in range(len(words) - 1, 0, -1):
                    field = rust.apply_case(words[len(words) - size :], "camelCase")
                    found = [(name, field) for name in owners.get(field, [])]
                    if found:
                        candidates = found
                        break
            entity = normalized("".join(words[1:]))
            preferred = [pair for pair in candidates if any(normalized(word) in normalized(pair[0]) for word in words[1:-1])] or candidates
            for name, field in preferred:
                out.setdefault(name, OrderedDict()).setdefault(field, harvest_ui(annotation))
    return out


def build_snapshot(artifact, builder, documents):
    facet = read_json(artifact.snapshot_path)
    facet = rewrite_definitions(facet)
    facet["$schema"] = DRAFT7
    facet["$id"] = artifact.snapshot_id
    head = OrderedDict([("$schema", DRAFT7), ("$id", artifact.snapshot_id)])
    for key, value in facet.items():
        if key not in head:
            head[key] = value
    facet = head
    defs = facet.get("$defs", OrderedDict())
    table = harvest(artifact, builder)
    for name, fields in labels.DEFS.get(artifact.key, {}).items():
        table.setdefault(name, OrderedDict()).update(fields)
    options = labels.OPTIONS.get(artifact.key, {})
    scoped = {**documents, "__artifact__": artifact.key}
    for name, generated in builder.defs.items():
        defs[name] = merge_definition(generated, defs.get(name), table.get(name), options.get(name), scoped, name)
    root = labels.SNAPSHOT_ROOT_FROM_RUST.get(artifact.key)
    if root:
        generated = builder.definition(root)
        old_properties = facet.get("properties", OrderedDict())
        facet["additionalProperties"] = False
        facet["required"] = generated["required"]
        facet["properties"] = OrderedDict((wire, merge_property(node, OrderedDict((key, value) for key, value in (old_properties.get(wire) or {}).items() if key in ANNOTATIONS), scoped)) for wire, node in generated["properties"].items())
    for name in list(defs):
        defs[name] = declared_formats(artifact, name, defs[name])
    facet["$defs"] = defs
    return facet


FORMAT_FILES = (("🦀️rust", "🦀️.rs"), ("🟦️typescript", "🟦️.ts"), ("🔗️graphql", "🔗️.graphql"), ("🛰️protobuf", "🛰️.proto"))


def declares(format_id, source, name):
    """🔎️ The schema checker's presence rule (`declaresSchemaExport`) for one twin format."""
    escaped = re.escape(name)
    patterns = {
        "🛰️protobuf": rf"^\s*message\s+{escaped}\b",
        "🔗️graphql": rf"^\s*(?:type|input|enum|interface|union|scalar)\s+{escaped}\b",
        "🦀️rust": rf"^\s*pub\s+(?:struct|enum|type)\s+{escaped}\b|^\s*pub\s+use\s+(?![^;\n]*[{{*])[A-Za-z_][A-Za-z0-9_]*(?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)*\s*::\s*(?:{escaped}|[A-Za-z_][A-Za-z0-9_]*\s+as\s+{escaped})\s*;",
        "🟦️typescript": rf"^\s*export\s+(?:interface|type|const|class)\s+{escaped}\b",
    }
    return re.search(patterns[format_id], source, flags=re.M) is not None


def declared_formats(artifact, name, node):
    """🏷️ `x-semio-formats` of one snapshot-facet export: the normative JSON Schema plus exactly the twin formats that declare it."""
    folder = os.path.dirname(os.path.join(REPO, artifact.snapshot_path))
    formats = ["🔣️jsonschema"]
    for format_id, filename in FORMAT_FILES:
        path = os.path.join(folder, filename)
        if os.path.exists(path) and declares(format_id, open(path, encoding="utf-8").read(), name):
            formats.append(format_id)
    node = OrderedDict((key, value) for key, value in node.items() if key != "x-semio-formats")
    node["x-semio-formats"] = formats
    return order_node(node)
#endregion 🔖️Definitions


#region 🔖️Aggregate
def build_aggregate(artifact):
    old = read_json(f"{artifact.mutations}/🔣️.json") if os.path.exists(os.path.join(REPO, artifact.mutations, "🔣️.json")) else OrderedDict()
    tag, content = artifact.layout.get("tag"), artifact.layout.get("content")
    by_variant = {leaf.variant: leaf for leaf in artifact.leaves}
    branches = []
    for variant in artifact.aggregate.variants:
        leaf = by_variant[variant["ident"]]
        ref = OrderedDict([("$ref", artifact.leaf_id(leaf))])
        if tag and content:
            branches.append(OrderedDict([("type", "object"), ("additionalProperties", False), ("required", [tag, content]), ("properties", OrderedDict([(tag, OrderedDict([("const", leaf.wire)])), (content, ref)]))]))
        elif tag:
            branches.append(ref)
        else:
            branches.append(OrderedDict([("type", "object"), ("additionalProperties", False), ("required", [leaf.wire]), ("properties", OrderedDict([(leaf.wire, ref)]))]))
    schema = OrderedDict([("$schema", DRAFT7), ("$id", artifact.aggregate_id()), ("title", artifact.aggregate.name)])
    if isinstance(old, dict) and old.get("description"):
        schema["description"] = old["description"]
    schema["oneOf"] = branches
    return schema
#endregion 🔖️Aggregate


def plan(selected=None):
    """🗺️ Every document this WP writes, as `{path: value}`, plus the builder problems."""
    out, problems = OrderedDict(), []
    for artifact in artifacts():
        if selected and artifact.key not in selected:
            continue
        builder = Builder(artifact)
        for leaf in artifact.leaves:
            for field, wire, required in leaf_fields(leaf):
                builder.schema(field["type"], False, os.path.dirname(os.path.join(REPO, leaf.rust_file)))
        builder.drain()
        if labels.SNAPSHOT_ROOT_FROM_RUST.get(artifact.key):
            builder.definition(labels.SNAPSHOT_ROOT_FROM_RUST[artifact.key])
            builder.drain()
        documents = {}
        if artifact.snapshot_path:
            snapshot = build_snapshot(artifact, builder, {})
            documents[artifact.snapshot_id] = snapshot
            snapshot = build_snapshot(artifact, builder, documents)
            documents[artifact.snapshot_id] = snapshot
            out[artifact.snapshot_path] = snapshot
        for leaf in artifact.leaves:
            out[leaf.schema_path] = build_leaf(artifact, leaf, builder, documents)
        out[f"{artifact.mutations}/🔣️.json"] = build_aggregate(artifact)
        for path, patch in labels.DOCUMENT_IDS.get(artifact.key, {}).items():
            document = read_json(f"{artifact.root}/{SUBSET}/{path}")
            head = OrderedDict([("$schema", DRAFT7), ("$id", patch)])
            for key, value in document.items():
                if key not in head:
                    head[key] = value
            out[f"{artifact.root}/{SUBSET}/{path}"] = rewrite_definitions(head)
        problems += [f"{artifact.key}: {problem}" for problem in builder.problems]
    return out, problems


#region 🔖️WireAttributes
#: 🗑️ Fixture keys no Rust field reads (FromValue ignores them, so they are not wire), dropped by the conversion.
DROPPED = []

def camel_targets(artifact):
    """🐫 Every type whose value wire this WP moves to camelCase: all leaf structs plus the artifact's listed payload records."""
    names = {leaf.struct.name for leaf in artifact.leaves}
    names.update(labels.CAMEL_VALUE.get(artifact.key, []))
    return names


class Shape:
    """🧭️ The wire attributes of one side of a fixture conversion: `serde` (test derive) or `value`, with camelCase overrides."""

    def __init__(self, artifact, mode, camel=()):
        self.artifact, self.mode, self.camel = artifact, mode, set(camel)

    def container(self, item):
        attrs = dict(item.serde if self.mode == "serde" else item.value)
        if item.name in self.camel:
            attrs["rename_all"] = "camelCase"
        return attrs

    def field(self, item, field):
        return rust.field_wire(field["ident"], (field["serde"] if self.mode == "serde" else field["value"]).get("rename"), self.container(item).get("rename_all"))

    def variant(self, item, variant):
        return rust.variant_wire(variant["ident"], (variant["serde"] if self.mode == "serde" else variant["value"]).get("rename"), self.container(item).get("rename_all"))


def convert(value, text, source, target, trail="$"):
    """🔁️ Re-encodes one JSON value of Rust type `text` from the `source` wire shape into the `target` one, type-directed (map keys and
    unknown shapes pass through untouched)."""
    tree = parse_type(text) if isinstance(text, str) else text
    name, args = tree[0], tree[1]
    if value is None:
        return None
    if name in ("()", "[;]", "[]") or name in SEQUENCES:
        inner = args if name == "()" else [args[0]] * len(value)
        return [convert(item, inner[index], source, target, f"{trail}[{index}]") for index, item in enumerate(value)] if isinstance(value, list) else value
    if name in PRIMITIVE_FLOAT | PRIMITIVE_INT_SIGNED | PRIMITIVE_INT_UNSIGNED | PRIMITIVE_STRING or name in ("bool", "DslValue", "Value"):
        return value
    if name == "Option" or name in WRAPPERS:
        return convert(value, args[0], source, target, trail)
    if name in MAPS:
        return OrderedDict((key, convert(item, args[1], source, target, f"{trail}.{key}")) for key, item in value.items()) if isinstance(value, dict) else value
    item = source.artifact.registry.lookup(name)
    if item is None:
        raise KeyError(f"{trail}: unknown type {name}")
    if item.kind == "alias":
        return convert(value, item.target, source, target, trail)
    if item.kind == "tuple" and len(item.fields) == 1:
        return convert(value, item.fields[0]["type"], source, target, trail)
    if item.kind == "struct":
        if not isinstance(value, dict):
            raise ValueError(f"{trail}: {name} expects an object")
        live = [field for field in item.fields if not field["value"].get("skip")]
        by_source = {source.field(item, field): field for field in live}
        by_spelling = {normalized(field["ident"]): field for field in live}
        out = OrderedDict()
        for key, entry in value.items():
            field = by_source.get(key) or by_spelling.get(normalized(key))
            if field is None:
                DROPPED.append(f"{trail}.{key}")
                continue
            out[target.field(item, field)] = convert(entry, field["type"], source, target, f"{trail}.{key}")
        fill = labels.FIXTURE_FILL.get(source.artifact.key, {}).get(name, {})
        if fill:
            ordered = OrderedDict()
            for field in live:
                wire = target.field(item, field)
                if wire in out:
                    ordered[wire] = out[wire]
                elif wire in fill:
                    ordered[wire] = fill[wire]
            out = ordered
        return out
    if item.kind == "enum":
        return convert_enum(value, item, source, target, trail)
    raise ValueError(f"{trail}: unsupported {item.kind} {name}")


def enum_split(value, item, shape, trail):
    attrs = shape.container(item)
    tag, content = attrs.get("tag"), attrs.get("content")
    wires = {shape.variant(item, variant): variant for variant in item.variants}
    if isinstance(value, str):
        return wires[value], None
    if tag:
        variant = wires[value[tag]]
        if content:
            return variant, value.get(content)
        return variant, OrderedDict((key, entry) for key, entry in value.items() if key != tag)
    (key, payload), = value.items()
    return wires[key], payload


def enum_join(variant, payload, item, shape):
    attrs = shape.container(item)
    tag, content = attrs.get("tag"), attrs.get("content")
    wire = shape.variant(item, variant)
    if variant["kind"] == "unit":
        return OrderedDict([(tag, wire)]) if tag else wire
    if tag and content:
        return OrderedDict([(tag, wire), (content, payload)])
    if tag:
        return OrderedDict([(tag, wire)] + list(payload.items()))
    return OrderedDict([(wire, payload)])


def convert_enum(value, item, source, target, trail):
    variant, payload = enum_split(value, item, source, trail)
    if variant["kind"] == "tuple":
        payload = convert(payload, variant["fields"][0], source, target, f"{trail}.{variant['ident']}")
    elif variant["kind"] == "named":
        case_source = variant["serde" if source.mode == "serde" else "value"].get("rename_all") or source.container(item).get("rename_all_fields")
        case_target = variant["value"].get("rename_all") or target.container(item).get("rename_all_fields")
        fields = {rust.field_wire(field["ident"], field[source.mode].get("rename"), case_source): field for field in variant["fields"]}
        payload = OrderedDict((rust.field_wire(fields[key]["ident"], fields[key]["value"].get("rename"), case_target), convert(entry, fields[key]["type"], source, target, f"{trail}.{key}")) for key, entry in payload.items())
    return enum_join(variant, payload, item, target)
#endregion 🔖️WireAttributes


#region 🔖️Fixtures
def fixture_cases(artifact):
    """🧫️ Every committed mutation fixture case of one artifact: `(case_dir, kind)` with `kind` the stripped kind directory."""
    for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, artifact.root)):
        dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated")]
        if os.path.basename(dirpath) != "🦠️mutation" or "🔣️.json" not in filenames:
            continue
        relative = os.path.relpath(dirpath, REPO)
        segments = relative.split("/")
        if not any(segment.endswith("fixtures") or segment == "🧪️tests" for segment in segments):
            continue
        anchor = max(index for index, segment in enumerate(segments) if segment == "🧬️mutations")
        yield "/".join(segments[:-1]), re.sub(r"^[^a-z]+", "", segments[anchor + 1])


def fixture_plan(artifact):
    """🗺️ `(rewrites {path: value}, orphans [case_dir])` for one artifact's fixtures."""
    rewrites, orphans = OrderedDict(), []
    by_kind = {leaf.kind: leaf for leaf in artifact.leaves}
    by_kind.update({re.sub(r"^[^a-z]+", "", os.path.basename(leaf.directory)): leaf for leaf in artifact.leaves})
    source_mode = labels.FIXTURE_SOURCE.get(artifact.key, "value")
    source = Shape(artifact, source_mode, labels.SOURCE_CAMEL.get(artifact.key, []))
    target = Shape(artifact, "value", camel_targets(artifact))
    aggregate = artifact.aggregate.name
    stale = labels.STALE_MODEL_CASES.get(artifact.key, [])
    for case, kind in fixture_cases(artifact):
        if kind not in by_kind or case.split("/🧬️mutations/")[-1] in stale:
            orphans.append(case)
            continue
        path = f"{case}/🦠️mutation/🔣️.json"
        raw = read_json(path)
        wire = raw
        if isinstance(raw, dict) and artifact.layout.get("tag") is None and "mutation" in raw and isinstance(raw["mutation"], str):
            leaf = by_kind[kind]
            wire = OrderedDict([(leaf.variant, OrderedDict((key, entry) for key, entry in raw.items() if key != "mutation"))])
        rewrites[path] = convert(wire, aggregate, source, target)
        if source_mode == "serde" or artifact.key in labels.SNAPSHOT_TYPES:
            for facet, type_name in (("📸️snapshot/⬅️before", labels.SNAPSHOT_TYPES[artifact.key]), ("📸️snapshot/➡️after", labels.SNAPSHOT_TYPES[artifact.key]), ("🔺️diff", labels.DIFF_TYPES[artifact.key])):
                facet_path = f"{case}/{facet}/🔣️.json"
                if os.path.exists(os.path.join(REPO, facet_path)):
                    rewrites[facet_path] = convert(read_json(facet_path), type_name, source, target)
    return rewrites, orphans
#endregion 🔖️Fixtures


#region 🔖️RustCamel
def attribute_block(text, start):
    """📜️ The attribute/doc lines directly above the item line starting at `start`, as `(block_start, block_text)`."""
    lines_start = start
    while True:
        previous_end = text.rfind("\n", 0, lines_start - 1)
        line = text[previous_end + 1 : lines_start - 1] if lines_start > 0 else ""
        stripped = line.strip()
        if lines_start == 0 or not (stripped.startswith("#[") or stripped.startswith("///") or stripped.endswith(")]") or stripped.endswith(",") or stripped.startswith("value_derive") or stripped.startswith("dsl::")):
            return lines_start, text[lines_start:start]
        lines_start = previous_end + 1


def camel_edit(text, name, want_value, want_serde):
    """🐫 Adds `rename_all = "camelCase"` to the value and/or serde attributes of item `name` in `text`."""
    match = re.search(r"(?m)^([ \t]*)pub (?:struct|enum) " + re.escape(name) + r"\b", text)
    if not match:
        raise KeyError(name)
    indent = match.group(1)
    block_start, block = attribute_block(text, match.start())
    edited = block
    inserted = ""
    if want_value and 'rename_all' not in (re.search(r"#\[value\(([^\]]*)\)\]", block) or [None, ""])[1]:
        if "#[value(" in block:
            edited = edited.replace("#[value(", '#[value(rename_all = "camelCase", ', 1)
        else:
            inserted += f'{indent}#[value(rename_all = "camelCase")]\n'
    if want_serde and "derive(serde::Serialize" in block:
        serde = re.search(r"#\[cfg_attr\(test, serde\(([^\]]*)\)\)\]", block)
        if serde is None:
            inserted = f'{indent}#[cfg_attr(test, serde(rename_all = "camelCase"))]\n' + inserted
        elif "rename_all" not in serde.group(1):
            edited = edited.replace("#[cfg_attr(test, serde(", '#[cfg_attr(test, serde(rename_all = "camelCase", ', 1)
    return text[:block_start] + edited + inserted + text[match.start():]


def rust_plan():
    """🗺️ `{path: new_text}` for every Rust file whose leaf/payload types move to camelCase on both wires."""
    edits = OrderedDict()
    for artifact in artifacts():
        targets = [(leaf.rust_file, leaf.struct.name, True, True) for leaf in artifact.leaves if leaf.struct.value.get("rename_all") != "camelCase" or leaf.struct.serde.get("rename_all") != "camelCase"]
        for name in labels.CAMEL_VALUE.get(artifact.key, []):
            item = artifact.registry.lookup(name)
            targets.append((os.path.relpath(item.path, REPO), name, True, True))
        for name in labels.CAMEL_SERDE.get(artifact.key, []):
            item = artifact.registry.lookup(name)
            targets.append((os.path.relpath(item.path, REPO), name, False, True))
        for path, name, want_value, want_serde in targets:
            text = edits.get(path) or open(os.path.join(REPO, path), encoding="utf-8").read()
            edits[path] = camel_edit(text, name, want_value, want_serde)
    return OrderedDict((path, text) for path, text in edits.items() if text != open(os.path.join(REPO, path), encoding="utf-8").read())
#endregion 🔖️RustCamel


#region 🔖️TypeScriptTwins
def ts_plan():
    """🟦️ `{path: text}` for every norm TypeScript twin whose `type X = {…}` / `interface X {…}` blocks name a Rust type whose value
    wire is camelCase but whose members are still spelled snake_case."""
    edits = OrderedDict()
    for artifact in artifacts():
        camel = {name for name, items in artifact.registry.by_name.items() if any(item.value.get("rename_all") == "camelCase" and item.kind == "struct" for item in items)}
        for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, artifact.root)):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "🧪️tests")]
            if "🟦️.ts" not in filenames or "🧬️schema" not in dirpath or "📝️text" in dirpath or "💾️binary" in dirpath:
                continue
            path = os.path.join(dirpath, "🟦️.ts")
            text = open(path, encoding="utf-8").read()
            out, name, depth = [], None, 0
            for line in text.split("\n"):
                head = re.match(r"\s*export (?:type (\w+) = \{|interface (\w+)(?: extends [^{]*)? \{)", line)
                if head and depth == 0:
                    name, depth = head.group(1) or head.group(2), 0
                if name is not None:
                    member = re.match(r"^(\s+(?:readonly\s+)?)([a-z]+(?:_[a-z0-9]+)+)(\??\s*:)", line)
                    if member and depth == 1 and name in camel:
                        line = member.group(1) + rust.field_wire(member.group(2), None, "camelCase") + line[member.end(2):]
                    depth += line.count("{") - line.count("}")
                    if depth <= 0:
                        name, depth = None, 0
                out.append(line)
            rewritten = "\n".join(out)
            if rewritten != text:
                edits[os.path.relpath(path, REPO)] = rewritten
    return edits
#endregion 🔖️TypeScriptTwins


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else "report"
    if command in ("ts", "ts-apply"):
        edits = ts_plan()
        print(f"[ts] {len(edits)} files to edit")
        import difflib
        for path, text in edits.items():
            if command == "ts-apply":
                with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
                    handle.write(text)
            elif "--show" in sys.argv:
                print("".join(difflib.unified_diff(open(os.path.join(REPO, path), encoding="utf-8").read().splitlines(True), text.splitlines(True), path, path, n=0)))
        sys.exit(0)
    if command in ("rust", "rust-apply"):
        edits = rust_plan()
        print(f"[rust] {len(edits)} files to edit")
        if command == "rust-apply":
            for path, text in edits.items():
                with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
                    handle.write(text)
        elif "--show" in sys.argv:
            import difflib
            for path, text in list(edits.items())[: int(sys.argv[sys.argv.index("--show") + 1])]:
                print("".join(difflib.unified_diff(open(os.path.join(REPO, path), encoding="utf-8").read().splitlines(True), text.splitlines(True), path, path, n=1)))
        sys.exit(0)
    selected = sys.argv[sys.argv.index("--only") + 1].split(",") if "--only" in sys.argv else None
    if command in ("fixtures", "fixtures-apply"):
        total_rewrites, total_orphans = 0, 0
        for artifact in artifacts():
            if selected and artifact.key not in selected:
                continue
            rewrites, orphans = fixture_plan(artifact)
            changed = [path for path, value in rewrites.items() if json.loads(open(os.path.join(REPO, path), encoding="utf-8").read()) != json.loads(json.dumps(value))]
            print(f"[fixtures] {artifact.key}: {len(rewrites)} files mapped, {len(changed)} to rewrite, {len(orphans)} orphan cases, {len(DROPPED)} unread keys dropped")
            for key in sorted(set(DROPPED)):
                print(f"[fixtures]   dropped {key}")
            DROPPED.clear()
            for case in orphans:
                print(f"[fixtures]   orphan {case.split('/🧬️mutations/')[-1]}")
            if command == "fixtures-apply":
                for path in changed:
                    write_json(path, rewrites[path])
                for case in orphans:
                    import shutil
                    shutil.rmtree(os.path.join(REPO, case))
                    kind_dir = os.path.dirname(os.path.join(REPO, case))
                    if not os.listdir(kind_dir):
                        os.rmdir(kind_dir)
            total_rewrites += len(changed)
            total_orphans += len(orphans)
        print(f"[fixtures] {total_rewrites} rewrites, {total_orphans} orphan cases")
        sys.exit(0)
    documents, problems = plan(selected)
    changed = []
    for path, value in documents.items():
        absolute = os.path.join(REPO, path)
        current = open(absolute, encoding="utf-8").read() if os.path.exists(absolute) else None
        if current is None or json.loads(current) != json.loads(json.dumps(value)) or current.rstrip("\n") != dump_json(value, False):
            changed.append(path)
    print(f"[parity] {len(documents)} documents, {len(changed)} to write, {len(problems)} problems")
    for problem in problems:
        print(f"[parity]   !! {problem}")
    if command == "apply":
        for path in changed:
            write_json(path, documents[path])
        print(f"[parity] wrote {len(changed)} documents")
    elif command == "dump":
        target = sys.argv[sys.argv.index("--out") + 1]
        with open(target, "w", encoding="utf-8") as handle:
            json.dump(documents, handle, ensure_ascii=False, indent=1)
