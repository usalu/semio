#!/usr/bin/env python3
"""🦀️ W2-S norm: a Rust type reader for the norm artifact crates and the value_derive wire-shape rules as JSON Schema.

Imported by `🧪️w2-s-norm-parity.py`; `python3 🧪️w2-s-norm-rust.py <artifact-dir> [Type…]` prints the parsed types.
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"


def clean_source(source):
    """✂️ Drops comments (keeping `///` doc lines as `@@doc …` markers) while keeping string, raw-string and char literals intact."""
    out, i, n = [], 0, len(source)
    while i < n:
        c = source[i]
        if source.startswith("///", i) and not source.startswith("////", i):
            end = source.find("\n", i)
            end = n if end < 0 else end
            out.append("@@doc " + json.dumps(source[i + 3 : end].strip()) + "\n")
            i = end + 1
        elif source.startswith("//", i):
            end = source.find("\n", i)
            i = n if end < 0 else end
        elif source.startswith("/*", i):
            depth = 0
            while i < n:
                if source.startswith("/*", i):
                    depth += 1
                    i += 2
                elif source.startswith("*/", i):
                    depth -= 1
                    i += 2
                    if depth == 0:
                        break
                else:
                    i += 1
        elif (c == "r" or (c == "b" and i + 1 < n and source[i + 1] == "r")) and re.match(r'b?r#*"', source[i : i + 12]) and (i == 0 or not (source[i - 1].isalnum() or source[i - 1] == "_")):
            m = re.match(r'(b?r)(#*)"', source[i:])
            close = '"' + m.group(2)
            end = source.find(close, i + len(m.group(0)))
            end = n if end < 0 else end + len(close)
            out.append(source[i:end])
            i = end
        elif c == '"':
            j = i + 1
            while j < n and source[j] != '"':
                j += 2 if source[j] == "\\" else 1
            out.append(source[i : j + 1])
            i = j + 1
        elif c == "'":
            m = re.match(r"'(\\.|\\u\{[0-9a-fA-F]+\}|[^\\'])'", source[i:])
            if m:
                out.append(m.group(0))
                i += len(m.group(0))
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return "".join(out)


def split_top(text, sep=","):
    parts, depth, current, i = [], 0, "", 0
    while i < len(text):
        char = text[i]
        if char == '"':
            j = i + 1
            while j < len(text) and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            current += text[i : j + 1]
            i = j + 1
            continue
        if char in "<([{":
            depth += 1
        elif char in ">)]}":
            if not (char == ">" and i > 0 and text[i - 1] == "-"):
                depth -= 1
        if char == sep and depth == 0:
            parts.append(current)
            current = ""
        else:
            current += char
        i += 1
    if current.strip():
        parts.append(current)
    return parts


def attr_pairs(attr):
    match = re.match(r"\s*(value|serde)\s*\((.*)\)\s*$", attr, flags=re.S)
    if not match:
        return None, {}
    pairs = {}
    for item in split_top(match.group(2)):
        kv = re.match(r'\s*([\w]+)\s*=\s*"((?:[^"\\]|\\.)*)"\s*$', item, flags=re.S)
        if kv:
            pairs[kv.group(1)] = kv.group(2)
        elif item.strip():
            pairs[item.strip()] = True
    return match.group(1), pairs


def take_attrs(text):
    """🏷️ Leading `#[…]` attributes and `@@doc` lines of an item or field chunk, and the rest."""
    attrs, docs = [], []
    rest = text.lstrip()
    while True:
        if rest.startswith("@@doc "):
            end = rest.find("\n")
            end = len(rest) if end < 0 else end
            docs.append(json.loads(rest[6:end].strip()))
            rest = rest[end + 1 :].lstrip()
            continue
        if rest.startswith("#["):
            depth, i = 0, 1
            while i < len(rest):
                if rest[i] == "[":
                    depth += 1
                elif rest[i] == "]":
                    depth -= 1
                    if depth == 0:
                        break
                elif rest[i] == '"':
                    j = i + 1
                    while j < len(rest) and rest[j] != '"':
                        j += 2 if rest[j] == "\\" else 1
                    i = j
                i += 1
            attrs.append(rest[2:i])
            rest = rest[i + 1 :].lstrip()
            continue
        return attrs, docs, rest


def value_attrs(attrs):
    value, serde, derives = {}, {}, set()
    for attr in attrs:
        attr = attr.strip()
        cfg = re.match(r"cfg_attr\s*\(\s*test\s*,\s*(.*)\)\s*$", attr, flags=re.S)
        if cfg:
            inner = cfg.group(1).strip()
            if re.match(r"derive\s*\(", inner):
                continue
            kind, pairs = attr_pairs(inner)
            if kind == "serde":
                serde.update(pairs)
            continue
        if re.match(r"derive\s*\(", attr):
            derives.update(name.split("::")[-1].strip() for name in split_top(attr[attr.index("(") + 1 : attr.rindex(")")]))
            continue
        kind, pairs = attr_pairs(attr)
        if kind == "value":
            value.update(pairs)
    return value, serde, derives


def match_close(text, open_index):
    pairs = {"{": "}", "(": ")", "[": "]", "<": ">"}
    opener, closer, depth, i = text[open_index], pairs[text[open_index]], 0, open_index
    while i < len(text):
        c = text[i]
        if c == '"':
            j = i + 1
            while j < len(text) and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            i = j + 1
            continue
        if c == opener:
            depth += 1
        elif c == closer and not (c == ">" and text[i - 1] == "-"):
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return len(text) - 1


class RType:
    def __init__(self, name, kind, path, value, serde, derives, docs):
        self.name, self.kind, self.path, self.value, self.serde, self.derives, self.docs = name, kind, path, value, serde, derives, docs
        self.fields, self.variants, self.target, self.generics = [], [], None, []

    def __repr__(self):
        return f"<{self.kind} {self.name} @{os.path.basename(os.path.dirname(self.path))}>"


def parse_fields(body):
    fields = []
    for part in split_top(body):
        attrs, docs, decl = take_attrs(part)
        if not decl.strip():
            continue
        m = re.match(r"(?:pub(?:\s*\([^)]*\))?\s+)?(\w+)\s*:\s*(.+)$", decl.strip(), flags=re.S)
        if not m:
            continue
        value, serde, _ = value_attrs(attrs)
        unit = next((u.group(1) for u in (re.search(r'unit\s*=\s*"([^"]*)"', attr) for attr in attrs if attr.strip().startswith("dsl")) if u), None)
        fields.append({"ident": m.group(1), "type": " ".join(m.group(2).split()), "value": value, "serde": serde, "doc": " ".join(docs), "unit": unit})
    return fields


def parse_items(path):
    source = clean_source(open(path, encoding="utf-8").read())
    items = []
    for m in re.finditer(r"(?:^|\n)([ \t]*(?:(?:@@doc[^\n]*\n|#\[[^\n]*\]\s*\n)[ \t]*)*)(?:pub(?:\s*\([^)]*\))?\s+)?(struct|enum|type)\s+(\w+)\s*(<[^>{(=;]*>)?", source):
        head, kind, name = m.group(1), m.group(2), m.group(3)
        attrs, docs, _ = take_attrs(head + "X")
        value, serde, derives = value_attrs(attrs)
        item = RType(name, kind, path, value, serde, derives, docs)
        item.generics = [g.strip() for g in (m.group(4) or "<>")[1:-1].split(",") if g.strip()]
        rest_index = m.end()
        tail = source[rest_index:]
        where = re.match(r"\s*(?:where[^{;(]*)?", tail)
        pos = rest_index + (where.end() if where else 0)
        if kind == "type":
            eq = source.find("=", rest_index)
            semi = source.find(";", eq)
            item.kind, item.target = "alias", " ".join(source[eq + 1 : semi].split())
            items.append(item)
            continue
        if pos >= len(source):
            continue
        opener = source[pos]
        if opener == ";":
            item.kind = "unit"
            items.append(item)
            continue
        if opener not in "{(":
            continue
        close = match_close(source, pos)
        body = source[pos + 1 : close]
        if kind == "struct" and opener == "(":
            item.kind = "tuple"
            item.fields = [{"ident": str(i), "type": " ".join(re.sub(r"^pub(\s*\([^)]*\))?\s+", "", take_attrs(p)[2].strip()).split()), "value": {}, "serde": {}, "doc": ""} for i, p in enumerate(split_top(body)) if take_attrs(p)[2].strip()]
        elif kind == "struct":
            item.kind = "struct"
            item.fields = parse_fields(body)
        else:
            item.kind = "enum"
            for part in split_top(body):
                attrs, vdocs, decl = take_attrs(part)
                decl = decl.strip()
                if not decl:
                    continue
                vvalue, vserde, _ = value_attrs(attrs)
                vm = re.match(r"(\w+)\s*(.*)$", decl, flags=re.S)
                vname, vrest = vm.group(1), vm.group(2).strip()
                vrest = re.sub(r"=\s*[-\w]+\s*$", "", vrest).strip()
                variant = {"ident": vname, "value": vvalue, "serde": vserde, "doc": " ".join(vdocs), "kind": "unit", "fields": []}
                if vrest.startswith("("):
                    inner = vrest[1 : match_close(vrest, 0)]
                    variant["kind"], variant["fields"] = "tuple", [" ".join(p.split()) for p in split_top(inner) if p.strip()]
                elif vrest.startswith("{"):
                    inner = vrest[1 : match_close(vrest, 0)]
                    variant["kind"], variant["fields"] = "named", parse_fields(inner)
                item.variants.append(variant)
        items.append(item)
    return items


class Registry:
    """📚️ Every struct/enum/alias of one crate (all `.rs` outside test trees), looked up by bare name."""

    def __init__(self, roots):
        self.by_name = {}
        for root in roots:
            for dirpath, dirnames, filenames in os.walk(root):
                dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated") and "🧪️tests" not in d and "🔬️" not in d]
                for filename in filenames:
                    if filename.endswith(".rs"):
                        for item in parse_items(os.path.join(dirpath, filename)):
                            self.by_name.setdefault(item.name, []).append(item)

    def lookup(self, name, hint=None):
        found = [item for item in self.by_name.get(name, []) if item.kind != "alias" or item.target != name]
        wired = [item for item in found if "ToValue" in item.derives or item.kind == "alias"]
        candidates = wired or found
        if len(candidates) > 1 and hint:
            narrowed = [item for item in candidates if hint in item.path]
            candidates = narrowed or candidates
        if len(candidates) > 1:
            shapes = {repr(sorted((f["ident"], f["type"]) for f in item.fields)) + repr([(v["ident"], v["kind"]) for v in item.variants]) for item in candidates}
            if len(shapes) > 1:
                raise KeyError(f"ambiguous type {name}: {[item.path for item in candidates]}")
        return candidates[0] if candidates else None


def split_words_snake(ident):
    return [word.lower() for word in ident.split("_") if word]


def split_words_pascal(ident):
    words, current = [], ""
    for ch in ident:
        if ch.isupper() and current:
            words.append(current.lower())
            current = ""
        current += ch
    if current:
        words.append(current.lower())
    return words


def apply_case(words, case):
    if case == "camelCase":
        return words[0] + "".join(w[:1].upper() + w[1:] for w in words[1:]) if words else ""
    if case == "kebab-case":
        return "-".join(words)
    if case == "lowercase":
        return "".join(words)
    if case == "snake_case":
        return "_".join(words)
    if case == "PascalCase":
        return "".join(w[:1].upper() + w[1:] for w in words)
    if case == "SCREAMING_SNAKE_CASE":
        return "_".join(words).upper()
    return None


def field_wire(ident, rename, rename_all):
    if rename:
        return rename
    if rename_all:
        cased = apply_case(split_words_snake(ident), rename_all)
        if cased is not None:
            return cased
    return ident


def variant_wire(ident, rename, rename_all):
    if rename:
        return rename
    if rename_all:
        cased = apply_case(split_words_pascal(ident), rename_all)
        if cased is not None:
            return cased
    return ident


if __name__ == "__main__":
    registry = Registry([sys.argv[1]])
    for name in sys.argv[2:] or sorted(registry.by_name):
        for item in registry.by_name.get(name, []):
            print(item, item.value, sorted(item.derives))
            for f in item.fields:
                print("   ", f["ident"], ":", f["type"], f["value"] or "")
            for v in item.variants:
                print("   |", v["ident"], v["kind"], v["fields"], v["value"] or "")
