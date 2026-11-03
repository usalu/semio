"""🔤 Lexing helpers for the codec layers: a code mask (strings and comments removed), function bodies and their body-level statements, and comma separated items, for Rust and TypeScript text."""
import re


def code_mask(text, lang="rs"):
    """🫗 List of booleans: True where the character is code (not inside a string, char literal or comment)."""
    n, mask, i = len(text), [True] * len(text), 0
    while i < n:
        ch = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                mask[k] = False
            i = j
        elif text.startswith("/*", i):
            j = text.find("*/", i + 2)
            j = n if j < 0 else j + 2
            for k in range(i, j):
                mask[k] = False
            i = j
        elif ch == '"' or (lang == "ts" and ch in "'`"):
            j = i + 1
            while j < n and text[j] != ch:
                j += 2 if text[j] == "\\" else 1
            for k in range(i, min(j + 1, n)):
                mask[k] = False
            i = j + 1
        elif lang == "rs" and ch == "'":
            m = re.match(r"'(?:\\.|[^'\\])'", text[i:i + 6])
            if m:
                for k in range(i, i + m.end()):
                    mask[k] = False
                i += m.end()
            else:
                i += 1
        else:
            i += 1
    return mask


def match_close(text, mask, open_at):
    """🫖 Index of the bracket that closes the one at `open_at`."""
    pairs = {"(": ")", "[": "]", "{": "}"}
    depth = 0
    for i in range(open_at, len(text)):
        if not mask[i]:
            continue
        c = text[i]
        if c in pairs:
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0:
                return i
    return len(text) - 1


def fn_bodies(text, mask, lang="rs"):
    """🫕 (open, close) brace indices of every function body."""
    out = []
    rx = re.compile(r"\bfn\s+\w+" if lang == "rs" else r"\bfunction\s*\*?\s*\w*")
    for m in rx.finditer(text):
        if not mask[m.start()]:
            continue
        i, paren = m.end(), 0
        while i < len(text):
            if mask[i]:
                c = text[i]
                if c in "([":
                    paren += 1
                elif c in ")]":
                    paren -= 1
                elif c == ";" and paren == 0:
                    break
                elif c == "{" and paren == 0:
                    out.append((i, match_close(text, mask, i)))
                    break
            i += 1
    return out


def statements(text, mask, open_at, close_at):
    """🫔 Body-level statements of a function body as (start, end) spans (end exclusive, terminator included)."""
    spans, i, depth, start = [], open_at + 1, 0, None
    while i < close_at:
        if not mask[i]:
            i += 1
            continue
        c = text[i]
        if start is None and not c.isspace():
            start = i
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0 and c == "}":
                rest = re.match(r"\s*(\S+)", text[i + 1:close_at + 1])
                nxt = rest.group(1) if rest else ""
                if nxt[:1] not in (";", ",", ")", ".", "?", "]") and not re.match(r"(else|catch|finally|as)\b", nxt):
                    spans.append((start, i + 1))
                    start = None
        elif c == ";" and depth == 0:
            spans.append((start, i + 1))
            start = None
        i += 1
    return spans


def split_items(text, mask, lo, hi, sep=","):
    """🫓 Spans of the `sep` separated items between lo and hi (exclusive) at the list's own depth; whitespace stripped."""
    spans, depth, start = [], 0, lo
    for i in range(lo, hi):
        if not mask[i]:
            continue
        c = text[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == sep and depth == 0:
            spans.append((start, i))
            start = i + 1
    spans.append((start, hi))
    out = []
    for a, b in spans:
        while a < b and text[a].isspace():
            a += 1
        while b > a and text[b - 1].isspace():
            b -= 1
        if b > a:
            out.append((a, b))
    return out


def tail_expression(text, mask, open_at, close_at):
    """🪡 The unterminated final expression of a function body (`Ok(Self{..})`), as (start, end), or None."""
    spans = statements(text, mask, open_at, close_at)
    start = spans[-1][1] if spans else open_at + 1
    while start < close_at and (text[start].isspace() or not mask[start]):
        start += 1
    return (start, close_at) if start < close_at else None
