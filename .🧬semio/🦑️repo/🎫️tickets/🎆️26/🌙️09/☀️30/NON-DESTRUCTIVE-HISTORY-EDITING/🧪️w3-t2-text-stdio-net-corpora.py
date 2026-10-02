#!/usr/bin/env python3
"""🧫️ W3-T2-TEXT (session 2): authors, by hand, the language-neutral net-leaf corpora of the stdio `txt` and `binary` text windows
(design §13.2 / F-5 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): one explicit Apply is ONE edit of the net leaves that
carry the committed document to the applied text. Every expected leaf is computed here independently of the Rust editors — the
shared ends are kept, paired lines that differ are re-set, surplus lines are removed last first or inserted; bytes collapse to ONE
range replacement — and a txt Apply that changes the line ending or the terminator is the whole-buffer lowering (`null`). Each
corpus ships its JSON Schema. Run from the repo root. Idempotent."""
import json
from pathlib import Path

TXT = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any")
BINARY = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any")


def shape(body):
    ending = "\r\n" if "\r\n" in body else "\n"
    trailing = body != "" and body.endswith(ending)
    lines = [] if body == "" else (body[: -len(ending)] if trailing else body).split(ending)
    return lines, trailing, ending


def ends(old, new):
    prefix = 0
    while prefix < min(len(old), len(new)) and old[prefix] == new[prefix]:
        prefix += 1
    suffix = 0
    while suffix < min(len(old), len(new)) - prefix and old[len(old) - 1 - suffix] == new[len(new) - 1 - suffix]:
        suffix += 1
    return prefix, suffix


def txt_leaves(before, after):
    (old, old_trailing, old_ending), (new, new_trailing, new_ending) = shape(before), shape(after)
    if (old_trailing, old_ending) != (new_trailing, new_ending):
        return None
    prefix, suffix = ends(old, new)
    old_middle, new_middle = old[prefix : len(old) - suffix], new[prefix : len(new) - suffix]
    paired = min(len(old_middle), len(new_middle))
    leaves = [{"kind": "set-line", "index": prefix + offset, "text": after_line} for offset, (before_line, after_line) in enumerate(zip(old_middle, new_middle)) if before_line != after_line]
    leaves += [{"kind": "remove-line", "index": prefix + offset} for offset in reversed(range(paired, len(old_middle)))]
    leaves += [{"kind": "insert-line", "index": prefix + offset, "text": line} for offset, line in enumerate(new_middle) if offset >= paired]
    return leaves


def binary_leaf(before, after):
    old, new = bytes.fromhex(before), bytes.fromhex(after)
    prefix, suffix = ends(old, new)
    remove, insert = len(old) - prefix - suffix, new[prefix : len(new) - suffix]
    return None if remove == 0 and not insert else {"kind": "replace-byte-range", "offset": prefix, "removeLen": remove, "insert": insert.hex()}


TXT_CASES = [
    ("unchanged", "alpha\nbeta\ngamma\n", "alpha\nbeta\ngamma\n"),
    ("edits-a-line", "alpha\nbeta\ngamma\n", "alpha\nBETA\ngamma\n"),
    ("inserts-a-line", "alpha\nbeta\ngamma\n", "alpha\nbeta\ndelta\ngamma\n"),
    ("removes-a-line", "alpha\nbeta\ngamma\n", "alpha\ngamma\n"),
    ("edits-two-adjacent-lines", "a\nb\nc\nd\n", "a\nB\nC\nd\n"),
    ("edits-around-a-kept-line", "a\nb\nc\nd\n", "a\nX\nc\nY\n"),
    ("grows-a-span", "a\nb\nc\n", "a\nx\ny\nz\nc\n"),
    ("shrinks-a-span", "a\nb\nc\nd\ne\n", "a\nX\ne\n"),
    ("appends-a-line", "a\n", "a\nb\n"),
    ("drops-a-repeated-line", "x\nx\nx\n", "x\nx\n"),
    ("edits-a-crlf-line", "a\r\nb\r\n", "a\r\nB\r\n"),
    ("edits-an-unterminated-last-line", "a\nb", "a\nc"),
    ("switches-the-line-ending", "a\nb\n", "a\r\nb\r\n"),
    ("drops-the-terminator", "a\nb\n", "a\nb"),
    ("empties-the-document", "a\n", ""),
]
BINARY_CASES = [
    ("unchanged", "deadbeef", "deadbeef"),
    ("changes-a-byte", "deadbeef", "de00beef"),
    ("inserts-a-byte", "deadbeef", "deadbe01ef"),
    ("removes-two-bytes", "deadbeef", "deef"),
    ("clears-the-buffer", "010203", ""),
    ("fills-an-empty-buffer", "", "cafe"),
    ("drops-a-repeated-byte", "aaaaaaaa", "aaaaaa"),
    ("replaces-every-byte", "0102", "0304"),
]

ID = {"type": "string", "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$"}
TXT_SCHEMA = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "https://json.schemas.assets.semio-tech.com/s/stdio/txt/utf-8/any/net-leaves/schema.json",
    "title": "TxtNetLeavesFixture",
    "description": "Language-neutral corpus of the net line leaves one explicit Apply of the plain-text window means; `null` is the whole-buffer lowering of an Apply that changes the line ending or the terminator.",
    "type": "object", "additionalProperties": False, "required": ["schema", "note", "cases"],
    "properties": {
        "schema": {"const": "semio.stdio.txt.net-leaves.v1"}, "note": {"type": "string"},
        "cases": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["id", "before", "after", "leaves"], "properties": {
            "id": ID, "before": {"type": "string"}, "after": {"type": "string"},
            "leaves": {"oneOf": [{"type": "null"}, {"type": "array", "items": {"oneOf": [
                {"type": "object", "additionalProperties": False, "required": ["kind", "index", "text"], "properties": {"kind": {"enum": ["set-line", "insert-line"]}, "index": {"type": "integer", "minimum": 0}, "text": {"type": "string"}}},
                {"type": "object", "additionalProperties": False, "required": ["kind", "index"], "properties": {"kind": {"const": "remove-line"}, "index": {"type": "integer", "minimum": 0}}},
            ]}}]},
        }}},
    },
}
BINARY_SCHEMA = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "https://json.schemas.assets.semio-tech.com/s/stdio/binary/raw/any/net-leaves/schema.json",
    "title": "BinaryNetLeavesFixture",
    "description": "Language-neutral corpus of the ONE net byte-range replacement one explicit Apply of the hex window means; `null` when the Apply changes no byte.",
    "type": "object", "additionalProperties": False, "required": ["schema", "note", "cases"],
    "properties": {
        "schema": {"const": "semio.stdio.binary.net-leaves.v1"}, "note": {"type": "string"},
        "cases": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["id", "before", "after", "leaf"], "properties": {
            "id": ID, "before": {"type": "string", "pattern": "^([0-9a-f]{2})*$"}, "after": {"type": "string", "pattern": "^([0-9a-f]{2})*$"},
            "leaf": {"oneOf": [{"type": "null"}, {"type": "object", "additionalProperties": False, "required": ["kind", "offset", "removeLen", "insert"], "properties": {"kind": {"const": "replace-byte-range"}, "offset": {"type": "integer", "minimum": 0}, "removeLen": {"type": "integer", "minimum": 0}, "insert": {"type": "string", "pattern": "^([0-9a-f]{2})*$"}}}]},
        }}},
    },
}


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


write(TXT / "🧫️fixtures/🧫️net-leaves/🔣️.json", {
    "schema": "semio.stdio.txt.net-leaves.v1",
    "note": "One explicit Apply of the plain-text window is ONE edit of these net line leaves, in application order (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); `null` is the whole-buffer lowering of an Apply that changes the line ending or the terminator.",
    "cases": [{"id": case_id, "before": before, "after": after, "leaves": txt_leaves(before, after)} for case_id, before, after in TXT_CASES],
})
write(TXT / "🧬️schema/🔣️net-leaves/🔣️.json", TXT_SCHEMA)
write(BINARY / "🧫️fixtures/🧫️net-leaves/🔣️.json", {
    "schema": "semio.stdio.binary.net-leaves.v1",
    "note": "One explicit Apply of the hex window is ONE edit of this ONE net byte-range replacement (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); `null` when the Apply changes no byte. Bytes are lowercase hex.",
    "cases": [{"id": case_id, "before": before, "after": after, "leaf": binary_leaf(before, after)} for case_id, before, after in BINARY_CASES],
})
write(BINARY / "🧬️schema/🔣️net-leaves/🔣️.json", BINARY_SCHEMA)
print("txt + binary net-leaf corpora written")
