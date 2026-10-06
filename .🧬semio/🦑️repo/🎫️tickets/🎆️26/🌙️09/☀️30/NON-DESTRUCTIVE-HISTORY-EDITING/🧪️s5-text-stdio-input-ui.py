#!/usr/bin/env python3
"""🎛️ S5-TEXT-STDIO: declares `x-semio-ui` (design §6 / §22.8, manifest `$defs/InputUi`) on every mutation input of the stdio,
writer, trinity and vcs leaves that carries none, from the reviewed table `🧪️s5-text-stdio-input-ui-table.py` (one hand-authored
row per input meaning: widget, role, en/de label and description, step/precision/unit/soft bounds/snaps, `ref`, `hidden` for an
opaque payload). Nothing is inferred here: an input without a row, a row without an input, a widget its value cannot take, an
annotation the `InputUi` meta-schema refuses (third-party `jsonschema`), an empty root or a file outside the explicit list all
fail closed. Edits are span-surgical: every byte of a file is kept and only the `x-semio-ui` member is inserted.

    python3 🧪️s5-text-stdio-input-ui.py                         # --check (default): what is pending, exit 1 while anything is
    python3 🧪️s5-text-stdio-input-ui.py --emit-files <list>     # the explicit file list the table reaches
    python3 🧪️s5-text-stdio-input-ui.py --emit-table <tsv>      # leaf, pointer, JSON type, what was missing, what is declared
    python3 🧪️s5-text-stdio-input-ui.py --emit-touch <list> [--only …]   # the files an apply of that scope would write
    python3 🧪️s5-text-stdio-input-ui.py --emit-withdraw <tsv>   # leaf descriptor → why it is withdraw-only (design §22.20)
    python3 🧪️s5-text-stdio-input-ui.py --emit-bundle <json>    # {file: {before, after}} for `🧪️s5-text-stdio-input-ui-check.ts`
    python3 🧪️s5-text-stdio-input-ui.py --apply --files <list> [--only <artifact>[,<artifact>…]] [--leave-blocked]
"""
from __future__ import annotations

import importlib.util
import json
import os
import re
import sys

import jsonschema

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 7))
ROOTS = ("✏️s/🔌️plugins/🗄️stdio", "✏️s/🔌️plugins/✒️writer", "✏️s/🔌️plugins/🔱️trinity", "✏️s/🔌️plugins/🌿️vcs")
SKIPPED = {"node_modules", "target", "dist", "🧫️fixtures", "🧪️tests", "🗑️generated"}
MANIFEST_SCHEMA = "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "optionSource", "options", "group", "order")


def table_module():
    spec = importlib.util.spec_from_file_location("s5_text_stdio_input_ui_table", os.path.join(HERE, "🧪️s5-text-stdio-input-ui-table.py"))
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


def ascii_name(name: str) -> str:
    """🔤️ A taxonomy directory name without its emoji prefix (`📸️set-snapshot` → `set-snapshot`)."""
    return re.sub(r"^[^A-Za-z0-9]+", "", name)


#region 🔖️Spans
class Span:
    def __init__(self, kind, start, end, members=None):
        self.kind, self.start, self.end, self.members = kind, start, end, members


def parse(text: str) -> Span:
    """🧭️ The byte spans of every JSON value of `text` (objects keep their members' key offsets)."""
    position = 0

    def skip():
        nonlocal position
        while position < len(text) and text[position] in " \t\r\n":
            position += 1

    def string():
        nonlocal position
        start = position
        position += 1
        while text[position] != '"':
            position += 2 if text[position] == "\\" else 1
        position += 1
        return json.loads(text[start:position]), start

    def value():
        nonlocal position
        skip()
        start = position
        head = text[position]
        if head in "{[":
            closing = "}" if head == "{" else "]"
            position += 1
            members = []
            skip()
            if text[position] == closing:
                position += 1
                return Span("object" if head == "{" else "array", start, position, members)
            while True:
                skip()
                key, key_start = (None, None)
                if head == "{":
                    key, key_start = string()
                    skip()
                    position += 1
                members.append((key, key_start, value()))
                skip()
                if text[position] == ",":
                    position += 1
                    continue
                position += 1
                return Span("object" if head == "{" else "array", start, position, members)
        if head == '"':
            string()
            return Span("scalar", start, position)
        while position < len(text) and text[position] not in ",}] \t\r\n":
            position += 1
        return Span("scalar", start, position)

    return value()


def locate(root: Span, path) -> Span:
    node = root
    for segment in path:
        node = node.members[int(segment)][2] if node.kind == "array" else next(child for key, _, child in node.members if key == segment)
    return node


def line_indent(text: str, offset: int) -> str:
    start = text.rfind("\n", 0, offset) + 1
    end = start
    while text[end] in " \t":
        end += 1
    return text[start:end]


def render(value, indent: str) -> str:
    return json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n" + indent)


def ordered(annotation: dict) -> dict:
    return {key: annotation[key] for key in KEY_ORDER if key in annotation}


def with_annotation(text: str, path, annotation: dict) -> str:
    """✍️ `text` with `"x-semio-ui": annotation` as the last member of the object at `path` (replacing the one it has), every
    other byte untouched."""
    node = locate(parse(text), path)
    existing = next((child for key, _, child in node.members if key == "x-semio-ui"), None)
    if existing is not None:
        return text[: existing.start] + render(ordered(annotation), line_indent(text, existing.start)) + text[existing.end :]
    inner = text[node.start : node.end]
    if "\n" in inner and node.members:
        last = node.members[-1]
        indent = line_indent(text, last[1])
        return text[: last[2].end] + ",\n" + indent + '"x-semio-ui": ' + render(ordered(annotation), indent) + text[last[2].end :]
    outer = line_indent(text, node.start)
    indent = outer + "  "
    lines = [indent + json.dumps(key, ensure_ascii=False) + ": " + text[child.start : child.end] for key, _, child in node.members]
    lines.append(indent + '"x-semio-ui": ' + render(ordered(annotation), indent))
    return text[: node.start] + "{\n" + ",\n".join(lines) + "\n" + outer + "}" + text[node.end :]
#endregion 🔖️Spans


#region 🔖️Leaves
def leaves(root: str) -> list[str]:
    """🔎️ Every mutation leaf payload schema under `root` (`…/🧬️mutations/<leaf…>/🧬️schema/🔣️.json`, outside fixtures and tests)."""
    found = []
    for directory, names, files in os.walk(os.path.join(REPO, root)):
        names[:] = [name for name in names if name not in SKIPPED and not name.startswith(".")]
        if os.path.basename(directory) == "🧬️schema" and "🔣️.json" in files and "/🧬️mutations/" in directory:
            if "/" in directory.split("/🧬️mutations/")[-1]:
                found.append(os.path.relpath(os.path.join(directory, "🔣️.json"), REPO))
    return sorted(found)


def identity(path: str) -> tuple[str, str, str]:
    """🪪️ `(artifact, subset, leaf)` of a leaf schema path, emoji prefixes dropped; a grouped leaf is `<group>/<leaf>`."""
    parts = path.split("/")
    artifact = ascii_name(parts[parts.index("🗿️artifacts") + 1])
    subset = ascii_name(parts[parts.index("🪆️subsets") + 1])
    leaf = "/".join(ascii_name(segment) for segment in path.split("/🧬️mutations/")[-1].rsplit("/🧬️schema/", 1)[0].split("/"))
    return artifact, subset, leaf


def settle(document: dict, node):
    """🧷️ `node` behind its local `$ref`s and its nullable union: `(schema, json path of that schema in the document, nullable)`;
    a cross-document `$ref` settles on itself (its shape is unknown here)."""
    path, nullable = None, False
    for _ in range(32):
        if not isinstance(node, dict):
            return {}, path, nullable
        reference = node.get("$ref")
        if isinstance(reference, str) and reference.startswith("#/"):
            path = [segment.replace("~1", "/").replace("~0", "~") for segment in reference[2:].split("/")]
            current = document
            for segment in path:
                current = current.get(segment, {}) if isinstance(current, dict) else {}
            node = current
            continue
        union = node.get("anyOf") or node.get("oneOf")
        if isinstance(union, list):
            concrete = [branch for branch in union if branch != {"type": "null"}]
            if len(concrete) == 1 and len(concrete) < len(union):
                node, nullable, path = concrete[0], True, None
                continue
        return node, path, nullable
    raise SystemExit("a $ref chain exceeds 32 hops")


def shape(document: dict, node) -> str:
    """🧬️ The value family a widget must fit: string, enum, integer, number, boolean, vector, numbers, ids, array, object, const, foreign."""
    settled, _, _ = settle(document, node)
    if isinstance(settled.get("$ref"), str):
        return "number" if settled["$ref"].endswith("framework/value/schema.json#/$defs/Binary64Transport") else "foreign"
    if "const" in settled:
        return "const"
    kind = settled.get("type")
    if isinstance(kind, list):
        kind = next((name for name in kind if name != "null"), None)
    if kind is None and "properties" in settled:
        kind = "object"
    if kind == "string" or (kind is None and isinstance(settled.get("enum"), list)):
        return "enum" if isinstance(settled.get("enum"), list) else "string"
    if kind in ("integer", "number", "boolean", "object"):
        return kind
    if kind == "array":
        items = settled.get("items")
        if isinstance(items, list):
            return "array"
        item = shape(document, items) if isinstance(items, dict) else "foreign"
        if item in ("integer", "number"):
            fixed = settled.get("minItems")
            return "vector" if isinstance(fixed, int) and fixed == settled.get("maxItems") and 2 <= fixed <= 4 else "numbers"
        return "ids" if item == "string" else "array"
    numeric = settled.get("anyOf") or settled.get("oneOf")
    if isinstance(numeric, list) and len(numeric) == 2 and any(branch.get("type") == "number" for branch in numeric if isinstance(branch, dict)) and any("bits" in json.dumps(branch) for branch in numeric):
        return "number"
    return "union" if numeric else "any"


FOREIGN: dict[str, dict[str, dict]] = {}


def foreign(file: str, document: dict, node) -> tuple[dict, dict] | None:
    """🌍️ The document and schema a cross-document `$ref` of `node` lands on, looked up by `$id` among the schema documents of
    the leaf's own artifact; `None` when it stays unknown."""
    settled, _, _ = settle(document, node)
    reference = settled.get("$ref")
    if not isinstance(reference, str) or "🗿️artifacts/" not in file:
        return None
    root = file[: file.index("/", file.index("🗿️artifacts/") + len("🗿️artifacts/"))]
    if root not in FOREIGN:
        FOREIGN[root] = {}
        for directory, names, files in os.walk(os.path.join(REPO, root)):
            names[:] = [name for name in names if name not in SKIPPED and not name.startswith(".")]
            for name in files:
                if name.endswith(".json"):
                    try:
                        loaded = json.load(open(os.path.join(directory, name), encoding="utf-8"))
                    except (ValueError, OSError):
                        continue
                    if isinstance(loaded, dict) and isinstance(loaded.get("$id"), str):
                        FOREIGN[root].setdefault(loaded["$id"], loaded)
    identifier, _, fragment = reference.partition("#")
    target = FOREIGN[root].get(identifier)
    if target is None:
        return None
    current = target
    for segment in [part for part in fragment.split("/")[1:]]:
        current = current.get(segment) if isinstance(current, dict) else None
    return (target, current) if isinstance(current, dict) else None


def bounds(document: dict, node) -> tuple[float | None, float | None]:
    settled, _, _ = settle(document, node)
    return settled.get("minimum", settled.get("exclusiveMinimum")), settled.get("maximum", settled.get("exclusiveMaximum"))
#endregion 🔖️Leaves


#region 🔖️Plan
WIDGET_SHAPES = {
    "stepper": {"integer", "number"},
    "slider": {"integer", "number"},
    "dial": {"integer", "number"},
    "toggle": {"boolean"},
    "text": {"string"},
    "multiline": {"string"},
    "select": {"enum"},
    "segmented": {"enum"},
    "vector": {"vector"},
    "reference": {"string", "integer", "ids", "numbers"},
    "hidden": {"string", "enum", "integer", "number", "boolean", "vector", "numbers", "ids", "array", "object", "union", "any", "foreign"},
    None: {"object", "array", "union", "any", "foreign", "numbers", "ids", "const", "vector"},
}


def faults(file: str, document: dict, node, annotation: dict, validator) -> list[str]:
    """🚦️ Why `annotation` may not sit on `node`: the meta-schema's refusals and the reader's widget/number rules."""
    found = [error.message for error in validator.iter_errors(annotation)]
    kind = shape(document, node)
    widget = annotation.get("widget")
    if kind == "foreign" and widget not in (None, "hidden"):
        landed = foreign(file, document, node)
        if landed is not None:
            document, node = landed
            kind = shape(document, node)
    if annotation.get("role") == "discriminator":
        return found
    if kind not in WIDGET_SHAPES[widget]:
        found.append(f"widget {widget} cannot edit a {kind}")
    if annotation.get("role") == "target" and kind not in WIDGET_SHAPES["reference"]:
        found.append(f"a target is a string or integer id, not a {kind}")
    if (annotation.get("role") == "target" or widget == "reference") and "kind" not in annotation.get("ref", {}):
        found.append("a reference names its ref.kind")
    numeric = kind in ("integer", "number", "vector", "numbers")
    for key in ("step", "precision", "softMin", "softMax", "snaps", "snapSource", "scale", "displayUnit", "displayFactor", "unit"):
        if key in annotation and (not numeric or widget in ("reference", "hidden")):
            found.append(f"{key} only applies to a number")
    if widget in ("stepper", "slider", "dial") and "step" not in annotation and "snaps" not in annotation and "snapSource" not in annotation:
        found.append("an interactive number declares its step or its snaps (design §22.8)")
    if kind == "integer" and "step" in annotation and annotation["step"] != int(annotation["step"]):
        found.append("an integer steps by integers")
    low, high = bounds(document, node)
    inside = lambda value: (low is None or value >= low) and (high is None or value <= high)
    for key in ("softMin", "softMax"):
        if key in annotation and not inside(annotation[key]):
            found.append(f"{key} lies outside the hard bounds")
    if any(not inside(snap) for snap in annotation.get("snaps", [])):
        found.append("a snap lies outside the hard bounds")
    if widget in ("slider", "dial") and (annotation.get("softMin", low) is None or annotation.get("softMax", high) is None):
        found.append(f"a {widget} needs both ends (hard or soft bounds)")
    return found


def plan(table) -> tuple[list[dict], list[str], list[str], list[tuple[str, str]]]:
    """🗺️ Every declaration the table makes — `{file, pointer, path, shape, before, annotation, state}` — and every reason the
    table and the tree disagree."""
    validator = jsonschema.Draft7Validator(dict(json.load(open(os.path.join(REPO, MANIFEST_SCHEMA), encoding="utf-8"))["$defs"]["InputUi"], **{"$defs": json.load(open(os.path.join(REPO, MANIFEST_SCHEMA), encoding="utf-8"))["$defs"]}))
    rows, problems, used, kept, withdraw, reached = [], [], set(), [], [], set()
    for root in ROOTS:
        files = leaves(root)
        if not files:
            raise SystemExit(f"refusing to run: {root} holds no mutation leaf (empty or moved root)")
        for file in files:
            artifact, subset, leaf = identity(file)
            withdrawn = next((key for key in [(artifact, subset, leaf), (artifact, "*", leaf), ("*", "*", leaf)] if key in table.WITHDRAW), None)
            document = json.load(open(os.path.join(REPO, file), encoding="utf-8"))
            properties = document.get("properties", {})
            parameterless = all(isinstance(node, dict) and "const" in node for node in properties.values())
            if withdrawn is not None or parameterless:
                reached.update([withdrawn] if withdrawn is not None else [])
                why = table.WITHDRAW[withdrawn] if withdrawn is not None else "nothing: the leaf takes no parameter"
                descriptor = os.path.join(os.path.dirname(os.path.dirname(file)), "🔣️.json")
                source = os.path.join(os.path.dirname(os.path.dirname(file)), "🦀️.rs")
                if not os.path.exists(os.path.join(REPO, descriptor)) or not os.path.exists(os.path.join(REPO, source)):
                    problems.append(f"withdraw-only leaf without descriptor or Rust leaf: {artifact} {subset} {leaf} — {file}")
                    continue
                attributes = re.findall(r"#\[mutation_leaf\(([^\]]*)\)\]", open(os.path.join(REPO, source), encoding="utf-8").read())
                marked = json.load(open(os.path.join(REPO, descriptor), encoding="utf-8")).get("editable")
                blocked = marked is not False and any(re.search(r"\b(payload|input_schema)\s*=", attribute) for attribute in attributes)
                withdraw.append({"file": descriptor, "artifact": artifact, "subset": subset, "leaf": leaf, "why": why, "source": source, "state": "declared" if marked is False else ("blocked" if blocked else "pending")})
                continue
            groups, ungrouped_enums = set(), []
            for position, (key, node) in enumerate((key, node) for key, node in properties.items() if key != "mutation"):
                candidates = [(artifact, subset, leaf, key), (artifact, "*", leaf, key), (artifact, subset, "*", key), (artifact, "*", "*", key), ("*", "*", leaf, key)]
                current = node.get("x-semio-ui") if isinstance(node, dict) else None
                match = next((candidate for candidate in candidates if candidate in table.ROWS and (current is not None or not table.ROWS[candidate]["amend"])), None)
                where = f"{artifact} {subset} {leaf} /{key}"
                if match is None or (current is not None and table.ROWS[match]["ui"] is not None and not table.ROWS[match]["force"] and not table.ROWS[match]["drop"] and not table.ROWS[match]["amend"] and ordered(current) != ordered(declared(table.ROWS[match]["ui"], position))):
                    if match is not None:
                        used.add(match)
                        kept.append(where)
                    elif current is None:
                        problems.append(f"no row: {where} ({shape(document, node)}) — {file}")
                    if current is not None:
                        groups.update([current["group"]] if "group" in current else [])
                        if "group" not in current and isinstance(settle(document, node)[0].get("enum"), list):
                            ungrouped_enums.append((key, set(settle(document, node)[0]["enum"])))
                    continue
                used.add(match)
                entry = table.ROWS[match]
                if entry["ui"] is not None:
                    annotation = declared(entry["ui"], position)
                    if current is not None:
                        annotation = {**{name: value for name, value in current.items() if name not in entry["drop"]}, **{name: value for name, value in annotation.items() if name not in current or name in entry["force"]}}
                    annotation = ordered(annotation)
                    groups.update([annotation["group"]] if "group" in annotation else [])
                    found = faults(file, document, node, annotation, validator)
                    if found:
                        problems.append(f"refused: {where}: {'; '.join(found)} — {file}")
                        continue
                    state = "declared" if current is not None and ordered(current) == annotation else ("revise" if current is not None else "pending")
                    rows.append({"file": file, "artifact": artifact, "subset": subset, "leaf": leaf, "pointer": f"/{key}", "path": ["properties", key], "shape": shape(document, node), "before": current, "annotation": annotation, "state": state})
                _, local, _ = settle(document, node)
                base = local if isinstance(node, dict) and isinstance(node.get("$ref"), str) and local is not None else ["properties", key]
                for nested, nested_ui in entry["nested"].items():
                    owner = base + nested.split("/")
                    child = document
                    for segment in owner:
                        child = child[int(segment)] if isinstance(child, list) and segment.isdigit() and int(segment) < len(child) else (child.get(segment) if isinstance(child, dict) else None)
                    if not isinstance(child, dict):
                        problems.append(f"nested member not in this document: {where}/{nested} — {file}")
                        continue
                    present = child.get("x-semio-ui")
                    merged = ordered({**(present or {}), **nested_ui})
                    refused = faults(file, document, child, merged, validator)
                    if refused:
                        problems.append(f"refused: {where}/{nested}: {'; '.join(refused)} — {file}")
                        continue
                    pointer = f"/{key}/" + "/".join(segment for segment in nested.split("/") if segment not in ("properties", "anyOf", "oneOf") and not segment.isdigit()).replace("items", "-")
                    rows.append({"file": file, "artifact": artifact, "subset": subset, "leaf": leaf, "pointer": pointer, "path": owner, "shape": shape(document, child), "before": present, "annotation": merged, "state": "declared" if present is not None and ordered(present) == merged else ("revise" if present is not None else "pending")})
            for key, values in ungrouped_enums:
                if groups and groups <= values:
                    problems.append(f"layout groups {sorted(groups)} would read as the variants of /{key}: {file}")
    for key in sorted(set(table.ROWS) - used):
        problems.append(f"row reaches no input: {' '.join(key)}")
    for key in sorted(set(table.WITHDRAW) - reached):
        problems.append(f"withdraw-only row reaches no leaf: {' '.join(key)}")
    for (file, pointer), ui in table.DOCS.items():
        if not any(file.startswith(root + "/") for root in ROOTS) or not os.path.exists(os.path.join(REPO, file)):
            problems.append(f"document row outside the owned roots or missing: {file}")
            continue
        document = json.load(open(os.path.join(REPO, file), encoding="utf-8"))
        owner = pointer.split("/")
        child = document
        for segment in owner:
            child = child[int(segment)] if isinstance(child, list) and segment.isdigit() and int(segment) < len(child) else (child.get(segment) if isinstance(child, dict) else None)
        if not isinstance(child, dict):
            problems.append(f"document row reaches nothing: {file} {pointer}")
            continue
        present = child.get("x-semio-ui")
        merged = ordered({**(present or {}), **ui})
        refused = faults(file, document, child, merged, validator)
        if refused:
            problems.append(f"refused: {file} {pointer}: {'; '.join(refused)}")
            continue
        artifact = ascii_name(file.split("🗿️artifacts/")[1].split("/")[0])
        rows.append({"file": file, "artifact": artifact, "subset": "(document)", "leaf": "(shared schema)", "pointer": "/" + pointer, "path": owner, "shape": shape(document, child), "before": present, "annotation": merged, "state": "declared" if present is not None and ordered(present) == merged else ("revise" if present is not None else "pending")})
    reached_members = set()
    for root in ROOTS:
        for directory, names, files in os.walk(os.path.join(REPO, root)):
            names[:] = [name for name in names if name not in SKIPPED and name not in ("🔮️oracles", "📚️examples") and not name.startswith(".")]
            for name in files:
                if not name.endswith(".json") or "🗿️artifacts/" not in directory:
                    continue
                file = os.path.relpath(os.path.join(directory, name), REPO)
                artifact = ascii_name(file.split("🗿️artifacts/")[1].split("/")[0])
                if not any(key[0] == artifact for key in table.MEMBERS):
                    continue
                try:
                    document = json.load(open(os.path.join(REPO, file), encoding="utf-8"))
                except ValueError:
                    continue
                if not isinstance(document, dict) or "$schema" not in document:
                    continue
                for keyword in ("$defs", "definitions"):
                    for record, definition in (document.get(keyword) or {}).items():
                        if not isinstance(definition, dict):
                            continue
                        owners = [([keyword, record], definition)] + [([keyword, record, union, str(index)], branch) for union in ("oneOf", "anyOf", "allOf") for index, branch in enumerate(definition.get(union) or []) if isinstance(branch, dict)]
                        for base, owner in owners:
                            for member, child in (owner.get("properties") or {}).items():
                                entry = table.MEMBERS.get((artifact, record, member))
                                if entry is None or not isinstance(child, dict):
                                    continue
                                reached_members.add((artifact, record, member))
                                present = child.get("x-semio-ui")
                                merged = ordered({**(present or {}), **{key: value for key, value in entry["ui"].items() if key not in (present or {}) or key in entry["force"]}})
                                refused = faults(file, document, child, merged, validator)
                                if refused:
                                    problems.append(f"refused: {artifact} {record}.{member}: {'; '.join(refused)} — {file}")
                                    continue
                                rows.append({"file": file, "artifact": artifact, "subset": "(record)", "leaf": record, "pointer": "." + member, "path": base + ["properties", member], "shape": shape(document, child), "before": present, "annotation": merged, "state": "declared" if present is not None and ordered(present) == merged else ("revise" if present is not None else "pending")})
    for key in sorted(set(table.MEMBERS) - reached_members):
        problems.append(f"member row reaches no record member: {' '.join(key)}")
    return rows, problems, kept, withdraw


def declared(ui: dict, position: int) -> dict:
    """🧾️ A row's declaration at its place in the leaf: laid-out inputs take their `order` from the property order."""
    return {**ui, "order": 10 * (position + 1)} if "group" in ui and "order" not in ui else dict(ui)
#endregion 🔖️Plan


#region 🔖️Run
def missing(row: dict) -> str:
    """🕳️ What an input lacked before its declaration."""
    before = row["before"] or {}
    lacks = [name for name in ("label", "description", "widget", "role", "ref", "step", "group") if name in row["annotation"] and name not in before]
    return "everything (no x-semio-ui)" if row["before"] is None else ", ".join(lacks) or "nothing"


def main() -> int:
    arguments = sys.argv[1:]
    option = lambda name: arguments[arguments.index(name) + 1] if name in arguments else None
    table = table_module()
    rows, problems, kept, withdraw = plan(table)
    if not rows:
        raise SystemExit("refusing to run: the table reaches no input")
    for problem in problems:
        print(problem)
    counts = {state: sum(1 for row in rows if row["state"] == state) for state in ("pending", "revise", "declared")}
    files = sorted({row["file"] for row in rows} | {leaf["file"] for leaf in withdraw})
    if kept:
        print(f"{len(kept)} input(s) keep a declaration the table did not write (not revised): " + ", ".join(kept[:6]) + (" …" if len(kept) > 6 else ""))
    print(f"{len(table.ROWS)} table row(s) reach {len(rows)} input(s) in {len(files)} leaf file(s): {counts['pending']} to declare, {counts['revise']} to revise, {counts['declared']} already declared; {len(problems)} problem(s)")
    marked = sum(1 for leaf in withdraw if leaf["state"] == "declared")
    for leaf in withdraw:
        if leaf["state"] == "blocked":
            print(f"blocked: {leaf['artifact']} {leaf['subset']} {leaf['leaf']} still declares an editable payload in Rust — drop `payload =` / `input_schema =` from its mutation_leaf attribute in the same wave ({leaf['source']})")
    print(f"{len(withdraw)} leaf file(s) are withdraw-only (design §22.20, descriptor `editable: false`; no input of theirs is declared): {len(withdraw) - marked} to mark, {marked} already marked")
    if option("--emit-withdraw"):
        open(option("--emit-withdraw"), "w", encoding="utf-8").write("".join(f"{leaf['artifact']}\t{leaf['subset']}\t{leaf['leaf']}\t{leaf['why']}\t{leaf['state']}\t{leaf['file']}\n" for leaf in withdraw))
    if option("--emit-files"):
        open(option("--emit-files"), "w", encoding="utf-8").write("".join(file + "\n" for file in files))
    if option("--emit-table"):
        with open(option("--emit-table"), "w", encoding="utf-8") as out:
            out.write("artifact\tsubset\tleaf\tpointer\tjson type\tmissing before\twidget\trole\tlabel en\tlabel de\tfacets\n")
            for row in rows:
                ui = row["annotation"]
                facets = ", ".join(f"{name}={json.dumps(ui[name], ensure_ascii=False)}" for name in ("ref", "unit", "step", "precision", "softMin", "softMax", "snaps", "snapSource", "scale") if name in ui)
                out.write("\t".join([row["artifact"], row["subset"], row["leaf"], row["pointer"], row["shape"], missing(row), str(ui.get("widget", "—")), str(ui.get("role", "—")), ui.get("label", {}).get("en", "—"), ui.get("label", {}).get("de", "—"), facets]) + "\n")
    if option("--emit-touch"):
        scope = set((option("--only") or "").split(",")) - {""}
        touched = {row["file"] for row in rows if row["state"] != "declared" and (not scope or row["artifact"] in scope)} | {leaf["file"] for leaf in withdraw if leaf["state"] == "pending" and (not scope or leaf["artifact"] in scope)}
        open(option("--emit-touch"), "w", encoding="utf-8").write("".join(file + "\n" for file in sorted(touched)))
    if option("--emit-bundle"):
        bundle = {}
        for file in files:
            before = open(os.path.join(REPO, file), encoding="utf-8").read()
            after = before
            for row in rows:
                if row["file"] == file and row["state"] != "declared":
                    after = with_annotation(after, row["path"], row["annotation"])
            bundle[file] = {"before": before, "after": after}
        json.dump(bundle, open(option("--emit-bundle"), "w", encoding="utf-8"), ensure_ascii=False)
    if "--apply" not in arguments:
        return 1 if problems or counts["pending"] or counts["revise"] or len(withdraw) > marked else 0
    if problems:
        raise SystemExit("refusing to apply: the table and the tree disagree (see above)")
    listed = option("--files")
    if listed is None:
        raise SystemExit("refusing to apply without --files <explicit list>")
    allowed = {line.strip() for line in open(listed, encoding="utf-8") if line.strip()}
    if not allowed or any(not any(file.startswith(root + "/") for root in ROOTS) for file in allowed):
        raise SystemExit("refusing to apply: the file list is empty or names a path outside the owned roots")
    only = set((option("--only") or "").split(",")) - {""}
    written = 0
    for leaf in withdraw:
        if leaf["state"] == "declared" or (only and leaf["artifact"] not in only):
            continue
        if leaf["state"] == "blocked":
            if "--leave-blocked" in arguments:
                continue
            raise SystemExit(f"refusing to apply: {leaf['artifact']} {leaf['leaf']} is blocked (see above); --leave-blocked lands the rest of its artifact")
        if leaf["file"] not in allowed:
            raise SystemExit(f"refusing to apply: {leaf['file']} is not in the explicit list")
        path = os.path.join(REPO, leaf["file"])
        original = open(path, encoding="utf-8").read()
        root = parse(original)
        last = root.members[-1]
        text = original[: last[2].end] + ",\n" + line_indent(original, last[1]) + '"editable": false' + original[last[2].end :]
        if json.loads(text) != {**json.loads(original), "editable": False}:
            raise SystemExit(f"refusing to write {leaf['file']}: the edit changes more than its editable marker")
        open(path, "w", encoding="utf-8").write(text)
        written += 1
    for file in files:
        changes = [row for row in rows if row["file"] == file and row["state"] != "declared" and (not only or row["artifact"] in only)]
        if not changes:
            continue
        if file not in allowed:
            raise SystemExit(f"refusing to apply: {file} is not in the explicit list")
        path = os.path.join(REPO, file)
        original = open(path, encoding="utf-8").read()
        text = original
        expected = json.loads(original)
        for row in changes:
            text = with_annotation(text, row["path"], row["annotation"])
            node = expected
            for segment in row["path"]:
                node = node[int(segment)] if isinstance(node, list) else node[segment]
            node["x-semio-ui"] = row["annotation"]
        if json.loads(text) != expected:
            raise SystemExit(f"refusing to write {file}: the edit changes more than its x-semio-ui members")
        open(path, "w", encoding="utf-8").write(text)
        written += 1
    print(f"wrote {written} file(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
#endregion 🔖️Run
