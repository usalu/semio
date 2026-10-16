"""Tokenizer, item outline and use-tree parser for the osm split engine (Rust source, text level)."""
import re

IDENT_RE = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
CHAR_RE = re.compile(r"'(\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'")
NUM_RE = re.compile(r"[0-9][0-9A-Za-z_]*(?:\.[0-9][0-9A-Za-z_]*)?")
RAW_RE = re.compile(r'(b?r)(#*)"')


def tokenize(text):
    """Tokens: (kind, value, start, end). kinds: id, p (single punct char), str, ch, life, num. Comments dropped."""
    toks = []
    n = len(text)
    i = 0
    while i < n:
        c = text[i]
        if c.isspace():
            i += 1
            continue
        if c == "/" and text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
            continue
        if c == "/" and text.startswith("/*", i):
            d = 1
            i += 2
            while i < n and d:
                if text.startswith("/*", i):
                    d += 1
                    i += 2
                elif text.startswith("*/", i):
                    d -= 1
                    i += 2
                else:
                    i += 1
            continue
        if c in "br" and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            m = RAW_RE.match(text, i)
            if m:
                close = '"' + m.group(2)
                j = text.find(close, m.end())
                j = n if j < 0 else j
                toks.append(("str", text[m.end():j], i, min(n, j + len(close))))
                i = min(n, j + len(close))
                continue
        if c == '"' or (c == "b" and text.startswith('b"', i) and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_"))):
            s = i
            if c == "b":
                i += 1
            i += 1
            while i < n and text[i] != '"':
                if text[i] == "\\":
                    i += 2
                    continue
                i += 1
            toks.append(("str", text[s:i], s, i + 1))
            i += 1
            continue
        if c == "'":
            m = CHAR_RE.match(text, i)
            if m:
                toks.append(("ch", m.group(0), i, m.end()))
                i = m.end()
                continue
            m = IDENT_RE.match(text, i + 1)
            if m:
                toks.append(("life", m.group(0), i, m.end()))
                i = m.end()
                continue
            i += 1
            continue
        m = IDENT_RE.match(text, i)
        if m:
            toks.append(("id", m.group(0), i, m.end()))
            i = m.end()
            continue
        if c.isdigit():
            m = NUM_RE.match(text, i)
            toks.append(("num", m.group(0), i, m.end()))
            i = m.end()
            continue
        toks.append(("p", c, i, i + 1))
        i += 1
    return toks


FMT_CAPTURE = re.compile(r"\{([A-Za-z_][A-Za-z0-9_]*)")


def idents_of(text):
    """Identifier set of code (comments/strings stripped) plus inline format-string captures."""
    out = set()
    for kind, val, s, e in tokenize(text):
        if kind == "id":
            out.add(val)
        elif kind == "str":
            for m in FMT_CAPTURE.finditer(val):
                out.add(m.group(1))
    return out


def scope_idents_of(text):
    """Identifiers that can be resolved through lexical scope: not a field/method (after '.') nor a later path segment (after '::')."""
    toks = tokenize(text)
    out = set()
    for i, (k, v, s, e) in enumerate(toks):
        if k == "id":
            if i >= 1 and toks[i - 1][0] == "p" and toks[i - 1][1] == ".":
                continue
            if i >= 2 and toks[i - 1][0] == "p" and toks[i - 1][1] == ":" and toks[i - 2][0] == "p" and toks[i - 2][1] == ":" and toks[i - 2][3] == toks[i - 1][2]:
                if not (i >= 3 and False):
                    continue
            if i + 1 < len(toks) and toks[i + 1][0] == "p" and toks[i + 1][1] == ":":
                nxt2 = toks[i + 2] if i + 2 < len(toks) else None
                if not (nxt2 and nxt2[0] == "p" and nxt2[1] == ":" and nxt2[2] == toks[i + 1][3]):
                    continue
            out.add(v)
        elif k == "str":
            for m in FMT_CAPTURE.finditer(v):
                out.add(m.group(1))
    return out


def method_calls_of(text):
    """Names appearing right after a '.' or '::' (method, field or path-call access)."""
    toks = tokenize(text)
    out = set()
    for a, b, c, d in zip(toks, toks[1:], toks[2:] + [("e", "", 0, 0)], toks[3:] + [("e", "", 0, 0)] * 2):
        if a[0] == "p" and a[1] in ".:" and b[0] == "id" and c[0] == "p" and (c[1] == "(" or (c[1] == ":" and d[1] == ":")):
            out.add(b[1])
    return out


def depth_per_line(text):
    """Bracket depth at the start of each line (code only)."""
    toks = tokenize(text)
    starts = [0]
    for m in re.finditer("\n", text):
        starts.append(m.end())
    depths = [0] * len(starts)
    d = 0
    ti = 0
    li = 0
    # walk tokens, assign depth at line starts
    li_next = 1
    for kind, val, s, e in toks:
        while li_next < len(starts) and starts[li_next] <= s:
            depths[li_next] = d
            li_next += 1
        if kind == "p":
            if val in "{([":
                d += 1
            elif val in "})]":
                d -= 1
    while li_next < len(starts):
        depths[li_next] = d
        li_next += 1
    return depths


ITEM_HEAD = re.compile(r'^(?P<vis>pub(?:\([^)]*\))?\s+)?(?:(?:default|const|async|unsafe|extern\s+"[^"]*"|extern)\s+)*(?P<kw>fn|struct|enum|trait|impl|mod|use|const|static|type|union|macro_rules!|extern\s+crate)\b\s*(?P<rest>.*)$')


def outline(text):
    """Top-level items. dict(start, core, end (0-based inclusive), kw, name, vis, attrs[list of attr lines])."""
    lines = text.split("\n")
    depths = depth_per_line(text)
    N = len(lines)
    items = []
    i = 0
    pend = None
    while i < N:
        s = lines[i]
        if depths[i] != 0:
            i += 1
            continue
        st = s.strip()
        if st == "":
            pend = None
            i += 1
            continue
        inline_attrs = []
        head_text = s
        mi = re.match(r"^((?:#\[[^\]]*\]\s*)+)(\S.*)$", st)
        if mi and not mi.group(2).startswith("//") and depths[i + 1 if i + 1 < N else i] == depths[i] or (mi and not mi.group(2).startswith("//") and ITEM_HEAD.match(mi.group(2))):
            inline_attrs = re.findall(r"#\[[^\]]*\]", mi.group(1))
            head_text = mi.group(2)
        elif st.startswith("//") or st.startswith("#[") or st.startswith("#!["):
            if pend is None:
                pend = i
            j = i
            if st.startswith("#[") or st.startswith("#!["):
                while j + 1 < N and depths[j + 1] != 0:
                    j += 1
            i = j + 1
            continue
        s = head_text
        m = ITEM_HEAD.match(s)
        start = pend if pend is not None else i
        pend = None
        k = i
        while True:
            nd = depths[k + 1] if k + 1 < N else 0
            if nd == 0:
                last = lines[k].rstrip()
                if last.endswith(";") or last.endswith("}") or k + 1 >= N or last.endswith(")") or last.endswith("]"):
                    break
            k += 1
            if k >= N - 1:
                break
        kw = m.group("kw") if m else "other"
        rest = m.group("rest") if m else s
        nm = None
        if kw in ("fn", "struct", "enum", "trait", "mod", "const", "static", "type", "union", "macro_rules!"):
            mm = IDENT_RE.search(rest)
            nm = mm.group(0) if mm else None
        attrs = [l.strip() for l in lines[start:i] if l.strip().startswith("#[")] + inline_attrs
        items.append(dict(start=start, core=i, end=k, kw=kw, name=nm, vis=(m.group("vis") or "").strip() if m else "", attrs=attrs, head=s))
        i = k + 1
    return items


def parse_use(text):
    """Leaves of one `use` statement text: list of (path_tuple, bound_name, is_glob). Attributes/vis stripped by caller."""
    toks = [t for t in tokenize(text) if True]
    # drop leading 'use'
    pos = 0
    while pos < len(toks) and not (toks[pos][0] == "id" and toks[pos][1] == "use"):
        pos += 1
    pos += 1
    leaves = []

    def parse_tree(prefix, pos):
        # path segments
        path = list(prefix)
        while True:
            k, v, s, e = toks[pos]
            if k == "p" and v == ":":
                pos += 2
                continue
            if k == "id":
                if v == "self" and prefix and path == list(prefix):
                    # `self` inside group: refers to prefix itself
                    pos += 1
                    alias = None
                    if pos < len(toks) and toks[pos][0] == "id" and toks[pos][1] == "as":
                        alias = toks[pos + 1][1]
                        pos += 2
                    leaves.append((tuple(prefix), alias or prefix[-1], False))
                    return pos
                path.append(v)
                pos += 1
                if pos + 1 < len(toks) and toks[pos][0] == "p" and toks[pos][1] == ":" and toks[pos + 1][0] == "p" and toks[pos + 1][1] == ":":
                    pos += 2
                    # next could be '{' or '*' or ident
                    k2, v2, _, _ = toks[pos]
                    if k2 == "p" and v2 == "{":
                        pos += 1
                        while True:
                            pos = parse_tree(path, pos)
                            k3, v3, _, _ = toks[pos]
                            if k3 == "p" and v3 == ",":
                                pos += 1
                                k4, v4, _, _ = toks[pos]
                                if k4 == "p" and v4 == "}":
                                    pos += 1
                                    return pos
                                continue
                            if k3 == "p" and v3 == "}":
                                pos += 1
                                return pos
                            raise ValueError("use parse: " + text)
                    if k2 == "p" and v2 == "*":
                        leaves.append((tuple(path), "*", True))
                        return pos + 1
                    continue
                alias = None
                if pos < len(toks) and toks[pos][0] == "id" and toks[pos][1] == "as":
                    alias = toks[pos + 1][1]
                    pos += 2
                leaves.append((tuple(path), alias or path[-1], False))
                return pos
            if k == "p" and v == "{":
                pos += 1
                while True:
                    pos = parse_tree(path, pos)
                    k3, v3, _, _ = toks[pos]
                    if k3 == "p" and v3 == ",":
                        pos += 1
                        if toks[pos][0] == "p" and toks[pos][1] == "}":
                            pos += 1
                            return pos
                        continue
                    if k3 == "p" and v3 == "}":
                        pos += 1
                        return pos
                    raise ValueError("use parse: " + text)
            if k == "p" and v == "*":
                leaves.append((tuple(path), "*", True))
                return pos + 1
            raise ValueError(f"use parse unexpected {toks[pos]} in {text}")

    parse_tree([], pos)
    return leaves
