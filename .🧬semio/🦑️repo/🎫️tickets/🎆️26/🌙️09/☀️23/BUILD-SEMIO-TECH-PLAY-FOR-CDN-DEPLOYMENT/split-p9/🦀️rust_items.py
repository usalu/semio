#!/usr/bin/env python3
"""🦀️ Offset-preserving Rust item parser for the P9 rehosting tools.

`strip` blanks comments and literals without moving any byte; `parse_scope` walks one module body and yields every
item (attributes, visibility, keyword, name, span, inline body span) so callers can move, rewrite or re-declare items
by byte offset in the raw text.
"""
import re

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
KEYWORDS = ("mod", "use", "struct", "enum", "union", "trait", "impl", "fn", "const", "static", "type", "extern", "macro_rules")


def strip(src):
    """🧹️ Blanks comments, string, raw-string and char literals while keeping every byte offset (`#[path]` strings stay)."""
    out, i, n = [], 0, len(src)
    blank = lambda text: re.sub(r"[^\n]", " ", text)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i))
            i = j
        elif src.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif src.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            out.append(blank(src[i:j]))
            i = j
        elif c in "rb" and re.match(r'(?:br|r)#*"', src[i:i + 40]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'(?:br|r)(#*)"', src[i:])
            close = '"' + m.group(1)
            j = src.find(close, i + len(m.group(0)))
            j = n if j < 0 else j + len(close)
            out.append('"' + blank(src[i + 1:j - 1]) + '"')
            i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            j += 1
            keep = re.search(r"#\[path\s*=\s*$", "".join(out[-12:]))
            out.append(src[i:j] if keep else '"' + blank(src[i + 1:j - 1]) + '"')
            i = j
        elif c == "'" and (re.match(r"'(\\.|[^\\'])'", src[i:i + 4]) or re.match(r"'\\(x..|u\{[0-9a-fA-F]+\})'", src[i:i + 12])):
            m = re.match(r"'(\\x..|\\u\{[0-9a-fA-F]+\}|\\.|[^\\'])'", src[i:])
            out.append(" " * len(m.group(0)))
            i += len(m.group(0))
        else:
            out.append(c)
            i += 1
    result = "".join(out)
    assert len(result) == len(src)
    return result


def match_close(s, i):
    """🔚️ Index of the bracket closing the one at `i` (stripped text)."""
    pairs = {"(": ")", "[": "]", "{": "}"}
    stack = [pairs[s[i]]]
    j = i + 1
    while j < len(s):
        ch = s[j]
        if ch in pairs:
            stack.append(pairs[ch])
        elif ch in ")]}":
            stack.pop()
            if not stack:
                return j
        j += 1
    raise ValueError(f"unbalanced bracket at {i}")


def skip_ws(s, i, end):
    while i < end and s[i].isspace():
        i += 1
    return i


class Item:
    """🧩️ One item of a module body: `start` covers its leading attributes, `head` its visibility/keyword."""

    def __init__(self, **kw):
        self.__dict__.update(kw)

    def __repr__(self):
        return f"Item({self.kind} {self.name} {self.start}:{self.end})"


def find_end(s, i, end, kind):
    """📏️ End offset (exclusive) of the item whose keyword starts at `i`."""
    semicolon_only = kind in ("use", "const", "static", "type", "extern")
    j = i
    while j < end:
        ch = s[j]
        if ch in "([":
            j = match_close(s, j) + 1
            continue
        if ch == "{":
            close = match_close(s, j)
            if semicolon_only:
                j = close + 1
                continue
            k = skip_ws(s, close + 1, end)
            return k + 1 if kind == "macro" and k < end and s[k] == ";" else close + 1
        if ch == ";":
            return j + 1
        j += 1
    return end


def parse_scope(s, start, end):
    """📜️ Items of the module body `s[start:end]` (stripped text), in source order."""
    items, i = [], start
    while True:
        i = skip_ws(s, i, end)
        if i >= end:
            break
        item_start, attrs = i, []
        while s.startswith("#", i) and re.match(r"#!?\s*\[", s[i:i + 4]):
            b = s.index("[", i)
            close = match_close(s, b)
            attrs.append((i, close + 1))
            i = skip_ws(s, close + 1, end)
        if attrs and s[attrs[0][0] + 1] == "!":
            items.append(Item(kind="inner-attr", name=None, start=item_start, end=attrs[-1][1], attrs=attrs, vis="", head=item_start, body=None))
            i = attrs[-1][1]
            continue
        head = i
        m = re.match(r"pub\s*(\(\s*(?:crate|super|self|in\s+[^)]*)\s*\))?\s*", s[i:end])
        vis = ""
        if m and re.match(r"pub\b", s[i:i + 4]):
            vis = re.sub(r"\s+", "", m.group(0))
            i += m.end()
        m = re.match(r"(?:(?:async|const|unsafe|default|extern\s+\"[^\"]*\")\s+)+(?=(?:fn|impl|trait)\b)", s[i:end])
        k = i + (m.end() if m else 0)
        word = re.match(IDENT + r"!?", s[k:end])
        if not word:
            raise ValueError(f"cannot parse item at {i}: {s[i:i + 60]!r}")
        w = word.group(0)
        name, body = None, None
        if w in ("struct", "enum", "union", "trait", "fn", "const", "static", "type", "mod"):
            kind = w
            nm = re.match(r"\s*(" + IDENT + r")", s[k + len(w):end])
            name = nm.group(1) if nm else None
            if kind == "const" and name == "_":
                name = None
        elif w == "impl":
            kind = "impl"
        elif w == "use":
            kind = "use"
        elif w == "extern":
            kind = "extern"
            nm = re.match(r"extern\s+crate\s+(" + IDENT + r")(?:\s+as\s+(" + IDENT + r"))?", s[k:end])
            name = (nm.group(2) or nm.group(1)) if nm else None
        elif w == "macro_rules!":
            kind = "macro_rules"
            nm = re.match(r"macro_rules!\s*(" + IDENT + r")", s[k:end])
            name = nm.group(1)
        elif w == "auto":
            kind = "trait"
            nm = re.match(r"auto\s+trait\s+(" + IDENT + r")", s[k:end])
            name = nm.group(1)
        else:
            mm = re.match(r"(" + IDENT + r"(?:\s*::\s*" + IDENT + r")*)\s*!", s[k:end])
            if not mm:
                raise ValueError(f"unknown item at {k}: {s[k:k + 80]!r}")
            kind, name = "macro", mm.group(1).replace(" ", "")
        stop = find_end(s, k, end, kind)
        if kind == "mod" and s[stop - 1] == "}":
            body = (s.index("{", k) + 1, stop - 1)
        items.append(Item(kind=kind, name=name, start=item_start, end=stop, attrs=attrs, vis=vis, head=head, body=body, kw=k))
        i = stop
    return items


def attr_text(raw, attrs):
    return [raw[a:b] for a, b in attrs]


def path_attr(raw, attrs):
    for a, b in attrs:
        m = re.match(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]', raw[a:b])
        if m:
            return m.group(1)
    return None


def is_test_gated(raw, attrs):
    for a, b in attrs:
        t = re.sub(r"\s+", "", raw[a:b])
        if t.startswith("#[cfg(") and re.search(r"\btest\b", t) and not re.search(r"not\(test", t) and not t.startswith("#[cfg(not("):
            return True
    return False


def split_top(text, sep=","):
    parts, d, cur = [], 0, ""
    for ch in text:
        if ch in "({[":
            d += 1
        if ch in ")}]":
            d -= 1
        if ch == sep and d == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    return parts


def use_leaves(tree):
    """🌿️ Flattens a `use` tree (stripped, no `use`/`;`) into (segments, alias) leaves; `*` and `self` are kept as segments."""
    tree = tree.strip()
    m = re.match(r"^((?:::)?(?:" + IDENT + r"\s*::\s*)*)\{(.*)\}\s*$", tree, re.S)
    if m:
        prefix = [x for x in re.split(r"\s*::\s*", m.group(1).strip()) if x]
        out = []
        for part in split_top(m.group(2)):
            for segs, alias in use_leaves(part):
                out.append((prefix + segs, alias))
        return out
    am = re.match(r"^(.*?)\s+as\s+(" + IDENT + r")\s*$", tree, re.S)
    alias = None
    if am:
        tree, alias = am.group(1), am.group(2)
    segs = [x for x in re.split(r"\s*::\s*", tree.strip()) if x]
    return [(segs, alias)]
