#!/usr/bin/env python3
"""🐍️ Independent model of the policy literal law (shares no code with 🟦️.ts and no parser library): a hand-written JavaScript lexer finds the numeric and string literals of a source; `🐍️.py --literals` reads a source on stdin and prints them, `🐍️.py REPOSITORY ROWFILE...` prints the findings of the law as JSON (literals of the listed sources that equal a policy value of the schema and are not allowed)."""
import json
import os
import re
import sys

ROOTS = {
    "library": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",
    "process": "🧰️framework/🔨️modules/🏃️process",
    "repository": "",
}
NUMBER = re.compile(r"0[xX][0-9a-fA-F_]+n?|0[bB][01_]+n?|0[oO][0-7_]+n?|(?:\d[\d_]*\.?[\d_]*|\.\d[\d_]*)(?:[eE][+-]?\d+)?n?")
KEYWORDS_BEFORE_REGEX = {"return", "typeof", "instanceof", "in", "of", "new", "delete", "void", "throw", "case", "do", "else", "yield", "await"}
VALUE_ENDS = {")", "]", "}", "name", "number", "string"}
ESCAPES = {"n": "\n", "t": "\t", "r": "\r", "b": "\b", "f": "\f", "v": "\v", "0": "\0"}


def number_value(text):
    clean = text.replace("_", "")
    if clean.endswith("n"):
        return None
    prefix = clean[:2].lower()
    if prefix in ("0x", "0b", "0o"):
        return int(clean[2:], {"0x": 16, "0b": 2, "0o": 8}[prefix])
    value = float(clean)
    return int(value) if value == int(value) else value


def decode(body):
    out = []
    at = 0
    while at < len(body):
        if body[at] != "\\":
            out.append(body[at])
            at += 1
            continue
        escape = body[at + 1 : at + 2]
        if escape == "u" and body[at + 2 : at + 3] == "{":
            end = body.index("}", at)
            out.append(chr(int(body[at + 3 : end], 16)))
            at = end + 1
        elif escape in ("u", "x"):
            size = 4 if escape == "u" else 2
            out.append(chr(int(body[at + 2 : at + 2 + size], 16)))
            at += 2 + size
        elif escape == "\n":
            at += 2
        else:
            out.append(ESCAPES.get(escape, escape))
            at += 2
    return "".join(out)


def literals(text):
    """Every numeric and string literal outside comments, regular expressions and the text parts of templates, as (line, kind, value)."""
    found = []
    size = len(text)
    at = 0
    line = 1
    previous = ""
    braces = []

    def template(start, whole):
        nonlocal at, line, previous
        at = start
        first = line
        while at < size:
            if text[at] == "\\":
                line += text.count("\n", at, at + 2)
                at += 2
            elif text[at] == "`":
                if whole:
                    found.append((first, "string", decode(text[start:at])))
                at += 1
                previous = "string"
                return
            elif text.startswith("${", at):
                braces.append(0)
                at += 2
                previous = "{"
                return
            else:
                line += text[at] == "\n"
                at += 1

    while at < size:
        char = text[at]
        if char in " \t\r﻿\n":
            line += char == "\n"
            at += 1
        elif text.startswith("//", at):
            end = text.find("\n", at)
            at = size if end < 0 else end
        elif text.startswith("/*", at):
            end = text.find("*/", at + 2)
            end = size if end < 0 else end + 2
            line += text.count("\n", at, end)
            at = end
        elif char in "\"'":
            start = at
            at += 1
            while at < size and text[at] != char:
                at += 2 if text[at] == "\\" else 1
            found.append((line, "string", decode(text[start + 1 : at])))
            line += text.count("\n", start, at)
            at += 1
            previous = "string"
        elif char == "`":
            template(at + 1, True)
        elif char == "}" and braces and braces[-1] == 0:
            braces.pop()
            template(at + 1, False)
        elif char == "/" and previous not in VALUE_ENDS:
            at += 1
            in_class = False
            while at < size and (in_class or text[at] != "/"):
                if text[at] == "\\":
                    at += 1
                elif text[at] == "[":
                    in_class = True
                elif text[at] == "]":
                    in_class = False
                at += 1
            at += 1
            while at < size and (text[at].isalnum() or text[at] == "_"):
                at += 1
            previous = "string"
        elif char.isdigit() or char == "." and text[at + 1 : at + 2].isdigit():
            match = NUMBER.match(text, at)
            value = number_value(match.group(0))
            if value is not None:
                found.append((line, "number", value))
            at = match.end()
            previous = "number"
        elif char.isalpha() or char in "_$#" or ord(char) > 127:
            end = at + 1
            while end < size and (text[end].isalnum() or text[end] in "_$" or ord(text[end]) > 127):
                end += 1
            word = text[at:end]
            previous = word if word in KEYWORDS_BEFORE_REGEX else "name"
            at = end
        else:
            if braces and char == "{":
                braces[-1] += 1
            elif braces and char == "}":
                braces[-1] -= 1
            previous = char
            at += 1
    return found


def policy_values(container, definitions, minimum_number, minimum_text):
    numbers = set()
    strings = set()
    constants = [container[name]["const"] for name in definitions]
    for constant in constants:
        for value in constant.values():
            for item in value if isinstance(value, list) else [value]:
                if isinstance(item, str) and len(item) >= minimum_text:
                    strings.add(item)
    stack = list(constants)
    while stack:
        item = stack.pop()
        if isinstance(item, bool):
            continue
        if isinstance(item, (int, float)) and abs(item) >= minimum_number:
            numbers.add(item)
        elif isinstance(item, list):
            stack.extend(item)
        elif isinstance(item, dict):
            stack.extend(item.values())
    return numbers, strings


def findings(repository, row_path):
    row = json.load(open(row_path, encoding="utf8"))
    base = os.path.join(repository, ROOTS[row["root"]])
    if "document" in row:
        value = json.load(open(os.path.join(base, row["document"]), encoding="utf8"))
        for key in row.get("pointer", []):
            value = value[key]
        container, names = {"policy": {"const": value}}, ["policy"]
    else:
        container, names = json.load(open(os.path.join(base, row["schema"]), encoding="utf8"))[row.get("container", "definitions")], row["definitions"]
    numbers, strings = policy_values(container, names, row.get("minimumNumber", 100), row.get("minimumText", 5))
    allowed = {(entry["source"], json.dumps(entry["value"])) for entry in row.get("allow", [])}
    out = []
    for source in row["sources"]:
        text = open(os.path.join(base, source), encoding="utf8").read()
        specifiers = set(re.findall(r"""(?:from|import)\s*\(?\s*["']([^"']+)["']""", text))
        for line, kind, value in literals(text):
            hit = kind == "number" and value in numbers or kind == "string" and value in strings and value not in specifiers
            if hit and (source, json.dumps(value)) not in allowed:
                out.append({"source": source, "line": line, "kind": kind, "value": value})
    return out


if sys.argv[1] == "--literals":
    print(json.dumps([{"line": line, "kind": kind, "value": value} for line, kind, value in literals(sys.stdin.read())], ensure_ascii=False))
else:
    print(json.dumps({path: findings(sys.argv[1], path) for path in sys.argv[2:]}, ensure_ascii=False))
