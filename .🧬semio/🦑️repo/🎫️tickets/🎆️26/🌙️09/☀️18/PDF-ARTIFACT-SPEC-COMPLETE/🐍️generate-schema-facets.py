"""🧬️ Derives the language-neutral facets of the 📖️pdf 1.7/🧱️base schema scopes from the Rust
sources: JSON Schema (draft-07), TypeScript (types + validating parsers), GraphQL and Protobuf
for the snapshot, the diff, every mutation leaf and the mutation aggregate — plus the Rust,
binary and text files of NEW mutation leaves from the spec table below.

The Rust model is the single source of truth (`#[derive(value_derive::ToValue)]` with
`rename_all = "camelCase"`, tagged enums, `Option`/`Vec`/arrays/tuples/`Box`); this script reads
exactly that subset. Re-run after any model change:

    python3 🐍️generate-schema-facets.py <snapshot.rs> <diff.rs> <mutations-dir> [--rust-only]

Ticket 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE.
"""
import json, os, re, sys

SNAPSHOT_RS, DIFF_RS, MUTATIONS_DIR = sys.argv[1], sys.argv[2], sys.argv[3]
BASE_URL = "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base"
SNAPSHOT_DIR = os.path.join(os.path.dirname(MUTATIONS_DIR), "📸️snapshot")
DIFF_DIR = os.path.join(os.path.dirname(MUTATIONS_DIR), "🔺️diff")

# ───────────────────────────── Rust parsing ─────────────────────────────
def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def split_top(s, sep=","):
    out, depth, cur = [], 0, ""
    for ch in s:
        if ch in "<([{":
            depth += 1
        elif ch in ">)]}":
            depth -= 1
        if ch == sep and depth == 0:
            out.append(cur.strip()); cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out

def camel(name):
    parts = name.split("_")
    return parts[0] + "".join(p[:1].upper() + p[1:] for p in parts[1:])

def parse_types(src):
    """Returns {name: {'kind': 'struct'|'enum', 'generics': [..], 'fields'|'variants', 'attrs'}}"""
    src = strip_comments(src)
    types = {}
    for m in re.finditer(r"((?:#\[[^\]]*\]\s*)*)pub\s+(struct|enum)\s+(\w+)(?:<([^>]*)>)?\s*\{", src):
        attrs, kind, name, generics = m.group(1), m.group(2), m.group(3), m.group(4)
        # `PdfSnapshot` carries a hand-written `ToValue`/`FromValue` pair with derive semantics
        # (camelCase keys, every field defaulting) — admit it like a derived struct.
        if "ToValue" not in attrs and name != "PdfSnapshot":
            continue
        start = m.end()
        depth, i = 1, start
        while depth:
            if src[i] == "{": depth += 1
            elif src[i] == "}": depth -= 1
            i += 1
        body = src[start:i - 1]
        vattr = re.search(r"#\[value\(([^\]]*)\)\]", attrs)
        vattrs = vattr.group(1) if vattr else ""
        tag = re.search(r'tag\s*=\s*"(\w+)"', vattrs)
        entry = {"kind": kind, "generics": [g.strip() for g in generics.split(",")] if generics else [], "tag": tag.group(1) if tag else None, "fields_camel": "rename_all_fields" in vattrs or kind == "struct"}
        if kind == "struct":
            entry["fields"] = parse_fields(body)
        else:
            entry["variants"] = parse_variants(body)
        types[name] = entry
    return types

def parse_fields(body):
    fields = []
    for chunk in split_top(body):
        chunk = chunk.strip()
        if not chunk:
            continue
        attrs = "".join(re.findall(r"#\[[^\]]*\]", chunk))
        chunk = re.sub(r"#\[[^\]]*\]", "", chunk).strip()
        m = re.match(r"pub\s+(\w+)\s*:\s*(.+)$", chunk, re.S)
        if not m:
            continue
        optional = "default" in attrs or m.group(2).strip().startswith("Option<")
        fields.append({"name": m.group(1), "type": m.group(2).strip(), "optional": optional})
    return fields

def parse_variants(body):
    variants = []
    for chunk in split_top(body):
        chunk = re.sub(r"#\[[^\]]*\]", "", chunk).strip()
        if not chunk:
            continue
        m = re.match(r"(\w+)\s*(\{.*\}|\(.*\))?$", chunk, re.S)
        if not m:
            continue
        name, payload = m.group(1), m.group(2)
        if payload is None:
            variants.append({"name": name, "shape": "unit"})
        elif payload.startswith("{"):
            variants.append({"name": name, "shape": "struct", "fields": [{"name": f.split(":")[0].strip(), "type": ":".join(f.split(":")[1:]).strip(), "optional": ":".join(f.split(":")[1:]).strip().startswith("Option<")} for f in split_top(payload[1:-1]) if f.strip()]})
        else:
            variants.append({"name": name, "shape": "newtype", "type": payload[1:-1].strip()})
    return variants

# ───────────────────────────── type resolution ─────────────────────────────
class Facets:
    def __init__(self, types):
        self.types = types
        self.instances = {}  # monomorphized generic instances: mono_name -> (base, args)
        self.order = []

    def mono_name(self, base, args):
        return base + "".join(self.tname(a) for a in args)

    def tname(self, rust):
        rust = rust.strip()
        m = re.match(r"(\w+)<(.*)>$", rust, re.S)
        if m and m.group(1) in self.types and self.types[m.group(1)]["generics"]:
            args = split_top(m.group(2))
            name = self.mono_name(m.group(1), args)
            if name not in self.instances:
                self.instances[name] = (m.group(1), args)
                self.order.append(name)
            return name
        if rust in self.types:
            return rust
        return rust.replace("<", "_").replace(">", "").replace(",", "_").replace(" ", "").replace("[", "Arr").replace("]", "").replace(";", "x")

    def json(self, rust, subst=None, doc=None):
        subst = subst or {}
        rust = rust.strip()
        if rust in subst:
            rust = subst[rust]
        if rust == "String" or rust == "&str":
            return {"type": "string"}
        if rust == "bool":
            return {"type": "boolean"}
        if rust in ("u8", "u16", "u32", "u64", "usize"):
            return {"type": "integer", "minimum": 0}
        if rust in ("i8", "i16", "i32", "i64", "isize"):
            return {"type": "integer"}
        if rust in ("f32", "f64"):
            return {"type": "number"}
        m = re.match(r"Vec<(.*)>$", rust, re.S)
        if m:
            return {"type": "array", "items": self.json(m.group(1), subst, doc)}
        m = re.match(r"Option<(.*)>$", rust, re.S)
        if m:
            return {"anyOf": [self.json(m.group(1), subst, doc), {"type": "null"}]}
        m = re.match(r"Box<(.*)>$", rust, re.S)
        if m:
            return self.json(m.group(1), subst, doc)
        m = re.match(r"\[(.*);\s*(\d+)\]$", rust, re.S)
        if m:
            n = int(m.group(2))
            return {"type": "array", "items": self.json(m.group(1), subst, doc), "minItems": n, "maxItems": n}
        m = re.match(r"\((.*)\)$", rust, re.S)
        if m:
            items = [self.json(p, subst, doc) for p in split_top(m.group(1))]
            return {"type": "array", "items": items, "minItems": len(items), "maxItems": len(items)}
        if rust in ("PdfRect", "PdfMatrix"):
            n = 4 if rust == "PdfRect" else 6
            return {"type": "array", "items": {"type": "number"}, "minItems": n, "maxItems": n}
        name = self.tname(rust)
        owner = self.owner(name)
        if doc is not None and owner != doc:
            return {"$ref": f"{BASE_URL}/{owner}.json#/$defs/{name}"}
        return {"$ref": f"#/$defs/{name}"}

    def owner(self, name):
        base = self.instances.get(name, (name, None))[0]
        return OWNER.get(base, "snapshot")

    def define(self, name):
        base, args = self.instances.get(name, (name, None))
        t = self.types[base]
        subst = dict(zip(t["generics"], args)) if args else {}
        doc = self.owner(name)
        if t["kind"] == "struct":
            props, required = {}, []
            for f in t["fields"]:
                props[camel(f["name"])] = self.json(f["type"], subst, doc)
                if not f["optional"]:
                    required.append(camel(f["name"]))
            out = {"type": "object", "properties": props}
            if required:
                out["required"] = required
            return out
        variants = t["variants"]
        if all(v["shape"] == "unit" for v in variants):
            return {"type": "string", "enum": [camel_variant(v["name"]) for v in variants]}
        tag = t["tag"] or "kind"
        one = []
        for v in variants:
            props = {tag: {"const": camel_variant(v["name"])}}
            required = [tag]
            if v["shape"] == "newtype":
                props["value"] = self.json(v["type"], subst, doc)
                required.append("value")
            elif v["shape"] == "struct":
                for f in v["fields"]:
                    key = camel(f["name"]) if t["fields_camel"] else f["name"]
                    props[key] = self.json(f["type"], subst, doc)
                    if not f["optional"]:
                        required.append(key)
            one.append({"type": "object", "properties": props, "required": required})
        return {"oneOf": one}

    # TypeScript
    def ts(self, rust, subst=None):
        subst = subst or {}
        rust = rust.strip()
        if rust in subst:
            rust = subst[rust]
        if rust in ("String", "&str"):
            return "string"
        if rust == "bool":
            return "boolean"
        if rust in ("u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize", "f32", "f64"):
            return "number"
        m = re.match(r"Vec<(.*)>$", rust, re.S)
        if m:
            inner = self.ts(m.group(1), subst)
            return f"({inner})[]" if "|" in inner else f"{inner}[]"
        m = re.match(r"Option<(.*)>$", rust, re.S)
        if m:
            return f"{self.ts(m.group(1), subst)} | null"
        m = re.match(r"Box<(.*)>$", rust, re.S)
        if m:
            return self.ts(m.group(1), subst)
        m = re.match(r"\[(.*);\s*(\d+)\]$", rust, re.S)
        if m:
            return "[" + ", ".join([self.ts(m.group(1), subst)] * int(m.group(2))) + "]"
        m = re.match(r"\((.*)\)$", rust, re.S)
        if m:
            return "[" + ", ".join(self.ts(p, subst) for p in split_top(m.group(1))) + "]"
        if rust == "PdfRect":
            return "[number, number, number, number]"
        if rust == "PdfMatrix":
            return "[number, number, number, number, number, number]"
        return self.tname(rust)

    def ts_define(self, name):
        base, args = self.instances.get(name, (name, None))
        t = self.types[base]
        subst = dict(zip(t["generics"], args)) if args else {}
        if t["kind"] == "struct":
            lines = [f"export interface {name} {{"]
            for f in t["fields"]:
                opt = "?" if f["optional"] else ""
                lines.append(f"  {camel(f['name'])}{opt}: {self.ts(f['type'], subst)};")
            lines.append("}")
            return "\n".join(lines)
        variants = t["variants"]
        if all(v["shape"] == "unit" for v in variants):
            return f"export type {name} =\n  | " + "\n  | ".join(json.dumps(camel_variant(v["name"])) for v in variants) + ";"
        tag = t["tag"] or "kind"
        alts = []
        for v in variants:
            parts = [f"{tag}: {json.dumps(camel_variant(v['name']))}"]
            if v["shape"] == "newtype":
                parts.append(f"value: {self.ts(v['type'], subst)}")
            elif v["shape"] == "struct":
                for f in v["fields"]:
                    key = camel(f["name"]) if t["fields_camel"] else f["name"]
                    opt = "?" if f["optional"] else ""
                    parts.append(f"{key}{opt}: {self.ts(f['type'], subst)}")
            alts.append("{ " + "; ".join(parts) + " }")
        return f"export type {name} =\n  | " + "\n  | ".join(alts) + ";"

    # GraphQL
    def gql(self, rust, subst=None, inp=False):
        subst = subst or {}
        rust = rust.strip()
        if rust in subst:
            rust = subst[rust]
        if rust in ("String", "&str"):
            return "String"
        if rust == "bool":
            return "Boolean"
        if rust in ("u8", "u16", "u32", "i8", "i16", "i32", "usize", "u64", "i64", "isize"):
            return "Int"
        if rust in ("f32", "f64"):
            return "Float"
        m = re.match(r"Vec<(.*)>$", rust, re.S)
        if m:
            return f"[{self.gql(m.group(1), subst, inp)}!]"
        m = re.match(r"Option<(.*)>$", rust, re.S)
        if m:
            return self.gql(m.group(1), subst, inp)
        m = re.match(r"Box<(.*)>$", rust, re.S)
        if m:
            return self.gql(m.group(1), subst, inp)
        m = re.match(r"\[(.*);\s*(\d+)\]$", rust, re.S)
        if m:
            return f"[{self.gql(m.group(1), subst, inp)}!]"
        m = re.match(r"\((.*)\)$", rust, re.S)
        if m:
            return "[JSON!]"
        if rust in ("PdfRect", "PdfMatrix"):
            return "[Float!]"
        return self.tname(rust) + ("Input" if inp else "")

    def gql_define(self, name, inp=False):
        base, args = self.instances.get(name, (name, None))
        t = self.types[base]
        subst = dict(zip(t["generics"], args)) if args else {}
        keyword = "input" if inp else "type"
        suffix = "Input" if inp else ""
        if t["kind"] == "struct":
            lines = [f"{keyword} {name}{suffix} {{"]
            for f in t["fields"]:
                bang = "" if f["optional"] else "!"
                lines.append(f"  {camel(f['name'])}: {self.gql(f['type'], subst, inp)}{bang}")
            lines.append("}")
            return "\n".join(lines)
        variants = t["variants"]
        if all(v["shape"] == "unit" for v in variants):
            if inp:
                return ""
            return f"enum {name} {{\n" + "\n".join("  " + gql_enum(v["name"]) for v in variants) + "\n}"
        tag = t["tag"] or "kind"
        lines = [f'"""Tagged union: exactly the fields of the variant `{tag}` names are populated."""', f"{keyword} {name}{suffix} {{", f"  {tag}: String!"]
        seen = set()
        for v in variants:
            if v["shape"] == "newtype":
                key = "value"
                if key not in seen:
                    seen.add(key); lines.append(f"  value: {self.gql(v['type'], subst, inp)}")
            elif v["shape"] == "struct":
                for f in v["fields"]:
                    key = camel(f["name"]) if t["fields_camel"] else f["name"]
                    if key in seen:
                        continue
                    seen.add(key)
                    lines.append(f"  {key}: {self.gql(f['type'], subst, inp)}")
        lines.append("}")
        return "\n".join(lines)

    # Protobuf
    def proto(self, rust, subst=None):
        subst = subst or {}
        rust = rust.strip()
        if rust in subst:
            rust = subst[rust]
        if rust in ("String", "&str"):
            return ("string", False)
        if rust == "bool":
            return ("bool", False)
        if rust in ("u8", "u16", "u32"):
            return ("uint32", False)
        if rust in ("u64", "usize"):
            return ("uint64", False)
        if rust in ("i8", "i16", "i32"):
            return ("int32", False)
        if rust in ("i64", "isize"):
            return ("int64", False)
        if rust in ("f32", "f64"):
            return ("double", False)
        if rust == "Vec<u8>":
            return ("bytes", False)
        m = re.match(r"Vec<(.*)>$", rust, re.S)
        if m:
            inner, rep = self.proto(m.group(1), subst)
            if rep:
                return ("bytes", False)  # nested repeats are carried as canonical JSON bytes
            return (inner, True)
        m = re.match(r"Option<(.*)>$", rust, re.S)
        if m:
            return self.proto(m.group(1), subst)
        m = re.match(r"Box<(.*)>$", rust, re.S)
        if m:
            return self.proto(m.group(1), subst)
        m = re.match(r"\[(.*);\s*(\d+)\]$", rust, re.S)
        if m:
            if m.group(1).strip() == "Vec<u8>":
                return ("bytes", True)
            inner, rep = self.proto(m.group(1), subst)
            return (inner, True)
        m = re.match(r"\((.*)\)$", rust, re.S)
        if m:
            return ("bytes", False)
        if rust in ("PdfRect", "PdfMatrix"):
            return ("double", True)
        return (self.tname(rust), False)

    def proto_define(self, name):
        base, args = self.instances.get(name, (name, None))
        t = self.types[base]
        subst = dict(zip(t["generics"], args)) if args else {}
        if t["kind"] == "struct":
            lines = [f"message {name} {{"]
            for index, f in enumerate(t["fields"], 1):
                ty, rep = self.proto(f["type"], subst)
                prefix = "repeated " if rep else ("optional " if f["optional"] and not rep and ty in ("string", "bool", "uint32", "uint64", "int32", "int64", "double", "bytes") else "")
                lines.append(f"  {prefix}{ty} {snake(f['name'])} = {index};")
            lines.append("}")
            return "\n".join(lines)
        variants = t["variants"]
        if all(v["shape"] == "unit" for v in variants):
            return f"enum {name} {{\n" + "\n".join(f"  {name.upper()}_{v['name'].upper()} = {i};" for i, v in enumerate(variants)) + "\n}"
        tag = t["tag"] or "kind"
        lines = [f"message {name} {{", f"  string {tag} = 1;"]
        seen, index = set(), 2
        for v in variants:
            fields = [("value", v["type"])] if v["shape"] == "newtype" else ([(f["name"], f["type"]) for f in v["fields"]] if v["shape"] == "struct" else [])
            for fname, ftype in fields:
                key = snake(fname)
                if key in seen:
                    continue
                seen.add(key)
                ty, rep = self.proto(ftype, subst)
                lines.append(f"  {'repeated ' if rep else 'optional ' if ty in ('string','bool','uint32','uint64','int32','int64','double','bytes') else ''}{ty} {key} = {index};")
                index += 1
        lines.append("}")
        return "\n".join(lines)

def camel_variant(name):
    return name[0].lower() + name[1:]

def gql_enum(name):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).upper()

def snake(name):
    return name

# ───────────────────────────── ownership ─────────────────────────────
snapshot_types = parse_types(open(SNAPSHOT_RS, encoding="utf-8").read())
diff_types = parse_types(open(DIFF_RS, encoding="utf-8").read())
OWNER = {name: "snapshot" for name in snapshot_types}
OWNER.update({name: "diff" for name in diff_types})
all_types = dict(snapshot_types)
all_types.update(diff_types)
facets = Facets(all_types)

def closure(roots):
    """Every named type reachable from roots (instances registered on the way)."""
    seen, stack = [], list(roots)
    while stack:
        name = stack.pop()
        if name in seen or name not in facets.types and name not in facets.instances:
            continue
        seen.append(name)
        base, args = facets.instances.get(name, (name, None))
        t = facets.types[base]
        subst = dict(zip(t["generics"], args)) if args else {}
        refs = []
        def collect(rust):
            rust = rust.strip()
            if rust in subst:
                rust = subst[rust]
            for inner in re.findall(r"\w+(?:<[^<>]*(?:<[^<>]*>[^<>]*)*>)?", rust):
                pass
            # walk generics
            m = re.match(r"(\w+)<(.*)>$", rust, re.S)
            if m and m.group(1) in facets.types and facets.types[m.group(1)]["generics"]:
                refs.append(facets.tname(rust))
                return
            if m:
                for a in split_top(m.group(2)):
                    collect(a)
                return
            m = re.match(r"\[(.*);\s*\d+\]$", rust, re.S)
            if m:
                collect(m.group(1)); return
            m = re.match(r"\((.*)\)$", rust, re.S)
            if m:
                for a in split_top(m.group(1)):
                    collect(a)
                return
            if rust in facets.types:
                refs.append(rust)
        if t["kind"] == "struct":
            for f in t["fields"]:
                collect(f["type"])
        else:
            for v in t["variants"]:
                if v["shape"] == "newtype":
                    collect(v["type"])
                elif v["shape"] == "struct":
                    for f in v["fields"]:
                        collect(f["type"])
        stack.extend(r for r in refs if r not in seen)
    return seen

# ───────────────────────────── documents ─────────────────────────────
def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)

TS_VALIDATOR = '''
//#region 🚪️Validation
type Schema = Record<string, unknown>;
export class SchemaRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}
const documents = new Map<string, Schema>();
export const registerSchemaDocument = (schema: Schema): void => {
  documents.set(String(schema["$id"]), schema);
};
const resolveRef = (ref: string, own: Schema): Schema => {
  const [documentId, pointer] = ref.split("#");
  const document = documentId === "" ? own : documents.get(documentId);
  if (!document) throw new SchemaRefusal("$ref", `unknown schema document ${documentId}`);
  let node: unknown = document;
  for (const step of (pointer ?? "").split("/").filter((s) => s.length > 0)) node = (node as Record<string, unknown>)[step];
  if (!node) throw new SchemaRefusal("$ref", `unresolved pointer ${ref}`);
  return node as Schema;
};
const matches = (schema: Schema, value: unknown, own: Schema, at: string, errors: string[]): boolean => {
  if (typeof schema["$ref"] === "string") return matches(resolveRef(schema["$ref"] as string, own), value, own, at, errors);
  if (schema["const"] !== undefined) return value === schema["const"] || (errors.push(`${at}: expected ${JSON.stringify(schema["const"])}`), false);
  if (Array.isArray(schema["enum"])) return (schema["enum"] as unknown[]).includes(value) || (errors.push(`${at}: not one of ${(schema["enum"] as unknown[]).join(", ")}`), false);
  if (Array.isArray(schema["anyOf"])) return (schema["anyOf"] as Schema[]).some((s) => matches(s, value, own, at, [])) || (errors.push(`${at}: matches no alternative`), false);
  if (Array.isArray(schema["oneOf"])) return (schema["oneOf"] as Schema[]).filter((s) => matches(s, value, own, at, [])).length === 1 || (errors.push(`${at}: matches no single alternative`), false);
  if (Array.isArray(schema["allOf"])) return (schema["allOf"] as Schema[]).every((s) => matches(s, value, own, at, errors));
  const types = Array.isArray(schema["type"]) ? (schema["type"] as string[]) : typeof schema["type"] === "string" ? [schema["type"] as string] : [];
  const kind = value === null ? "null" : Array.isArray(value) ? "array" : typeof value === "number" ? (Number.isInteger(value) ? "integer" : "number") : typeof value;
  if (types.length > 0 && !types.includes(kind) && !(kind === "integer" && types.includes("number"))) return (errors.push(`${at}: expected ${types.join("|")}, found ${kind}`), false);
  if (kind === "integer" || kind === "number") {
    if (typeof schema["minimum"] === "number" && (value as number) < (schema["minimum"] as number)) return (errors.push(`${at}: below ${schema["minimum"]}`), false);
    if (typeof schema["maximum"] === "number" && (value as number) > (schema["maximum"] as number)) return (errors.push(`${at}: above ${schema["maximum"]}`), false);
  }
  if (kind === "array") {
    const items = value as unknown[];
    if (typeof schema["minItems"] === "number" && items.length < (schema["minItems"] as number)) return (errors.push(`${at}: fewer than ${schema["minItems"]} items`), false);
    if (typeof schema["maxItems"] === "number" && items.length > (schema["maxItems"] as number)) return (errors.push(`${at}: more than ${schema["maxItems"]} items`), false);
    if (Array.isArray(schema["items"])) return items.every((item, index) => matches((schema["items"] as Schema[])[index] ?? {}, item, own, `${at}[${index}]`, errors));
    if (schema["items"]) return items.every((item, index) => matches(schema["items"] as Schema, item, own, `${at}[${index}]`, errors));
  }
  if (kind === "object") {
    const row = value as Record<string, unknown>;
    for (const key of (schema["required"] as string[] | undefined) ?? []) if (row[key] === undefined) return (errors.push(`${at}.${key}: missing`), false);
    const properties = (schema["properties"] as Record<string, Schema> | undefined) ?? {};
    for (const [key, sub] of Object.entries(properties)) if (row[key] !== undefined && !matches(sub, row[key], own, `${at}.${key}`, errors)) return false;
  }
  return true;
};
export const validateAgainst = <T,>(schema: Schema, pointer: string, value: unknown): T => {
  const node = pointer === "" ? schema : resolveRef(`#${pointer}`, schema);
  const errors: string[] = [];
  if (!matches(node, value, schema, "$", errors)) throw new SchemaRefusal("$", errors[0] ?? "invalid");
  return value as T;
};
//#endregion 🚪️Validation
'''

def ts_header_doc(title, doc):
    return f"/** 🧬️ {title} — TypeScript facet of `s.stdio.pdf.1.7` ({doc}), generated from the Rust model\n *  by 🐍️generate-schema-facets.py (ticket 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE). */\n"

def emit_document(doc, title, roots, out_dir, extra_ts_imports=""):
    names = closure(roots)
    own = [n for n in names if facets.owner(n) == doc]
    # JSON schema
    defs = {n: facets.define(n) for n in own}
    root_def = defs.pop(title) if title in defs else None
    schema = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"{BASE_URL}/{doc}.json", "title": title}
    if root_def:
        schema.update(root_def)
    schema["$defs"] = dict(sorted(defs.items()))
    write(os.path.join(out_dir, "🔣️.json"), json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
    # TypeScript
    foreign = sorted({n for n in names if facets.owner(n) != doc})
    ts = ts_header_doc(title, doc)
    if foreign:
        other = "📸️snapshot" if doc != "snapshot" else "🔺️diff"
        ts += f"import type {{ {', '.join(foreign)} }} from '../{other}/🟦️.ts';\n"
        ts += f"import {{ schema as {'snapshot' if doc != 'snapshot' else 'diff'}Schema, registerSchemaDocument as registerOther, validateAgainst as _validateOther }} from '../{other}/🟦️.ts';\n"
    ts += "\n" + "\n\n".join(facets.ts_define(n) for n in own) + "\n"
    ts += f"\n/** 🔣️ The JSON Schema document this facet is validated against. */\nexport const schema = {json.dumps(schema, indent=2, ensure_ascii=False)} as const;\n"
    ts += TS_VALIDATOR
    ts += "registerSchemaDocument(schema);\n"
    if foreign:
        ts += f"registerOther({'snapshot' if doc != 'snapshot' else 'diff'}Schema);\n"
    for n in own:
        pointer = "" if n == title else f"/$defs/{n}"
        ts += f"export const parse{n} = (value: unknown): {n} => validateAgainst<{n}>(schema, {json.dumps(pointer)}, value);\n"
    write(os.path.join(out_dir, "🟦️.ts"), ts)
    # GraphQL
    gql = f"# 🧬️ {title} — GraphQL facet of s.stdio.pdf.1.7 ({doc}), generated from the Rust model.\nscalar JSON\n\n"
    gql += "\n\n".join(x for x in (facets.gql_define(n) for n in own) if x) + "\n"
    if doc == "snapshot":
        gql += "\n" + "\n\n".join(x for x in (facets.gql_define(n, inp=True) for n in own) if x) + "\n"
    write(os.path.join(out_dir, "🔗️.graphql"), gql)
    # Protobuf
    proto = f'syntax = "proto3";\npackage semio.s_stdio_pdf_1_7.{doc};\n\n'
    if doc != "snapshot":
        proto += 'import "../📸️snapshot/🛰️.proto";\n\n'
    proto += "\n\n".join(facets.proto_define(n) for n in own) + "\n"
    write(os.path.join(out_dir, "🛰️.proto"), proto)
    return names

snapshot_names = emit_document("snapshot", "PdfSnapshot", ["PdfSnapshot"], SNAPSHOT_DIR)
diff_names = emit_document("diff", "PdfDiff", ["PdfDiff"], DIFF_DIR)
print(f"snapshot: {len([n for n in snapshot_names if facets.owner(n)=='snapshot'])} types, diff: {len([n for n in diff_names if facets.owner(n)=='diff'])} types")

# ───────────────────────────── mutation leaves ─────────────────────────────
LEAVES = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🔣️mutation-leaves.json"), encoding="utf-8"))

def leaf_payload_types(leaf):
    """Parses the leaf's Rust payload struct from its 🦀️.rs (already written)."""
    src = open(os.path.join(MUTATIONS_DIR, leaf["directory"], "🦀️.rs"), encoding="utf-8").read()
    types = parse_types(src)
    return types

aggregate_ts_imports, aggregate_ts_union, aggregate_json, aggregate_proto, aggregate_gql = [], [], [], [], []
for index, leaf in enumerate(LEAVES):
    pascal = leaf["pascal"]
    kebab = leaf["kebab"]
    directory = os.path.join(MUTATIONS_DIR, leaf["directory"])
    types = leaf_payload_types(leaf)
    payload = types[pascal]
    OWNER[pascal] = f"mutation/{kebab}"
    facets.types[pascal] = payload
    names = closure([pascal])
    props, required = {}, []
    for f in payload["fields"]:
        props[camel(f["name"])] = facets.json(f["type"], {}, f"mutation/{kebab}")
        if not f["optional"]:
            required.append(camel(f["name"]))
    props["mutation"] = {"const": camel_variant(pascal), "description": "Aggregate discriminator spliced in by PdfMutation; absent when the payload stands alone."}
    schema = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"{BASE_URL}/mutation/{kebab}/schema.json", "title": pascal, "type": "object", "additionalProperties": False, "required": required, "properties": props}
    write(os.path.join(directory, "🧬️schema", "🔣️.json"), json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
    manifest = {"schemaVersion": 1, "owner": f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/{leaf['directory']}", "semanticKind": kebab, "displayName": leaf["display"], "emoji": leaf["emoji"], "aggregateVariant": pascal, "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": kebab, "binaryTag": index, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]}
    write(os.path.join(directory, "🔣️.json"), json.dumps(manifest, indent=2, ensure_ascii=False) + "\n")
    foreign = sorted({n for n in names if n != pascal and facets.owner(n) in ("snapshot", "diff")})
    ts = f"/** {leaf['emoji']} Direct {kebab} TypeScript payload. */\n"
    snap = [n for n in foreign if facets.owner(n) == "snapshot"]
    dif = [n for n in foreign if facets.owner(n) == "diff"]
    if snap:
        ts += f"import type {{ {', '.join(snap)} }} from '../../📸️snapshot/🟦️.ts';\n"
    if dif:
        ts += f"import type {{ {', '.join(dif)} }} from '../../🔺️diff/🟦️.ts';\n"
    ts += f"export interface {pascal}Mutation {{\n  mutation: '{camel_variant(pascal)}';\n"
    for f in payload["fields"]:
        ts += f"  {camel(f['name'])}{'?' if f['optional'] else ''}: {facets.ts(f['type'])};\n"
    ts += "}\n"
    write(os.path.join(directory, "🟦️.ts"), ts)
    proto = f'syntax = "proto3";\npackage semio.s_stdio_pdf_1_7.mutation.{kebab.replace("-", "_")};\n\nimport "../../📸️snapshot/🛰️.proto";\nimport "../../🔺️diff/🛰️.proto";\n\n// {leaf["emoji"]} Direct {kebab} protobuf payload.\nmessage {pascal}Mutation {{\n'
    for i, f in enumerate(payload["fields"], 1):
        ty, rep = facets.proto(f["type"])
        prefix = "repeated " if rep else ""
        proto += f"  {prefix}{ty} {f['name']} = {i};\n"
    proto += "}\n"
    write(os.path.join(directory, "🛰️.proto"), proto)
    gql = f"# {leaf['emoji']} Direct {kebab} GraphQL payload.\ninput {pascal}MutationInput {{\n"
    for f in payload["fields"]:
        gql += f"  {camel(f['name'])}: {facets.gql(f['type'], {}, True)}\n"
    gql += "}\n"
    write(os.path.join(directory, "🔗️.graphql"), gql)
    binary = f"//! {leaf['emoji']} Direct binary identity for `{kebab}`.\n\npub const TAG: u8 = {index};\npub const BINARY_TAG: u8 = TAG;\n\nuse super::{pascal};\n\n/// 📤️ Encodes this direct payload as canonical schema JSON bytes.\npub fn encode(payload: &{pascal}) -> Result<Vec<u8>, String> {{\n    Ok(pack::to_json_string(payload).into_bytes())\n}}\n\n/// 📥️ Decodes this direct payload from canonical schema JSON bytes.\npub fn decode(bytes: &[u8]) -> Result<{pascal}, String> {{\n    let parsed = pack::parse_json_bytes(bytes).map_err(|error| error.to_string())?;\n    dsl::FromValue::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())\n}}\n"
    write(os.path.join(directory, "💾️binary", "🦀️.rs"), binary)
    text = f"//! {leaf['emoji']} Direct text identity for `{kebab}`.\n\npub const OPCODE: &str = \"{kebab}\";\npub const TEXT_OPCODE: &str = OPCODE;\n\nuse super::{pascal};\n\n/// 🖨️ Prints this direct payload through its schema-derived JSON representation.\npub fn print(payload: &{pascal}) -> Result<String, String> {{\n    Ok(pack::to_json_string(payload))\n}}\n\n/// 📥️ Parses this direct payload through its schema-derived JSON representation.\npub fn parse(text: &str) -> Result<{pascal}, String> {{\n    pack::from_json_str(text).map_err(|error| error.to_string())\n}}\n"
    write(os.path.join(directory, "📝️text", "🦀️.rs"), text)
    aggregate_ts_imports.append(f"import type {{ {pascal}Mutation }} from './{leaf['directory']}/🟦️.ts';")
    aggregate_ts_union.append(f"  | {pascal}Mutation")
    aggregate_json.append({"allOf": [{"$ref": f"{BASE_URL}/mutation/{kebab}/schema.json"}, {"type": "object", "required": ["mutation"], "properties": {"mutation": {"const": camel_variant(pascal)}}}]})
    aggregate_proto.append(f"    {pascal}Mutation {kebab.replace('-', '_')} = {index + 1};")
    aggregate_gql.append(f"  {camel_variant(pascal)}: {pascal}MutationInput")

aggregate_json_doc = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"{BASE_URL}/mutations.json", "title": "PdfMutation", "description": "Discriminated union over this module's mutation leaves (internally tagged); every payload is the leaf's own schema, referenced, never restated.", "oneOf": aggregate_json}
write(os.path.join(MUTATIONS_DIR, "🔣️.json"), json.dumps(aggregate_json_doc, indent=2, ensure_ascii=False) + "\n")
write(os.path.join(MUTATIONS_DIR, "🟦️.ts"), "/** 🧬️ Transparent PDF mutation TypeScript union assembled from direct owners. */\n\n" + "\n".join(aggregate_ts_imports) + "\n\nexport type PdfMutation =\n" + "\n".join(aggregate_ts_union) + ";\n")
write(os.path.join(MUTATIONS_DIR, "🛰️.proto"), 'syntax = "proto3";\npackage semio.s_stdio_pdf_1_7.mutation;\n\n' + "\n".join(f'import "{leaf["directory"]}/🛰️.proto";' for leaf in LEAVES) + "\n\nmessage PdfMutation {\n  oneof mutation {\n" + "\n".join(aggregate_proto) + "\n  }\n}\n")
write(os.path.join(MUTATIONS_DIR, "🔗️.graphql"), "# 🧬️ PDF 1.7/base direct mutation aggregate.\ninput PdfMutationInput {\n" + "\n".join(aggregate_gql) + "\n}\n")
print(f"mutations: {len(LEAVES)} leaves")
