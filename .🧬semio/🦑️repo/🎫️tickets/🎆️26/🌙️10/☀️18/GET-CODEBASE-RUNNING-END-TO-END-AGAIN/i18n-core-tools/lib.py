#!/usr/bin/env python3
"""Shared machinery of the i18n-core tools: a virtual tree over a root, anchored edits and the JS statement scanner."""
import hashlib, json, os, re, sys

LIVE = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
P = "🧰️framework/🔨️modules/🏃️process"
LANG = P + "/🏗️build/🗣️language"
MAX_PATH = 256


def sha(data):
    return hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf8")).hexdigest()


class Refusal(Exception):
    pass


class Tree:
    """A root plus the in-memory result of every edit; nothing is written to the root."""

    def __init__(self, root):
        self.root = root
        self.new = {}
        self.base = {}
        self.created = set()

    def path(self, rel):
        return os.path.join(self.root, rel)

    def exists(self, rel):
        return rel in self.new or os.path.isfile(self.path(rel))

    def text(self, rel):
        if rel in self.new:
            return self.new[rel]
        with open(self.path(rel), encoding="utf8") as handle:
            return handle.read()

    def _remember(self, rel):
        if rel not in self.base and rel not in self.created:
            self.base[rel] = sha(open(self.path(rel), "rb").read())

    def put(self, rel, text):
        if len(rel.encode("utf8")) > MAX_PATH or len(rel) > MAX_PATH:
            raise Refusal("path longer than %d characters: %s" % (MAX_PATH, rel))
        if rel not in self.new and not os.path.isfile(self.path(rel)):
            self.created.add(rel)
        elif rel not in self.new:
            self._remember(rel)
        self.new[rel] = text

    def edit(self, rel, old, new, count=1):
        if not self.exists(rel):
            raise Refusal("missing file: " + rel)
        text = self.text(rel)
        found = text.count(old)
        if found != count:
            raise Refusal("anchor found %d times (expected %d) in %s: %r" % (found, count, rel, old[:120]))
        self.put(rel, text.replace(old, new))

    def edit_if_present(self, rel, old, new):
        if old in self.text(rel):
            self.edit(rel, old, new)

    def edit_regex(self, rel, pattern, repl, count=1, flags=0):
        text = self.text(rel)
        found = len(re.findall(pattern, text, flags))
        if found != count:
            raise Refusal("pattern found %d times (expected %d) in %s: %r" % (found, count, rel, pattern[:120]))
        self.put(rel, re.sub(pattern, repl, text, flags=flags))

    def changed(self):
        return sorted(self.new)


def rel_import(from_file, target):
    """Relative import specifier of `target` (a root-relative path) from `from_file`."""
    start = os.path.dirname(from_file).split("/")
    goal = target.split("/")
    i = 0
    while i < len(start) and i < len(goal) - 1 and start[i] == goal[i]:
        i += 1
    ups = [".."] * (len(start) - i)
    out = "/".join(ups + goal[i:])
    return out if out.startswith(".") else "./" + out


def scan_string(text, i):
    """Index just after the string or template literal that starts at text[i]."""
    quote = text[i]
    i += 1
    while i < len(text):
        c = text[i]
        if c == "\\":
            i += 2
            continue
        if quote == "`" and c == "$" and text[i + 1:i + 2] == "{":
            i = scan_braces(text, i + 1)
            continue
        if c == quote:
            return i + 1
        i += 1
    raise Refusal("unterminated literal")


def scan_braces(text, i):
    """Index just after the balanced `{...}` that starts at text[i]."""
    depth = 0
    while i < len(text):
        c = text[i]
        if c in "\"'`":
            i = scan_string(text, i)
            continue
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    raise Refusal("unbalanced braces")


def scan_call(text, open_index):
    """Index of the `)` that closes the `(` at text[open_index]."""
    depth = 0
    i = open_index
    while i < len(text):
        c = text[i]
        if c in "\"'`":
            i = scan_string(text, i)
            continue
        if c == "/" and text[i + 1:i + 2] == "/":
            i = text.index("\n", i)
            continue
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise Refusal("unbalanced call")


def split_template(literal):
    """Parts of a JS literal: ('t', text) static pieces (escapes decoded) and ('e', source) `${}` expressions."""
    quote, body = literal[0], literal[1:-1]
    parts, buffer, i = [], [], 0
    esc = {"n": "\n", "t": "\t", "r": "\r", "\\": "\\", '"': '"', "'": "'", "`": "`", "$": "$", "0": "\0"}
    while i < len(body):
        c = body[i]
        if c == "\\":
            n = body[i + 1]
            if n == "u":
                if body[i + 2] == "{":
                    end = body.index("}", i)
                    buffer.append(chr(int(body[i + 3:end], 16)))
                    i = end + 1
                else:
                    buffer.append(chr(int(body[i + 2:i + 6], 16)))
                    i += 6
                continue
            buffer.append(esc.get(n, n))
            i += 2
            continue
        if quote == "`" and c == "$" and body[i + 1:i + 2] == "{":
            end = scan_braces(body, i + 1)
            if buffer:
                parts.append(("t", "".join(buffer)))
                buffer = []
            parts.append(("e", body[i + 2:end - 1]))
            i = end
            continue
        buffer.append(c)
        i += 1
    if buffer:
        parts.append(("t", "".join(buffer)))
    return parts


def escape_braces(text):
    return text.replace("{", "{{").replace("}", "}}")


def js_string(text):
    return json.dumps(text, ensure_ascii=False)


def edit_json(tree, rel, fn):
    """Edits a JSON file through `fn(document)` when re-serialization reproduces the file byte for byte."""
    text = tree.text(rel)
    document = json.loads(text)
    for indent in (1, 2, 4):
        for tail in ("\n", ""):
            if json.dumps(document, indent=indent, ensure_ascii=False) + tail == text:
                fn(document)
                tree.put(rel, json.dumps(document, indent=indent, ensure_ascii=False) + tail)
                return
    raise Refusal("JSON file is not in canonical json.dumps form: " + rel)


def import_path(from_file, process_relative):
    """Import specifier of a file of the process module (`P`-relative) from a library file."""
    return rel_import(from_file, P + "/" + process_relative)


def split_args(text, open_index, close_index):
    """Top-level argument spans (start, end) of the call whose parentheses are at open_index and close_index."""
    spans, depth, start, i = [], 0, open_index + 1, open_index + 1
    while i < close_index:
        c = text[i]
        if c in "\"'`":
            i = scan_string(text, i)
            continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == "," and depth == 0:
            spans.append((start, i))
            start = i + 1
        i += 1
    spans.append((start, close_index))
    return spans


def rewrite_calls(text, name, arg_index, fn):
    """Rewrites argument `arg_index` of every call `name(`...`)` through fn(source) -> source; returns (text, count)."""
    out, pos, count = [], 0, 0
    for match in re.finditer(r"(?<![A-Za-z0-9_$])" + re.escape(name) + r"\(", text):
        if match.start() < pos:
            continue
        open_index = match.end() - 1
        close_index = scan_call(text, open_index)
        spans = split_args(text, open_index, close_index)
        if len(spans) <= arg_index:
            continue
        start, end = spans[arg_index]
        new = fn(text[start:end])
        out.append(text[pos:start])
        out.append(new)
        pos = end
        count += 1
    out.append(text[pos:])
    return "".join(out), count
