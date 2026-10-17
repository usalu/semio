#!/usr/bin/env python3
"""🔬️ Structural Rust scanner for the integration tools: splits a module body into top-level statements (items, uses, mods, macro calls) with brace-aware scanning over comment- and string-blanked text.
Returns spans in the original text so edits can be exact. Only what the flip needs: item names and kinds, visibility, attributes, `use` trees, `mod` declarations (file or inline), `include!` calls, `macro_rules!`."""
import re
from rs import blank, expand_use

HEAD = re.compile(r"""\s*((?:\#\!?\[(?:[^\[\]]|\[[^\]]*\])*\]\s*)*)          # attributes
    (pub(?:\s*\([^)]*\))?\s+)?                                                  # visibility
    (?:(?:default|async|unsafe|const(?=\s+(?:fn|unsafe|extern|async))|extern(?:\s+"[^"]*")?(?=\s+(?:fn|crate|\{)))\s+)*   # qualifiers
    (fn|struct|enum|union|trait|impl|mod|use|type|const|static|extern\s+crate|macro_rules!|macro)(?:\b|(?<=!))\s*(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*|\{|<|\()?""", re.X | re.S)
BLOCK_KINDS = {"fn", "struct", "enum", "union", "trait", "impl", "mod", "macro_rules!", "macro"}


class Stmt:
    __slots__ = ("kind", "name", "vis", "attrs", "start", "end", "head_end", "text", "body", "inline")

    def __repr__(self):
        return f"<{self.kind} {self.name} {self.vis!r} {self.start}-{self.end}>"


def _skip_ws(s, i):
    n = len(s)
    while i < n and s[i].isspace():
        i += 1
    return i


def statements(text, clean=None, lo=0, hi=None):
    """top-level statements of text[lo:hi] (depth 0 of that range)"""
    clean = clean if clean is not None else blank(text)
    hi = len(clean) if hi is None else hi
    out = []
    i = _skip_ws(clean, lo)
    while i < hi:
        start = i
        m = HEAD.match(clean, i)
        kind = None
        stmt = Stmt()
        if m and m.group(3):
            kind = m.group(3)
            kind = re.sub(r"\s+", " ", kind)
        # find the end: scan to ';' or block end at depth 0
        depth_b = depth_p = depth_k = 0
        j = i
        end = None
        block_end_ok = kind in BLOCK_KINDS or kind is None
        saw_block = False
        while j < hi:
            c = clean[j]
            if c in "([":
                depth_p += 1
            elif c in ")]":
                depth_p -= 1
            elif c == "{":
                depth_b += 1
            elif c == "}":
                depth_b -= 1
                if depth_b == 0 and depth_p == 0:
                    saw_block = True
                    if block_end_ok:
                        # statement like `struct S {..}` ends here; an expression-ish `macro!{..}` too
                        k = _skip_ws(clean, j + 1)
                        if kind in ("fn", "impl", "trait", "mod", "enum", "struct", "union", "macro_rules!", "macro") or kind is None:
                            if kind == "struct" or kind == "union" or kind is None:
                                # `struct S { .. }` ends; `where`-style continuations cannot follow a block
                                pass
                            end = j + 1
                            break
            elif c == ";" and depth_b == 0 and depth_p == 0:
                end = j + 1
                break
            elif c == "<" and kind in ("fn", "impl", "struct", "enum", "trait", "type", "union") and False:
                pass
            j += 1
        if end is None:
            end = hi
        stmt.start, stmt.end = start, end
        stmt.text = text[start:end]
        stmt.kind = kind or "other"
        stmt.attrs = text[m.start(1):m.end(1)].strip() if m else ""
        stmt.vis = (m.group(2) or "").strip() if m else ""
        stmt.name = None
        stmt.head_end = m.end() if m else start
        stmt.body = None
        stmt.inline = False
        if m and m.group(4) and re.match(r"[A-Za-z_]", m.group(4)) and kind not in ("impl", "use"):
            stmt.name = m.group(4)
        if stmt.kind == "extern crate":
            mm = re.match(r"extern\s+crate\s+([A-Za-z_][A-Za-z0-9_]*)(?:\s+as\s+([A-Za-z_][A-Za-z0-9_]*))?", clean[m.start(3):end])
            if mm:
                stmt.name = mm.group(2) or mm.group(1)
        if stmt.kind == "mod":
            # inline when a block follows the name
            rest = clean[m.end(4) if m.group(4) else m.end():end] if m else ""
            bi = clean.find("{", m.end(3), end) if m else -1
            si = clean.find(";", m.end(3), end) if m else -1
            if bi != -1 and (si == -1 or bi < si):
                stmt.inline = True
                stmt.body = (bi + 1, end - 1)
        out.append(stmt)
        i = _skip_ws(clean, end)
    return out


def mod_statements(text, clean=None):
    return statements(text, clean)


USE_BODY = re.compile(r"use\s+(.*)$", re.S)


def use_entries(stmt, clean_text):
    """[(path list, alias|None, glob bool)] for a `use` statement"""
    body = clean_text[stmt.start:stmt.end]
    m = re.search(r"\buse\s+(.*?);?\s*$", body, re.S)
    if not m:
        return []
    return expand_use(m.group(1).rstrip(";"))


def cfg_of(attrs):
    return " ".join(re.findall(r"#\[\s*cfg\((?:[^\[\]]|\[[^\]]*\])*\)\s*\]", attrs))


def is_test_cfg(attrs):
    return bool(re.search(r"cfg\(\s*test\s*\)", attrs))

if __name__ == "__main__":
    import sys
    t = open(sys.argv[1], encoding="utf8").read()
    for s in statements(t):
        print(s.kind, s.name, s.vis, s.start, s.end, "inline" if s.inline else "", s.attrs[:40].replace("\n", " "))


ATTR_GROUP = re.compile(r"(?:\#\!?\[(?:[^\[\]]|\[[^\]]*\])*\]\s*)*")


def split_attrs(text, clean, st):
    """(attribute text, offset where the statement proper starts) for any statement kind"""
    m = ATTR_GROUP.match(clean, st.start)
    end = m.end() if m else st.start
    return text[st.start:end].strip(), end
