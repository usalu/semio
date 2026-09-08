#!/usr/bin/env python3
"""🔗️ One-off derivation of `🔌️mcp/🧬️schema/🔣️.json` from the sibling `🔗️.graphql` SDL.

Run once to author the JSON Schema facet; afterwards both files are hand-owned and their parity is
re-derived at test time by `🔌️mcp/🧪️tests/🔬️schema/🟦️.ts`, never by re-running this script.
"""
import collections
import json
import re
import sys

SCALARS = {
    "String": {"type": "string"},
    "ID": {"type": "string"},
    "Int": {"type": "integer"},
    "Boolean": {"type": "boolean"},
    "Float": {"type": "number"},
    "DateTime": {"$ref": "#/$defs/DateTime"},
}


def parse(sdl):
    body = re.sub(r"^\s*#.*$", "", sdl, flags=re.M)
    definitions = []
    for match in re.finditer(r"^(type|input|interface|enum)\s+(\w+)(?:\s+implements\s+([\w\s&]+?))?\s*\{([^}]*)\}", body, re.M):
        definitions.append((match.group(1), match.group(2), (match.group(3) or "").split("&"), match.group(4)))
    return definitions


def field_type(text):
    text = text.strip()
    required = text.endswith("!")
    if required:
        text = text[:-1]
    if text.startswith("["):
        inner, inner_required = field_type(text[1:-1])
        shape = collections.OrderedDict([("type", "array"), ("items", inner)])
        del inner_required
    elif text in SCALARS:
        shape = collections.OrderedDict(SCALARS[text])
    else:
        shape = collections.OrderedDict([("$ref", f"#/$defs/{text}")])
    return shape, required


def nullable(shape):
    return collections.OrderedDict([("anyOf", [shape, collections.OrderedDict([("type", "null")])])])


def fields(text):
    out = []
    for line in text.split("\n"):
        line = line.strip()
        if not line:
            continue
        match = re.match(r"^(\w+)(\([^)]*\))?\s*:\s*(.+)$", line)
        if not match:
            raise SystemExit(f"unparsed field: {line}")
        out.append((match.group(1), bool(match.group(2)), match.group(3).strip()))
    return out


def main(sdl_path, out_path, schema_id):
    definitions = parse(open(sdl_path, encoding="utf8").read())
    defs = collections.OrderedDict()
    defs["DateTime"] = collections.OrderedDict([
        ("title", "DateTime"),
        ("description", "🕰️ The `DateTime` GraphQL scalar: an RFC-3339 instant, the encoding `encoding/json` gives a Go `time.Time`."),
        ("type", "string"),
        ("pattern", "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\\.[0-9]+)?(?:Z|[+-][0-9]{2}:[0-9]{2})$"),
    ])
    for kind, name, _implements, body in definitions:
        entry = collections.OrderedDict([("title", name)])
        if kind == "enum":
            entry["type"] = "string"
            entry["enum"] = [v.strip() for v in body.split("\n") if v.strip()]
        else:
            parsed = fields(body)
            if kind == "interface":
                entry["description"] = f"🔌️ The `{name}` GraphQL interface: every implementing type carries these fields and adds its own."
            elif any(has_arguments for _field, has_arguments, _type_text in parsed):
                entry["$comment"] = "Field arguments are declared in the 🔗️.graphql facet; a JSON instance is one already-resolved selection."
            entry["type"] = "object"
            entry["additionalProperties"] = kind == "interface"
            properties = collections.OrderedDict()
            required = []
            for field, _has_arguments, type_text in parsed:
                shape, is_required = field_type(type_text)
                properties[field] = shape if is_required else nullable(shape)
                if is_required:
                    required.append(field)
            if kind == "input":
                entry["required"] = required
            entry["properties"] = properties
        defs[name] = entry
    document = collections.OrderedDict([
        ("$schema", "http://json-schema.org/draft-07/schema#"),
        ("$id", schema_id),
        ("title", "RepoGraphqlSchema"),
        ("description", "🔗️ The `repo.client.mcp` scope: the repo GraphQL surface the CLI executor answers and the VS Code extension queries. The sibling 🔗️.graphql is the normative SDL; this document is the JSON Schema facet of the same exports, so responses and mutation inputs can be validated outside a GraphQL runtime."),
        ("$comment", "The root is one resolved `Query` selection: the `data` payload the CLI executor answers with. Every field is optional because a GraphQL selection chooses them; nullability is carried by each field's own shape."),
        ("$ref", "#/$defs/Query"),
        ("$defs", defs),
    ])
    open(out_path, "w", encoding="utf8").write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    print(f"{len(defs)} exports")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2], sys.argv[3])
