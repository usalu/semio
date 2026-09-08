#!/usr/bin/env python3
"""🧬 Authors, relocates and validates the `🗄️stdio` mutation-leaf payload JSON Schemas.

The projection is a *reading* of `value_derive` (`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs`),
not of `serde`: `#[value(...)]` container/field attributes decide the wire shape, a container with
no `rename_all` keeps the Rust identifier verbatim, `Option<T>` encodes as `null`, a one-field
tuple struct is transparent, and a `tag`-less enum with data-carrying variants is externally
tagged. Every rule below cites that module's own documented behaviour.

Subcommands
  census      — descriptor inventory (present / missing payload schema)
  author      — write `<leaf>/🧬️schema/🔣️.json` for every leaf missing its declared schema
  relocate    — move an existing leaf schema onto the canonical `🧬️schema/🔣️.json` path
  aggregates  — rewrite `🧬️mutations/🔣️.json` as a pure `$ref` union over its leaves
  ids         — rewrite every leaf/aggregate `$id` to the contract §A grammar, proving id uniqueness
  casing      — plan the camelCase conformance change: containers to annotate + fixtures it re-cases
  facets      — dialect / `$id` / `$defs` / export-id repair of the `📝️text`+`💾️binary` codec facets
  roots       — author the missing `🪆️subsets/<s>/🧬️schema/🔣️.json` subset-root artifact documents
  verify      — descriptor-path / dialect / `$id` / `title` structural check
"""

from __future__ import annotations

import json
import os
import re
import sys
import unicodedata
from collections import Counter, OrderedDict, defaultdict

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
STDIO = os.path.join(REPO, "✏️s", "🔌️plugins", "🗄️stdio")
ARTIFACTS = os.path.join(STDIO, "🗿️artifacts")
MUTATIONS_DIR = "🧬️mutations"
SCHEMA_DIR = "🧬️schema"
CANONICAL_SCHEMA_REL = "🧬️schema/🔣️.json"
DIALECT = "http://json-schema.org/draft-07/schema#"
ID_ROOT = "https://semio.tech/schema/s/stdio"

# 🔢 `value_derive`'s scalar codecs (`🌱️value/🔁️codec/🦀️.rs` §Scalars) plus the width bound the
# repository's own hand-authored stdio schemas already carry (`maximum: 255 | 65535 | 4294967295`).
SCALARS = {
    "String": {"type": "string"},
    "str": {"type": "string"},
    "char": {"type": "string", "minLength": 1, "maxLength": 1},
    "bool": {"type": "boolean"},
    "u8": {"type": "integer", "minimum": 0, "maximum": 255},
    "u16": {"type": "integer", "minimum": 0, "maximum": 65535},
    "u32": {"type": "integer", "minimum": 0, "maximum": 4294967295},
    "u64": {"type": "integer", "minimum": 0},
    "u128": {"type": "integer", "minimum": 0},
    "usize": {"type": "integer", "minimum": 0},
    "i8": {"type": "integer", "minimum": -128, "maximum": 127},
    "i16": {"type": "integer", "minimum": -32768, "maximum": 32767},
    "i32": {"type": "integer", "minimum": -2147483648, "maximum": 2147483647},
    "i64": {"type": "integer"},
    "i128": {"type": "integer"},
    "isize": {"type": "integer"},
    "f32": {"type": "number"},
    "f64": {"type": "number"},
}

ANY_VALUE_NAMES = {"DslValue", "JsonValue", "Value", "GltfJson", "JsonNode"}

# 🧩 The two `#[value(with = …)]` codecs stdio payloads reach, each read from its own function body:
# `ordered_attr_map_from_value` (📸️snapshot/🦀️.rs:228) maps `Vec<(String, usize)>` onto a plain object,
# and `deserialize_page` (📥️insert-page/🦀️.rs:28) decodes `PageDoc` through a `PagePayload` that names
# exactly `PageDoc`'s own fields, so the payload type itself is the honest shape.
CUSTOM_CODECS = {
    "ordered_attr_map_from_value": {"type": "object", "additionalProperties": {"type": "integer", "minimum": 0}},
    "ordered_attr_map_to_value": {"type": "object", "additionalProperties": {"type": "integer", "minimum": 0}},
    "deserialize_page": None,
}

# ✍️ Types whose `ToValue`/`FromValue` are hand-written, so no attribute on the declaration describes
# their wire shape — each entry is read from that impl. `GltfMorphTarget` (📸️snapshot/🦀️.rs:413-421)
# routes through `ordered_attr_map_*`, so it is an object of accessor indices, not an array of pairs.
HANDWRITTEN_TYPES = {
    "GltfMorphTarget": {"type": "object", "additionalProperties": {"type": "integer", "minimum": 0}},
}


# ───────────────────────────── naming ─────────────────────────────


def strip_leading_emoji(name: str) -> str:
    """🏷️ Drops one directory name's leading emoji/keycap marker, keeping the semantic remainder."""
    markers = ("️", "︎", "⃣", "‍")
    index = 0
    while index < len(name):
        character = name[index]
        if character in markers:
            index += 1
            continue
        if index + 1 < len(name) and name[index + 1] in ("️", "⃣"):
            index += 1
            continue
        if ord(character) >= 0x1F000:
            index += 1
            continue
        if ord(character) > 0x2000 and unicodedata.category(character) in ("So", "Sk", "Cf", "Cn", "Sm"):
            index += 1
            continue
        break
    return name[index:]


def snake_words(ident: str) -> list[str]:
    return [part for part in ident.split("_") if part != ""]


def pascal_words(ident: str) -> list[str]:
    return [part.lower() for part in re.findall(r"[A-Z]+(?![a-z])|[A-Z][a-z0-9]*|[a-z0-9]+", ident)]


def apply_case(words: list[str], case: str | None) -> str | None:
    if case is None:
        return None
    if case == "camelCase":
        return words[0] + "".join(word.capitalize() for word in words[1:]) if words else ""
    if case == "kebab-case":
        return "-".join(words)
    if case == "lowercase":
        return "".join(words)
    if case == "snake_case":
        return "_".join(words)
    return None


# 🧭 `casing` (cross-partition request 46) answers "what would the wire look like if every container
# that declares no `rename_all` declared `camelCase`?". Setting this makes the projection model the
# POST-change crate; it is never set while authoring or auditing the tree.
ASSUME_CAMEL_CASE = False


def field_wire_name(ident: str, rename: str | None, rename_all: str | None) -> str:
    if rename is not None:
        return rename
    if rename_all is None and ASSUME_CAMEL_CASE:
        rename_all = "camelCase"
    cased = apply_case(snake_words(ident), rename_all)
    return cased if cased is not None else ident


def variant_wire_name(ident: str, rename: str | None, rename_all: str | None) -> str:
    if rename is not None:
        return rename
    cased = apply_case(pascal_words(ident), rename_all)
    return cased if cased is not None else ident


# ───────────────────────────── rust parsing ─────────────────────────────


# 🔍 The four lexical openers a Rust file can present at any point. A regex `sub` over the whole file
# cannot do this job: leftmost-match semantics let a quote inside one construct swallow the opener of
# the next, which silently deleted a documented field from a struct body.
LEXICAL_OPENER = re.compile(r"r#*\"|\"|'(?:\\.|[^\\'\n])'|//|/\*")


def strip_comments(text: str) -> str:
    """🧹 Blanks line and block comments, scanning string, raw-string and char literals rather than
    matching them, so offsets, line breaks and every declaration survive intact."""
    blanks: list[tuple[int, int]] = []
    index = 0
    length = len(text)
    while index < length:
        match = LEXICAL_OPENER.search(text, index)
        if match is None:
            break
        start = match.start()
        token = match.group(0)
        if token.startswith("r"):
            hashes = token.count("#")
            close = text.find('"' + "#" * hashes, start + len(token))
            index = length if close == -1 else close + 1 + hashes
        elif token == '"':
            cursor = start + 1
            while cursor < length:
                if text[cursor] == "\\":
                    cursor += 2
                    continue
                if text[cursor] == '"':
                    cursor += 1
                    break
                cursor += 1
            index = cursor
        elif token.startswith("'"):
            index = match.end()
        elif token == "//":
            close = text.find("\n", start)
            close = length if close == -1 else close
            blanks.append((start, close))
            index = close
        else:
            depth = 1
            cursor = start + 2
            while cursor < length and depth > 0:
                if text.startswith("/*", cursor):
                    depth += 1
                    cursor += 2
                elif text.startswith("*/", cursor):
                    depth -= 1
                    cursor += 2
                else:
                    cursor += 1
            blanks.append((start, cursor))
            index = cursor
    if not blanks:
        return text
    pieces: list[str] = []
    previous = 0
    for start, close in blanks:
        pieces.append(text[previous:start])
        pieces.append("".join(character if character == "\n" else " " for character in text[start:close]))
        previous = close
    pieces.append(text[previous:])
    return "".join(pieces)


def match_block(text: str, open_index: int) -> int:
    """🧱 Index of the brace closing the one opened at `open_index`, or -1."""
    depth = 0
    index = open_index
    length = len(text)
    while index < length:
        character = text[index]
        if character == '"':
            index += 1
            while index < length:
                if text[index] == "\\":
                    index += 2
                    continue
                if text[index] == '"':
                    break
                index += 1
        elif character == "{":
            depth += 1
        elif character == "}":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    return -1


def parse_meta(attribute_body: str) -> list[tuple[str, str | None]]:
    """🧾 Splits one `#[value(a = "b", c)]` body into `(key, value)` pairs."""
    pairs: list[tuple[str, str | None]] = []
    for item in re.finditer(r'([A-Za-z_][A-Za-z0-9_]*)\s*(?:=\s*"((?:[^"\\]|\\.)*)")?', attribute_body):
        pairs.append((item.group(1), item.group(2)))
    return pairs


def attribute_runs(text: str, start: int) -> tuple[str, int]:
    """🔎 Reads the attribute block immediately preceding `start`, returning it and its own start."""
    index = start
    collected: list[str] = []
    while True:
        probe = index
        while probe > 0 and text[probe - 1] in " \t\n\r":
            probe -= 1
        if probe == 0 or text[probe - 1] != "]":
            break
        depth = 0
        cursor = probe - 1
        while cursor >= 0:
            if text[cursor] == "]":
                depth += 1
            elif text[cursor] == "[":
                depth -= 1
                if depth == 0:
                    break
            cursor -= 1
        if cursor <= 0 or not (text[cursor - 1] == "#" or (cursor >= 2 and text[cursor - 2 : cursor] == "#!")):
            break
        begin = cursor - 1 if text[cursor - 1] == "#" else cursor - 2
        collected.insert(0, text[begin:probe])
        index = begin
    return "\n".join(collected), index


class Container:
    def __init__(self, attributes: str) -> None:
        self.raw = attributes
        self.rename_all: str | None = None
        self.rename_all_fields: str | None = None
        self.tag: str | None = None
        self.content: str | None = None
        self.deny_unknown = False
        self.transparent = False
        self.default = False
        for body in re.findall(r"#\[value\(([\s\S]*?)\)\]", attributes):
            for key, value in parse_meta(body):
                if key == "rename_all":
                    self.rename_all = value
                elif key == "rename_all_fields":
                    self.rename_all_fields = value
                elif key == "tag":
                    self.tag = value
                elif key == "content":
                    self.content = value
                elif key == "deny_unknown_fields":
                    self.deny_unknown = True
                elif key == "transparent":
                    self.transparent = True
                elif key == "default":
                    self.default = True

    def field_rename_all(self) -> str | None:
        return self.rename_all_fields if self.rename_all_fields is not None else self.rename_all


class FieldAttrs:
    def __init__(self, attributes: str) -> None:
        self.raw = attributes
        self.rename: str | None = None
        self.default = False
        self.skip = False
        self.flatten = False
        self.custom: str | None = None
        for body in re.findall(r"#\[value\(([\s\S]*?)\)\]", attributes):
            for key, value in parse_meta(body):
                if key == "rename":
                    self.rename = value
                elif key == "default":
                    self.default = True
                elif key == "skip":
                    self.skip = True
                elif key == "flatten":
                    self.flatten = True
                elif key in ("with", "serialize_with", "deserialize_with"):
                    self.custom = value
                elif key == "skip_serializing_if":
                    self.default = True


def split_top_level(text: str, separator: str = ",") -> list[str]:
    parts: list[str] = []
    depth = 0
    current: list[str] = []
    for character in text:
        if character in "<([{":
            depth += 1
        elif character in ">)]}":
            depth -= 1
        if character == separator and depth == 0:
            parts.append("".join(current))
            current = []
            continue
        current.append(character)
    parts.append("".join(current))
    return [part.strip() for part in parts if part.strip() != ""]


def skip_attributes(body: str, cursor: int) -> tuple[int, str]:
    """🏷️ Consumes whitespace and any `#[…]` run at `cursor`, returning the new cursor and the run."""
    collected: list[str] = []
    while cursor < len(body):
        while cursor < len(body) and body[cursor] in " \t\r\n":
            cursor += 1
        if not body.startswith("#[", cursor):
            break
        depth = 0
        end = cursor + 1
        while end < len(body):
            if body[end] == "[":
                depth += 1
            elif body[end] == "]":
                depth -= 1
                if depth == 0:
                    end += 1
                    break
            end += 1
        collected.append(body[cursor:end])
        cursor = end
    return cursor, "\n".join(collected)


def read_type(body: str, cursor: int) -> tuple[str, int]:
    """🧮 Reads one type expression up to the top-level `,` or the end of the field list."""
    depth = 0
    end = cursor
    while end < len(body):
        character = body[end]
        if character in "<([{":
            depth += 1
        elif character in ">)]}":
            if depth == 0:
                break
            depth -= 1
        elif character == "," and depth == 0:
            break
        elif character == "-" and end + 1 < len(body) and body[end + 1] == ">" and depth == 0:
            depth += 0
        end += 1
    return body[cursor:end].strip(), end


class Fields(list):
    """📋 A parsed field list that remembers anything in the body it could not account for."""

    gaps: list[str] = []


def parse_named_fields(body: str) -> Fields:
    """📋 `(ident, rust type, attributes)` for one named-field body, attribute runs kept out of the scan.

    Text the scan cannot read as an attribute or a field is recorded as a GAP rather than skipped: a
    silently dropped field produces a schema that is wrong in the one way nothing downstream can see."""
    fields = Fields()
    fields.gaps = []
    cursor = 0
    while cursor < len(body):
        cursor, attributes = skip_attributes(body, cursor)
        if cursor >= len(body):
            break
        match = re.compile(r"(?:pub(?:\s*\([^)]*\))?\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:").match(body, cursor)
        if match is None:
            comma = body.find(",", cursor)
            fields.gaps.append(body[cursor : comma if comma != -1 else len(body)].strip()[:80])
            if comma == -1:
                break
            cursor = comma + 1
            continue
        rust_type, end = read_type(body, match.end())
        if rust_type != "":
            fields.append((match.group(1), rust_type, FieldAttrs(attributes)))
        cursor = end + 1
    return fields


def parse_variants(body: str) -> list[tuple[str, str, object, FieldAttrs]]:
    """🌿 `(ident, shape, payload, attributes)` for one enum body — `unit` | `tuple` | `named`."""
    variants: list[tuple[str, str, object, FieldAttrs]] = []
    cursor = 0
    while cursor < len(body):
        cursor, attributes = skip_attributes(body, cursor)
        match = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)\s*").match(body, cursor)
        if match is None:
            comma = body.find(",", cursor)
            if comma == -1:
                break
            cursor = comma + 1
            continue
        ident = match.group(1)
        cursor = match.end()
        if cursor < len(body) and body[cursor] == "{":
            close_index = match_block(body, cursor)
            if close_index == -1:
                break
            variants.append((ident, "named", parse_named_fields(body[cursor + 1 : close_index]), FieldAttrs(attributes)))
            cursor = close_index + 1
        elif cursor < len(body) and body[cursor] == "(":
            depth = 0
            end = cursor
            while end < len(body):
                if body[end] == "(":
                    depth += 1
                elif body[end] == ")":
                    depth -= 1
                    if depth == 0:
                        break
                end += 1
            variants.append((ident, "tuple", split_top_level(body[cursor + 1 : end]), FieldAttrs(attributes)))
            cursor = end + 1
        elif cursor < len(body) and body[cursor] == "=":
            end = body.find(",", cursor)
            variants.append((ident, "unit", [], FieldAttrs(attributes)))
            cursor = len(body) if end == -1 else end + 1
            continue
        else:
            variants.append((ident, "unit", [], FieldAttrs(attributes)))
        comma = body.find(",", cursor)
        cursor = len(body) if comma == -1 else comma + 1
    return variants


class TypeDef:
    def __init__(self, kind: str, name: str, container: Container, payload, path: str) -> None:
        self.kind = kind
        self.name = name
        self.container = container
        self.payload = payload
        self.path = path
        self.generics: list[str] = []


def index_rust(*roots: str) -> dict[str, list[TypeDef]]:
    """🗂️ Indexes every `pub struct`/`pub enum`/`pub type` under the given roots, by bare name."""
    index: dict[str, list[TypeDef]] = defaultdict(list)
    for root in roots:
        index_rust_into(index, root)
    return index


def index_rust_into(index: dict[str, list[TypeDef]], root: str) -> None:
    for directory, subdirectories, files in os.walk(root):
        subdirectories[:] = [name for name in subdirectories if name not in ("target", "node_modules") and "🧪️tests" not in name and "🧫️" not in name]
        for name in files:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(directory, name)
            try:
                text = strip_comments(open(path, encoding="utf8").read())
            except OSError:
                continue
            relative = os.path.relpath(path, REPO)
            for match in re.finditer(r"\b(?:pub(?:\s*\([^)]*\))?\s+)?(struct|enum)\s+([A-Za-z_][A-Za-z0-9_]*)\s*(<[^{;(]*>)?\s*([{(;])", text):
                kind, type_name, generics, opener = match.group(1), match.group(2), match.group(3), match.group(4)
                attributes, attribute_start = attribute_runs(text, match.start())
                if "#[cfg(test)]" in attributes:
                    continue
                container = Container(attributes)
                if opener == "{":
                    open_index = text.index("{", match.end() - 1)
                    close_index = match_block(text, open_index)
                    if close_index == -1:
                        continue
                    body = text[open_index + 1 : close_index]
                    payload = parse_named_fields(body) if kind == "struct" else body
                elif opener == "(":
                    depth = 0
                    cursor = match.end() - 1
                    end = cursor
                    while end < len(text):
                        if text[end] == "(":
                            depth += 1
                        elif text[end] == ")":
                            depth -= 1
                            if depth == 0:
                                break
                        end += 1
                    payload = split_top_level(re.sub(r"#\[[^\]]*\]", "", re.sub(r"\bpub\b", "", text[cursor + 1 : end])))
                    kind = "tuple-struct" if kind == "struct" else kind
                else:
                    payload = []
                    kind = "unit-struct" if kind == "struct" else kind
                definition = TypeDef(kind, type_name, container, payload, relative)
                definition.generics = [parameter.split(":")[0].strip() for parameter in split_top_level((generics or "<>")[1:-1]) if parameter.strip() != "" and not parameter.strip().startswith("'") and not parameter.strip().startswith("const ")]
                index[type_name].append(definition)
            for match in re.finditer(r"\bpub(?:\s*\([^)]*\))?\s+type\s+([A-Za-z_][A-Za-z0-9_]*)\s*(<[^=]*>)?\s*=\s*([^;]+);", text):
                alias = TypeDef("alias", match.group(1), Container(""), match.group(3).strip(), relative)
                alias.generics = [parameter.split(":")[0].strip() for parameter in split_top_level((match.group(2) or "<>")[1:-1]) if parameter.strip() != "" and not parameter.strip().startswith("'")]
                index[match.group(1)].append(alias)


def nearest(candidates: list[TypeDef], owner: str) -> TypeDef:
    def score(definition: TypeDef) -> int:
        own = owner.split("/")
        theirs = definition.path.split("/")
        shared = 0
        for left, right in zip(own, theirs):
            if left != right:
                break
            shared += 1
        return shared

    return max(candidates, key=score)


# ───────────────────────────── projection ─────────────────────────────


class Unmapped(Exception):
    pass


class Projector:
    def __init__(self, index: dict[str, list[TypeDef]], owner: str) -> None:
        self.index = index
        self.owner = owner
        self.defs: "OrderedDict[str, dict]" = OrderedDict()
        self.stack: list[str] = []
        self.bindings: dict[str, str] = {}
        self.inlining: set[str] = set()

    # 🔗️ `ArtifactChild<S>` hand-writes `ToValue`/`FromValue` (🏪️store/🦀️.rs:2817-2837): the phantom
    # marker and the local owner never reach the wire, so its shape is fixed and generic-free.
    ARTIFACT_CHILD = {
        "type": "object",
        "additionalProperties": False,
        "required": ["childId", "target"],
        "properties": {"childId": {"type": "string"}, "target": {"$ref": "#/$defs/ArtifactRef"}},
    }

    def lookup(self, name: str) -> TypeDef | None:
        bare = name.split("::")[-1]
        candidates = self.index.get(bare)
        return None if not candidates else nearest(candidates, self.owner)

    def project(self, rust_type: str) -> dict:
        rust_type = rust_type.strip()
        if rust_type in self.bindings:
            # 🔁️ A generic parameter can be bound to its own name (`Sampler<T>` reached as `Sampler<T>`);
            # the binding is dropped while resolving it so the substitution cannot chase its own tail.
            bound = self.bindings[rust_type]
            saved = self.bindings
            self.bindings = {name: value for name, value in saved.items() if name != rust_type}
            try:
                return self.project(bound)
            finally:
                self.bindings = saved
        if re.match(r"^(?:[a-z_]+::)*ArtifactChild\s*<", rust_type):
            reference = self.project("ArtifactRef")
            if reference != {"$ref": "#/$defs/ArtifactRef"}:
                raise Unmapped("ArtifactRef did not project to its own $def")
            return dict(self.ARTIFACT_CHILD)
        if rust_type in SCALARS:
            return dict(SCALARS[rust_type])
        if rust_type in ("()",):
            return {"type": "null"}
        bare = rust_type.split("::")[-1]
        if bare in ANY_VALUE_NAMES and self.lookup(rust_type) is None or bare in ("DslValue", "JsonValue"):
            return {"description": "any JSON value"}
        option = re.match(r"^Option\s*<(.+)>$", rust_type, re.S)
        if option:
            inner = self.project(option.group(1))
            return {"anyOf": [inner, {"type": "null"}]}
        for wrapper in ("Box", "Arc", "Rc", "Cow"):
            match = re.match(rf"^{wrapper}\s*<(.+)>$", rust_type, re.S)
            if match:
                parts = split_top_level(match.group(1))
                return self.project(parts[-1])
        for wrapper in ("Vec", "VecDeque", "HashSet", "BTreeSet"):
            match = re.match(rf"^{wrapper}\s*<(.+)>$", rust_type, re.S)
            if match:
                return {"type": "array", "items": self.project(match.group(1))}
        match = re.match(r"^(?:HashMap|BTreeMap|IndexMap)\s*<(.+)>$", rust_type, re.S)
        if match:
            parts = split_top_level(match.group(1))
            if len(parts) != 2:
                raise Unmapped(f"map with {len(parts)} parameters: {rust_type}")
            if parts[0].split("::")[-1] not in ("String", "str"):
                raise Unmapped(f"map keyed by {parts[0]}: {rust_type}")
            return {"type": "object", "additionalProperties": self.project(parts[1])}
        match = re.match(r"^\[\s*(.+)\s*;\s*([0-9A-Za-z_:]+)\s*\]$", rust_type, re.S)
        if match:
            items = self.project(match.group(1))
            if match.group(2).isdigit():
                count = int(match.group(2))
                return {"type": "array", "items": items, "minItems": count, "maxItems": count}
            return {"type": "array", "items": items}
        match = re.match(r"^Vec\s*<\s*u8\s*>$", rust_type)
        if match:
            return {"type": "array", "items": dict(SCALARS["u8"])}
        if rust_type.startswith("(") and rust_type.endswith(")"):
            parts = split_top_level(rust_type[1:-1])
            if len(parts) == 0:
                return {"type": "null"}
            if len(parts) == 1:
                return self.project(parts[0])
            return {"type": "array", "items": [self.project(part) for part in parts], "additionalItems": False, "minItems": len(parts), "maxItems": len(parts)}
        match = re.match(r"^([A-Za-z_][A-Za-z0-9_:]*)\s*<(.+)>$", rust_type, re.S)
        if match:
            definition = self.lookup(match.group(1))
            # 🧬️ Substitute the CURRENT bindings into the arguments first: inside `GltfCollectionDiff<T, D>`
            # the field `Vec<GltfModified<D>>` names `D`, which only means something in this instantiation.
            arguments = [self.bindings.get(argument, argument) for argument in split_top_level(match.group(2))]
            if definition is None or len(definition.generics) != len(arguments):
                raise Unmapped(f"generic type: {rust_type}")
            saved = self.bindings
            self.bindings = {**saved, **dict(zip(definition.generics, arguments))}
            try:
                return self.project_definition(definition, inline=True)
            finally:
                self.bindings = saved
        if "<" in rust_type:
            raise Unmapped(f"generic type: {rust_type}")
        definition = self.lookup(rust_type)
        if definition is None:
            raise Unmapped(f"unresolved type: {rust_type}")
        return self.project_definition(definition)

    def project_definition(self, definition: TypeDef, inline: bool = False) -> dict:
        if definition.name in HANDWRITTEN_TYPES:
            return dict(HANDWRITTEN_TYPES[definition.name])
        if definition.kind == "alias":
            return self.project(definition.payload)
        if definition.kind == "unit-struct":
            return {"type": "null"}
        if definition.kind == "tuple-struct":
            if len(definition.payload) == 1:
                return self.project(definition.payload[0])
            raise Unmapped(f"{definition.name}: multi-field tuple struct has no value_derive wire form")
        if definition.container.transparent:
            if definition.kind == "struct" and len(definition.payload) == 1:
                return self.project(definition.payload[0][1])
            raise Unmapped(f"{definition.name}: #[value(transparent)] on a non-single-field container")
        name = definition.name
        if inline:
            if name in self.inlining:
                raise Unmapped(f"{name}: generic instantiation is self-referential and cannot be inlined")
            self.inlining.add(name)
            try:
                return self.project_struct(definition) if definition.kind == "struct" else self.project_enum(definition)
            finally:
                self.inlining.discard(name)
        if name in self.defs:
            return {"$ref": f"#/$defs/{name}"}
        if name in self.stack:
            return {"$ref": f"#/$defs/{name}"}
        self.stack.append(name)
        self.defs[name] = {}
        try:
            body = self.project_struct(definition) if definition.kind == "struct" else self.project_enum(definition)
        finally:
            self.stack.pop()
        self.defs[name] = body
        return {"$ref": f"#/$defs/{name}"}

    def project_struct(self, definition: TypeDef) -> dict:
        container = definition.container
        for gap in getattr(definition.payload, "gaps", []):
            raise Unmapped(f"{definition.name}: unreadable text in the struct body ({gap!r})")
        properties: "OrderedDict[str, dict]" = OrderedDict()
        required: list[str] = []
        flattened: list[dict] = []
        for ident, rust_type, attributes in definition.payload:
            if attributes.skip:
                continue
            override = None
            if attributes.custom is not None:
                codec = attributes.custom.split("::")[-1]
                if codec.endswith("deserialize_double_option"):
                    attributes.default = True
                elif codec in CUSTOM_CODECS:
                    override = CUSTOM_CODECS[codec]
                else:
                    raise Unmapped(f"{definition.name}.{ident}: custom codec {attributes.custom}")
            if attributes.flatten:
                flattened.append(self.project(rust_type))
                continue
            wire = field_wire_name(ident, attributes.rename, container.rename_all)
            properties[wire] = dict(override) if override is not None else self.project(rust_type)
            optional = attributes.default or container.default or re.match(r"^Option\s*<", rust_type.strip()) is not None
            if not optional:
                required.append(wire)
        body: dict = {"type": "object", "additionalProperties": False}
        if required:
            body["required"] = required
        body["properties"] = properties
        if flattened:
            body["additionalProperties"] = True
            return {"allOf": [body, *flattened]}
        return body

    def variant_named_body(self, definition: TypeDef, fields) -> tuple["OrderedDict[str, dict]", list[str]]:
        container = definition.container
        properties: "OrderedDict[str, dict]" = OrderedDict()
        required: list[str] = []
        for ident, rust_type, attributes in fields:
            if attributes.skip:
                continue
            wire = field_wire_name(ident, attributes.rename, container.field_rename_all())
            properties[wire] = self.project(rust_type)
            if not (attributes.default or re.match(r"^Option\s*<", rust_type.strip())):
                required.append(wire)
        return properties, required

    def expand_object(self, schema: dict) -> tuple["OrderedDict[str, dict]", list[str]] | None:
        """🫗 An internally tagged newtype variant splices its payload's own object entries beside the
        tag (`✨️derive/🦀️.rs:758-767`); a NON-object payload is wrapped under `"value"` instead."""
        resolved = schema
        reference = schema.get("$ref")
        if isinstance(reference, str) and reference.startswith("#/$defs/"):
            resolved = self.defs.get(reference[len("#/$defs/") :], {})
        if resolved.get("type") == "object" and "properties" in resolved:
            return OrderedDict(resolved["properties"]), list(resolved.get("required", []))
        return None

    def project_enum(self, definition: TypeDef) -> dict:
        container = definition.container
        variants = parse_variants(definition.payload) if isinstance(definition.payload, str) else definition.payload
        if not variants:
            raise Unmapped(f"{definition.name}: no enum variants parsed")
        names = [variant_wire_name(ident, attributes.rename, container.rename_all) for ident, _, _, attributes in variants]
        if all(shape == "unit" for _, shape, _, _ in variants) and container.tag is None:
            return {"type": "string", "enum": names}
        branches: list[dict] = []
        for (ident, shape, payload, _), wire in zip(variants, names):
            if container.tag is not None and container.content is not None:
                branch: dict = {"type": "object", "additionalProperties": False, "properties": {container.tag: {"const": wire}}}
                if shape == "unit":
                    branch["required"] = [container.tag]
                elif shape == "tuple":
                    if len(payload) != 1:
                        raise Unmapped(f"{definition.name}::{ident}: adjacently tagged multi-field tuple variant")
                    branch["properties"][container.content] = self.project(payload[0])
                    branch["required"] = [container.tag, container.content]
                else:
                    properties, required = self.variant_named_body(definition, payload)
                    branch["properties"][container.content] = {"type": "object", "additionalProperties": False, "required": required, "properties": properties} if required else {"type": "object", "additionalProperties": False, "properties": properties}
                    branch["required"] = [container.tag, container.content]
                branches.append(branch)
                continue
            if container.tag is not None:
                if shape == "tuple":
                    if len(payload) != 1:
                        raise Unmapped(f"{definition.name}::{ident}: internally tagged multi-field tuple variant")
                    inner = self.project(payload[0])
                    expanded = self.expand_object(inner)
                    if expanded is None:
                        merged = OrderedDict([(container.tag, {"const": wire}), ("value", inner)])
                        branches.append({"type": "object", "additionalProperties": False, "required": [container.tag, "value"], "properties": merged})
                        continue
                    properties, required = expanded
                else:
                    properties, required = self.variant_named_body(definition, payload) if shape == "named" else (OrderedDict(), [])
                merged: "OrderedDict[str, dict]" = OrderedDict()
                merged[container.tag] = {"const": wire}
                merged.update(properties)
                branches.append({"type": "object", "additionalProperties": False, "required": [container.tag, *required], "properties": merged})
                continue
            if shape == "unit":
                branches.append({"type": "string", "const": wire})
            elif shape == "tuple":
                if len(payload) != 1:
                    raise Unmapped(f"{definition.name}::{ident}: externally tagged multi-field tuple variant")
                branches.append({"type": "object", "additionalProperties": False, "required": [wire], "properties": {wire: self.project(payload[0])}})
            else:
                properties, required = self.variant_named_body(definition, payload)
                inner: dict = {"type": "object", "additionalProperties": False, "properties": properties}
                if required:
                    inner["required"] = required
                branches.append({"type": "object", "additionalProperties": False, "required": [wire], "properties": {wire: inner}})
        return {"oneOf": branches}


# ───────────────────────────── leaf discovery ─────────────────────────────


def leaf_identity(directory: str, descriptor: dict) -> str:
    """🆔 The execution-contract §A `$id` of one mutation leaf, derived from its path's artifact /
    standard / subset and the descriptor's `semanticKind` — never from the leaf directory name (gltf
    nests `<domain>/<verb>`). Single source of truth for `leaves()`, the aggregate `$ref`s and the
    per-subset projections, so an aggregate branch can never name an id no leaf declares."""
    segments = os.path.relpath(directory, REPO).split(os.sep)
    artifact_index = segments.index("🗿️artifacts")
    artifact = strip_leading_emoji(segments[artifact_index + 1])
    standard = strip_leading_emoji(segments[artifact_index + 3])
    subset = strip_leading_emoji(segments[artifact_index + 5])
    return f"{ID_ROOT}/{artifact}/{standard}/{subset}/mutation/{descriptor['semanticKind']}/schema.json"


def leaves() -> list[dict]:
    """📇 Every stdio mutation leaf carrying a descriptor, with its resolved schema paths."""
    found: list[dict] = []
    for directory, subdirectories, files in os.walk(ARTIFACTS):
        subdirectories[:] = [name for name in subdirectories if name not in ("target", "node_modules")]
        if MUTATIONS_DIR not in directory.split(os.sep):
            continue
        if "🔣️.json" not in files:
            continue
        path = os.path.join(directory, "🔣️.json")
        try:
            descriptor = json.load(open(path, encoding="utf8"))
        except Exception:
            continue
        if not isinstance(descriptor, dict) or "payloadSchema" not in descriptor or "semanticKind" not in descriptor:
            continue
        relative = os.path.relpath(directory, REPO)
        segments = relative.split(os.sep)
        artifact_index = segments.index("🗿️artifacts")
        artifact = strip_leading_emoji(segments[artifact_index + 1])
        standard = strip_leading_emoji(segments[artifact_index + 3])
        subset = strip_leading_emoji(segments[artifact_index + 5])
        mutation_index = len(segments) - 1 - segments[::-1].index(MUTATIONS_DIR)
        leaf = "/".join(strip_leading_emoji(segment) for segment in segments[mutation_index + 1 :])
        found.append(
            {
                "dir": directory,
                "rel": relative,
                "descriptor": descriptor,
                "descriptorPath": path,
                "artifact": artifact,
                "standard": standard,
                "subset": subset,
                "leaf": leaf,
                "id": leaf_identity(directory, descriptor),
                "declared": os.path.join(directory, *descriptor["payloadSchema"].split("/")),
                "canonical": os.path.join(directory, SCHEMA_DIR, "🔣️.json"),
                "module": os.path.join(REPO, os.sep.join(segments[: mutation_index + 1])),
                "artifactRoot": os.path.join(REPO, *segments[: artifact_index + 2]),
            }
        )
    return found


def aggregate_representation(module: str) -> dict:
    """🏷️ How one `🧬️mutations/🦀️.rs` aggregate enum puts a leaf on the wire.

    `value_derive` (`✨️derive/🦀️.rs:700-800`): `tag`+`content` is adjacent (`{tag, content}`), `tag`
    alone is internal (the leaf's own entries spliced beside the tag), no `tag` is external
    (`{"<Variant>": payload}`)."""
    path = os.path.join(module, "🦀️.rs")
    if not os.path.isfile(path):
        return {"kind": "absent", "tag": None, "content": None, "renameAll": None, "title": None}
    text = strip_comments(open(path, encoding="utf8").read())
    # 🎯 The aggregate is the enum carrying `#[mutations(...)]`, not merely the first `pub enum` in the
    # file — a mutations root may declare a helper vocabulary enum (e.g. a path segment) above it.
    match = None
    for candidate in re.finditer(r"\bpub enum ([A-Za-z_][A-Za-z0-9_]*)\s*\{", text):
        run, _ = attribute_runs(text, candidate.start())
        if "#[mutations(" in run.replace(" ", ""):
            match = candidate
            break
    if match is None:
        match = re.search(r"\bpub enum ([A-Za-z_][A-Za-z0-9_]*)\s*\{", text)
    if match is None:
        reexport = re.search(r"pub use crate::standards::([A-Za-z0-9_]+)::subsets::([A-Za-z0-9_]+)::schema::mutations::\*\s*;", text)
        if reexport is not None:
            source = reexported_module(module, reexport.group(1), reexport.group(2))
            if source is not None:
                return aggregate_representation(source)
        return {"kind": "absent", "tag": None, "content": None, "renameAll": None, "title": None}
    attributes, _ = attribute_runs(text, match.start())
    container = Container(attributes)
    kind = "adjacent" if container.tag and container.content else "internal" if container.tag else "external"
    return {"kind": kind, "tag": container.tag, "content": container.content, "renameAll": container.rename_all, "title": match.group(1)}


def reexported_module(module: str, standard_module: str, subset_module: str) -> str | None:
    """↪️ Follows a `pub use crate::standards::<s>::subsets::<x>::schema::mutations::*;` re-export back to
    the directory that owns the aggregate enum."""
    segments = module.split(os.sep)
    standards_index = len(segments) - 1 - segments[::-1].index("🏅️standards")
    standards_root = os.sep.join(segments[:standards_index])
    wanted_standard = re.sub(r"^v_", "", standard_module).replace("_", "-")
    for candidate in os.listdir(os.path.join(standards_root, "🏅️standards")):
        if strip_leading_emoji(candidate).replace(".", "-").replace("_", "-") != wanted_standard.replace(".", "-"):
            continue
        subsets = os.path.join(standards_root, "🏅️standards", candidate, "🪆️subsets")
        if not os.path.isdir(subsets):
            continue
        for subset in os.listdir(subsets):
            if strip_leading_emoji(subset).replace("-", "_") != subset_module:
                continue
            path = os.path.join(subsets, subset, SCHEMA_DIR, MUTATIONS_DIR)
            if os.path.isdir(path):
                return path
    return None


def discriminator(leaf: dict, representation: dict) -> str:
    return variant_wire_name(leaf["descriptor"]["aggregateVariant"], None, representation["renameAll"])


def payload_source(leaf: dict) -> tuple[str, str] | None:
    for candidate in ("🦀️.rs", os.path.join("🦠️mutation", "🦀️.rs")):
        path = os.path.join(leaf["dir"], candidate)
        if os.path.isfile(path):
            text = strip_comments(open(path, encoding="utf8").read())
            for match in re.finditer(r"#\[derive\(([^)]*)\)\]", text):
                if "MutationLeaf" not in match.group(1):
                    continue
                tail = re.compile(r"\b(?:pub\s+)?(?:struct|enum)\s+([A-Za-z_][A-Za-z0-9_]*)").search(text, match.end())
                if tail is not None:
                    return path, tail.group(1)
    return None


def attach_discriminator(document: dict, leaf: dict) -> bool:
    """🏷️ An internally tagged aggregate splices the leaf's own entries beside the discriminator, so the
    leaf's committed wire objects (`🧪️tests/*/🦠️mutation/🔣️.json`) carry it. A CLOSED payload schema has
    to declare it — as a non-required `const` — or it rejects its own aggregate-carried form."""
    representation = aggregate_representation(leaf["module"])
    if representation["kind"] != "internal":
        return False
    if document.get("additionalProperties") is not False or not isinstance(document.get("properties"), dict):
        return False
    tag = representation["tag"]
    if tag in document["properties"]:
        return False
    document["properties"] = OrderedDict([*document["properties"].items(), (tag, {"const": discriminator(leaf, representation), "description": f"Aggregate discriminator spliced in by {representation['title']}; absent when the payload stands alone."})])
    return True


def dump(path: str, document: dict) -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def ordered(document: dict) -> "OrderedDict[str, object]":
    order = ["$schema", "$id", "title", "description", "type", "additionalProperties", "required", "properties", "oneOf", "anyOf", "allOf", "enum", "const", "$defs"]
    out: "OrderedDict[str, object]" = OrderedDict()
    for key in order:
        if key in document:
            out[key] = document[key]
    for key, value in document.items():
        if key not in out:
            out[key] = value
    return out


# ───────────────────────────── commands ─────────────────────────────


# 🌍 Types stdio payloads name from outside their own artifact crate: the io `ArtifactRef` family and
# the store's `ArtifactChild`/`ArtifactLink`/`LinkPin` handles. Nothing else in the framework is
# reachable from a mutation payload, so the index stays targeted instead of walking the whole tree.
EXTERNAL_ROOTS = (
    os.path.join("🧰️framework", "🔨️modules", "🚪️io"),
    os.path.join("🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🏪️store"),
)


def shared_index() -> dict[str, list[TypeDef]]:
    return index_rust(STDIO, *(os.path.join(REPO, root) for root in EXTERNAL_ROOTS))


def command_census() -> None:
    inventory = leaves()
    present = [leaf for leaf in inventory if os.path.isfile(leaf["declared"])]
    missing = [leaf for leaf in inventory if not os.path.isfile(leaf["declared"])]
    canonical = [leaf for leaf in inventory if os.path.abspath(leaf["declared"]) == os.path.abspath(leaf["canonical"])]
    print(f"leaves={len(inventory)} present={len(present)} missing={len(missing)} already-canonical={len(canonical)}")
    print("declared payloadSchema spellings:", Counter(leaf["descriptor"]["payloadSchema"] for leaf in inventory).most_common())
    divergent = [leaf for leaf in inventory if leaf["leaf"].split("/")[-1] != leaf["descriptor"]["semanticKind"]]
    print(f"leaf-directory name != semanticKind: {len(divergent)}")
    for leaf in divergent[:20]:
        print("   ", leaf["rel"], "->", leaf["descriptor"]["semanticKind"])
    identifiers = Counter(leaf["id"] for leaf in inventory)
    collisions = [identifier for identifier, count in identifiers.items() if count > 1]
    print(f"$id collisions: {len(collisions)}")
    for identifier in collisions[:20]:
        print("   ", identifier)


def build_document(leaf: dict, index: dict[str, list[TypeDef]]) -> tuple[dict | None, str | None]:
    source = payload_source(leaf)
    variant = leaf["descriptor"].get("aggregateVariant")
    projector = Projector(index, leaf["rel"])
    if source is None:
        if variant is None or projector.lookup(variant) is None:
            return None, "no MutationLeaf payload type found at the leaf and aggregateVariant does not resolve"
        definition = projector.lookup(variant)
        title = definition.name
    else:
        _, name = source
        definition = projector.lookup(name)
        if definition is None:
            return None, f"payload type {name} not found in the artifact crate index"
        title = definition.name
    try:
        if definition.kind == "struct":
            body = projector.project_struct(definition)
        elif definition.kind == "enum":
            body = projector.project_enum(definition)
        else:
            body = projector.project_definition(definition)
    except Unmapped as error:
        return None, str(error)
    except RecursionError:
        return None, "recursive projection exceeded the interpreter stack"
    document: dict = {"$schema": DIALECT, "$id": leaf["id"], "title": title}
    document.update(body)
    attach_discriminator(document, leaf)
    if projector.defs:
        document["$defs"] = dict(projector.defs)
    return ordered(document), None


def command_author(write: bool) -> None:
    inventory = [leaf for leaf in leaves() if not os.path.isfile(leaf["declared"])]
    index = shared_index()
    authored = 0
    failures: list[tuple[str, str]] = []
    for leaf in inventory:
        document, problem = build_document(leaf, index)
        if document is None:
            failures.append((leaf["rel"], problem or "unknown"))
            continue
        if write:
            dump(leaf["canonical"], document)
            descriptor = leaf["descriptor"]
            descriptor["payloadSchema"] = CANONICAL_SCHEMA_REL
            with open(leaf["descriptorPath"], "w", encoding="utf8") as handle:
                json.dump(descriptor, handle, ensure_ascii=False, indent=2)
                handle.write("\n")
        authored += 1
    print(f"authored={authored} refused={len(failures)} (write={write})")
    for path, problem in failures:
        print("  REFUSED", path, "::", problem)


def command_audit(write: bool) -> None:
    """🔎 Compares every leaf schema on disk against the projection of its own Rust payload type and, with
    `--write`, replaces the ones whose top-level wire keys disagree. The crate is the authority: a
    hand-authored schema that names fields the payload struct does not put on the wire is wrong."""
    index = shared_index()
    agreed = 0
    replaced: list[str] = []
    unprojectable: list[tuple[str, str]] = []
    for leaf in leaves():
        document, problem = build_document(leaf, index)
        if document is None:
            unprojectable.append((leaf["rel"], problem or "unknown"))
            continue
        current = json.load(open(leaf["canonical"], encoding="utf8")) if os.path.isfile(leaf["canonical"]) else {}
        if schema_body(current) == schema_body(document):
            agreed += 1
            continue
        replaced.append(leaf["rel"])
        if write:
            # 📎 The projection replaces the SHAPE; annotations the payload struct cannot express
            # (`x-semio*` vocabulary metadata, a hand-written top-level `description`) are carried over.
            for key, value in current.items():
                if key.startswith("x-semio") or (key == "description" and "description" not in document):
                    document[key] = value
            dump(leaf["canonical"], ordered(document))
    print(f"leaf schemas agreeing with their Rust payload={agreed} disagreeing={len(replaced)} unprojectable={len(unprojectable)} (write={write})")
    for path in replaced:
        print("  DRIFTED", path)
    for path, problem in unprojectable:
        print("  UNPROJECTABLE", path, "::", problem)


def schema_body(document: dict) -> str:
    """🧾 The part of a leaf schema the Rust payload decides — identity and prose annotations excluded."""
    body = {key: value for key, value in document.items() if key not in ("$schema", "$id", "title", "description") and not key.startswith("x-semio")}
    return json.dumps(body, sort_keys=True, ensure_ascii=False)


def top_level_keys(document: dict) -> tuple:
    """🔑 The identity a projection and a hand-authored document must agree on: the wire key set, which
    keys are required, and (for a union payload) the branch count."""
    properties = document.get("properties")
    if isinstance(properties, dict):
        return ("object", tuple(sorted(properties)), tuple(sorted(document.get("required", []))))
    if isinstance(document.get("oneOf"), list):
        return ("union", len(document["oneOf"]))
    if isinstance(document.get("enum"), list):
        return ("enum", tuple(sorted(str(value) for value in document["enum"])))
    return ("opaque",)


def command_relocate(write: bool) -> None:
    moved = 0
    rewritten = 0
    for leaf in leaves():
        declared = leaf["declared"]
        canonical = leaf["canonical"]
        if not os.path.isfile(declared):
            continue
        document = json.load(open(declared, encoding="utf8"))
        document["$schema"] = DIALECT
        document["$id"] = leaf["id"]
        if "title" not in document:
            source = payload_source(leaf)
            document["title"] = source[1] if source is not None else leaf["descriptor"]["aggregateVariant"]
        migrate_dialect(document)
        attach_discriminator(document, leaf)
        document = ordered(document)
        if write:
            if os.path.abspath(declared) != os.path.abspath(canonical):
                dump(canonical, document)
                os.remove(declared)
                moved += 1
            else:
                dump(canonical, document)
            rewritten += 1
            descriptor = leaf["descriptor"]
            if descriptor["payloadSchema"] != CANONICAL_SCHEMA_REL:
                descriptor["payloadSchema"] = CANONICAL_SCHEMA_REL
                with open(leaf["descriptorPath"], "w", encoding="utf8") as handle:
                    json.dump(descriptor, handle, ensure_ascii=False, indent=2)
                    handle.write("\n")
        else:
            if os.path.abspath(declared) != os.path.abspath(canonical):
                moved += 1
            rewritten += 1
    print(f"relocated={moved} rewritten={rewritten} (write={write})")


def migrate_dialect(node) -> None:
    """📐 draft-07 has no `prefixItems`/`unevaluated*`; rewrite them in place."""
    if isinstance(node, dict):
        if "prefixItems" in node:
            node["items"] = node.pop("prefixItems")
            node.setdefault("additionalItems", False)
        for key in ("unevaluatedProperties", "unevaluatedItems", "$recursiveRef", "$recursiveAnchor", "$dynamicRef", "$dynamicAnchor"):
            node.pop(key, None)
        for value in node.values():
            migrate_dialect(value)
    elif isinstance(node, list):
        for value in node:
            migrate_dialect(value)


def aggregate_roots() -> dict[str, list[dict]]:
    roots: dict[str, list[dict]] = defaultdict(list)
    for leaf in leaves():
        segments = leaf["dir"].split(os.sep)
        mutation_index = len(segments) - 1 - segments[::-1].index(MUTATIONS_DIR)
        roots[os.sep.join(segments[: mutation_index + 1])].append(leaf)
    return roots


def command_aggregates(write: bool) -> None:
    converted = 0
    skipped: list[str] = []
    kinds = Counter()
    for root, members in sorted(aggregate_roots().items()):
        path = os.path.join(root, "🔣️.json")
        if os.path.dirname(root).split(os.sep)[-1] != SCHEMA_DIR:
            skipped.append(f"{os.path.relpath(root, REPO)} :: not a 🧬️schema/🧬️mutations root")
            continue
        representation = aggregate_representation(root)
        first = members[0]
        if representation["kind"] == "absent":
            skipped.append(f"{os.path.relpath(root, REPO)} :: no aggregate enum in 🦀️.rs")
            continue
        kinds[representation["kind"]] += 1
        branches = []
        for leaf in sorted(members, key=lambda item: item["leaf"]):
            # 🔗 Cross-partition row 79: a branch names the leaf's absolute `$id` and is resolved
            # through the catalog. No leaf declares `$defs.Payload` — the leaf root IS the payload —
            # so the bare `$id` is the reference, never `<$id>#/$defs/Payload`.
            target = leaf["id"]
            wire = discriminator(leaf, representation)
            if representation["kind"] == "adjacent":
                branches.append({"type": "object", "additionalProperties": False, "required": [representation["tag"], representation["content"]], "properties": {representation["tag"]: {"const": wire}, representation["content"]: {"$ref": target}}})
            elif representation["kind"] == "external":
                branches.append({"type": "object", "additionalProperties": False, "required": [wire], "properties": {wire: {"$ref": target}}})
            else:
                branches.append({"allOf": [{"$ref": target}, {"type": "object", "required": [representation["tag"]], "properties": {representation["tag"]: {"const": wire}}}]})
        document = ordered(
            {
                "$schema": DIALECT,
                "$id": f"{ID_ROOT}/{first['artifact']}/{first['standard']}/{first['subset']}/mutations.json",
                "title": representation["title"],
                "description": f"Discriminated union over this module's mutation leaves ({representation['kind']}ly tagged); every payload is the leaf's own schema, referenced, never restated.",
                "oneOf": branches,
            }
        )
        if write:
            dump(path, document)
        converted += 1
    print(f"aggregates={converted} {dict(kinds)} skipped={len(skipped)} (write={write})")
    for entry in skipped:
        print("  SKIPPED", entry)


def branch_reference(branch) -> str | None:
    """🔗 The single leaf `$ref` an aggregate branch carries, whatever the tagging shape (adjacent and
    external nest it under a property, internal puts it first in an `allOf`). Shape-agnostic so a
    catalogue can be re-read after its branches were rewritten to absolute `$id`s."""
    if not isinstance(branch, dict):
        return None
    for entry in branch.get("allOf", []) if isinstance(branch.get("allOf"), list) else []:
        if isinstance(entry, dict) and isinstance(entry.get("$ref"), str):
            return entry["$ref"]
    for value in branch.get("properties", {}).values() if isinstance(branch.get("properties"), dict) else []:
        if isinstance(value, dict) and isinstance(value.get("$ref"), str):
            return value["$ref"]
    return None


def command_projections(write: bool) -> None:
    """🪞 Rewrites the per-subset gltf mutation catalogues: they own no leaves of their own, they are
    views over another subset's leaves, and their `$ref`s pointed at leaf paths that do not exist here.
    Cross-partition row 79: a view branch names the owning leaf's absolute `$id`, exactly like an owning
    aggregate, so a view never encodes a filesystem traversal into a sibling subset."""
    owners = {root: members for root, members in aggregate_roots().items()}
    by_id = {leaf["id"]: leaf for leaf in leaves()}
    rewritten = 0
    skipped: list[str] = []
    for directory, subdirectories, files in os.walk(ARTIFACTS):
        if os.path.basename(directory) != MUTATIONS_DIR or os.path.basename(os.path.dirname(directory)) != SCHEMA_DIR:
            continue
        if directory in owners or "🔣️.json" not in files:
            continue
        path = os.path.join(directory, "🔣️.json")
        document = json.load(open(path, encoding="utf8"))
        references = [branch_reference(branch) for branch in document.get("oneOf", [])]
        references = [reference for reference in references if isinstance(reference, str)]
        if not references:
            skipped.append(f"{os.path.relpath(directory, REPO)} :: no leaf $refs to repoint")
            continue
        subsets_root = os.path.dirname(os.path.dirname(os.path.dirname(directory)))
        resolved: list[tuple[str, dict]] = []
        owner_module = None
        for reference in references:
            if reference in by_id:
                owner = by_id[reference]
                owner_module = owner["module"]
                resolved.append((owner["dir"], owner["descriptor"]))
                continue
            parts = [part for part in reference.split("/") if part not in ("", ".")]
            leaf_rel = "/".join(parts[:-2] if len(parts) >= 2 and parts[-2] == SCHEMA_DIR else parts[:-1])
            for sibling in sorted(os.listdir(subsets_root)):
                candidate = os.path.join(subsets_root, sibling, SCHEMA_DIR, MUTATIONS_DIR, *leaf_rel.split("/"))
                if not os.path.isfile(os.path.join(candidate, "🔣️.json")):
                    continue
                descriptor = json.load(open(os.path.join(candidate, "🔣️.json"), encoding="utf8"))
                if not isinstance(descriptor, dict) or "aggregateVariant" not in descriptor:
                    continue
                owner_module = os.path.join(subsets_root, sibling, SCHEMA_DIR, MUTATIONS_DIR)
                resolved.append((candidate, descriptor))
                break
            else:
                skipped.append(f"{os.path.relpath(directory, REPO)} :: {reference} resolves to no leaf in any sibling subset")
                owner_module = None
                break
        if owner_module is None:
            continue
        representation = aggregate_representation(owner_module)
        segments = os.path.relpath(directory, REPO).split(os.sep)
        artifact_index = segments.index("🗿️artifacts")
        branches = []
        for candidate, descriptor in resolved:
            target = leaf_identity(candidate, descriptor)
            wire = variant_wire_name(descriptor["aggregateVariant"], None, representation["renameAll"])
            if representation["kind"] == "adjacent":
                branches.append({"type": "object", "additionalProperties": False, "required": [representation["tag"], representation["content"]], "properties": {representation["tag"]: {"const": wire}, representation["content"]: {"$ref": target}}})
            elif representation["kind"] == "external":
                branches.append({"type": "object", "additionalProperties": False, "required": [wire], "properties": {wire: {"$ref": target}}})
            else:
                branches.append({"allOf": [{"$ref": target}, {"type": "object", "required": [representation["tag"]], "properties": {representation["tag"]: {"const": wire}}}]})
        view = ordered(
            {
                "$schema": DIALECT,
                "$id": f"{ID_ROOT}/{strip_leading_emoji(segments[artifact_index + 1])}/{strip_leading_emoji(segments[artifact_index + 3])}/{strip_leading_emoji(segments[artifact_index + 5])}/mutations.json",
                "title": document.get("title") or representation["title"],
                "description": f"View over the {strip_leading_emoji(os.path.basename(os.path.dirname(os.path.dirname(owner_module))))} subset's mutation leaves ({representation['kind']}ly tagged); every payload is the owning leaf's own schema, referenced, never restated.",
                "oneOf": branches,
            }
        )
        if write:
            dump(path, view)
        rewritten += 1
    print(f"projections={rewritten} skipped={len(skipped)} (write={write})")
    for entry in skipped:
        print("  SKIPPED", entry)


def declaration_line(path: str, kind: str, name: str) -> tuple[int, int]:
    """📍 1-based line of a `struct`/`enum` declaration and of the first line of its attribute run —
    the insertion point for a new container attribute. `(0, 0)` when the declaration cannot be located."""
    absolute = os.path.join(REPO, path)
    try:
        raw = open(absolute, encoding="utf8").read()
    except OSError:
        return (0, 0)
    text = strip_comments(raw)
    keyword = "struct" if kind.endswith("struct") else kind
    match = re.search(rf"\b(?:pub(?:\s*\([^)]*\))?\s+)?{keyword}\s+{re.escape(name)}\s*(?:<[^{{;(]*>)?\s*[{{(;]", text)
    if match is None:
        return (0, 0)
    _, attribute_start = attribute_runs(text, match.start())
    return (text.count("\n", 0, match.start()) + 1, text.count("\n", 0, attribute_start) + 1)


def reachable_definitions(leaf: dict, index: dict[str, list[TypeDef]]) -> "OrderedDict[str, TypeDef]":
    """🕸️ Every named struct/enum the leaf's projection walks through, root first."""
    seen: "OrderedDict[str, TypeDef]" = OrderedDict()
    original_struct, original_enum = Projector.project_struct, Projector.project_enum

    def remember(definition: TypeDef) -> None:
        seen.setdefault(f"{definition.path}::{definition.name}", definition)

    def patched_struct(self, definition):  # type: ignore[no-untyped-def]
        remember(definition)
        return original_struct(self, definition)

    def patched_enum(self, definition):  # type: ignore[no-untyped-def]
        remember(definition)
        return original_enum(self, definition)

    Projector.project_struct, Projector.project_enum = patched_struct, patched_enum
    try:
        build_document(leaf, index)
    finally:
        Projector.project_struct, Projector.project_enum = original_struct, original_enum
    return seen


def rename_map(before: dict, after: dict) -> dict[str, str]:
    """🔤 snake → camel field renames between two projections of the same leaf, read off the property
    key sets rather than re-deriving the casing."""
    renames: dict[str, str] = {}

    def walk(left, right) -> None:
        if isinstance(left, dict) and isinstance(right, dict):
            left_properties, right_properties = left.get("properties"), right.get("properties")
            if isinstance(left_properties, dict) and isinstance(right_properties, dict):
                for old, new in zip(left_properties, right_properties):
                    if old != new:
                        renames[old] = new
            for key in left:
                if key in right:
                    walk(left[key], right[key])
        elif isinstance(left, list) and isinstance(right, list) and len(left) == len(right):
            for one, other in zip(left, right):
                walk(one, other)

    walk(before, after)
    return renames


def has_free_map(document) -> bool:
    """🗺️ True when the projection contains a string-keyed map, i.e. object keys that are DATA and must
    not be re-cased by a key substitution."""
    if isinstance(document, dict):
        if isinstance(document.get("additionalProperties"), dict):
            return True
        return any(has_free_map(value) for value in document.values())
    if isinstance(document, list):
        return any(has_free_map(value) for value in document)
    return False


def object_keys(node) -> list[str]:
    if isinstance(node, dict):
        return [key for entry in node.items() for key in (entry[0], *object_keys(entry[1]))]
    if isinstance(node, list):
        return [key for value in node for key in object_keys(value)]
    return []


def recase(node, renames: dict[str, str]):
    if isinstance(node, dict):
        return OrderedDict((renames.get(key, key), recase(value, renames)) for key, value in node.items())
    if isinstance(node, list):
        return [recase(value, renames) for value in node]
    return node


def command_casing() -> None:
    """🐫 Plans cross-partition requests 46 and 49 without building stdio: every container reached by a
    leaf projection that declares no `#[value(rename_all)]`, with the file:line to annotate, plus the
    committed fixtures whose keys the annotation moves."""
    global ASSUME_CAMEL_CASE
    index = shared_index()
    inventory = leaves()
    roots: dict[str, TypeDef] = {}
    reached: "OrderedDict[str, TypeDef]" = OrderedDict()
    per_leaf: dict[str, "OrderedDict[str, TypeDef]"] = {}
    for leaf in inventory:
        found = reachable_definitions(leaf, index)
        per_leaf[leaf["rel"]] = found
        if found:
            roots[next(iter(found))] = found[next(iter(found))]
        for key, definition in found.items():
            reached.setdefault(key, definition)
    bare = OrderedDict((key, definition) for key, definition in reached.items() if definition.container.rename_all is None and definition.container.rename_all_fields is None)
    print(f"reached-containers={len(reached)} without-rename-all={len(bare)} roots={len(roots)} roots-without-rename-all={len([key for key in roots if key in bare])}")
    print("## containers to annotate")
    for key, definition in bare.items():
        declaration, attributes = declaration_line(definition.path, definition.kind, definition.name)
        multiword = [ident for ident, _, _ in definition.payload if "_" in ident] if definition.kind == "struct" and not isinstance(definition.payload, str) else []
        print(f"ANNOTATE\t{definition.path}:{declaration}\tattrs@{attributes}\t{definition.kind}\t{definition.name}\troot={'yes' if key in roots else 'no'}\tmultiword={len(multiword)}\t{','.join(multiword)}")
    print("## fixtures")
    for leaf in inventory:
        fixtures = fixture_paths(leaf)
        if not fixtures:
            continue
        before, problem = build_document(leaf, index)
        if before is None:
            print(f"FIXTURE-UNPROJECTABLE\t{leaf['rel']}\t{problem}")
            continue
        ASSUME_CAMEL_CASE = True
        try:
            after, _ = build_document(leaf, index)
        finally:
            ASSUME_CAMEL_CASE = False
        renames = rename_map(before, after) if after is not None else {}
        for fixture in fixtures:
            data = json.load(open(fixture, encoding="utf8"), object_pairs_hook=OrderedDict)
            touched = sorted(set(object_keys(data)) & set(renames))
            if not touched:
                continue
            relative = os.path.relpath(fixture, REPO)
            flag = "REVIEW" if has_free_map(before) else "MECHANICAL"
            print(f"FIXTURE\t{flag}\t{relative}\t{';'.join(f'{name}->{renames[name]}' for name in touched)}")
            print("FIXTURE-JSON\t" + json.dumps(recase(data, renames), ensure_ascii=False))


def fixture_paths(leaf: dict) -> list[str]:
    """🧪 The leaf's committed wire vectors (`🧪️tests/<case>/🦠️mutation/🔣️.json`)."""
    tests = os.path.join(leaf["dir"], "🧪️tests")
    found: list[str] = []
    for directory, _, files in os.walk(tests):
        if os.path.basename(directory) != "🦠️mutation" or "🔣️.json" not in files:
            continue
        found.append(os.path.join(directory, "🔣️.json"))
    return sorted(found)


def command_ids(write: bool) -> None:
    """🆔 Rewrites every leaf `$id` to the execution-contract §A grammar and proves the result is a
    bijection: 915 leaves → 915 distinct ids. The body is untouched (`$id` is identity, not shape), so
    this is independent of `audit`, whose comparison deliberately excludes `$id`."""
    inventory = leaves()
    claimed: dict[str, list[str]] = defaultdict(list)
    for leaf in inventory:
        claimed[leaf["id"]].append(leaf["rel"])
    collisions = {identity: paths for identity, paths in claimed.items() if len(paths) > 1}
    rewritten: list[str] = []
    unchanged = 0
    missing: list[str] = []
    for leaf in inventory:
        path = leaf["canonical"]
        if not os.path.isfile(path):
            missing.append(leaf["rel"])
            continue
        document = json.load(open(path, encoding="utf8"), object_pairs_hook=OrderedDict)
        if document.get("$id") == leaf["id"]:
            unchanged += 1
            continue
        document["$id"] = leaf["id"]
        rewritten.append(leaf["rel"])
        if write:
            dump(path, ordered(document))
    aggregates = 0
    aggregate_rewritten: list[str] = []
    aggregate_claimed: dict[str, list[str]] = defaultdict(list)
    for module, members in aggregate_roots().items():
        path = os.path.join(module, "🔣️.json")
        if not os.path.isfile(path):
            continue
        first = members[0]
        identity = f"{ID_ROOT}/{first['artifact']}/{first['standard']}/{first['subset']}/mutations.json"
        aggregates += 1
        aggregate_claimed[identity].append(os.path.relpath(path, REPO))
        document = json.load(open(path, encoding="utf8"), object_pairs_hook=OrderedDict)
        if document.get("$id") == identity:
            continue
        document["$id"] = identity
        aggregate_rewritten.append(os.path.relpath(path, REPO))
        if write:
            dump(path, ordered(document))
    aggregate_collisions = {identity: paths for identity, paths in aggregate_claimed.items() if len(paths) > 1}
    print(f"leaves={len(inventory)} distinct-ids={len(claimed)} collisions={len(collisions)} rewritten={len(rewritten)} already-correct={unchanged} missing-file={len(missing)} (write={write})")
    print(f"owning-aggregates={aggregates} distinct-aggregate-ids={len(aggregate_claimed)} aggregate-collisions={len(aggregate_collisions)} aggregate-rewritten={len(aggregate_rewritten)}")
    for identity, paths in list(collisions.items())[:40]:
        print("   COLLISION", identity)
        for path in paths:
            print("      ", path)
    for identity, paths in list(aggregate_collisions.items())[:40]:
        print("   AGGREGATE COLLISION", identity, paths)
    for path in missing[:20]:
        print("   MISSING", path)


def rust_module_name(directory: str) -> str:
    """🦀 The Rust module identifier a standard/subset directory compiles to: the ascii tail with every
    non-alphanumeric character replaced by `_`, prefixed `v`/`v_` for a standard (`🔖️2.0` → `v2_0`,
    `🔖️ecma-376` → `v_ecma_376`, `🔖️2x3` → `v2x3`). Derived, never tabled — the crates spell it this
    way in every `#[path]`/`pub use` in the partition."""
    tail = strip_leading_emoji(directory).lower()
    return re.sub(r"[^a-z0-9]", "_", tail)


def standard_module_name(directory: str) -> str:
    body = rust_module_name(directory)
    return f"v{body}" if body[:1].isdigit() else f"v_{body}"


def subset_modules() -> list[dict]:
    """🪆 Every `🪆️subsets/<s>/🧬️schema` module of the partition, with the pieces its `$id` is made of."""
    found: list[dict] = []
    for artifact in sorted(os.listdir(ARTIFACTS)):
        standards = os.path.join(ARTIFACTS, artifact, "🏅️standards")
        if not os.path.isdir(standards):
            continue
        for standard in sorted(os.listdir(standards)):
            subsets = os.path.join(standards, standard, "🪆️subsets")
            if not os.path.isdir(subsets):
                continue
            for subset in sorted(os.listdir(subsets)):
                module = os.path.join(subsets, subset, SCHEMA_DIR)
                if not os.path.isdir(module):
                    continue
                found.append(
                    {
                        "module": module,
                        "rel": os.path.relpath(module, REPO),
                        "artifact_dir": artifact,
                        "standard_dir": standard,
                        "subset_dir": subset,
                        "artifact": strip_leading_emoji(artifact),
                        "standard": strip_leading_emoji(standard),
                        "subset": strip_leading_emoji(subset),
                        "id": f"{ID_ROOT}/{strip_leading_emoji(artifact)}/{strip_leading_emoji(standard)}/{strip_leading_emoji(subset)}/artifact.json",
                        "root": os.path.join(module, "🔣️.json"),
                    }
                )
    return found


def reexported_schema_owner(entry: dict) -> dict | None:
    """🔗 The subset a profile module re-exports its artifact state from, read out of the module's own
    `pub use crate::standards::<v>::subsets::<s>::schema::*;` — the single statement that makes the
    profile's Rust artifact type the base subset's type rather than one of its own."""
    source = os.path.join(entry["module"], "🦀️.rs")
    if not os.path.isfile(source):
        return None
    text = strip_comments(open(source, encoding="utf8").read())
    match = re.search(r"pub\s+use\s+crate::standards::(\w+)::subsets::(\w+)::schema::\*\s*;", text)
    if match is None:
        return None
    standard_module, subset_module = match.group(1), match.group(2)
    for candidate in subset_modules():
        if candidate["artifact_dir"] != entry["artifact_dir"]:
            continue
        if standard_module_name(candidate["standard_dir"]) != standard_module:
            continue
        if rust_module_name(candidate["subset_dir"]) != subset_module:
            continue
        return candidate
    return None


def viewed_schema_owner(entry: dict) -> dict | None:
    """👁 The subset a view module's mutation catalogue draws its leaves from: every branch `$ref` of
    `🧬️mutations/🔣️.json` is an absolute leaf `$id` under the owning subset's scope path (row 79), so
    the owner is read from the references rather than guessed from the directory."""
    aggregate = os.path.join(entry["module"], MUTATIONS_DIR, "🔣️.json")
    if not os.path.isfile(aggregate):
        return None
    body = open(aggregate, encoding="utf8").read()
    owners = {tuple(match.group(1).split("/")) for match in re.finditer(rf"{re.escape(ID_ROOT)}/([^/]+/[^/]+/[^/]+)/mutation/", body)}
    if len(owners) != 1:
        return None
    artifact, standard, subset = next(iter(owners))
    for candidate in subset_modules():
        if (candidate["artifact"], candidate["standard"], candidate["subset"]) == (artifact, standard, subset):
            return candidate
    return None


def attach_state_lanes(document: dict, definition: TypeDef, projector: Projector) -> None:
    """🪧 Copies the artifact struct's `#[state(<lane>)]` field attributes onto the projected properties
    as `x-semio-state`, the annotation every sibling subset root already carries. The lane is read from
    the declaring source rather than inferred: a field with no attribute gets none."""
    properties = document.get("properties")
    if not isinstance(properties, dict):
        return
    text = strip_comments(open(definition.path, encoding="utf8").read())
    body = re.search(rf"pub\s+struct\s+{re.escape(definition.name)}\b[^{{]*{{", text)
    if body is None:
        return
    region = text[body.end() : match_block(text, body.end() - 1)]
    lanes = {name: lane for lane, name in re.findall(r"#\[state\(\s*(\w+)\s*\)\][\s\S]*?pub\s+(\w+)\s*:", region)}
    for field, lane in lanes.items():
        wire = field_wire_name(field, None, definition.container.rename_all)
        if wire in properties and isinstance(properties[wire], dict):
            properties[wire]["x-semio-state"] = lane


def root_export(entry: dict) -> tuple[str, str] | None:
    """🏷 A subset root's `(title, $id)` — the export id the module publishes and the identity another
    module references it by."""
    if not os.path.isfile(entry["root"]):
        return None
    document = json.load(open(entry["root"], encoding="utf8"))
    title, identity = document.get("title"), document.get("$id")
    return (title, identity) if isinstance(title, str) and isinstance(identity, str) else None


def command_roots(write: bool) -> None:
    """🌱 Authors the missing subset-root artifact documents (`🪆️subsets/<s>/🧬️schema/🔣️.json`).

    A subset that declares its own artifact struct gets the `value_derive` projection of that struct
    (`x-semio-state` per `#[state(...)]`, nested `$defs`, crate casing) — the same reading `author`
    applies to mutation payloads. A subset that RE-EXPORTS another subset's schema module, or that is
    a per-domain view over one, owns no artifact state of its own: execution contract §A forbids
    restating another scope's `$defs`, so its root names the owning scope's document by absolute `$id`
    and annotates `x-semio-formats` — in such a module the type exists in JSON Schema only, because
    the Rust glob re-export and the meta-only TypeScript leaf declare nothing (contract §A)."""
    index = shared_index()
    authored = projected = referenced = 0
    unmapped: list[str] = []
    for entry in subset_modules():
        if os.path.isfile(entry["root"]):
            continue
        own = None
        source = os.path.join(entry["module"], "🦀️.rs")
        if os.path.isfile(source):
            text = strip_comments(open(source, encoding="utf8").read())
            match = re.search(r"pub\s+struct\s+(\w+Artifact)\b", text)
            own = match.group(1) if match else None
        if own is not None:
            projector = Projector(index, entry["rel"])
            definition = projector.lookup(own)
            if definition is None:
                unmapped.append(f"{entry['rel']}: declares `pub struct {own}` and the index carries no definition for it")
                continue
            try:
                body = projector.project_struct(definition)
            except Unmapped as error:
                unmapped.append(f"{entry['rel']}: {own} — {error}")
                continue
            document = OrderedDict({"$schema": DIALECT, "$id": entry["id"], "title": own})
            document.update(body)
            attach_state_lanes(document, definition, projector)
            if projector.defs:
                document["$defs"] = dict(projector.defs)
            projected += 1
        else:
            owner = reexported_schema_owner(entry) or viewed_schema_owner(entry)
            if owner is None:
                unmapped.append(f"{entry['rel']}: no own artifact struct, no `pub use …::schema::*`, no view catalogue to read an owner from")
                continue
            export = root_export(owner)
            if export is None:
                unmapped.append(f"{entry['rel']}: owner {owner['rel']} declares no title/$id to reference")
                continue
            title, identity = export
            relation = "re-exports" if reexported_schema_owner(entry) is not None else "is a per-domain view over"
            document = OrderedDict(
                {
                    "$schema": DIALECT,
                    "$id": entry["id"],
                    "title": title,
                    "description": f"{entry['artifact']} {entry['standard']}/{entry['subset']} {relation} the {owner['subset']} subset's {title}; the artifact contract is that scope's and is referenced, never restated.",
                    "type": "object",
                    "allOf": [{"$ref": identity}],
                    "x-semio-formats": ["🔣️jsonschema"],
                }
            )
            referenced += 1
        authored += 1
        if write:
            dump(entry["root"], ordered(document))
    print(f"subset-modules={len(subset_modules())} authored={authored} projected={projected} referenced={referenced} unmapped={len(unmapped)} (write={write})")
    for problem in unmapped:
        print("   UNMAPPED", problem)


MUTATION_FACETS = {"📝️text": "text", "💾️binary": "binary"}


def aggregate_title(module: str) -> str:
    """🏷️ The `title` of a mutations module's aggregate document — the export id every codec facet of
    that module is named after."""
    document = json.load(open(os.path.join(module, "🔣️.json"), encoding="utf8"))
    return document["title"]


def facet_documents() -> list[dict]:
    """🧾 The codec facet documents of a mutations module (`🧬️schema/🧬️mutations/📝️text|💾️binary/🔣️.json`).
    They are facets of the SUBSET scope, so contract §A gives them the module's own scope path with a
    two-segment facet filename (`mutations/text.json`), never a scope of their own. A `🚪️io/🧬️mutations`
    tree is an io collection descriptor, not a schema module, and is out of this partition."""
    found: list[dict] = []
    for directory, subdirectories, files in os.walk(ARTIFACTS):
        subdirectories[:] = [name for name in subdirectories if name not in ("target", "node_modules")]
        segments = os.path.relpath(directory, REPO).split(os.sep)
        if MUTATIONS_DIR not in segments or "🔣️.json" not in files:
            continue
        mutation_index = segments.index(MUTATIONS_DIR)
        if mutation_index == 0 or segments[mutation_index - 1] != SCHEMA_DIR:
            continue
        tail = segments[mutation_index + 1 :]
        if len(tail) != 1 or tail[0] not in MUTATION_FACETS:
            continue
        artifact_index = segments.index("🗿️artifacts")
        artifact = strip_leading_emoji(segments[artifact_index + 1])
        standard = strip_leading_emoji(segments[artifact_index + 3])
        subset = strip_leading_emoji(segments[artifact_index + 5])
        facet = MUTATION_FACETS[tail[0]]
        found.append(
            {
                "path": os.path.join(directory, "🔣️.json"),
                "rel": os.path.relpath(os.path.join(directory, "🔣️.json"), REPO),
                "artifact": artifact,
                "subset": subset,
                "facet": facet,
                "id": f"{ID_ROOT}/{artifact}/{standard}/{subset}/mutations/{facet}.json",
                # 🏷️ Cross-partition row 81: one spelling, derived from the aggregate this document is a
                # codec facet of — `<AggregateTitle>Text` / `<AggregateTitle>Binary`, and every stdio
                # aggregate title already ends in `Mutation`, so the result is `<…>MutationText`.
                "title": f"{aggregate_title(os.path.dirname(directory))}{facet.capitalize()}",
            }
        )
    return sorted(found, key=lambda entry: entry["rel"])


def command_facets(write: bool) -> None:
    """🧷 Brings the mutation codec facet documents onto the contract: draft-07 dialect, one `$id` per
    document under the module's scope path, `$defs` instead of draft-07's `definitions`, and a
    PascalCase export id where the title is still a generator slug."""
    dialects = ids = defs = titles = 0
    for entry in facet_documents():
        document = json.load(open(entry["path"], encoding="utf8"), object_pairs_hook=OrderedDict)
        touched = False
        if document.get("$schema") != DIALECT:
            migrate_dialect(document)
            document["$schema"] = DIALECT
            dialects += 1
            touched = True
        if document.get("$id") != entry["id"]:
            document["$id"] = entry["id"]
            ids += 1
            touched = True
        # 🧩 Cross-partition row 77: a codec facet document exports exactly one thing — its `title` —
        # so every named subschema in it is a module-internal helper and belongs in `definitions`.
        # Leaving them in `$defs` makes them exports of the subset scope and collides with the
        # same-named export of a sibling facet (`SemioFlowSnapshot`, semio/flow).
        if "$defs" in document:
            document["definitions"] = document.pop("$defs")
            body = json.dumps(document, ensure_ascii=False).replace("#/$defs/", "#/definitions/")
            document = json.loads(body, object_pairs_hook=OrderedDict)
            defs += 1
            touched = True
        if document.get("title") != entry["title"]:
            document["title"] = entry["title"]
            titles += 1
            touched = True
        if touched and write:
            dump(entry["path"], ordered(document))
    print(f"facet-documents={len(facet_documents())} dialect-fixed={dialects} id-fixed={ids} defs-to-definitions={defs} title-fixed={titles} (write={write})")
    claimed: dict[str, list[str]] = defaultdict(list)
    for entry in facet_documents():
        claimed[entry["id"]].append(entry["rel"])
    print(f"distinct-facet-ids={len(claimed)} collisions={len([one for one in claimed.values() if len(one) > 1])}")


def command_verify() -> None:
    inventory = leaves()
    problems: list[str] = []
    for leaf in inventory:
        declared = leaf["declared"]
        if leaf["descriptor"]["payloadSchema"] != CANONICAL_SCHEMA_REL:
            problems.append(f"{leaf['rel']}: payloadSchema is {leaf['descriptor']['payloadSchema']!r}, not {CANONICAL_SCHEMA_REL!r}")
            continue
        if not os.path.isfile(declared):
            problems.append(f"{leaf['rel']}: declared payload schema missing at {os.path.relpath(declared, REPO)}")
            continue
        try:
            document = json.load(open(declared, encoding="utf8"))
        except Exception as error:
            problems.append(f"{leaf['rel']}: unparseable schema ({error})")
            continue
        if document.get("$schema") != DIALECT:
            problems.append(f"{leaf['rel']}: dialect is {document.get('$schema')!r}")
        if document.get("$id") != leaf["id"]:
            problems.append(f"{leaf['rel']}: $id is {document.get('$id')!r}, expected {leaf['id']!r}")
        if not isinstance(document.get("title"), str) or document["title"] == "":
            problems.append(f"{leaf['rel']}: missing title")
    stale = []
    for directory, subdirectories, files in os.walk(ARTIFACTS):
        if MUTATIONS_DIR not in directory.split(os.sep):
            continue
        for name in files:
            if name.endswith(".schema.json"):
                stale.append(os.path.relpath(os.path.join(directory, name), REPO))
    print(f"leaves={len(inventory)} problems={len(problems)} stale-schema-files={len(stale)}")
    for problem in problems[:60]:
        print("  ", problem)
    for path in stale[:60]:
        print("   STALE", path)


def main() -> int:
    command = sys.argv[1] if len(sys.argv) > 1 else "census"
    write = "--write" in sys.argv
    if command == "census":
        command_census()
    elif command == "author":
        command_author(write)
    elif command == "audit":
        command_audit(write)
    elif command == "relocate":
        command_relocate(write)
    elif command == "aggregates":
        command_aggregates(write)
    elif command == "projections":
        command_projections(write)
    elif command == "ids":
        command_ids(write)
    elif command == "casing":
        command_casing()
    elif command == "facets":
        command_facets(write)
    elif command == "roots":
        command_roots(write)
    elif command == "verify":
        command_verify()
    else:
        print(__doc__)
        return 2
    return 0


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    raise SystemExit(main())
