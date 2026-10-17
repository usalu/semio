"""📝️ The envelope-level reader and printer of the committed .dsl.semio carrier."""

from __future__ import annotations

# region 🔖️Imports
import re

from semio_repo_test import digest
# endregion 🔖️Imports


# region 🔖️Carrier
PREAMBLE = re.compile(r"^semio\s+(\S+)\s+v(\d+)\n")
TABLE_HEAD = re.compile(r"^(\S+) \[([^\]]*)\] \{$")
#: 🧱 The record and list delimiters a field line may carry between its `key=value` fields — `[ { id=… } ]` writes a list of
#: records — kept as written, because they are carrier bytes, not a grammar to infer. Inside a list a field line opens
#: (`key=[ -0.4 0.6 ]`), the bare tokens are its elements and are kept as written too.
BRACKETS = frozenset("{}[]")


def split_fields(line):
    """✂️ Splits one carrier line on unquoted whitespace."""
    fields, current, quoted = [], "", False
    for character in line:
        if character == '"':
            quoted = not quoted
            current += character
        elif character.isspace() and not quoted:
            if current:
                fields.append(current)
                current = ""
        else:
            current += character
    if current:
        fields.append(current)
    return fields


def parse_dsl(envelope, text):
    """📖 Reads the committed `.dsl.semio` artifact at the carrier level: the envelope preamble, the
    ordered `key=value` field lines, and any typed table block written `name [col:TYPE …] { rows }`. Each
    row keeps the indentation it is written with, because a record-valued cell opens a nested table whose
    rows sit deeper than the table's own, and that depth is carrier bytes, not a grammar to infer. A field
    line whose value opens a record or list that closes lines later carries those lines as its `body`,
    kept the same way."""
    header = PREAMBLE.match(text)
    if header is None:
        raise AssertionError("identity-round-trip: the committed artifact does not open with a semio text preamble")
    if header.group(1) != envelope:
        raise AssertionError("identity-round-trip: expected the envelope %r, the artifact declares %r" % (envelope, header.group(1)))
    blocks, lines, cursor = [], text[header.end():].split("\n"), 0
    while cursor < len(lines):
        line = lines[cursor]
        cursor += 1
        if line == "":
            continue
        table = TABLE_HEAD.match(line)
        if table is not None:
            columns = [column.split(":", 1) for column in split_fields(table.group(2))]
            rows = []
            while cursor < len(lines) and lines[cursor] != "}":
                rows.append({"indent": lines[cursor][: len(lines[cursor]) - len(lines[cursor].lstrip(" "))], "fields": split_fields(lines[cursor])})
                cursor += 1
            if cursor >= len(lines):
                raise AssertionError("identity-round-trip: table %r is never closed" % table.group(1))
            cursor += 1
            blocks.append({"table": table.group(1), "columns": columns, "rows": rows})
            continue
        fields, lists = [], 0
        for token in split_fields(line):
            if token in BRACKETS:
                lists += {"[": 1, "]": -1}.get(token, 0)
                fields.append([token, None])
                continue
            key, separator, value = token.partition("=")
            if separator != "=" and lists > 0:
                fields.append([token, None])
                continue
            lists += value.endswith("[")
            if separator != "=":
                raise AssertionError(
                    "identity-round-trip: this artifact's carrier cannot be read by a second implementation. %r is not a "
                    "`key=value` field: the notation nests records and tables and flattens nested records into "
                    "`key=key=value` runs with no delimiter, and this repository publishes no grammar for it — the "
                    "subset's own `🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio` is the repository-wide "
                    "`payload = OCTET+` placeholder and the framework's DSL notation module carries no grammar document "
                    "either. Reconstructing these bytes would mean inferring a grammar from one example rather than "
                    "reading a specification, so this implementation refuses instead of pretending. The mutation "
                    "vocabulary above is unaffected: it is specified, and both implementations agree on all of it. This "
                    "is a documentation gap in the subset, not a defect in either codec." % token)
            fields.append([key, value])
        block, depth = {"fields": fields}, bracket_depth(line)
        if depth > 0:
            block["body"] = []
            while cursor < len(lines) and depth > 0:
                block["body"].append({"indent": lines[cursor][: len(lines[cursor]) - len(lines[cursor].lstrip(" "))], "fields": split_fields(lines[cursor])})
                depth += bracket_depth(lines[cursor])
                cursor += 1
            if depth > 0:
                raise AssertionError("identity-round-trip: the record %r opens is never closed" % fields[-1][0])
        blocks.append(block)
    return {"envelope": header.group(1), "version": int(header.group(2)), "blocks": blocks}


def bracket_depth(line):
    """🔢 How many record `{` and list `[` brackets one carrier line leaves open, quoted text excluded. A field
    line that leaves one open continues its value over the following lines, which are kept as written —
    indentation included — until the brackets balance, exactly as a table keeps its rows."""
    depth, quoted = 0, False
    for character in line:
        if character == '"':
            quoted = not quoted
        elif not quoted and character in "{[":
            depth += 1
        elif not quoted and character in "}]":
            depth -= 1
    return depth


def carrier_projection(text):
    """🧵️ The canonical carrier bytes as a comparable projection: the envelope preamble, every body line
    as written, and the digest and length of what this implementation emitted. The Rust subject builds
    the identical shape from ITS re-emission, and `digest` is the coordinator's own sha256, so the two
    languages' bytes are directly comparable."""
    preamble, _newline, body = text.partition("\n")
    body = body[:-1] if body.endswith("\n") else body
    return {
        "preamble": preamble,
        "lines": body.split("\n") if body else [],
        "dslDigest": digest(text.encode("utf-8")),
        "dslLength": len(text.encode("utf-8")),
    }


def print_dsl(document):
    """🖨️ Prints a parsed carrier document back to its canonical `.dsl.semio` bytes."""
    lines = []
    for block in document["blocks"]:
        if "fields" in block:
            lines.append(" ".join(key if value is None else "%s=%s" % (key, value) for key, value in block["fields"]))
            lines.extend(row["indent"] + " ".join(row["fields"]) for row in block.get("body", []))
            continue
        lines.append("%s [%s] {" % (block["table"], " ".join(":".join(column) for column in block["columns"])))
        lines.extend(row["indent"] + " ".join(row["fields"]) for row in block["rows"])
        lines.append("}")
    return "semio %s v%d\n%s\n" % (document["envelope"], document["version"], "\n".join(lines))
# endregion 🔖️Carrier
