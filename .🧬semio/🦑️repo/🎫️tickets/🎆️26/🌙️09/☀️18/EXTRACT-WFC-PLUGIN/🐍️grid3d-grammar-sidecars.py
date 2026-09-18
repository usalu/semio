#!/usr/bin/env python3
"""🔤 Derives the text sidecars (`🔤️.ebnf`, `🅰️.g4`) from each facet's normative `📖️.grammar.semio`
and the binary sidecars (`🥋️.ksy`, `🌶️.spicy`, `🔠️.abnf`) from its `📡️.protocol.semio`, so all of them
share one rule/field vocabulary. Re-running produces byte-identical output when already in sync."""

import io
import os
import re

ROOT = "/Users/ueli/Documents/semio"
X = os.path.join(ROOT, "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema")

FACETS = [("📸️snapshot", "snapshot"), ("🧬️mutations", "mutations")]


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    io.open(path, "w", encoding="utf-8").write(text)


def grammar_rules(text):
    """📖 Every `name = body` production of a `dialect grammar`, in source order."""
    return [(match.group(1), match.group(2).strip()) for match in re.finditer(r"^([a-z][a-z0-9-]*) = (.+)$", text, re.MULTILINE)]


def protocol_fields(text):
    """📡 Every `field <name> <type>` and `segment <name> …` of a `dialect protocol`, in source order."""
    fields = []
    section = "header"
    for line in text.splitlines():
        parts = line.split()
        if not parts:
            continue
        if parts[0] in {"header", "footer"}:
            section = parts[0]
        elif parts[0] == "segment":
            fields.append(("body", parts[1], "bytes"))
        elif parts[0] == "field":
            fields.append((section, parts[1], parts[2]))
    return fields


def split_strings(body):
    """📖 Splits a production body into `(is_literal, chunk)` runs, so a rename never reaches inside a
    quoted terminal — the keywords ARE the wire tokens and must survive verbatim."""
    runs = []
    literal = False
    for chunk in body.split('"'):
        runs.append((literal, chunk))
        literal = not literal
    return runs


def spaced_outside_strings(body):
    return '"'.join(chunk if literal else chunk.replace("-", " ") for literal, chunk in split_strings(body))


def camelled_outside_strings(body, camel):
    return "'".join(
        f"'{chunk}'" if literal else re.sub(r"[a-z][a-z0-9-]*", lambda match: camel(match.group(0)), chunk) for literal, chunk in split_strings(body)
    ).replace("''", "'")


KSY_TYPES = {"u16": "u2", "u32": "u4", "utf8": "str", "bytes": "bytes"}
SPICY_TYPES = {"u16": "uint16", "u32": "uint32", "utf8": "bytes", "bytes": "bytes"}


for directory, facet in FACETS:
    grammar_path = os.path.join(X, directory, "📝️text", "📖️.grammar.semio")
    protocol_path = os.path.join(X, directory, "💾️binary", "📡️.protocol.semio")
    grammar = io.open(grammar_path, encoding="utf-8").read()
    protocol = io.open(protocol_path, encoding="utf-8").read()
    rules = grammar_rules(grammar)
    fields = protocol_fields(protocol)
    name = f"wfc.grid3d.{facet}"

    # 🔤 EBNF: the same rule names, kebab spelled as "space case", terminals quoted as authored.
    ebnf = [f"(* 🔤 {name} — derived from 📖️.grammar.semio; rule names are the grammar's own, kebab folded to spaces. *)", ""]
    for rule, body in rules:
        ebnf.append(f"{rule.replace('-', ' ')} = {spaced_outside_strings(body)} ;")
    write(os.path.join(X, directory, "📝️text", "🔤️.ebnf"), "\n".join(ebnf) + "\n")

    # 🅰️ ANTLR: the same rule names in camelCase, one parser rule each.
    def camel(rule):
        head, *rest = rule.split("-")
        return head + "".join(part.capitalize() for part in rest)

    g4 = [f"// 🅰️ {name} — derived from 📖️.grammar.semio; rule names are the grammar's own, kebab folded to camelCase.", f"grammar {facet.capitalize()}Grid3d;", ""]
    for rule, body in rules:
        g4.append(f"{camel(rule)} : {camelled_outside_strings(body, camel)} ;")
    g4 += ["", "SP : ' ' ;", "NL : '\\r'? '\\n' ;", "OCTET : . ;"]
    write(os.path.join(X, directory, "📝️text", "🅰️.g4"), "\n".join(g4) + "\n")

    # 🥋 Kaitai: the framed record, field for field.
    ksy = [f"# 🥋 {name} — derived from 📡️.protocol.semio; one Kaitai attribute per declared field.", "meta:", f"  id: {facet}_grid3d", "  endian: le", "seq:"]
    for section, field, kind in fields:
        ksy.append(f"  - id: {section}_{field}")
        if kind == "bytes":
            ksy.append("    size-eos: false")
            ksy.append("    type: str" if kind == "utf8" else "    size: 0")
        else:
            ksy.append(f"    type: {KSY_TYPES.get(kind, 'u4')}")
    write(os.path.join(X, directory, "💾️binary", "🥋️.ksy"), "\n".join(ksy) + "\n")

    # 🌶️ Spicy: the same record, same order.
    spicy = [f"# 🌶️ {name} — derived from 📡️.protocol.semio; one Spicy unit field per declared field.", f"module {facet.capitalize()}Grid3d;", "", "public type Record = unit {"]
    for section, field, kind in fields:
        spicy.append(f"    {section}_{field}: {SPICY_TYPES.get(kind, 'uint32')};")
    spicy += ["};"]
    write(os.path.join(X, directory, "💾️binary", "🌶️.spicy"), "\n".join(spicy) + "\n")

    # 🔠 ABNF: the byte layout as a concatenation, in declaration order.
    abnf = [f"; 🔠 {name} — derived from 📡️.protocol.semio; the record is the concatenation of its declared fields.", ""]
    abnf.append("record = " + " ".join(f"{section}-{field}" for section, field, _ in fields))
    for section, field, kind in fields:
        width = {"u16": "2OCTET", "u32": "4OCTET", "utf8": "*OCTET", "bytes": "*OCTET"}.get(kind, "4OCTET")
        abnf.append(f"{section}-{field} = {width}")
    write(os.path.join(X, directory, "💾️binary", "🔠️.abnf"), "\n".join(abnf) + "\n")

print("derived the text and binary sidecars for both facets")
