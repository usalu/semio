#!/usr/bin/env python3
# 🧬️ Definition scanner: items, struct fields and inherent-impl members of a Rust source with their visibility and position.
import bisect, re
from rs import blank

ATTR = r"(?:#!?\[[^\]]*\]\s*)*"
HEAD = re.compile(r"^\s*(?P<attrs>" + ATTR + r")(?P<vis>pub(?:\s*\([^)]*\))?\s+)?(?P<mods>(?:(?:async|unsafe|const|extern(?:\s+\"[^\"]*\")?|default|safe)\s+)*)(?P<kind>fn|struct|enum|trait|type|const|static|mod|union|use|macro_rules!|impl|extern)\b\s*(?:mut\s+)?(?P<name>[A-Za-z_][A-Za-z0-9_]*)?", re.S)
FIELD = re.compile(r"\s*(?P<attrs>" + ATTR + r")(?P<vis>pub(?:\s*\([^)]*\))?\s+)?(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*:(?!:)", re.S)


class Def:
    def __init__(self, kind, name, vis, pos, vlen, parent, line, ctx):
        self.kind, self.name, self.vis, self.pos, self.vlen, self.parent, self.line, self.ctx = kind, name, vis, pos, vlen, parent, line, ctx

    def __repr__(self):
        return f"{self.kind}:{self.parent or ''}::{self.name}[{self.vis or 'private'}]@{self.line}"


def restricted(vis):
    return (vis or "").strip() != "pub"


def split_top(body, base):
    """split on depth-0 commas; yield (offset, text)."""
    depth, start = 0, 0
    for i, c in enumerate(body):
        if c in "<([{":
            depth += 1
        elif c in ">)]}":
            if c == ">" and i > 0 and body[i - 1] in "-=":
                continue
            depth -= 1
        elif c == "," and depth == 0:
            yield base + start, body[start:i]
            start = i + 1
    yield base + start, body[start:]


def scan(text):
    clean = blank(text)
    n = len(clean)
    lines = [0] + [m.end() for m in re.finditer(r"\n", clean)]

    def lineno(p):
        return bisect.bisect_right(lines, p)

    defs = []
    stack = []  # (kind, name)
    i = 0
    seg = 0
    while i <= n:
        c = clean[i] if i < n else ""
        if c in "{};" or i == n:
            head = clean[seg:i]
            m = HEAD.match(head)
            kind = m.group("kind") if m else None
            name = m.group("name") if m else None
            owner = stack[-1] if stack else ("mod", None)
            skip = any(k in ("fn", "block", "enum", "traitimpl", "trait") for k, _ in stack)
            vis = m.group("vis") if m else None
            vpos = seg + (m.start("vis") if m and m.group("vis") else (m.start("kind") if m else 0))
            if m and not skip:
                mods = m.group("mods") or ""
                dpos = seg + m.start("mods") if mods else seg + m.start("kind")
                dpos = seg + (m.start("vis") if m.group("vis") else (m.start("mods") if mods else m.start("kind")))
                vlen = len(m.group("vis") or "")
                if kind == "use" and owner[0] == "mod" and c == ";" and (vis or "").strip() not in ("", "pub"):
                    import rs as _rs
                    body = head[m.end("kind"):]
                    names = []
                    for path, alias, glob in _rs.expand_use(body.strip()):
                        if glob:
                            continue
                        loc = alias or (path[-1] if path and path[-1] != "self" else (path[-2] if len(path) > 1 else ""))
                        if loc:
                            names.append(loc)
                    if names:
                        defs.append(Def("reexport", "|".join(names), (vis or "").strip(), dpos, len(m.group("vis") or ""), None, lineno(seg + m.start("kind")), "use"))
                elif kind in ("fn", "const", "static", "type") and owner[0] == "impl" and name:
                    defs.append(Def("method" if kind == "fn" else "assoc", name, (vis or "").strip(), dpos, vlen, owner[1], lineno(seg + m.start("kind")), "impl"))
                elif kind in ("fn", "struct", "enum", "trait", "type", "const", "static", "mod", "union", "macro_rules!") and owner[0] in ("mod",) and name:
                    defs.append(Def("item", name, (vis or "").strip(), dpos, vlen, None, lineno(seg + m.start("kind")), kind))
            if c == "{":
                if m and kind in ("struct", "union") and not skip:
                    body_end = matching_brace(clean, i)
                    body = clean[i + 1:body_end]
                    for off, part in split_top(body, i + 1):
                        fm = FIELD.match(part)
                        if fm:
                            defs.append(Def("field", fm.group("name"), (fm.group("vis") or "").strip(), off + (fm.start("vis") if fm.group("vis") else fm.start("name")), len(fm.group("vis") or ""), name, lineno(off + fm.start("name")), "struct"))
                if m and kind in ("struct", "union"):
                    stack.append((kind if skip else "struct", name))
                elif m and kind in ("fn", "extern"):
                    stack.append(("fn", name))
                elif m and kind == "enum":
                    stack.append(("enum", name))
                elif m and kind == "trait":
                    stack.append(("trait", name))
                elif m and kind == "mod":
                    stack.append(("mod", name))
                elif m and kind == "impl":
                    rest = head[m.end("kind"):]
                    rest = rest if True else rest
                    full = head[m.start("kind"):]
                    if re.search(r"\bfor\b", full):
                        stack.append(("traitimpl", None))
                    else:
                        stripped = re.sub(r"<[^<>]*(?:<[^<>]*(?:<[^<>]*>[^<>]*)*>[^<>]*)*>", "", full[len("impl"):].split(" where ")[0])
                        names = [x for x in re.findall(r"[A-Za-z_][A-Za-z0-9_]*", stripped) if x not in ("dyn", "mut", "const", "unsafe", "async", "impl")]
                        stack.append(("impl", names[-1] if names else None))
                else:
                    stack.append(("block", None))
            elif c == "}":
                if stack:
                    stack.pop()
            elif c == ";" and m and kind == "struct" and not skip:
                pass
            seg = i + 1
        i += 1
    # tuple structs: `struct Name(pub(crate) A, B);`
    for m in re.finditer(r"(?P<attrs>" + ATTR + r")(?P<vis>pub(?:\s*\([^)]*\))?\s+)?struct\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^>{(;]*>)?\s*\(", clean):
        start = m.end() - 1
        end = matching_paren(clean, start)
        body = clean[start + 1:end]
        for idx, (off, part) in enumerate(split_top(body, start + 1)):
            pm = re.match(r"\s*(?:#\[[^\]]*\]\s*)*(pub(?:\s*\([^)]*\))?\s+)?", part)
            if part.strip():
                defs.append(Def("tfield", str(idx), (pm.group(1) or "").strip(), off + (pm.start(1) if pm.group(1) else pm.end()), len(pm.group(1) or ""), m.group("name"), lineno(off), "tuple"))
    return defs


def matching_brace(s, i):
    d = 0
    for j in range(i, len(s)):
        if s[j] == "{":
            d += 1
        elif s[j] == "}":
            d -= 1
            if d == 0:
                return j
    return len(s) - 1


def matching_paren(s, i):
    d = 0
    for j in range(i, len(s)):
        if s[j] == "(":
            d += 1
        elif s[j] == ")":
            d -= 1
            if d == 0:
                return j
    return len(s) - 1


def widen(text, d, hide=True):
    """return text with definition d made `pub` (marked `#[doc(hidden)]` on its own line unless already hidden)."""
    if d.vis == "pub":
        return text
    prefix = ""
    if hide and not re.search(r"#\[doc\(hidden\)\]\s*$", text[max(0, d.pos - 60):d.pos]):
        line_start = text.rfind("\n", 0, d.pos) + 1
        indent = text[line_start:d.pos]
        if indent.strip() == "":
            prefix = "#[doc(hidden)]\n" + indent
        else:
            prefix = "#[doc(hidden)] "
    if d.vlen:
        return text[:d.pos] + prefix + "pub " + text[d.pos + d.vlen:]
    return text[:d.pos] + prefix + "pub " + text[d.pos:]

if __name__ == "__main__":
    import sys
    t = open(sys.argv[1], encoding="utf8").read()
    for d in scan(t):
        print(d)
