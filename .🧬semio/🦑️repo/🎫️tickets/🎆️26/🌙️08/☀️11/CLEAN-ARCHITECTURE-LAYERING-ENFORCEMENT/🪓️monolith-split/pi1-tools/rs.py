#!/usr/bin/env python3
# 🧩️ Minimal Rust lexer helpers for the RENDERER-WP1 tools: comment/string stripping, identifier tokens, use-tree expansion, item names.
import re

KEYWORDS = set("as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try union".split())
PRIMS = set("bool char str u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 String Vec Option Some None Result Ok Err Box Default Clone Copy Send Sync Sized Drop Fn FnMut FnOnce Iterator IntoIterator From Into ToString ToOwned AsRef AsMut PartialEq Eq PartialOrd Ord Debug Display".split())


def blank(text):
    out = []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i))
            i = j
        elif text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth += 1
                    j += 2
                elif text.startswith("*/", j):
                    depth -= 1
                    j += 2
                else:
                    j += 1
            out.append(re.sub(r"[^\n]", " ", text[i:j]))
            i = j
        elif c == '"' or (c in "rb" and re.match(r'(?:b?r#*")|b"', text[i:i + 12])):
            m = re.match(r'(b?)(r?)(#*)"', text[i:i + 40])
            if not m or (c == "b" and not m.group(0)):
                out.append(c)
                i += 1
                continue
            if m.group(2):
                close = '"' + m.group(3)
                j = text.find(close, i + len(m.group(0)))
                j = n if j < 0 else j + len(close)
            else:
                j = i + len(m.group(0))
                while j < n and text[j] != '"':
                    j += 2 if text[j] == "\\" else 1
                j += 1
            out.append(re.sub(r"[^\n]", " ", text[i:j]))
            i = j
        elif c == "'":
            m = re.match(r"'(?:\\.[^']*|[^\\'])'", text[i:i + 16])
            if m:
                out.append(" " * len(m.group(0)))
                i += len(m.group(0))
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return "".join(out)


def idents(text):
    for m in re.finditer(r"(?<![A-Za-z0-9_$])([A-Za-z_][A-Za-z0-9_]*)", text):
        yield m.group(1), m.start()


def expand_use(body):
    # body: text after `use` up to `;`, returns list of (path_tokens list, alias or None, is_glob)
    pos = 0
    body = body.strip()

    def parse(prefix):
        nonlocal pos
        results = []
        segs = []
        while pos < len(body):
            c = body[pos]
            if c.isspace():
                pos += 1
                continue
            if c == "{":
                pos += 1
                while True:
                    sub = parse(prefix + segs)
                    results.extend(sub)
                    while pos < len(body) and body[pos].isspace():
                        pos += 1
                    if pos < len(body) and body[pos] == ",":
                        pos += 1
                        continue
                    break
                while pos < len(body) and body[pos] != "}":
                    pos += 1
                pos += 1
                return results
            if c == "*":
                pos += 1
                return results + [(prefix + segs, None, True)]
            if c == ":" and body.startswith("::", pos):
                pos += 2
                continue
            m = re.match(r"[A-Za-z_][A-Za-z0-9_]*", body[pos:])
            if m:
                segs.append(m.group(0))
                pos += len(m.group(0))
                am = re.match(r"\s+as\s+([A-Za-z_][A-Za-z0-9_]*)", body[pos:])
                if am:
                    pos += len(am.group(0))
                    return results + [(prefix + segs, am.group(1), False)]
                continue
            break
        if segs:
            results.append((prefix + segs, None, False))
        return results

    return parse([])


def uses(text, top_only=False):
    """Return list of (cfg_text, vis, path, local_name, glob) for every top-level style `use` item found in blanked text."""
    clean = blank(text)
    out = []
    pattern = r"((?:#\[[^\]]*\]\s*)*)(pub(?:\([^)]*\))?\s+)?\buse\s+([^;]*);"
    if top_only:
        pattern = r"(?m)^" + pattern
    for m in re.finditer(pattern, clean):
        cfg = text[m.start(1):m.end(1)].strip()
        vis = (m.group(2) or "").strip()
        for path, alias, glob in expand_use(m.group(3)):
            if not path and not glob:
                continue
            if glob:
                out.append((cfg, vis, path, None, True))
            else:
                local = alias or (path[-1] if path[-1] != "self" else path[-2])
                if path[-1] == "self" and not alias:
                    path = path[:-1]
                out.append((cfg, vis, path, local, False))
    return out


ITEM = re.compile(r"(?m)^[ \t]*(?:#\[[^\]\n]*\][ \t]*)*(?:pub(?:\([^)]*\))?[ \t]+)?(?:(?:async|unsafe|const|extern(?:[ \t]+\"[^\"]*\")?)[ \t]+)*(fn|struct|enum|trait|type|const|static|mod|union|macro_rules!)[ \t]+(?:mut[ \t]+)?([A-Za-z_][A-Za-z0-9_]*)")


def item_names(text):
    clean = blank(text)
    names = {}
    for m in ITEM.finditer(clean):
        names.setdefault(m.group(2), m.group(1))
    for m in re.finditer(r"thread_local!\s*\{(.*?)\n\}", clean, re.S):
        for s in re.finditer(r"static\s+([A-Za-z_][A-Za-z0-9_]*)", m.group(1)):
            names.setdefault(s.group(1), "static")
    return names
