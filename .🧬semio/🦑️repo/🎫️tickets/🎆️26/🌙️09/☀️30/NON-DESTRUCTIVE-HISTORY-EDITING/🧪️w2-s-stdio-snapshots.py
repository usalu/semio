#!/usr/bin/env python3
"""📸️ W2-S stdio: every stdio `set-snapshot` leaf references the one catalogued snapshot contract its Rust payload is
(`XmlSnapshot`, `SvgSnapshot`, `PdfSnapshot`, `ZipSnapshot`) instead of a restated, opaque or stale copy.

- xml / svg / pdf 1.7 `artifact.json` become the zip form: `allOf` of the base `snapshot.json`, referenced not restated; the
  `x-semio-ui` annotations they carried move onto the snapshot contract's own members;
- svg `doc` is the xml `XmlDocument` it is in Rust;
- the xml / pdf 1.7 `set-snapshot` leaves reference `snapshot.json`; the zip base / iso21320 leaves drop their inlined
  `ZipSnapshot` copy (no `metadata`, no `commentUtf8`) for the base `snapshot.json`, whose members take the copy's labels.

Edits are span-surgical (only the touched member changes) unless a file is a canonical `json.dumps(indent=2)` document.
Idempotent; each file is re-read right before it is written.

    python3 🧪️w2-s-stdio-snapshots.py [--dry-run]
"""
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
ART = REPO + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
IDS = "https://json.schemas.assets.semio-tech.com/s/stdio/"
DRY = "--dry-run" in sys.argv
XML = ART + "/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema"
SVG = ART + "/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema"
PDF = ART + "/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema"
ZIP = ART + "/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets"
SCHEMA_UI = {"widget": "text", "label": {"en": "Snapshot schema", "de": "Snapshot-Schema"}, "description": {"en": "Identifier of the snapshot schema version.", "de": "Kennung der Version des Snapshot-Schemas."}}


class Span:
    def __init__(self, kind, start, end, members=None):
        self.kind, self.start, self.end, self.members = kind, start, end, members


def parse(text):
    """🔎️ Byte spans of every JSON value of `text` (objects keep `(key, key_start, child)` members)."""
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
        start, head = position, text[position]
        if head in "{[":
            position += 1
            members = []
            skip()
            if text[position] in "}]":
                position += 1
                return Span("object" if head == "{" else "array", start, position, members)
            while True:
                skip()
                if head == "{":
                    key, key_start = string()
                    skip()
                    position += 1
                    members.append((key, key_start, value()))
                else:
                    members.append((None, None, value()))
                skip()
                position += 1
                if text[position - 1] == ",":
                    continue
                return Span("object" if head == "{" else "array", start, position, members)
        if head == '"':
            string()
            return Span("scalar", start, position)
        while position < len(text) and text[position] not in ",}] \t\r\n":
            position += 1
        return Span("scalar", start, position)

    return value()


def locate(text, path):
    node = parse(text)
    for segment in path:
        node = next(child for key, _, child in node.members if key == segment) if node.kind == "object" else node.members[int(segment)][2]
    return node


def indent_at(text, offset):
    start = text.rfind("\n", 0, offset) + 1
    end = start
    while text[end] in " \t":
        end += 1
    return text[start:end]


def add_member(text, path, key, value):
    """✍️ `text` with `"key": value` appended to the object at `path` in that object's own layout (no-op when present)."""
    node = locate(text, path)
    if any(name == key for name, _, _ in node.members):
        return text
    inner = text[node.start:node.end]
    if "\n" in inner and node.members:
        last = node.members[-1]
        indent = indent_at(text, last[1])
        rendered = json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n" + indent)
        return text[: last[2].end] + ",\n" + indent + json.dumps(key) + ": " + rendered + text[last[2].end:]
    rendered = json.dumps(value, ensure_ascii=False, separators=(", ", ": "))
    return text[: node.end - 1].rstrip() + (", " if node.members else " ") + json.dumps(key) + ": " + rendered + " " + text[node.end - 1:]


def replace_value(text, path, value):
    """🔁️ `text` with the value at `path` replaced by `value`, rendered in its surrounding layout."""
    node = locate(text, path)
    multiline = "\n" in text[node.start:node.end]
    rendered = json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n" + indent_at(text, node.start)) if multiline else json.dumps(value, ensure_ascii=False, separators=(", ", ": "))
    return text[: node.start] + rendered + text[node.end:]


def remove_member(text, path, key):
    """✂️ `text` without the member `key` of the object at `path` (and its separating comma)."""
    node = locate(text, path)
    index = next((index for index, (name, _, _) in enumerate(node.members) if name == key), None)
    if index is None:
        return text
    _, key_start, child = node.members[index]
    if index > 0:
        previous = node.members[index - 1][2]
        return text[: previous.end] + text[child.end:]
    following = node.members[1][1] if len(node.members) > 1 else node.end - 1
    return text[:key_start] + text[following:]


def referenced(title, what, snapshot_id):
    """🔗️ An `artifact.json` in the zip form: the snapshot contract, referenced rather than restated."""
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": snapshot_id.replace("snapshot.json", "artifact.json"), "title": title, "description": "The complete persisted %s artifact contract is the base snapshot contract and is referenced rather than restated." % what, "type": "object", "allOf": [{"$ref": snapshot_id}]}


def edit(path, change):
    text = open(path, encoding="utf-8").read()
    updated = change(text)
    json.loads(updated)
    if updated == text:
        return
    print(("[dry] " if DRY else "") + os.path.relpath(path, REPO))
    if DRY:
        return
    if open(path, encoding="utf-8").read() != text:
        print("  changed while scanning; rerun")
        return
    open(path, "w", encoding="utf-8").write(updated)


def artifact(path, title, what, snapshot_id):
    edit(path, lambda text: json.dumps(referenced(title, what, snapshot_id), indent=2, ensure_ascii=False) + "\n")


def annotate(text, members):
    for key, ui in members.items():
        text = add_member(text, ["properties", key], "x-semio-ui", ui)
    return text


def main():
    xml_id, svg_id, pdf_id, zip_id = IDS + "xml/1.0/base/snapshot.json", IDS + "svg/1.1/base/snapshot.json", IDS + "pdf/1.7/base/snapshot.json", IDS + "zip/2.0/base/snapshot.json"
    xml_ui = json.load(open(XML + "/🔣️.json", encoding="utf-8")).get("properties", {})
    pdf_ui = json.load(open(PDF + "/🔣️.json", encoding="utf-8")).get("properties", {})
    edit(XML + "/📸️snapshot/🔣️.json", lambda text: annotate(text, {key: member["x-semio-ui"] for key, member in xml_ui.items() if "x-semio-ui" in member}))
    edit(PDF + "/📸️snapshot/🔣️.json", lambda text: annotate(text, {key: member["x-semio-ui"] for key, member in pdf_ui.items() if "x-semio-ui" in member}))
    artifact(XML + "/🔣️.json", "XmlArtifact", "XML", xml_id)
    artifact(PDF + "/🔣️.json", "PdfArtifact", "PDF", pdf_id)
    artifact(SVG + "/🔣️.json", "SvgArtifact", "SVG", svg_id)
    edit(XML + "/🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json", lambda text: replace_value(text, ["properties", "snapshot", "$ref"], xml_id))
    edit(PDF + "/🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json", lambda text: replace_value(text, ["properties", "snapshot", "$ref"], pdf_id))

    def svg_doc(text):
        doc = json.loads(text)["properties"]["doc"]
        if "$ref" in doc:
            return text
        node = locate(text, ["properties", "doc"])
        _, key_start, child = next(member for member in node.members if member[0] == "type")
        return text[:key_start] + '"$ref": ' + json.dumps(xml_id + "#/$defs/XmlDocument") + text[child.end:]

    edit(SVG + "/📸️snapshot/🔣️.json", svg_doc)
    zip_leaves = [ZIP + "/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json", ZIP + "/🌐️iso21320/🧬️schema/🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json"]
    inline = json.load(open(zip_leaves[0], encoding="utf-8")).get("$defs", {}).get("ZipSnapshot", {}).get("properties", {})
    carried = {key: member["x-semio-ui"] for key, member in inline.items() if "x-semio-ui" in member}
    edit(ZIP + "/🧱️base/🧬️schema/📸️snapshot/🔣️.json", lambda text: annotate(text, carried or {"schema": SCHEMA_UI}))
    for leaf in zip_leaves:
        edit(leaf, lambda text: remove_member(replace_value(text, ["properties", "snapshot"], {"$ref": zip_id}), [], "$defs"))


if __name__ == "__main__":
    main()
