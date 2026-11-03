"""Schema-driven DSL emitter shared by the showcase builders: scalars as `key=value`, collections as headed tables (header lines copied from a committed asset), nested tables as `[ {k=v ...} ]`, records as `{k=v}`, null as `_`, enums kebab-cased."""
import re


def kebab(key):
    return re.sub(r"(?<=[a-zA-Z0-9])(?=[A-Z])", "-", key).lower()


def deref(schema, root):
    while True:
        for key in ("anyOf", "oneOf"):
            if key in schema:
                schema = next(s for s in schema[key] if s.get("type") != "null")
        if "$ref" not in schema:
            return schema
        schema = root["$defs"][schema["$ref"].rsplit("/", 1)[1]]


def cell(value, schema, root, units=None, unit=None):
    schema = deref(schema, root)
    if value is None:
        return "_"
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float)):
        return plain(value) + (unit or "")
    if isinstance(value, list):
        item = schema["items"]
        return "[ " + "".join(cell(v, item, root, units) + " " for v in value) + "]"
    if isinstance(value, dict):
        props = schema["properties"]
        return "{" + " ".join(f"{kebab(k)}={cell(v, props[k], root, units, (units or {}).get(k))}" for k, v in value.items()) + "}"
    if "enum" in schema:
        return kebab(value)
    return value if re.fullmatch(r"[A-Za-z0-9_.\-]+", value) else '"' + value.replace('"', '\\"') + '"'


def emit(doc, schema, template_text, magic, trailing=(), units=None):
    heads = {ln.split(" ", 1)[0]: ln for ln in template_text.split("\n") if re.match(r"^[a-z\-]+ \[.*\] \{$", ln)}
    props = schema["properties"]
    scalars = [k for k, v in doc.items() if not isinstance(v, list) and k not in trailing]
    out = [magic, " ".join(f"{kebab(k)}={doc[k].lower() if k in ('reliabilityClass', 'annex') else cell(doc[k], props[k], schema, units, (units or {}).get(k))}" for k in scalars)]
    for k, rows in ((k, v) for k, v in doc.items() if isinstance(v, list)):
        item = deref(props[k]["items"], schema)
        head = heads[kebab(k)]
        for f, spec in item["properties"].items():
            if "enum" in deref(spec, schema):
                head = head.replace(f"{kebab(f)}:TEXT", f"{kebab(f)}:ENUM")
        out.append(head)
        qty = {m.group(1) for m in re.finditer(r"([a-z0-9\-]+):QTY", heads[kebab(k)])}
        for r in rows:
            out.append("  " + " ".join(cell(r[f], item["properties"][f], schema, units, (units or {}).get(f) if kebab(f) in qty else None) for f in item["properties"]))
        out.append("}")
    for k in trailing:
        out.append(f"{kebab(k)}={doc[k].lower() if k == 'reliabilityClass' else cell(doc[k], props[k], schema)}")
    return "\n".join(out) + "\n"


def plain(value):
    return str(int(value)) if float(value) == int(value) else format(value, ".15f").rstrip("0") if abs(value) < 1e-4 else repr(float(value))
