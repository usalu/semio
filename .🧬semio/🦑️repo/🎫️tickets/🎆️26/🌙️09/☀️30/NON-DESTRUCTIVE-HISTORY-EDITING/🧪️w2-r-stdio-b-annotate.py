#!/usr/bin/env python3
"""🏷️ W2-R stdio-b: writes `x-semio-ui` (design §6, manifest `$defs/InputUi`) onto every in-scope stdio mutation input the
walker (`🧪️w2-r-stdio-b-walk.py`) reports as unlabelled or with unlabelled options, from the hand-written tables in
`🧪️w2-r-stdio-b-labels.py`. Edits are span-surgical: the original text of every file is kept byte for byte and only the
`x-semio-ui` member is inserted (as the last member of its property object, in the object's own layout); files that were
Prettier-clean are re-run through Prettier.

    python3 🧪️w2-r-stdio-b-annotate.py [--dry-run] [--revise <backup.json>]
"""
import importlib.util
import json
import os
import subprocess
import sys

GAPS = []
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, name + ".py"))
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


walk = module("🧪️w2-r-stdio-b-walk")
tables = module("🧪️w2-r-stdio-b-labels")


class Span:
    def __init__(self, kind, start, end, members=None):
        self.kind, self.start, self.end, self.members = kind, start, end, members


def parse(text):
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
        if head == "{":
            position += 1
            members = []
            skip()
            if text[position] == "}":
                position += 1
                return Span("object", start, position, members)
            while True:
                skip()
                key, key_start = string()
                skip()
                position += 1
                child = value()
                members.append((key, key_start, child))
                skip()
                if text[position] == ",":
                    position += 1
                    continue
                position += 1
                return Span("object", start, position, members)
        if head == "[":
            position += 1
            members = []
            skip()
            if text[position] == "]":
                position += 1
                return Span("array", start, position, members)
            while True:
                members.append((None, None, value()))
                skip()
                if text[position] == ",":
                    position += 1
                    continue
                position += 1
                return Span("array", start, position, members)
        if head == '"':
            string()
            return Span("scalar", start, position)
        while position < len(text) and text[position] not in ",}] \t\r\n":
            position += 1
        return Span("scalar", start, position)

    return value()


def locate(root, path):
    node = root
    for segment in path:
        if node.kind == "object":
            node = next(child for key, _, child in node.members if key == segment)
        else:
            node = node.members[int(segment)][2]
    return node


def line_indent(text, offset):
    start = text.rfind("\n", 0, offset) + 1
    end = start
    while text[end] in " \t":
        end += 1
    return text[start:end]


def render(value, indent):
    return json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n" + indent)


def insert_member(text, path, annotation, replace=False):
    """✍️ Returns `text` with `"x-semio-ui": annotation` merged into (or, with `replace`, replacing) the one on the object at
    `path`, all other bytes untouched."""
    node = locate(parse(text), path)
    existing = next((child for key, _, child in node.members if key == "x-semio-ui"), None)
    if existing is not None:
        merged = {} if replace else json.loads(text[existing.start:existing.end])
        merged.update({key: value for key, value in annotation.items() if key not in merged})
        merged = {key: merged[key] for key in KEY_ORDER if key in merged}
        return text[: existing.start] + render(merged, line_indent(text, existing.start)) + text[existing.end :]
    inner = text[node.start : node.end]
    if "\n" in inner and node.members:
        last = node.members[-1]
        indent = line_indent(text, last[1])
        return text[: last[2].end] + ",\n" + indent + '"x-semio-ui": ' + render(annotation, indent) + text[last[2].end :]
    outer = line_indent(text, node.start)
    indent = outer + "  "
    lines = [indent + json.dumps(key, ensure_ascii=False) + ": " + text[child.start : child.end] for key, _, child in node.members]
    lines.append(indent + '"x-semio-ui": ' + render(annotation, indent))
    return text[: node.start] + "{\n" + ",\n".join(lines) + "\n" + outer + "}" + text[node.end :]


def number_bounds(shape):
    low = "minimum" in shape or "exclusiveMinimum" in shape
    high = "maximum" in shape or "exclusiveMaximum" in shape
    return low and high


def widget_for(shape, spec):
    if "widget" in spec:
        return spec["widget"]
    if spec.get("ref"):
        return "reference"
    kind = shape.get("type")
    if kind == "boolean":
        return "toggle"
    if kind == "string":
        if isinstance(shape.get("enum"), list):
            return "segmented" if len(shape["enum"]) <= 3 else "select"
        return "multiline" if spec.get("multiline") else "text"
    if kind == "integer":
        return "stepper"
    if kind == "number":
        return "slider" if number_bounds(shape) else "stepper"
    if kind == "array" and shape.get("itemType") in ("integer", "number") and shape.get("minItems") == shape.get("maxItems") and isinstance(shape.get("minItems"), int) and 2 <= shape["minItems"] <= 4:
        return "vector"
    return None


def localized(pair):
    return {"en": pair[0], "de": pair[1]}


def option_labels(entry, values):
    labels = {}
    for value in values:
        pair = tables.option(entry["artifact"], entry["parent"], entry["key"], value, entry.get("subset"))
        if pair is None:
            GAPS.append("no option label for %s %s:%s.%s = %r (%s)" % (entry["artifact"], entry.get("subset"), entry["parent"], entry["key"], value, entry["leaf"]))
            continue
        labels[value] = localized(pair)
    return labels


def annotation_for(entries):
    first = entries[0]
    ui = {}
    missing = sorted({value for entry in entries if entry["problem"] == "optionLabelMissing" for value in entry["missing"]}, key=lambda value: (first["shape"].get("enum") or []).index(value) if value in (first["shape"].get("enum") or []) else 0)
    if any(entry["problem"] == "labelMissing" for entry in entries):
        spec = tables.spec(first["artifact"], first["parent"], first["key"], first.get("subset"))
        if spec is None:
            GAPS.append("no label for %s %s:%s.%s (%s)" % (first["artifact"], first.get("subset"), first["parent"], first["key"], first["leaf"]))
            return {}
        widget = widget_for(first["shape"], spec)
        if widget is not None:
            ui["widget"] = widget
        if spec.get("ref"):
            ui["role"] = "target"
        ui["label"] = localized(spec["label"])
        if spec.get("description"):
            ui["description"] = localized(spec["description"])
        if spec.get("ref"):
            ui["ref"] = {"kind": spec["ref"]}
        for key in ("unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps"):
            if key in spec:
                ui[key] = spec[key]
    if missing:
        ui["options"] = option_labels(first, missing)
    return {key: ui[key] for key in KEY_ORDER if key in ui}


def node_at(document, path):
    for segment in path:
        document = document[int(segment)] if isinstance(document, list) else document[segment]
    return document


def revisions(backup):
    """🔁️ Every site this script annotated (no `x-semio-ui` in the pre-edit `backup` bundle {path: text}, one now), re-derived
    from the tables."""
    glossary = walk.load(walk.GLOSSARY)["labels"]
    bundle = json.load(open(backup, encoding="utf-8"))
    sites, documents = {}, {}
    for entry in walk.findings(everything=True):
        if "site" not in entry or entry["problem"] not in ("input", "labelMissing"):
            continue
        file, path = entry["site"]["file"], entry["site"]["path"]
        if file not in bundle:
            continue
        if file not in documents:
            documents[file] = (json.loads(bundle[file]), walk.load(file))
        before, after = documents[file]
        if "x-semio-ui" in node_at(before, path) or "x-semio-ui" not in node_at(after, path):
            continue
        values = [value for value in entry["shape"].get("enum") or [] if isinstance(value, str) and value not in glossary]
        synthetic = [dict(entry, problem="labelMissing")] + ([dict(entry, problem="optionLabelMissing", missing=values)] if values else [])
        if "label" not in node_at(after, path)["x-semio-ui"]:
            synthetic = synthetic[1:]
        sites.setdefault((file, json.dumps(path)), synthetic)
    return sites


def main():
    dry = "--dry-run" in sys.argv
    revise = sys.argv[sys.argv.index("--revise") + 1] if "--revise" in sys.argv else None
    sites = {}
    if revise is not None:
        sites = revisions(revise)
    else:
        for entry in walk.findings():
            if "site" in entry and entry["problem"] in ("labelMissing", "optionLabelMissing"):
                sites.setdefault((entry["site"]["file"], json.dumps(entry["site"]["path"])), []).append(entry)
    edits = {}
    for (file, path), entries in sorted(sites.items()):
        edits.setdefault(file, []).append((json.loads(path), annotation_for(entries)))
    if GAPS:
        print("\n".join(sorted(set(GAPS))))
        raise SystemExit("%d table gap(s)" % len(set(GAPS)))
    count = 0
    for file, changes in sorted(edits.items()):
        absolute = os.path.join(REPO, file)
        text = open(absolute, encoding="utf-8").read()
        dumps_clean = json.dumps(json.loads(text), indent=2, ensure_ascii=False) + "\n" == text
        prettier_clean = not dumps_clean and subprocess.run([os.path.join(REPO, "node_modules/.bin/prettier"), "--check", absolute], cwd=REPO, capture_output=True).returncode == 0
        for path, annotation in changes:
            text = insert_member(text, path, annotation, replace=revise is not None)
            count += 1
        json.loads(text)
        if dry:
            continue
        with open(absolute, "w", encoding="utf-8") as handle:
            handle.write(text)
        if prettier_clean and not dumps_clean:
            subprocess.run([os.path.join(REPO, "node_modules/.bin/prettier"), "--write", absolute], cwd=REPO, check=True, capture_output=True)
        if dumps_clean and json.dumps(json.loads(open(absolute, encoding="utf-8").read()), indent=2, ensure_ascii=False) + "\n" != open(absolute, encoding="utf-8").read():
            raise SystemExit("layout drift in " + file)
    print("%s %d annotation(s) in %d file(s)" % ("would write" if dry else "wrote", count, len(edits)))


if __name__ == "__main__":
    main()
