#!/usr/bin/env python3
"""🔓️ Name-keyed visibility widening: definitions of a flipped crate whose visibility is narrower than `pub` and whose name occurs in a file outside the crate become `pub` (items also `#[doc(hidden)]`). Works on texts (overlay) and returns the changed texts plus a report."""
import re, os, collections
from lib import *
from rs import blank
import rsparse as P

IDENT = re.compile(r"(?<![A-Za-z0-9_$'])([A-Za-z_][A-Za-z0-9_]*)")
RESTRICTED = re.compile(r"^(?:pub\s*\(\s*(?:crate|super|self|in\s+[^)]*)\s*\))?$")
SKIP_NAMES = set("fmt eq ne cmp partial_cmp hash clone drop deref deref_mut from into try_from as_ref as_mut borrow next default".split())


def vis_of(stmt_vis):
    return stmt_vis.strip()


def collect(text, clean, lo, hi, depth_inline=True):
    """[(kind, name, vis, start, end, is_item)] restricted definitions at module level of text[lo:hi] and inside struct/impl/enum bodies"""
    out = []
    for st in P.statements(text, clean, lo, hi):
        if st.kind == "use" and (st.vis or "").strip() and RESTRICTED.match(st.vis.strip()):
            for path, alias, glob in P.use_entries(st, clean):
                if not glob:
                    local = alias or (path[-1] if path[-1] != "self" else path[-2])
                    out.append(("use", local, st.vis, st.start, st.head_end if st.head_end > st.start else st.end, False, st))
            continue
        if st.kind in ("fn", "struct", "enum", "union", "trait", "type", "const", "static", "mod") and st.name:
            if RESTRICTED.match(st.vis or ""):
                out.append((st.kind, st.name, st.vis, st.start, st.head_end, True, st))
            if st.kind == "mod" and st.inline:
                out += collect(text, clean, st.body[0], st.body[1])
            if st.kind in ("struct", "union"):
                out += fields(text, clean, st)
        if st.kind == "impl":
            out += impl_members(text, clean, st)
    return out


def body_range(clean, st):
    i = clean.find("{", st.start, st.end)
    return (i + 1, st.end - 1) if i != -1 and clean[st.end - 1] == "}" else None


def fields(text, clean, st):
    rng = body_range(clean, st)
    out = []
    if rng:
        lo, hi = rng
        depth = 0
        start = lo
        pieces = []
        for i in range(lo, hi):
            c = clean[i]
            if c in "<([{":
                depth += 1
            elif c in ">)]}":
                depth -= 1
            elif c == "," and depth == 0:
                pieces.append((start, i))
                start = i + 1
        pieces.append((start, hi))
        for a, b in pieces:
            seg = clean[a:b]
            m = re.match(r"\s*((?:\#\[[^\]]*\]\s*)*)(pub(?:\s*\([^)]*\))?\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:", seg)
            if m and RESTRICTED.match((m.group(2) or "").strip()):
                out.append(("field", m.group(3), (m.group(2) or "").strip(), a + m.start(2) if m.group(2) else a + m.start(3), a + m.end(2) if m.group(2) else a + m.start(3), False, None))
    else:
        # tuple struct: struct S(pub(crate) T, ...);
        m = re.search(r"\(\s*", clean[st.start:st.end])
        # tuple fields are positional and have no name: skipped (callers use `.0`)
    return out


def impl_members(text, clean, st):
    rng = body_range(clean, st)
    out = []
    header = clean[st.start:st.head_end + 200]
    is_trait_impl = bool(re.search(r"\bimpl\b[^{]*\bfor\b", clean[st.start:(rng[0] if rng else st.end)]))
    if rng and not is_trait_impl:
        for s2 in P.statements(text, clean, rng[0], rng[1]):
            if s2.kind in ("fn", "const", "type") and s2.name and RESTRICTED.match(s2.vis or ""):
                out.append((s2.kind if s2.kind != "fn" else "method", s2.name, s2.vis, s2.start, s2.head_end, False, s2))
    return out


def apply_widen(text, defs, names):
    """text with the definitions whose name is in names widened"""
    edits = []
    done_uses = set()
    for kind, name, vis, a, b, is_item, st in defs:
        if name not in names or name in SKIP_NAMES:
            continue
        clean = blank(text)
        if kind == "use":
            if st.start in done_uses:
                continue
            done_uses.add(st.start)
            m = re.search(r"pub\s*\([^)]*\)\s*", clean[st.start:st.end])
            if m:
                edits.append((st.start + m.start(), st.start + m.end(), "pub "))
            continue
        seg = text[a:b]
        # find the visibility token inside [a, b)
        if kind == "field":
            if vis:
                edits.append((a, b, "pub "))
            else:
                edits.append((a, a, "pub "))
            continue
        m = re.search(r"pub\s*\([^)]*\)\s*", clean[a:b])
        if m:
            if is_item:
                edits.append((a, a, "#[doc(hidden)]\n"))
            edits.append((a + m.start(), a + m.end(), "pub "))
        else:
            # private: insert `pub ` before the keyword after attributes
            am = P.ATTR_GROUP.match(clean, a)
            at = am.end() if am else a
            if is_item:
                edits.append((a, a, "#[doc(hidden)]\n"))
            edits.append((at, at, "pub "))
    edits.sort(key=lambda e: (e[0], e[1]))
    out, last = [], 0
    for lo, hi, rep in edits:
        if lo < last:
            continue
        out.append(text[last:lo])
        out.append(rep)
        last = hi
    out.append(text[last:])
    return "".join(out)


def outside_idents(files_texts):
    ids = set()
    for t in files_texts:
        ids |= set(IDENT.findall(blank(t)))
    return ids


_CACHE = {}


def file_idents(path):
    st = os.stat(path)
    key = (path, st.st_mtime_ns, st.st_size)
    if key not in _CACHE:
        _CACHE[key] = set(IDENT.findall(blank(read(path))))
    return _CACHE[key]


def live_rs_files():
    out = []
    for base, dirs, files in os.walk(PLUG):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "target", ".cache") and not (d == "📦️packages")]
        for f in files:
            if f.endswith(".rs"):
                out.append(os.path.join(base, f))
    return out
