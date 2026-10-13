#!/usr/bin/env python3
"""🧱️ Rust source item splitter shared by the E2 plugin tools.

`clean(src)` blanks comments, string and char literals (newlines kept, offsets preserved) and
`items(raw, cl, a, b)` splits the region `[a, b)` of one module body into its top-level items. Every
item carries the raw span that starts right after the previous item (so leading docs, attributes and
region comments travel with it), its head span (attributes and docs excluded), its kind, its name and,
for an `impl`, the self type and trait. Braces inside generic arguments (`PagedList<T, {usize::MAX}>`) are skipped.
Copied from `e1-os-kernel/🧱️rust_items.py` for wave E1b; a `const`/`static` initializer after `=` no longer counts `<`/`>` as
generic brackets (`1 << 12`). It compiles nothing.
"""
import re


def clean(src):
    out = list(src)
    i, n = 0, len(src)

    def blank(a, b):
        for k in range(a, b):
            if out[k] != "\n":
                out[k] = " "

    while i < n:
        c = src[i]
        if c == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i)
            j = n if j < 0 else j
            blank(i, j); i = j; continue
        if c == "/" and i + 1 < n and src[i + 1] == "*":
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j): depth += 1; j += 2
                elif src.startswith("*/", j): depth -= 1; j += 2
                else: j += 1
            blank(i, j); i = j; continue
        m = re.match(r'b?r(#*)"', src[i:i + 20]) if c in "rb" and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")) else None
        if m:
            hashes = m.group(1)
            start = i + m.end()
            end = src.find('"' + hashes, start)
            end = n if end < 0 else end
            blank(start, end); i = end + 1 + len(hashes); continue
        if c == '"' or (c == "b" and i + 1 < n and src[i + 1] == '"' and not (i and (src[i - 1].isalnum() or src[i - 1] == "_"))):
            j = i + (2 if c == "b" else 1)
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            blank(i + (2 if c == "b" else 1), j); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(\\.[^']*|[^'\\])'", src[i:i + 12])
            if m:
                blank(i + 1, i + m.end() - 1); i += m.end(); continue
        i += 1
    return "".join(out)


OPEN = {"{": "}", "(": ")", "[": "]"}


def skip_group(cl, i):
    stack = [OPEN[cl[i]]]
    j = i + 1
    while j < len(cl) and stack:
        ch = cl[j]
        if ch in OPEN: stack.append(OPEN[ch])
        elif ch in ")]}":
            stack.pop()
        j += 1
    return j


def skip_ws(cl, i, b):
    while i < b and cl[i].isspace(): i += 1
    return i


def skip_attrs(cl, i, b):
    while True:
        i = skip_ws(cl, i, b)
        if cl.startswith("#![", i) or cl.startswith("#[", i):
            k = cl.index("[", i)
            i = skip_group(cl, k)
            continue
        return i


HEAD_RE = re.compile(r"\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:default\s+)?(?:const\s+(?=(?:async\s+|unsafe\s+|extern\s+(?:\"[^\"]*\"\s+)?)*fn\b))?(?:async\s+)?(?:unsafe\s+)?(?:extern\s+(?:\"[^\"]*\"\s+)?)?((?:\w+::)*\w+)\s*(!?)")


def head_kind(cl, h):
    m = HEAD_RE.match(cl, h)
    if not m: return "?", None
    word, bang = m.group(1), m.group(2)
    rest = cl[m.end():m.end() + 200]
    if bang:
        if word == "macro_rules":
            nm = re.match(r"\s*(\w+)", rest)
            return "macro_rules", nm.group(1) if nm else None
        return "macro_call", word.split("::")[-1]
    if word in ("fn", "struct", "enum", "trait", "union", "type", "static", "mod", "const"):
        nm = re.match(r"\s*(?:mut\s+)?(?:r#)?(\w+)", rest)
        return word, nm.group(1) if nm else None
    if word == "impl": return "impl", None
    if word == "use": return "use", None
    if word == "extern":
        if re.match(r"\s*crate", rest): return "extern_crate", None
        return "extern_block", None
    if word == "crate": return "extern_crate", None
    if word == "auto": return "trait", (re.match(r"\s*trait\s+(\w+)", rest) or [None, None])[1]
    return "?" + word, None


def item_end(cl, h, b, kind):
    if kind in ("macro_call", "macro_rules"):
        i = re.compile(r"(?:\w+::)*\w+\s*!").search(cl, h).end()
        i = skip_ws(cl, i, b)
        if kind == "macro_rules":
            m = re.match(r"\w+", cl[i:i + 200]); i = skip_ws(cl, i + (m.end() if m else 0), b)
        if i < b and cl[i] in OPEN:
            j = skip_group(cl, i)
            k = skip_ws(cl, j, b)
            if cl[i] == "{": return k + 1 if k < b and cl[k] == ";" else j
            return k + 1 if k < b and cl[k] == ";" else j
    i = h
    angle = 0
    initializer = False
    while i < b:
        ch = cl[i]
        if ch == "=" and not angle and kind in ("const", "static") and cl[i + 1] not in "=>" and cl[i - 1] not in "=!<>": initializer = True
        if initializer and ch in "<>": i += 1; continue
        if ch == "<": angle += 1
        elif ch == ">" and angle and cl[i - 1] not in "-=": angle -= 1
        if angle and ch == "{" and kind != "use":
            i = skip_group(cl, i); continue
        if ch == ";" and not angle:
            return i + 1
        if ch == "{":
            j = skip_group(cl, i)
            if kind in ("const", "static", "type", "use", "extern_crate"):
                i = j; continue
            if kind == "struct" or kind == "union" or kind == "enum" or kind == "trait" or kind == "impl" or kind == "fn" or kind == "mod" or kind == "macro_rules" or kind == "macro_call" or kind == "extern_block":
                k = skip_ws(cl, j, b)
                if kind in ("macro_call", "macro_rules") and k < b and cl[k] == ";": return k + 1
                return j
            i = j; continue
        if ch in "([":
            j = skip_group(cl, i)
            if kind in ("macro_call", "macro_rules") and i == cl.find("!", h) + 1 + (len(cl[cl.find("!", h) + 1:i]) - len(cl[cl.find("!", h) + 1:i].lstrip())):
                k = skip_ws(cl, j, b)
                return k + 1 if k < b and cl[k] == ";" else j
            i = j; continue
        i += 1
    return b


def impl_target(cl, h, e):
    head = cl[h:e]
    brace, angle = -1, 0
    for k, ch in enumerate(head):
        if ch == "<": angle += 1
        elif ch == ">" and angle and head[k - 1] not in "-=": angle -= 1
        elif ch == "{" and not angle: brace = k; break
    sig = head[:brace if brace >= 0 else len(head)]
    sig = re.sub(r"\s+", " ", sig)
    sig = re.sub(r"^\s*(?:unsafe\s+)?impl\s*", "", sig)
    if sig.startswith("<"):
        depth = 0
        for k, ch in enumerate(sig):
            if ch == "<": depth += 1
            elif ch == ">":
                depth -= 1
                if depth == 0: sig = sig[k + 1:]; break
    sig = re.split(r"\bwhere\b", sig)[0].strip()
    trait, ty = None, sig
    depth, cut = 0, -1
    for k in range(len(sig) - 3):
        ch = sig[k]
        if ch in "<([": depth += 1
        elif ch in ">)]": depth -= 1
        elif depth == 0 and sig[k:k + 5] == " for ":
            cut = k; break
    if cut >= 0:
        trait, ty = sig[:cut].strip(), sig[cut + 5:].strip()
    def base(t):
        if t is None: return None
        t = t.lstrip("&").strip()
        t = re.sub(r"^(?:mut\s+|dyn\s+|'\w+\s+)", "", t)
        t = re.split(r"[<\s(]", t)[0]
        return t.split("::")[-1] or None
    return base(ty), base(trait)


def items(raw, cl, a, b):
    out = []
    prev = a
    i = a
    while True:
        h = skip_attrs(cl, i, b)
        if h >= b: break
        kind, name = head_kind(cl, h)
        e = item_end(cl, h, b, kind)
        it = {"start": prev, "head": h, "end": e, "kind": kind, "name": name}
        if kind == "impl":
            it["self"], it["trait"] = impl_target(cl, h, e)
        out.append(it)
        prev = e
        i = e
    return out, prev


def line_of(text, pos):
    return text.count("\n", 0, pos) + 1
